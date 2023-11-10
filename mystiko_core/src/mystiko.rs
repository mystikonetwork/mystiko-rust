use crate::{
    AccountHandler, Accounts, Database, DepositHandler, Deposits, FromContext, MystikoContext, MystikoError,
    MystikoOptions, Synchronizer, SynchronizerHandler, WalletHandler, Wallets,
};
use anyhow::Result;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::loader::ChainDataLoader;
use mystiko_protos::core::document::v1::{Account, Deposit, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateDepositOptions, CreateWalletOptions, DepositQuote, DepositSummary, QuoteDepositOptions,
    SendDepositOptions, UpdateAccountOptions,
};
use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerStatus};
use mystiko_storage::{StatementFormatter, Storage};
use std::sync::Arc;

pub struct Mystiko<
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions> = Wallets<F, S>,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions> = Accounts<F, S>,
    D: DepositHandler<
        Deposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
    > = Deposits<F, S>,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus> = Synchronizer<ChainDataLoader<FullData>>,
> {
    pub db: Arc<Database<F, S>>,
    pub config: Arc<MystikoConfig>,
    pub wallets: W,
    pub accounts: A,
    pub deposits: D,
    pub synchronizer: Y,
}

impl<F, S, W, A, D, Y> Mystiko<F, S, W, A, D, Y>
where
    F: StatementFormatter + 'static,
    S: Storage + 'static,
    W: FromContext<F, S> + WalletHandler<Wallet, CreateWalletOptions> + 'static,
    A: FromContext<F, S> + AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions> + 'static,
    D: FromContext<F, S>
        + DepositHandler<
            Deposit,
            QuoteDepositOptions,
            DepositQuote,
            CreateDepositOptions,
            DepositSummary,
            SendDepositOptions,
        > + 'static,
    Y: FromContext<F, S> + SynchronizerHandler<SyncOptions, SynchronizerStatus> + 'static,
{
    pub async fn new(database: Database<F, S>, options: Option<MystikoOptions>) -> Result<Self, MystikoError> {
        database.migrate().await.map_err(MystikoError::DatabaseMigrationError)?;
        let context = MystikoContext::new(database, options).await?;
        let mystiko = Self {
            db: context.db.clone(),
            config: context.config.clone(),
            accounts: A::from_context(&context).await?,
            wallets: W::from_context(&context).await?,
            deposits: D::from_context(&context).await?,
            synchronizer: Y::from_context(&context).await?,
        };
        log::info!(
            "mystiko on {} has been initialized, config git revision {}",
            context.network,
            context.config.git_revision().unwrap_or("unknown")
        );
        Ok(mystiko)
    }
}
