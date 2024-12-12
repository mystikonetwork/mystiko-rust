use crate::handler::SpendContext;
use crate::{
    Accounts, Commitment, CommitmentColumn, CommitmentPoolContractHandler, Scanner, ScannerError, SpendsError,
    WalletHandler,
};
use ethers_providers::Middleware;
use itertools::Itertools;
use mystiko_ethers::Providers;
use mystiko_protos::core::scanner::v1::{AssetSyncOptions, BalanceResult};
use mystiko_protos::data::v1::{Commitment as ProtosCommitment, CommitmentStatus, Nullifier as ProtosNullifier};
use mystiko_protos::sequencer::v1::{FetchChainRequest, FetchChainResponse};
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_sequencer_client::SequencerClient;
use mystiko_storage::{Document, StatementFormatter, Storage};
use prost::Message;
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
    pub(crate) async fn asset_sync(&self, options: AssetSyncOptions) -> Result<BalanceResult, ScannerError> {
        self.wallets.check_password(&options.wallet_password).await?;

        let cms = self.get_commitments().await?;
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

        let cms_failed = self
            .sync_commitment_from_provider(cms_from_provider.values().cloned().collect())
            .await;
        for cm in cms_failed {
            for cm in cms_by_hash.iter() {
                if cm.data.nullifier == cm.data.nullifier {
                    cms_to_update.push(cm.clone());
                }
            }
        }

        unimplemented!("asset_sync")
    }

    async fn sync_commitment_from_provider(&self, commitments: Vec<Document<Commitment>>) -> Vec<Document<Commitment>> {
        unimplemented!("sync_commitment_from_provider")
    }

    async fn sync_by_provider_hash(&self, commitments: Vec<Document<Commitment>>) -> Vec<Document<Commitment>> {
        unimplemented!("sync_commitment_from_provider")
    }

    async fn get_commitments(&self) -> Result<Vec<Document<Commitment>>, ScannerError> {
        let accounts = self.build_filter_accounts(&[]).await?;
        let shielded_addresses = accounts
            .into_iter()
            .map(|account| account.data.shielded_address)
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
}
