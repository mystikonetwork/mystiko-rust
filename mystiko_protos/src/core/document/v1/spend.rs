use crate::core::document::v1::Spend;
use num_bigint::{BigUint, ParseBigIntError};
use std::str::FromStr;

impl Spend {
    pub fn decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.decimal_amount)
    }

    pub fn rollup_fee_decimal_amount_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.rollup_fee_decimal_amount
            .as_ref()
            .map(|s| BigUint::from_str(s))
            .transpose()
    }

    pub fn gas_relayer_fee_decimal_amount_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.gas_relayer_fee_decimal_amount
            .as_ref()
            .map(|s| BigUint::from_str(s))
            .transpose()
    }

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

    pub fn signature_public_key_hashes_as_biguint(&self) -> Result<Option<Vec<BigUint>>, ParseBigIntError> {
        if self.signature_public_key_hashes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(
                self.signature_public_key_hashes
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

    pub fn encrypted_auditor_notes_as_biguint(&self) -> Result<Option<Vec<BigUint>>, ParseBigIntError> {
        if self.encrypted_auditor_notes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(
                self.encrypted_auditor_notes
                    .iter()
                    .map(|n| BigUint::from_str(n))
                    .collect::<Result<_, _>>()?,
            ))
        }
    }
}
