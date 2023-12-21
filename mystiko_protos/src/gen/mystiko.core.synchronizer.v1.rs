// @generated
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncOptions {
    #[prost(bool, optional, tag="1")]
    pub disable_datapacker_fetcher: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub enable_datapacker_fetcher_validate: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub disable_sequencer_fetcher: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub enable_sequencer_fetcher_validate: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="5")]
    pub disable_provider_fetcher: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="6")]
    pub disable_provider_fetcher_validate: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="7")]
    pub disable_rule_validator: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="8")]
    pub disable_rule_validator_integrity_check: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="9")]
    pub disable_rule_validator_sequence_check: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="10")]
    pub disable_rule_validator_counter_check: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="11")]
    pub disable_rule_validator_tree_check: ::core::option::Option<bool>,
    #[prost(uint64, optional, tag="12")]
    pub fetcher_fetch_timeout_ms: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="13")]
    pub fetcher_query_loaded_block_timeout_ms: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="14")]
    pub validator_validate_concurrency: ::core::option::Option<u64>,
    #[prost(uint64, repeated, tag="15")]
    pub chain_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetChainOptions {
    #[prost(uint64, tag="1")]
    pub chain_id: u64,
    #[prost(string, repeated, tag="2")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub block_number: ::core::option::Option<u64>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetOptions {
    #[prost(message, repeated, tag="1")]
    pub chains: ::prost::alloc::vec::Vec<ResetChainOptions>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractStatus {
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub synced_block: u64,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainStatus {
    #[prost(uint64, tag="1")]
    pub chain_id: u64,
    #[prost(uint64, tag="2")]
    pub synced_block: u64,
    #[prost(uint64, tag="3")]
    pub target_block: u64,
    #[prost(message, repeated, tag="4")]
    pub contracts: ::prost::alloc::vec::Vec<ContractStatus>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynchronizerStatus {
    #[prost(message, repeated, tag="1")]
    pub chains: ::prost::alloc::vec::Vec<ChainStatus>,
}
/// Encoded file descriptor set for the `mystiko.core.synchronizer.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x9a, 0x17, 0x0a, 0x27, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x72,
    0x65, 0x2f, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2f, 0x76,
    0x31, 0x2f, 0x73, 0x79, 0x6e, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1c, 0x6d, 0x79,
    0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x79, 0x6e, 0x63, 0x68,
    0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x22, 0xc3, 0x0c, 0x0a, 0x0b, 0x53,
    0x79, 0x6e, 0x63, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x41, 0x0a, 0x1a, 0x64, 0x69,
    0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x72,
    0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00,
    0x52, 0x18, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x61, 0x74, 0x61, 0x70, 0x61, 0x63,
    0x6b, 0x65, 0x72, 0x46, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x88, 0x01, 0x01, 0x12, 0x50, 0x0a,
    0x22, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x70, 0x61, 0x63, 0x6b,
    0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x48, 0x01, 0x52, 0x1f, 0x65, 0x6e, 0x61,
    0x62, 0x6c, 0x65, 0x44, 0x61, 0x74, 0x61, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x46, 0x65, 0x74,
    0x63, 0x68, 0x65, 0x72, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x88, 0x01, 0x01, 0x12,
    0x3f, 0x0a, 0x19, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x73, 0x65, 0x71, 0x75, 0x65,
    0x6e, 0x63, 0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x02, 0x52, 0x17, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x65, 0x71,
    0x75, 0x65, 0x6e, 0x63, 0x65, 0x72, 0x46, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x88, 0x01, 0x01,
    0x12, 0x4e, 0x0a, 0x21, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x73, 0x65, 0x71, 0x75, 0x65,
    0x6e, 0x63, 0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x5f, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x48, 0x03, 0x52, 0x1e, 0x65,
    0x6e, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x72, 0x46, 0x65,
    0x74, 0x63, 0x68, 0x65, 0x72, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x88, 0x01, 0x01,
    0x12, 0x3d, 0x0a, 0x18, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x70, 0x72, 0x6f, 0x76,
    0x69, 0x64, 0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x04, 0x52, 0x16, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x50, 0x72, 0x6f,
    0x76, 0x69, 0x64, 0x65, 0x72, 0x46, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x88, 0x01, 0x01, 0x12,
    0x4e, 0x0a, 0x21, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x70, 0x72, 0x6f, 0x76, 0x69,
    0x64, 0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x5f, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x48, 0x05, 0x52, 0x1e, 0x64, 0x69,
    0x73, 0x61, 0x62, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x46, 0x65, 0x74,
    0x63, 0x68, 0x65, 0x72, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x88, 0x01, 0x01, 0x12,
    0x39, 0x0a, 0x16, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x5f,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x48,
    0x06, 0x52, 0x14, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x75, 0x6c, 0x65, 0x56, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x88, 0x01, 0x01, 0x12, 0x57, 0x0a, 0x26, 0x64, 0x69,
    0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x67, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x63,
    0x68, 0x65, 0x63, 0x6b, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x48, 0x07, 0x52, 0x22, 0x64, 0x69,
    0x73, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x75, 0x6c, 0x65, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x6f, 0x72, 0x49, 0x6e, 0x74, 0x65, 0x67, 0x72, 0x69, 0x74, 0x79, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x88, 0x01, 0x01, 0x12, 0x55, 0x0a, 0x25, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x72,
    0x75, 0x6c, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x73, 0x65,
    0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x08, 0x52, 0x21, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x75, 0x6c,
    0x65, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e,
    0x63, 0x65, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x88, 0x01, 0x01, 0x12, 0x53, 0x0a, 0x24, 0x64, 0x69,
    0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x6f, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x63, 0x68, 0x65,
    0x63, 0x6b, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x48, 0x09, 0x52, 0x20, 0x64, 0x69, 0x73, 0x61,
    0x62, 0x6c, 0x65, 0x52, 0x75, 0x6c, 0x65, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72,
    0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x88, 0x01, 0x01, 0x12,
    0x4d, 0x0a, 0x21, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x5f,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x74, 0x72, 0x65, 0x65, 0x5f, 0x63,
    0x68, 0x65, 0x63, 0x6b, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x48, 0x0a, 0x52, 0x1d, 0x64, 0x69,
    0x73, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x75, 0x6c, 0x65, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x6f, 0x72, 0x54, 0x72, 0x65, 0x65, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x88, 0x01, 0x01, 0x12, 0x3c,
    0x0a, 0x18, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x5f,
    0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x5f, 0x6d, 0x73, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x04,
    0x48, 0x0b, 0x52, 0x15, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x46, 0x65, 0x74, 0x63, 0x68,
    0x54, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x4d, 0x73, 0x88, 0x01, 0x01, 0x12, 0x54, 0x0a, 0x25,
    0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x5f, 0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x6c, 0x6f,
    0x61, 0x64, 0x65, 0x64, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x6f,
    0x75, 0x74, 0x5f, 0x6d, 0x73, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x04, 0x48, 0x0c, 0x52, 0x20, 0x66,
    0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x51, 0x75, 0x65, 0x72, 0x79, 0x4c, 0x6f, 0x61, 0x64, 0x65,
    0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x54, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x4d, 0x73, 0x88,
    0x01, 0x01, 0x12, 0x49, 0x0a, 0x1e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x5f,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x04, 0x48, 0x0d, 0x52, 0x1c, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x43,
    0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x88, 0x01, 0x01, 0x12, 0x1b, 0x0a,
    0x09, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x0f, 0x20, 0x03, 0x28, 0x04,
    0x52, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x49, 0x64, 0x73, 0x42, 0x1d, 0x0a, 0x1b, 0x5f, 0x64,
    0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x70, 0x61, 0x63, 0x6b, 0x65,
    0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x42, 0x25, 0x0a, 0x23, 0x5f, 0x65, 0x6e,
    0x61, 0x62, 0x6c, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x5f,
    0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x42, 0x1c, 0x0a, 0x1a, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x73, 0x65, 0x71,
    0x75, 0x65, 0x6e, 0x63, 0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x42, 0x24,
    0x0a, 0x22, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e,
    0x63, 0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x5f, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x42, 0x1b, 0x0a, 0x19, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65,
    0x5f, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65,
    0x72, 0x42, 0x24, 0x0a, 0x22, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x70, 0x72,
    0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x5f, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x42, 0x19, 0x0a, 0x17, 0x5f, 0x64, 0x69, 0x73, 0x61,
    0x62, 0x6c, 0x65, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x6f, 0x72, 0x42, 0x29, 0x0a, 0x27, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x72,
    0x75, 0x6c, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x6e,
    0x74, 0x65, 0x67, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x42, 0x28, 0x0a,
    0x26, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x5f, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63,
    0x65, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x42, 0x27, 0x0a, 0x25, 0x5f, 0x64, 0x69, 0x73, 0x61,
    0x62, 0x6c, 0x65, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x6f, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b,
    0x42, 0x24, 0x0a, 0x22, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x72, 0x75, 0x6c,
    0x65, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x74, 0x72, 0x65, 0x65,
    0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x42, 0x1b, 0x0a, 0x19, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68,
    0x65, 0x72, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74,
    0x5f, 0x6d, 0x73, 0x42, 0x28, 0x0a, 0x26, 0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x72, 0x5f,
    0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x65, 0x64, 0x5f, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x5f, 0x6d, 0x73, 0x42, 0x21, 0x0a,
    0x1f, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79,
    0x42, 0xc0, 0x01, 0x0a, 0x20, 0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f,
    0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a,
    0x65, 0x72, 0x2e, 0x76, 0x31, 0x42, 0x09, 0x53, 0x79, 0x6e, 0x63, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43, 0x53, 0xaa, 0x02, 0x1c, 0x4d, 0x79, 0x73, 0x74, 0x69,
    0x6b, 0x6f, 0x2e, 0x43, 0x6f, 0x72, 0x65, 0x2e, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e,
    0x69, 0x7a, 0x65, 0x72, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x1c, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b,
    0x6f, 0x5c, 0x43, 0x6f, 0x72, 0x65, 0x5c, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69,
    0x7a, 0x65, 0x72, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x28, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f,
    0x5c, 0x43, 0x6f, 0x72, 0x65, 0x5c, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a,
    0x65, 0x72, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0xea, 0x02, 0x1f, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x72,
    0x65, 0x3a, 0x3a, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x3a,
    0x3a, 0x56, 0x31, 0x4a, 0xbf, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x14, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02,
    0x00, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x14, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x05, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x12,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x3b, 0x3c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x06, 0x12, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x06, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x02,
    0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x12, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x08, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x08,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x12, 0x33,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08, 0x3b, 0x3c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x09, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x09, 0x12, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x09, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0a, 0x02, 0x3d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x0a, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06,
    0x12, 0x03, 0x0b, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x0b, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0b, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0c, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x05, 0x12, 0x03, 0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x0c, 0x12, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x0c,
    0x3b, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x0d, 0x02, 0x3d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x0d, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12,
    0x03, 0x0e, 0x02, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x0e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x36, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x0e, 0x3b, 0x3d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x0f, 0x02, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05,
    0x12, 0x03, 0x0f, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x0f, 0x12, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x0f, 0x3b,
    0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x10, 0x02, 0x3e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0b, 0x01, 0x12, 0x03, 0x10, 0x12, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b,
    0x03, 0x12, 0x03, 0x10, 0x3b, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03,
    0x11, 0x02, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x11, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x11, 0x12, 0x37, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x11, 0x3b, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x0d, 0x12, 0x03, 0x12, 0x02, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d,
    0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x05, 0x12,
    0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x12,
    0x12, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x12, 0x3b, 0x3d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x13, 0x02, 0x3e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0e, 0x01, 0x12, 0x03, 0x13, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x03,
    0x12, 0x03, 0x13, 0x3b, 0x3d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xdb, 0x06,
    0x0a, 0x28, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x73,
    0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x72,
    0x65, 0x73, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1c, 0x6d, 0x79, 0x73, 0x74,
    0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f,
    0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x22, 0x96, 0x01, 0x0a, 0x11, 0x52, 0x65, 0x73,
    0x65, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x19,
    0x0a, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x07, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x49, 0x64, 0x12, 0x2d, 0x0a, 0x12, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x61, 0x63, 0x74, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x11, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x41,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x12, 0x26, 0x0a, 0x0c, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00,
    0x52, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x88, 0x01, 0x01,
    0x42, 0x0f, 0x0a, 0x0d, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65,
    0x72, 0x22, 0x57, 0x0a, 0x0c, 0x52, 0x65, 0x73, 0x65, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x12, 0x47, 0x0a, 0x06, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x2f, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65,
    0x2e, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2e, 0x76, 0x31,
    0x2e, 0x52, 0x65, 0x73, 0x65, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x4f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x52, 0x06, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x73, 0x42, 0xc1, 0x01, 0x0a, 0x20, 0x63,
    0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e,
    0x73, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x42,
    0x0a, 0x52, 0x65, 0x73, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03,
    0x4d, 0x43, 0x53, 0xaa, 0x02, 0x1c, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x43, 0x6f,
    0x72, 0x65, 0x2e, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2e,
    0x56, 0x31, 0xca, 0x02, 0x1c, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x72,
    0x65, 0x5c, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x5c, 0x56,
    0x31, 0xe2, 0x02, 0x28, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x72, 0x65,
    0x5c, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x5c, 0x56, 0x31,
    0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1f, 0x4d,
    0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x72, 0x65, 0x3a, 0x3a, 0x53, 0x79,
    0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xd2,
    0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x25, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x04, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05,
    0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x09, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x06, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x27,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x02, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x07, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0a, 0x00,
    0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0b, 0x1d, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0b, 0x26, 0x27, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0x90, 0x09, 0x0a, 0x29,
    0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x73, 0x79, 0x6e,
    0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1c, 0x6d, 0x79, 0x73, 0x74, 0x69,
    0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e,
    0x69, 0x7a, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x22, 0x5e, 0x0a, 0x0e, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x61, 0x63, 0x74, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x29, 0x0a, 0x10, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x61, 0x63, 0x74, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x41, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x79, 0x6e, 0x63, 0x65, 0x64, 0x5f, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x73, 0x79, 0x6e, 0x63,
    0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x22, 0xba, 0x01, 0x0a, 0x0b, 0x43, 0x68, 0x61, 0x69,
    0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x63, 0x68, 0x61, 0x69, 0x6e,
    0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x79, 0x6e, 0x63, 0x65, 0x64, 0x5f, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x73, 0x79, 0x6e, 0x63, 0x65, 0x64,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x21, 0x0a, 0x0c, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x74, 0x61, 0x72,
    0x67, 0x65, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x4a, 0x0a, 0x09, 0x63, 0x6f, 0x6e, 0x74,
    0x72, 0x61, 0x63, 0x74, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x6d, 0x79,
    0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x79, 0x6e, 0x63, 0x68,
    0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x61, 0x63, 0x74, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x09, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x61, 0x63, 0x74, 0x73, 0x22, 0x57, 0x0a, 0x12, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e,
    0x69, 0x7a, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x41, 0x0a, 0x06, 0x63, 0x68,
    0x61, 0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x6d, 0x79, 0x73,
    0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72,
    0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x73, 0x42, 0xc2, 0x01,
    0x0a, 0x20, 0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f,
    0x72, 0x65, 0x2e, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x2e,
    0x76, 0x31, 0x42, 0x0b, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50,
    0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43, 0x53, 0xaa, 0x02, 0x1c, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b,
    0x6f, 0x2e, 0x43, 0x6f, 0x72, 0x65, 0x2e, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69,
    0x7a, 0x65, 0x72, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x1c, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f,
    0x5c, 0x43, 0x6f, 0x72, 0x65, 0x5c, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a,
    0x65, 0x72, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x28, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c,
    0x43, 0x6f, 0x72, 0x65, 0x5c, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65,
    0x72, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61,
    0xea, 0x02, 0x1f, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x72, 0x65,
    0x3a, 0x3a, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x72, 0x3a, 0x3a,
    0x56, 0x31, 0x4a, 0x81, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x12, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00,
    0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x05, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x05, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x09, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x1c, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x06, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x09,
    0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x0a, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x0b, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x0b, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x26, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x0c, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x0c, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x02,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x1a, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x10, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x11, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x11, 0x20, 0x21, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("mystiko.core.synchronizer.v1.serde.rs");
// @@protoc_insertion_point(module)