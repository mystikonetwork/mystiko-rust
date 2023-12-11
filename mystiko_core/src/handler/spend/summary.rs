use crate::{calc_min_gas_relayer_fee, CommitmentPoolContractHandler, SpendContext, Spends, SpendsError};
use mystiko_protos::core::handler::v1::{CreateSpendOptions, GasRelayer, QuoteSpendOptions, SpendQuote, SpendSummary};
use mystiko_protos::core::v1::SpendType;
use mystiko_relayer_client::RelayerClient;
use mystiko_storage::{StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{decimal_to_number, number_to_biguint_decimal};
use num_bigint::BigUint;
use num_traits::Zero;
use std::ops::{Add, Mul, Sub};

impl<F, S, A, C, T, P, R, V> Spends<F, S, A, C, T, P, R, V>
where
    F: StatementFormatter,
    S: Storage,
    C: CommitmentPoolContractHandler,
    R: RelayerClient,
    SpendsError: From<C::Error> + From<R::Error>,
{
    pub(crate) async fn execute_summary(
        &self,
        options: &CreateSpendOptions,
    ) -> Result<(SpendQuote, SpendSummary), SpendsError> {
        let context = SpendContext::from_create_options(self.config.clone(), options)?;
        self.execute_summary_with_context(&context, options).await
    }

    pub(crate) async fn execute_summary_with_context(
        &self,
        context: &SpendContext,
        options: &CreateSpendOptions,
    ) -> Result<(SpendQuote, SpendSummary), SpendsError> {
        let quote = if let Some(quote) = options.spend_quote.as_ref() {
            quote.clone()
        } else {
            let quote_options = QuoteSpendOptions::builder()
                .chain_id(context.chain_id)
                .asset_symbol(context.contract_config.asset_symbol().to_string())
                .bridge_type(options.bridge_type)
                .version(options.version)
                .spend_type(options.spend_type)
                .query_timeout_ms(options.query_timeout_ms)
                .amount(options.amount)
                .use_relayer(options.gas_relayer.is_some())
                .build();
            self.execute_quote(&quote_options).await?
        };
        let gas_relayer = quote
            .gas_relayers
            .iter()
            .find(|gas_relayer| options.gas_relayer == Some(gas_relayer.name.clone()));
        let (amount, rollup_fee_amount, gas_relayer_fee) = validate_create_options(options, &quote, gas_relayer)?;
        let gas_relayer_fee_amount = decimal_to_number::<f64, _>(&gas_relayer_fee, Some(quote.asset_decimals))?;
        let new_decimal_balance = quote.current_decimal_balance_as_biguint()?.sub(&amount);
        let new_balance = decimal_to_number::<f64, _>(&new_decimal_balance, Some(quote.asset_decimals))?;
        let summary = SpendSummary::builder()
            .asset_symbol(quote.asset_symbol.clone())
            .asset_decimals(quote.asset_decimals)
            .current_balance(quote.current_balance)
            .current_decimal_balance(quote.current_decimal_balance.clone())
            .new_balance(new_balance)
            .new_decimal_balance(new_decimal_balance.to_string())
            .amount(options.amount)
            .decimal_amount(amount.to_string())
            .recipient(options.recipient.clone())
            .rollup_fee_amount(options.rollup_fee_amount())
            .rollup_fee_decimal_amount(rollup_fee_amount.to_string())
            .rollup_fee_total_amount(options.rollup_fee_amount() * (quote.num_of_outputs as f64))
            .rollup_fee_total_decimal_amount(rollup_fee_amount.mul(quote.num_of_outputs).to_string())
            .rollup_fee_asset_symbol(quote.rollup_fee_asset_symbol.clone())
            .rollup_fee_asset_decimals(quote.rollup_fee_asset_decimals)
            .gas_relayer_fee_amount(gas_relayer.is_some().then_some(gas_relayer_fee_amount))
            .gas_relayer_fee_decimal_amount(gas_relayer.is_some().then_some(gas_relayer_fee.to_string()))
            .gas_relayer_fee_asset_symbol(quote.gas_relayer_fee_asset_symbol.clone())
            .gas_relayer_fee_asset_decimals(quote.gas_relayer_fee_asset_decimals)
            .gas_relayer_address(gas_relayer.map(|gas_relayer| gas_relayer.address.clone()))
            .gas_relayer_name(gas_relayer.map(|gas_relayer| gas_relayer.name.clone()))
            .gas_relayer_url(gas_relayer.map(|gas_relayer| gas_relayer.url.clone()))
            .build();
        Ok((quote, summary))
    }
}

fn validate_create_options(
    options: &CreateSpendOptions,
    quote: &SpendQuote,
    gas_relayer: Option<&GasRelayer>,
) -> Result<(BigUint, BigUint, BigUint), SpendsError> {
    if !quote.valid {
        return Err(SpendsError::InvalidCreateOptionsError(quote.invalid_code()));
    }
    if options.spend_type == Some(SpendType::Transfer as i32) {
        if !mystiko_protocol::address::ShieldedAddress::is_valid_address(&options.recipient) {
            return Err(SpendsError::InvalidMystikoAddressError(options.recipient.clone()));
        }
    } else if ethers_address_from_string(&options.recipient).is_err() {
        return Err(SpendsError::InvalidPublicAddressError(options.recipient.clone()));
    }
    let amount = number_to_biguint_decimal(options.amount, Some(quote.asset_decimals))?;
    let rollup_fee_amount = number_to_biguint_decimal(options.rollup_fee_amount(), Some(quote.asset_decimals))?;
    let gas_relayer_fee = gas_relayer
        .map(|gas_relayer| calc_min_gas_relayer_fee(&amount, gas_relayer))
        .transpose()?
        .unwrap_or_default();
    if amount.le(&gas_relayer_fee
        .clone()
        .add(&rollup_fee_amount.clone().mul(quote.num_of_outputs)))
    {
        return Err(SpendsError::InvalidAmountError(options.amount));
    }
    if (quote.num_of_outputs > 0 && rollup_fee_amount.lt(&quote.min_rollup_fee_decimal_as_biguint()?))
        || (quote.num_of_outputs == 0 && rollup_fee_amount.ne(&BigUint::zero()))
    {
        return Err(SpendsError::InvalidRollupFeeAmountError(options.rollup_fee_amount()));
    }
    Ok((amount, rollup_fee_amount, gas_relayer_fee))
}
