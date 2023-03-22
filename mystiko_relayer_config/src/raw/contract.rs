use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawContractConfig {
    #[validate(length(min = 1))]
    pub asset_symbol: String,

    #[validate(range(min = 0))]
    pub relayer_fee_of_ten_thousandth: u32,

    #[serde(skip_serializing)]
    #[builder(default = default_minimum_gas_fee())]
    pub minimum_gas_fee: String,
}

fn default_minimum_gas_fee() -> String {
    "0".to_string()
}