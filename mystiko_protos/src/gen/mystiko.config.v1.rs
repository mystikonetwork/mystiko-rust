// @generated
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigOptions {
    #[prost(string, optional, tag="1")]
    pub file_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="2")]
    pub is_testnet: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub is_staging: ::core::option::Option<bool>,
    #[prost(string, optional, tag="4")]
    pub remote_base_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub git_revision: ::core::option::Option<::prost::alloc::string::String>,
}
/// Encoded file descriptor set for the `mystiko.config.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfc, 0x06, 0x0a, 0x1f, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2f, 0x76, 0x31, 0x2f, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x11, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x76, 0x31, 0x22, 0x9f, 0x02, 0x0a, 0x0d, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x20, 0x0a, 0x09, 0x66, 0x69, 0x6c,
    0x65, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x08,
    0x66, 0x69, 0x6c, 0x65, 0x50, 0x61, 0x74, 0x68, 0x88, 0x01, 0x01, 0x12, 0x22, 0x0a, 0x0a, 0x69,
    0x73, 0x5f, 0x74, 0x65, 0x73, 0x74, 0x6e, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x48,
    0x01, 0x52, 0x09, 0x69, 0x73, 0x54, 0x65, 0x73, 0x74, 0x6e, 0x65, 0x74, 0x88, 0x01, 0x01, 0x12,
    0x22, 0x0a, 0x0a, 0x69, 0x73, 0x5f, 0x73, 0x74, 0x61, 0x67, 0x69, 0x6e, 0x67, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x08, 0x48, 0x02, 0x52, 0x09, 0x69, 0x73, 0x53, 0x74, 0x61, 0x67, 0x69, 0x6e, 0x67,
    0x88, 0x01, 0x01, 0x12, 0x2b, 0x0a, 0x0f, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x5f, 0x62, 0x61,
    0x73, 0x65, 0x5f, 0x75, 0x72, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x03, 0x52, 0x0d,
    0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x42, 0x61, 0x73, 0x65, 0x55, 0x72, 0x6c, 0x88, 0x01, 0x01,
    0x12, 0x26, 0x0a, 0x0c, 0x67, 0x69, 0x74, 0x5f, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x48, 0x04, 0x52, 0x0b, 0x67, 0x69, 0x74, 0x52, 0x65, 0x76,
    0x69, 0x73, 0x69, 0x6f, 0x6e, 0x88, 0x01, 0x01, 0x42, 0x0c, 0x0a, 0x0a, 0x5f, 0x66, 0x69, 0x6c,
    0x65, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x42, 0x0d, 0x0a, 0x0b, 0x5f, 0x69, 0x73, 0x5f, 0x74, 0x65,
    0x73, 0x74, 0x6e, 0x65, 0x74, 0x42, 0x0d, 0x0a, 0x0b, 0x5f, 0x69, 0x73, 0x5f, 0x73, 0x74, 0x61,
    0x67, 0x69, 0x6e, 0x67, 0x42, 0x12, 0x0a, 0x10, 0x5f, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x5f,
    0x62, 0x61, 0x73, 0x65, 0x5f, 0x75, 0x72, 0x6c, 0x42, 0x0f, 0x0a, 0x0d, 0x5f, 0x67, 0x69, 0x74,
    0x5f, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x8b, 0x01, 0x0a, 0x15, 0x63, 0x6f,
    0x6d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x2e, 0x76, 0x31, 0x42, 0x0c, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43, 0x58, 0xaa, 0x02, 0x11, 0x4d, 0x79, 0x73, 0x74,
    0x69, 0x6b, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x11,
    0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5c, 0x56,
    0x31, 0xe2, 0x02, 0x1d, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0xea, 0x02, 0x13, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0x8d, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x0a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00,
    0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x15, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x05, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x05, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x07, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x07, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x12, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07, 0x24, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x08, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x08, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x08, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08,
    0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x09, 0x02, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x09, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x09, 0x24, 0x25, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("mystiko.config.v1.serde.rs");
// @@protoc_insertion_point(module)