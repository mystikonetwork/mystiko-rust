use crate::tx_manager::config::TxManagerConfig;
use crate::tx_manager::error::{Result, TxManagerError};
use ethers_core::types::transaction::eip1559::Eip1559TransactionRequest;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::transaction::request::TransactionRequest;
use ethers_core::types::transaction::response::TransactionReceipt;
use ethers_core::types::{Address, BlockNumber, TxHash, U256, U64};
// use ethers_middleware::gas_escalator::GasEscalatorMiddleware;
// use ethers_middleware::gas_escalator::{Frequency, GeometricGasPrice};
use ethers_middleware::gas_oracle::{GasOracle, ProviderOracle};
use ethers_middleware::{NonceManagerMiddleware, SignerMiddleware};
use ethers_providers::{JsonRpcClient, Middleware, Provider};
use ethers_signers::{LocalWallet, Signer};
use std::marker::PhantomData;
use std::time::Duration;
use tracing::{debug, info};
use typed_builder::TypedBuilder;

pub struct TxManager<P> {
    _marker: PhantomData<P>,
    config: TxManagerConfig,
    chain_id: U64,
    wallet: LocalWallet,
    is_1559_tx: bool,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct TxBuilder {
    config: TxManagerConfig,
    chain_id: U64,
    wallet: LocalWallet,
}

impl TxBuilder {
    fn is_force_chain(&self) -> bool {
        self.config.force_gas_price_chains.contains(&self.chain_id)
    }

