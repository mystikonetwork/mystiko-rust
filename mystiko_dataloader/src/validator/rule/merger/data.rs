use crate::data::{DataRef, DataType, FullData, LiteData, LoadedData};
use crate::handler::{CommitmentQueryOption, DataHandler};
use crate::validator::rule::merger::error::DataMergeError;
use crate::validator::rule::merger::DataMergeResult;
use crate::validator::rule::types::{ValidateCommitment, ValidateMergedData, ValidateNullifier};
use crate::validator::rule::ValidateOriginalData;
use log::error;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::bytes_to_biguint;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct DataMerger<R, H = Box<dyn DataHandler<R>>> {
    handler: Arc<H>,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

impl<R, H> DataMerger<R, H>
where
    R: LoadedData,
    H: DataHandler<R>,
{
    pub async fn merge_contract_data(&self, data: &ValidateOriginalData<'_, R>) -> DataMergeResult<ValidateMergedData> {
        if data.contract_data.start_block < 1 {
            return Err(DataMergeError::StartBlockError);
        }

        let (commitments, nullifiers) = match data.contract_data.data {
            None => match R::data_type() {
                DataType::Full => (vec![], Some(vec![])),
                DataType::Lite => (vec![], None),
            },
            Some(ref d) => match R::data_ref(d) {
                DataRef::Full(full) => self.merge_contract_full_data(data, full).await?,
                DataRef::Lite(lite) => self.merge_contract_lite_data(data, lite).await?,
            },
        };

        Ok(ValidateMergedData::builder()
            .chain_id(data.chain_id)
            .contract_address(data.contract_data.address.clone())
            .start_block(data.contract_data.start_block)
            .end_block(data.contract_data.end_block)
            .commitments(commitments)
            .nullifiers(nullifiers)
            .build())
    }

    async fn merge_contract_full_data(
        &self,
        data: &ValidateOriginalData<'_, R>,
        full_data: &FullData,
    ) -> DataMergeResult<(Vec<ValidateCommitment>, Option<Vec<ValidateNullifier>>)> {
        let cms = self.merge_commitment(data, &full_data.commitments).await?;
        let nullifiers = self.merge_nullifier(&full_data.nullifiers).await?;
        Ok((cms, Some(nullifiers)))
    }

    async fn merge_contract_lite_data(
        &self,
        data: &ValidateOriginalData<'_, R>,
        lite_data: &LiteData,
    ) -> DataMergeResult<(Vec<ValidateCommitment>, Option<Vec<ValidateNullifier>>)> {
        let cms = self.merge_commitment(data, &lite_data.commitments).await?;
        Ok((cms, None))
    }

    async fn merge_nullifier(&self, nullifiers: &[Nullifier]) -> DataMergeResult<Vec<ValidateNullifier>> {
        let mut validate_nullifiers = vec![];
        for n in nullifiers {
            validate_nullifiers.push(
                ValidateNullifier::builder()
                    .nullifier(bytes_to_biguint(n.nullifier.as_slice()))
                    .build(),
            );
        }
        Ok(validate_nullifiers)
    }

    async fn merge_commitment(
        &self,
        data: &ValidateOriginalData<'_, R>,
        commitments: &[Commitment],
    ) -> DataMergeResult<Vec<ValidateCommitment>> {
        if commitments.is_empty() {
            return Ok(vec![]);
        }

        let mut validates = merge_fetched_commitment(commitments)?;
        self.recovery_leaf_index(data, &mut validates).await?;
        self.sort_by_leaf_index(&mut validates);
        Ok(validates)
    }

    async fn recovery_leaf_index(
        &self,
        data: &ValidateOriginalData<'_, R>,
        commitments: &mut [ValidateCommitment],
    ) -> DataMergeResult<()> {
        let filled_cms: Vec<_> = commitments
            .iter_mut()
            .filter(|cm| cm.status == CommitmentStatus::Included && !cm.inner_merge)
            .collect();
        if filled_cms.is_empty() {
            return Ok(());
        }

        let commitment_hash: Vec<_> = filled_cms.iter().map(|cm| cm.commitment_hash.clone()).collect();
        let target_block = data.contract_data.start_block - 1;
        let query_option = CommitmentQueryOption::builder()
            .chain_id(data.chain_id)
            .contract_address(data.contract_data.address.clone())
            .end_block(target_block)
            .commitment_hash(commitment_hash)
            .build();
        let query_result = self.handler.query_commitments(&query_option).await?;
        if query_result.end_block != target_block {
            return Err(DataMergeError::TargetBlockError(target_block, query_result.end_block));
        }

        if query_result.result.len() != filled_cms.len() {
            return Err(DataMergeError::CommitmentDataLenError(
                filled_cms.len(),
                query_result.result.len(),
            ));
        }

        for (i, cm) in filled_cms.into_iter().enumerate() {
            if cm.commitment_hash != bytes_to_biguint(query_result.result[i].commitment_hash.as_slice()) {
                return Err(DataMergeError::CommitmentHashMismatchError);
            }

            if let Some(leaf_index) = query_result.result[i].leaf_index {
                cm.leaf_index = leaf_index;
            } else {
                return Err(DataMergeError::LeafIndexIsNoneError);
            }
        }

        Ok(())
    }

    fn sort_by_leaf_index(&self, commitments: &mut [ValidateCommitment]) {
        commitments.sort_by(|a, b| a.leaf_index.cmp(&b.leaf_index));
    }
}

fn merge_fetched_commitment(commitments: &[Commitment]) -> DataMergeResult<Vec<ValidateCommitment>> {
    let mut commitment_map = HashMap::new();

    commitments
        .iter()
        .filter_map(|cm| match CommitmentStatus::from_i32(cm.status) {
            Some(status) if status == CommitmentStatus::Included || status == CommitmentStatus::Queued => {
                let commitment_hash = bytes_to_biguint(cm.commitment_hash.as_slice());
                let validate_commitment = ValidateCommitment::builder()
                    .commitment_hash(commitment_hash)
                    .status(status)
                    .leaf_index(cm.leaf_index.unwrap_or(u64::MAX))
                    .inner_merge(false)
                    .build();
                Some(validate_commitment)
            }
            _ => None,
        })
        .try_for_each(|validate_commitment| -> DataMergeResult<()> {
            match commitment_map.entry(validate_commitment.commitment_hash.clone()) {
                Entry::Occupied(mut entry) => {
                    let elem: &mut ValidateCommitment = entry.get_mut();
                    if elem.commitment_hash != validate_commitment.commitment_hash {
                        error!("commitment hash collision");
                        return Err(DataMergeError::CommitmentHashCollisionError);
                    }

                    merge_same_commitments(elem, &validate_commitment)?;
                }
                Entry::Vacant(entry) => {
                    entry.insert(validate_commitment);
                }
            }
            Ok(())
        })?;

    Ok(commitment_map.into_values().collect())
}

fn merge_same_commitments(src: &mut ValidateCommitment, dst: &ValidateCommitment) -> DataMergeResult<()> {
    if !src.inner_merge {
        if src.status != dst.status {
            if dst.status == CommitmentStatus::Queued {
                src.leaf_index = dst.leaf_index;
            } else if dst.status == CommitmentStatus::Included {
                src.status = dst.status;
            }
            src.inner_merge = true;
        } else if src.leaf_index != dst.leaf_index {
            return Err(DataMergeError::LeafIndexMismatchError(src.leaf_index, dst.leaf_index));
        }
    } else if dst.status == CommitmentStatus::Queued && src.leaf_index != dst.leaf_index {
        return Err(DataMergeError::LeafIndexMismatchError(src.leaf_index, dst.leaf_index));
    }

    Ok(())
}
