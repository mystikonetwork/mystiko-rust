use crate::core::handler::v1::{DepositQuote, DepositSummary};
use num_bigint::{BigUint, ParseBigIntError};
use std::collections::HashMap;
use std::str::FromStr;

impl DepositQuote {
    pub fn min_decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.min_decimal_amount)
    }

    pub fn max_decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.max_decimal_amount)
    }

    pub fn min_rollup_fee_decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.min_rollup_fee_decimal_amount)
    }

    pub fn recommended_decimal_amounts_as_biguint(&self) -> Result<Vec<BigUint>, ParseBigIntError> {
        self.recommended_decimal_amounts
            .iter()
            .map(|recommended_decimal_amount| BigUint::from_str(recommended_decimal_amount))
            .collect()
    }

    pub fn min_bridge_fee_decimal_amount_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.min_bridge_fee_decimal_amount
            .as_ref()
            .map(|min_bridge_fee_decimal_amount| BigUint::from_str(min_bridge_fee_decimal_amount))
            .transpose()
    }

    pub fn min_executor_fee_decimal_amount_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.min_executor_fee_decimal_amount
            .as_ref()
            .map(|min_executor_fee_decimal_amount| BigUint::from_str(min_executor_fee_decimal_amount))
            .transpose()
    }
}

impl DepositSummary {
    pub fn decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.decimal_amount)
    }

    pub fn rollup_fee_decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.rollup_fee_decimal_amount)
    }

    pub fn bridge_fee_decimal_amount_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.bridge_fee_decimal_amount
            .as_ref()
            .map(|bridge_fee_decimal_amount| BigUint::from_str(bridge_fee_decimal_amount))
            .transpose()
    }

    pub fn executor_fee_decimal_amount_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.executor_fee_decimal_amount
            .as_ref()
            .map(|executor_fee_decimal_amount| BigUint::from_str(executor_fee_decimal_amount))
            .transpose()
    }

    pub fn total_decimal_amounts_as_biguint(&self) -> Result<HashMap<String, BigUint>, ParseBigIntError> {
        self.total_decimal_amounts
            .iter()
            .map(|(asset_symbol, total_decimal_amount)| {
                BigUint::from_str(total_decimal_amount)
                    .map(|total_decimal_amount| (asset_symbol.to_string(), total_decimal_amount))
            })
            .collect::<Result<HashMap<_, _>, _>>()
    }
}
