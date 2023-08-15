use crate::data::chain::ChainData;
use crate::data::contract::ContractData;
use crate::data::result::{ChainResult, ContractResult};
use crate::data::types::{DataRef, FullData, LiteData, LoadedData};
use crate::handler::types::{CommitmentQueryOption, DataHandler};
use crate::validator::rule::checker::counter::CounterCheckerBuilder;
use crate::validator::rule::checker::sequence::SequenceCheckerBuilder;
use crate::validator::rule::checker::tree::TreeCheckerBuilder;
use crate::validator::rule::data::{ValidateCommitment, ValidateContractData, ValidateNullifier};
use crate::validator::rule::error::{Result, RuleValidatorError};
use crate::validator::rule::types::{RuleChecker, RuleCheckerType};
use crate::validator::types::{DataValidator, ValidateOption, ValidateResult};
use async_trait::async_trait;
use mystiko_ethers::provider::factory::Provider;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::bytes_to_biguint;
use std::collections::HashSet;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug)]
pub struct RuleValidator<R, H = Box<dyn DataHandler<R>>, L = Box<dyn RuleChecker>> {
    _phantom: std::marker::PhantomData<R>,
    handler: Arc<H>,
    rules: Vec<Arc<L>>,
}

#[derive(Debug, Default, TypedBuilder)]
pub struct RuleValidatorBuilder<R, H = Box<dyn DataHandler<R>>> {
    _phantom: std::marker::PhantomData<R>,
    rule_types: Option<Vec<RuleCheckerType>>,
    handler: Option<Arc<H>>,
    provider: Option<Arc<Provider>>,
}

#[async_trait]
impl<R, H, L> DataValidator<R> for RuleValidator<R, H, L>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
    L: 'static + RuleChecker,
{
    async fn validate(&self, data: &ChainData<R>, option: &ValidateOption) -> ValidateResult {
        if data.contracts_data.is_empty() {
            return Err(anyhow::Error::msg("data to be validated is empty").into());
        }

        let mut futures = Vec::new();
        for contract_data in &data.contracts_data {
            futures.push(self.validate_contract_data(data.chain_id, contract_data, option));
        }

        let results = futures::future::join_all(futures).await;
        let chain_result = ChainResult::builder()
            .chain_id(data.chain_id)
            .contract_results(results)
            .build();

        Ok(chain_result)
    }
}

impl<R, H> RuleValidatorBuilder<R, H>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
{
    pub fn new() -> Self {
        RuleValidatorBuilder {
            _phantom: std::marker::PhantomData,
            rule_types: None,
            handler: None,
            provider: None,
        }
    }

    pub fn rule_types(mut self, rules: Vec<RuleCheckerType>) -> Self {
        let mut seen = HashSet::new();
        let unique_rules: Vec<RuleCheckerType> = rules.into_iter().filter(|r| seen.insert(r.clone())).collect();
        self.rule_types = Some(unique_rules);
        self
    }

    pub fn handle(mut self, handle: H) -> Self {
        self.handler = Some(Arc::new(handle));
        self
    }

    pub fn shared_handle(mut self, handle: Arc<H>) -> Self {
        self.handler = Some(handle);
        self
    }

    pub fn shared_provider(mut self, provider: Arc<Provider>) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn build(self) -> Result<RuleValidator<R, H>> {
        let handler = self
            .handler
            .ok_or_else(|| RuleValidatorError::BuildError("handle cannot be None".to_string()))?;
        let provider = self
            .provider
            .ok_or_else(|| RuleValidatorError::BuildError("provider cannot be None".to_string()))?;
        let rule_types = self
            .rule_types
            .ok_or_else(|| RuleValidatorError::BuildError("rule types cannot be None".to_string()))?;

        let mut rules = vec![];
        for r in rule_types.iter() {
            match r {
                RuleCheckerType::Sequence => {
                    let sequence = SequenceCheckerBuilder::new().shared_handle(handler.clone()).build()?;
                    rules.push(Arc::new(Box::new(sequence) as Box<dyn RuleChecker>))
                }
                RuleCheckerType::Counter => {
                    let counter = CounterCheckerBuilder::new()
                        .shared_provider(provider.clone())
                        .shared_handle(handler.clone())
                        .build()?;
                    rules.push(Arc::new(Box::new(counter) as Box<dyn RuleChecker>))
                }
                RuleCheckerType::Tree => {
                    let tree = TreeCheckerBuilder::new()
                        .shared_provider(provider.clone())
                        .shared_handle(handler.clone())
                        .build()?;
                    rules.push(Arc::new(Box::new(tree) as Box<dyn RuleChecker>))
                }
            }
        }

        Ok(RuleValidator {
            _phantom: std::marker::PhantomData,
            handler,
            rules,
        })
    }
}

