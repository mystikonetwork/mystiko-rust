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
use std::cmp::Ordering;
use std::sync::Arc;
use tracing::{error, info};

pub struct CommitmentData {
    pub cm: BigInt,
    pub rollup_fee: U256,
    pub block_number: u64,
    pub deposit_tx: String,
}

impl CommitmentData {
    pub fn from(info: &CommitmentInfo) -> Self {
        CommitmentData {
            cm: info.commitment_hash.clone(),
            rollup_fee: U256::from_str_radix(&info.rollup_fee, 10).expect("rollup fee error"),
            block_number: info.block_number,
            deposit_tx: info.tx_hash.clone(),
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
    commitments: Vec<CommitmentData>,
    tree: MerkleTree,
    empty_queue_check_counter: u32,
    next_sync_block: u64,
    latest_rollup_tx_block_number: u64,
}

impl DataHandler {
    pub async fn new(chain_id: u64, pool_contract: &PoolContractConfig, context: Arc<dyn ContextTrait + Send>) -> Self {
        let height = context.cfg().rollup.merkle_tree_height;
        let tree = MerkleTree::new(None, Some(height), None).unwrap();
        DataHandler {
            chain_id,
            pool_contract: pool_contract.clone(),
            context,
            commitments: vec![],
            tree,
            empty_queue_check_counter: 0,
            next_sync_block: 0_u64,
            latest_rollup_tx_block_number: 0_u64,
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        let cms = self
            .context
            .db()
            .await
            .find_all_commitment(self.chain_id, self.pool_contract.address())
            .await;
        info!("init load {:?} commitments from db", cms.len());

        for doc in &cms {
            self.push_commitment_in_queue(&doc.data)?;
        }

        self.revert_next_sync_block();
        info!("start block number at {:?}", self.next_sync_block);
        Ok(())
    }

    pub fn get_empty_queue_check_counter(&self) -> u32 {
        self.empty_queue_check_counter
    }

    pub fn inc_empty_queue_check_counter(&mut self) {
        self.empty_queue_check_counter += 1;
    }

    pub fn set_empty_queue_check_counter(&mut self, counter: u32) {
        self.empty_queue_check_counter = counter;
    }

    pub fn get_latest_rollup_tx_block_number(&self) -> u64 {
        self.latest_rollup_tx_block_number
    }

    pub fn set_latest_rollup_tx_block_number(&mut self, block_number: u64) {
        self.latest_rollup_tx_block_number = block_number;
    }

    pub fn get_next_sync_block(&self) -> u64 {
        self.next_sync_block
    }

    pub fn update_next_sync_block(&mut self, block_number: u64) {
        self.next_sync_block = block_number;
    }

    pub fn revert_next_sync_block(&mut self) {
        let block_number = match self.commitments.last() {
            Some(cm) => cm.block_number,
            None => self.pool_contract.start_block(),
        };
        self.next_sync_block = block_number;
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
            self.revert_next_sync_block();
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

    pub async fn insert_commitments(&mut self, cms: &[CommitmentInfo]) -> Result<()> {
        for cm in cms {
            let index = cm.leaf_index as usize;
            if index < self.commitments.len() {
                assert_eq!(self.commitments[index].cm, cm.commitment_hash);
            } else {
                info!(
                    "push commitment in queue {:?} {:?} from deposit tx {:?}",
                    cm.leaf_index, cm.commitment_hash, cm.tx_hash
                );
                self.push_commitment_in_queue(cm)?;
                self.context.db().await.insert_commitment(cm).await;
            }
        }
        Ok(())
    }

    pub fn get_included_count(&self) -> usize {
        self.tree.count()
    }

    pub fn get_commitments_queue_count(&self) -> usize {
        self.commitments.len()
    }

    fn build_rollup_plan(&self, force_rollup_block_count: u64) -> RollupPlan {
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

    pub fn generate_plan(&mut self, included_count: usize, force_rollup_block_count: u64) -> Result<RollupPlan> {
        assert!(included_count <= self.commitments.len());
        let tree_len = self.tree.count();
        match tree_len.cmp(&included_count) {
            Ordering::Equal => {}
            Ordering::Less => {
                let cms = self
                    .commitments
                    .iter()
                    .skip(tree_len)
                    .take(included_count - tree_len)
                    .map(|cm| cm.cm.clone())
                    .collect();
                self.tree.bulk_insert(cms)?;
            }
            Ordering::Greater => {
                info!("revert merkle tree with included count {:?}", included_count);
                self.tree.revert(included_count)?
            }
        }

        Ok(self.build_rollup_plan(force_rollup_block_count))
    }

    pub async fn generate_proof(&mut self, plan: &RollupPlan) -> Result<ProofInfo> {
        assert!(!plan.sizes.is_empty());
        let rollup_size = plan.sizes[0];

        info!("generate proof size {:?}", rollup_size);
        let tree_len = self.tree.count();
        let new_leaves = self
            .commitments
            .iter()
            .skip(tree_len)
            .take(plan.sizes[0])
            .map(|cm| cm.cm.clone())
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

        let mut rollup = Rollup::new(&mut self.tree, new_leaves, program, abi, pkey);
        let proof = rollup.prove()?;
        Ok(ProofInfo { r: proof })
    }
}
