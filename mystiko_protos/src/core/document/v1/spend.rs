use crate::core::document::v1::Spend;
use num_bigint::{BigUint, ParseBigIntError};
use std::str::FromStr;

impl Spend {
    pub fn root_hash_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.root_hash)
    }

    pub fn input_commitments_as_biguint(&self) -> Result<Vec<BigUint>, ParseBigIntError> {
        self.input_commitments
            .iter()
            .map(|n| BigUint::from_str(n))
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn output_commitments_as_biguint(&self) -> Result<Option<Vec<BigUint>>, ParseBigIntError> {
        if self.output_commitments.is_empty() {
            Ok(None)
        } else {
            Ok(Some(
                self.output_commitments
                    .iter()
                    .map(|n| BigUint::from_str(n))
                    .collect::<Result<_, _>>()?,
            ))
        }
    }

    pub fn nullifiers_as_biguint(&self) -> Result<Option<Vec<BigUint>>, ParseBigIntError> {
        if self.nullifiers.is_empty() {
            Ok(None)
        } else {
            Ok(Some(
                self.nullifiers
                    .iter()
                    .map(|n| BigUint::from_str(n))
                    .collect::<Result<_, _>>()?,
            ))
        }
    }

    pub fn random_auditing_public_key_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.random_auditing_public_key
            .as_ref()
            .map(|n| BigUint::from_str(n))
            .transpose()
    }
}
