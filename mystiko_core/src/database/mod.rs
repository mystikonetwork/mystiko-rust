mod account;
mod commitment;
mod contract;
mod deposit;
mod nullifier;
mod spend;
mod wallet;

pub use account::*;
pub use commitment::*;
pub use contract::*;
pub use deposit::*;
pub use nullifier::*;
pub use spend::*;
pub use wallet::*;

use anyhow::Result;
use mystiko_storage::{Collection, Document, MigrationHistory, StatementFormatter, Storage};
use std::sync::Arc;

#[derive(Debug)]
pub struct Database<F: StatementFormatter, S: Storage> {
    pub accounts: AccountCollection<F, S>,
    pub commitments: CommitmentCollection<F, S>,
    pub contracts: ContractCollection<F, S>,
    pub deposits: DepositCollection<F, S>,
    pub nullifiers: NullifierCollection<F, S>,
    pub spends: SpendCollection<F, S>,
    pub wallets: WalletCollection<F, S>,
}

impl<F: StatementFormatter, S: Storage> Database<F, S> {
    pub fn new(formatter: F, storage: S) -> Self {
        let collection = Arc::new(Collection::new(formatter, storage));
        Database {
            accounts: AccountCollection::new(collection.clone()),
            commitments: CommitmentCollection::new(collection.clone()),
            contracts: ContractCollection::new(collection.clone()),
            deposits: DepositCollection::new(collection.clone()),
            nullifiers: NullifierCollection::new(collection.clone()),
            spends: SpendCollection::new(collection.clone()),
            wallets: WalletCollection::new(collection),
        }
    }

    pub async fn migrate(&self) -> Result<Vec<Document<MigrationHistory>>> {
        let migrations: Vec<Document<MigrationHistory>> = vec![
            self.accounts.migrate().await?,
            self.commitments.migrate().await?,
            self.contracts.migrate().await?,
            self.deposits.migrate().await?,
            self.nullifiers.migrate().await?,
            self.spends.migrate().await?,
            self.wallets.migrate().await?,
        ];
        Ok(migrations)
    }
}