impl<R, H, L> RuleValidator<R, H, L>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
    L: 'static + RuleChecker,
{
    async fn validate_contract_data(
        &self,
        chain_id: u64,
        data: &ContractData<R>,
        option: &ValidateOption,
    ) -> ContractResult<()> {
        match self.merge_and_check(chain_id, data, option).await {
            Ok(_) => ContractResult::builder()
                .address(data.address.clone())
                .result(Ok(()))
                .build(),
            Err(e) => ContractResult::builder()
                .address(data.address.clone())
                .result(Err(e.into()))
                .build(),
        }
    }

    async fn merge_and_check(&self, chain_id: u64, data: &ContractData<R>, option: &ValidateOption) -> Result<()> {
        let merged_data = self.merge_contract_data(chain_id, data).await?;

        for rule in &self.rules {
            rule.check(&merged_data, option).await?;
        }
        Ok(())
    }

    async fn merge_contract_data(&self, chain_id: u64, data: &ContractData<R>) -> Result<ValidateContractData> {
        if data.start_block < 1 {
            return Err(RuleValidatorError::ValidateError("start block error".to_string()));
        }

        let (commitments, nullifiers) = match data.data {
            None => (vec![], vec![]),
            Some(ref d) => match R::data_ref(d) {
                DataRef::Full(full) => {
                    self.merge_contract_full_data(chain_id, &data.address, data.start_block, data.end_block, full)
                        .await?
                }
                DataRef::Lite(lite) => {
                    self.merge_contract_lite_data(chain_id, &data.address, data.start_block, data.end_block, lite)
                        .await?
                }
            },
        };

        Ok(ValidateContractData::builder()
            .chain_id(chain_id)
            .contract_address(data.address.clone())
            .start_block(data.start_block)
            .end_block(data.end_block)
            .commitments(commitments)
            .nullifiers(nullifiers)
            .build())
    }

    async fn merge_contract_full_data(
        &self,
        chain_id: u64,
        contract_address: &str,
        start_block: u64,
        end_block: u64,
        data: &FullData,
    ) -> Result<(Vec<ValidateCommitment>, Vec<ValidateNullifier>)> {
        let cms = self
            .merge_commitment(chain_id, contract_address, start_block, end_block, &data.commitments)
            .await?;
        let nullifiers = self.merge_nullifier(&data.nullifiers).await?;
        Ok((cms, nullifiers))
    }

    async fn merge_contract_lite_data(
        &self,
        chain_id: u64,
        contract_address: &str,
        start_block: u64,
        end_block: u64,
        data: &LiteData,
    ) -> Result<(Vec<ValidateCommitment>, Vec<ValidateNullifier>)> {
        let cms = self
            .merge_commitment(chain_id, contract_address, start_block, end_block, &data.commitments)
            .await?;
        Ok((cms, vec![]))
    }

    async fn merge_nullifier(&self, data: &[Nullifier]) -> Result<Vec<ValidateNullifier>> {
        let mut nullifiers = vec![];
        for n in data {
            nullifiers.push(
                ValidateNullifier::builder()
                    .nullifier(bytes_to_biguint(n.nullifier.as_slice()))
                    .build(),
            );
        }
        Ok(nullifiers)
    }

    async fn merge_commitment(
        &self,
        chain_id: u64,
        contract_address: &str,
        start_block: u64,
        end_block: u64,
        data: &[Commitment],
    ) -> Result<Vec<ValidateCommitment>> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let mut commitments = self.combine_commitment(data).await?;
        self.recovery_leaf_index(chain_id, contract_address, start_block, end_block, &mut commitments)
            .await?;
        self.sort_commitment_by_leaf_index(&mut commitments).await?;
        self.check_commitment_status(&commitments)?;

        Ok(commitments)
    }

    async fn combine_commitment(&self, data: &[Commitment]) -> Result<Vec<ValidateCommitment>> {
        let mut queued_cms = construct_validate_commitments(data, CommitmentStatus::Queued);
        let mut included_cms = construct_validate_commitments(data, CommitmentStatus::Included);

        if queued_cms.is_empty() {
            return Ok(included_cms);
        } else if included_cms.is_empty() {
            return Ok(queued_cms);
        }

        let mut unmatched_cms = Vec::new();
        for queued_cm in queued_cms.drain(..) {
            let mut matched = false;
            for included in &mut included_cms {
                if included.commitment_hash == queued_cm.commitment_hash {
                    included.leaf_index = queued_cm.leaf_index;
                    included.merged = true;
                    matched = true;
                    break;
                }
            }
            if !matched {
                unmatched_cms.push(queued_cm);
            }
        }

        included_cms.extend(unmatched_cms);
        Ok(included_cms)
    }

    async fn recovery_leaf_index(
        &self,
        chain_id: u64,
        contract_address: &str,
        start_block: u64,
        _end_block: u64,
        cms: &mut [ValidateCommitment],
    ) -> Result<()> {
        let filled_cms: Vec<_> = cms
            .iter_mut()
            .filter(|cm| cm.status == CommitmentStatus::Included && !cm.merged)
            .collect();
        if filled_cms.is_empty() {
            return Ok(());
        }

        let commitment_hash: Vec<_> = filled_cms.iter().map(|cm| cm.commitment_hash.clone()).collect();
        let query_end_block = start_block - 1;
        let query_option = CommitmentQueryOption::builder()
            .chain_id(chain_id)
            .contract_address(contract_address.to_string())
            .end_block(query_end_block)
            .commitment_hash(commitment_hash)
            .build();
        let query_result = self.handler.query_commitments(&query_option).await?;
        if query_result.end_block != query_end_block || query_result.result.len() != filled_cms.len() {
            return Err(RuleValidatorError::ValidateError(
                "query commitment data invalid".to_string(),
            ));
        }

        for (i, cm) in filled_cms.into_iter().enumerate() {
            if cm.commitment_hash != bytes_to_biguint(query_result.result[i].commitment_hash.as_slice()) {
                return Err(RuleValidatorError::ValidateError(
                    "query commitment hash mismatch".to_string(),
                ));
            }

            if let Some(leaf_index) = query_result.result[i].leaf_index {
                cm.leaf_index = leaf_index;
            } else {
                return Err(RuleValidatorError::ValidateError(
                    "query commitment leaf index is none".to_string(),
                ));
            }
        }

        Ok(())
    }

    async fn sort_commitment_by_leaf_index(&self, commitments: &mut [ValidateCommitment]) -> Result<()> {
        commitments.sort_by(|a, b| a.leaf_index.cmp(&b.leaf_index));

        if commitments
            .windows(2)
            .any(|window| window[0].leaf_index + 1 != window[1].leaf_index)
        {
            return Err(RuleValidatorError::ValidateError(
                "leaf index values not sequence".to_string(),
            ));
        }

        Ok(())
    }

    fn check_commitment_status(&self, commitments: &[ValidateCommitment]) -> Result<()> {
        let first_status = commitments[0].status;
        if first_status == CommitmentStatus::Queued {
            for cm in commitments {
                if cm.status != first_status {
                    return Err(RuleValidatorError::ValidateError(
                        "commitment status not all queued".to_string(),
                    ));
                }
            }
        } else if first_status == CommitmentStatus::Included {
            let mut queued_seen = false;
            let mut merged_seen = false;

            for cm in commitments {
                if cm.status == CommitmentStatus::Queued {
                    queued_seen = true;
                } else if cm.status == CommitmentStatus::Included && queued_seen {
                    return Err(RuleValidatorError::ValidateError(
                        "invalid sequence of commitment included after queued".to_string(),
                    ));
                }

                if cm.merged {
                    merged_seen = true;
                } else if cm.status == CommitmentStatus::Included && !cm.merged && merged_seen {
                    return Err(RuleValidatorError::ValidateError(
                        "invalid sequence of commitment merged status".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}

fn construct_validate_commitments(cms: &[Commitment], status: CommitmentStatus) -> Vec<ValidateCommitment> {
    cms.iter()
        .filter(|cm| cm.status == status as i32)
        .map(|cm| {
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(cm.commitment_hash.as_slice()))
                .status(status)
                .leaf_index(cm.leaf_index.unwrap_or(0_u64))
                .merged(false)
                .build()
        })
        .collect()
}
