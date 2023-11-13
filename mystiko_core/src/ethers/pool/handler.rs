use crate::{
    CommitmentPoolContractHandler, FromContext, IsHistoricCommitmentOptions, IsKnownRootOptions,
    IsSpentNullifierOptions, MystikoContext, MystikoError,
};
use async_trait::async_trait;
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_ethers::{Provider, Providers};
use mystiko_storage::{StatementFormatter, Storage};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitmentPoolContracts<P: Providers> {
    providers: Arc<P>,
}

#[derive(Debug, Error)]
pub enum CommitmentPoolContractsError {
    #[error("providers raised error: {0}")]
    ProviderPoolError(anyhow::Error),
    #[error("querying is_historic_commitment timed out after {0} ms")]
    IsHistoricCommitmentTimeoutError(u64),
    #[error("querying is_spent_nullifier timed out after {0} ms")]
    IsSpentNullifierTimeoutError(u64),
    #[error("querying is_known_root timed out after {0} ms")]
    IsKnownRootTimeoutError(u64),
    #[error(transparent)]
    ContractError(#[from] ethers_contract::ContractError<Provider>),
}

#[async_trait]
impl<P> CommitmentPoolContractHandler for CommitmentPoolContracts<P>
where
    P: Providers,
{
    type Error = CommitmentPoolContractsError;

    async fn is_historic_commitment(&self, options: IsHistoricCommitmentOptions) -> Result<bool, Self::Error> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(CommitmentPoolContractsError::ProviderPoolError)?;
        let contract = CommitmentPool::new(options.contract_address, provider);
        if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(Duration::from_millis(timeout_ms), async {
                contract.is_historic_commitment(options.commitment_hash).await
            })
            .await
            {
                Err(_) => Err(CommitmentPoolContractsError::IsHistoricCommitmentTimeoutError(
                    timeout_ms,
                )),
                Ok(result) => Ok(result?),
            }
        } else {
            Ok(contract.is_historic_commitment(options.commitment_hash).await?)
        }
    }

    async fn is_spent_nullifier(&self, options: IsSpentNullifierOptions) -> Result<bool, Self::Error> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(CommitmentPoolContractsError::ProviderPoolError)?;
        let contract = CommitmentPool::new(options.contract_address, provider);
        if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(Duration::from_millis(timeout_ms), async {
                contract.is_spent_serial_number(options.nullifier).await
            })
            .await
            {
                Err(_) => Err(CommitmentPoolContractsError::IsSpentNullifierTimeoutError(timeout_ms)),
                Ok(result) => Ok(result?),
            }
        } else {
            Ok(contract.is_spent_serial_number(options.nullifier).await?)
        }
    }

    async fn is_known_root(&self, options: IsKnownRootOptions) -> Result<bool, Self::Error> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(CommitmentPoolContractsError::ProviderPoolError)?;
        let contract = CommitmentPool::new(options.contract_address, provider);
        if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(Duration::from_millis(timeout_ms), async {
                contract.is_known_root(options.root_hash).await
            })
            .await
            {
                Err(_) => Err(CommitmentPoolContractsError::IsKnownRootTimeoutError(timeout_ms)),
                Ok(result) => Ok(result?),
            }
        } else {
            Ok(contract.is_known_root(options.root_hash).await?)
        }
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for CommitmentPoolContracts<Box<dyn Providers>>
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        Ok(Self::builder().providers(context.providers.clone()).build())
    }
}
