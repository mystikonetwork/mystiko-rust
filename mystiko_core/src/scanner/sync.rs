use crate::scanner::scan::ScanningAccount;
use crate::{
    Commitment, CommitmentColumn, CommitmentPoolContractHandler, IncludedCountOptions, IsSpentNullifierOptions,
    Scanner, ScannerError, ScannerHandler, WalletHandler,
};
use anyhow::anyhow;
use log::error;
use mystiko_ethers::Providers;
use mystiko_protos::core::scanner::v1::{BalanceOptions, BalanceResult, SyncOptions};
use mystiko_protos::data::v1::{Commitment as ProtosCommitment, CommitmentStatus, Nullifier as ProtosNullifier};
use mystiko_protos::sequencer::v1::{FetchChainRequest, FetchChainResponse};
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_sequencer_client::SequencerClient;
use mystiko_storage::{Document, StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::biguint_to_u256;
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
    pub(crate) async fn asset_sync(&self, options: SyncOptions) -> Result<BalanceResult, ScannerError> {
        self.wallets.check_password(&options.wallet_password).await?;
        let accounts = self.build_filter_accounts(&[]).await?;
        let scan_accounts = self.build_scan_accounts(&accounts, &options.wallet_password).await?;
        let cms = self.get_scan_commitments(&scan_accounts).await?;
        let mut cms_from_provider: HashMap<ImportFromProviderKey, Document<Commitment>> = HashMap::new();
        let mut cms_by_hash = Vec::new();
        let mut cms_to_update = Vec::new();
        for cm in cms.into_iter() {
            let cm_data = &cm.data;
            if cm_data.leaf_index.is_none() {
                if cm_data.leaf_index.is_none() {
                    if let Some(queued_transaction_hash) = cm_data.queued_transaction_hash.clone() {
                        cms_from_provider.insert(
                            ImportFromProviderKey::builder()
                                .chain_id(cm_data.chain_id)
                                .queued_transaction_hash(queued_transaction_hash)
                                .build(),
                            cm,
                        );
                    } else {
                        cms_by_hash.push(cm);
                    }
                }
            } else {
                cms_to_update.push(cm);
            }
        }

        if !cms_from_provider.is_empty() {
            let failed_cms = self
                .sync_commitments_from_provider(&options, &scan_accounts, cms_from_provider.values().cloned().collect())
                .await;
            for cm in failed_cms {
                if cms_by_hash.iter().any(|c| {
                    c.data.chain_id == cm.data.chain_id
                        && c.data.contract_address == cm.data.contract_address
                        && c.data.commitment_hash == cm.data.commitment_hash
                }) {
                    cms_by_hash.push(cm);
                }
            }
        }

        if !cms_by_hash.is_empty() {
            self.sync_by_commitment_hash(&options, &scan_accounts, cms_by_hash)
                .await;
        }

        if !cms_to_update.is_empty() {
            self.sync_commitment_status(&options, cms_to_update).await;
        }

        self.balance(BalanceOptions::builder().build()).await
    }

    async fn get_scan_commitments(
        &self,
        accounts: &[ScanningAccount],
    ) -> Result<Vec<Document<Commitment>>, ScannerError> {
        let shielded_addresses = accounts
            .iter()
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
        for commitment in commitments.iter() {
            let result = self.sync_one_from_provider(accounts, commitment).await;
            if result.map_or(false, |r| !r) {
                failed_cms.push(commitment.clone());
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
            if !result.is_empty() {
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

    async fn sync_commitment_status(&self, options: &SyncOptions, commitments: Vec<Document<Commitment>>) {
        let concurrency = std::cmp::max(1, options.concurrency.unwrap_or_default()) as usize;
        let chunk_nums = commitments.len().div_ceil(concurrency);
        let chunks = commitments.chunks(chunk_nums);
        let mut group_task = Vec::with_capacity(chunks.len());
        for chunk in chunks {
            group_task.push(self.sync_batch_commitment_status(chunk));
        }
        futures::future::join_all(group_task).await;
    }

    async fn sync_batch_commitment_status(&self, commitments: &[Document<Commitment>]) {
        for cm in commitments.iter() {
            let _ = self.sync_one_commitment_status(cm).await.map_err(|e| {
                error!("sync commitment status error: {}", e);
            });
        }
    }

    async fn sync_one_commitment_status(&self, commitment: &Document<Commitment>) -> Result<(), ScannerError> {
        let chain_id = commitment.data.chain_id;
        let contract_address = ethers_address_from_string(commitment.data.contract_address.clone())?;
        let leaf_index = commitment.data.leaf_index.ok_or(anyhow!("leaf index not found"))?;
        let mut commitment = commitment.data.clone();
        let mut status_changed = false;
        if let Some(ref nullifier) = commitment.nullifier {
            let spend = self
                .commitment_pool_contracts
                .is_spent_nullifier(
                    IsSpentNullifierOptions::builder()
                        .chain_id(chain_id)
                        .contract_address(contract_address)
                        .nullifier(biguint_to_u256(nullifier))
                        .build(),
                )
                .await?;
            if spend {
                commitment.status = CommitmentStatus::Included as i32;
                commitment.spent = true;
                status_changed = true;
            }
        }

        if !status_changed && commitment.status == CommitmentStatus::Queued as i32 {
            let included_count = self
                .commitment_pool_contracts
                .get_commitment_included_count(
                    IncludedCountOptions::builder()
                        .chain_id(chain_id)
                        .contract_address(contract_address)
                        .build(),
                )
                .await?
                .as_u64();
            if leaf_index.lt(&included_count) {
                commitment.status = CommitmentStatus::Included as i32;
                status_changed = true;
            }
        }

        if status_changed {
            self.update_or_insert_commitment(commitment).await?;
        }

        Ok(())
    }
}
