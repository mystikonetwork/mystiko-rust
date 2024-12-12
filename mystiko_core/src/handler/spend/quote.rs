use crate::{
    calc_commitments_fixed_amounts, select_commitments, spend_circuit_type, Accounts, Commitment, CommitmentColumn,
    CommitmentPoolContractHandler, MinRollupFeeOptions, SpendContext, Spends, SpendsError, MAX_NUM_INPUTS,
    MAX_NUM_OUTPUTS,
};
use mystiko_protos::core::handler::v1::{AmountRange, GasRelayer, QuoteSpendOptions, SpendInvalidCode, SpendQuote};
use mystiko_protos::core::v1::SpendType;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_relayer_client::types::register::RegisterInfo;
use mystiko_relayer_client::RelayerClient;
use mystiko_relayer_types::{RegisterInfoRequest, RegisterOptions};
use mystiko_storage::{Document, StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{decimal_to_number, number_to_biguint_decimal, u256_to_biguint};
use num_bigint::BigUint;
use std::ops::{Mul, Sub};
use std::str::FromStr;
use std::time::Duration;

impl<F, S, A, C, T, P, R, V, K, X> Spends<F, S, A, C, T, P, R, V, K, X>
where
    F: StatementFormatter,
    S: Storage,
    C: CommitmentPoolContractHandler,
    R: RelayerClient,
    SpendsError: From<C::Error> + From<R::Error>,
{
    pub(crate) async fn execute_quote(&self, options: &QuoteSpendOptions) -> Result<SpendQuote, SpendsError> {
        let context = SpendContext::from_quote_options(self.config.clone(), options)?;
        let commitments = self.get_spendable_commitments(&context).await?;
        let mut quote = self
            .execute_quote_without_amount(&context, &commitments, options)
            .await?;
        if !quote.valid {
            return Ok(quote);
        }
        if let Some(amount) = options.amount {
            let amount = number_to_biguint_decimal(amount, Some(context.contract_config.asset_decimals()))?;
            let mut valid_amount = false;
            if let Some(amount_range) = quote.amount_range.as_ref() {
                if amount.gt(&amount_range.decimal_min_as_biguint()?)
                    && amount.le(&amount_range.decimal_max_as_biguint()?)
                {
                    valid_amount = true;
                }
            }
            if !valid_amount {
                for fixed_amount in quote.fixed_decimal_amounts_as_biguint()?.iter() {
                    if amount.eq(fixed_amount) {
                        valid_amount = true;
                        break;
                    }
                }
            }
            if !valid_amount {
                quote.valid = false;
                quote.invalid_code = Some(SpendInvalidCode::InvalidAmount as i32);
                return Ok(quote);
            }
            let selected = select_commitments(&commitments, Some(amount.clone()), MAX_NUM_INPUTS);
            let selected_sum = selected
                .iter()
                .filter_map(|commitment| commitment.data.amount.clone())
                .sum::<BigUint>();
            quote.selected_commitments = selected
                .iter()
                .map(|commitment| commitment.data.commitment_hash.to_string())
                .collect::<Vec<_>>();
            quote.num_of_inputs = selected.len() as u64;
            if selected_sum.ne(&amount) {
                quote.num_of_outputs += 1;
            }
            if options.use_relayer() {
                quote.gas_relayer_fee_asset_symbol = Some(quote.asset_symbol.to_string());
                quote.gas_relayer_fee_asset_decimals = Some(quote.asset_decimals);
                let gas_relayers = self
                    .query_gas_relayer(
                        &context,
                        quote.num_of_inputs,
                        quote.num_of_outputs,
                        options.query_timeout_ms,
                        None,
                    )
                    .await?;
                quote.gas_relayers = build_gas_relayers(
                    gas_relayers,
                    context.contract_config.asset_symbol(),
                    context.contract_config.asset_decimals(),
                )?;
                let total_rollup_fee_amount = quote
                    .min_rollup_fee_decimal_as_biguint()?
                    .mul(BigUint::from(quote.num_of_outputs));
                let max_gas_relayer_fee = amount.sub(&total_rollup_fee_amount);
                let max_gas_relayer_fee_number =
                    decimal_to_number::<f64, _>(&max_gas_relayer_fee, Some(context.contract_config.asset_decimals()))?;
                quote.max_gas_relayer_fee = Some(max_gas_relayer_fee_number);
                quote.max_gas_relayer_fee_decimal = Some(max_gas_relayer_fee.to_string());
            }
        }
        if quote.num_of_outputs > 0 && context.contract_config.disabled() {
            quote.valid = false;
            quote.invalid_code = Some(SpendInvalidCode::SplitDisabled as i32);
            return Ok(quote);
        }
        Ok(quote)
    }

    pub(crate) async fn query_gas_relayer(
        &self,
        context: &SpendContext,
        num_inputs: u64,
        num_outputs: u64,
        query_timeout_ms: Option<u64>,
        relayer_name: Option<String>,
    ) -> Result<Vec<RegisterInfo>, SpendsError> {
        let register_options = RegisterOptions::builder()
            .asset_symbol(context.contract_config.asset_symbol().to_string())
            .circuit_type(spend_circuit_type(num_inputs, num_outputs)?)
            .show_unavailable(false)
            .build();
        let request = RegisterInfoRequest::builder()
            .chain_id(context.chain_id)
            .name(relayer_name)
            .options(Some(register_options))
            .build();
        if let Some(query_timeout_ms) = query_timeout_ms {
            let result = tokio::time::timeout(
                Duration::from_millis(query_timeout_ms),
                self.relayers.register_info(request),
            )
            .await;
            match result {
                Err(_) => {
                    log::warn!("querying relayer information timed out after {}ms", query_timeout_ms);
                    Ok(vec![])
                }
                Ok(result) => Ok(result?),
            }
        } else {
            Ok(self.relayers.register_info(request).await?)
        }
    }

    async fn execute_quote_without_amount(
        &self,
        context: &SpendContext,
        commitments: &[Document<Commitment>],
        options: &QuoteSpendOptions,
    ) -> Result<SpendQuote, SpendsError> {
        if commitments.is_empty() {
            return Ok(SpendQuote::builder()
                .valid(false)
                .invalid_code(Some(SpendInvalidCode::NoAvailableAssets as i32))
                .build());
        }
        let balance: BigUint = commitments
            .iter()
            .filter_map(|commitment| commitment.data.amount.clone())
            .sum();
        let asset_symbol = context.contract_config.asset_symbol();
        let asset_decimals = context.contract_config.asset_decimals();
        let balance_number: f64 = decimal_to_number(&balance, Some(asset_decimals))?;
        let (min_rollup_fee_number, min_rollup_fee) =
            self.get_min_rollup_fee(context, options.query_timeout_ms).await?;
        let max_commitments = select_commitments(commitments, None, MAX_NUM_INPUTS);
        let max_amount: BigUint = max_commitments
            .iter()
            .filter_map(|commitment| commitment.data.amount.clone())
            .sum();
        let max_amount_number = decimal_to_number::<f64, _>(&max_amount, Some(asset_decimals))?;
        let spend_type = options.spend_type.unwrap_or(SpendType::Withdraw as i32);
        let spend_type = SpendType::from_i32(spend_type).unwrap_or_default();
        let max_num_outputs = match spend_type {
            SpendType::Transfer => MAX_NUM_OUTPUTS,
            _ => MAX_NUM_OUTPUTS - 1,
        };
        let min_num_outputs = match spend_type {
            SpendType::Transfer => 1_usize,
            _ => 0_usize,
        };
        let fixed_amount_threshold = min_rollup_fee.clone().mul(BigUint::from(max_num_outputs));
        let fixed_amount_threshold_number: f64 = decimal_to_number(&fixed_amount_threshold, Some(asset_decimals))?;
        let fixed_amounts = calc_commitments_fixed_amounts(commitments, &fixed_amount_threshold, MAX_NUM_INPUTS)
            .into_iter()
            .filter(|fixed_amount| fixed_amount.gt(&min_rollup_fee.clone().mul(min_num_outputs)))
            .collect::<Vec<_>>();
        let fixed_amounts_string = fixed_amounts
            .iter()
            .map(|amount| amount.to_string())
            .collect::<Vec<_>>();
        let fixed_amounts_number = fixed_amounts
            .iter()
            .map(|amount| decimal_to_number::<f64, _>(amount, Some(asset_decimals)))
            .collect::<anyhow::Result<Vec<_>>>()?;
        let amount_range = AmountRange::builder()
            .min(fixed_amount_threshold_number)
            .decimal_min(fixed_amount_threshold.to_string())
            .max(max_amount_number)
            .decimal_max(max_amount.to_string())
            .build();
        Ok(SpendQuote::builder()
            .valid(true)
            .asset_symbol(asset_symbol.to_string())
            .asset_decimals(asset_decimals)
            .current_balance(balance_number)
            .current_decimal_balance(balance.to_string())
            .num_of_inputs(max_commitments.len() as u64)
            .num_of_outputs(min_num_outputs as u64)
            .min_rollup_fee(min_rollup_fee_number)
            .min_rollup_fee_decimal(min_rollup_fee.to_string())
            .rollup_fee_asset_symbol(asset_symbol.to_string())
            .rollup_fee_asset_decimals(asset_decimals)
            .amount_range(amount_range)
            .fixed_amounts(fixed_amounts_number)
            .fixed_decimal_amounts(fixed_amounts_string)
            .build())
    }

    async fn get_spendable_commitments(
        &self,
        context: &SpendContext,
    ) -> Result<Vec<Document<Commitment>>, SpendsError> {
        let accounts = Accounts::new(self.db.clone());
        let shielded_addresses = accounts
            .find_all_documents()
            .await?
            .into_iter()
            .map(|account| account.data.shielded_address)
            .collect::<Vec<_>>();
        let sub_filters = vec![
            SubFilter::equal(CommitmentColumn::ChainId, context.chain_id),
            SubFilter::equal(
                CommitmentColumn::ContractAddress,
                context.contract_config.address().to_string(),
            ),
            SubFilter::equal(CommitmentColumn::Status, CommitmentStatus::Included as i32),
            SubFilter::equal(CommitmentColumn::Spent, false),
            SubFilter::in_list(CommitmentColumn::ShieldedAddress, shielded_addresses),
            SubFilter::is_not_null(CommitmentColumn::Amount),
        ];
        Ok(self.db.commitments.find(Condition::and(sub_filters)).await?)
    }

    async fn get_min_rollup_fee(
        &self,
        context: &SpendContext,
        query_timeout_ms: Option<u64>,
    ) -> Result<(f64, BigUint), SpendsError> {
        let options = MinRollupFeeOptions::builder()
            .chain_id(context.chain_id)
            .contract_address(ethers_address_from_string(context.contract_config.address())?)
            .timeout_ms(query_timeout_ms)
            .build();
        let default_min_rollup_fee = context.contract_config.min_rollup_fee()?;
        let min_rollup_fee = self
            .commitment_pool_contracts
            .min_rollup_fee(options)
            .await
            .map(|fee| u256_to_biguint(&fee))
            .unwrap_or(default_min_rollup_fee);
        let min_rollup_fee_number: f64 =
            decimal_to_number(&min_rollup_fee, Some(context.contract_config.asset_decimals()))?;
        Ok((min_rollup_fee_number, min_rollup_fee))
    }
}

fn build_gas_relayers(
    gas_relayers: Vec<RegisterInfo>,
    asset_symbol: &str,
    asset_decimals: u32,
) -> Result<Vec<GasRelayer>, SpendsError> {
    let mut converted = Vec::new();
    for gas_relayer in gas_relayers.into_iter() {
        if let Some(contract_info) = gas_relayer
            .contracts
            .iter()
            .find(|info| info.asset_symbol == asset_symbol)
        {
            let min_gas_fee_decimal = contract_info
                .minimum_gas_fee
                .as_ref()
                .map(|fee| BigUint::from_str(fee))
                .transpose()?
                .unwrap_or_default();
            let min_gas_fee = decimal_to_number::<f64, _>(&min_gas_fee_decimal, Some(asset_decimals))?;
            converted.push(
                GasRelayer::builder()
                    .url(gas_relayer.url)
                    .name(gas_relayer.name)
                    .address(gas_relayer.relayer_address)
                    .min_gas_fee(min_gas_fee)
                    .min_gas_fee_decimal(min_gas_fee_decimal.to_string())
                    .service_fee_ratio(contract_info.relayer_fee_of_ten_thousandth as f64 / 10_000_f64)
                    .service_fee_of_ten_thousandth(contract_info.relayer_fee_of_ten_thousandth)
                    .build(),
            );
        }
    }
    Ok(converted)
}
