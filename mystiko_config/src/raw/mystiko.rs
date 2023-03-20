use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::raw::base::{RawConfig, Validator};
use crate::raw::chain::RawChainConfig;
use crate::raw::circuit::RawCircuitConfig;
use crate::raw::validator::{array_unique, is_sem_ver, validate_nested_vec};

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawMystikoConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(custom = "is_sem_ver")]
    pub version: String,

    #[validate(
    custom(function = "array_unique"),
    custom(function = "validate_nested_vec")
    )]
    pub circuits: Vec<RawCircuitConfig>,

    #[validate(
    custom(function = "array_unique"),
    custom(function = "validate_nested_vec")
    )]
    pub chains: Vec<RawChainConfig>,
}

impl Validator for RawMystikoConfig {
    fn validation(&self) -> anyhow::Result<()> {
        self.base.validate_object::<RawMystikoConfig>(self)
    }
}