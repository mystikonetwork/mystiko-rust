use crate::raw::gas_cost::RawGasCostConfig;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawTransactionInfoConfig {
    #[validate]
    main_gas_cost: RawGasCostConfig,

    #[validate]
    erc20_gas_cost: RawGasCostConfig,
}
