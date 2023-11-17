use async_trait::async_trait;
use mockall::mock;
use mystiko_config::ContractConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::LoadedData;
use mystiko_dataloader::handler::{
    CommitmentQueryOption, DataHandler, HandleOption, HandleResult as HandlerHandleResult, NullifierQueryOption,
    QueryResult, Result as HandlerResult,
};
use mystiko_dataloader::loader::ResetOptions;
use mystiko_protos::data::v1::{Commitment, Nullifier};

mock! {
    #[derive(Debug)]
    pub SyncDataHandler<R>
    where
        R: LoadedData, {}

    #[async_trait]
    impl<R> DataHandler<R> for SyncDataHandler<R>
    where
        R: LoadedData,
    {
        async fn query_loading_contracts(&self, _chain_id: u64) -> HandlerResult<Option<Vec<ContractConfig>>>;
        async fn query_chain_loaded_block(&self, chain_id: u64) -> HandlerResult<Option<u64>>;
        async fn query_contract_loaded_block(
            &self,
            chain_id: u64,
            contract_address: &str,
        ) -> HandlerResult<Option<u64>>;
        async fn query_commitment(&self, option: &CommitmentQueryOption) -> HandlerResult<Option<Commitment>>;
        async fn query_commitments(
            &self,
            option: &CommitmentQueryOption,
        ) -> HandlerResult<QueryResult<Vec<Commitment>>>;
        async fn count_commitments(&self, option: &CommitmentQueryOption) -> HandlerResult<QueryResult<u64>>;
        async fn query_nullifier(&self, option: &NullifierQueryOption) -> HandlerResult<Option<Nullifier>>;
        async fn query_nullifiers(&self, option: &NullifierQueryOption) -> HandlerResult<QueryResult<Vec<Nullifier>>>;
        async fn count_nullifiers(&self, option: &NullifierQueryOption) -> HandlerResult<QueryResult<u64>>;
        async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> HandlerHandleResult;
        async fn reset(&self, options: &ResetOptions) -> HandlerResult<()>;
    }
}
