use crate::context::mock_context::{create_mock_context, get_pool_contracts, MockContext};
use mystiko_roller::context::ContextTrait;
use mystiko_roller::data::handler::DataHandler;
use mystiko_roller::pull::handler::PullHandle;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod pull_scenario_tests;
pub mod pull_tests;

pub async fn create_pull_handle(
    chain_id: u64,
    indexer_port: u16,
    token_price_port: u16,
    disable_indexer: bool,
) -> (PullHandle, Arc<RwLock<DataHandler>>, Arc<MockContext>) {
    let mut c = create_mock_context(indexer_port, token_price_port).await;
    if disable_indexer {
        c.disable_indexer();
    }

    let c = Arc::new(c);
    let pool_contract = get_pool_contracts(&c);

    let context_trait: Arc<dyn ContextTrait + Send> = Arc::clone(&c) as Arc<dyn ContextTrait + Send>;
    let data = DataHandler::new(chain_id, &pool_contract, context_trait).await;
    let data_rc = Arc::new(RwLock::new(data));
    let context_trait2: Arc<dyn ContextTrait + Send> = Arc::clone(&c) as Arc<dyn ContextTrait + Send>;
    let mut handle = PullHandle::new(pool_contract.address(), context_trait2, Arc::clone(&data_rc));
    handle.chain_id = chain_id;
    (handle, data_rc, c)
}
