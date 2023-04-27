#![forbid(unsafe_code)]

use crate::collection::account::AccountCollection;
use crate::collection::chain::ChainCollection;
use crate::collection::commitment::CommitmentCollection;
use crate::collection::contract::ContractCollection;
use crate::collection::deposit::DepositCollection;
use crate::collection::nullifier::NullifierCollection;
use crate::collection::transaction::TransactionCollection;
use crate::collection::wallet::WalletCollection;
use anyhow::Result;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentRawData};
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::sync::Arc;

#[derive(Debug)]
pub struct Database<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    pub accounts: AccountCollection<F, R, S>,
    pub chains: ChainCollection<F, R, S>,
    pub commitments: CommitmentCollection<F, R, S>,
    pub contracts: ContractCollection<F, R, S>,
    pub deposits: DepositCollection<F, R, S>,
    pub nullifiers: NullifierCollection<F, R, S>,
    pub transactions: TransactionCollection<F, R, S>,
    pub wallets: WalletCollection<F, R, S>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> Database<F, R, S> {
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

    pub async fn migrate(&self) -> Result<Vec<Document<Migration>>> {
        let migrations: Vec<Document<Migration>> = vec![
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
