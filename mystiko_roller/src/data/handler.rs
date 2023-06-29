use crate::common::env::load_roller_circuits_path;
use crate::common::error::{Result, RollerError};
use crate::context::ContextTrait;
use crate::data::calc::{calc_rollup_size_array, circuit_type_from_rollup_size};
use crate::db::document::commitment::CommitmentInfo;
use ethers_core::types::U256;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_downloader::DownloaderBuilder;
use mystiko_protocol::rollup::{Rollup, RollupProof};
use num_bigint::BigInt;
use std::sync::Arc;
use tracing::{debug, error, info};

pub struct CommitmentData {
    pub hash: BigInt,
    rollup_fee: U256,
    block_number: u64,
}

impl CommitmentData {
    pub fn from(info: &CommitmentInfo) -> Self {
        CommitmentData {
            hash: info.commitment_hash.clone(),
            rollup_fee: U256::from_str_radix(&info.rollup_fee, 10).expect("rollup fee error"),
            block_number: info.block_number,
        }
    }
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

pub struct DataHandler {
    chain_id: u64,
    pool_contract: PoolContractConfig,
    context: Arc<dyn ContextTrait>,
    next_sync_block: u64,
    commitments: Vec<CommitmentData>,
    tree: Option<MerkleTree>,
}

impl DataHandler {
    pub async fn new(chain_id: u64, pool_contract: &PoolContractConfig, context: Arc<dyn ContextTrait + Send>) -> Self {
        DataHandler {
            chain_id,
            pool_contract: pool_contract.clone(),
            context,
            next_sync_block: 0_u64,
            commitments: vec![],
            tree: None,
        }
    }

    pub fn get_next_sync_block(&self) -> u64 {
        self.next_sync_block
    }

    pub fn set_next_sync_block(&mut self, block_number: u64) {
        if block_number > self.next_sync_block {
            debug!("set new start block {:?}", block_number);
            self.next_sync_block = block_number;
        }
    }

    pub fn reset_next_sync_block(&mut self) {
        if let Some(last_commitment) = self.commitments.last() {
            self.set_next_sync_block(last_commitment.block_number);
        } else {
            self.set_next_sync_block(self.pool_contract.start_block());
        }
    }

    pub fn get_batch_commitments(&self, start: usize, count: usize) -> &[CommitmentData] {
        if start + count > self.commitments.len() {
            error!("get batch commitments out of range");
            &[]
        } else {
            &self.commitments[start..(start + count)]
        }
    }

    fn push_commitment_in_queue(&mut self, cm: &CommitmentInfo) -> Result<()> {
        let expect_leaf_index = self.commitments.len();
        if cm.leaf_index as usize != expect_leaf_index {
            self.reset_next_sync_block();
            error!(
                "leaf index mismatch insert {:?} expect {:?}, revert next sync block number to {:?}",
                cm.leaf_index,
                expect_leaf_index,
                self.get_next_sync_block()
            );

            return Err(RollerError::CommitmentMissing);
        }

        self.commitments.push(CommitmentData::from(cm));
        Ok(())
    }

    pub async fn load_commitment_from_db(&mut self) -> Result<()> {
        let cms = self
            .context
            .db()
            .await
            .find_all_commitment(self.chain_id, self.pool_contract.address())
            .await;

        info!("load {:?} commitments from db", cms.len());

        for doc in &cms {
            self.push_commitment_in_queue(&doc.data)?;
        }

        self.reset_next_sync_block();
        info!("start block number at {:?}", self.next_sync_block);
        Ok(())
    }

    pub async fn insert_commitments(&mut self, cms: &[CommitmentInfo]) -> Result<()> {
        for cm in cms {
            let index = cm.leaf_index as usize;
            if index < self.commitments.len() {
                assert_eq!(self.commitments[index].hash, cm.commitment_hash);
            } else {
                info!("push commitment in queue {:?} {:?}", cm.leaf_index, cm.commitment_hash);
                self.push_commitment_in_queue(cm)?;
                self.context.db().await.insert_commitment(cm).await;
            }
        }
        Ok(())
    }

    pub fn get_included_count(&self) -> usize {
        self.tree.as_ref().map_or(0, |t| t.count())
    }

    pub fn get_commitments_queue_count(&self) -> usize {
        self.commitments.len()
    }

    fn rebuild_tree(&mut self, count: usize) {
        info!("rebuild merkle tree with included count {:?}", count);
        let height = self.context.cfg().rollup.merkle_tree_height;
        let init_elem: Vec<BigInt> = self.commitments.iter().take(count).map(|cm| cm.hash.clone()).collect();
        self.tree = Some(MerkleTree::new(Some(init_elem), Some(height), None).expect("rebuild merkle tree meet error"));
    }

    fn build_rollup_plan(&self, force_rollup_block_count: u64) -> RollupPlan {
        debug!("build rollup plan");

        let included_len = self.get_included_count();
        let queued_len = self.get_commitments_queue_count() - included_len;
        let sizes = calc_rollup_size_array(included_len, queued_len);

        let counter: usize = sizes.iter().sum();
        let total_fee = self.commitments[included_len..(included_len + counter)]
            .iter()
            .map(|cm| cm.rollup_fee)
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
                tree.bulk_insert(cms).expect("tree bulk insert error");
            }
        }

        Ok(self.build_rollup_plan(force_rollup_block_count))
    }

    pub async fn generate_proof(&mut self, plan: &RollupPlan) -> Result<ProofInfo> {
        assert!(!plan.sizes.is_empty());
        let rollup_size = plan.sizes[0];

        info!("generate proof size {:?}", rollup_size);
        let tree = self.tree.as_mut().unwrap();
        let tree_len = tree.count();
        let new_leaves = self
            .commitments
            .iter()
            .skip(tree_len)
            .take(plan.sizes[0])
            .map(|cm| cm.hash.clone())
            .collect();

        let circuits_type = circuit_type_from_rollup_size(rollup_size);
        let circuits_cfg = self
            .pool_contract
            .circuit_by_type(&circuits_type)
            .ok_or(RollerError::CircuitNotFound)?;

        let mut downloader = DownloaderBuilder::new()
            .folder(&load_roller_circuits_path())
            .build()
            .await?;

        let program = downloader
            .read_bytes_failover(circuits_cfg.program_file(), None)
            .await?;
        let abi = downloader.read_bytes_failover(circuits_cfg.abi_file(), None).await?;
        let pkey = downloader
            .read_bytes_failover(circuits_cfg.proving_key_file(), None)
            .await?;

        let mut rollup = Rollup::new(tree, new_leaves, program, abi, pkey);
        let proof = rollup.prove()?;
        Ok(ProofInfo { r: proof })
    }
}
