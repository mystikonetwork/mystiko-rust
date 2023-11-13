use crate::{
    BalanceOptions, CommitmentPoolContractHandler, CommitmentPoolContracts, CommitmentPoolContractsError, Database,
    Deposit, DepositColumn, DepositContractHandler, DepositContracts, DepositContractsError, DepositHandler,
    Erc20ApproveOptions, Erc20BalanceOptions, FromContext, IsHistoricCommitmentOptions, MystikoContext, MystikoError,
    PrivateKeySigner, PrivateKeySignerOptions, PublicAssetHandler, PublicAssets, PublicAssetsError, TransactionHandler,
    TransactionSigner, Transactions, TransactionsError, WaitOptions, WalletHandler, Wallets, WalletsError,
};
use async_trait::async_trait;
use ethers_core::abi::AbiEncode;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, Bytes, TxHash, U128, U256};
use mystiko_config::{AssetConfig, ChainConfig, DepositContractConfig, MystikoConfig};
use mystiko_ethers::Providers;
use mystiko_protocol::error::ProtocolError;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::{Deposit as ProtoDeposit, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateDepositOptions, DepositQuote, DepositSummary, QuoteDepositOptions, SendDepositOptions,
};
use mystiko_protos::core::v1::{DepositStatus, Transaction};
use mystiko_protos::storage::v1::{QueryFilter, SubFilter};
use mystiko_storage::{ColumnValues, Document, StatementFormatter, Storage, StorageError};
use mystiko_utils::address::{ethers_address_from_string, ethers_address_to_string};
use mystiko_utils::convert::{bytes_to_biguint, decimal_to_number, number_to_u256_decimal, u256_to_biguint};
use mystiko_utils::hex::encode_hex_with_prefix;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Deposits<
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler = PublicAssets<Box<dyn Providers>>,
    D: DepositContractHandler = DepositContracts<Box<dyn Providers>>,
    C: CommitmentPoolContractHandler = CommitmentPoolContracts<Box<dyn Providers>>,
    T: TransactionHandler<Transaction> = Transactions<Box<dyn Providers>>,
    P: Providers = Box<dyn Providers>,
