use crate::{
    convert_transaction, wait_transaction, BalanceOptions, Database, Deposit, DepositContract, DepositHandler,
    Deposits, DepositsError, Erc20ApproveOptions, Erc20BalanceOptions, FromContext, MystikoContext, MystikoError,
    PrivateKeySigner, PrivateKeySignerOptions, PublicAssetHandler, PublicAssets, PublicAssetsError, TransactionSigner,
    WalletHandler, WalletHandlerV1, WalletHandlerV1Error,
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
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{bytes_to_biguint, decimal_to_number, number_to_decimal};
use mystiko_utils::hex::encode_hex_with_prefix;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DepositHandlerV1<
    F: StatementFormatter,
    S: Storage,
    P: Providers = Box<dyn Providers>,
    A: PublicAssetHandler = PublicAssets<P>,
    D: DepositContract = Deposits<P>,
> {
    db: Arc<Database<F, S>>,
    config: Arc<MystikoConfig>,
    wallets: WalletHandlerV1<F, S>,
    providers: Arc<P>,
    signer_providers: Arc<P>,
    assets: A,
    deposits: D,
}

#[derive(Debug, Error)]
pub enum DepositHandlerV1Error {
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
    DepositsError(#[from] DepositsError),
    #[error(transparent)]
    ProtocolError(#[from] ProtocolError),
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    WalletHandlerError(#[from] WalletHandlerV1Error),
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

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DepositHandlerV1Options<
    F: StatementFormatter,
    S: Storage,
    P: Providers,
    A: PublicAssetHandler,
    D: DepositContract,
> {
    db: Arc<Database<F, S>>,
    config: Arc<MystikoConfig>,
    providers: Arc<P>,
    signer_providers: Arc<P>,
    assets: A,
    deposits: D,
}

type Result<T> = std::result::Result<T, DepositHandlerV1Error>;

#[async_trait]
impl<F, S, P, A, D>
    DepositHandler<
        ProtoDeposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
    > for DepositHandlerV1<F, S, P, A, D>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContract,
    P: Providers + 'static,
    DepositHandlerV1Error: From<A::Error> + From<D::Error>,
{
    type Error = DepositHandlerV1Error;

    async fn quote(&self, options: QuoteDepositOptions) -> Result<DepositQuote> {
        let context = DepositContext::from_quote_options(self.config.clone(), &options)?;
        context.quote(&self.deposits).await
    }

    async fn summary(&self, options: CreateDepositOptions) -> Result<DepositSummary> {
        let context = DepositContext::from_create_options(self.config.clone(), &options)?;
        context.summary(&options, &self.deposits).await
    }

    async fn create(&self, options: CreateDepositOptions) -> Result<ProtoDeposit> {
        let wallet = self.wallets.check_current().await?;
        let context = DepositContext::from_create_options(self.config.clone(), &options)?;
        let deposit = context.create_deposit(&options, &self.deposits, wallet.id).await?;
        Ok(Deposit::document_into_proto(self.db.deposits.insert(&deposit).await?))
    }

    async fn send(&self, options: SendDepositOptions) -> Result<ProtoDeposit> {
        let private_key = options
            .private_key
            .clone()
            .ok_or(DepositHandlerV1Error::MissingPrivateKeyError)?;
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
            .ok_or_else(|| DepositHandlerV1Error::IdNotFoundError(options.deposit_id.clone()))?;
        match self.send_transaction(&options, deposit.clone(), signer).await {
            Err(err) => {
                deposit.data.status = DepositStatus::Failed as i32;
                deposit.data.error_message = Some(err.to_string());
                self.db.deposits.update(&deposit).await?;
                Err(err)
            }
            Ok(deposit) => Ok(Deposit::document_into_proto(deposit)),
        }
    }
}

#[async_trait]
impl<F, S, A, D> FromContext<F, S> for DepositHandlerV1<F, S, Box<dyn Providers>, A, D>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler + FromContext<F, S>,
    D: DepositContract + FromContext<F, S>,
    DepositHandlerV1Error: From<A::Error> + From<D::Error>,
{
    async fn from_context(context: &MystikoContext<F, S>) -> std::result::Result<Self, MystikoError> {
        let options = DepositHandlerV1Options::builder()
            .db(context.db.clone())
            .config(context.config.clone())
            .providers(context.providers.clone())
            .signer_providers(context.signer_providers.clone())
            .assets(A::from_context(context).await?)
            .deposits(D::from_context(context).await?)
            .build();
        Ok(Self::new(options))
    }
}

impl<F, S, P, A, D> DepositHandlerV1<F, S, P, A, D>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContract,
    P: Providers + 'static,
    DepositHandlerV1Error: From<A::Error> + From<D::Error>,
{
    pub fn new(options: DepositHandlerV1Options<F, S, P, A, D>) -> Self {
        let wallets = WalletHandlerV1::new(options.db.clone());
        Self::builder()
            .db(options.db)
            .config(options.config)
            .providers(options.providers)
            .signer_providers(options.signer_providers)
            .wallets(wallets)
            .assets(options.assets)
            .deposits(options.deposits)
            .build()
    }

    async fn send_transaction<Signer>(
        &self,
        options: &SendDepositOptions,
        deposit: Document<Deposit>,
        signer: Arc<Signer>,
    ) -> Result<Document<Deposit>>
    where
        Signer: TransactionSigner + 'static,
    {
        let context = DepositContext::from_send_options(self.config.clone(), &deposit, options)?;
        let deposit = self
            .send_assets_approve(options, deposit, &context, signer.clone())
            .await?;
        let deposit = self.send_deposit(options, deposit, &context, signer.clone()).await?;
        Ok(deposit)
    }

    async fn send_assets_approve<Signer>(
        &self,
        options: &SendDepositOptions,
        mut deposit: Document<Deposit>,
        context: &DepositContext,
        signer: Arc<Signer>,
    ) -> Result<Document<Deposit>>
    where
        Signer: TransactionSigner + 'static,
    {
        let owner = signer.address().await?;
        context.validate_balances(&owner, &self.assets).await?;
        let provider = self.providers.get_provider(deposit.data.chain_id).await?;
        let asset_approve_tx_hashes = context
            .send_assets_approve(&options.asset_approve_tx, &self.assets, signer.clone())
            .await?;
        if !asset_approve_tx_hashes.is_empty() {
            deposit.data.asset_approve_transaction_hash = Some(
                asset_approve_tx_hashes
                    .iter()
                    .map(|h| h.encode_hex())
                    .collect::<Vec<_>>(),
            );
        }
        deposit.data.status = DepositStatus::AssetApproving as i32;
        deposit = self.db.deposits.update(&deposit).await?;
        for tx_hash in asset_approve_tx_hashes.into_iter() {
            wait_transaction(tx_hash, provider.clone(), options.asset_approve_confirmations).await?;
        }
        deposit.data.status = DepositStatus::AssetApproved as i32;
        Ok(self.db.deposits.update(&deposit).await?)
    }

    async fn send_deposit<Signer>(
        &self,
        options: &SendDepositOptions,
        mut deposit: Document<Deposit>,
        context: &DepositContext,
        signer: Arc<Signer>,
    ) -> Result<Document<Deposit>>
    where
        Signer: TransactionSigner + 'static,
    {
        let provider = self.providers.get_provider(deposit.data.chain_id).await?;
        let send_tx_hash = context
            .send_deposit(&mut deposit, &options.deposit_tx, &self.deposits, signer.clone())
            .await?;
        self.db.deposits.update(&deposit).await?;
        wait_transaction(send_tx_hash, provider.clone(), options.deposit_confirmations).await?;
        if context.contract_config.bridge_type() == &mystiko_types::BridgeType::Loop {
            deposit.data.status = DepositStatus::SrcSucceeded as i32;
        } else {
            deposit.data.status = DepositStatus::Queued as i32;
        }
        Ok(self.db.deposits.update(&deposit).await?)
    }
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct AssetContext {
    pub(crate) chain_id: u64,
    pub(crate) asset_config: AssetConfig,
    pub(crate) amount: f64,
    pub(crate) converted_amount: U256,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct DepositContext {
    pub(crate) chain_id: u64,
    pub(crate) chain_config: ChainConfig,
    pub(crate) contract_config: DepositContractConfig,
    #[builder(default)]
    pub(crate) peer_contract_config: Option<DepositContractConfig>,
    #[builder(default)]
    pub(crate) quote: Option<DepositQuote>,
    #[builder(default)]
    pub(crate) query_remote_timeout_ms: Option<u64>,
    #[builder(default)]
    pub(crate) assets: HashMap<String, AssetContext>,
}

impl DepositContext {
    pub(crate) async fn quote<D>(&self, deposits: &D) -> Result<DepositQuote>
    where
        D: DepositContract,
        DepositHandlerV1Error: From<D::Error>,
    {
        let quote_options = crate::DepositQuoteOptions::builder()
            .chain_id(self.chain_id)
            .contract_address(ethers_address_from_string(self.contract_config.address())?)
            .query_timeout_ms(self.query_remote_timeout_ms)
            .build();
        let quote = deposits.quote(quote_options).await?;
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

    pub(crate) async fn summary<D>(&self, options: &CreateDepositOptions, deposits: &D) -> Result<DepositSummary>
    where
        D: DepositContract,
        DepositHandlerV1Error: From<D::Error>,
    {
        self.validate_amounts(options, deposits).await?;
        let bridge_fee_asset_symbol = (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(self.contract_config.bridge_fee_asset().asset_symbol().to_string());
        let executor_fee_asset_symbol = (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(self.contract_config.executor_fee_asset().asset_symbol().to_string());
        let total_amounts = self
            .assets
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

    pub(crate) async fn create_deposit<D>(
        &self,
        options: &CreateDepositOptions,
        deposit: &D,
        wallet_id: String,
    ) -> Result<Deposit>
    where
        D: DepositContract,
        DepositHandlerV1Error: From<D::Error>,
    {
        self.validate_amounts(options, deposit).await?;
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
        let amount = number_to_decimal(options.amount, Some(self.contract_config.asset_decimals()))?;
        let amount = BigUint::from_str(&amount.to_string())?;
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
            bridge_fee_asset_address: (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
                .then_some(self.contract_config.bridge_fee_asset().asset_address().to_string()),
            bridge_fee_asset_symbol: (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
                .then_some(self.contract_config.bridge_fee_asset().asset_symbol().to_string()),
            executor_fee_amount: options.executor_fee_amount,
            executor_fee_asset_address: (self.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
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

    pub(crate) async fn validate_amounts<D>(&self, options: &CreateDepositOptions, deposits: &D) -> Result<()>
    where
        D: DepositContract,
        DepositHandlerV1Error: From<D::Error>,
    {
        let quote = self.quote.clone().unwrap_or(self.quote(deposits).await?);
        if options.amount < quote.min_amount || options.amount > quote.max_amount {
            return Err(DepositHandlerV1Error::InvalidDepositAmountError(
                options.amount,
                quote.min_amount,
                quote.max_amount,
            ));
        }
        if options.rollup_fee_amount < quote.min_rollup_fee_amount {
            return Err(DepositHandlerV1Error::InvalidRollupFeeAmountError(
                options.rollup_fee_amount,
                quote.min_rollup_fee_amount,
            ));
        }
        if let Some(min_bridge_fee_amount) = quote.min_bridge_fee_amount {
            if options.bridge_fee_amount() < min_bridge_fee_amount {
                return Err(DepositHandlerV1Error::InvalidBridgeFeeAmountError(
                    options.bridge_fee_amount(),
                    min_bridge_fee_amount,
                ));
            }
        }
        if let Some(min_executor_fee_amount) = quote.min_executor_fee_amount {
            if options.executor_fee_amount() < min_executor_fee_amount {
                return Err(DepositHandlerV1Error::InvalidExecutorFeeAmountError(
                    options.executor_fee_amount(),
                    min_executor_fee_amount,
                ));
            }
        }
        Ok(())
    }

    pub(crate) async fn send_assets_approve<A, S>(
        &self,
        transaction: &Option<Transaction>,
        assets: &A,
        signer: Arc<S>,
    ) -> Result<Vec<TxHash>>
    where
        A: PublicAssetHandler,
        S: TransactionSigner + 'static,
        DepositHandlerV1Error: From<A::Error>,
    {
        let mut tx_hashes = vec![];
        for (_, asset_context) in self.assets.iter() {
            if let Some(tx_hash) = asset_context
                .approve(
                    &self.contract_config,
                    self.chain_config.transaction_type(),
                    transaction,
                    assets,
                    signer.clone(),
                )
                .await?
            {
                tx_hashes.push(tx_hash);
            }
        }
        Ok(tx_hashes)
    }

    pub(crate) async fn send_deposit<D, S>(
        &self,
        deposit: &mut Document<Deposit>,
        transaction: &Option<Transaction>,
        deposits: &D,
        signer: Arc<S>,
    ) -> Result<TxHash>
    where
        D: DepositContract,
        S: TransactionSigner + 'static,
        DepositHandlerV1Error: From<D::Error>,
    {
        let tx = convert_transaction(self.chain_config.transaction_type(), transaction)?;
        let contract_address = ethers_address_from_string(self.contract_config.address())?;
        let asset_decimals = Some(self.contract_config.asset_decimals());
        let amount = U256::from_dec_str(&number_to_decimal(deposit.data.amount, asset_decimals)?.to_string())?;
        let rollup_fee_amount =
            U256::from_dec_str(&number_to_decimal(deposit.data.rollup_fee_amount, asset_decimals)?.to_string())?;
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
                .build();
            let tx_hash = deposits.deposit(options).await?;
            deposit.data.queued_transaction_hash = Some(tx_hash.encode_hex());
            deposit.data.status = DepositStatus::SrcPending as i32;
            Ok(tx_hash)
        } else {
            let bridge_fee_amount = U256::from_dec_str(
                &number_to_decimal(
                    deposit.data.bridge_fee_amount.unwrap_or_default(),
                    Some(self.contract_config.bridge_fee_asset().asset_decimals()),
                )?
                .to_string(),
            )?;
            let executor_fee_amount = U256::from_dec_str(
                &number_to_decimal(
                    deposit.data.executor_fee_amount.unwrap_or_default(),
                    Some(self.contract_config.executor_fee_asset().asset_decimals()),
                )?
                .to_string(),
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
            let tx_hash = deposits.cross_chain_deposit(options).await?;
            deposit.data.src_chain_transaction_hash = Some(tx_hash.encode_hex());
            deposit.data.status = DepositStatus::SrcPending as i32;
            Ok(tx_hash)
        }
    }

    pub(crate) async fn validate_balances<A>(&self, owner: &Address, assets: &A) -> Result<()>
    where
        A: PublicAssetHandler,
        DepositHandlerV1Error: From<A::Error>,
    {
        let validations = self
            .assets
            .values()
            .map(|asset_context| asset_context.validate_balance(owner, assets))
            .collect::<Vec<_>>();
        futures::future::try_join_all(validations).await?;
        Ok(())
    }

    pub(crate) fn from_quote_options(config: Arc<MystikoConfig>, options: &QuoteDepositOptions) -> Result<Self> {
        let contract_config = find_deposit_config(
            config.clone(),
            options.chain_id,
            &options.asset_symbol,
            options.dst_chain_id,
            options.bridge_type,
        )?;
        let mut context = Self::from_contract_config(options.chain_id, config, contract_config)?;
        context.query_remote_timeout_ms = options.query_remote_timeout_ms;
        Ok(context)
    }

    pub(crate) fn from_create_options(config: Arc<MystikoConfig>, options: &CreateDepositOptions) -> Result<Self> {
        let contract_config = find_deposit_config(
            config.clone(),
            options.chain_id,
            &options.asset_symbol,
            options.dst_chain_id,
            options.bridge_type,
        )?;
        let assets = create_assets_map(
            options.chain_id,
            &contract_config,
            options.amount,
            options.rollup_fee_amount,
            options.bridge_fee_amount(),
            options.executor_fee_amount(),
        )?;
        let mut context = Self::from_contract_config(options.chain_id, config, contract_config)?;
        context.quote = options.deposit_quote.clone();
        context.query_remote_timeout_ms = options.query_remote_timeout_ms;
        context.assets = assets;
        Ok(context)
    }

    pub(crate) fn from_send_options(
        config: Arc<MystikoConfig>,
        deposit: &Document<Deposit>,
        options: &SendDepositOptions,
    ) -> Result<Self> {
        let bridge_type: mystiko_types::BridgeType = BridgeType::from_i32(deposit.data.bridge_type)
            .unwrap_or_default()
            .into();
        let contract_config = config
            .find_deposit_contract_by_address(deposit.data.chain_id, &deposit.data.contract_address)
            .ok_or(DepositHandlerV1Error::NoDepositContractFoundError(
                deposit.data.chain_id,
                deposit.data.contract_address.clone(),
                deposit.data.dst_chain_id,
                bridge_type,
            ))?
            .clone();
        let assets = create_assets_map(
            deposit.data.chain_id,
            &contract_config,
            deposit.data.amount,
            deposit.data.rollup_fee_amount,
            deposit.data.bridge_fee_amount.unwrap_or_default(),
            deposit.data.executor_fee_amount.unwrap_or_default(),
        )?;
        let mut context = Self::from_contract_config(deposit.data.chain_id, config, contract_config)?;
        context.assets = assets;
        context.query_remote_timeout_ms = options.query_remote_timeout_ms;
        Ok(context)
    }

    pub(crate) fn from_contract_config(
        chain_id: u64,
        config: Arc<MystikoConfig>,
        contract_config: DepositContractConfig,
    ) -> Result<Self> {
        let chain_config = config
            .find_chain(chain_id)
            .ok_or(DepositHandlerV1Error::UnsupportedChainIdError(chain_id))?
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
            .build())
    }
}

impl AssetContext {
    pub(crate) fn new(chain_id: u64, asset_config: AssetConfig, amount: f64) -> Result<Self> {
        let converted_amount = number_to_decimal(amount, Some(asset_config.asset_decimals()))?;
        let converted_amount = U256::from_dec_str(&converted_amount.to_string())?;
        Ok(Self {
            chain_id,
            asset_config,
            amount,
            converted_amount,
        })
    }

    pub(crate) async fn approve<A, S>(
        &self,
        contract_config: &DepositContractConfig,
        transaction_type: &mystiko_types::TransactionType,
        transaction: &Option<Transaction>,
        assets: &A,
        signer: Arc<S>,
    ) -> Result<Option<TxHash>>
    where
        A: PublicAssetHandler,
        S: TransactionSigner + 'static,
        DepositHandlerV1Error: From<A::Error>,
    {
        if self.asset_config.asset_type() != &mystiko_types::AssetType::Main && self.converted_amount.gt(&U256::zero())
        {
            let asset_address = ethers_address_from_string(self.asset_config.asset_address())?;
            let contract_address = ethers_address_from_string(contract_config.address())?;
            let options = Erc20ApproveOptions::<TypedTransaction, S>::builder()
                .chain_id(self.chain_id)
                .asset_address(asset_address)
                .owner(signer.address().await?)
                .recipient(contract_address)
                .amount(self.converted_amount)
                .signer(signer)
                .tx(convert_transaction(transaction_type, transaction)?)
                .build();
            return Ok(assets.erc20_approve(options).await?);
        }
        Ok(None)
    }

    pub(crate) async fn validate_balance<A>(&self, owner: &Address, assets: &A) -> Result<()>
    where
        A: PublicAssetHandler,
        DepositHandlerV1Error: From<A::Error>,
    {
        if self.converted_amount.gt(&U256::zero()) {
            let balance = if self.asset_config.asset_type() == &mystiko_types::AssetType::Main {
                let options = BalanceOptions::builder().chain_id(self.chain_id).owner(*owner).build();
                assets.balance_of(options).await?
            } else {
                let asset_address = ethers_address_from_string(self.asset_config.asset_address())?;
                let options = Erc20BalanceOptions::builder()
                    .chain_id(self.chain_id)
                    .asset_address(asset_address)
                    .owner(*owner)
                    .build();
                assets.erc20_balance_of(options).await?
            };
            if balance.lt(&self.converted_amount) {
                return Err(DepositHandlerV1Error::InsufficientBalanceError(
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
        Err(DepositHandlerV1Error::NoDepositContractFoundError(
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
) -> Result<HashMap<String, AssetContext>> {
    let mut assets = HashMap::from([(
        contract_config.asset().asset_address().to_string(),
        AssetContext::new(chain_id, contract_config.asset().clone(), amount + rollup_fee_amount)?,
    )]);
    if contract_config.bridge_type() != &mystiko_types::BridgeType::Loop {
        if bridge_fee_amount > 0_f64 {
            let bridge_fee_asset = contract_config.bridge_fee_asset().clone();
            if let Some(asset_context) = assets.get_mut(bridge_fee_asset.asset_address()) {
                asset_context.amount += bridge_fee_amount;
            } else {
                assets.insert(
                    bridge_fee_asset.asset_address().to_string(),
                    AssetContext::new(chain_id, bridge_fee_asset, bridge_fee_amount)?,
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
                    AssetContext::new(chain_id, executor_fee_asset, executor_fee_amount)?,
                );
            }
        }
    }
    Ok(assets)
}
