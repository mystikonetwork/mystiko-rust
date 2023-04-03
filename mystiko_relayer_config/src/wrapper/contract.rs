use crate::raw::contract::RawContractConfig;
use rust_decimal::Decimal;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct ContractConfig {
    raw: Arc<RawContractConfig>,
}

impl ContractConfig {
    pub fn new(raw: Arc<RawContractConfig>) -> Self {
        ContractConfig { raw }
    }

    pub fn asset_symbol(&self) -> &str {
        &self.raw.asset_symbol
    }

    pub fn relayer_fee_of_ten_thousandth(&self) -> u32 {
        self.raw.relayer_fee_of_ten_thousandth
    }

    pub fn minimum_gas_fee(&self) -> &Decimal {
        &self.raw.minimum_gas_fee
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        Ok(self.raw.validate()?)
    }
}
