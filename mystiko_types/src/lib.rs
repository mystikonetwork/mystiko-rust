use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum BridgeType {
    #[default]
    Loop,
    Poly,
    Tbridge,
    Celer,
    LayerZero,
    Axelar,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ContractType {
    Deposit,
    Pool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum AssetType {
    #[default]
    Erc20,
    Main,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
#[serde(rename_all = "lowercase")]
pub enum CircuitType {
    Rollup1,
    Rollup2,
    Rollup4,
    Rollup8,
    Rollup16,
    Transaction1x0,
    Transaction1x1,
    Transaction1x2,
    Transaction2x0,
    Transaction2x1,
    Transaction2x2,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ProviderType {
    #[default]
    Failover,
    Quorum,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AccountStatus {
    Created,
    Scanning,
    Scanned,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CommitmentStatus {
    Init,
    SrcSucceeded,
    Queued,
    Included,
    Spent,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DepositStatus {
    Init,
    AssetApproving,
    AssetApproved,
    SrcPending,
    SrcSucceeded,
    Queued,
    Included,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TransactionStatus {
    Init,
    ProofGenerating,
    ProofGenerated,
    Pending,
    Succeeded,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TransactionType {
    Transfer,
    Withdraw,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum NetworkType {
    Testnet,
    Mainnet,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum PackerChecksum {
    #[default]
    Sha512,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum PackerCompression {
    #[default]
    Zstd,
}

impl CircuitType {
    pub fn all() -> Vec<CircuitType> {
        vec![
            CircuitType::Rollup1,
            CircuitType::Rollup2,
            CircuitType::Rollup4,
            CircuitType::Rollup8,
            CircuitType::Rollup16,
            CircuitType::Transaction1x0,
            CircuitType::Transaction1x1,
            CircuitType::Transaction1x2,
            CircuitType::Transaction2x0,
            CircuitType::Transaction2x1,
            CircuitType::Transaction2x2,
        ]
    }
}

impl From<AccountStatus> for i32 {
    fn from(value: AccountStatus) -> Self {
        match value {
            AccountStatus::Created => 1,
            AccountStatus::Scanning => 2,
            AccountStatus::Scanned => 3,
        }
    }
}

impl From<i32> for AccountStatus {
    fn from(value: i32) -> Self {
        match value {
            1 => AccountStatus::Created,
            2 => AccountStatus::Scanning,
            3 => AccountStatus::Scanned,
            _ => AccountStatus::Created,
        }
    }
}

impl From<i32> for ContractType {
    fn from(value: i32) -> Self {
        match value {
            1 => ContractType::Deposit,
            2 => ContractType::Pool,
            _ => ContractType::Deposit,
        }
    }
}

impl From<&BridgeType> for i32 {
    fn from(value: &BridgeType) -> Self {
        match value {
            BridgeType::Loop => 1,
            BridgeType::Poly => 2,
            BridgeType::Tbridge => 3,
            BridgeType::Celer => 4,
            BridgeType::LayerZero => 5,
            BridgeType::Axelar => 6,
        }
    }
}

impl From<&AssetType> for i32 {
    fn from(value: &AssetType) -> Self {
        match value {
            AssetType::Erc20 => 1,
            AssetType::Main => 2,
        }
    }
}

impl From<&CircuitType> for i32 {
    fn from(value: &CircuitType) -> Self {
        match value {
            CircuitType::Rollup1 => 1,
            CircuitType::Rollup2 => 2,
            CircuitType::Rollup4 => 3,
            CircuitType::Rollup8 => 4,
            CircuitType::Rollup16 => 5,
            CircuitType::Transaction1x0 => 6,
            CircuitType::Transaction1x1 => 7,
            CircuitType::Transaction1x2 => 8,
            CircuitType::Transaction2x0 => 9,
            CircuitType::Transaction2x1 => 10,
            CircuitType::Transaction2x2 => 11,
        }
    }
}

impl From<&ContractType> for i32 {
    fn from(value: &ContractType) -> Self {
        match value {
            ContractType::Deposit => 1,
            ContractType::Pool => 2,
        }
    }
}

impl From<&ProviderType> for i32 {
    fn from(value: &ProviderType) -> Self {
        match value {
            ProviderType::Failover => 1,
            ProviderType::Quorum => 2,
        }
    }
}

impl From<&PackerCompression> for i32 {
    fn from(value: &PackerCompression) -> Self {
        match value {
            PackerCompression::Zstd => 1,
        }
    }
}

impl From<&PackerChecksum> for i32 {
    fn from(value: &PackerChecksum) -> Self {
        match value {
            PackerChecksum::Sha512 => 1,
        }
    }
}
