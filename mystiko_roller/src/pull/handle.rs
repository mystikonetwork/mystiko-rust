use crate::common::error::{Result, RollerError};
use crate::config::config::Pull as PullConfig;
use crate::context::Context;
use crate::data::data::DataHandle;
use crate::db::document::commitment::CommitmentInfo;
use num_bigint::BigInt;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tracing::debug;

pub struct PullHandle {
    chain_id: u64,
    contract_address: String,
    cfg: PullConfig,
    context: Arc<Context>,
    data: Rc<RefCell<DataHandle>>,
}

impl PullHandle {
    pub fn new(contract_address: &str, context: Arc<Context>, data: Rc<RefCell<DataHandle>>) -> Self {
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
        Err(RollerError::NoProvider)
        // let providers = self.context.providers().await.check_provider(self.chain_id)?;
        // let latest_block = providers.get_block_number().await? as u64;
        //
        // let batch = self.cfg.batch_block_from_provider as usize;
        // for start in (self.data.borrow().get_start_block() + 1..=latest_block).step_by(batch) {
        //     let end = std::cmp::min(start + batch as u64 - 1, latest_block);
        //
        //     let cms = providers
        //         .get_commitments(self.chain_id, &self.contract_address, start, end)
        //         .await?;
        //     let info_cms = cms
        //         .iter()
        //         .map(|cm| {
        //             let commitment_hash =
        //                 BigInt::parse_bytes(cm.commit_hash.as_bytes(), 10).ok_or(PullError::InvalidCommitmentHash)?;
        //             Ok(CommitmentInfo {
        //                 chain_id: self.chain_id,
        //                 contract_address: self.contract_address.clone(),
        //                 commitment_hash: commitment_hash,
        //                 block_number: cm.block_num,
        //                 rollup_fee: cm.rollup_fee.clone(),
        //                 leaf_index: cm.leaf_index,
        //                 tx_hash: cm.tx_hash.clone(),
        //             })
        //         })
        //         .collect::<Result<Vec<CommitmentInfo>, PullError>>()?;
        //     self.data.borrow_mut().insert_commitments(info_cms).await;
        // }
        // self.data.borrow_mut().set_new_start_block(latest_block);
        // Ok(())
    }

    async fn pull_from_indexer(&self) -> Result<()> {
        debug!("pull from indexer");

        let indexer = self.context.indexer().await.ok_or(RollerError::NoIndexer)?;

        let latest_block = indexer
            .get_latest_block_number(self.chain_id, &self.contract_address)
            .await?;

        let batch = self.cfg.batch_block_from_indexer as usize;
        for start in (self.data.borrow().get_next_sync_block() + 1..=latest_block).step_by(batch) {
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
            self.data.borrow_mut().insert_commitments(info_cms).await;
        }

        self.data.borrow_mut().set_new_next_sync_block(latest_block + 1);
        Ok(())
    }

    pub async fn pull(&self) -> Result<()> {
        debug!("pull");

        // todo pull from xsacn
        if let Some(_) = self.context.indexer().await {
            if self.pull_from_indexer().await.is_err() {
                self.pull_from_provider().await?
            }
        } else {
            self.pull_from_provider().await?
        }

        Ok(())
    }
}
