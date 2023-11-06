// @generated
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWalletOptions {
    #[prost(string, tag="1")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub mnemonic_phrase: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountOptions {
    #[prost(string, tag="1")]
    pub wallet_password: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub secret_key: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(mystiko_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountOptions {
    #[prost(string, tag="1")]
    pub wallet_password: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Encoded file descriptor set for the `mystiko.core.handler.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x9b, 0x04, 0x0a, 0x24, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x72,
    0x65, 0x2f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x77, 0x61, 0x6c,
    0x6c, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x6d, 0x79, 0x73, 0x74, 0x69,
    0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x2e,
    0x76, 0x31, 0x22, 0x73, 0x0a, 0x13, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x57, 0x61, 0x6c, 0x6c,
    0x65, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x61, 0x73,
    0x73, 0x77, 0x6f, 0x72, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x70, 0x61, 0x73,
    0x73, 0x77, 0x6f, 0x72, 0x64, 0x12, 0x2c, 0x0a, 0x0f, 0x6d, 0x6e, 0x65, 0x6d, 0x6f, 0x6e, 0x69,
    0x63, 0x5f, 0x70, 0x68, 0x72, 0x61, 0x73, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00,
    0x52, 0x0e, 0x6d, 0x6e, 0x65, 0x6d, 0x6f, 0x6e, 0x69, 0x63, 0x50, 0x68, 0x72, 0x61, 0x73, 0x65,
    0x88, 0x01, 0x01, 0x42, 0x12, 0x0a, 0x10, 0x5f, 0x6d, 0x6e, 0x65, 0x6d, 0x6f, 0x6e, 0x69, 0x63,
    0x5f, 0x70, 0x68, 0x72, 0x61, 0x73, 0x65, 0x42, 0xa9, 0x01, 0x0a, 0x1b, 0x63, 0x6f, 0x6d, 0x2e,
    0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x68, 0x61, 0x6e,
    0x64, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x42, 0x0b, 0x57, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43, 0x48, 0xaa, 0x02, 0x17, 0x4d,
    0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x43, 0x6f, 0x72, 0x65, 0x2e, 0x48, 0x61, 0x6e, 0x64,
    0x6c, 0x65, 0x72, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x17, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f,
    0x5c, 0x43, 0x6f, 0x72, 0x65, 0x5c, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x5c, 0x56, 0x31,
    0xe2, 0x02, 0x23, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x72, 0x65, 0x5c,
    0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1a, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f,
    0x3a, 0x3a, 0x43, 0x6f, 0x72, 0x65, 0x3a, 0x3a, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x3a,
    0x3a, 0x56, 0x31, 0x4a, 0xb0, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x07, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02,
    0x00, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x07, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x05, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x05, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x24,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x06, 0x24, 0x25, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xfb,
    0x06, 0x0a, 0x25, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f,
    0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b,
    0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x2e, 0x76,
    0x31, 0x22, 0x94, 0x01, 0x0a, 0x14, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x41, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x27, 0x0a, 0x0f, 0x77, 0x61,
    0x6c, 0x6c, 0x65, 0x74, 0x5f, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0e, 0x77, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x50, 0x61, 0x73, 0x73, 0x77,
    0x6f, 0x72, 0x64, 0x12, 0x17, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x48, 0x00, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x88, 0x01, 0x01, 0x12, 0x22, 0x0a, 0x0a,
    0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x48, 0x01, 0x52, 0x09, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x88, 0x01, 0x01,
    0x42, 0x07, 0x0a, 0x05, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x42, 0x0d, 0x0a, 0x0b, 0x5f, 0x73, 0x65,
    0x63, 0x72, 0x65, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x22, 0x61, 0x0a, 0x14, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x12, 0x27, 0x0a, 0x0f, 0x77, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x5f, 0x70, 0x61, 0x73, 0x73, 0x77,
    0x6f, 0x72, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x77, 0x61, 0x6c, 0x6c, 0x65,
    0x74, 0x50, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x12, 0x17, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x88,
    0x01, 0x01, 0x42, 0x07, 0x0a, 0x05, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x42, 0xaa, 0x01, 0x0a, 0x1b,
    0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65,
    0x2e, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x42, 0x0c, 0x41, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x43,
    0x48, 0xaa, 0x02, 0x17, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x2e, 0x43, 0x6f, 0x72, 0x65,
    0x2e, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x17, 0x4d, 0x79,
    0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c, 0x43, 0x6f, 0x72, 0x65, 0x5c, 0x48, 0x61, 0x6e, 0x64, 0x6c,
    0x65, 0x72, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x23, 0x4d, 0x79, 0x73, 0x74, 0x69, 0x6b, 0x6f, 0x5c,
    0x43, 0x6f, 0x72, 0x65, 0x5c, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x5c, 0x56, 0x31, 0x5c,
    0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1a, 0x4d, 0x79,
    0x73, 0x74, 0x69, 0x6b, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x72, 0x65, 0x3a, 0x3a, 0x48, 0x61, 0x6e,
    0x64, 0x6c, 0x65, 0x72, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0x89, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x0d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04,
    0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x05, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x06, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07,
    0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07, 0x1f, 0x20,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0a, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x0b, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x0b,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x1d, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0c, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0c, 0x1d, 0x1e, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("mystiko.core.handler.v1.serde.rs");
// @@protoc_insertion_point(module)