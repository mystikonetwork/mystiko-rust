use crate::data::ChainData;
use crate::data::ContractData;
use crate::data::LoadedData;
use anyhow::Result;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Error, Debug)]
pub struct ContractError {
    pub address: String,
    #[source]
    pub source: anyhow::Error,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContractResult<T> {
    pub address: String,
    pub result: Result<T>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainResult<T> {
    pub chain_id: u64,
    #[builder(default)]
    pub contract_results: Vec<ContractResult<T>>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct UnwrappedChainResult<T> {
    pub result: T,
    #[builder(default)]
    pub contract_errors: Vec<ContractError>,
}

impl<R> From<ChainResult<ContractData<R>>> for UnwrappedChainResult<ChainData<R>>
where
    R: LoadedData,
{
    fn from(value: ChainResult<ContractData<R>>) -> Self {
        let mut contract_errors: Vec<ContractError> = vec![];
        let mut contracts_data: Vec<ContractData<R>> = vec![];
        for result in value.contract_results.into_iter() {
            match result.result {
                Ok(r) => contracts_data.push(r),
                Err(e) => contract_errors.push(ContractError {
                    address: result.address,
                    source: e,
                }),
            }
        }
        UnwrappedChainResult {
            result: ChainData::builder()
                .chain_id(value.chain_id)
                .contracts_data(contracts_data)
                .build(),
            contract_errors,
        }
    }
}

impl From<ChainResult<()>> for UnwrappedChainResult<Vec<String>> {
    fn from(value: ChainResult<()>) -> Self {
        let mut contract_errors: Vec<ContractError> = vec![];
        let mut contracts_data: Vec<String> = vec![];
        for result in value.contract_results.into_iter() {
            match result.result {
                Ok(_) => contracts_data.push(result.address),
                Err(e) => contract_errors.push(ContractError {
                    address: result.address,
                    source: e,
                }),
            }
        }
        UnwrappedChainResult {
            result: contracts_data,
            contract_errors,
        }
    }
}

impl Display for ContractError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContractError(address = {}): {}", self.address, self.source)
    }
}

impl<T> ChainResult<T>
where
    T: Debug,
{
    pub fn into_unwrapped<R>(self) -> UnwrappedChainResult<R>
    where
        UnwrappedChainResult<R>: From<ChainResult<T>>,
    {
        self.into()
    }
}
