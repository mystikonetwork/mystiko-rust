#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commitment {
    #[prost(bytes = "bytes", tag = "1")]
    pub commitment_hash: ::prost::bytes::Bytes,
    #[prost(enumeration = "CommitmentStatus", tag = "2")]
    pub status: i32,
    #[prost(uint64, optional, tag = "3")]
    pub leaf_index: ::core::option::Option<u64>,
    #[prost(bytes = "bytes", optional, tag = "4")]
    pub rollup_fee: ::core::option::Option<::prost::bytes::Bytes>,
    #[prost(bytes = "bytes", optional, tag = "5")]
    pub encrypted_note: ::core::option::Option<::prost::bytes::Bytes>,
    #[prost(bytes = "bytes", optional, tag = "6")]
    pub creation_transaction_hash: ::core::option::Option<::prost::bytes::Bytes>,
    #[prost(bytes = "bytes", optional, tag = "7")]
    pub rollup_transaction_hash: ::core::option::Option<::prost::bytes::Bytes>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommitmentStatus {
    Queued = 0,
    Included = 1,
}
impl CommitmentStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommitmentStatus::Queued => "Commitment_STATUS_QUEUED",
            CommitmentStatus::Included => "Commitment_STATUS_INCLUDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Commitment_STATUS_QUEUED" => Some(Self::Queued),
            "Commitment_STATUS_INCLUDED" => Some(Self::Included),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nullifier {
    #[prost(bytes = "bytes", tag = "1")]
    pub nullifier: ::prost::bytes::Bytes,
    #[prost(bytes = "bytes", tag = "2")]
    pub creation_transaction_hash: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractData {
    #[prost(uint64, tag = "1")]
    pub start_block: u64,
    #[prost(uint64, tag = "2")]
    pub end_block: u64,
    #[prost(bytes = "bytes", tag = "3")]
    pub contract_address: ::prost::bytes::Bytes,
    #[prost(message, repeated, tag = "4")]
    pub commitments: ::prost::alloc::vec::Vec<Commitment>,
    #[prost(message, repeated, tag = "5")]
    pub nullifiers: ::prost::alloc::vec::Vec<Nullifier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainData {
    #[prost(message, repeated, tag = "1")]
    pub contract_data: ::prost::alloc::vec::Vec<ContractData>,
}
