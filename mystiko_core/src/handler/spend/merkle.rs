use crate::handler::SpendContext;
use crate::{CommitmentPoolContractHandler, IsKnownRootOptions, PublicAssetHandler, Spends, SpendsError};
use log::info;
use mystiko_config::ContractConfig;
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_crypto::zkp::{G16Proof, ZKProver};
use mystiko_dataloader::data::LiteData;
use mystiko_dataloader::fetcher::DataFetcher;
use mystiko_dataloader::fetcher::{ChainLoadedBlockOptions, ContractFetchOptions, FetchOptions, ProviderFetcher};
use mystiko_datapacker_client::DataPackerClient;
use mystiko_ethers::Providers;
use mystiko_protos::core::handler::v1::SendSpendOptions;
use mystiko_protos::data::v1::{ChainData, MerkleTree as ProtoMerkleTree};
use mystiko_protos::loader::v1::ProviderFetcherConfig;
use mystiko_storage::{StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{biguint_to_u256, bytes_to_biguint};
use num_bigint::BigUint;
use prost::Message;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

impl<F, S, A, C, T, P, R, V, K> Spends<F, S, A, C, T, P, R, V, K>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    C: CommitmentPoolContractHandler,
    V: ZKProver<G16Proof>,
    P: Providers + 'static,
    K: DataPackerClient<ChainData, ProtoMerkleTree>,
    SpendsError: From<A::Error> + From<C::Error> + From<V::Error>,
{
    pub(crate) async fn build_merkle_tree(
        &self,
        context: &SpendContext,
        options: &SendSpendOptions,
        expect_leaf_index: u64,
    ) -> Result<MerkleTree, SpendsError> {
        match &options.raw_merkle_tree {
            Some(raw) => self.build_from_raw(context, options, raw, expect_leaf_index).await,
            None => self.build_from_packer(context, options, expect_leaf_index).await,
        }
    }

    async fn build_from_raw(
        &self,
        context: &SpendContext,
        options: &SendSpendOptions,
        raw: &[u8],
        expect_leaf_index: u64,
    ) -> Result<MerkleTree, SpendsError> {
        let decompress_data = zstd::decode_all(raw)?;
        let proto_merkle_tree = ProtoMerkleTree::decode(&*decompress_data)?;
        if (proto_merkle_tree.commitment_hashes.len() as u64) <= expect_leaf_index {
            return Err(SpendsError::RawResourceError("Invalid raw merkle tree".to_string()));
        }

        let merkle_tree = self.build_tree_from_protos(&proto_merkle_tree)?;
        self.verify_merkle_tree(context, &merkle_tree, options.query_timeout_ms)
            .await?;
        Ok(merkle_tree)
    }

    async fn build_from_packer(
        &self,
        context: &SpendContext,
        options: &SendSpendOptions,
        expect_leaf_index: u64,
    ) -> Result<MerkleTree, SpendsError> {
        let (wrap_tree, is_cache) = self.build_from_packer_or_provider(context, expect_leaf_index).await?;
        let merkle_tree = MerkleTree::new(Some(wrap_tree.leaves.clone()), None, None)?;
        self.verify_merkle_tree(context, &merkle_tree, options.query_timeout_ms)
            .await?;
        if !is_cache {
            self.cache_tree
                .update(context.chain_id, context.contract_config.address(), wrap_tree)
                .await;
        }
        Ok(merkle_tree)
    }

    async fn build_from_packer_or_provider(
        &self,
        context: &SpendContext,
        expect_leaf_index: u64,
    ) -> Result<(MerkleTreeWrapper, bool), SpendsError> {
        let (mut wrap_tree, is_cache) =
            if let (Some(wrap_tree), is_cache) = self.build_from_cache_or_packer(context, expect_leaf_index).await? {
                if wrap_tree.leaves.len() as u64 > expect_leaf_index {
                    return Ok((wrap_tree, is_cache));
                }
                (wrap_tree, is_cache)
            } else {
                (
                    MerkleTreeWrapper::builder()
                        .leaves(vec![])
                        .loaded_block_number(context.contract_config.start_block() + 1)
                        .build(),
                    false,
                )
            };

        info!(
            "Fetching commitments from provider for chain_id: {}, contract: {} start block: {}",
            context.chain_id,
            context.contract_config.address(),
            wrap_tree.loaded_block_number
        );
        let (mut cms, target_block) = self
            .fetch_from_provider(context, wrap_tree.loaded_block_number + 1)
            .await?;
        wrap_tree.leaves.append(&mut cms);
        wrap_tree.loaded_block_number = target_block;
        if (wrap_tree.leaves.len() as u64) <= expect_leaf_index {
            return Err(SpendsError::RawResourceError(
                "Insufficient commitments in fetched data".to_string(),
            ));
        }
        Ok((wrap_tree, is_cache))
    }

    async fn build_from_cache_or_packer(
        &self,
        context: &SpendContext,
        expect_leaf_index: u64,
    ) -> Result<(Option<MerkleTreeWrapper>, bool), SpendsError> {
        if let Some(wrap_tree) = self
            .cache_tree
            .get(context.chain_id, context.contract_config.address())
            .await
        {
            if wrap_tree.leaves.len() as u64 > expect_leaf_index {
                return Ok((Some(wrap_tree), true));
            }

            if let Ok(Some(fetch_tree)) = self.fetch_from_packer(context).await {
                return Ok((Some(fetch_tree), false));
            }

            Ok((Some(wrap_tree), false))
        } else {
            Ok((self.fetch_from_packer(context).await?, false))
        }
    }

    async fn fetch_from_packer(&self, context: &SpendContext) -> Result<Option<MerkleTreeWrapper>, SpendsError> {
        let address = ethers_address_from_string(context.contract_config.address())?;
        let result = self.packer.query_merkle_tree(context.chain_id, &address).await;
        if let Ok(Some(data)) = result {
            Ok(Some(
                MerkleTreeWrapper::builder()
                    .loaded_block_number(data.loaded_block_number)
                    .leaves(data.commitment_hashes.iter().map(bytes_to_biguint).collect())
                    .build(),
            ))
        } else {
            Ok(None)
        }
    }

    async fn fetch_from_provider(
        &self,
        context: &SpendContext,
        start_block: u64,
    ) -> Result<(Vec<BigUint>, u64), SpendsError> {
        let provider_fetcher = ProviderFetcher::<LiteData, P>::from_config(
            ProviderFetcherConfig::builder().build(),
            self.providers.clone(),
        );
        let target_block = provider_fetcher
            .chain_loaded_block(
                &ChainLoadedBlockOptions::builder()
                    .config(self.config.clone())
                    .chain_id(context.chain_id)
                    .build(),
            )
            .await?;
        let options = FetchOptions::builder()
            .config(self.config.clone())
            .chain_id(context.chain_id)
            .start_block(start_block)
            .target_block(target_block)
            .contract_options(vec![ContractFetchOptions::builder()
                .contract_config(ContractConfig::Pool(Arc::new(context.contract_config.clone())))
                .start_block(start_block)
                .target_block(target_block)
                .build()])
            .build();
        let result = provider_fetcher.fetch(&options).await?;
        if result.contract_results.is_empty() {
            return Err(SpendsError::FetchFromProviderError("Empty results".to_string()));
        }
        let fetch_result = provider_fetcher.fetch(&options).await?;
        let contract_result = fetch_result
            .contract_results
            .first()
            .and_then(|c| c.result.as_ref().ok()) // Handle the Result<ContractData, Error> as Option<ContractData>
            .ok_or_else(|| SpendsError::FetchFromProviderError("No contract results found".to_string()))?;
        let leaves = contract_result.data.as_ref().map_or_else(Vec::new, |data| {
            data.commitments
                .iter()
                .map(|c| bytes_to_biguint(&c.commitment_hash))
                .collect()
        });
        Ok((leaves, target_block))
    }

    fn build_tree_from_protos(&self, tree: &ProtoMerkleTree) -> Result<MerkleTree, SpendsError> {
        let leaves = tree.commitment_hashes.iter().map(bytes_to_biguint).collect::<Vec<_>>();
        let merkle_tree = MerkleTree::new(Some(leaves), None, None)?;
        Ok(merkle_tree)
    }

    async fn verify_merkle_tree(
        &self,
        context: &SpendContext,
        merkle_tree: &MerkleTree,
        query_timeout_ms: Option<u64>,
    ) -> Result<(), SpendsError> {
        let merkle_root = merkle_tree.root();
        let contract_address = ethers_address_from_string(context.contract_config.address())?;
        let options = IsKnownRootOptions::builder()
            .chain_id(context.chain_id)
            .contract_address(contract_address)
            .root_hash(biguint_to_u256(&merkle_root))
            .timeout_ms(query_timeout_ms)
            .build();
        if !self.commitment_pool_contracts.is_known_root(options).await? {
            return Err(SpendsError::UnknownMerkleRootError(merkle_root.to_string()));
        }
        Ok(())
    }

    // async fn build_merkle_tree_from_db(
    //     &self,
    //     context: &SpendContext,
    //     query_timeout_ms: Option<u64>,
    // ) -> Result<MerkleTree, SpendsError> {
    //     let condition = Condition::and(vec![
    //         SubFilter::equal(CommitmentColumn::ChainId, context.chain_id),
    //         SubFilter::equal(
    //             CommitmentColumn::ContractAddress,
    //             context.contract_config.address().to_string(),
    //         ),
    //         SubFilter::equal(CommitmentColumn::Status, CommitmentStatus::Included as i32),
    //         SubFilter::is_not_null(CommitmentColumn::LeafIndex),
    //     ]);
    //     let order_by = OrderBy::builder()
    //         .order(Order::Asc)
    //         .columns(vec![CommitmentColumn::LeafIndex.to_string()])
    //         .build();
    //     let filter = QueryFilter::builder()
    //         .conditions(vec![condition])
    //         .conditions_operator(ConditionOperator::And)
    //         .order_by(order_by)
    //         .build();
    //     let commitments = self.db.commitments.find(filter).await?;
    //     let leaves = commitments
    //         .into_iter()
    //         .map(|commitment| commitment.data.commitment_hash)
    //         .collect::<Vec<_>>();
    //     let merkle_tree = MerkleTree::new(Some(leaves), None, None)?;
    //     self.verify_merkle_tree(context, &merkle_tree, query_timeout_ms).await?;
    //     Ok(merkle_tree)
    // }
}

#[derive(Debug, Clone, TypedBuilder)]
pub(crate) struct MerkleTreeWrapper {
    loaded_block_number: u64,
    leaves: Vec<BigUint>,
}

#[derive(Debug)]
pub(crate) struct CacheMerkleTree {
    tree: RwLock<HashMap<(u64, String), MerkleTreeWrapper>>,
}

impl CacheMerkleTree {
    pub fn new() -> Self {
        Self {
            tree: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(&self, chain_id: u64, contract: &str) -> Option<MerkleTreeWrapper> {
        let tree = self.tree.read().await;
        tree.get(&(chain_id, contract.to_string())).cloned()
    }

    pub async fn update(&self, chain_id: u64, contract: &str, wrap_tree: MerkleTreeWrapper) {
        let mut tree = self.tree.write().await;
        tree.insert((chain_id, contract.to_string()), wrap_tree);
    }
}
