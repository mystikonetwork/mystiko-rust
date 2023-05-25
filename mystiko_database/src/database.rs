#![forbid(unsafe_code)]

use crate::document::account::AccountCollection;
use crate::document::chain::ChainCollection;
use crate::document::commitment::CommitmentCollection;
use crate::document::contract::ContractCollection;
use crate::document::deposit::DepositCollection;
use crate::document::nullifier::NullifierCollection;
use crate::document::transaction::TransactionCollection;
use crate::document::wallet::WalletCollection;
use anyhow::Result;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::formatter::types::StatementFormatter;
use mystiko_storage::migration::history::MigrationHistory;
use mystiko_storage::storage::Storage;
use std::sync::Arc;

#[derive(Debug)]
pub struct Database<F: StatementFormatter, S: Storage> {
    pub accounts: AccountCollection<F, S>,
    pub chains: ChainCollection<F, S>,
    pub commitments: CommitmentCollection<F, S>,
    pub contracts: ContractCollection<F, S>,
    pub deposits: DepositCollection<F, S>,
    pub nullifiers: NullifierCollection<F, S>,
    pub transactions: TransactionCollection<F, S>,
    pub wallets: WalletCollection<F, S>,
}

impl<F: StatementFormatter, S: Storage> Database<F, S> {
    pub fn new(formatter: F, storage: S) -> Self {
        let collection = Arc::new(Collection::new(formatter, storage));
        Database {
            accounts: AccountCollection::new(collection.clone()),
            chains: ChainCollection::new(collection.clone()),
            commitments: CommitmentCollection::new(collection.clone()),
            contracts: ContractCollection::new(collection.clone()),
            deposits: DepositCollection::new(collection.clone()),
            nullifiers: NullifierCollection::new(collection.clone()),
            transactions: TransactionCollection::new(collection.clone()),
            wallets: WalletCollection::new(collection),
        }
    }

    pub async fn migrate(&self) -> Result<Vec<Document<MigrationHistory>>> {
        let migrations: Vec<Document<MigrationHistory>> = vec![
            self.accounts.migrate().await?,
            self.chains.migrate().await?,
            self.commitments.migrate().await?,
            self.contracts.migrate().await?,
            self.deposits.migrate().await?,
            self.nullifiers.migrate().await?,
            self.transactions.migrate().await?,
            self.wallets.migrate().await?,
        ];
        Ok(migrations)
    }
}
