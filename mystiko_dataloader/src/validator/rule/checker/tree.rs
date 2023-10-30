use crate::data::LoadedData;
use crate::handler::{CommitmentQueryOption, DataHandler};
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::types::ValidateMergedData;
use crate::validator::rule::CheckerResult;
use crate::validator::rule::RuleCheckError;
use crate::validator::rule::{MerkleTreeCheckerError, ValidateOriginalData};
use async_trait::async_trait;
use ethers_core::types::{BlockId, BlockNumber};
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_ethers::Providers;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{biguint_to_u256, bytes_to_biguint};
use num_bigint::BigUint;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub const RULE_MERKLE_TREE_CHECKER_NAME: &str = "merkle_tree";

#[derive(Debug, TypedBuilder)]
pub struct MerkleTreeChecker<R, H = Box<dyn DataHandler<R>>, P = Box<dyn Providers>> {
    providers: Arc<P>,
    handler: Arc<H>,
    #[builder(default = 20)]
    tree_max_levels: u32,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R, H, P> RuleChecker<R> for MerkleTreeChecker<R, H, P>
where
    R: LoadedData,
    H: DataHandler<R>,
    P: Providers,
{
    fn name(&self) -> &'static str {
        RULE_MERKLE_TREE_CHECKER_NAME
    }

    async fn check<'a>(
        &self,
        data: &ValidateOriginalData<'a, R>,
        merged_data: &ValidateMergedData,
    ) -> CheckerResult<()> {
        if data.should_skip_commitment_check() {
            return Ok(());
        }

        if !merged_data
            .commitments
            .iter()
            .any(|c| c.status == CommitmentStatus::Included)
        {
            return Ok(());
        }

        let tree_root = self.build_tree(merged_data).await?;
        self.check_tree_root(merged_data, &tree_root).await
    }
}

impl<R, H, P> MerkleTreeChecker<R, H, P>
where
    R: LoadedData,
    H: DataHandler<R>,
    P: Providers,
{
    async fn build_tree(&self, data: &ValidateMergedData) -> CheckerResult<BigUint> {
        let target_block = data.start_block - 1;
        let option = CommitmentQueryOption::builder()
            .chain_id(data.chain_id)
            .contract_address(data.contract_address.clone())
            .end_block(target_block)
            .status(CommitmentStatus::Included)
            .build();
        let mut query_result = self.handler.query_commitments(&option).await?;
        if query_result.end_block != target_block {
            return Err(MerkleTreeCheckerError::TargetBlockError(target_block, query_result.end_block).into());
        }
        query_result.result.sort_by(|a, b| a.leaf_index.cmp(&b.leaf_index));
        let mut elements = query_result
            .result
            .iter()
            .map(|c| bytes_to_biguint(&c.commitment_hash))
            .collect::<Vec<_>>();
        data.commitments
            .iter()
            .filter(|c| c.status == CommitmentStatus::Included)
            .for_each(|c| elements.push(c.commitment_hash.clone()));

        let tree = MerkleTree::new(Some(elements), Some(self.tree_max_levels), None)?;
        Ok(tree.root())
    }

    async fn check_tree_root(&self, data: &ValidateMergedData, tree_root: &BigUint) -> CheckerResult<()> {
        let address = ethers_address_from_string(&data.contract_address)
            .map_err(|_| RuleCheckError::ContractAddressError(data.contract_address.clone()))?;
        let provider = self.providers.get_provider(data.chain_id).await?;
        let commitment_contract = CommitmentPool::new(address, provider);
        let known = commitment_contract
            .is_known_root(biguint_to_u256(tree_root))
            .block(BlockId::Number(BlockNumber::Number(data.end_block.into())))
            .await?;
        if !known {
            Err(MerkleTreeCheckerError::MerkleTreeRootNotKnownError.into())
        } else {
            Ok(())
        }
    }
}
