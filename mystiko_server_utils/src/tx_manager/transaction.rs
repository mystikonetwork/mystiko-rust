use crate::tx_manager::config::TxManagerConfig;
use crate::tx_manager::error::TxManagerError;
use anyhow::Result;
use ethers_core::types::transaction::eip1559::Eip1559TransactionRequest;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::transaction::request::TransactionRequest;
use ethers_core::types::transaction::response::TransactionReceipt;
use ethers_core::types::{Address, BlockNumber, TxHash, H256, U256, U64};
use std::time::Duration;
// use ethers_middleware::gas_escalator::GasEscalatorMiddleware;
// use ethers_middleware::gas_escalator::{Frequency, GeometricGasPrice};
use ethers_middleware::gas_oracle::{GasOracle, ProviderOracle};
use ethers_middleware::{NonceManagerMiddleware, SignerMiddleware};
use ethers_providers::{JsonRpcClient, Middleware, Provider};
use ethers_signers::{LocalWallet, Signer};
use std::marker::PhantomData;
use typed_builder::TypedBuilder;

pub struct TxManager<P> {
    _marker: PhantomData<P>,
    config: TxManagerConfig,
    chain_id: U64,
    to: Address,
    wallet: LocalWallet,
    is_1559_tx: bool,
    data: Vec<u8>,
    value: U256,
    max_gas_price: Option<U256>,
    tx_hash: Option<TxHash>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct TxBuilder {
    config: TxManagerConfig,
    chain_id: U64,
    to: Address,
    wallet: LocalWallet,
}

impl TxBuilder {
    fn is_force_chain(&self) -> bool {
        self.config.force_gas_price_chains.contains(&self.chain_id)
    }

