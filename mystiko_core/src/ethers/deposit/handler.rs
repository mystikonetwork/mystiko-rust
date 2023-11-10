use crate::{
    CrossChainDepositOptions, DepositContractHandler, DepositOptions, DepositQuote, DepositQuoteOptions, FromContext,
    MystikoContext, MystikoError, TransactionSigner,
};
use async_trait::async_trait;
use ethers_contract::ContractCall;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash, U256};
use mystiko_config::{DepositContractConfig, MystikoConfig, PoolContractConfig};
use mystiko_ethers::{Provider, Providers};
use mystiko_storage::{StatementFormatter, Storage};
use mystiko_types::{AssetType, BridgeType};
use mystiko_utils::address::{ethers_address_from_string, ethers_address_to_string};
use mystiko_utils::convert::biguint_to_u256;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use typed_builder::TypedBuilder;

pub const DEFAULT_QUOTE_TIMEOUT_MS: u64 = 2000;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DepositContracts<P: Providers> {
    providers: Arc<P>,
    config: Arc<MystikoConfig>,
}

#[derive(Debug, Error)]
pub enum DepositContractsError {
    #[error("unsupported chain_id={0}")]
    UnsupportedChainError(u64),
    #[error("unsupported chain_id={0}, contract_address={1}")]
    UnsupportedContractError(u64, String),
    #[error("providers raised error: {0}")]
    ProviderPoolError(anyhow::Error),
    #[error("transaction signer raised error: {0}")]
    SignerError(String),
    #[error("mystiko_config raised error: {0}")]
    ConfigError(anyhow::Error),
    #[error("sending deposit transaction timed out after {0} ms")]
    DepositTimeoutError(u64),
    #[error("sending cross_chain_deposit transaction timed out after {0} ms")]
    CrossChainDepositTimeoutError(u64),
    #[error(transparent)]
    ContractError(#[from] ethers_contract::ContractError<Provider>),
    #[error(transparent)]
    FromHexError(#[from] rustc_hex::FromHexError),
}

#[async_trait]
impl<P> DepositContractHandler for DepositContracts<P>
where
    P: Providers,
{
    type Error = DepositContractsError;

    async fn quote(&self, options: DepositQuoteOptions) -> Result<DepositQuote, Self::Error> {
        let contract_config = self.get_config(options.chain_id, &options.contract_address)?;
        let mut tasks = vec![
            self.get_remote_config(&options, contract_config, RemoteConfigType::MinAmount),
            self.get_remote_config(&options, contract_config, RemoteConfigType::MaxAmount),
            self.get_remote_config(&options, contract_config, RemoteConfigType::MinRollupFee),
        ];
        if contract_config.bridge_type() != &BridgeType::Loop {
            tasks.push(self.get_remote_config(&options, contract_config, RemoteConfigType::MinBridgeFee));
            tasks.push(self.get_remote_config(&options, contract_config, RemoteConfigType::MinExecutorFee));
        }
        let mut results = futures::future::try_join_all(tasks)
            .await?
            .into_iter()
            .collect::<HashMap<_, _>>();
        Ok(DepositQuote::builder()
            .min_amount(results.remove(&RemoteConfigType::MinAmount).unwrap_or_default())
            .max_amount(results.remove(&RemoteConfigType::MaxAmount).unwrap_or_default())
            .min_rollup_fee_amount(results.remove(&RemoteConfigType::MinRollupFee).unwrap_or_default())
            .min_bridge_fee_amount(results.remove(&RemoteConfigType::MinBridgeFee))
            .min_executor_fee_amount(results.remove(&RemoteConfigType::MinExecutorFee))
            .build())
    }

    async fn deposit<T, S>(&self, options: DepositOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
        S: TransactionSigner + 'static,
    {
        let contract_config = self.get_config(options.chain_id, &options.contract_address)?;
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(DepositContractsError::ProviderPoolError)?;
        let contract = mystiko_abi::mystiko_v2_loop::MystikoV2Loop::new(options.contract_address, provider);
        let mut tx = options.tx.into();
        tx.set_to(options.contract_address);
        if contract_config.asset_type() == &AssetType::Main {
            tx.set_value(options.amount.add(options.rollup_fee));
        }
        let request = mystiko_abi::mystiko_v2_loop::DepositRequest {
            amount: options.amount,
            commitment: options.commitment,
            hash_k: options.hash_k,
            random_s: options.random_s,
            encrypted_note: options.encrypted_notes,
            rollup_fee: options.rollup_fee,
        };
        if let Some(data) = contract.deposit(request).calldata() {
            tx.set_data(data);
        }
        let tx_hash = if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(
                Duration::from_millis(timeout_ms),
                options.signer.send_transaction(options.chain_id, tx),
            )
            .await
            {
                Err(_) => return Err(DepositContractsError::DepositTimeoutError(timeout_ms)),
                Ok(result) => result.map_err(|err| DepositContractsError::SignerError(format!("{:?}", err)))?,
            }
        } else {
            options
                .signer
                .send_transaction(options.chain_id, tx)
                .await
                .map_err(|err| DepositContractsError::SignerError(format!("{:?}", err)))?
        };
        Ok(tx_hash)
    }

    async fn cross_chain_deposit<T, S>(&self, options: CrossChainDepositOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
        S: TransactionSigner + 'static,
    {
        let contract_config = self.get_config(options.chain_id, &options.contract_address)?;
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(DepositContractsError::ProviderPoolError)?;
        let contract = mystiko_abi::mystiko_v2_bridge::MystikoV2Bridge::new(options.contract_address, provider);
        let mut tx = options.tx.into();
        tx.set_to(options.contract_address);
        let mut value = U256::zero();
        if contract_config.asset_type() == &AssetType::Main {
            value = value.add(options.amount).add(options.rollup_fee);
        }
        if contract_config.executor_fee_asset().asset_type() == &AssetType::Main {
            value = value.add(options.executor_fee);
        }
        if contract_config.bridge_fee_asset().asset_type() == &AssetType::Main {
            value = value.add(options.bridge_fee);
        }
        if value.gt(&U256::zero()) {
            tx.set_value(value);
        }
        let request = mystiko_abi::mystiko_v2_bridge::DepositRequest {
            amount: options.amount,
            commitment: options.commitment,
            hash_k: options.hash_k,
            random_s: options.random_s,
            encrypted_note: options.encrypted_notes,
            rollup_fee: options.rollup_fee,
            executor_fee: options.executor_fee,
            bridge_fee: options.bridge_fee,
        };
        if let Some(data) = contract.deposit(request).calldata() {
            tx.set_data(data);
        }
        let tx_hash = if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(
                Duration::from_millis(timeout_ms),
                options.signer.send_transaction(options.chain_id, tx),
            )
            .await
            {
                Err(_) => return Err(DepositContractsError::CrossChainDepositTimeoutError(timeout_ms)),
                Ok(result) => result.map_err(|err| DepositContractsError::SignerError(format!("{:?}", err)))?,
            }
        } else {
            options
                .signer
                .send_transaction(options.chain_id, tx)
                .await
                .map_err(|err| DepositContractsError::SignerError(format!("{:?}", err)))?
        };
        Ok(tx_hash)
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for DepositContracts<Box<dyn Providers>>
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        Ok(Self::builder()
            .providers(context.providers.clone())
            .config(context.config.clone())
            .build())
    }
}

impl<P> DepositContracts<P>
where
    P: Providers,
{
    fn get_config(&self, chain_id: u64, address: &Address) -> Result<&DepositContractConfig, DepositContractsError> {
        if let Some(chain_config) = self.config.find_chain(chain_id) {
            let address = ethers_address_to_string(address);
            if let Some(contract_config) = chain_config.find_deposit_contract_by_address(&address) {
                return Ok(contract_config);
            }
            return Err(DepositContractsError::UnsupportedContractError(chain_id, address));
        }
        Err(DepositContractsError::UnsupportedChainError(chain_id))
    }

    async fn get_remote_config(
        &self,
        options: &DepositQuoteOptions,
        contract_config: &DepositContractConfig,
        config_type: RemoteConfigType,
    ) -> Result<(RemoteConfigType, U256), DepositContractsError> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(DepositContractsError::ProviderPoolError)?;
        let (pool_contract_config, peer_provider) = if let (Some(peer_chain_id), Some(peer_contract_address)) =
            (contract_config.peer_chain_id(), contract_config.peer_contract_address())
        {
            let peer_contract_config =
                self.get_config(*peer_chain_id, &ethers_address_from_string(peer_contract_address)?)?;
            let peer_provider = self
                .providers
                .get_provider(*peer_chain_id)
                .await
                .map_err(DepositContractsError::ProviderPoolError)?;
            (peer_contract_config.pool_contract(), peer_provider)
        } else {
            (contract_config.pool_contract(), provider.clone())
        };
        let (default_value, call) = match config_type {
            RemoteConfigType::MinAmount => get_min_amount(provider, &options.contract_address, contract_config),
            RemoteConfigType::MaxAmount => get_max_amount(provider, &options.contract_address, contract_config),
            RemoteConfigType::MinRollupFee => get_min_rollup_fee(peer_provider, pool_contract_config)?,
            RemoteConfigType::MinBridgeFee => get_min_bridge_fee(provider, &options.contract_address, contract_config),
            RemoteConfigType::MinExecutorFee => {
                get_min_executor_fee(provider, &options.contract_address, contract_config)
            }
        };
        let timeout_ms = options.timeout_ms.unwrap_or(DEFAULT_QUOTE_TIMEOUT_MS);
        let timeout = Duration::from_millis(timeout_ms);
        match tokio::time::timeout(timeout, get_remote_config_with_call(call, &config_type, default_value)).await {
            Ok(value) => Ok((config_type, value)),
            Err(_) => {
                log::warn!(
                    "get_remote_config with config_type={:?} timed out after {} ms",
                    config_type,
                    timeout_ms
                );
                Ok((config_type, default_value))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum RemoteConfigType {
    MinAmount,
    MaxAmount,
    MinRollupFee,
    MinBridgeFee,
    MinExecutorFee,
}

fn get_min_amount(
    provider: Arc<Provider>,
    address: &Address,
    contract_config: &DepositContractConfig,
) -> (U256, ContractCall<Provider, U256>) {
    let default_value = biguint_to_u256(&contract_config.min_amount().unwrap_or_default());
    let call = if contract_config.bridge_type() == &BridgeType::Loop {
        mystiko_abi::mystiko_v2_loop::MystikoV2Loop::new(*address, provider).get_min_amount()
    } else {
        mystiko_abi::mystiko_v2_bridge::MystikoV2Bridge::new(*address, provider).get_min_amount()
    };
    (default_value, call)
}

fn get_max_amount(
    provider: Arc<Provider>,
    address: &Address,
    contract_config: &DepositContractConfig,
) -> (U256, ContractCall<Provider, U256>) {
    let default_value = biguint_to_u256(&contract_config.max_amount().unwrap_or_default());
    let call = if contract_config.bridge_type() == &BridgeType::Loop {
        mystiko_abi::mystiko_v2_loop::MystikoV2Loop::new(*address, provider).get_max_amount()
    } else {
        mystiko_abi::mystiko_v2_bridge::MystikoV2Bridge::new(*address, provider).get_max_amount()
    };
    (default_value, call)
}

fn get_min_rollup_fee(
    provider: Arc<Provider>,
    contract_config: &PoolContractConfig,
) -> Result<(U256, ContractCall<Provider, U256>), DepositContractsError> {
    let address = ethers_address_from_string(contract_config.address())?;
    let default_value = biguint_to_u256(&contract_config.min_rollup_fee().unwrap_or_default());
    let call = mystiko_abi::commitment_pool::CommitmentPool::new(address, provider).get_min_rollup_fee();
    Ok((default_value, call))
}

fn get_min_bridge_fee(
    provider: Arc<Provider>,
    address: &Address,
    contract_config: &DepositContractConfig,
) -> (U256, ContractCall<Provider, U256>) {
    let default_value = biguint_to_u256(&contract_config.min_bridge_fee().unwrap_or_default());
    let call = mystiko_abi::mystiko_v2_bridge::MystikoV2Bridge::new(*address, provider).get_min_bridge_fee();
    (default_value, call)
}

fn get_min_executor_fee(
    provider: Arc<Provider>,
    address: &Address,
    contract_config: &DepositContractConfig,
) -> (U256, ContractCall<Provider, U256>) {
    let default_value = biguint_to_u256(&contract_config.min_executor_fee().unwrap_or_default());
    let call = mystiko_abi::mystiko_v2_bridge::MystikoV2Bridge::new(*address, provider).get_min_executor_fee();
    (default_value, call)
}

async fn get_remote_config_with_call(
    call: ContractCall<Provider, U256>,
    config_type: &RemoteConfigType,
    default_value: U256,
) -> U256 {
    match call.await {
        Ok(value) => value,
        Err(err) => {
            log::warn!(
                "get_remote_config with config_type={:?} raised error: {}",
                config_type,
                err
            );
            default_value
        }
    }
}
