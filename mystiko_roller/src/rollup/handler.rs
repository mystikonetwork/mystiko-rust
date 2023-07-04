use crate::chain::ChainDataGiver;
use crate::common::env::load_roller_private_key;
use crate::common::error::{Result, RollerError};
use crate::config::roller::{create_tx_manager_config, RollupConfig};
use crate::context::ContextTrait;
use crate::core::slice::SlicePattern;
use crate::data::handler::{DataHandler, ProofInfo, RollupPlan};
use crate::rollup::static_data::{STATIC_ERROR_INVALID_LEAF_HASH, STATIC_ERROR_INVALID_ROLLUP_SIZE, STATIC_PROOF_DATA};
use ethers_core::types::{Address, Bytes, U256};
use ethers_signers::{LocalWallet, Signer};
use mystiko_abi::commitment_pool::{CommitmentPool, RollupRequest};
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::wrapper::{JsonRpcClientWrapper, ProviderWrapper};
use mystiko_server_utils::tx_manager::error::TxManagerError;
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
    cfg: RollupConfig,
    to_address: Address,
    commitment_contract: CommitmentPool<Provider>,
    context: Arc<dyn ContextTrait>,
    data: Arc<RwLock<DataHandler>>,
    tx: TxManager<ProviderWrapper<Box<dyn JsonRpcClientWrapper>>>,
}

impl RollupHandle {
    pub async fn new(
        pool_contract_cfg: &PoolContractConfig,
        context: Arc<dyn ContextTrait + Send>,
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
        debug!("local wallet address: {:?}", local_wallet.address());
        let builder = TxBuilder::builder()
            .config(tx_manager_cfg)
            .chain_id(chain_id.into())
            .wallet(local_wallet)
            .build();
        let signer = context.signer();
        let tx = builder.build_tx(&signer).await;
        let commitment_contract = CommitmentPool::new(to_address, signer.clone());

        RollupHandle {
            chain_id,
            pool_contract_cfg: pool_contract_cfg.clone(),
            cfg,
            commitment_contract,
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

        let call = self.commitment_contract.rollup(request);
        let call_data = call
            .calldata()
            .ok_or(RollerError::InvalidCallData("rollup data".to_string()))?;
        Ok(call_data)
    }

    async fn log_rollup_transaction(&self, tx_hash: &str, include_count: usize, rollup_size: usize) {
        self.data
            .read()
            .await
            .get_batch_commitments(include_count, rollup_size)
            .iter()
            .for_each(|cm| info!("rollup commitment {:?} in transaction {:?}", cm.hash, tx_hash));
    }

    pub async fn send_rollup_transaction(&self, plan: &RollupPlan, proof_info: &ProofInfo) -> Result<String> {
        info!("send rollup transaction");
        let signer = self.context.signer();
        let tx_data = self.build_rollup_transaction_param(plan.sizes[0], proof_info).await?;
        let gas_limit = self
            .tx
            .estimate_gas(self.to_address, tx_data.as_slice(), &U256::zero(), &signer)
            .await?;
        info!("rollup transaction gas limit: {:?}", gas_limit);
        let tx_hash = if plan.force {
            info!("force send rollup transaction");
            self.tx
                .send(
                    self.to_address,
                    tx_data.as_slice(),
                    &U256::zero(),
                    &gas_limit,
                    None,
                    &signer,
                )
                .await?
        } else {
            let max_gas_price = self.calc_tx_max_gas_price(plan).await?;
            info!("send rollup transaction with max gas price {:?}", max_gas_price);
            self.tx
                .send(
                    self.to_address,
                    tx_data.as_slice(),
                    &U256::zero(),
                    &gas_limit,
                    Some(max_gas_price),
                    &signer,
                )
                .await?
        };
        info!("send rollup transaction hash: {:?}", tx_hash.to_string());

        let receipt = self.tx.confirm(&signer, tx_hash).await?;
        if let Some(block_number) = receipt.block_number {
            self.data
                .write()
                .await
                .set_latest_rollup_block_number(block_number.as_u64());
        } else {
            error!("rollup transaction block number is none receipt {:?}", receipt);
            panic!("rollup transaction block number is none");
        }

        info!("rollup transaction have been confirmed");
        Ok(receipt.transaction_hash.to_string())
    }

    async fn build_rollup_plan(&self, included_count: usize) -> Result<(RollupPlan, ProofInfo)> {
        info!("build rollup plan");
        let force_rollup_block_count = self.context.cfg().rollup.force_rollup_block_count;
        let plan = self
            .data
            .write()
            .await
            .generate_plan(included_count, force_rollup_block_count)?;
        info!("rollup plan {:?}", plan);
        let proof = self.data.write().await.generate_proof(&plan).await?;
        Ok((plan, proof))
    }

    async fn do_rollup(&self, included_count: usize) -> Result<()> {
        let queue_count = self.data.read().await.get_commitments_queue_count();
        match queue_count.cmp(&(included_count)) {
            Ordering::Equal => {
                debug!("no queued commitment");
                Ok(())
            }
            Ordering::Less => {
                error!("commitment slow {:?}, included: {:?}", queue_count, included_count);
                Err(RollerError::CommitmentQueueSlow)
            }
            Ordering::Greater => {
                info!("do rollup {:?}", included_count);
                let (plan, proof) = self.build_rollup_plan(included_count).await?;
                let tx_hash = self.send_rollup_transaction(&plan, &proof).await?;
                self.log_rollup_transaction(&tx_hash, included_count, plan.sizes[0])
                    .await;
                Ok(())
            }
        }
    }

    async fn rollup_with_chain_data_giver<T: ChainDataGiver + ?Sized>(&self, giver: Arc<T>) -> Result<()> {
        debug!("rollup from giver {:?}", giver.data_source());
        let latest_block = giver
            .get_latest_block_number(self.chain_id, self.pool_contract_cfg.address())
            .await?;
        if latest_block <= self.data.read().await.get_latest_rollup_block_number() {
            info!("giver {:?} latest {:?} block slow", giver.data_source(), latest_block);
            return Ok(());
        }

        let included_count = giver
            .get_included_count(self.chain_id, self.pool_contract_cfg.address())
            .await?;
        self.do_rollup(included_count).await
    }

    pub async fn rollup<T: ChainDataGiver + ?Sized>(&self, giver: Arc<T>) -> Result<()> {
        debug!("rollup");
        self.rollup_with_chain_data_giver(giver).await
    }

    pub async fn commitment_queue_check_by_transaction(&self) -> Result<bool> {
        let signer = self.context.signer();
        let tx_data = self.build_rollup_transaction_param(1, &STATIC_PROOF_DATA).await?;
        let gas_limit = self
            .tx
            .estimate_gas(self.to_address, tx_data.as_slice(), &U256::zero(), &signer)
            .await;

        match gas_limit {
            Ok(_) => panic!("must error when check queue"),
            Err(err) => {
                let err_string = format!("{}", err);
                if matches!(err, TxManagerError::EstimateGasError(_)) {
                    if err_string.contains(&*STATIC_ERROR_INVALID_ROLLUP_SIZE) {
                        return Ok(true);
                    } else if err_string.contains(&*STATIC_ERROR_INVALID_LEAF_HASH) {
                        return Ok(false);
                    } else {
                        panic!("unexpected estimate gas error {:?}", err);
                    }
                }

                error!("unexpected error {:?}", err);
                Ok(true)
            }
        }
    }
}
