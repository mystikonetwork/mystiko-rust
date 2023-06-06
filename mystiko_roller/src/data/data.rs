use crate::common::env::load_roller_circuits_path;
use crate::common::error::Result;
use crate::context::ContextTrait;
use crate::data::calc::calc_rollup_size_array;
use crate::db::document::commitment::CommitmentInfo;
use ethers_core::types::U256;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_fs::read_file_bytes;
use mystiko_protocol::rollup::{Rollup, RollupProof};
use num_bigint::BigInt;
use std::sync::Arc;
use tracing::{debug, info};

pub struct CommitmentData {
    hash: BigInt,
    rollup_fee: U256,
    block_number: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RollupPlan {
    pub sizes: Vec<usize>,
    pub total_fee: U256,
    pub force: bool,
}

pub struct ProofInfo {
    pub r: RollupProof,
}

pub struct DataHandle {
    chain_id: u64,
    pool_contract: PoolContractConfig,
    context: Arc<dyn ContextTrait>,
    next_sync_block: u64,
    commitments: Vec<CommitmentData>,
    tree: Option<MerkleTree>,
}

impl DataHandle {
    pub async fn new(chain_id: u64, pool_contract: &PoolContractConfig, context: Arc<dyn ContextTrait>) -> Self {
        DataHandle {
            chain_id,
            pool_contract: pool_contract.clone(),
            context,
            next_sync_block: 0 as u64,
            commitments: vec![],
            tree: None,
        }
    }

    pub fn get_next_sync_block(&self) -> u64 {
        self.next_sync_block
    }

    pub fn set_new_next_sync_block(&mut self, block_number: u64) {
        if block_number > self.next_sync_block {
            debug!("set new start block {:?}", block_number);
            self.next_sync_block = block_number;
        }
    }

    fn push_commitment_in_queue(&mut self, cm: &CommitmentInfo) {
        let cm_data = build_commitment_data(cm);

        self.commitments.push(cm_data);
    }

    pub async fn load_commitment_from_db(&mut self) {
        let cms = self
            .context
            .db()
            .await
            .find_all_commitment(self.chain_id, self.pool_contract.address())
            .await;

        info!("load {:?} commitments from db", cms.len());

        for doc in &cms {
            self.push_commitment_in_queue(&doc.data);
        }

        if cms.len() == 0 {
            self.set_new_next_sync_block(self.pool_contract.start_block());
        } else {
            self.set_new_next_sync_block(cms[cms.len() - 1].data.block_number);
        }
        info!("set start block number {:?}", self.next_sync_block);
    }

    pub async fn insert_commitments(&mut self, cms: &[CommitmentInfo]) {
        for cm in cms {
            let index = cm.leaf_index as usize;
            if index < self.commitments.len() {
                assert_eq!(self.commitments[index].hash, cm.commitment_hash);
            } else {
                self.push_commitment_in_queue(cm);
                self.context.db().await.insert_commitment(cm).await;
            }
        }
    }

    pub fn get_included_count(&self) -> usize {
        self.tree.as_ref().map_or(0, |t| t.count())
    }

    pub fn get_commitments_queue_count(&self) -> usize {
        self.commitments.len()
    }

    fn rebuild_tree(&mut self, count: usize) {
        info!("rebuild merkle tree");
        let height = self.context.cfg().rollup.merkle_tree_height;
        let init_elem: Vec<BigInt> = self.commitments.iter().take(count).map(|cm| cm.hash.clone()).collect();
        self.tree = Some(MerkleTree::new(Some(init_elem), Some(height), None).expect("rebuild merkle tree meet error"));
    }

    fn build_rollup_plan(&self, force_rollup_block_count: u64) -> RollupPlan {
        let included_len = self.get_included_count();
        let queued_len = self.get_commitments_queue_count() - included_len;
        let sizes = calc_rollup_size_array(included_len, queued_len);

        let counter: usize = sizes.iter().sum();
        let total_fee = self.commitments[included_len..(included_len + counter)]
            .iter()
            .map(|cm| cm.rollup_fee.clone())
            .fold(U256::zero(), |acc, x| acc + x);

        let force = {
            let mut record_block: u64 = 0;
            if !self.commitments.is_empty() {
                record_block = self.commitments[included_len].block_number;
            }
            self.next_sync_block >= record_block + force_rollup_block_count
        };

        RollupPlan {
            force,
            sizes,
            total_fee,
        }
    }

    pub fn generate_plan(&mut self, included: usize, force_rollup_block_count: u64) -> Result<RollupPlan> {
        assert!(included <= self.commitments.len());
        let tree_count = self.tree.as_ref().map_or(0, |t| t.count());
        if self.tree.is_none() || tree_count > included {
            self.rebuild_tree(included);
        } else if tree_count < included {
            if let Some(tree) = &mut self.tree {
                let cms = self
                    .commitments
                    .iter()
                    .skip(tree_count)
                    .take(included - tree_count)
                    .map(|cm| cm.hash.clone())
                    .collect();
                let _ = tree.bulk_insert(cms).expect("tree bulk insert error");
            }
        }

        Ok(self.build_rollup_plan(force_rollup_block_count))
    }

    pub async fn generate_proof(&mut self, plan: &RollupPlan) -> Result<ProofInfo> {
        assert!(!plan.sizes.is_empty());
        info!("generate proof size {:?}", plan.sizes[0]);

        let tree = self.tree.as_mut().unwrap();
        let tree_len = tree.count();
        let new_leaves = self
            .commitments
            .iter()
            .skip(tree_len)
            .take(plan.sizes[0])
            .map(|cm| cm.hash.clone())
            .collect();

        let circuits = CircuitsConfig::new(plan.sizes[0]);
        let program = read_file_bytes(&circuits.program_file)
            .await
            .expect("read zk program error");
        let abi = read_file_bytes(&circuits.abi_file).await.expect("read zk abi error");
        let pkey = read_file_bytes(&circuits.proving_key_file)
            .await
            .expect("read zk proving key error");
        let mut rollup = Rollup::new(tree, new_leaves, program, abi, pkey);
        let proof = rollup.prove().expect("build proof error");

        Ok(ProofInfo { r: proof })
    }
}

fn build_commitment_data(cm: &CommitmentInfo) -> CommitmentData {
    CommitmentData {
        hash: cm.commitment_hash.clone(),
        rollup_fee: U256::from_str_radix(&cm.rollup_fee, 10).expect("rollup fee error"),
        block_number: cm.block_number,
    }
}

pub struct CircuitsConfig {
    pub program_file: String,
    pub abi_file: String,
    pub proving_key_file: String,
}

impl CircuitsConfig {
    pub fn new(rollup_size: usize) -> CircuitsConfig {
        let rollup_name = format!("Rollup{}", rollup_size);

        let circuits_path = load_roller_circuits_path();
        CircuitsConfig {
            program_file: circuits_path.clone() + &(format!("/{}.program", rollup_name)),
            abi_file: circuits_path.clone() + &(format!("/{}.abi.json", rollup_name)),
            proving_key_file: circuits_path + &(format!("/{}.pkey", rollup_name)),
        }
    }
}
