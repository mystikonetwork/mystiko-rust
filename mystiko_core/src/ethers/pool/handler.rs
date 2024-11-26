use crate::{
    AuditorPublicKeysOptions, CommitmentPoolContractHandler, FromContext, IncludedCountOptions,
    IsHistoricCommitmentOptions, IsKnownRootOptions, IsSpentNullifierOptions, MinRollupFeeOptions, MystikoContext,
    MystikoError, TransactOptions, TransactionSigner,
};
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{TxHash, U256};
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
    #[error(transparent)]
    ContractError(#[from] ethers_contract::ContractError<Provider>),
    #[error("providers raised error: {0}")]
    ProviderPoolError(anyhow::Error),
    #[error("querying is_historic_commitment timed out after {0} ms")]
    IsHistoricCommitmentTimeoutError(u64),
    #[error("querying is_spent_nullifier timed out after {0} ms")]
    IsSpentNullifierTimeoutError(u64),
    #[error("querying is_known_root timed out after {0} ms")]
    IsKnownRootTimeoutError(u64),
    #[error("querying min_rollup_fee timed out after {0} ms")]
    MinRollupFeeTimeoutError(u64),
    #[error("querying auditor_public_keys timed out after {0} ms")]
    AuditorPublicKeysTimeoutError(u64),
    #[error("sending transact transaction timed out after {0} ms")]
    TransactTimeoutError(u64),
    #[error("transaction signer raised error: {0}")]
    SignerError(String),
    #[error("converting proof raised error: {0}")]
    ConvertProofError(anyhow::Error),
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

    async fn min_rollup_fee(&self, options: MinRollupFeeOptions) -> Result<U256, Self::Error> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(CommitmentPoolContractsError::ProviderPoolError)?;
        let contract = CommitmentPool::new(options.contract_address, provider);
        if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(Duration::from_millis(timeout_ms), async {
                contract.get_min_rollup_fee().await
            })
            .await
            {
                Err(_) => Err(CommitmentPoolContractsError::MinRollupFeeTimeoutError(timeout_ms)),
                Ok(result) => Ok(result?),
            }
        } else {
            Ok(contract.get_min_rollup_fee().await?)
        }
    }

    async fn get_commitment_included_count(&self, options: IncludedCountOptions) -> Result<U256, Self::Error> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(CommitmentPoolContractsError::ProviderPoolError)?;
        let contract = CommitmentPool::new(options.contract_address, provider);
        if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(Duration::from_millis(timeout_ms), async {
                contract.get_commitment_included_count().await
            })
            .await
            {
                Err(_) => Err(CommitmentPoolContractsError::AuditorPublicKeysTimeoutError(timeout_ms)),
                Ok(result) => Ok(result?),
            }
        } else {
            Ok(contract.get_commitment_included_count().await?)
        }
    }

    async fn auditor_public_keys(&self, options: AuditorPublicKeysOptions) -> Result<Vec<U256>, Self::Error> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(CommitmentPoolContractsError::ProviderPoolError)?;
        let contract = CommitmentPool::new(options.contract_address, provider);
        if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(Duration::from_millis(timeout_ms), async {
                contract.get_all_auditor_public_keys().await
            })
            .await
            {
                Err(_) => Err(CommitmentPoolContractsError::AuditorPublicKeysTimeoutError(timeout_ms)),
                Ok(result) => Ok(result?),
            }
        } else {
            Ok(contract.get_all_auditor_public_keys().await?)
        }
    }

    async fn transact<T, S>(&self, options: TransactOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
        S: TransactionSigner + 'static,
    {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(CommitmentPoolContractsError::ProviderPoolError)?;
        let contract = CommitmentPool::new(options.contract_address, provider);
        let mut tx = options.tx.into();
        tx.set_to(options.contract_address);
        if let Some(data) = contract.transact(options.request, options.signature).calldata() {
            tx.set_data(data);
        }
        let tx_hash = if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(
                Duration::from_millis(timeout_ms),
                options.signer.send_transaction(options.chain_id, tx),
            )
            .await
            {
                Err(_) => return Err(CommitmentPoolContractsError::TransactTimeoutError(timeout_ms)),
                Ok(result) => result.map_err(|err| CommitmentPoolContractsError::SignerError(format!("{:?}", err)))?,
            }
        } else {
            options
                .signer
                .send_transaction(options.chain_id, tx)
                .await
                .map_err(|err| CommitmentPoolContractsError::SignerError(format!("{:?}", err)))?
        };
        Ok(tx_hash)
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
