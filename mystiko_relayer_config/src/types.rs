use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AssetType {
    Erc20,
    Main,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CircuitType {
    Transaction1x0,
    Transaction1x1,
    Transaction1x2,
    Transaction2x0,
    Transaction2x1,
    Transaction2x2,
}
