use crate::{format_deposit_log, format_tx_hash, DepositContext};
use crate::{
    BalanceOptions, Deposit, Deposits, DepositsError, Erc20ApproveOptions, Erc20BalanceOptions, PublicAssetHandler,
    TransactionHandler, TransactionSigner, WaitOptions,
};
use ethers_core::abi::AbiEncode;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash, U256};
use mystiko_config::AssetConfig;
use mystiko_protos::core::handler::v1::SendDepositOptions;
use mystiko_protos::core::v1::{DepositStatus, Transaction};
use mystiko_storage::{Document, StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::number_to_u256_decimal;
use std::collections::HashMap;
use std::sync::Arc;
use typed_builder::TypedBuilder;

impl<F, S, A, D, C, T, P> Deposits<F, S, A, D, C, T, P>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    T: TransactionHandler<Transaction>,
    DepositsError: From<A::Error> + From<T::Error>,
{
    pub(crate) async fn execute_assets_approve<Signer>(
        &self,
        context: &DepositContext,
        options: &SendDepositOptions,
        mut deposit: Document<Deposit>,
        signer: Arc<Signer>,
        owner: Address,
    ) -> Result<Document<Deposit>, DepositsError>
    where
        Signer: TransactionSigner + 'static,
    {
        let asset_amounts = AssetAmounts::builder()
            .amount(deposit.data.amount)
            .rollup_fee_amount(deposit.data.rollup_fee_amount)
            .bridge_fee_amount(deposit.data.bridge_fee_amount)
            .executor_fee_amount(deposit.data.executor_fee_amount)
            .build();
        let assets_context = asset_amounts.to_contexts(context, options.query_timeout_ms)?;
        self.validate_assets_balance(context, &assets_context, &owner).await?;
        let asset_approve_tx_hashes = self
            .assets_approve(context, &assets_context, options, signer, owner)
            .await?;
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
                format_tx_hash(self.config.clone(), deposit.data.chain_id, tx_hash).unwrap_or_default(),
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
                format_tx_hash(self.config.clone(), deposit.data.chain_id, tx_hash).unwrap_or_default(),
                format_deposit_log(&deposit),
            )
        }
        Ok(deposit)
    }
    pub(crate) async fn assets_approve<Signer>(
        &self,
        context: &DepositContext,
        assets_context: &HashMap<String, AssetContext>,
        options: &SendDepositOptions,
        signer: Arc<Signer>,
        owner: Address,
    ) -> Result<Vec<(String, TxHash)>, DepositsError>
    where
        Signer: TransactionSigner + 'static,
    {
        let mut tx_hashes = vec![];
        for (_, asset_context) in assets_context.iter() {
            if let Some(tx_hash) = self
                .asset_approve(context, asset_context, options, signer.clone(), owner)
                .await?
            {
                tx_hashes.push((asset_context.asset_config.asset_symbol().to_string(), tx_hash));
            }
        }
        Ok(tx_hashes)
    }

    pub(crate) async fn asset_approve<Signer>(
        &self,
        context: &DepositContext,
        asset_context: &AssetContext,
        options: &SendDepositOptions,
        signer: Arc<Signer>,
        owner: Address,
    ) -> Result<Option<TxHash>, DepositsError>
    where
        Signer: TransactionSigner + 'static,
    {
        if asset_context.asset_config.asset_type() != &mystiko_types::AssetType::Main
            && asset_context.converted_amount.gt(&U256::zero())
        {
            let asset_address = ethers_address_from_string(asset_context.asset_config.asset_address())?;
            let contract_address = ethers_address_from_string(context.contract_config.address())?;
            let options = Erc20ApproveOptions::<TypedTransaction, Signer>::builder()
                .chain_id(context.chain_config.chain_id())
                .asset_address(asset_address)
                .owner(owner)
                .recipient(contract_address)
                .amount(asset_context.converted_amount)
                .signer(signer)
                .tx(self.transactions.create(
                    options.asset_approve_tx.clone(),
                    context.chain_config.transaction_type(),
                )?)
                .timeout_ms(options.tx_send_timeout_ms)
                .build();
            return Ok(self.assets.erc20_approve(options).await?);
        }
        Ok(None)
    }

    pub(crate) async fn validate_assets_balance(
        &self,
        context: &DepositContext,
        assets_context: &HashMap<String, AssetContext>,
        owner: &Address,
    ) -> Result<(), DepositsError> {
        let validations = assets_context
            .values()
            .map(|asset_context| self.validate_asset_balance(context, asset_context, owner))
            .collect::<Vec<_>>();
        futures::future::try_join_all(validations).await?;
        Ok(())
    }

    pub(crate) async fn validate_asset_balance(
        &self,
        context: &DepositContext,
        asset_context: &AssetContext,
        owner: &Address,
    ) -> Result<(), DepositsError> {
        if asset_context.converted_amount.gt(&U256::zero()) {
            let balance = if asset_context.asset_config.asset_type() == &mystiko_types::AssetType::Main {
                let options = BalanceOptions::builder()
                    .chain_id(context.chain_config.chain_id())
                    .owner(*owner)
                    .timeout_ms(asset_context.query_timeout_ms)
                    .build();
                self.assets.balance_of(options).await?
            } else {
                let asset_address = ethers_address_from_string(asset_context.asset_config.asset_address())?;
                let options = Erc20BalanceOptions::builder()
                    .chain_id(context.chain_config.chain_id())
                    .asset_address(asset_address)
                    .owner(*owner)
                    .timeout_ms(asset_context.query_timeout_ms)
                    .build();
                self.assets.erc20_balance_of(options).await?
            };
            if balance.lt(&asset_context.converted_amount) {
                return Err(DepositsError::InsufficientBalanceError(
                    asset_context.asset_config.asset_symbol().to_string(),
                    asset_context.amount,
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub(crate) struct AssetContext {
    pub(crate) asset_config: AssetConfig,
    pub(crate) amount: f64,
    pub(crate) converted_amount: U256,
    pub(crate) query_timeout_ms: Option<u64>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub(crate) struct AssetAmounts {
    pub(crate) amount: f64,
    pub(crate) rollup_fee_amount: f64,
    #[builder(default)]
    pub(crate) bridge_fee_amount: Option<f64>,
    #[builder(default)]
    pub(crate) executor_fee_amount: Option<f64>,
}

impl AssetContext {
    pub(crate) fn new(
        asset_config: AssetConfig,
        amount: f64,
        query_timeout_ms: Option<u64>,
    ) -> Result<Self, DepositsError> {
        let converted_amount = number_to_u256_decimal(amount, Some(asset_config.asset_decimals()))?;
        Ok(Self {
            asset_config,
            amount,
            converted_amount,
            query_timeout_ms,
        })
    }
}
impl AssetAmounts {
    pub(crate) fn to_contexts(
        &self,
        deposit_context: &DepositContext,
        query_timeout_ms: Option<u64>,
    ) -> Result<HashMap<String, AssetContext>, DepositsError> {
        let mut assets = HashMap::from([(
            deposit_context.contract_config.asset().asset_address().to_string(),
            AssetContext::new(
                deposit_context.contract_config.asset().clone(),
                self.amount + self.rollup_fee_amount,
                query_timeout_ms,
            )?,
        )]);
        if deposit_context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop {
            let bridge_fee_amount = self.bridge_fee_amount.unwrap_or_default();
            let executor_fee_amount = self.executor_fee_amount.unwrap_or_default();
            if bridge_fee_amount > 0_f64 {
                let bridge_fee_asset = deposit_context.contract_config.bridge_fee_asset().clone();
                update_assets_map(&mut assets, bridge_fee_asset, bridge_fee_amount, query_timeout_ms)
            }
            if executor_fee_amount > 0_f64 {
                let executor_fee_asset = deposit_context.contract_config.executor_fee_asset().clone();
                update_assets_map(&mut assets, executor_fee_asset, executor_fee_amount, query_timeout_ms)
            }
        }
        Ok(assets)
    }
}

fn update_assets_map(
    assets: &mut HashMap<String, AssetContext>,
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
            AssetContext::new(asset_config, amount, query_timeout_ms).unwrap(),
        );
    }
}
