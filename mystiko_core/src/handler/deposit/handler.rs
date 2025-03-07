use crate::{format_deposit_log, wrap_filter};
use crate::{
    Accounts, CommitmentPoolContractHandler, CommitmentPoolContracts, Database, Deposit, DepositColumn,
    DepositContractHandler, DepositContracts, DepositHandler, DepositsError, FromContext, MystikoContext, MystikoError,
    PrivateKeySigner, PrivateKeySignerOptions, PublicAssetHandler, PublicAssets, TransactionHandler, TransactionSigner,
    Transactions, WalletHandler, Wallets,
};
use async_trait::async_trait;
use mystiko_config::MystikoConfig;
use mystiko_ethers::Providers;
use mystiko_protos::core::document::v1::Deposit as ProtoDeposit;
use mystiko_protos::core::handler::v1::{
    CreateDepositOptions, DepositQuote, DepositSummary, FixDepositStatusOptions, QuoteDepositOptions,
    SendDepositOptions,
};
use mystiko_protos::core::v1::{DepositStatus, Transaction};
use mystiko_protos::storage::v1::{QueryFilter, SubFilter};
use mystiko_screening_client::v1::ScreeningClientV1;
use mystiko_screening_client::ScreeningClient;
use mystiko_storage::{ColumnValues, StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_to_string;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Deposits<
    F: StatementFormatter,
    S: Storage,
    A = PublicAssets<Box<dyn Providers>>,
    D = DepositContracts<Box<dyn Providers>>,
    C = CommitmentPoolContracts<Box<dyn Providers>>,
    T = Transactions<Box<dyn Providers>>,
    P = Box<dyn Providers>,
    N = Box<dyn ScreeningClient>,
> {
    pub(crate) db: Arc<Database<F, S>>,
    pub(crate) config: Arc<MystikoConfig>,
    pub(crate) wallets: Wallets<F, S>,
    pub(crate) accounts: Accounts<F, S>,
    pub(crate) assets: Arc<A>,
    pub(crate) deposit_contracts: Arc<D>,
    pub(crate) commitment_pool_contracts: Arc<C>,
    pub(crate) transactions: Arc<T>,
    pub(crate) signer_providers: Arc<P>,
    pub(crate) screening: Arc<N>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DepositsOptions<
    F: StatementFormatter,
    S: Storage,
    A = PublicAssets<Box<dyn Providers>>,
    D = DepositContracts<Box<dyn Providers>>,
    C = CommitmentPoolContracts<Box<dyn Providers>>,
    T = Transactions<Box<dyn Providers>>,
    P = Box<dyn Providers>,
    N = Box<dyn ScreeningClient>,
> {
    db: Arc<Database<F, S>>,
    config: Arc<MystikoConfig>,
    assets: Arc<A>,
    deposit_contracts: Arc<D>,
    commitment_pool_contracts: Arc<C>,
    transactions: Arc<T>,
    signer_providers: Arc<P>,
    screening: Arc<N>,
}

type Result<T> = std::result::Result<T, DepositsError>;

#[async_trait]
impl<F, S, A, D, C, T, P, N>
    DepositHandler<
        ProtoDeposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
        FixDepositStatusOptions,
    > for Deposits<F, S, A, D, C, T, P, N>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContractHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    P: Providers + 'static,
    N: ScreeningClient + 'static,
    DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
{
    type Error = DepositsError;

    async fn quote(&self, options: QuoteDepositOptions) -> Result<DepositQuote> {
        self.execute_quote(&options).await
    }

    async fn summary(&self, options: CreateDepositOptions) -> Result<DepositSummary> {
        self.execute_summary(&options).await
    }

    async fn create(&self, options: CreateDepositOptions) -> Result<ProtoDeposit> {
        let deposit = self.execute_create(&options).await?;
        let deposit = self.db.deposits.insert(&deposit).await?;
        log::info!("successfully created a {}", format_deposit_log(&deposit));
        Ok(Deposit::document_into_proto(deposit))
    }

    async fn send(&self, options: SendDepositOptions) -> Result<ProtoDeposit> {
        let private_key = options
            .private_key
            .clone()
            .ok_or(DepositsError::MissingPrivateKeyError)?;
        let signer = PrivateKeySigner::<P>::new(
            PrivateKeySignerOptions::builder()
                .private_key(private_key)
                .providers(self.signer_providers.clone())
                .signer_provider(options.signer_provider.clone())
                .build(),
        )?;
        self.send_with_signer(options, Arc::new(signer)).await
    }

    async fn send_with_signer<Signer>(&self, options: SendDepositOptions, signer: Arc<Signer>) -> Result<ProtoDeposit>
    where
        Signer: TransactionSigner + 'static,
    {
        let mut deposit = self
            .db
            .deposits
            .find_by_id(&options.deposit_id)
            .await?
            .ok_or_else(|| DepositsError::IdNotFoundError(options.deposit_id.clone()))?;
        let owner = signer.address().await?;
        let owner_address = ethers_address_to_string(&owner);
        log::info!(
            "sending {} with signer at address {}",
            format_deposit_log(&deposit),
            owner_address
        );
        match self.execute_send(&options, deposit.clone(), signer, owner).await {
            Err(err) => {
                log::error!("failed to send {}: {}", format_deposit_log(&deposit), err);
                deposit.data.status = DepositStatus::Failed as i32;
                deposit.data.error_message = Some(err.to_string());
                self.db.deposits.update(&deposit).await?;
                Err(err)
            }
            Ok(deposit) => {
                log::info!("successfully sent {}", format_deposit_log(&deposit));
                Ok(Deposit::document_into_proto(deposit))
            }
        }
    }

    async fn fix_status(&self, options: FixDepositStatusOptions) -> Result<ProtoDeposit> {
        let deposit = self.execute_fix_status(options).await?;
        Ok(Deposit::document_into_proto(deposit))
    }

    async fn find<Filter>(&self, filter: Filter) -> Result<Vec<ProtoDeposit>>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .deposits
            .find(wrap_filter::<Filter, _, _>(
                filter,
                DepositColumn::WalletId,
                wallet.id.clone(),
            ))
            .await?
            .into_iter()
            .map(Deposit::document_into_proto)
            .collect::<Vec<_>>())
    }

    async fn find_all(&self) -> Result<Vec<ProtoDeposit>> {
        let wallet = self.wallets.check_current().await?;
        let filter = SubFilter::equal(DepositColumn::WalletId, wallet.id);
        Ok(self
            .db
            .deposits
            .find(filter)
            .await?
            .into_iter()
            .map(Deposit::document_into_proto)
            .collect::<Vec<_>>())
    }

    async fn find_one<Filter>(&self, filter: Filter) -> Result<Option<ProtoDeposit>>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .deposits
            .find_one(wrap_filter::<Filter, _, _>(
                filter,
                DepositColumn::WalletId,
                wallet.id.clone(),
            ))
            .await?
            .map(Deposit::document_into_proto))
    }

    async fn find_by_id(&self, id: String) -> Result<Option<ProtoDeposit>> {
        Ok(self
            .db
            .deposits
            .find_by_id(&id)
            .await?
            .map(Deposit::document_into_proto))
    }

    async fn count<Filter>(&self, filter: Filter) -> Result<u64>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .deposits
            .count(wrap_filter::<Filter, _, _>(
                filter,
                DepositColumn::WalletId,
                wallet.id.clone(),
            ))
            .await?)
    }

    async fn count_all(&self) -> Result<u64> {
        let wallet = self.wallets.check_current().await?;
        let filter = SubFilter::equal(DepositColumn::WalletId, wallet.id);
        Ok(self.db.deposits.count(filter).await?)
    }

    async fn update(&self, deposit: ProtoDeposit) -> Result<ProtoDeposit> {
        Ok(self
            .db
            .deposits
            .update(&Deposit::document_from_proto(deposit)?)
            .await
            .map(Deposit::document_into_proto)?)
    }

    async fn update_batch(&self, deposits: Vec<ProtoDeposit>) -> Result<Vec<ProtoDeposit>> {
        let deposits = deposits
            .into_iter()
            .map(Deposit::document_from_proto)
            .collect::<core::result::Result<Vec<_>, _>>()?;
        let deposits = self.db.deposits.update_batch(&deposits).await?;
        Ok(deposits
            .into_iter()
            .map(Deposit::document_into_proto)
            .collect::<Vec<_>>())
    }

    async fn update_by_filter<Filter, Values>(&self, column_values: Values, filter: Filter) -> Result<()>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
        Values: Into<ColumnValues> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .deposits
            .update_by_filter(
                column_values,
                wrap_filter::<Filter, _, _>(filter, DepositColumn::WalletId, wallet.id.clone()),
            )
            .await?)
    }

    async fn update_all<Values>(&self, column_values: Values) -> Result<()>
    where
        Values: Into<ColumnValues> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        let filter = SubFilter::equal(DepositColumn::WalletId, wallet.id);
        Ok(self.db.deposits.update_by_filter(column_values, filter).await?)
    }

    async fn delete(&self, deposit: ProtoDeposit) -> Result<()> {
        Ok(self.db.deposits.delete(&Deposit::document_from_proto(deposit)?).await?)
    }

    async fn delete_batch(&self, deposits: Vec<ProtoDeposit>) -> Result<()> {
        let deposits = deposits
            .into_iter()
            .map(Deposit::document_from_proto)
            .collect::<core::result::Result<Vec<_>, _>>()?;
        Ok(self.db.deposits.delete_batch(&deposits).await?)
    }

    async fn delete_by_filter<Filter>(&self, filter: Filter) -> Result<()>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .deposits
            .delete_by_filter(wrap_filter::<Filter, _, _>(
                filter,
                DepositColumn::WalletId,
                wallet.id.clone(),
            ))
            .await?)
    }

    async fn delete_all(&self) -> Result<()> {
        let wallet = self.wallets.check_current().await?;
        let filter = SubFilter::equal(DepositColumn::WalletId, wallet.id);
        Ok(self.db.deposits.delete_by_filter(filter).await?)
    }
}

