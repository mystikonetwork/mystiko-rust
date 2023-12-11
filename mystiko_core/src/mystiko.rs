use crate::{
    Accounts, Database, Deposits, FromContext, MystikoContext, MystikoError, MystikoOptions, Scanner, Spends,
    Synchronizer, Wallets,
};
use anyhow::Result;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::loader::ChainDataLoader;
use mystiko_storage::{StatementFormatter, Storage};
use std::sync::Arc;

pub struct Mystiko<
    F: StatementFormatter,
    S: Storage,
    W = Wallets<F, S>,
    A = Accounts<F, S>,
    D = Deposits<F, S>,
    X = Spends<F, S>,
    Y = Synchronizer<ChainDataLoader<FullData>>,
    R = Scanner<F, S>,
> {
    pub db: Arc<Database<F, S>>,
    pub config: Arc<MystikoConfig>,
    pub wallets: W,
    pub accounts: A,
    pub deposits: D,
    pub spends: X,
    pub synchronizer: Y,
    pub scanner: R,
}

impl<F, S, W, A, D, X, Y, R> Mystiko<F, S, W, A, D, X, Y, R>
where
    F: StatementFormatter + 'static,
    S: Storage + 'static,
    W: FromContext<F, S> + 'static,
    A: FromContext<F, S> + 'static,
    D: FromContext<F, S> + 'static,
    X: FromContext<F, S> + 'static,
    Y: FromContext<F, S> + 'static,
    R: FromContext<F, S> + 'static,
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
            spends: X::from_context(&context).await?,
            synchronizer: Y::from_context(&context).await?,
            scanner: R::from_context(&context).await?,
        };
        log::info!(
            "mystiko on {} has been initialized, config git revision {}",
            context.network,
            context.config.git_revision().unwrap_or("unknown")
        );
        Ok(mystiko)
    }
}
