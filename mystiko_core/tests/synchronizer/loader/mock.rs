use async_trait::async_trait;
use mockall::mock;
use mystiko_config::ContractConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::LoadedData;
use mystiko_dataloader::handler::{
    CommitmentQueryOption, DataHandler, HandleOption, HandleResult, NullifierQueryOption, QueryResult,
    Result as HandleQueryResult,
};
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
        async fn query_loading_contracts(&self, _chain_id: u64) -> HandleQueryResult<Option<Vec<ContractConfig>>>;
        async fn query_chain_loaded_block(&self, chain_id: u64) -> HandleQueryResult<Option<u64>>;
        async fn query_contract_loaded_block(
            &self,
            chain_id: u64,
            contract_address: &str,
        ) -> HandleQueryResult<Option<u64>>;
        async fn query_commitment(&self, option: &CommitmentQueryOption) -> HandleQueryResult<Option<Commitment>>;
        async fn query_commitments(
            &self,
            option: &CommitmentQueryOption,
        ) -> HandleQueryResult<QueryResult<Vec<Commitment>>>;
        async fn count_commitments(&self, option: &CommitmentQueryOption) -> HandleQueryResult<QueryResult<u64>>;
        async fn query_nullifier(&self, option: &NullifierQueryOption) -> HandleQueryResult<Option<Nullifier>>;
        async fn query_nullifiers(&self, option: &NullifierQueryOption) -> HandleQueryResult<QueryResult<Vec<Nullifier>>>;
        async fn count_nullifiers(&self, option: &NullifierQueryOption) -> HandleQueryResult<QueryResult<u64>>;
        async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> HandleResult;
    }
}
