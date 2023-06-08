use crate::common::env::load_roller_private_key;
use crate::common::error::{Result, RollerError};
use crate::config::roller::{create_tx_manager_config, RollupConfig};
use crate::context::ContextTrait;
use crate::core::slice::SlicePattern;
use crate::data::handler::{DataHandler, ProofInfo, RollupPlan};
use crate::sync::provider::SyncProvider;
use crate::sync::SyncTrait;
use ethers_core::types::{Address, Bytes, U256};
use ethers_signers::{LocalWallet, Signer};
use mystiko_abi::commitment_pool::{CommitmentPool, RollupRequest};
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::wrapper::{JsonRpcClientWrapper, ProviderWrapper};
use mystiko_server_utils::tx_manager::transaction::{TxBuilder, TxManager};
use mystiko_utils::convert::big_int_to_u256;
use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

pub struct RollupHandle {
    pub chain_id: u64,
    pub pool_contract_cfg: PoolContractConfig,
    pool: CommitmentPool<Provider>,
    cfg: RollupConfig,
    context: Arc<dyn ContextTrait>,
    data: Arc<RwLock<DataHandler>>,
    tx: TxManager<ProviderWrapper<Box<dyn JsonRpcClientWrapper>>>,
    to_address: Address,
}

impl RollupHandle {
    pub async fn new(
        pool_contract_cfg: &PoolContractConfig,
        context: Arc<dyn ContextTrait>,
        data: Arc<RwLock<DataHandler>>,
    ) -> Self {
        let cfg = context.cfg().rollup.clone();
        let chain_id = context.cfg().chain.chain_id;
        let to_address = Address::from_str(pool_contract_cfg.address()).expect("invalid contract address");

        let tx_manager_cfg = create_tx_manager_config();
        let sk_str = load_roller_private_key().expect("load private key error");
        let local_wallet = LocalWallet::from_str(&sk_str)
            .expect("invalid private key")
            .with_chain_id(chain_id);
        info!("local wallet address: {:?}", local_wallet.address());
        let builder = TxBuilder::builder()
            .config(tx_manager_cfg)
            .chain_id(chain_id.into())
            .wallet(local_wallet)
            .build();
        let provider = context.sign_provider().await;
        let tx = builder.build_tx(&provider).await;
        let pool_contract = CommitmentPool::new(to_address, provider);
        RollupHandle {
            chain_id,
            pool_contract_cfg: pool_contract_cfg.clone(),
            pool: pool_contract,
            cfg,
            context,
            data,
            tx,
            to_address,
        }
    }

    async fn calc_tx_max_gas_price(&self, plan: &RollupPlan) -> Result<U256> {
        debug!("calc tx max gas price");
        let mut total_gas_cost = 0;
        plan.sizes.iter().for_each(|size| {
            let cost = self.cfg.get_rollup_cost(*size);
            total_gas_cost += cost;
        });

        let core_cfg_parser = self.context.core_cfg_parser();
        let chain_config = core_cfg_parser.chain(self.chain_id);
        let asset_symbol = chain_config.asset_symbol().to_string();
        let asset_decimals = chain_config.asset_decimals();
        let swap_amount = self
            .context
            .token_price()
            .await
            .swap(
                self.pool_contract_cfg.asset_symbol(),
                self.pool_contract_cfg.asset_decimals(),
                plan.total_fee,
                &asset_symbol,
                asset_decimals,
            )
            .await?;
        Ok(swap_amount / total_gas_cost)
    }

    async fn build_rollup_transaction_param(&self, size: usize, proof_info: &ProofInfo) -> Result<Bytes> {
        let request = RollupRequest {
            proof: proof_info.r.zk_proof.proof.convert_to()?,
            rollup_size: size as u32,
            new_root: big_int_to_u256(&proof_info.r.new_root),
            leaf_hash: big_int_to_u256(&proof_info.r.leaves_hash),
        };

        let call = self.pool.rollup(request);
        let call_data = call
            .calldata()
            .ok_or(RollerError::InvalidCallData("rollup data".to_string()))?;
        Ok(call_data)
    }

