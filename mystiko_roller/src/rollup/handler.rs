use crate::chain::ChainDataGiver;
use crate::common::env::load_roller_private_key;
use crate::common::error::{Result, RollerError};
use crate::config::roller::{ChainDataSource, RollupConfig};
use crate::context::ContextTrait;
use crate::data::handler::{DataHandler, ProofInfo, RollupPlan};
use crate::rollup::static_data::{STATIC_ERROR_INVALID_LEAF_HASH, STATIC_ERROR_INVALID_ROLLUP_SIZE, STATIC_PROOF_DATA};
use ethers_core::types::{Address, Bytes, H256, U256};
use ethers_signers::{LocalWallet, Signer};
use mystiko_abi::commitment_pool::{CommitmentPool, RollupRequest};
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::wrapper::{JsonRpcClientWrapper, ProviderWrapper};
use mystiko_server_utils::tx_manager::config::TxManagerConfig;
use mystiko_server_utils::tx_manager::error::TxManagerError;
use mystiko_server_utils::tx_manager::transaction::{TxBuilder, TxManager};
use mystiko_utils::convert::biguint_to_u256;
use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

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
        tx_manager_cfg: &TxManagerConfig,
        context: Arc<dyn ContextTrait + Send>,
        data: Arc<RwLock<DataHandler>>,
    ) -> Self {
        let cfg = context.cfg().rollup.clone();
        let chain_id = context.cfg().chain.chain_id;
        let to_address = Address::from_str(pool_contract_cfg.address()).expect("invalid contract address");

        let sk_str = load_roller_private_key().expect("load private key error");
        let local_wallet = LocalWallet::from_str(&sk_str)
            .expect("invalid private key")
            .with_chain_id(chain_id);
        info!("local wallet address: {:?}", local_wallet.address());
        let builder = TxBuilder::builder()
            .config(tx_manager_cfg.clone())
            .chain_id(chain_id)
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
        let mut total_gas_cost = 0;
        plan.sizes.iter().for_each(|size| {
            let cost = self.cfg.get_rollup_cost(*size);
            total_gas_cost += cost;
        });

        let core_cfg_parser = self.context.core_cfg_parser();
        let chain_config = core_cfg_parser.chain_cfg(self.chain_id);
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
            new_root: biguint_to_u256(&proof_info.r.new_root),
            leaf_hash: biguint_to_u256(&proof_info.r.leaves_hash),
        };

        let call = self.commitment_contract.rollup(request);
        let call_data = call
            .calldata()
            .ok_or(RollerError::InvalidCallData("rollup data".to_string()))?;
        Ok(call_data)
    }

    pub async fn log_transaction(&self, tx_hash: &H256, include_count: usize, rollup_size: usize) {
        self.data
            .read()
            .await
            .get_batch_commitments(include_count, rollup_size)
            .iter()
            .for_each(|cm| {
                info!(
                    "rollup commitment 0x{:x} from deposit tx {:?} with {}",
                    cm.cm, cm.deposit_tx, tx_hash
                )
            });
    }

    // todo check if tx meet revert error when run two roller at the same time
    pub async fn send_rollup_transaction(&self, plan: &RollupPlan, proof_info: &ProofInfo) -> Result<H256> {
        info!("send rollup transaction");
        let signer = self.context.signer();
        let tx_data = self.build_rollup_transaction_param(plan.sizes[0], proof_info).await?;
        let max_gas_price = self.calc_tx_max_gas_price(plan).await?;
        let gas_price: U256 = if max_gas_price > self.cfg.max_gas_price.into() || !plan.force {
            max_gas_price
        } else {
            self.cfg.max_gas_price.into()
        };

        let gas_limit = self
            .tx
            .estimate_gas(
                self.to_address,
                tx_data.to_vec().as_slice(),
                &U256::zero(),
                &gas_price,
                &signer,
            )
            .await?;
        info!("rollup transaction gas limit: {:?}", gas_limit);

        info!("send rollup transaction with gas price {:?}", gas_price);
        let tx_hash = self
            .tx
            .send(
                self.to_address,
                tx_data.to_vec().as_slice(),
                &U256::zero(),
                &gas_limit,
                &gas_price,
                &signer,
            )
            .await?;

        info!("send rollup transaction hash: {:?}", tx_hash.to_string());
        let receipt = self.tx.confirm(&signer, tx_hash).await?;
        if let Some(block_number) = receipt.block_number {
            self.data
                .write()
                .await
                .set_latest_rollup_tx_block_number(block_number.as_u64());
        } else {
            error!("receipt block number is none");
        }

        info!("rollup transaction have been confirmed");
        Ok(receipt.transaction_hash)
    }

    async fn build_rollup_plan(&self, included_count: usize) -> Result<(RollupPlan, ProofInfo)> {
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

    async fn do_rollup(&self, data_source: &ChainDataSource, included_count: usize) -> Result<()> {
        let queue_count = self.data.read().await.get_commitments_queue_count();
        match queue_count.cmp(&(included_count)) {
            Ordering::Equal => Ok(()),
            Ordering::Less => {
                error!(
                    "commitment queue {:?} slow than {:?} included count: {:?}",
                    queue_count, data_source, included_count
                );
                Err(RollerError::CommitmentQueueSlow)
            }
            Ordering::Greater => {
                // todo check commitment tree root match?
                info!("do rollup {:?} {:?}", data_source, included_count);
                let (plan, proof) = self.build_rollup_plan(included_count).await?;
                let tx_hash = self.send_rollup_transaction(&plan, &proof).await?;
                self.log_transaction(&tx_hash, included_count, plan.sizes[0]).await;
                Ok(())
            }
        }
    }

    pub async fn rollup<T: ChainDataGiver + ?Sized>(&self, giver: Arc<T>) -> Result<()> {
        let latest_block = giver
            .get_latest_block_number(self.chain_id, self.pool_contract_cfg.address())
            .await?;
        if latest_block <= self.data.read().await.get_latest_rollup_tx_block_number() {
            info!("giver {:?} latest {:?} block slow", giver.data_source(), latest_block);
            return Ok(());
        }

        let included_count = giver
            .get_included_count(self.chain_id, self.pool_contract_cfg.address())
            .await?;
        self.do_rollup(&giver.data_source(), included_count).await
    }

    pub async fn check_commitment_queue<T: ChainDataGiver + ?Sized>(&self, giver: Arc<T>) -> Result<()> {
        let queue_len = self.data.read().await.get_commitments_queue_count();
        let include_count = giver
            .get_included_count(self.chain_id, self.pool_contract_cfg.address())
            .await?;
        let source = &giver.data_source();
        match queue_len.cmp(&include_count) {
            Ordering::Greater => {
                self.data.write().await.reset_giver_check_counter(source, 0);
                Ok(())
            }
            Ordering::Equal => {
                self.data.write().await.inc_giver_check_counter(source);
                let counter = self.data.read().await.get_giver_check_counter(source)?;
                if counter > self.cfg.max_empty_queue_count {
                    info!("{:?} commitment check at included count {}", source, include_count);
                    if !self.commitment_queue_check_by_transaction().await? {
                        self.data.write().await.revert_next_sync_block();
                        self.data.write().await.set_latest_rollup_tx_block_number(0);
                        return Err(RollerError::CommitmentMissing);
                    } else {
                        self.data.write().await.reset_giver_check_counter(source, 0);
                    }
                }
                Ok(())
            }
            Ordering::Less => {
                panic!("commitment queue {} < included count {}", queue_len, include_count);
            }
        }
    }

    pub async fn commitment_queue_check_by_transaction(&self) -> Result<bool> {
        let signer = self.context.signer();
        let tx_data = self.build_rollup_transaction_param(1, &STATIC_PROOF_DATA).await?;
        let gas_limit = self
            .tx
            .estimate_gas(
                self.to_address,
                tx_data.to_vec().as_slice(),
                &U256::zero(),
                &self.cfg.max_gas_price.into(),
                &signer,
            )
            .await;
        match gas_limit {
            Ok(_) => panic!("must error for commitment queue check"),
            Err(err) => {
                let err_string = format!("{}", err);
                if matches!(err, TxManagerError::EstimateGasError(_)) {
                    return if err_string.contains(&*STATIC_ERROR_INVALID_ROLLUP_SIZE) {
                        Ok(true)
                    } else if err_string.contains(&*STATIC_ERROR_INVALID_LEAF_HASH) {
                        error!("commitment queue not empty, should do pull");
                        Ok(false)
                    } else {
                        error!("unexpected estimate gas error {:?}", err);
                        Err(err.into())
                    };
                }
                error!("unexpected error {:?}", err);
                Err(err.into())
            }
        }
    }
}
