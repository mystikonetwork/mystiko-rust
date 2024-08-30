use crate::handler::{AssetAmounts, DepositContext};
use crate::{DepositContractHandler, Deposits, DepositsError};
use mystiko_protos::core::handler::v1::{CreateDepositOptions, DepositQuote, DepositSummary, QuoteDepositOptions};
use mystiko_storage::{StatementFormatter, Storage};
use mystiko_utils::convert::number_to_biguint_decimal;
use std::collections::HashMap;

impl<F, S, A, D, C, T, P, N> Deposits<F, S, A, D, C, T, P, N>
where
    F: StatementFormatter,
    S: Storage,
    D: DepositContractHandler,
    DepositsError: From<D::Error>,
{
    pub(crate) async fn execute_summary(
        &self,
        options: &CreateDepositOptions,
    ) -> Result<DepositSummary, DepositsError> {
        let context = DepositContext::from_create_options(self.config.clone(), options)?;
        let quote = self.validate_amounts(options).await?;
        let bridge_fee_asset_symbol = (context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(context.contract_config.bridge_fee_asset().asset_symbol().to_string());
        let executor_fee_asset_symbol = (context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(context.contract_config.executor_fee_asset().asset_symbol().to_string());
        let rollup_fee_amount = options.rollup_fee_amount.unwrap_or(quote.min_rollup_fee_amount);
        let rollup_fee_decimal_amount =
            number_to_biguint_decimal(rollup_fee_amount, Some(quote.rollup_fee_asset_decimals))?.to_string();
        let bridge_fee_amount = (context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(options.bridge_fee_amount.unwrap_or(quote.min_bridge_fee_amount()));
        let bridge_fee_decimal_amount = bridge_fee_amount
            .map(|v| number_to_biguint_decimal(v, quote.bridge_fee_asset_decimals))
            .transpose()?
            .map(|v| v.to_string());
        let executor_fee_amount = (context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop)
            .then_some(options.executor_fee_amount.unwrap_or(quote.min_executor_fee_amount()));
        let executor_fee_decimal_amount = executor_fee_amount
            .map(|v| number_to_biguint_decimal(v, quote.executor_fee_asset_decimals))
            .transpose()?
            .map(|v| v.to_string());
        let asset_amounts = AssetAmounts::builder()
            .amount(options.amount)
            .rollup_fee_amount(rollup_fee_amount)
            .bridge_fee_amount(bridge_fee_amount)
            .executor_fee_amount(executor_fee_amount)
            .build();
        let assets_context = asset_amounts.to_contexts(&context, options.query_timeout_ms)?;
        let total_amounts = assets_context
            .values()
            .map(|asset_context| {
                (
                    asset_context.asset_config.asset_symbol().to_string(),
                    asset_context.amount,
                )
            })
            .collect::<HashMap<_, _>>();
        let total_decimal_amounts = assets_context
            .values()
            .map(|asset_context| {
                (
                    asset_context.asset_config.asset_symbol().to_string(),
                    asset_context.converted_amount.to_string(),
                )
            })
            .collect::<HashMap<_, _>>();
        let decimal_amount = number_to_biguint_decimal(options.amount, Some(quote.asset_decimals))?.to_string();
        Ok(DepositSummary::builder()
            .chain_id(options.chain_id)
            .asset_symbol(options.asset_symbol.clone())
            .asset_decimals(quote.asset_decimals)
            .amount(options.amount)
            .decimal_amount(decimal_amount)
            .shielded_address(options.shielded_address.clone())
            .rollup_fee_amount(rollup_fee_amount)
            .rollup_fee_decimal_amount(rollup_fee_decimal_amount)
            .rollup_fee_asset_symbol(context.contract_config.asset_symbol().to_string())
            .rollup_fee_asset_decimals(quote.rollup_fee_asset_decimals)
            .dst_chain_id(options.dst_chain_id)
            .bridge_fee_amount(bridge_fee_amount)
            .bridge_fee_decimal_amount(bridge_fee_decimal_amount)
            .bridge_fee_asset_symbol(bridge_fee_asset_symbol)
            .bridge_fee_asset_decimals(quote.bridge_fee_asset_decimals)
            .executor_fee_amount(executor_fee_amount)
            .executor_fee_decimal_amount(executor_fee_decimal_amount)
            .executor_fee_asset_symbol(executor_fee_asset_symbol)
            .executor_fee_asset_decimals(quote.executor_fee_asset_decimals)
            .bridge_type(options.bridge_type)
            .total_amounts(total_amounts)
            .total_decimal_amounts(total_decimal_amounts)
            .build())
    }

    pub(crate) async fn validate_amounts(&self, options: &CreateDepositOptions) -> Result<DepositQuote, DepositsError>
    where
        D: DepositContractHandler,
        DepositsError: From<D::Error>,
    {
        let quote = if let Some(quote) = options.deposit_quote.clone() {
            quote
        } else {
            let quote_options = QuoteDepositOptions::builder()
                .chain_id(options.chain_id)
                .asset_symbol(options.asset_symbol.clone())
                .bridge_type(options.bridge_type)
                .dst_chain_id(options.dst_chain_id)
                .query_timeout_ms(options.query_timeout_ms)
                .build();
            self.execute_quote(&quote_options).await?
        };
        let amount = number_to_biguint_decimal(options.amount, Some(quote.asset_decimals))?;
        let rollup_fee_amount = options.rollup_fee_amount.unwrap_or(quote.min_rollup_fee_amount);
        let rollup_fee_decimal_amount = number_to_biguint_decimal(rollup_fee_amount, Some(quote.asset_decimals))?;
        if amount.lt(&quote.min_decimal_amount_as_biguint()?) || amount.gt(&quote.max_decimal_amount_as_biguint()?) {
            return Err(DepositsError::InvalidDepositAmountError(
                options.amount,
                quote.min_amount,
                quote.max_amount,
            ));
        }
        if rollup_fee_decimal_amount.lt(&quote.min_rollup_fee_decimal_amount_as_biguint()?) {
            return Err(DepositsError::InvalidRollupFeeAmountError(
                rollup_fee_amount,
                quote.min_rollup_fee_amount,
            ));
        }
        if let Some(min_bridge_fee_amount) = quote.min_bridge_fee_decimal_amount_as_biguint()? {
            let bridge_fee_amount = number_to_biguint_decimal(
                options.bridge_fee_amount.unwrap_or(quote.min_bridge_fee_amount()),
                Some(quote.bridge_fee_asset_decimals()),
            )?;
            if bridge_fee_amount.lt(&min_bridge_fee_amount) {
                return Err(DepositsError::InvalidBridgeFeeAmountError(
                    options.bridge_fee_amount(),
                    quote.min_bridge_fee_amount(),
                ));
            }
        }
        if let Some(min_executor_fee_amount) = quote.min_executor_fee_decimal_amount_as_biguint()? {
            let executor_fee_amount = number_to_biguint_decimal(
                options.executor_fee_amount.unwrap_or(quote.min_executor_fee_amount()),
                Some(quote.executor_fee_asset_decimals()),
            )?;
            if executor_fee_amount.lt(&min_executor_fee_amount) {
                return Err(DepositsError::InvalidExecutorFeeAmountError(
                    options.executor_fee_amount(),
                    quote.min_executor_fee_amount(),
                ));
            }
        }
        Ok(quote)
    }
}
