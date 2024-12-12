use crate::scanner::scan::ScanningAccount;
use crate::{Commitment, CommitmentColumn, CommitmentPoolContractHandler, Scanner, ScannerError, WalletHandler};
use anyhow::anyhow;
use mystiko_ethers::Providers;
use mystiko_protos::core::scanner::v1::SyncOptions;
use mystiko_protos::data::v1::{Commitment as ProtosCommitment, CommitmentStatus, Nullifier as ProtosNullifier};
use mystiko_protos::sequencer::v1::{FetchChainRequest, FetchChainResponse};
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_sequencer_client::SequencerClient;
use mystiko_storage::{Document, StatementFormatter, Storage};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Eq, Hash, PartialEq, TypedBuilder)]
struct ImportFromProviderKey {
    chain_id: u64,
    queued_transaction_hash: String,
}

impl<F, S, C, Q, P> Scanner<F, S, C, Q, P>
where
    F: StatementFormatter,
    S: Storage,
    C: CommitmentPoolContractHandler,
    Q: SequencerClient<FetchChainRequest, FetchChainResponse, ProtosCommitment, ProtosNullifier>,
    P: Providers + 'static,
    ScannerError: From<C::Error>,
{
    pub(crate) async fn asset_sync(&self, options: SyncOptions) -> Result<(), ScannerError> {
        self.wallets.check_password(&options.wallet_password).await?;
        let accounts = self.build_filter_accounts(&[]).await?;
        let scan_accounts = self.build_scan_accounts(&accounts, &options.wallet_password).await?;
        let cms = self.get_scan_commitments(&scan_accounts).await?;
        let mut cms_from_provider: HashMap<ImportFromProviderKey, Document<Commitment>> = HashMap::new();
        let mut cms_by_hash = Vec::new();
        let mut cms_to_update = Vec::new();
        for cm in cms.into_iter() {
            let cm_data = &cm.data;
            if let Some(queued_transaction_hash) = cm_data.queued_transaction_hash.clone() {
                if cm_data.leaf_index.is_none() {
                    cms_from_provider.insert(
                        ImportFromProviderKey::builder()
                            .chain_id(cm_data.chain_id)
                            .queued_transaction_hash(queued_transaction_hash)
                            .build(),
                        cm,
                    );
                }
            } else if cm_data.leaf_index.is_none() {
                cms_by_hash.push(cm);
            } else {
                cms_to_update.push(cm);
            }
        }

        let failed_cms = self
            .sync_commitments_from_provider(&options, &scan_accounts, cms_from_provider.values().cloned().collect())
            .await;
        for cm in failed_cms {
            if cms_by_hash
                .iter()
                .find(|c| {
                    c.data.chain_id == cm.data.chain_id
                        && c.data.contract_address == cm.data.contract_address
                        && c.data.commitment_hash == cm.data.commitment_hash
                })
                .is_none()
            {
                cms_by_hash.push(cm);
            }
        }
        self.sync_by_commitment_hash(&options, &scan_accounts, cms_by_hash)
            .await;
        // self.sync_commitment_status(cms_to_update).await;
        Ok(())
    }

    async fn get_scan_commitments(
        &self,
        accounts: &[ScanningAccount],
    ) -> Result<Vec<Document<Commitment>>, ScannerError> {
        let shielded_addresses = accounts
            .into_iter()
            .map(|account| account.shielded_address.address())
            .collect::<Vec<_>>();
        let status = vec![
            CommitmentStatus::SrcSucceeded as i32,
            CommitmentStatus::Queued as i32,
            CommitmentStatus::Included as i32,
        ];
        let sub_filters = vec![
            SubFilter::in_list(CommitmentColumn::Status, status),
            SubFilter::equal(CommitmentColumn::Spent, false),
            SubFilter::in_list(CommitmentColumn::ShieldedAddress, shielded_addresses),
        ];
        Ok(self.db.commitments.find(Condition::and(sub_filters)).await?)
    }

    async fn sync_commitments_from_provider(
        &self,
        options: &SyncOptions,
        accounts: &[ScanningAccount],
        commitments: Vec<Document<Commitment>>,
    ) -> Vec<Document<Commitment>> {
        let concurrency = std::cmp::max(1, options.concurrency.unwrap_or_default()) as usize;
        let chunk_nums = commitments.len().div_ceil(concurrency);
        let chunks = commitments.chunks(chunk_nums);
        let mut group_task = Vec::with_capacity(chunks.len());
        for chunk in chunks {
            group_task.push(self.sync_batch_from_provider(accounts, chunk));
        }
        let results = futures::future::join_all(group_task).await;
        results.into_iter().flatten().collect()
    }

    async fn sync_batch_from_provider(
        &self,
        accounts: &[ScanningAccount],
        commitments: &[Document<Commitment>],
    ) -> Vec<Document<Commitment>> {
        let mut failed_cms = Vec::new();
        for cm in commitments.iter() {
            let result = self.sync_one_from_provider(accounts, cm).await;
            if result.map_or(false, |r| r == false) {
                failed_cms.push(cm.clone());
            }
        }
        failed_cms
    }

    async fn sync_one_from_provider(
        &self,
        accounts: &[ScanningAccount],
        commitment: &Document<Commitment>,
    ) -> Result<bool, ScannerError> {
        let provider = self.providers.get_provider(commitment.data.chain_id).await?;
        if let Some(transaction_hash) = commitment.data.queued_transaction_hash.clone() {
            let result = self
                .asset_import_by_chain_transaction(accounts, provider, commitment.data.chain_id, &transaction_hash)
                .await;
            if result.len() > 0 {
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn sync_by_commitment_hash(
        &self,
        options: &SyncOptions,
        accounts: &[ScanningAccount],
        commitments: Vec<Document<Commitment>>,
    ) {
        let concurrency = std::cmp::max(1, options.concurrency.unwrap_or_default()) as usize;
        let chunk_nums = commitments.len().div_ceil(concurrency);
        let chunks = commitments.chunks(chunk_nums);
        let mut group_task = Vec::with_capacity(chunks.len());
        for chunk in chunks {
            group_task.push(self.sync_batch_by_commitment_hash(accounts, chunk));
        }
        futures::future::join_all(group_task).await;
    }

    async fn sync_batch_by_commitment_hash(&self, accounts: &[ScanningAccount], commitments: &[Document<Commitment>]) {
        for cm in commitments.iter() {
            let _ = self.sync_one_by_commitment_hash(accounts, cm).await;
        }
    }

    async fn sync_one_by_commitment_hash(
        &self,
        accounts: &[ScanningAccount],
        commitment: &Document<Commitment>,
    ) -> Result<(), ScannerError> {
        let pool_contract = self
            .config
            .find_pool_contract_by_address(commitment.data.chain_id, &commitment.data.contract_address)
            .ok_or(anyhow!("pool contract not found"))?;
        self.import_asset_by_commitment_hash(
            commitment.data.chain_id,
            pool_contract,
            commitment.data.commitment_hash.clone(),
            accounts,
        )
        .await?;
        Ok(())
    }
}