    pub async fn send_rollup_transaction(&mut self, plan: &RollupPlan, proof_info: &ProofInfo) -> Result<()> {
        info!("send rollup transaction");
        let provider = self.context.sign_provider().await;
        let tx_data = self.build_rollup_transaction_param(plan.sizes[0], proof_info).await?;
        let gas_limit = self
            .tx
            .estimate_gas(self.to_address, tx_data.as_slice(), &U256::zero(), &provider)
            .await?;
        info!("rollup transaction gas limit: {:?}", gas_limit);
        if plan.force {
            let tx_hash = self
                .tx
                .send(
                    self.to_address,
                    tx_data.as_slice(),
                    &U256::zero(),
                    &gas_limit,
                    None,
                    &provider,
                )
                .await?;
            info!("force send rollup transaction hash: {:?}", tx_hash);
        } else {
            let max_gas_price = self.calc_tx_max_gas_price(plan).await?;
            info!("send rollup transaction with max gas price {:?}", max_gas_price);
            let tx_hash = self
                .tx
                .send(
                    self.to_address,
                    tx_data.as_slice(),
                    &U256::zero(),
                    &gas_limit,
                    Some(max_gas_price),
                    &provider,
                )
                .await?;
            info!("send rollup transaction hash: {:?}", tx_hash.to_string());
        }

        let _ = self.tx.confirm(&provider).await?;
        info!("rollup transaction have been confirmed");
        Ok(())
    }

    async fn build_rollup_plan(&self, included_count: u32) -> Result<(RollupPlan, ProofInfo)> {
        info!("build rollup plan");
        // let mut data = self.data.borrow_mut();
        let force_rollup_block_count = self.context.cfg().rollup.force_rollup_block_count;
        let plan = self
            .data
            .write()
            .await
            .generate_plan(included_count as usize, force_rollup_block_count)?;
        info!("rollup plan {:?}", plan);
        let proof = self.data.write().await.generate_proof(&plan).await?;
        Ok((plan, proof))
    }

    async fn do_rollup(&mut self, included_count: u32) -> Result<()> {
        let queue_count = self.data.read().await.get_commitments_queue_count();
        match queue_count.cmp(&(included_count as usize)) {
            Ordering::Equal => {
                info!("no commitment in queue");
                Ok(())
            }
            Ordering::Less => {
                error!("commitment slow {:?}, included: {:?}", queue_count, included_count);
                Err(RollerError::CommitmentQueueSlow)
            }
            Ordering::Greater => {
                info!("do rollup {:?}", included_count);
                let (plan, proof) = self.build_rollup_plan(included_count).await?;
                self.send_rollup_transaction(&plan, &proof).await
            }
        }
    }

    pub async fn rollup_with_indexer(&mut self) -> Result<()> {
        debug!("rollup with indexer");
        let mut included_count = 0;
        {
            if let Some(indexer) = self.context.indexer().await {
                let count = indexer
                    .get_included_count(self.chain_id, self.pool_contract_cfg.address())
                    .await;
                if count.is_err() {
                    error!("get included count from indexer failed: {:?}", included_count);
                    return Err(count.err().unwrap());
                }

                included_count = count.unwrap();
            } else {
                return Err(RollerError::NoIndexer);
            }
        }

        self.do_rollup(included_count).await
    }

    pub async fn rollup_with_provider(&mut self) -> Result<()> {
        debug!("rollup with provider");
        let provider = self.context.provider().await?;
        let sync_provider = SyncProvider::new(self.pool_contract_cfg.address(), provider);
        let included_count = sync_provider
            .get_included_count(self.chain_id, self.pool_contract_cfg.address())
            .await?;
        self.do_rollup(included_count).await
    }

    pub async fn rollup(&mut self) -> Result<()> {
        debug!("rollup");
        if self.rollup_with_indexer().await.is_err() {
            self.rollup_with_provider().await?
        }

        Ok(())
    }
}
