use crate::common::error::{Result, RollerError};
use crate::config::settings::PullConfig;
use crate::context::ContextTrait;
use crate::data::data::DataHandle;
use crate::db::document::commitment::CommitmentInfo;
use crate::instance::contract::commitment::CommitmentContractInstance;
use ethers_providers::Middleware;
use num_bigint::BigInt;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tracing::{debug, error};

pub struct PullHandle {
    chain_id: u64,
    contract_address: String,
    cfg: PullConfig,
    context: Arc<dyn ContextTrait>,
    data: Rc<RefCell<DataHandle>>,
}

impl PullHandle {
    pub fn new(contract_address: &str, context: Arc<dyn ContextTrait>, data: Rc<RefCell<DataHandle>>) -> Self {
        let cfg = context.cfg().pull.clone();
        PullHandle {
            chain_id: context.cfg().chain.chain_id,
            contract_address: contract_address.to_string(),
            cfg,
            context,
            data,
        }
    }

    async fn pull_from_provider(&self) -> Result<()> {
        debug!("pull from provider");

        let provider = self.context.provider().await?;
        let latest_block = provider.get_block_number().await?.as_u64();

        let batch = self.cfg.batch_block_from_provider as usize;
        let mut data_handle = self.data.borrow_mut();
        let pool_contract = CommitmentContractInstance::new(self.chain_id, &self.contract_address, provider);

        for start in (data_handle.get_next_sync_block() + 1..=latest_block).step_by(batch) {
            let end = std::cmp::min(start + batch as u64 - 1, latest_block);
            let info_cms = pool_contract.query_queued_commitments(start, end).await?;
            data_handle.insert_commitments(info_cms.as_slice()).await;
        }

        data_handle.set_new_next_sync_block(latest_block);
        Ok(())
    }

    async fn pull_from_indexer(&self) -> Result<()> {
        debug!("pull from indexer");

        let indexer = self.context.indexer().await.ok_or(RollerError::NoIndexer)?;

        let latest_block = indexer
            .get_latest_block_number(self.chain_id, &self.contract_address)
            .await?;
        debug!("indexer latest block number {:?}", latest_block);

        let mut data = self.data.borrow_mut();
        let batch = self.cfg.batch_block_from_indexer as usize;
        for start in (data.get_next_sync_block() + 1..=latest_block).step_by(batch) {
            let end = std::cmp::min(start + batch as u64 - 1, latest_block);
            let cms = indexer
                .get_queued_commitment(self.chain_id, &self.contract_address, start, end)
                .await?;

            let info_cms = cms
                .iter()
                .map(|cm| {
                    let commitment_hash =
                        BigInt::parse_bytes(cm.commit_hash.as_bytes(), 10).ok_or(RollerError::InvalidCommitmentHash)?;
                    Ok(CommitmentInfo {
                        chain_id: self.chain_id,
                        contract_address: self.contract_address.clone(),
                        commitment_hash: commitment_hash,
                        block_number: cm.block_num,
                        rollup_fee: cm.rollup_fee.clone(),
                        leaf_index: cm.leaf_index,
                        tx_hash: cm.tx_hash.clone(),
                    })
                })
                .collect::<anyhow::Result<Vec<CommitmentInfo>, RollerError>>()?;
            data.insert_commitments(info_cms.as_slice()).await;
        }

        data.set_new_next_sync_block(latest_block + 1);
        Ok(())
    }

    pub async fn pull(&self) -> Result<()> {
        debug!("pull");

        // todo pull from xsacn
        if let Some(_) = self.context.indexer().await {
            let result = self.pull_from_indexer().await;
            if result.is_err() {
                error!("pull from indexer meet error: {:?}", result.err().unwrap());
                return self.pull_from_provider().await;
            }
        } else {
            return self.pull_from_provider().await;
        }

        Ok(())
    }
}
