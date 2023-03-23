use crate::raw::validator::{array_unique, is_ethereum_address, is_number_string_vec};
use crate::types::AssetType;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RawAssetConfig {
    #[serde(rename = "assetType")]
    pub asset_type: AssetType,

    #[validate(length(min = 1))]
    pub asset_symbol: String,

    #[validate(range(min = 1))]
    pub asset_decimals: u32,

    #[validate(custom = "is_ethereum_address")]
    pub asset_address: String,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "is_number_string_vec::<true>")
    )]
    #[serde(default)]
    #[builder(default = vec![])]
    pub recommended_amounts: Vec<String>,
}

impl Hash for RawAssetConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.asset_address.hash(state)
    }
}

impl PartialEq for RawAssetConfig {
    fn eq(&self, other: &Self) -> bool {
        self.asset_address == other.asset_address
    }
}