    pub async fn build_tx<P: JsonRpcClient>(&self, provider: &Provider<P>) -> TxManager<P> {
        let mut is_1559_tx = true;
        if self.is_force_chain() {
            is_1559_tx = false;
        } else {
            let base = provider.estimate_eip1559_fees(None).await;
            if base.is_err() {
                is_1559_tx = false
            }
        }

        TxManager {
            _marker: Default::default(),
            config: self.config.clone(),
            chain_id: self.chain_id,
            to: self.to,
            wallet: self.wallet.clone(),
            is_1559_tx,
            max_gas_price: None,
            value: U256::zero(),
            data: vec![],
            tx_hash: None,
        }
    }
}

impl<P> TxManager<P>
where
    P: JsonRpcClient + Send + Sync,
{
    pub async fn gas_price(&self, provider: &Provider<P>) -> Result<U256, TxManagerError> {
        let gas_oracle = ProviderOracle::new(provider);

        if self.is_1559_tx {
            gas_oracle
                .estimate_eip1559_fees()
                .await
                .map_err(|e| TxManagerError::GasPriceError(e.to_string()))
                .map(|(max_fee_per_gas, _)| max_fee_per_gas)
        } else {
            gas_oracle
                .fetch()
                .await
                .map_err(|e| TxManagerError::GasPriceError(e.to_string()))
        }
    }

    pub async fn estimate_gas(
        &mut self,
        data: &[u8],
        value: U256,
        provider: &Provider<P>,
    ) -> Result<U256, TxManagerError> {
        self.data = data.to_vec();
        self.value = value;

        let gas_price = match self.is_1559_tx {
            true => self.choose_max_gas_price(),
            false => self.gas_price(provider).await?,
        };

        let typed_tx = match self.is_1559_tx {
            true => {
                let tx = self.build_1559_tx(&gas_price, provider).await?;
                TypedTransaction::try_from(tx).expect("Failed to convert Eip1559TransactionRequest to TypedTransaction")
            }
            false => {
                let tx = self.build_legacy_tx(&gas_price, provider).await?;
                TypedTransaction::try_from(tx).expect("Failed to convert TransactionRequest to TypedTransaction")
            }
        };

        provider
            .estimate_gas(&typed_tx, None)
            .await
            .map_err(|why| TxManagerError::EstimateGasError(why.to_string()))
    }

    pub async fn send(
        &mut self,
        data: &[u8],
        value: U256,
        max_gas_price: Option<U256>,
        provider: &Provider<P>,
    ) -> Result<TxHash, TxManagerError> {
        self.max_gas_price = max_gas_price;
        self.data = data.to_vec();
        self.value = value;
        self.tx_hash = None;

        let current_gas_price = self.gas_price(provider).await?;
        let max_gas_price = self.choose_max_gas_price();
        if current_gas_price >= max_gas_price {
            return Err(TxManagerError::GasPriceError("gas price too high".into()));
        }

        if self.is_1559_tx {
            self.send_1559_tx(&max_gas_price, provider).await
        } else {
            self.send_legacy_tx(&current_gas_price, provider).await
        }
    }

    pub async fn confirm(&self, provider: &Provider<P>) -> Result<TransactionReceipt, TxManagerError> {
        let tx_hash = match self.tx_hash {
            Some(hash) => H256::from_slice(hash.as_bytes()),
            None => return Err(TxManagerError::ConfirmTxError("tx hash none".into())),
        };

        for _ in 0..self.config.max_confirm_count {
            tokio::time::sleep(Duration::from_secs(self.config.confirm_interval_secs)).await;

            let _ = provider
                .get_transaction(tx_hash)
                .await
                .map_err(|why| TxManagerError::ConfirmTxError(why.to_string()))?
                .ok_or_else(|| TxManagerError::TxDropped)?;

            let receipt = provider
                .get_transaction_receipt(tx_hash)
                .await
                .map_err(|why| TxManagerError::ConfirmTxError(why.to_string()))?;

            if let Some(receipt) = receipt {
                if receipt.status == Some(U64::from(0)) {
                    return Err(TxManagerError::ConfirmTxError(format!("failed: {:?}", receipt)));
                }
                return Ok(receipt);
            }
        }

        Err(TxManagerError::ConfirmTxError("reach max confirm count".into()))
    }

    async fn build_legacy_tx(
        &self,
        gas_price: &U256,
        provider: &Provider<P>,
    ) -> Result<TransactionRequest, TxManagerError> {
        let curr_nonce = self.get_current_nonce(provider).await?;

        Ok(TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(ethers_core::types::NameOrAddress::Address(self.to))
            .value(self.value)
            .data(self.data.clone())
            .nonce(curr_nonce)
            .gas_price(*gas_price))
    }

    async fn send_legacy_tx(&mut self, gas_price: &U256, provider: &Provider<P>) -> Result<TxHash, TxManagerError> {
        // Create the transaction
        let tx = self.build_legacy_tx(gas_price, provider).await?;
        let signer = SignerMiddleware::new(provider, self.wallet.clone());

        // todo support gas escalator, meet provider lifetime problem
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

        // Send the transaction
        let pending_tx = signer
            .send_transaction(tx, None)
            .await
            .map_err(|why| TxManagerError::SendTxError(why.to_string()))?;

        self.tx_hash = Some(pending_tx.tx_hash());
        Ok(pending_tx.tx_hash())
    }

    async fn build_1559_tx(
        &self,
        max_gas_price: &U256,
        provider: &Provider<P>,
    ) -> Result<Eip1559TransactionRequest, TxManagerError> {
        let curr_nonce = self.get_current_nonce(provider).await?;

        Ok(Eip1559TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(ethers_core::types::NameOrAddress::Address(self.to))
            .value(self.value)
            .data(self.data.clone())
            .nonce(curr_nonce)
            .max_fee_per_gas(*max_gas_price)
            .max_priority_fee_per_gas(self.config.max_priority_fee_per_gas))
    }

    async fn send_1559_tx(&mut self, max_gas_price: &U256, provider: &Provider<P>) -> Result<TxHash, TxManagerError> {
        // Create the transaction
        let tx = self.build_1559_tx(max_gas_price, provider).await?;
        let signer = SignerMiddleware::new(provider, self.wallet.clone());
        // Send the transaction
        let pending_tx = signer
            .send_transaction(tx, None)
            .await
            .map_err(|why| TxManagerError::SendTxError(why.to_string()))?;

        self.tx_hash = Some(pending_tx.tx_hash());
        Ok(pending_tx.tx_hash())
    }

    pub fn is_1559_tx(&self) -> bool {
        self.is_1559_tx
    }

    fn choose_max_gas_price(&self) -> U256 {
        match self.max_gas_price {
            Some(price) => std::cmp::min(price, self.config.max_gas_price),
            None => self.config.max_gas_price,
        }
    }

    async fn get_current_nonce(&self, provider: &Provider<P>) -> Result<U256, TxManagerError> {
        let nonce_manager = NonceManagerMiddleware::new(provider, self.wallet.address());
        nonce_manager
            .get_transaction_count(self.wallet.address(), Some(BlockNumber::Pending.into()))
            .await
            .map_err(|why| TxManagerError::NonceError(why.to_string()))
    }
}
