use crate::{
    BalanceOptions, Database, Deposit, DepositContractHandler, DepositContracts, DepositContractsError, DepositHandler,
    Erc20ApproveOptions, Erc20BalanceOptions, FromContext, MystikoContext, MystikoError, PrivateKeySigner,
    PrivateKeySignerOptions, PublicAssetHandler, PublicAssets, PublicAssetsError, TransactionHandler,
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
use mystiko_protos::core::document::v1::Deposit as ProtoDeposit;
use mystiko_protos::core::handler::v1::{
    CreateDepositOptions, DepositQuote, DepositSummary, QuoteDepositOptions, SendDepositOptions,
};
use mystiko_protos::core::v1::{DepositStatus, Transaction};
use mystiko_storage::{Document, StatementFormatter, Storage, StorageError};
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
    T: TransactionHandler<Transaction> = Transactions<Box<dyn Providers>>,
    P: Providers = Box<dyn Providers>,
> {
    db: Arc<Database<F, S>>,
    config: Arc<MystikoConfig>,
    wallets: Wallets<F, S>,
    assets: Arc<A>,
    deposit_contracts: Arc<D>,
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
    T: TransactionHandler<Transaction> = Transactions<Box<dyn Providers>>,
    P: Providers = Box<dyn Providers>,
> {
    db: Arc<Database<F, S>>,
    config: Arc<MystikoConfig>,
    assets: Arc<A>,
    deposit_contracts: Arc<D>,
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
}

type Result<T> = std::result::Result<T, DepositsError>;

#[async_trait]
impl<F, S, A, D, T, P>
    DepositHandler<
        ProtoDeposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
    > for Deposits<F, S, A, D, T, P>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContractHandler,
    T: TransactionHandler<Transaction>,
    P: Providers + 'static,
    DepositsError: From<A::Error> + From<D::Error> + From<T::Error>,
{
    type Error = DepositsError;

    async fn quote(&self, options: QuoteDepositOptions) -> Result<DepositQuote> {
        let context = DepositContext::from_quote_options(
            self.config.clone(),
            self.assets.clone(),
            self.deposit_contracts.clone(),
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
            self.transactions.clone(),
            &options,
        )?;
        let deposit = context.create_deposit(&options, wallet.id).await?;
        let deposit = self.db.deposits.insert(&deposit).await?;
        log::info!("successfully created a deposit(id = {:?})", deposit.id);
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
            "sending deposit(id = {:?}) transaction with signer at address {}",
            deposit.id,
            owner_address
        );
        match self.send_transaction(&options, deposit.clone(), signer, owner).await {
            Err(err) => {
                log::error!("failed to send deposit(id = {:?}) transaction: {}", deposit.id, err);
                deposit.data.status = DepositStatus::Failed as i32;
                deposit.data.error_message = Some(err.to_string());
                self.db.deposits.update(&deposit).await?;
                Err(err)
            }
            Ok(deposit) => {
                let status = DepositStatus::from_i32(deposit.data.status).unwrap_or_default();
                log::info!(
                    "successfully sent deposit(id = {:?}, status = {:?}) transaction",
                    deposit.id,
                    status
                );
                Ok(Deposit::document_into_proto(deposit))
            }
        }
    }
}

impl<F, S, A, D, T, P> Deposits<F, S, A, D, T, P>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContractHandler,
    T: TransactionHandler<Transaction>,
    P: Providers + 'static,
    DepositsError: From<A::Error> + From<D::Error> + From<T::Error>,
{
    pub fn new(options: DepositsOptions<F, S, A, D, T, P>) -> Self {
        let wallets = Wallets::new(options.db.clone());
        Self::builder()
            .db(options.db)
            .config(options.config)
            .signer_providers(options.signer_providers)
            .wallets(wallets)
            .assets(options.assets)
            .deposit_contracts(options.deposit_contracts)
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
            self.transactions.clone(),
            &deposit,
            options,
        )?;
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
        context: &DepositContext<A, D, T>,
        signer: Arc<Signer>,
        owner: Address,
    ) -> Result<Document<Deposit>>
    where
        Signer: TransactionSigner + 'static,
    {
        context.validate_balances(&owner).await?;
        let asset_approve_tx_hashes = context.send_assets_approve(options, signer.clone(), owner).await?;
        if !asset_approve_tx_hashes.is_empty() {
            deposit.data.asset_approve_transaction_hash = Some(
                asset_approve_tx_hashes
                    .iter()
                    .map(|(asset_symbol, tx_hash)| {
                        let tx_hash = tx_hash.encode_hex();
                        log::info!(
                            "successfully submitted {} approving transaction(chain_id = {}, hash = {:?}) \
                            for deposit(id = {:?})",
                            asset_symbol,
                            deposit.data.chain_id,
                            tx_hash,
                            deposit.id,
                        );
                        tx_hash
                    })
                    .collect::<Vec<_>>(),
            );
        }
        deposit.data.status = DepositStatus::AssetApproving as i32;
        deposit = self.db.deposits.update(&deposit).await?;
        for (asset_symbol, tx_hash) in asset_approve_tx_hashes.into_iter() {
            let wait_options = WaitOptions::builder()
                .chain_id(deposit.data.chain_id)
                .tx_hash(tx_hash)
                .confirmations(options.asset_approve_confirmations)
                .interval_ms(options.tx_wait_interval_ms)
                .timeout_ms(options.tx_wait_timeout_ms)
                .build();
            self.transactions.wait(wait_options).await?;
            log::info!(
                "{} approving transaction(chain_id = {}, hash = {:?}) for deposit(id = {:?}) is confirmed",
                deposit.data.chain_id,
                asset_symbol,
                tx_hash,
                deposit.id,
            )
        }
        deposit.data.status = DepositStatus::AssetApproved as i32;
        Ok(self.db.deposits.update(&deposit).await?)
    }

    async fn send_deposit<Signer>(
        &self,
        options: &SendDepositOptions,
        mut deposit: Document<Deposit>,
        context: &DepositContext<A, D, T>,
        signer: Arc<Signer>,
    ) -> Result<Document<Deposit>>
    where
        Signer: TransactionSigner + 'static,
    {
        let send_tx_hash = context.send_deposit(&mut deposit, options, signer.clone()).await?;
        log::info!(
            "successfully submitted deposit(id = {:?}) transaction(chain_id = {}, hash = {:?})",
            deposit.id,
            deposit.data.chain_id,
            send_tx_hash.encode_hex()
        );
        self.db.deposits.update(&deposit).await?;
        let wait_options = WaitOptions::builder()
            .chain_id(deposit.data.chain_id)
            .tx_hash(send_tx_hash)
            .confirmations(options.deposit_confirmations)
            .interval_ms(options.tx_wait_interval_ms)
            .timeout_ms(options.tx_wait_timeout_ms)
            .build();
        self.transactions.wait(wait_options).await?;
        log::info!(
            "deposit(id = {:?}) transaction(chain_id = {}, hash = {:?}) is confirmed",
            deposit.id,
            deposit.data.chain_id,
            send_tx_hash.encode_hex()
        );
        if context.contract_config.bridge_type() == &mystiko_types::BridgeType::Loop {
            deposit.data.status = DepositStatus::SrcSucceeded as i32;
        } else {
            deposit.data.status = DepositStatus::Queued as i32;
        }
        Ok(self.db.deposits.update(&deposit).await?)
    }
}

