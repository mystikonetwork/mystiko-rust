// @generated
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractConfig {
    #[prost(uint32, tag="1")]
    pub version: u32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub start_block: u64,
    #[prost(bool, tag="5")]
    pub disabled: bool,
    #[prost(string, tag="6")]
    pub min_rollup_fee: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub asset_config: ::core::option::Option<super::super::v1::AssetConfig>,
    #[prost(enumeration="super::super::super::common::v1::BridgeType", tag="8")]
    pub bridge_type: i32,
    #[prost(enumeration="super::super::super::common::v1::ContractType", tag="9")]
    pub contract_type: i32,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolContractConfig {
    #[prost(uint32, tag="1")]
    pub version: u32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub start_block: u64,
    #[prost(string, tag="5")]
    pub pool_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub min_rollup_fee: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::super::common::v1::ContractType", tag="7")]
    pub contract_type: i32,
    #[prost(enumeration="super::super::super::common::v1::BridgeType", tag="8")]
    pub bridge_type: i32,
    #[prost(message, optional, tag="9")]
    pub asset_config: ::core::option::Option<super::super::v1::AssetConfig>,
    #[prost(message, repeated, tag="10")]
    pub circuit_configs: ::prost::alloc::vec::Vec<super::super::v1::CircuitConfig>,
    #[prost(string, repeated, tag="11")]
    pub circuits: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositContractConfig {
    #[prost(uint32, tag="1")]
    pub version: u32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub start_block: u64,
    #[prost(bool, tag="5")]
    pub disabled: bool,
    #[prost(string, tag="6")]
    pub min_amount: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub max_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag="8")]
    pub pool_contract_config: ::core::option::Option<PoolContractConfig>,
    #[prost(enumeration="super::super::super::common::v1::BridgeType", tag="9")]
    pub bridge_type: i32,
    #[prost(enumeration="super::super::super::common::v1::ContractType", tag="10")]
    pub contract_type: i32,
    #[prost(string, optional, tag="11")]
    pub min_bridge_fee: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub min_executor_fee: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="13")]
    pub service_fee: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="14")]
    pub service_fee_divider: ::core::option::Option<u32>,
    #[prost(message, optional, tag="15")]
    pub bridge_fee_asset_config: ::core::option::Option<super::super::v1::AssetConfig>,
    #[prost(message, optional, tag="16")]
    pub executor_fee_asset_config: ::core::option::Option<super::super::v1::AssetConfig>,
    #[prost(uint64, optional, tag="17")]
    pub peer_chain_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="18")]
    pub peer_contract_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Encoded file descriptor set for the `mystiko.config.contract.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x8d, 0x0a, 0x0a, 0x29, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2f, 0x76, 0x31, 0x2f,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1a,
    0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2e, 0x76, 0x31, 0x1a, 0x1d, 0x6d, 0x79, 0x73, 0x74,
    0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x6d, 0x79, 0x73, 0x74, 0x69,
    0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x76, 0x31, 0x2f, 0x61, 0x73, 0x73,
    0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x84, 0x03, 0x0a, 0x0e, 0x43, 0x6f, 0x6e,
    0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x18, 0x0a, 0x07, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x42,
    0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64,
    0x12, 0x24, 0x0a, 0x0e, 0x6d, 0x69, 0x6e, 0x5f, 0x72, 0x6f, 0x6c, 0x6c, 0x75, 0x70, 0x5f, 0x66,
    0x65, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x6d, 0x69, 0x6e, 0x52, 0x6f, 0x6c,
    0x6c, 0x75, 0x70, 0x46, 0x65, 0x65, 0x12, 0x41, 0x0a, 0x0c, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6d,
    0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x76, 0x31,
    0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x0b, 0x61, 0x73,
    0x73, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x3e, 0x0a, 0x0b, 0x62, 0x72, 0x69,
    0x64, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d,
    0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
    0x76, 0x31, 0x2e, 0x42, 0x72, 0x69, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0a, 0x62,
    0x72, 0x69, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x44, 0x0a, 0x0d, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x61, 0x63, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x1f, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f,
    0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x54, 0x79, 0x70,
    0x65, 0x52, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x42,
    0xba, 0x01, 0x0a, 0x1e, 0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2e,
    0x76, 0x31, 0x42, 0x0d, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43, 0x43, 0xaa, 0x02, 0x1a, 0x4d, 0x79, 0x73, 0x74,
    0x69, 0x6b, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x61, 0x63, 0x74, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x1a, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f,
    0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5c, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74,
    0x5c, 0x56, 0x31, 0xe2, 0x02, 0x26, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x5c, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x5c, 0x56, 0x31,
    0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1d, 0x4d,
    0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3a, 0x3a,
    0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xb9, 0x04, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x11, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x03, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x05, 0x00,
    0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x08, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x08, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08,
    0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x02, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x09, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x0a, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0a,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x09, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x0b, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x0b, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0c,
    0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x06,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0c, 0x09, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x0d, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x0d, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0e, 0x02, 0x33,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x0e, 0x02, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0e, 0x21, 0x2d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0e, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x0f, 0x02, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06,
    0x12, 0x03, 0x0f, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x0f, 0x21, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x0f, 0x31,
    0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x10, 0x02, 0x33, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x10, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x10, 0x21, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x10, 0x31, 0x32, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
    0x0a, 0xa7, 0x0c, 0x0a, 0x25, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2f, 0x76, 0x31, 0x2f,
    0x70, 0x6f, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1a, 0x6d, 0x79, 0x73, 0x74,
    0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x61, 0x63, 0x74, 0x2e, 0x76, 0x31, 0x1a, 0x1d, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f,
    0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x76, 0x31, 0x2f, 0x61, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xf0, 0x03, 0x0a, 0x12, 0x50, 0x6f, 0x6f, 0x6c, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x18, 0x0a, 0x07,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x62, 0x6c,
    0x6f, 0x63, 0x6b, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x1b, 0x0a, 0x09, 0x70, 0x6f, 0x6f, 0x6c, 0x5f, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x70, 0x6f, 0x6f, 0x6c, 0x4e, 0x61,
    0x6d, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x6d, 0x69, 0x6e, 0x5f, 0x72, 0x6f, 0x6c, 0x6c, 0x75, 0x70,
    0x5f, 0x66, 0x65, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x6d, 0x69, 0x6e, 0x52,
    0x6f, 0x6c, 0x6c, 0x75, 0x70, 0x46, 0x65, 0x65, 0x12, 0x44, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74,
    0x72, 0x61, 0x63, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1f, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e,
    0x2e, 0x76, 0x31, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65,
    0x52, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x3e,
    0x0a, 0x0b, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f,
    0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x42, 0x72, 0x69, 0x64, 0x67, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x52, 0x0a, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x41,
    0x0a, 0x0c, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x52, 0x0b, 0x61, 0x73, 0x73, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x12, 0x49, 0x0a, 0x0f, 0x63, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x5f, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x73, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x6d, 0x79, 0x73,
    0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x43,
    0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x0e, 0x63, 0x69,
    0x72, 0x63, 0x75, 0x69, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x73, 0x12, 0x1a, 0x0a, 0x08,
    0x63, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x73, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x09, 0x52, 0x08,
    0x63, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x73, 0x42, 0xb6, 0x01, 0x0a, 0x1e, 0x63, 0x6f, 0x6d,
    0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2e, 0x76, 0x31, 0x42, 0x09, 0x50, 0x6f, 0x6f,
    0x6c, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43, 0x43, 0xaa, 0x02,
    0x1a, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x1a, 0x4d, 0x79,
    0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5c, 0x43, 0x6f, 0x6e,
    0x74, 0x72, 0x61, 0x63, 0x74, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x26, 0x4d, 0x79, 0x73, 0x74, 0x69,
    0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5c, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61,
    0x63, 0x74, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0xea, 0x02, 0x1d, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x3a, 0x3a, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x3a, 0x3a, 0x56,
    0x31, 0x4a, 0xce, 0x05, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x14, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x23,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x05, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x00,
    0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x09, 0x02, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09,
    0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x3d, 0x3e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x3f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x0a, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x0b, 0x02, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x09, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x3d, 0x3e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x0c, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x0c, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d,
    0x02, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x0e, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x0e, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x02, 0x3f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x0f, 0x02, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0f, 0x21, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0f, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x10, 0x02, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06,
    0x12, 0x03, 0x10, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x10, 0x1f, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x10, 0x3d,
    0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x11, 0x02, 0x3f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x11, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x11, 0x20, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x11, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09,
    0x12, 0x03, 0x12, 0x02, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03,
    0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x06, 0x12, 0x03, 0x12, 0x0b,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x12, 0x2b, 0x3a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x12, 0x3d, 0x3f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x13, 0x02, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0a, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a,
    0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12,
    0x03, 0x13, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x13,
    0x3d, 0x3f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xe3, 0x14, 0x0a, 0x28, 0x6d,
    0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2f, 0x76, 0x31, 0x2f, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1a, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f,
    0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74,
    0x2e, 0x76, 0x31, 0x1a, 0x1d, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x1d, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2f, 0x76, 0x31, 0x2f, 0x61, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x25, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2f, 0x76, 0x31, 0x2f, 0x70, 0x6f,
    0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xcb, 0x08, 0x0a, 0x15, 0x44, 0x65, 0x70,
    0x6f, 0x73, 0x69, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x1a, 0x0a, 0x08, 0x64,
    0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x64,
    0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x69, 0x6e, 0x5f, 0x61,
    0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6d, 0x69, 0x6e,
    0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x61, 0x78, 0x5f, 0x61, 0x6d,
    0x6f, 0x75, 0x6e, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6d, 0x61, 0x78, 0x41,
    0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x60, 0x0a, 0x14, 0x70, 0x6f, 0x6f, 0x6c, 0x5f, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2e, 0x76, 0x31,
    0x2e, 0x50, 0x6f, 0x6f, 0x6c, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x52, 0x12, 0x70, 0x6f, 0x6f, 0x6c, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63,
    0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x3e, 0x0a, 0x0b, 0x62, 0x72, 0x69, 0x64, 0x67,
    0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x6d,
    0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31,
    0x2e, 0x42, 0x72, 0x69, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0a, 0x62, 0x72, 0x69,
    0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x44, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x61, 0x63, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1f,
    0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
    0x76, 0x31, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52,
    0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x29, 0x0a,
    0x0e, 0x6d, 0x69, 0x6e, 0x5f, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x5f, 0x66, 0x65, 0x65, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0c, 0x6d, 0x69, 0x6e, 0x42, 0x72, 0x69, 0x64,
    0x67, 0x65, 0x46, 0x65, 0x65, 0x88, 0x01, 0x01, 0x12, 0x2d, 0x0a, 0x10, 0x6d, 0x69, 0x6e, 0x5f,
    0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x66, 0x65, 0x65, 0x18, 0x0c, 0x20, 0x01,
    0x28, 0x09, 0x48, 0x01, 0x52, 0x0e, 0x6d, 0x69, 0x6e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f,
    0x72, 0x46, 0x65, 0x65, 0x88, 0x01, 0x01, 0x12, 0x24, 0x0a, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x5f, 0x66, 0x65, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x02, 0x52, 0x0a,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x46, 0x65, 0x65, 0x88, 0x01, 0x01, 0x12, 0x33, 0x0a,
    0x13, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x66, 0x65, 0x65, 0x5f, 0x64, 0x69, 0x76,
    0x69, 0x64, 0x65, 0x72, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x03, 0x52, 0x11, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x46, 0x65, 0x65, 0x44, 0x69, 0x76, 0x69, 0x64, 0x65, 0x72, 0x88,
    0x01, 0x01, 0x12, 0x5a, 0x0a, 0x17, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x5f, 0x66, 0x65, 0x65,
    0x5f, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x0f, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x48, 0x04, 0x52, 0x14, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x46, 0x65, 0x65,
    0x41, 0x73, 0x73, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x88, 0x01, 0x01, 0x12, 0x5e,
    0x0a, 0x19, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x66, 0x65, 0x65, 0x5f, 0x61,
    0x73, 0x73, 0x65, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x10, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1e, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x48, 0x05, 0x52, 0x16, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x46, 0x65, 0x65,
    0x41, 0x73, 0x73, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x88, 0x01, 0x01, 0x12, 0x27,
    0x0a, 0x0d, 0x70, 0x65, 0x65, 0x72, 0x5f, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18,
    0x11, 0x20, 0x01, 0x28, 0x04, 0x48, 0x06, 0x52, 0x0b, 0x70, 0x65, 0x65, 0x72, 0x43, 0x68, 0x61,
    0x69, 0x6e, 0x49, 0x64, 0x88, 0x01, 0x01, 0x12, 0x37, 0x0a, 0x15, 0x70, 0x65, 0x65, 0x72, 0x5f,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x18, 0x12, 0x20, 0x01, 0x28, 0x09, 0x48, 0x07, 0x52, 0x13, 0x70, 0x65, 0x65, 0x72, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x88, 0x01, 0x01,
    0x42, 0x11, 0x0a, 0x0f, 0x5f, 0x6d, 0x69, 0x6e, 0x5f, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x5f,
    0x66, 0x65, 0x65, 0x42, 0x13, 0x0a, 0x11, 0x5f, 0x6d, 0x69, 0x6e, 0x5f, 0x65, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x6f, 0x72, 0x5f, 0x66, 0x65, 0x65, 0x42, 0x0e, 0x0a, 0x0c, 0x5f, 0x73, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x5f, 0x66, 0x65, 0x65, 0x42, 0x16, 0x0a, 0x14, 0x5f, 0x73, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x5f, 0x66, 0x65, 0x65, 0x5f, 0x64, 0x69, 0x76, 0x69, 0x64, 0x65, 0x72,
    0x42, 0x1a, 0x0a, 0x18, 0x5f, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x5f, 0x66, 0x65, 0x65, 0x5f,
    0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0x1c, 0x0a, 0x1a,
    0x5f, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x66, 0x65, 0x65, 0x5f, 0x61, 0x73,
    0x73, 0x65, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0x10, 0x0a, 0x0e, 0x5f, 0x70,
    0x65, 0x65, 0x72, 0x5f, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x42, 0x18, 0x0a, 0x16,
    0x5f, 0x70, 0x65, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x5f, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x42, 0xb9, 0x01, 0x0a, 0x1e, 0x63, 0x6f, 0x6d, 0x2e, 0x6d,
    0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2e, 0x76, 0x31, 0x42, 0x0c, 0x44, 0x65, 0x70, 0x6f, 0x73,
    0x69, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43, 0x43, 0xaa,
    0x02, 0x1a, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x1a, 0x4d,
    0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5c, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x26, 0x4d, 0x79, 0x73, 0x74,
    0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5c, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x61, 0x63, 0x74, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61,
    0x74, 0x61, 0xea, 0x02, 0x1d, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x3a, 0x3a, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x3a, 0x3a,
    0x56, 0x31, 0x4a, 0xa3, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1b, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00,
    0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06,
    0x00, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x1b, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x09, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x45,
    0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x47, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x45, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x0b, 0x02, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x0b,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x45, 0x46, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x47, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x0c, 0x45, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03,
    0x0d, 0x02, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0d, 0x02,
    0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x0b, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x45, 0x46, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x0e, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x0e, 0x45, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x02,
    0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0f, 0x0b, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0f, 0x45, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x07, 0x12, 0x03, 0x10, 0x02, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x06, 0x12, 0x03, 0x10, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x10, 0x15, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x10,
    0x45, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x11, 0x02, 0x47, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x11, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x11, 0x1f, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x11, 0x45, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x09, 0x12, 0x03, 0x12, 0x02, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x06, 0x12,
    0x03, 0x12, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x12,
    0x21, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x12, 0x45, 0x47,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x13, 0x02, 0x48, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x01, 0x12, 0x03, 0x13, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03,
    0x12, 0x03, 0x13, 0x45, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x14,
    0x02, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x14, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x14, 0x45, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0c, 0x12, 0x03, 0x15, 0x02, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04,
    0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03,
    0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x15, 0x12,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x15, 0x45, 0x47, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x16, 0x02, 0x48, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0d, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d,
    0x01, 0x12, 0x03, 0x16, 0x12, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12,
    0x03, 0x16, 0x45, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x17, 0x02,
    0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x06, 0x12, 0x03, 0x17, 0x0b, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x17, 0x29, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x17, 0x45, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0f, 0x12, 0x03, 0x18, 0x02, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x04, 0x12,
    0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x18,
    0x0b, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x18, 0x29, 0x42,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x18, 0x45, 0x47, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x10, 0x12, 0x03, 0x19, 0x02, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x10, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x10, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x01,
    0x12, 0x03, 0x19, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x03, 0x12, 0x03,
    0x19, 0x45, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x11, 0x12, 0x03, 0x1a, 0x02, 0x48,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x11, 0x03, 0x12, 0x03, 0x1a, 0x45, 0x47, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("mystiko.config.contract.v1.serde.rs");
// @@protoc_insertion_point(module)