use crate::common::env::load_roller_private_key;
use crate::common::error::{Result, RollerError};
use crate::config::settings::{create_tx_manager_config, RollupConfig};
use crate::context::ContextTrait;
use crate::core::slice::SlicePattern;
use crate::data::data::{DataHandle, ProofInfo, RollupPlan};
use ethers_core::types::{Address, Bytes, U256};
use ethers_signers::{LocalWallet, Signer};
use mystiko_abi::commitment_pool::{CommitmentPool, RollupRequest};
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::wrapper::{JsonRpcClientWrapper, ProviderWrapper};
use mystiko_server_utils::tx_manager::transaction::{TxBuilder, TxManager};
use mystiko_utils::convert::big_int_to_u256;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, error, info};

pub struct RollupHandle {
    chain_id: u64,
    pool_contract_cfg: PoolContractConfig,
    pool_contract: CommitmentPool<Provider>,
    cfg: RollupConfig,
    context: Arc<dyn ContextTrait>,
    data: Rc<RefCell<DataHandle>>,
    tx: TxManager<ProviderWrapper<Box<dyn JsonRpcClientWrapper>>>,
    to_address: Address,
}

impl RollupHandle {
    pub async fn new(
        pool_contract_cfg: &PoolContractConfig,
        context: Arc<dyn ContextTrait>,
        data: Rc<RefCell<DataHandle>>,
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
            pool_contract,
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

    pub async fn build_rollup_transaction_param(&self, size: usize, proof_info: &ProofInfo) -> Result<Bytes> {
        let request = RollupRequest {
            proof: proof_info.r.zk_proof.proof.convert_to()?,
            rollup_size: size as u32,
            new_root: big_int_to_u256(&proof_info.r.new_root),
            leaf_hash: big_int_to_u256(&proof_info.r.leaves_hash),
        };

        let call = self.pool_contract.rollup(request);
        let call_data = call
            .calldata()
            .ok_or(RollerError::InvalidCallData("rollup data".to_string()))?;
        Ok(call_data)
    }

    async fn send_rollup_transaction(&mut self, plan: &RollupPlan, proof_info: &ProofInfo) -> Result<()> {
        info!("send rollup transaction");
        let provider = self.context.sign_provider().await;
        let tx_data = self.build_rollup_transaction_param(plan.sizes[0], proof_info).await?;
        if plan.force {
            let tx_hash = self
                .tx
                .send(self.to_address, tx_data.as_slice(), U256::zero(), None, &provider)
                .await?;
            info!("force send rollup transaction hash: {:?}", tx_hash);
        } else {
            let max_gas_price = self.calc_tx_max_gas_price(&plan).await?;
            info!("send rollup transaction with max gas price {:?}", max_gas_price);
            let tx_hash = self
                .tx
                .send(
                    self.to_address,
                    tx_data.as_slice(),
                    U256::zero(),
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
        let mut data = self.data.borrow_mut();
        let force_rollup_block_count = self.context.cfg().rollup.force_rollup_block_count;
        let plan = data.generate_plan(included_count as usize, force_rollup_block_count)?;
        info!("rollup plan {:?}", plan);
        let proof = data.generate_proof(&plan).await?;
        Ok((plan, proof))
    }

    async fn do_rollup(&mut self, included_count: u32) -> Result<()> {
        let queue_count = self.data.borrow().get_commitments_queue_count();
        if queue_count < included_count as usize {
            error!("commitment slow {:?}, included: {:?}", queue_count, included_count);
            return Err(RollerError::CommitmentQueueSlow);
        } else if queue_count == included_count as usize {
            debug!("queue len same with included count");
            return Ok(());
        }

        info!("do rollup {:?}", included_count);
        let (plan, proof) = self.build_rollup_plan(included_count).await?;
        self.send_rollup_transaction(&plan, &proof).await
    }

    async fn get_included_count_from_indexer(&self) -> Result<u32> {
        debug!("get included count from indexer");

        let indexer = self.context.indexer().await.ok_or(RollerError::NoIndexer)?;
        let next_block_number = self.data.borrow().get_next_sync_block();
        let include_count = indexer
            .get_commitment_included_count(self.chain_id, &self.pool_contract_cfg.address(), next_block_number - 1)
            .await?;
        Ok(include_count)
    }

    async fn get_included_count_from_provider(&mut self) -> Result<u32> {
        info!("get included count from provider");

        let address = Address::from_str(self.pool_contract_cfg.address()).expect("invalid contract address");
        let sign_provider = self.context.sign_provider().await;
        let pool = CommitmentPool::new(address, sign_provider);
        let count = pool
            .get_commitment_included_count()
            .call()
            .await
            .map_err(|e| RollerError::RpcCallError(e.to_string()))?;
        let count = count.as_u32();
        Ok(count)
    }

    pub async fn rollup(&mut self) -> Result<()> {
        debug!("rollup");
        let indexer_count = self.get_included_count_from_indexer().await;
        if indexer_count.is_err() {
            error!(
                "get included count from indexer failed {:?}, rollup by provider",
                indexer_count
            );
            let included_count = self.get_included_count_from_provider().await?;
            self.do_rollup(included_count).await
        } else {
            let result = self.do_rollup(indexer_count.unwrap()).await;
            if result.is_err() {
                error!("rollup by indexer failed {:?}, rollup by provider", result);
                let included_count = self.get_included_count_from_provider().await?;
                self.do_rollup(included_count).await
            } else {
                result
            }
        }
    }
}
