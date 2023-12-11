use crate::handler::DepositContext;
use crate::{DepositContractHandler, Deposits, DepositsError};
use mystiko_protos::core::handler::v1::{DepositQuote, QuoteDepositOptions};
use mystiko_storage::{StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::decimal_to_number;

impl<F, S, A, D, C, T, P> Deposits<F, S, A, D, C, T, P>
where
    F: StatementFormatter,
    S: Storage,
    D: DepositContractHandler,
    DepositsError: From<D::Error>,
{
    pub(crate) async fn execute_quote(&self, options: &QuoteDepositOptions) -> Result<DepositQuote, DepositsError> {
        let context = DepositContext::from_quote_options(self.config.clone(), options)?;
        let asset_decimals = context.contract_config.asset_decimals();
        let quote_options = crate::DepositQuoteOptions::builder()
            .chain_id(options.chain_id)
            .contract_address(ethers_address_from_string(context.contract_config.address())?)
            .timeout_ms(options.query_timeout_ms)
            .build();
        let quote = self.deposit_contracts.quote(quote_options).await?;
        let min_amount: f64 = decimal_to_number(&quote.min_amount, Some(asset_decimals))?;
        let max_amount: f64 = decimal_to_number(&quote.max_amount, Some(asset_decimals))?;
        let min_rollup_fee_amount: f64 = decimal_to_number(
            &quote.min_rollup_fee_amount,
            Some(context.contract_config.asset_decimals()),
        )?;
        let min_bridge_fee_amount: Option<f64> = quote
            .min_bridge_fee_amount
            .map(|v| decimal_to_number(&v, Some(context.contract_config.bridge_fee_asset().asset_decimals())))
            .transpose()?;
        let min_executor_fee_amount: Option<f64> = quote
            .min_executor_fee_amount
            .map(|v| decimal_to_number(&v, Some(context.contract_config.executor_fee_asset().asset_decimals())))
            .transpose()?;
        let rollup_fee_asset_symbol = context.contract_config.asset_symbol().to_string();
        let bridge_fee_asset_symbol = (context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(context.contract_config.bridge_fee_asset().asset_symbol().to_string());
        let bridge_fee_asset_decimals = (context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(context.contract_config.bridge_fee_asset().asset_decimals());
        let executor_fee_asset_symbol = (context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(context.contract_config.executor_fee_asset().asset_symbol().to_string());
        let executor_fee_asset_decimals = (context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(context.contract_config.executor_fee_asset().asset_decimals());
        let recommended_amounts = context.contract_config.recommended_amounts_number::<f64>()?;
        let recommended_decimal_amounts = context
            .contract_config
            .recommended_amounts()?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>();
        Ok(DepositQuote::builder()
            .asset_symbol(context.contract_config.asset_symbol().to_string())
            .asset_decimals(asset_decimals)
            .min_amount(min_amount)
            .min_decimal_amount(quote.min_amount.to_string())
            .max_amount(max_amount)
            .max_decimal_amount(quote.max_amount.to_string())
            .min_rollup_fee_amount(min_rollup_fee_amount)
            .min_rollup_fee_decimal_amount(quote.min_rollup_fee_amount.to_string())
            .min_bridge_fee_amount(min_bridge_fee_amount)
            .min_bridge_fee_decimal_amount(quote.min_bridge_fee_amount.map(|v| v.to_string()))
            .min_executor_fee_amount(min_executor_fee_amount)
            .min_executor_fee_decimal_amount(quote.min_executor_fee_amount.map(|v| v.to_string()))
            .rollup_fee_asset_symbol(rollup_fee_asset_symbol)
            .rollup_fee_asset_decimals(asset_decimals)
            .bridge_fee_asset_symbol(bridge_fee_asset_symbol)
            .bridge_fee_asset_decimals(bridge_fee_asset_decimals)
            .executor_fee_asset_symbol(executor_fee_asset_symbol)
            .executor_fee_asset_decimals(executor_fee_asset_decimals)
            .recommended_amounts(recommended_amounts)
            .recommended_decimal_amounts(recommended_decimal_amounts)
            .build())
    }
}
