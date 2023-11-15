// @generated
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(string, tag="1")]
    pub asset_symbol: ::prost::alloc::string::String,
    #[prost(double, tag="2")]
    pub unspent: f64,
    #[prost(double, tag="3")]
    pub pending: f64,
    #[prost(double, optional, tag="4")]
    pub spent: ::core::option::Option<f64>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceOptions {
    #[prost(bool, optional, tag="1")]
    pub with_spent: ::core::option::Option<bool>,
    #[prost(string, repeated, tag="2")]
    pub shielded_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, repeated, tag="3")]
    pub chain_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, repeated, tag="4")]
    pub asset_symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::super::super::common::v1::BridgeType", repeated, tag="6")]
    pub bridge_types: ::prost::alloc::vec::Vec<i32>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceResult {
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsOptions {
    #[prost(string, repeated, tag="1")]
    pub shielded_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BridgeAssets {
    #[prost(enumeration="super::super::super::common::v1::BridgeType", tag="1")]
    pub bridge_type: i32,
    #[prost(message, repeated, tag="2")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainAssetsResult {
    #[prost(uint64, tag="1")]
    pub chain_id: u64,
    #[prost(message, repeated, tag="2")]
    pub bridges: ::prost::alloc::vec::Vec<BridgeAssets>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetOptions {
    #[prost(string, optional, tag="1")]
    pub reset_to_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub shielded_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetResult {
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanOptions {
    #[prost(string, tag="1")]
    pub wallet_password: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="2")]
    pub batch_size: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub concurrency: ::core::option::Option<u32>,
    #[prost(string, repeated, tag="4")]
    pub shielded_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanResult {
    #[prost(uint64, tag="1")]
    pub total_count: u64,
    #[prost(uint64, tag="2")]
    pub owned_count: u64,
    #[prost(string, repeated, tag="3")]
    pub scanned_shielded_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub to_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Encoded file descriptor set for the `mystiko.core.scanner.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd2, 0x0c, 0x0a, 0x25, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x72,
    0x65, 0x2f, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x62, 0x61, 0x6c,
    0x61, 0x6e, 0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x6d, 0x79, 0x73, 0x74,
    0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72,
    0x2e, 0x76, 0x31, 0x1a, 0x1d, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x85, 0x01, 0x0a, 0x07, 0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x12, 0x21,
    0x0a, 0x0c, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x61, 0x73, 0x73, 0x65, 0x74, 0x53, 0x79, 0x6d, 0x62, 0x6f,
    0x6c, 0x12, 0x18, 0x0a, 0x07, 0x75, 0x6e, 0x73, 0x70, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x07, 0x75, 0x6e, 0x73, 0x70, 0x65, 0x6e, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x70,
    0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x07, 0x70, 0x65,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x12, 0x19, 0x0a, 0x05, 0x73, 0x70, 0x65, 0x6e, 0x74, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x01, 0x48, 0x00, 0x52, 0x05, 0x73, 0x70, 0x65, 0x6e, 0x74, 0x88, 0x01, 0x01,
    0x42, 0x08, 0x0a, 0x06, 0x5f, 0x73, 0x70, 0x65, 0x6e, 0x74, 0x22, 0xa5, 0x02, 0x0a, 0x0e, 0x42,
    0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x22, 0x0a,
    0x0a, 0x77, 0x69, 0x74, 0x68, 0x5f, 0x73, 0x70, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x08, 0x48, 0x00, 0x52, 0x09, 0x77, 0x69, 0x74, 0x68, 0x53, 0x70, 0x65, 0x6e, 0x74, 0x88, 0x01,
    0x01, 0x12, 0x2d, 0x0a, 0x12, 0x73, 0x68, 0x69, 0x65, 0x6c, 0x64, 0x65, 0x64, 0x5f, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x11, 0x73,
    0x68, 0x69, 0x65, 0x6c, 0x64, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73,
    0x12, 0x1b, 0x0a, 0x09, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x04, 0x52, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x49, 0x64, 0x73, 0x12, 0x23, 0x0a,
    0x0d, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x73, 0x18, 0x04,
    0x20, 0x03, 0x28, 0x09, 0x52, 0x0c, 0x61, 0x73, 0x73, 0x65, 0x74, 0x53, 0x79, 0x6d, 0x62, 0x6f,
    0x6c, 0x73, 0x12, 0x2d, 0x0a, 0x12, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x5f, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x09, 0x52, 0x11,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65,
    0x73, 0x12, 0x40, 0x0a, 0x0c, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b,
    0x6f, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x42, 0x72, 0x69, 0x64,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0b, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x73, 0x42, 0x0d, 0x0a, 0x0b, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x5f, 0x73, 0x70, 0x65,
    0x6e, 0x74, 0x22, 0x4d, 0x0a, 0x0d, 0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x12, 0x3c, 0x0a, 0x08, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x73, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e,
    0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e,
    0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x52, 0x08, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65,
    0x73, 0x42, 0xaa, 0x01, 0x0a, 0x1b, 0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b,
    0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x76,
    0x31, 0x42, 0x0c, 0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50,
    0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43, 0x53, 0xaa, 0x02, 0x17, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b,
    0x6f, 0x2e, 0x43, 0x6f, 0x72, 0x65, 0x2e, 0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x56,
    0x31, 0xca, 0x02, 0x17, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x72, 0x65,
    0x5c, 0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x23, 0x4d, 0x79,
    0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x72, 0x65, 0x5c, 0x53, 0x63, 0x61, 0x6e, 0x6e,
    0x65, 0x72, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0xea, 0x02, 0x1a, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x72,
    0x65, 0x3a, 0x3a, 0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xbc,
    0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x18, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x06, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x02, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x07, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x08, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x08,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x0b, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x09, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x09, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0a,
    0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0a, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x0d, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0d,
    0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x39, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0e, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x0f, 0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0f,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x10, 0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x10, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x37,
    0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x11, 0x02, 0x39, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x11, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x11, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03,
    0x12, 0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x12, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x12, 0x12, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x12, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x05, 0x12, 0x03, 0x13, 0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x13, 0x0b, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x13,
    0x28, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x13, 0x37, 0x38,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x16, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x16, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x17, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x17, 0x0b,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x13, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x1e, 0x1f, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xb1, 0x08, 0x0a, 0x23, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b,
    0x6f, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2f, 0x76,
    0x31, 0x2f, 0x61, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x6d,
    0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x63, 0x61, 0x6e,
    0x6e, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x1a, 0x1d, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f,
    0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x25, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63,
    0x6f, 0x72, 0x65, 0x2f, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x62,
    0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3e, 0x0a, 0x0d,
    0x41, 0x73, 0x73, 0x65, 0x74, 0x73, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x2d, 0x0a,
    0x12, 0x73, 0x68, 0x69, 0x65, 0x6c, 0x64, 0x65, 0x64, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x11, 0x73, 0x68, 0x69, 0x65, 0x6c,
    0x64, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x22, 0x8c, 0x01, 0x0a,
    0x0c, 0x42, 0x72, 0x69, 0x64, 0x67, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x73, 0x12, 0x3e, 0x0a,
    0x0b, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x42, 0x72, 0x69, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70,
    0x65, 0x52, 0x0a, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x3c, 0x0a,
    0x08, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x20, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73,
    0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63,
    0x65, 0x52, 0x08, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x73, 0x22, 0x6f, 0x0a, 0x11, 0x43,
    0x68, 0x61, 0x69, 0x6e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x73, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x12, 0x19, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x07, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x49, 0x64, 0x12, 0x3f, 0x0a, 0x07, 0x62,
    0x72, 0x69, 0x64, 0x67, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x6d,
    0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x63, 0x61, 0x6e,
    0x6e, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x42, 0x72, 0x69, 0x64, 0x67, 0x65, 0x41, 0x73, 0x73,
    0x65, 0x74, 0x73, 0x52, 0x07, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x73, 0x42, 0xa8, 0x01, 0x0a,
    0x1b, 0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72,
    0x65, 0x2e, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x42, 0x0a, 0x41, 0x73,
    0x73, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43, 0x53,
    0xaa, 0x02, 0x17, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x43, 0x6f, 0x72, 0x65, 0x2e,
    0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x17, 0x4d, 0x79, 0x73,
    0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x72, 0x65, 0x5c, 0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65,
    0x72, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x23, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43,
    0x6f, 0x72, 0x65, 0x5c, 0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5c, 0x56, 0x31, 0x5c, 0x47,
    0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1a, 0x4d, 0x79, 0x73,
    0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x72, 0x65, 0x3a, 0x3a, 0x53, 0x63, 0x61, 0x6e,
    0x6e, 0x65, 0x72, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xb7, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x13, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00,
    0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2f, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x07, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02,
    0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x37, 0x38, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x0b, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x39, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0c, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x1f, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x0d, 0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0d, 0x0b,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x13, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x37, 0x38, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x10, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x10, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x11,
    0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x09, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x12, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x12, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x37,
    0x38, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xcc, 0x04, 0x0a, 0x23, 0x6d, 0x79,
    0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x73, 0x63, 0x61, 0x6e, 0x6e,
    0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x72, 0x65, 0x73, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x17, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e,
    0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x22, 0x72, 0x0a, 0x0c, 0x52, 0x65,
    0x73, 0x65, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x23, 0x0a, 0x0b, 0x72, 0x65,
    0x73, 0x65, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48,
    0x00, 0x52, 0x09, 0x72, 0x65, 0x73, 0x65, 0x74, 0x54, 0x6f, 0x49, 0x64, 0x88, 0x01, 0x01, 0x12,
    0x2d, 0x0a, 0x12, 0x73, 0x68, 0x69, 0x65, 0x6c, 0x64, 0x65, 0x64, 0x5f, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x11, 0x73, 0x68, 0x69,
    0x65, 0x6c, 0x64, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x42, 0x0e,
    0x0a, 0x0c, 0x5f, 0x72, 0x65, 0x73, 0x65, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x69, 0x64, 0x22, 0x0d,
    0x0a, 0x0b, 0x52, 0x65, 0x73, 0x65, 0x74, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x42, 0xa8, 0x01,
    0x0a, 0x1b, 0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f,
    0x72, 0x65, 0x2e, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x42, 0x0a, 0x52,
    0x65, 0x73, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43,
    0x53, 0xaa, 0x02, 0x17, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x43, 0x6f, 0x72, 0x65,
    0x2e, 0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x17, 0x4d, 0x79,
    0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x72, 0x65, 0x5c, 0x53, 0x63, 0x61, 0x6e, 0x6e,
    0x65, 0x72, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x23, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c,
    0x43, 0x6f, 0x72, 0x65, 0x5c, 0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5c, 0x56, 0x31, 0x5c,
    0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1a, 0x4d, 0x79,
    0x73, 0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x72, 0x65, 0x3a, 0x3a, 0x53, 0x63, 0x61,
    0x6e, 0x6e, 0x65, 0x72, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xd5, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x09, 0x16, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04,
    0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x05, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x05, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x27, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x09, 0x00, 0x16, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x13,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xc1, 0x09, 0x0a, 0x22, 0x6d, 0x79, 0x73,
    0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65,
    0x72, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x63, 0x61, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x17, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x63,
    0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x22, 0xcf, 0x01, 0x0a, 0x0b, 0x53, 0x63, 0x61,
    0x6e, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x27, 0x0a, 0x0f, 0x77, 0x61, 0x6c, 0x6c,
    0x65, 0x74, 0x5f, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0e, 0x77, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x50, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72,
    0x64, 0x12, 0x22, 0x0a, 0x0a, 0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x09, 0x62, 0x61, 0x74, 0x63, 0x68, 0x53, 0x69,
    0x7a, 0x65, 0x88, 0x01, 0x01, 0x12, 0x25, 0x0a, 0x0b, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x01, 0x52, 0x0b, 0x63, 0x6f,
    0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x88, 0x01, 0x01, 0x12, 0x2d, 0x0a, 0x12,
    0x73, 0x68, 0x69, 0x65, 0x6c, 0x64, 0x65, 0x64, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x52, 0x11, 0x73, 0x68, 0x69, 0x65, 0x6c, 0x64,
    0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x42, 0x0d, 0x0a, 0x0b, 0x5f,
    0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x42, 0x0e, 0x0a, 0x0c, 0x5f, 0x63,
    0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x22, 0xb0, 0x01, 0x0a, 0x0a, 0x53,
    0x63, 0x61, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x1f, 0x0a, 0x0b, 0x74, 0x6f, 0x74,
    0x61, 0x6c, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a,
    0x74, 0x6f, 0x74, 0x61, 0x6c, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x1f, 0x0a, 0x0b, 0x6f, 0x77,
    0x6e, 0x65, 0x64, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0a, 0x6f, 0x77, 0x6e, 0x65, 0x64, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x3c, 0x0a, 0x1a, 0x73,
    0x63, 0x61, 0x6e, 0x6e, 0x65, 0x64, 0x5f, 0x73, 0x68, 0x69, 0x65, 0x6c, 0x64, 0x65, 0x64, 0x5f,
    0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x52,
    0x18, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x64, 0x53, 0x68, 0x69, 0x65, 0x6c, 0x64, 0x65, 0x64,
    0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x12, 0x18, 0x0a, 0x05, 0x74, 0x6f, 0x5f,
    0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x04, 0x74, 0x6f, 0x49, 0x64,
    0x88, 0x01, 0x01, 0x42, 0x08, 0x0a, 0x06, 0x5f, 0x74, 0x6f, 0x5f, 0x69, 0x64, 0x42, 0xa7, 0x01,
    0x0a, 0x1b, 0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f,
    0x72, 0x65, 0x2e, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x42, 0x09, 0x53,
    0x63, 0x61, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43, 0x53,
    0xaa, 0x02, 0x17, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x43, 0x6f, 0x72, 0x65, 0x2e,
    0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x17, 0x4d, 0x79, 0x73,
    0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x72, 0x65, 0x5c, 0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65,
    0x72, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x23, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43,
    0x6f, 0x72, 0x65, 0x5c, 0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5c, 0x56, 0x31, 0x5c, 0x47,
    0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1a, 0x4d, 0x79, 0x73,
    0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x72, 0x65, 0x3a, 0x3a, 0x53, 0x63, 0x61, 0x6e,
    0x6e, 0x65, 0x72, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xca, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x10, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00,
    0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x05, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x05, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06,
    0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x07, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x12,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07, 0x27, 0x28, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x08, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x08, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x08, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x08, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0b, 0x00, 0x10, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0c, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0c, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x0e, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x12,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x2f, 0x30, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x0f, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x0f, 0x2f, 0x30, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("mystiko.core.scanner.v1.serde.rs");
// @@protoc_insertion_point(module)