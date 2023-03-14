use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub contract_id: u64,
    pub chain_id: u64,
    pub version: String,
    pub name: String,
    pub address: String,
    pub chain_asset_symbol: String,
    pub peer_chain_id: Option<u32>,
    pub peer_contract_address: Option<String>,
    pub r#type: String,
    pub start_block: Option<u64>,
    pub bridge_type: Option<String>,
    pub disabled: bool,
    pub pool_address: Option<String>,
    pub pool_asset_address: Option<String>,
}
