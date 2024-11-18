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
        let wrap_tree = self
            .build_from_raw_and_provider(context, raw, expect_leaf_index)
            .await?;
        self.build_protocol_merkle_tree(context, options, wrap_tree, false)
            .await
    }

    async fn build_from_packer(
        &self,
        context: &SpendContext,
        options: &SendSpendOptions,
        expect_leaf_index: u64,
    ) -> Result<MerkleTree, SpendsError> {
        let (wrap_tree, is_cache) = self.build_from_packer_and_provider(context, expect_leaf_index).await?;
        self.build_protocol_merkle_tree(context, options, wrap_tree, is_cache)
            .await
    }

    async fn build_from_raw_and_provider(
        &self,
        context: &SpendContext,
        raw: &[u8],
        expect_leaf_index: u64,
    ) -> Result<MerkleTreeWrapper, SpendsError> {
        let decompress_data = zstd::decode_all(raw)?;
        let proto_merkle_tree = ProtoMerkleTree::decode(&*decompress_data)?;
        let wrap_tree = MerkleTreeWrapper::builder()
            .leaves(
                proto_merkle_tree
                    .commitment_hashes
                    .iter()
                    .map(bytes_to_biguint)
                    .collect(),
            )
            .loaded_block_number(proto_merkle_tree.loaded_block_number)
            .build();
        if (wrap_tree.leaves.len() as u64) > expect_leaf_index {
            return Ok(wrap_tree);
        }
        self.try_filling_from_provider(context, wrap_tree, expect_leaf_index)
            .await
    }
    async fn build_from_packer_and_provider(
        &self,
        context: &SpendContext,
        expect_leaf_index: u64,
    ) -> Result<(MerkleTreeWrapper, bool), SpendsError> {
        if let (Some(wrap_tree), is_cache) = self.build_from_cache_or_packer(context, expect_leaf_index).await? {
            if wrap_tree.leaves.len() as u64 > expect_leaf_index {
                return Ok((wrap_tree, is_cache));
            }
            let filling_tree = self
                .try_filling_from_provider(context, wrap_tree, expect_leaf_index)
                .await?;
            Ok((filling_tree, false))
        } else {
            let wrap_tree = MerkleTreeWrapper::builder()
                .leaves(vec![])
                .loaded_block_number(context.contract_config.start_block())
                .build();
            let filling_tree = self
                .try_filling_from_provider(context, wrap_tree, expect_leaf_index)
                .await?;
            Ok((filling_tree, false))
        }
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

    async fn try_filling_from_provider(
        &self,
        context: &SpendContext,
        wrap_tree: MerkleTreeWrapper,
        expect_leaf_index: u64,
    ) -> Result<MerkleTreeWrapper, SpendsError> {
        info!(
            "fetching commitments from provider for chain_id: {}, contract: {} start block: {}",
            context.chain_id,
            context.contract_config.address(),
            wrap_tree.loaded_block_number + 1
        );
        let (cms, target_block) = self
            .fetch_from_provider(context, wrap_tree.loaded_block_number + 1)
            .await?;
        let mut tree = wrap_tree;
        tree.leaves.extend(cms);
        tree.loaded_block_number = target_block;
        if (tree.leaves.len() as u64) <= expect_leaf_index {
            return Err(SpendsError::RawResourceError(format!(
                "insufficient commitments in fetched tree len: {}, expect: {}",
                tree.leaves.len(),
                expect_leaf_index + 1
            )));
        }
        Ok(tree)
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

    async fn build_protocol_merkle_tree(
        &self,
        context: &SpendContext,
        options: &SendSpendOptions,
        wrap_tree: MerkleTreeWrapper,
        is_cache: bool,
    ) -> Result<MerkleTree, SpendsError> {
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