    pub async fn build_tx<P: JsonRpcClient>(&self, provider: &Provider<P>) -> TxManager<P> {
        debug!("build tx manager");
        let mut is_1559_tx = true;
        if self.is_force_chain() {
            debug!("force chain, use legacy tx");
            is_1559_tx = false;
        } else {
            debug!("try to use 1559 tx");
            let gas_oracle = ProviderOracle::new(provider);
            let base = gas_oracle.estimate_eip1559_fees().await;
            if base.is_err() {
                is_1559_tx = false
            }
        }

        TxManager {
            _marker: Default::default(),
            config: self.config.clone(),
            chain_id: self.chain_id,
            wallet: self.wallet.clone(),
            is_1559_tx,
        }
    }
}

impl<P> TxManager<P>
where
    P: JsonRpcClient + Send + Sync,
{
    async fn gas_price_1559_tx(&self, provider: &Provider<P>) -> Result<(U256, U256)> {
        debug!("gas price 1559 tx");
        let gas_oracle = ProviderOracle::new(provider);

        let (max_fee_per_gas, mut priority_fee) = gas_oracle
            .estimate_eip1559_fees()
            .await
            .map_err(|e| TxManagerError::GasPriceError(e.to_string()))?;
        if priority_fee < self.config.min_priority_fee_per_gas {
            priority_fee = self.config.min_priority_fee_per_gas;
        }
        Ok((max_fee_per_gas, priority_fee))
    }

    async fn gas_price_legacy_tx(&self, provider: &Provider<P>) -> Result<U256> {
        debug!("gas price legacy tx");
        let gas_oracle = ProviderOracle::new(provider);

        gas_oracle
            .fetch()
            .await
            .map_err(|e| TxManagerError::GasPriceError(e.to_string()))
    }

    pub async fn gas_price(&self, provider: &Provider<P>) -> Result<U256> {
        debug!("gas price");
        if self.is_1559_tx {
            let (max_fee_per_gas, priority_fee) = self.gas_price_1559_tx(provider).await?;
            Ok(max_fee_per_gas + priority_fee)
        } else {
            self.gas_price_legacy_tx(provider).await
        }
    }

    pub async fn estimate_gas(&self, to: Address, data: &[u8], value: &U256, provider: &Provider<P>) -> Result<U256> {
        debug!("estimate gas");
        let typed_tx = match self.is_1559_tx {
            true => {
                let max_fee_per_gas = self.choose_max_gas_price(None);
                let priority_fee = self.config.min_priority_fee_per_gas;

                let tx = self
                    .build_1559_tx(to, data, value, &max_fee_per_gas, &priority_fee, provider)
                    .await?;
                TypedTransaction::try_from(tx).expect("Failed to convert Eip1559TransactionRequest to TypedTransaction")
            }
            false => {
                let gas_price = self.gas_price_legacy_tx(provider).await?;
                let tx = self.build_legacy_tx(to, data, value, &gas_price, provider).await?;
                TypedTransaction::try_from(tx).expect("Failed to convert TransactionRequest to TypedTransaction")
            }
        };

        let signer = SignerMiddleware::new(provider, self.wallet.clone());
        signer
            .estimate_gas(&typed_tx, None)
            .await
            .map_err(|why| TxManagerError::EstimateGasError(why.to_string()))
    }

    pub async fn send(
        &self,
        to: Address,
        data: &[u8],
        value: &U256,
        gas_limit: &U256,
        tx_max_gas_price: Option<U256>,
        provider: &Provider<P>,
    ) -> Result<TxHash> {
        info!(
            "send tx to {:?} with gas_limit {:?} and max_gas_price {:?}",
            to, gas_limit, tx_max_gas_price
        );

        let gas_limit = gas_limit * (100 + self.config.gas_limit_reserve_percentage) / 100;
        if self.is_1559_tx {
            let (max_fee_per_gas, priority_fee) = self.gas_price_1559_tx(provider).await?;
            let max_gas_price = self.choose_max_gas_price(tx_max_gas_price);
            if max_fee_per_gas + priority_fee > max_gas_price {
                return Err(TxManagerError::GasPriceError("gas price too high".into()));
            }
            let max_fee_per_gas = max_gas_price - priority_fee;
            let mut tx_request = self
                .build_1559_tx(to, data, value, &max_fee_per_gas, &priority_fee, provider)
                .await?;
            tx_request.gas = Some(gas_limit);
            self.send_1559_tx(tx_request, provider).await
        } else {
            let gas_price = self.gas_price_legacy_tx(provider).await?;
            let max_gas_price = self.choose_max_gas_price(tx_max_gas_price);
            if gas_price > max_gas_price {
                return Err(TxManagerError::GasPriceError("gas price too high".into()));
            }
            let mut tx_request = self.build_legacy_tx(to, data, value, &gas_price, provider).await?;
            tx_request.gas = Some(gas_limit);
            self.send_legacy_tx(tx_request, provider).await
        }
    }

    pub async fn confirm(&self, provider: &Provider<P>, tx_hash: TxHash) -> Result<TransactionReceipt> {
        info!("confirm tx {:?}", tx_hash);
        for _ in 0..self.config.max_confirm_count {
            tokio::time::sleep(Duration::from_secs(self.config.confirm_interval_secs)).await;

            let tx = provider
                .get_transaction(tx_hash)
                .await
                .map_err(|why| TxManagerError::ConfirmTxError(why.to_string()))?
                .ok_or_else(|| TxManagerError::TxDropped)?;

            if let Some(block_number) = tx.block_number {
                let current_block_number = provider
                    .get_block_number()
                    .await
                    .map_err(|why| TxManagerError::ConfirmTxError(why.to_string()))?;
                if current_block_number - block_number < self.config.confirm_blocks {
                    info!("waiting for tx to be confirmed");
                    continue;
                }
            } else {
                continue;
            }

            let receipt = provider
                .get_transaction_receipt(tx_hash)
                .await
                .map_err(|why| TxManagerError::ConfirmTxError(why.to_string()))?;

            if let Some(receipt) = receipt {
                if receipt.status != Some(U64::from(1)) {
                    return Err(TxManagerError::ConfirmTxError(format!("failed: {:?}", receipt)));
                }
                return Ok(receipt);
            }
        }

        Err(TxManagerError::ConfirmTxError("reach max confirm count".into()))
    }

    async fn build_legacy_tx(
        &self,
        to: Address,
        data: &[u8],
        value: &U256,
        gas_price: &U256,
        provider: &Provider<P>,
    ) -> Result<TransactionRequest> {
        let curr_nonce = self.get_current_nonce(provider).await?;

        Ok(TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(ethers_core::types::NameOrAddress::Address(to))
            .value(value)
            .data(data.to_vec())
            .nonce(curr_nonce)
            .gas_price(*gas_price))
    }

    async fn send_legacy_tx(&self, tx_request: TransactionRequest, provider: &Provider<P>) -> Result<TxHash> {
        let signer = SignerMiddleware::new(provider, self.wallet.clone());
        // todo support gas escalator
        // let geometric_escalator = GeometricGasPrice::new(
        //     // self.config.gas_price_coefficient,
        //     // self.config.gas_price_every_secs,
        //     5.0,
        //     10u64,
        //     Some(self.choose_max_gas_price()),
        // );
        //
        // let gas_escalator = GasEscalatorMiddleware::new(
        //     signer,
        //     geometric_escalator,
        //     // Frequency::Duration(self.config.bump_gas_price_ms),
        //     Frequency::Duration(300),
        // );

        let pending_tx = signer
            .send_transaction(tx_request, None)
            .await
            .map_err(|why| TxManagerError::SendTxError(why.to_string()))?;

        Ok(pending_tx.tx_hash())
    }

    async fn build_1559_tx(
        &self,
        to: Address,
        data: &[u8],
        value: &U256,
        max_fee_per_gas: &U256,
        priority_fee: &U256,
        provider: &Provider<P>,
    ) -> Result<Eip1559TransactionRequest> {
        let curr_nonce = self.get_current_nonce(provider).await?;

        // todo set priority_fee_per_gas from provider
        Ok(Eip1559TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(ethers_core::types::NameOrAddress::Address(to))
            .value(value)
            .data(data.to_vec())
            .nonce(curr_nonce)
            .max_fee_per_gas(*max_fee_per_gas)
            .max_priority_fee_per_gas(*priority_fee))
    }

    async fn send_1559_tx(&self, tx_request: Eip1559TransactionRequest, provider: &Provider<P>) -> Result<TxHash> {
        let signer = SignerMiddleware::new(provider, self.wallet.clone());
        let pending_tx = signer
            .send_transaction(tx_request, None)
            .await
            .map_err(|why| TxManagerError::SendTxError(why.to_string()))?;
        Ok(pending_tx.tx_hash())
    }

    pub fn is_1559_tx(&self) -> bool {
        self.is_1559_tx
    }

    fn choose_max_gas_price(&self, tx_max_gas_price: Option<U256>) -> U256 {
        match tx_max_gas_price {
            Some(price) => std::cmp::min(price, self.config.max_gas_price),
            None => self.config.max_gas_price,
        }
    }

    async fn get_current_nonce(&self, provider: &Provider<P>) -> Result<U256> {
        debug!("get current nonce");
        let nonce_manager = NonceManagerMiddleware::new(provider, self.wallet.address());
        nonce_manager
            .get_transaction_count(self.wallet.address(), Some(BlockNumber::Pending.into()))
            .await
            .map_err(|why| TxManagerError::NonceError(why.to_string()))
    }
}