#[async_trait]
impl<F, S, A, D, T> FromContext<F, S> for Deposits<F, S, A, D, T>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler + FromContext<F, S>,
    D: DepositContractHandler + FromContext<F, S>,
    T: TransactionHandler<Transaction> + FromContext<F, S>,
    DepositsError: From<A::Error> + From<D::Error> + From<T::Error>,
{
    async fn from_context(context: &MystikoContext<F, S>) -> std::result::Result<Self, MystikoError> {
        let options = DepositsOptions::builder()
            .db(context.db.clone())
            .config(context.config.clone())
            .signer_providers(context.signer_providers.clone())
            .assets(A::from_context(context).await?)
            .deposit_contracts(D::from_context(context).await?)
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
struct DepositContext<A: PublicAssetHandler, D: DepositContractHandler, T: TransactionHandler<Transaction>> {
    pub(crate) chain_id: u64,
    pub(crate) chain_config: ChainConfig,
    pub(crate) contract_config: DepositContractConfig,
    pub(crate) assets: Arc<A>,
    pub(crate) deposit_contracts: Arc<D>,
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

impl<A, D, T> DepositContext<A, D, T>
where
    A: PublicAssetHandler,
    D: DepositContractHandler,
    T: TransactionHandler<Transaction>,
    DepositsError: From<A::Error> + From<D::Error> + From<T::Error>,
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
                .build();
            let tx_hash = self.deposit_contracts.cross_chain_deposit(options).await?;
            deposit.data.src_chain_transaction_hash = Some(tx_hash.encode_hex());
            deposit.data.status = DepositStatus::SrcPending as i32;
            Ok(tx_hash)
        }
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
            transactions,
        )?;
        context.query_timeout_ms = options.query_timeout_ms;
        Ok(context)
    }

    pub(crate) fn from_create_options(
        config: Arc<MystikoConfig>,
        assets: Arc<A>,
        deposit_contracts: Arc<D>,
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

    pub(crate) async fn approve<A, D, T, S>(
        &self,
        context: &DepositContext<A, D, T>,
        options: &SendDepositOptions,
        signer: Arc<S>,
        owner: Address,
    ) -> Result<Option<TxHash>>
    where
        D: DepositContractHandler,
        A: PublicAssetHandler,
        T: TransactionHandler<Transaction>,
        S: TransactionSigner + 'static,
        DepositsError: From<A::Error> + From<D::Error> + From<T::Error>,
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

    pub(crate) async fn validate_balance<A, D, T>(
        &self,
        context: &DepositContext<A, D, T>,
        owner: &Address,
    ) -> Result<()>
    where
        D: DepositContractHandler,
        A: PublicAssetHandler,
        T: TransactionHandler<Transaction>,
        DepositsError: From<A::Error> + From<D::Error> + From<T::Error>,
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
            if let Some(asset_context) = assets.get_mut(bridge_fee_asset.asset_address()) {
                asset_context.amount += bridge_fee_amount;
            } else {
                assets.insert(
                    bridge_fee_asset.asset_address().to_string(),
                    AssetContext::new(chain_id, bridge_fee_asset, bridge_fee_amount, query_timeout_ms)?,
                );
            }
        }
        if executor_fee_amount > 0_f64 {
            let executor_fee_asset = contract_config.executor_fee_asset().clone();
            if let Some(asset_context) = assets.get_mut(executor_fee_asset.asset_address()) {
                asset_context.amount += executor_fee_amount;
            } else {
                assets.insert(
                    executor_fee_asset.asset_address().to_string(),
                    AssetContext::new(chain_id, executor_fee_asset, executor_fee_amount, query_timeout_ms)?,
                );
            }
        }
    }
    Ok(assets)
}
