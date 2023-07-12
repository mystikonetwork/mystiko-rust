use crate::handler::transaction::{TransactionHandler, UpdateTransactionOptions};
use anyhow::{bail, Result};
use core::slice::SlicePattern;
use ethers_core::abi::{AbiEncode, Address};
use ethers_core::types::{Bytes, TxHash, U256};
use log::{debug, error, info};
use mystiko_abi::commitment_pool::{CommitmentPool, TransactRequest};
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::pool::ProviderPool;
use mystiko_ethers::provider::wrapper::{JsonRpcClientWrapper, ProviderWrapper};
use mystiko_relayer_types::{TransactRequestData, TransactStatus};
use mystiko_server_utils::token_price::price::TokenPrice;
use mystiko_server_utils::tx_manager::transaction::TxManager;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use std::ops::{Add, Mul};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;
use tokio::time::sleep;

pub struct TransactionConsumer {
    pub chain_id: u64,
    pub main_asset_symbol: String,
    pub main_asset_decimals: u32,
    pub receiver: Receiver<(String, TransactRequestData)>,
    pub providers: Arc<RwLock<ProviderPool>>,
    pub handler: Arc<TransactionHandler<SqlStatementFormatter, SqliteStorage>>,
    pub token_price: Arc<RwLock<TokenPrice>>,
    pub tx_manager: TxManager<ProviderWrapper<Box<dyn JsonRpcClientWrapper>>>,
}

impl TransactionConsumer {
    pub async fn run(mut self) {
        let chain_id = self.chain_id;
        info!("Launching a consumer for chain_id: {}", chain_id);
        loop {
            if let Some((id, data)) = self.receiver.recv().await {
                info!(
                    "consumer receive a transaction(id = {}, chain_id = {}, transaction_type = {:?})",
                    id, self.chain_id, data.transaction_type
                );

                let options = match self.consume(id.as_str(), &data).await {
                    Ok(tx_hash) => UpdateTransactionOptions::builder()
                        .status(TransactStatus::Succeeded)
                        .transaction_hash(tx_hash)
                        .build(),
                    Err(err) => {
                        error!("consume transaction error: {}", err);
                        UpdateTransactionOptions::builder()
                            .status(TransactStatus::Failed)
                            .error_message(err.to_string())
                            .build()
                    }
                };

                // update database
                self.update_transaction_status(id.as_str(), options).await;
            }
        }
    }

    async fn consume(&mut self, uuid: &str, data: &TransactRequestData) -> Result<String> {
        // get provider
        let provider = self.get_provider(self.chain_id).await?;
        // parse address to Address
        let contract_address = Address::from_str(&data.pool_address)?;
        // build call data
        let call_data = self
            .build_call_data(contract_address, &provider, &data.contract_param, &data.signature)
            .await?;
        // estimate gas
        let estimate_gas = self.estimate_gas(contract_address, &call_data, &provider).await?;
        // validate relayer fee
        self.validate_relayer_fee(&provider, data, &estimate_gas).await?;
        // send transaction
        let tx_hash = self
            .send(contract_address, &call_data, &provider, &estimate_gas)
            .await?;

        // update transaction status to pending
        self.update_transaction_status(
            uuid,
            UpdateTransactionOptions::builder()
                .status(TransactStatus::Pending)
                .transaction_hash(tx_hash.clone())
                .build(),
        )
        .await;

        // wait transaction until confirmed
        info!("Wait for the transaction(hash={}) to be confirmed", tx_hash.as_str());
        self.wait_confirm(&provider, &tx_hash).await
    }