impl<F, S, A, D, C, T, P, N> Deposits<F, S, A, D, C, T, P, N>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContractHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
{
    pub fn new(options: DepositsOptions<F, S, A, D, C, T, P, N>) -> Self {
        let wallets = Wallets::new(options.db.clone());
        let accounts = Accounts::new(options.db.clone());
        Self::builder()
            .db(options.db)
            .config(options.config)
            .signer_providers(options.signer_providers)
            .wallets(wallets)
            .accounts(accounts)
            .assets(options.assets)
            .deposit_contracts(options.deposit_contracts)
            .commitment_pool_contracts(options.commitment_pool_contracts)
            .transactions(options.transactions)
            .screening(options.screening)
            .build()
    }
}

#[async_trait]
impl<F, S, A, D, C, T> FromContext<F, S> for Deposits<F, S, A, D, C, T>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler + FromContext<F, S>,
    D: DepositContractHandler + FromContext<F, S>,
    C: CommitmentPoolContractHandler + FromContext<F, S>,
    T: TransactionHandler<Transaction> + FromContext<F, S>,
    DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
{
    async fn from_context(context: &MystikoContext<F, S>) -> std::result::Result<Self, MystikoError> {
        let screening = ScreeningClientV1::new(context.screening_client_options.clone());
        let screening = Arc::new(Box::new(screening) as Box<dyn ScreeningClient>);
        let options = DepositsOptions::<F, S, A, D, C, T>::builder()
            .db(context.db.clone())
            .config(context.config.clone())
            .signer_providers(context.signer_providers.clone())
            .assets(A::from_context(context).await?)
            .deposit_contracts(D::from_context(context).await?)
            .commitment_pool_contracts(C::from_context(context).await?)
            .transactions(T::from_context(context).await?)
            .screening(screening)
            .build();
        Ok(Self::new(options))
    }
}
