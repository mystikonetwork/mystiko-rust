use crate::data::LoadedData;
use crate::handler::{CommitmentQueryOption, DataHandler};
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::data::ValidateContractData;
use crate::validator::rule::error::{Result, RuleValidatorError};
use crate::validator::types::ValidateOption;
use async_trait::async_trait;
use ethers_core::types::Address;
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_ethers::provider::pool::Providers;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::convert::{biguint_to_u256, bytes_to_biguint};
use num_bigint::BigUint;
use std::str::FromStr;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct TreeChecker<R, H = Box<dyn DataHandler<R>>, P = Box<dyn Providers>> {
    providers: Arc<P>,
    handler: Arc<H>,
    #[builder(default = 20)]
    tree_max_levels: u32,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R, H, P> RuleChecker for TreeChecker<R, H, P>
where
    R: LoadedData,
    H: DataHandler<R>,
    P: Providers,
{
    async fn check(&self, data: &ValidateContractData, _option: &ValidateOption) -> Result<()> {
        let tree_root = self.build_tree(data).await?;
        match tree_root {
            Some(root) => {
                if !self.check_tree_root(data, &root).await? {
                    Err(RuleValidatorError::ValidateError("tree root not exist".to_string()))
                } else {
                    Ok(())
                }
            }
            None => return Ok(()),
        }
    }
}

impl<R, H, P> TreeChecker<R, H, P>
where
    R: LoadedData,
    H: DataHandler<R>,
    P: Providers,
{
    async fn build_tree(&self, data: &ValidateContractData) -> Result<Option<BigUint>> {
        if !data.commitments.iter().any(|c| c.status == CommitmentStatus::Included) {
            return Ok(None);
        }

        let target_block = data.start_block - 1;
        let option = CommitmentQueryOption::builder()
            .chain_id(data.chain_id)
            .contract_address(data.contract_address.clone())
            .end_block(target_block)
            .status(CommitmentStatus::Included)
            .build();
        let query_result = self.handler.query_commitments(&option).await?;
        if query_result.end_block != target_block {
            return Err(RuleValidatorError::ValidateError("end block mismatch".to_string()));
        }

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
        Ok(Some(tree.root()))
    }

    async fn check_tree_root(&self, data: &ValidateContractData, tree_root: &BigUint) -> Result<bool> {
        let address = Address::from_str(data.contract_address.as_str())
            .map_err(|_| RuleValidatorError::ValidateError("invalid contract address".to_string()))?;
        let provider = self.providers.get_provider(data.chain_id).ok_or_else(|| {
            RuleValidatorError::ValidateError(format!("no provider found for chain id {}", data.chain_id))
        })?;
        let commitment_contract = CommitmentPool::new(address, provider);
        let known = commitment_contract.is_known_root(biguint_to_u256(tree_root)).await?;
        Ok(known)
    }
}