> {
    db: Arc<Database<F, S>>,
    config: Arc<MystikoConfig>,
    wallets: Wallets<F, S>,
    assets: Arc<A>,
    deposit_contracts: Arc<D>,
    commitment_pool_contracts: Arc<C>,
    transactions: Arc<T>,
    signer_providers: Arc<P>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DepositsOptions<
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler = PublicAssets<Box<dyn Providers>>,
    D: DepositContractHandler = DepositContracts<Box<dyn Providers>>,
    C: CommitmentPoolContractHandler = CommitmentPoolContracts<Box<dyn Providers>>,
    T: TransactionHandler<Transaction> = Transactions<Box<dyn Providers>>,
    P: Providers = Box<dyn Providers>,
> {
    db: Arc<Database<F, S>>,
    config: Arc<MystikoConfig>,
    assets: Arc<A>,
    deposit_contracts: Arc<D>,
    commitment_pool_contracts: Arc<C>,
    transactions: Arc<T>,
    signer_providers: Arc<P>,
}

#[derive(Debug, Error)]
pub enum DepositsError {
    #[error(transparent)]
    HexStringError(#[from] rustc_hex::FromHexError),
    #[error(transparent)]
    FromDecStrError(#[from] ethers_core::abi::ethereum_types::FromDecStrErr),
    #[error(transparent)]
    ParseBytesError(#[from] ethers_core::types::ParseBytesError),
    #[error(transparent)]
    ProviderError(#[from] ethers_providers::ProviderError),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    PublicAssetsError(#[from] PublicAssetsError),
    #[error(transparent)]
    DepositContractsError(#[from] DepositContractsError),
    #[error(transparent)]
    CommitmentPoolContractsError(#[from] CommitmentPoolContractsError),
    #[error(transparent)]
    TransactionsError(#[from] TransactionsError),
    #[error(transparent)]
    ProtocolError(#[from] ProtocolError),
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    WalletsError(#[from] WalletsError),
    #[error(transparent)]
    ParseBigIntError(#[from] num_bigint::ParseBigIntError),
    #[error("unsupported chain_id={0}")]
    UnsupportedChainIdError(u64),
    #[error("no deposit contract found for chain_id={0} asset_symbol={1}, dst_chain_id={2:?}, bridge_type={3:?}")]
    NoDepositContractFoundError(u64, String, u64, mystiko_types::BridgeType),
    #[error("deposit amount {0} is less than min_amount {1} or greater than max_amount {2}")]
    InvalidDepositAmountError(f64, f64, f64),
    #[error("rollup fee amount {0} is less than min_rollup_fee_amount {1}")]
    InvalidRollupFeeAmountError(f64, f64),
    #[error("bridge fee amount {0} is less than min_bridge_fee_amount {1}")]
    InvalidBridgeFeeAmountError(f64, f64),
    #[error("executor fee amount {0} is less than min_executor_fee_amount {1}")]
    InvalidExecutorFeeAmountError(f64, f64),
    #[error("insufficient balance for asset {0} amount {1}")]
    InsufficientBalanceError(String, f64),
    #[error("deposit with id {0} not found")]
    IdNotFoundError(String),
    #[error("missing private key")]
    MissingPrivateKeyError,
    #[error("cannot send deposit with status={0}")]
    DepositStatusError(String),
    #[error("duplicate commitment={0} in chain_id={1} contract_address={2}")]
    DuplicateCommitmentError(String, u64, String),
}

type Result<T> = std::result::Result<T, DepositsError>;

#[async_trait]
impl<F, S, A, D, C, T, P>
    DepositHandler<
        ProtoDeposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
    > for Deposits<F, S, A, D, C, T, P>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContractHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    P: Providers + 'static,
    DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
{
    type Error = DepositsError;

    async fn quote(&self, options: QuoteDepositOptions) -> Result<DepositQuote> {
        let context = DepositContext::from_quote_options(
            self.config.clone(),
            self.assets.clone(),
            self.deposit_contracts.clone(),
            self.commitment_pool_contracts.clone(),
            self.transactions.clone(),
            &options,
        )?;
        context.quote().await
    }

    async fn summary(&self, options: CreateDepositOptions) -> Result<DepositSummary> {
        let context = DepositContext::from_create_options(
            self.config.clone(),
            self.assets.clone(),
            self.deposit_contracts.clone(),
            self.commitment_pool_contracts.clone(),
            self.transactions.clone(),
            &options,
        )?;
        context.summary(&options).await
    }

    async fn create(&self, options: CreateDepositOptions) -> Result<ProtoDeposit> {
        let wallet = self.wallets.check_current().await?;
        let context = DepositContext::from_create_options(
            self.config.clone(),
            self.assets.clone(),
            self.deposit_contracts.clone(),
            self.commitment_pool_contracts.clone(),
            self.transactions.clone(),
            &options,
        )?;
        let deposit = context.create_deposit(&options, wallet.id).await?;
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
        match self.send_transaction(&options, deposit.clone(), signer, owner).await {
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

    async fn find<Filter>(&self, filter: Filter) -> Result<Vec<ProtoDeposit>>
    where
        Filter: Into<QueryFilter> + Send + Sync,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .deposits
            .find(wrap_filter::<Filter>(filter, &wallet))
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
        Filter: Into<QueryFilter> + Send + Sync,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .deposits
            .find_one(wrap_filter::<Filter>(filter, &wallet))
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
        Filter: Into<QueryFilter> + Send + Sync,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self.db.deposits.count(wrap_filter::<Filter>(filter, &wallet)).await?)
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
            .update(&Deposit::document_from_proto(deposit))
            .await
            .map(Deposit::document_into_proto)?)
    }

    async fn update_batch(&self, deposits: Vec<ProtoDeposit>) -> Result<Vec<ProtoDeposit>> {
        let deposits = deposits
            .into_iter()
            .map(Deposit::document_from_proto)
            .collect::<Vec<_>>();
        let deposits = self.db.deposits.update_batch(&deposits).await?;
        Ok(deposits
            .into_iter()
            .map(Deposit::document_into_proto)
            .collect::<Vec<_>>())
    }

    async fn update_by_filter<Filter, Values>(&self, column_values: Values, filter: Filter) -> Result<()>
    where
        Filter: Into<QueryFilter> + Send + Sync,
        Values: Into<ColumnValues> + Send + Sync,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .deposits
            .update_by_filter(column_values, wrap_filter::<Filter>(filter, &wallet))
            .await?)
    }

    async fn update_all<Values>(&self, column_values: Values) -> Result<()>
    where
        Values: Into<ColumnValues> + Send + Sync,
    {
        let wallet = self.wallets.check_current().await?;
        let filter = SubFilter::equal(DepositColumn::WalletId, wallet.id);
        Ok(self.db.deposits.update_by_filter(column_values, filter).await?)
    }

    async fn delete(&self, deposit: ProtoDeposit) -> Result<()> {
        Ok(self.db.deposits.delete(&Deposit::document_from_proto(deposit)).await?)
    }

    async fn delete_batch(&self, deposits: Vec<ProtoDeposit>) -> Result<()> {
        let deposits = deposits
            .into_iter()
            .map(Deposit::document_from_proto)
            .collect::<Vec<_>>();
        Ok(self.db.deposits.delete_batch(&deposits).await?)
    }

    async fn delete_by_filter<Filter>(&self, filter: Filter) -> Result<()>
    where
        Filter: Into<QueryFilter> + Send + Sync,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .deposits
            .delete_by_filter(wrap_filter::<Filter>(filter, &wallet))
            .await?)
    }

    async fn delete_all(&self) -> Result<()> {
        let wallet = self.wallets.check_current().await?;
        let filter = SubFilter::equal(DepositColumn::WalletId, wallet.id);
        Ok(self.db.deposits.delete_by_filter(filter).await?)
    }
}

impl<F, S, A, D, C, T, P> Deposits<F, S, A, D, C, T, P>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContractHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    P: Providers + 'static,
    DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
{
    pub fn new(options: DepositsOptions<F, S, A, D, C, T, P>) -> Self {
        let wallets = Wallets::new(options.db.clone());
        Self::builder()
            .db(options.db)
            .config(options.config)
            .signer_providers(options.signer_providers)
            .wallets(wallets)
            .assets(options.assets)
            .deposit_contracts(options.deposit_contracts)
            .commitment_pool_contracts(options.commitment_pool_contracts)
            .transactions(options.transactions)
            .build()
    }

    async fn send_transaction<Signer>(
        &self,
        options: &SendDepositOptions,
        deposit: Document<Deposit>,
        signer: Arc<Signer>,
        owner: Address,
    ) -> Result<Document<Deposit>>
    where
        Signer: TransactionSigner + 'static,
    {
        let context = DepositContext::from_send_options(
            self.config.clone(),
            self.assets.clone(),
            self.deposit_contracts.clone(),
            self.commitment_pool_contracts.clone(),
            self.transactions.clone(),
            &deposit,
            options,
        )?;
        context.validate_deposit(&deposit, options).await?;
        let deposit = self
            .send_assets_approve(options, deposit, &context, signer.clone(), owner)
            .await?;
        let deposit = self.send_deposit(options, deposit, &context, signer.clone()).await?;
        Ok(deposit)
    }

    async fn send_assets_approve<Signer>(
        &self,
        options: &SendDepositOptions,
        mut deposit: Document<Deposit>,
        context: &DepositContext<A, D, C, T>,
        signer: Arc<Signer>,
        owner: Address,
    ) -> Result<Document<Deposit>>
    where
        Signer: TransactionSigner + 'static,
    {
        context.validate_balances(&owner).await?;
        let asset_approve_tx_hashes = context.send_assets_approve(options, signer.clone(), owner).await?;
        if !asset_approve_tx_hashes.is_empty() {
            deposit.data.status = DepositStatus::AssetApproving as i32;
            deposit.data.asset_approve_transaction_hash = Some(
                asset_approve_tx_hashes
                    .iter()
                    .map(|(_, tx_hash)| tx_hash.encode_hex())
                    .collect::<Vec<_>>(),
            );
            deposit = self.db.deposits.update(&deposit).await?;
        }
        for (asset_symbol, tx_hash) in asset_approve_tx_hashes.iter() {
            log::info!(
                "successfully submitted {} approving {} for {}",
                asset_symbol,
                format_tx(self.config.clone(), deposit.data.chain_id, tx_hash)?,
                format_deposit_log(&deposit),
            );
            let wait_options = WaitOptions::builder()
                .chain_id(deposit.data.chain_id)
                .tx_hash(*tx_hash)
                .confirmations(options.asset_approve_confirmations)
                .interval_ms(options.tx_wait_interval_ms)
                .timeout_ms(options.tx_wait_timeout_ms)
                .build();
            self.transactions.wait(wait_options).await?;
        }
        deposit.data.status = DepositStatus::AssetApproved as i32;
        let deposit = self.db.deposits.update(&deposit).await?;
        for (asset_symbol, tx_hash) in asset_approve_tx_hashes.iter() {
            log::info!(
                "successfully confirmed {} approving {} for {}",
                asset_symbol,
                format_tx(self.config.clone(), deposit.data.chain_id, tx_hash)?,
                format_deposit_log(&deposit),
            )
        }
        Ok(deposit)
    }

    async fn send_deposit<Signer>(
        &self,
        options: &SendDepositOptions,
        mut deposit: Document<Deposit>,
        context: &DepositContext<A, D, C, T>,
        signer: Arc<Signer>,
    ) -> Result<Document<Deposit>>
    where
        Signer: TransactionSigner + 'static,
    {
        let send_tx_hash = context.send_deposit(&mut deposit, options, signer.clone()).await?;
        let send_tx_url = format_tx(self.config.clone(), deposit.data.chain_id, &send_tx_hash)?;
        deposit = self.db.deposits.update(&deposit).await?;
        log::info!(
            "successfully submitted {} for {}",
            send_tx_url,
            format_deposit_log(&deposit),
        );
        let wait_options = WaitOptions::builder()
            .chain_id(deposit.data.chain_id)
            .tx_hash(send_tx_hash)
            .confirmations(options.deposit_confirmations)
            .interval_ms(options.tx_wait_interval_ms)
            .timeout_ms(options.tx_wait_timeout_ms)
            .build();
        self.transactions.wait(wait_options).await?;
        if context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop {
            deposit.data.status = DepositStatus::SrcSucceeded as i32;
        } else {
            deposit.data.status = DepositStatus::Queued as i32;
        }
        deposit = self.db.deposits.update(&deposit).await?;
        log::info!(
            "successfully confirmed {} for {}",
            send_tx_url,
            format_deposit_log(&deposit),
        );
        Ok(deposit)
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
        let options = DepositsOptions::<F, S, A, D, C, T>::builder()
            .db(context.db.clone())
            .config(context.config.clone())
            .signer_providers(context.signer_providers.clone())
            .assets(A::from_context(context).await?)
            .deposit_contracts(D::from_context(context).await?)
            .commitment_pool_contracts(C::from_context(context).await?)
            .transactions(T::from_context(context).await?)
            .build();
        Ok(Self::new(options))
    }
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct AssetContext {
    pub(crate) chain_id: u64,
    pub(crate) asset_config: AssetConfig,
    pub(crate) amount: f64,
    pub(crate) converted_amount: U256,
    pub(crate) query_timeout_ms: Option<u64>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct DepositContext<
    A: PublicAssetHandler,
    D: DepositContractHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
> {
    pub(crate) chain_id: u64,
    pub(crate) chain_config: ChainConfig,
    pub(crate) contract_config: DepositContractConfig,
    pub(crate) assets: Arc<A>,
    pub(crate) deposit_contracts: Arc<D>,
    pub(crate) commitment_pool_contracts: Arc<C>,
    pub(crate) transactions: Arc<T>,
    #[builder(default)]
    pub(crate) peer_contract_config: Option<DepositContractConfig>,
    #[builder(default)]
    pub(crate) quote: Option<DepositQuote>,
    #[builder(default)]
    pub(crate) query_timeout_ms: Option<u64>,
    #[builder(default)]
    pub(crate) deposit_assets: HashMap<String, AssetContext>,
}

impl<A, D, C, T> DepositContext<A, D, C, T>
where
    A: PublicAssetHandler,
    D: DepositContractHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
{
    pub(crate) async fn quote(&self) -> Result<DepositQuote> {
        let quote_options = crate::DepositQuoteOptions::builder()
            .chain_id(self.chain_id)
            .contract_address(ethers_address_from_string(self.contract_config.address())?)
            .timeout_ms(self.query_timeout_ms)
            .build();
        let quote = self.deposit_contracts.quote(quote_options).await?;
        let min_amount: f64 = decimal_to_number(&quote.min_amount, Some(self.contract_config.asset_decimals()))?;
        let max_amount: f64 = decimal_to_number(&quote.max_amount, Some(self.contract_config.asset_decimals()))?;
        let min_rollup_fee_amount: f64 = decimal_to_number(
            &quote.min_rollup_fee_amount,
            Some(self.contract_config.asset_decimals()),
        )?;
        let min_bridge_fee_amount: Option<f64> = quote
            .min_bridge_fee_amount
            .map(|v| decimal_to_number(&v, Some(self.contract_config.bridge_fee_asset().asset_decimals())))
            .transpose()?;
        let min_executor_fee_amount: Option<f64> = quote
            .min_executor_fee_amount
            .map(|v| decimal_to_number(&v, Some(self.contract_config.executor_fee_asset().asset_decimals())))
            .transpose()?;
        let rollup_fee_asset_symbol = self.contract_config.asset_symbol().to_string();
        let bridge_fee_asset_symbol = (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(self.contract_config.bridge_fee_asset().asset_symbol().to_string());
        let executor_fee_asset_symbol = (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(self.contract_config.executor_fee_asset().asset_symbol().to_string());
        let recommended_amounts = self.contract_config.recommended_amounts_number::<f64>()?;
        Ok(DepositQuote::builder()
            .min_amount(min_amount)
            .max_amount(max_amount)
            .min_rollup_fee_amount(min_rollup_fee_amount)
            .min_bridge_fee_amount(min_bridge_fee_amount)
            .min_executor_fee_amount(min_executor_fee_amount)
            .rollup_fee_asset_symbol(rollup_fee_asset_symbol)
            .bridge_fee_asset_symbol(bridge_fee_asset_symbol)
            .executor_fee_asset_symbol(executor_fee_asset_symbol)
            .recommended_amounts(recommended_amounts)
            .build())
    }

    pub(crate) async fn summary(&self, options: &CreateDepositOptions) -> Result<DepositSummary> {
        self.validate_amounts(options).await?;
        let bridge_fee_asset_symbol = (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(self.contract_config.bridge_fee_asset().asset_symbol().to_string());
        let executor_fee_asset_symbol = (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(self.contract_config.executor_fee_asset().asset_symbol().to_string());
        let total_amounts = self
            .deposit_assets
            .values()
            .map(|asset_context| {
                (
                    asset_context.asset_config.asset_symbol().to_string(),
                    asset_context.amount,
                )
            })
            .collect::<HashMap<_, _>>();
        Ok(DepositSummary::builder()
            .chain_id(options.chain_id)
            .asset_symbol(options.asset_symbol.clone())
            .amount(options.amount)
            .shielded_address(options.shielded_address.clone())
            .rollup_fee_amount(options.rollup_fee_amount)
            .rollup_fee_asset_symbol(self.contract_config.asset_symbol().to_string())
            .dst_chain_id(options.dst_chain_id)
            .bridge_fee_amount(options.bridge_fee_amount)
            .bridge_fee_asset_symbol(bridge_fee_asset_symbol)
            .executor_fee_amount(options.executor_fee_amount)
            .executor_fee_asset_symbol(executor_fee_asset_symbol)
            .bridge_type(options.bridge_type)
            .total_amounts(total_amounts)
            .build())
    }

    pub(crate) async fn create_deposit(&self, options: &CreateDepositOptions, wallet_id: String) -> Result<Deposit> {
        self.validate_amounts(options).await?;
        let dst_chain_id = self.contract_config.peer_chain_id().unwrap_or(options.chain_id);
        let dst_chain_contract_address = self
            .contract_config
            .peer_contract_address()
            .unwrap_or(self.contract_config.address())
            .to_string();
        let dst_pool_address = self
            .peer_contract_config
            .as_ref()
            .map(|c| c.pool_contract_address())
            .unwrap_or(self.contract_config.pool_contract_address())
            .to_string();
        let amount = number_to_u256_decimal(options.amount, Some(self.contract_config.asset_decimals()))?;
        let amount = u256_to_biguint(&amount);
        let commitment = mystiko_protocol::commitment::Commitment::new(
            mystiko_protocol::address::ShieldedAddress::from_string(&options.shielded_address)?,
            Some(mystiko_protocol::commitment::Note::new(Some(amount), None)),
            None,
        )?;
        let bridge_type: BridgeType = self.contract_config.bridge_type().into();
        Ok(Deposit {
            chain_id: options.chain_id,
            contract_address: self.contract_config.address().to_string(),
            pool_address: self.contract_config.pool_contract_address().to_string(),
            dst_chain_id,
            dst_chain_contract_address,
            dst_pool_address,
            commitment_hash: commitment.commitment_hash.to_string(),
            hash_k: commitment.k.to_string(),
            random_s: bytes_to_biguint(commitment.note.random_s).to_string(),
            encrypted_note: encode_hex_with_prefix(commitment.encrypted_note),
            asset_symbol: self.contract_config.asset_symbol().to_string(),
            asset_address: (self.contract_config.asset_type() == &mystiko_types::AssetType::Erc20)
                .then_some(self.contract_config.asset().asset_address().to_string()),
            bridge_type: bridge_type as i32,
            amount: options.amount,
            rollup_fee_amount: options.rollup_fee_amount,
            bridge_fee_amount: options.bridge_fee_amount,
            bridge_fee_asset_address: (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop
                && self.contract_config.bridge_fee_asset_address().is_some())
            .then_some(self.contract_config.bridge_fee_asset().asset_address().to_string()),
            bridge_fee_asset_symbol: (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
                .then_some(self.contract_config.bridge_fee_asset().asset_symbol().to_string()),
            executor_fee_amount: options.executor_fee_amount,
            executor_fee_asset_address: (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop
                && self.contract_config.executor_fee_asset_address().is_some())
            .then_some(self.contract_config.executor_fee_asset().asset_address().to_string()),
            executor_fee_asset_symbol: (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
                .then_some(self.contract_config.executor_fee_asset().asset_symbol().to_string()),
            shielded_recipient_address: options.shielded_address.clone(),
            status: DepositStatus::Unspecified as i32,
            error_message: None,
            wallet_id,
            asset_approve_transaction_hash: None,
            src_chain_transaction_hash: None,
            queued_transaction_hash: None,
            included_transaction_hash: None,
        })
    }

    pub(crate) async fn validate_amounts(&self, options: &CreateDepositOptions) -> Result<()>
    where
        D: DepositContractHandler,
        DepositsError: From<D::Error>,
    {
        let quote = if let Some(quote) = self.quote.clone() {
            quote
        } else {
            self.quote().await?
        };
        if options.amount < quote.min_amount || options.amount > quote.max_amount {
            return Err(DepositsError::InvalidDepositAmountError(
                options.amount,
                quote.min_amount,
                quote.max_amount,
            ));
        }
        if options.rollup_fee_amount < quote.min_rollup_fee_amount {
            return Err(DepositsError::InvalidRollupFeeAmountError(
                options.rollup_fee_amount,
                quote.min_rollup_fee_amount,
            ));
        }
        if let Some(min_bridge_fee_amount) = quote.min_bridge_fee_amount {
            if options.bridge_fee_amount() < min_bridge_fee_amount {
                return Err(DepositsError::InvalidBridgeFeeAmountError(
                    options.bridge_fee_amount(),
                    min_bridge_fee_amount,
                ));
            }
        }
        if let Some(min_executor_fee_amount) = quote.min_executor_fee_amount {
            if options.executor_fee_amount() < min_executor_fee_amount {
                return Err(DepositsError::InvalidExecutorFeeAmountError(
                    options.executor_fee_amount(),
                    min_executor_fee_amount,
                ));
            }
        }
        Ok(())
    }

    pub(crate) async fn send_assets_approve<S>(
        &self,
        options: &SendDepositOptions,
        signer: Arc<S>,
        owner: Address,
    ) -> Result<Vec<(String, TxHash)>>
    where
        S: TransactionSigner + 'static,
    {
        let mut tx_hashes = vec![];
        for (_, asset_context) in self.deposit_assets.iter() {
            if let Some(tx_hash) = asset_context.approve(self, options, signer.clone(), owner).await? {
                tx_hashes.push((asset_context.asset_config.asset_symbol().to_string(), tx_hash));
            }
        }
        Ok(tx_hashes)
    }

    pub(crate) async fn send_deposit<S>(
        &self,
        deposit: &mut Document<Deposit>,
        options: &SendDepositOptions,
        signer: Arc<S>,
    ) -> Result<TxHash>
    where
        D: DepositContractHandler,
        T: TransactionHandler<Transaction>,
        S: TransactionSigner + 'static,
        DepositsError: From<D::Error> + From<T::Error>,
    {
        let tx = self
            .transactions
            .create(options.deposit_tx.clone(), self.chain_config.transaction_type())?;
        let contract_address = ethers_address_from_string(self.contract_config.address())?;
        let asset_decimals = Some(self.contract_config.asset_decimals());
        let amount = number_to_u256_decimal(deposit.data.amount, asset_decimals)?;
        let rollup_fee_amount = number_to_u256_decimal(deposit.data.rollup_fee_amount, asset_decimals)?;
        let commitment = U256::from_dec_str(&deposit.data.commitment_hash)?;
        let hash_k = U256::from_dec_str(&deposit.data.hash_k)?;
        let random_s: u128 = U128::from_dec_str(&deposit.data.random_s)?.as_u128();
        let encrypted_notes = Bytes::from_str(&deposit.data.encrypted_note)?;
        if self.contract_config.bridge_type() == &mystiko_types::BridgeType::Loop {
            let options = crate::DepositOptions::<TypedTransaction, S>::builder()
                .chain_id(self.chain_id)
                .contract_address(contract_address)
                .amount(amount)
                .commitment(commitment)
                .hash_k(hash_k)
                .random_s(random_s)
                .encrypted_notes(encrypted_notes)
                .rollup_fee(rollup_fee_amount)
                .tx(tx)
                .signer(signer)
                .timeout_ms(options.tx_send_timeout_ms)
                .build();
            let tx_hash = self.deposit_contracts.deposit(options).await?;
            deposit.data.queued_transaction_hash = Some(tx_hash.encode_hex());
            deposit.data.status = DepositStatus::SrcPending as i32;
            Ok(tx_hash)
        } else {
            let bridge_fee_amount = number_to_u256_decimal(
                deposit.data.bridge_fee_amount.unwrap_or_default(),
                Some(self.contract_config.bridge_fee_asset().asset_decimals()),
            )?;
            let executor_fee_amount = number_to_u256_decimal(
                deposit.data.executor_fee_amount.unwrap_or_default(),
                Some(self.contract_config.executor_fee_asset().asset_decimals()),
            )?;
            let options = crate::CrossChainDepositOptions::<TypedTransaction, S>::builder()
                .chain_id(self.chain_id)
                .contract_address(contract_address)
                .amount(amount)
                .bridge_fee(bridge_fee_amount)
                .executor_fee(executor_fee_amount)
                .commitment(commitment)
                .hash_k(hash_k)
                .random_s(random_s)
                .encrypted_notes(encrypted_notes)
                .rollup_fee(rollup_fee_amount)
                .tx(tx)
                .signer(signer)
                .timeout_ms(options.tx_send_timeout_ms)
                .build();
            let tx_hash = self.deposit_contracts.cross_chain_deposit(options).await?;
            deposit.data.src_chain_transaction_hash = Some(tx_hash.encode_hex());
            deposit.data.status = DepositStatus::SrcPending as i32;
            Ok(tx_hash)
        }
    }

    pub(crate) async fn validate_deposit(
        &self,
        deposit: &Document<Deposit>,
        options: &SendDepositOptions,
    ) -> Result<()> {
        if deposit.data.status != DepositStatus::Unspecified as i32 {
            return Err(DepositsError::DepositStatusError(format!(
                "{:?}",
                DepositStatus::from_i32(deposit.data.status).unwrap_or_default()
            )));
        }
        let (chain_id, pool_contract_address) = if let Some(peer_contract_config) = &self.peer_contract_config {
            (deposit.data.dst_chain_id, peer_contract_config.pool_contract_address())
        } else {
            (deposit.data.chain_id, self.contract_config.pool_contract_address())
        };
        let contract_address = ethers_address_from_string(pool_contract_address)?;
        let is_historic_commitment_options = IsHistoricCommitmentOptions::builder()
            .chain_id(chain_id)
            .contract_address(contract_address)
            .commitment_hash(U256::from_dec_str(&deposit.data.commitment_hash)?)
            .timeout_ms(options.query_timeout_ms)
            .build();
        if self
            .commitment_pool_contracts
            .is_historic_commitment(is_historic_commitment_options)
            .await?
        {
            return Err(DepositsError::DuplicateCommitmentError(
                deposit.data.commitment_hash.clone(),
                chain_id,
                pool_contract_address.to_string(),
            ));
        }
        Ok(())
    }

    pub(crate) async fn validate_balances(&self, owner: &Address) -> Result<()> {
        let validations = self
            .deposit_assets
            .values()
            .map(|asset_context| asset_context.validate_balance(self, owner))
            .collect::<Vec<_>>();
        futures::future::try_join_all(validations).await?;
        Ok(())
    }

    pub(crate) fn from_quote_options(
        config: Arc<MystikoConfig>,
        assets: Arc<A>,
        deposit_contracts: Arc<D>,
        commitment_pool_contracts: Arc<C>,
        transactions: Arc<T>,
        options: &QuoteDepositOptions,
    ) -> Result<Self> {
        let contract_config = find_deposit_config(
            config.clone(),
            options.chain_id,
            &options.asset_symbol,
            options.dst_chain_id,
            options.bridge_type,
        )?;
        let mut context = Self::from_contract_config(
            options.chain_id,
            config,
            contract_config,
            assets,
            deposit_contracts,
            commitment_pool_contracts,
            transactions,
        )?;
        context.query_timeout_ms = options.query_timeout_ms;
        Ok(context)
    }

    pub(crate) fn from_create_options(
        config: Arc<MystikoConfig>,
        assets: Arc<A>,
        deposit_contracts: Arc<D>,
        commitment_pool_contracts: Arc<C>,
        transactions: Arc<T>,
        options: &CreateDepositOptions,
    ) -> Result<Self> {
        let contract_config = find_deposit_config(
            config.clone(),
            options.chain_id,
            &options.asset_symbol,
            options.dst_chain_id,
            options.bridge_type,
        )?;
        let deposit_assets = create_assets_map(
            options.chain_id,
            &contract_config,
            options.amount,
            options.rollup_fee_amount,
            options.bridge_fee_amount(),
            options.executor_fee_amount(),
            options.query_timeout_ms,
        )?;
        let mut context = Self::from_contract_config(
            options.chain_id,
            config,
            contract_config,
            assets,
            deposit_contracts,
            commitment_pool_contracts,
            transactions,
        )?;
        context.quote = options.deposit_quote.clone();
        context.query_timeout_ms = options.query_timeout_ms;
        context.deposit_assets = deposit_assets;
        Ok(context)
    }

    pub(crate) fn from_send_options(
        config: Arc<MystikoConfig>,
        assets: Arc<A>,
        deposit_contracts: Arc<D>,
        commitment_pool_contracts: Arc<C>,
        transactions: Arc<T>,
        deposit: &Document<Deposit>,
        options: &SendDepositOptions,
    ) -> Result<Self> {
        let bridge_type: mystiko_types::BridgeType = BridgeType::from_i32(deposit.data.bridge_type)
            .unwrap_or_default()
            .into();
        let contract_config = config
            .find_deposit_contract_by_address(deposit.data.chain_id, &deposit.data.contract_address)
            .ok_or(DepositsError::NoDepositContractFoundError(
                deposit.data.chain_id,
                deposit.data.contract_address.clone(),
                deposit.data.dst_chain_id,
                bridge_type,
            ))?
            .clone();
        let deposit_assets = create_assets_map(
            deposit.data.chain_id,
            &contract_config,
            deposit.data.amount,
            deposit.data.rollup_fee_amount,
            deposit.data.bridge_fee_amount.unwrap_or_default(),
            deposit.data.executor_fee_amount.unwrap_or_default(),
            options.query_timeout_ms,
        )?;
        let mut context = Self::from_contract_config(
            deposit.data.chain_id,
            config,
            contract_config,
            assets,
            deposit_contracts,
            commitment_pool_contracts,
            transactions,
        )?;
        context.deposit_assets = deposit_assets;
        context.query_timeout_ms = options.query_timeout_ms;
        Ok(context)
    }

    pub(crate) fn from_contract_config(
        chain_id: u64,
        config: Arc<MystikoConfig>,
        contract_config: DepositContractConfig,
        assets: Arc<A>,
        deposit_contracts: Arc<D>,
        commitment_pool_contracts: Arc<C>,
        transactions: Arc<T>,
    ) -> Result<Self> {
        let chain_config = config
            .find_chain(chain_id)
            .ok_or(DepositsError::UnsupportedChainIdError(chain_id))?
            .clone();
        let peer_contract_config = if let (Some(peer_chain_id), Some(peer_contract_address)) =
            (contract_config.peer_chain_id(), contract_config.peer_contract_address())
        {
            config.find_deposit_contract_by_address(*peer_chain_id, peer_contract_address)
        } else {
            None
        };
        Ok(DepositContext::builder()
            .chain_id(chain_id)
            .chain_config(chain_config)
            .contract_config(contract_config)
            .peer_contract_config(peer_contract_config.cloned())
            .assets(assets)
            .deposit_contracts(deposit_contracts)
            .commitment_pool_contracts(commitment_pool_contracts)
            .transactions(transactions)
            .build())
    }
}

impl AssetContext {
    pub(crate) fn new(
        chain_id: u64,
        asset_config: AssetConfig,
        amount: f64,
        query_timeout_ms: Option<u64>,
    ) -> Result<Self> {
        let converted_amount = number_to_u256_decimal(amount, Some(asset_config.asset_decimals()))?;
        Ok(Self {
            chain_id,
            asset_config,
            amount,
            converted_amount,
            query_timeout_ms,
        })
    }

    pub(crate) async fn approve<A, D, C, T, S>(
        &self,
        context: &DepositContext<A, D, C, T>,
        options: &SendDepositOptions,
        signer: Arc<S>,
        owner: Address,
    ) -> Result<Option<TxHash>>
    where
        D: DepositContractHandler,
        A: PublicAssetHandler,
        T: TransactionHandler<Transaction>,
        C: CommitmentPoolContractHandler,
        S: TransactionSigner + 'static,
        DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
    {
        if self.asset_config.asset_type() != &mystiko_types::AssetType::Main && self.converted_amount.gt(&U256::zero())
        {
            let asset_address = ethers_address_from_string(self.asset_config.asset_address())?;
            let contract_address = ethers_address_from_string(context.contract_config.address())?;
            let options = Erc20ApproveOptions::<TypedTransaction, S>::builder()
                .chain_id(self.chain_id)
                .asset_address(asset_address)
                .owner(owner)
                .recipient(contract_address)
                .amount(self.converted_amount)
                .signer(signer)
                .tx(context.transactions.create(
                    options.asset_approve_tx.clone(),
                    context.chain_config.transaction_type(),
                )?)
                .timeout_ms(options.tx_send_timeout_ms)
                .build();
            return Ok(context.assets.erc20_approve(options).await?);
        }
        Ok(None)
    }

    pub(crate) async fn validate_balance<A, D, C, T>(
        &self,
        context: &DepositContext<A, D, C, T>,
        owner: &Address,
    ) -> Result<()>
    where
        A: PublicAssetHandler,
        D: DepositContractHandler,
        C: CommitmentPoolContractHandler,
        T: TransactionHandler<Transaction>,
        DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
    {
        if self.converted_amount.gt(&U256::zero()) {
            let balance = if self.asset_config.asset_type() == &mystiko_types::AssetType::Main {
                let options = BalanceOptions::builder()
                    .chain_id(self.chain_id)
                    .owner(*owner)
                    .timeout_ms(self.query_timeout_ms)
                    .build();
                context.assets.balance_of(options).await?
            } else {
                let asset_address = ethers_address_from_string(self.asset_config.asset_address())?;
                let options = Erc20BalanceOptions::builder()
                    .chain_id(self.chain_id)
                    .asset_address(asset_address)
                    .owner(*owner)
                    .timeout_ms(self.query_timeout_ms)
                    .build();
                context.assets.erc20_balance_of(options).await?
            };
            if balance.lt(&self.converted_amount) {
                return Err(DepositsError::InsufficientBalanceError(
                    self.asset_config.asset_symbol().to_string(),
                    self.amount,
                ));
            }
        }
        Ok(())
    }
}

fn find_deposit_config(
    config: Arc<MystikoConfig>,
    chain_id: u64,
    asset_symbol: &str,
    dst_chain_id: Option<u64>,
    bridge_type: Option<i32>,
) -> Result<DepositContractConfig> {
    let dst_chain_id = dst_chain_id.unwrap_or(chain_id);
    let bridge_type: mystiko_types::BridgeType = bridge_type
        .and_then(BridgeType::from_i32)
        .unwrap_or(BridgeType::Loop)
        .into();
    if let Some(contract_config) = config.find_deposit_contract(chain_id, dst_chain_id, asset_symbol, &bridge_type) {
        Ok(contract_config.clone())
    } else {
        Err(DepositsError::NoDepositContractFoundError(
            chain_id,
            asset_symbol.to_string(),
            dst_chain_id,
            bridge_type,
        ))
    }
}

fn create_assets_map(
    chain_id: u64,
    contract_config: &DepositContractConfig,
    amount: f64,
    rollup_fee_amount: f64,
    bridge_fee_amount: f64,
    executor_fee_amount: f64,
    query_timeout_ms: Option<u64>,
) -> Result<HashMap<String, AssetContext>> {
    let mut assets = HashMap::from([(
        contract_config.asset().asset_address().to_string(),
        AssetContext::new(
            chain_id,
            contract_config.asset().clone(),
            amount + rollup_fee_amount,
            query_timeout_ms,
        )?,
    )]);
    if contract_config.bridge_type() != &mystiko_types::BridgeType::Loop {
        if bridge_fee_amount > 0_f64 {
            let bridge_fee_asset = contract_config.bridge_fee_asset().clone();
            update_assets_map(
                &mut assets,
                chain_id,
                bridge_fee_asset,
                bridge_fee_amount,
                query_timeout_ms,
            )
        }
        if executor_fee_amount > 0_f64 {
            let executor_fee_asset = contract_config.executor_fee_asset().clone();
            update_assets_map(
                &mut assets,
                chain_id,
                executor_fee_asset,
                executor_fee_amount,
                query_timeout_ms,
            )
        }
    }
    Ok(assets)
}

fn update_assets_map(
    assets: &mut HashMap<String, AssetContext>,
    chain_id: u64,
    asset_config: AssetConfig,
    amount: f64,
    query_timeout_ms: Option<u64>,
) {
    if let Some(asset_context) = assets.get_mut(asset_config.asset_address()) {
        asset_context.amount += amount;
        asset_context.converted_amount =
            number_to_u256_decimal(asset_context.amount, Some(asset_config.asset_decimals())).unwrap();
    } else {
        assets.insert(
            asset_config.asset_address().to_string(),
            AssetContext::new(chain_id, asset_config, amount, query_timeout_ms).unwrap(),
        );
    }
}

fn format_deposit_log(deposit: &Document<Deposit>) -> String {
    if deposit.data.bridge_type == BridgeType::Loop as i32 {
        format!(
            "deposit(id={:?}, chain_id={}, asset_symbol={}, amount={}, rollup_fee_amount={}, status={:?})",
            deposit.id,
            deposit.data.chain_id,
            deposit.data.asset_symbol,
            deposit.data.amount,
            deposit.data.rollup_fee_amount,
            DepositStatus::from_i32(deposit.data.status).unwrap_or_default()
        )
    } else {
        format!(
            "deposit(id={:?}, chain_id={}, dst_chain_id={}, bridge_type={:?}, asset_symbol={}, \
            amount={}, rollup_fee_amount={}, bridge_fee_amount={}, executor_fee_amount={}, status={:?})",
            deposit.id,
            deposit.data.chain_id,
            deposit.data.dst_chain_id,
            BridgeType::from_i32(deposit.data.bridge_type).unwrap_or_default(),
            deposit.data.asset_symbol,
            deposit.data.amount,
            deposit.data.rollup_fee_amount,
            deposit.data.bridge_fee_amount.unwrap_or_default(),
            deposit.data.executor_fee_amount.unwrap_or_default(),
            DepositStatus::from_i32(deposit.data.status).unwrap_or_default()
        )
    }
}

fn wrap_filter<F>(filter: F, wallet: &Wallet) -> QueryFilter
where
    F: Into<QueryFilter> + Send + Sync,
{
    let mut filter = filter.into();
    let sub_filter = SubFilter::equal(DepositColumn::WalletId, wallet.id.clone());
    filter.additional_condition = Some(sub_filter.into());
    filter
}

fn format_tx(config: Arc<MystikoConfig>, chain_id: u64, tx_hash: &TxHash) -> Result<String> {
    let tx_url = config
        .transaction_url(chain_id, &tx_hash.encode_hex())
        .ok_or(DepositsError::UnsupportedChainIdError(chain_id))?;
    Ok(format!("tx({})", tx_url))
}
