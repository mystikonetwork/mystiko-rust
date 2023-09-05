#![forbid(unsafe_code)]

use crate::document::AccountCollection;
use crate::document::ChainCollection;
use crate::document::CommitmentCollection;
use crate::document::ContractCollection;
use crate::document::DepositCollection;
use crate::document::NullifierCollection;
use crate::document::TransactionCollection;
use crate::document::WalletCollection;
use anyhow::Result;
use mystiko_storage::{Collection, Document, MigrationHistory, StatementFormatter, Storage};
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