    async fn validate_relayer_fee(
        &mut self,
        provider: &Arc<Provider>,
        data: &TransactRequestData,
        estimate_gas: &U256,
    ) -> Result<()> {
        let out_rollup_fees = &data.contract_param.out_rollup_fees;
        let relayer_fee_amount = &data.contract_param.relayer_fee_amount;
        let asset_symbol = &data.asset_symbol;
        let asset_decimals = data.asset_decimals;

        debug!("out rollup fees {:?}", out_rollup_fees);
        debug!("relayer fee amount {:?}", relayer_fee_amount);

        let gas_price = self.tx_manager.gas_price(provider).await?;
        debug!("chain id {} gas price {}", self.chain_id, gas_price);
        let estimate_transaction_fee = gas_price.mul(estimate_gas);
        debug!("estimate transaction fee {}", estimate_transaction_fee);

        // swap estimate gas to asset symbol
        let mut price_service = self.token_price.write().await;
        debug!(
            "main asset symbol {} decimals {} asset symbol {} decimals {}",
            self.main_asset_symbol, self.main_asset_decimals, asset_symbol, asset_decimals
        );
        let estimate_symbol_transaction_fee = price_service
            .swap(
                self.main_asset_symbol.as_str(),
                self.main_asset_decimals,
                estimate_transaction_fee,
                asset_symbol,
                asset_decimals,
            )
            .await?;
        drop(price_service);
        debug!(
            "swap asset symbol {} amount {} to symbol {} amount {}",
            self.main_asset_symbol, estimate_transaction_fee, asset_symbol, estimate_symbol_transaction_fee
        );

        // relayer_fee_amount - estimate_gas - out_rollup_fees > 0
        let mut total_out_rollup_fee = U256::zero();
        for fee in out_rollup_fees {
            total_out_rollup_fee = total_out_rollup_fee.add(fee);
        }
        debug!("total rollup fee {}", total_out_rollup_fee);

        if relayer_fee_amount.lt(&estimate_symbol_transaction_fee) {
            bail!(
                "Relayer fee amount not enough(asset_symbol:{},asset_decimals:{},relayer_fee_amount:{},\
                total_out_rollup_fee:{},estimate_gas:{},gas_price:{},estimate_transaction_fee:{}(symbol:{} amount:{})",
                asset_symbol,
                asset_decimals,
                relayer_fee_amount,
                total_out_rollup_fee,
                estimate_gas,
                gas_price,
                estimate_transaction_fee,
                asset_symbol,
                estimate_symbol_transaction_fee,
            );
        }

        info!(
            "validate relayer fee amount successful: asset_symbol:{},asset_decimals:{},relayer_fee_amount:{},\
            total_out_rollup_fee:{},estimate_gas:{},gas_price:{},estimate_transaction_fee:{}(symbol:{} amount:{})",
            asset_symbol,
            asset_decimals,
            relayer_fee_amount,
            total_out_rollup_fee,
            estimate_gas,
            gas_price,
            estimate_transaction_fee,
            asset_symbol,
            estimate_symbol_transaction_fee,
        );

        Ok(())
    }

    async fn send(
        &mut self,
        contract_address: Address,
        call_data: &Bytes,
        provider: &Arc<Provider>,
        gas_limit: &U256,
    ) -> Result<String> {
        let tx_hash = self
            .tx_manager
            .send(
                contract_address,
                call_data.as_slice(),
                &U256::zero(),
                gas_limit,
                None,
                provider,
            )
            .await?
            .encode_hex();

        Ok(tx_hash)
    }

    async fn wait_confirm(&self, provider: &Arc<Provider>, tx_hash: &str) -> Result<String> {
        let tx_hash = TxHash::from_str(tx_hash)?;
        info!("wait transaction hash {:?} until confirmed", tx_hash);
        let receipt = self.tx_manager.confirm(provider, tx_hash).await?;
        Ok(receipt.transaction_hash.encode_hex())
    }

    async fn get_provider(&mut self, chain_id: u64) -> Result<Arc<Provider>> {
        let mut pool = self.providers.write().await;
        let provider = pool.get_or_create_provider(chain_id).await?;
        drop(pool);

        Ok(provider)
    }

    async fn build_call_data(
        &self,
        contract_address: Address,
        provider: &Arc<Provider>,
        data: &TransactRequest,
        signature: &str,
    ) -> Result<Bytes> {
        let contract = CommitmentPool::new(contract_address, provider.clone());
        let call_data = contract.transact(data.clone(), Bytes::from_str(signature)?).calldata();
        match call_data {
            None => {
                bail!("Invalid call data")
            }
            Some(result) => Ok(result),
        }
    }

    async fn update_transaction_status(&self, uuid: &str, options: UpdateTransactionOptions) {
        let mut retry_count = 0;
        let max_retry_count = 5;
        loop {
            if let Err(err) = self.handler.update_by_id(uuid, &options).await {
                error!(
                    "Failed to update transaction(id = {}) to status {:?}: {}",
                    uuid, &options.status, err
                );

                if retry_count >= max_retry_count {
                    error!(
                        "Exceeded maximum retry count. Failed to update transaction(id = {}) to status {:?}",
                        uuid, &options.status
                    );
                    break;
                }

                retry_count += 1;
                let wait_duration = Duration::from_secs(2);
                sleep(wait_duration).await;
                continue;
            } else {
                info!(
                    "Successfully update transaction(id = {}) to status {:?}",
                    uuid, &options.status
                );
                break;
            }
        }
    }

    async fn estimate_gas(
        &mut self,
        contract_address: Address,
        call_data: &Bytes,
        provider: &Arc<Provider>,
    ) -> Result<U256> {
        debug!("estimate gas for contract_address: {:?}", contract_address);
        let estimate_gas = self
            .tx_manager
            .estimate_gas(contract_address, call_data.as_slice(), &U256::zero(), provider)
            .await?;
        debug!("estimate gas successful: {}", estimate_gas);
        Ok(estimate_gas)
    }
}
