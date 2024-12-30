use crate::{
    CommitmentPoolContractHandler, Deposit, DepositContractHandler, Deposits, DepositsError,
    IsHistoricCommitmentOptions, PublicAssetHandler, TransactionHandler,
};
use mystiko_dataloader::handler::document::CommitmentColumn;
use mystiko_ethers::Providers;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::FixDepositStatusOptions;
use mystiko_protos::core::v1::{DepositStatus, Transaction};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_storage::{Document, StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::biguint_to_u256;
use mystiko_utils::time::current_timestamp;

impl<F, S, A, D, C, T, P, N> Deposits<F, S, A, D, C, T, P, N>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContractHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    P: Providers + 'static,
    DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
{
    pub(crate) async fn execute_fix_status(
        &self,
        options: FixDepositStatusOptions,
    ) -> Result<Document<Deposit>, DepositsError> {
        let mut deposit = self
            .db
            .deposits
            .find_by_id(&options.deposit_id)
            .await?
            .ok_or_else(|| DepositsError::IdNotFoundError(options.deposit_id))?;
        let (commitment, is_historic) = if deposit.data.bridge_type == BridgeType::Loop as i32 {
            let conditions = Condition::and(vec![
                SubFilter::equal(CommitmentColumn::ChainId, deposit.data.chain_id),
                SubFilter::equal(CommitmentColumn::ContractAddress, deposit.data.pool_address.clone()),
                SubFilter::equal(CommitmentColumn::CommitmentHash, deposit.data.commitment_hash.clone()),
            ]);
            let commitment = self.db.commitments.find_one(conditions).await?;
            let pool_contract_address = ethers_address_from_string(deposit.data.pool_address.clone())?;
            let historic_commitment_options = IsHistoricCommitmentOptions::builder()
                .chain_id(deposit.data.chain_id)
                .contract_address(pool_contract_address)
                .commitment_hash(biguint_to_u256(&deposit.data.commitment_hash))
                .timeout_ms(options.query_timeout_ms)
                .build();
            let is_historic = self
                .commitment_pool_contracts
                .is_historic_commitment(historic_commitment_options)
                .await?;
            (commitment, is_historic)
        } else {
            let conditions = Condition::and(vec![
                SubFilter::equal(CommitmentColumn::ChainId, deposit.data.dst_chain_id),
                SubFilter::equal(CommitmentColumn::ContractAddress, deposit.data.dst_pool_address.clone()),
                SubFilter::equal(CommitmentColumn::CommitmentHash, deposit.data.commitment_hash.clone()),
            ]);
            let commitment = self.db.commitments.find_one(conditions).await?;
            let pool_contract_address = ethers_address_from_string(deposit.data.dst_pool_address.clone())?;
            let historic_commitment_options = IsHistoricCommitmentOptions::builder()
                .chain_id(deposit.data.dst_chain_id)
                .contract_address(pool_contract_address)
                .commitment_hash(biguint_to_u256(&deposit.data.commitment_hash))
                .timeout_ms(options.query_timeout_ms)
                .build();
            let is_historic = self
                .commitment_pool_contracts
                .is_historic_commitment(historic_commitment_options)
                .await?;
            (commitment, is_historic)
        };

        if is_historic
            && deposit.data.status != DepositStatus::Queued as i32
            && deposit.data.status != DepositStatus::Included as i32
        {
            if let Some(mut cm) = commitment {
                match cm.data.included_transaction_hash {
                    Some(_) => {
                        deposit.data.status = DepositStatus::Included as i32;
                        cm.data.status = CommitmentStatus::Included as i32;
                    }
                    None => {
                        deposit.data.status = DepositStatus::Queued as i32;
                        cm.data.status = CommitmentStatus::Queued as i32;
                    }
                };
                cm.updated_at = current_timestamp();
                self.db.commitments.update(&cm).await?;
            } else {
                deposit.data.status = DepositStatus::Queued as i32;
            }
            deposit.data.error_message = None;
            deposit.updated_at = current_timestamp();
            self.db.deposits.update(&deposit).await?;
        } else if !is_historic
            && deposit.data.bridge_type == BridgeType::Loop as i32
            && (deposit.data.status == DepositStatus::Queued as i32
                || deposit.data.status == DepositStatus::Included as i32)
        {
            if let Some(mut cm) = commitment {
                cm.data.status = CommitmentStatus::Unspecified as i32;
                cm.updated_at = current_timestamp();
                self.db.commitments.update(&cm).await?;
            }
            deposit.data.status = DepositStatus::Failed as i32;
            deposit.updated_at = current_timestamp();
            self.db.deposits.update(&deposit).await?;
        }
        Ok(deposit)
    }
}
