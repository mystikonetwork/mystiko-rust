pub mod consumer;
pub mod producer;

use crate::channel::producer::TransactionProducer;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TransactSendersMapKey {
    pub chain_id: u64,
    pub private_key: String,
}

pub type TransactSendersMap = HashMap<TransactSendersMapKey, TransactionProducer>;

pub mod transact_channel {
    use crate::channel::consumer::TransactionConsumer;
    use crate::channel::producer::TransactionProducer;
    use crate::channel::{TransactSendersMap, TransactSendersMapKey};
    use crate::configs::ServerConfig;
    use crate::handler::transaction::TransactionHandler;
    use anyhow::{bail, Result};
    use ethers_signers::{LocalWallet, Signer};
    use mystiko_ethers::provider::pool::ProviderPool;
    use mystiko_relayer_config::wrapper::relayer::RelayerConfig;
    use mystiko_relayer_types::TransactRequestData;
    use mystiko_server_utils::token_price::price::TokenPrice;
    use mystiko_server_utils::tx_manager::config::TxManagerConfig;
    use mystiko_server_utils::tx_manager::transaction::TxBuilder;
    use mystiko_storage::formatter::sql::SqlStatementFormatter;
    use mystiko_storage_sqlite::SqliteStorage;
    use mystiko_types::AssetType;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::mpsc::channel;
    use tokio::sync::RwLock;

    pub async fn init(
        server_config: &ServerConfig,
        relayer_config: &RelayerConfig,
        providers: Arc<RwLock<ProviderPool>>,
        handler: Arc<TransactionHandler<SqlStatementFormatter, SqliteStorage>>,
        token_price: Arc<RwLock<TokenPrice>>,
        queue_capacity: usize,
    ) -> Result<(TransactSendersMap, Vec<TransactionConsumer>)> {
        let mut transact_senders_map = HashMap::new();
        let mut consumers = Vec::new();
        for account in server_config.accounts.iter() {
            let chain_id = account.chain_id;
            let pk = &account.private_key;
            let (sender, receiver) = channel::<(String, TransactRequestData)>(queue_capacity);
            transact_senders_map.insert(
                TransactSendersMapKey {
                    chain_id,
                    private_key: pk.to_string(),
                },
                TransactionProducer::new(
                    account.supported_erc20_tokens.clone(),
                    Arc::new(sender),
                    handler.clone(),
                ),
            );
            let wallet: LocalWallet = pk.parse::<LocalWallet>()?.with_chain_id(chain_id);

            // create tx manager config
            let tx_manager_config = TxManagerConfig::new(
                serde_json::to_string(&server_config.settings.network_type)?.as_str(),
                None,
            )?;
            // create tx builder
            let tx_builder = TxBuilder::builder()
                .config(tx_manager_config)
                .chain_id(chain_id.into())
                .wallet(wallet)
                .build();
            // get or create provider
            let mut pool = providers.write().await;
            let provider = pool.get_or_create_provider(chain_id).await?;
            drop(pool);
            // build tx manager
            let tx_manager = tx_builder.build_tx(&provider).await;

            if let Some(chain_config) = relayer_config.find_chain_config(chain_id) {
                consumers.push(TransactionConsumer {
                    chain_id,
                    main_asset_symbol: String::from(chain_config.asset_symbol()),
                    main_asset_decimals: chain_config.asset_decimals(),
                    receiver,
                    providers: providers.clone(),
                    handler: handler.clone(),
                    token_price: token_price.clone(),
                    tx_manager,
                });
            } else {
                bail!("chain id {} config not found in relayer config", chain_id)
            }
        }

        Ok((transact_senders_map, consumers))
    }

    pub fn find_producer_by_id_and_symbol(
        senders: &TransactSendersMap,
        chain_id: u64,
        asset_symbol: &str,
        asset_type: AssetType,
    ) -> Option<TransactionProducer> {
        let matching_producers = senders
            .iter()
            .filter(|(key, value)| {
                if key.chain_id != chain_id {
                    return false;
                }
                if asset_type == AssetType::Main {
                    return true;
                }
                let contains = value
                    .supported_erc20_tokens
                    .iter()
                    .map(|symbol| symbol.to_lowercase())
                    .any(|symbol| symbol == asset_symbol.to_lowercase());
                contains
            })
            .map(|(_, value)| value.clone())
            .collect::<Vec<_>>();

        // Select one at random and return
        let mut rng = thread_rng();
        if let Some(sender) = matching_producers.choose(&mut rng) {
            return Some(sender.clone());
        }

        None
    }
}
