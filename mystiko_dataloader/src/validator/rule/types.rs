use crate::data::{ContractData, LoadedData};
use crate::validator::ValidateOption;
use mystiko_config::PoolContractConfig;
use mystiko_protos::data::v1::CommitmentStatus;
use num_bigint::BigUint;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct ValidateOriginalData<'a, R: LoadedData> {
    pub chain_id: u64,
    pub contract_config: &'a PoolContractConfig,
    pub contract_data: &'a ContractData<R>,
    pub option: &'a ValidateOption,
}

impl<'a, R> ValidateOriginalData<'a, R>
where
    R: LoadedData,
{
    pub fn should_skip_commitment_check(&self) -> bool {
        if let Some(disabled_at) = self.contract_config.disabled_at() {
            if self.contract_data.start_block > *disabled_at {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Eq, PartialEq, TypedBuilder)]
pub struct ValidateMergedData {
    pub chain_id: u64,
    pub contract_address: String,
    pub start_block: u64,
    pub end_block: u64,
    pub commitments: Vec<ValidateCommitment>,
    pub nullifiers: Option<Vec<ValidateNullifier>>,
}

#[derive(Debug, Clone, Eq, PartialEq, TypedBuilder)]
pub struct ValidateCommitment {
    pub commitment_hash: BigUint,
    pub status: CommitmentStatus,
    pub leaf_index: u64,
    pub inner_merge: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, TypedBuilder)]
pub struct ValidateNullifier {
    pub nullifier: BigUint,
}
