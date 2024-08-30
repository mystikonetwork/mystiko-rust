///`CertificateParams(address,address,uint256,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct CertificateParams {
    pub account: ::ethers::core::types::Address,
    pub asset: ::ethers::core::types::Address,
    pub deadline: ::ethers::core::types::U256,
    pub signature: ::ethers::core::types::Bytes,
}
///`Checkpoint208(uint48,uint208)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Checkpoint208 {
    pub key: u64,
    pub value: ::ethers::core::types::U256,
}
///`AuditorNote(uint64,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct AuditorNote {
    pub id: u64,
    pub public_key: ::ethers::core::types::U256,
    pub note: ::ethers::core::types::U256,
}
///`CommitmentRequest(uint256,uint256,uint256,uint256,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct CommitmentRequest {
    pub amount: ::ethers::core::types::U256,
    pub commitment: ::ethers::core::types::U256,
    pub executor_fee: ::ethers::core::types::U256,
    pub rollup_fee: ::ethers::core::types::U256,
    pub encrypted_note: ::ethers::core::types::Bytes,
}
///`RollupRequest(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct RollupRequest {
    pub proof: Proof,
    pub rollup_size: u32,
    pub new_root: ::ethers::core::types::U256,
    pub leaf_hash: ::ethers::core::types::U256,
}
///`TransactRequest(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TransactRequest {
    pub proof: Proof,
    pub root_hash: ::ethers::core::types::U256,
    pub serial_numbers: ::std::vec::Vec<::ethers::core::types::U256>,
    pub sig_hashes: ::std::vec::Vec<::ethers::core::types::U256>,
    pub sig_pk: [u8; 32],
    pub public_amount: ::ethers::core::types::U256,
    pub relayer_fee_amount: ::ethers::core::types::U256,
    pub out_commitments: ::std::vec::Vec<::ethers::core::types::U256>,
    pub out_rollup_fees: ::std::vec::Vec<::ethers::core::types::U256>,
    pub public_recipient: ::ethers::core::types::Address,
    pub relayer_address: ::ethers::core::types::Address,
    pub out_encrypted_notes: ::std::vec::Vec<::ethers::core::types::Bytes>,
    pub random_auditing_public_key: ::ethers::core::types::U256,
    pub encrypted_auditor_notes: ::std::vec::Vec<::ethers::core::types::U256>,
}
///`BridgeDepositRequest(uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct BridgeDepositRequest {
    pub amount: ::ethers::core::types::U256,
    pub commitment: ::ethers::core::types::U256,
    pub hash_k: ::ethers::core::types::U256,
    pub random_s: u128,
    pub encrypted_note: ::ethers::core::types::Bytes,
    pub bridge_fee: ::ethers::core::types::U256,
    pub executor_fee: ::ethers::core::types::U256,
    pub rollup_fee: ::ethers::core::types::U256,
}
///`BridgeLocalConfig(uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct BridgeLocalConfig {
    pub min_amount: ::ethers::core::types::U256,
    pub max_amount: ::ethers::core::types::U256,
    pub min_bridge_fee: ::ethers::core::types::U256,
}
///`BridgePeerConfig(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct BridgePeerConfig {
    pub peer_min_executor_fee: ::ethers::core::types::U256,
    pub peer_min_rollup_fee: ::ethers::core::types::U256,
}
///`BridgePeerContract(uint64,string,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct BridgePeerContract {
    pub peer_chain_id: u64,
    pub peer_chain_name: ::std::string::String,
    pub peer_contract: ::ethers::core::types::Address,
}
///`LoopDepositRequest(uint256,uint256,uint256,uint128,bytes,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct LoopDepositRequest {
    pub amount: ::ethers::core::types::U256,
    pub commitment: ::ethers::core::types::U256,
    pub hash_k: ::ethers::core::types::U256,
    pub random_s: u128,
    pub encrypted_note: ::ethers::core::types::Bytes,
    pub rollup_fee: ::ethers::core::types::U256,
}
///`LoopLocalConfig(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct LoopLocalConfig {
    pub min_amount: ::ethers::core::types::U256,
    pub max_amount: ::ethers::core::types::U256,
}
///`G1Point(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct G1Point {
    pub x: ::ethers::core::types::U256,
    pub y: ::ethers::core::types::U256,
}
///`G2Point(uint256[2],uint256[2])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct G2Point {
    pub x: [::ethers::core::types::U256; 2],
    pub y: [::ethers::core::types::U256; 2],
}
///`Proof((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Proof {
    pub a: G1Point,
    pub b: G2Point,
    pub c: G1Point,
}
///`RelayerValidateParams(address,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct RelayerValidateParams {
    pub pool: ::ethers::core::types::Address,
    pub relayer: ::ethers::core::types::Address,
}
///`RollerValidateParams(address,address,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct RollerValidateParams {
    pub pool: ::ethers::core::types::Address,
    pub roller: ::ethers::core::types::Address,
    pub rollup_size: ::ethers::core::types::U256,
    pub queue_count: ::ethers::core::types::U256,
    pub included_count: ::ethers::core::types::U256,
}
///`WrappedVerifier(address,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct WrappedVerifier {
    pub verifier: ::ethers::core::types::Address,
    pub enabled: bool,
}
