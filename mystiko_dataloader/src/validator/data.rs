use mystiko_protos::data::v1::CommitmentStatus;
use num_bigint::BigUint;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct ValidateContractData {
    pub chain_id: u64,
    pub contract_address: String,
    pub start_block: u64,
    pub end_block: u64,
    pub commitments: Vec<ValidateCommitment>,
    pub nullifiers: Vec<ValidateNullifier>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct ValidateCommitment {
    pub commitment_hash: BigUint,
    pub status: CommitmentStatus,
    pub leaf_index: u64,
    pub merged: bool,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct ValidateNullifier {
    pub nullifier: BigUint,
}
