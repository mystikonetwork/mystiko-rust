use crate::{
    CommitmentPoolContractHandler, IsHistoricCommitmentOptions, IsSpentNullifierOptions, PublicAssetHandler, Spend,
    Spends, SpendsError,
};
use mystiko_dataloader::handler::document::CommitmentColumn;
use mystiko_ethers::Providers;
use mystiko_protos::core::handler::v1::FixSpendStatusOptions;
use mystiko_protos::core::v1::SpendStatus;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_storage::{Document, StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::biguint_to_u256;
use mystiko_utils::time::current_timestamp;

impl<F, S, A, C, T, P, R, V, K, X> Spends<F, S, A, C, T, P, R, V, K, X>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    C: CommitmentPoolContractHandler,
    P: Providers + 'static,
    SpendsError: From<A::Error> + From<C::Error>,
{
    pub(crate) async fn execute_fix_status(
        &self,
        options: FixSpendStatusOptions,
    ) -> Result<Document<Spend>, SpendsError> {
        let mut spend = self
            .db
            .spends
            .find_by_id(&options.spend_id.clone())
            .await?
            .ok_or_else(|| SpendsError::SpendNotFoundError(options.spend_id))?;
        let input_conditions = Condition::and(vec![
            SubFilter::equal(CommitmentColumn::ChainId, spend.data.chain_id),
            SubFilter::equal(CommitmentColumn::ContractAddress, spend.data.contract_address.clone()),
            SubFilter::in_list(CommitmentColumn::CommitmentHash, spend.data.input_commitments.clone()),
        ]);
        let mut input_commitments = self.db.commitments.find(input_conditions).await?;
        let contract_address = ethers_address_from_string(spend.data.contract_address.clone())?;
        let mut spent_succeeded = false;
        for commitment in input_commitments.iter_mut() {
            if let Some(ref nullifier) = commitment.data.nullifier {
                let is_spent_options = IsSpentNullifierOptions::builder()
                    .chain_id(spend.data.chain_id)
                    .contract_address(contract_address)
                    .nullifier(biguint_to_u256(nullifier))
                    .timeout_ms(options.query_timeout_ms)
                    .build();
                let spent = self
                    .commitment_pool_contracts
                    .is_spent_nullifier(is_spent_options)
                    .await?;
                commitment.data.spent = spent;
                if spent {
                    commitment.data.status = CommitmentStatus::Included as i32;
                }
                commitment.updated_at = current_timestamp();
                self.db.commitments.update(commitment).await?;
                if spent {
                    spent_succeeded = true;
                }
            }
        }

        if let Some(output_commitments) = &spend.data.output_commitments {
            let output_conditions = Condition::and(vec![
                SubFilter::equal(CommitmentColumn::ChainId, spend.data.chain_id),
                SubFilter::equal(CommitmentColumn::ContractAddress, spend.data.contract_address.clone()),
                SubFilter::in_list(CommitmentColumn::CommitmentHash, output_commitments.clone()),
            ]);
            let mut out_commitments = self.db.commitments.find(output_conditions).await?;
            for commitment in out_commitments.iter_mut() {
                let historic_commitment_options = IsHistoricCommitmentOptions::builder()
                    .chain_id(spend.data.chain_id)
                    .contract_address(contract_address)
                    .commitment_hash(biguint_to_u256(&commitment.data.commitment_hash))
                    .timeout_ms(options.query_timeout_ms)
                    .build();
                let is_historic = self
                    .commitment_pool_contracts
                    .is_historic_commitment(historic_commitment_options)
                    .await?;
                if is_historic {
                    commitment.data.status = if commitment.data.included_transaction_hash.is_some() {
                        CommitmentStatus::Included as i32
                    } else {
                        CommitmentStatus::Queued as i32
                    };
                    commitment.updated_at = current_timestamp();
                    self.db.commitments.update(commitment).await?;
                }
            }
        }

        if spent_succeeded {
            spend.data.status = SpendStatus::Succeeded as i32;
            spend.data.error_message = None;
            spend.updated_at = current_timestamp();
            self.db.spends.update(&spend).await?;
        } else if spend.data.status == SpendStatus::Succeeded as i32 {
            spend.data.status = SpendStatus::Failed as i32;
            spend.updated_at = current_timestamp();
            self.db.spends.update(&spend).await?;
        }

        Ok(spend)
    }
}
