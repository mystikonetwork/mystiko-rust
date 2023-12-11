use crate::core::handler::v1::{AmountRange, SpendQuote, SpendSummary};
use num_bigint::{BigUint, ParseBigIntError};
use std::str::FromStr;

impl AmountRange {
    pub fn decimal_min_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.decimal_min)
    }

    pub fn decimal_max_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.decimal_max)
    }
}

impl SpendQuote {
    pub fn current_decimal_balance_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.current_decimal_balance)
    }

    pub fn min_rollup_fee_decimal_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.min_rollup_fee_decimal)
    }

    pub fn max_gas_relayer_fee_decimal_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.max_gas_relayer_fee_decimal
            .as_ref()
            .map(|amount| BigUint::from_str(amount))
            .transpose()
    }

    pub fn fixed_decimal_amounts_as_biguint(&self) -> Result<Vec<BigUint>, ParseBigIntError> {
        self.fixed_decimal_amounts
            .iter()
            .map(|amount| BigUint::from_str(amount))
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn selected_commitments_as_biguint(&self) -> Result<Vec<BigUint>, ParseBigIntError> {
        self.selected_commitments
            .iter()
            .map(|commitment| BigUint::from_str(commitment))
            .collect::<Result<Vec<_>, _>>()
    }
}

impl SpendSummary {
    pub fn current_decimal_balance_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.current_decimal_balance)
    }

    pub fn new_decimal_balance_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.new_decimal_balance)
    }

    pub fn decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.decimal_amount)
    }

    pub fn rollup_fee_decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.rollup_fee_decimal_amount)
    }

    pub fn rollup_fee_total_decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.rollup_fee_total_decimal_amount)
    }

    pub fn gas_relayer_fee_decimal_amount_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.gas_relayer_fee_decimal_amount
            .as_ref()
            .map(|amount| BigUint::from_str(amount))
            .transpose()
    }
}
