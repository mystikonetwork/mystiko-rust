use crate::{format_deposit_log, format_tx_hash, DepositContext};
use crate::{
    AccountHandler, Commitment, CommitmentPoolContractHandler, Deposit, DepositContractHandler, Deposits,
    DepositsError, IsHistoricCommitmentOptions, PublicAssetHandler, TransactionHandler, TransactionSigner, WaitOptions,
};
use ethers_core::abi::AbiEncode;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, Bytes, TxHash};
use mystiko_abi::mystiko_v2_bridge::DepositRequest as CrossChainDepositRequest;
use mystiko_abi::mystiko_v2_loop::DepositRequest;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::SendDepositOptions;
use mystiko_protos::core::v1::{DepositStatus, Transaction};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_storage::{Document, StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{biguint_to_u128, biguint_to_u256, number_to_u256_decimal};
use std::str::FromStr;
use std::sync::Arc;

impl<F, S, A, D, C, T, P> Deposits<F, S, A, D, C, T, P>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContractHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
{
    pub(crate) async fn execute_send<Signer>(
        &self,
        options: &SendDepositOptions,
        deposit: Document<Deposit>,
        signer: Arc<Signer>,
        owner: Address,
    ) -> Result<Document<Deposit>, DepositsError>
    where
        Signer: TransactionSigner + 'static,
    {
        let context = DepositContext::from_deposit(self.config.clone(), &deposit)?;
        self.validate_deposit(&context, &deposit, options).await?;
        let deposit = self
            .execute_assets_approve(&context, options, deposit, signer.clone(), owner)
            .await?;
        let deposit = self.send_deposit(&context, options, deposit, signer.clone()).await?;
        Ok(deposit)
    }

    async fn send_deposit<Signer>(
        &self,
        context: &DepositContext,
        options: &SendDepositOptions,
        mut deposit: Document<Deposit>,
        signer: Arc<Signer>,
    ) -> Result<Document<Deposit>, DepositsError>
    where
        Signer: TransactionSigner + 'static,
    {
        let send_tx_hash = self
            .send_deposit_transaction(context, &mut deposit, options, signer.clone())
            .await?;
        let send_tx_url = format_tx_hash(self.config.clone(), deposit.data.chain_id, &send_tx_hash).unwrap_or_default();
        deposit = self.db.deposits.update(&deposit).await?;
        log::info!(
            "successfully submitted {} for {}",
            send_tx_url,
            format_deposit_log(&deposit),
        );
        let wait_options = WaitOptions::builder()
            .chain_id(deposit.data.chain_id)
            .tx_hash(send_tx_hash)
            .confirmations(options.deposit_confirmations)
            .interval_ms(options.tx_wait_interval_ms)
            .timeout_ms(options.tx_wait_timeout_ms)
            .build();
        let tx_receipt = self.transactions.wait(wait_options).await?;
        if context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop {
            deposit.data.status = DepositStatus::SrcSucceeded as i32;
        } else {
            deposit.data.status = DepositStatus::Queued as i32;
        }
        deposit = self.db.deposits.update(&deposit).await?;
        log::info!(
            "successfully confirmed {} for {}",
            send_tx_url,
            format_deposit_log(&deposit),
        );
        if let Some(tx_receipt) = tx_receipt {
            if let Some(block_number) = tx_receipt.block_number {
                self.create_commitment(block_number.as_u64(), &deposit).await?;
            }
        }
        Ok(deposit)
    }

    async fn send_deposit_transaction<Signer>(
        &self,
        context: &DepositContext,
        deposit: &mut Document<Deposit>,
        options: &SendDepositOptions,
        signer: Arc<Signer>,
    ) -> Result<TxHash, DepositsError>
    where
        D: DepositContractHandler,
        T: TransactionHandler<Transaction>,
        Signer: TransactionSigner + 'static,
        DepositsError: From<D::Error> + From<T::Error>,
    {
        let tx = self
            .transactions
            .create(options.deposit_tx.clone(), context.chain_config.transaction_type())?;
        let contract_address = ethers_address_from_string(context.contract_config.address())?;
        let asset_decimals = Some(context.contract_config.asset_decimals());
        let amount = number_to_u256_decimal(deposit.data.amount, asset_decimals)?;
        let rollup_fee_amount = number_to_u256_decimal(deposit.data.rollup_fee_amount, asset_decimals)?;
        let commitment = biguint_to_u256(&deposit.data.commitment_hash);
        let hash_k = biguint_to_u256(&deposit.data.hash_k);
        let random_s = biguint_to_u128(&deposit.data.random_s);
        let encrypted_notes = Bytes::from_str(&deposit.data.encrypted_note)?;
        if context.contract_config.bridge_type() == &mystiko_types::BridgeType::Loop {
            let request = DepositRequest {
                amount,
                commitment,
                hash_k,
                random_s,
                encrypted_note: encrypted_notes,
                rollup_fee: rollup_fee_amount,
            };
            let options = crate::DepositOptions::<TypedTransaction, Signer>::builder()
                .chain_id(deposit.data.chain_id)
                .contract_address(contract_address)
                .request(request)
                .tx(tx)
                .signer(signer)
                .timeout_ms(options.tx_send_timeout_ms)
                .build();
            let tx_hash = self.deposit_contracts.deposit(options).await?;
            deposit.data.queued_transaction_hash = Some(tx_hash.encode_hex());
            deposit.data.status = DepositStatus::SrcPending as i32;
            Ok(tx_hash)
        } else {
            let bridge_fee_amount = number_to_u256_decimal(
                deposit.data.bridge_fee_amount.unwrap_or_default(),
                Some(context.contract_config.bridge_fee_asset().asset_decimals()),
            )?;
            let executor_fee_amount = number_to_u256_decimal(
                deposit.data.executor_fee_amount.unwrap_or_default(),
                Some(context.contract_config.executor_fee_asset().asset_decimals()),
            )?;
            let request = CrossChainDepositRequest {
                amount,
                commitment,
                hash_k,
                random_s,
                encrypted_note: encrypted_notes,
                rollup_fee: rollup_fee_amount,
                bridge_fee: bridge_fee_amount,
                executor_fee: executor_fee_amount,
            };
            let options = crate::CrossChainDepositOptions::<TypedTransaction, Signer>::builder()
                .chain_id(deposit.data.chain_id)
                .contract_address(contract_address)
                .request(request)
                .tx(tx)
                .signer(signer)
                .timeout_ms(options.tx_send_timeout_ms)
                .build();
            let tx_hash = self.deposit_contracts.cross_chain_deposit(options).await?;
            deposit.data.src_chain_transaction_hash = Some(tx_hash.encode_hex());
            deposit.data.status = DepositStatus::SrcPending as i32;
            Ok(tx_hash)
        }
    }

    async fn create_commitment(
        &self,
        block_number: u64,
        deposit: &Document<Deposit>,
    ) -> Result<Option<Document<Commitment>>, DepositsError> {
        let account = self
            .accounts
            .find_by_shielded_address(&deposit.data.shielded_address)
            .await?;
        if account.is_some() {
            let commitment = create_commitment(block_number, deposit)?;
            let commitment = self.db.commitments.insert(&commitment).await?;
            log::info!("successfully inserted a new commitment(id={})", commitment.id);
            return Ok(Some(commitment));
        }
        Ok(None)
    }

    async fn validate_deposit(
        &self,
        context: &DepositContext,
        deposit: &Document<Deposit>,
        options: &SendDepositOptions,
    ) -> Result<(), DepositsError> {
        if deposit.data.status != DepositStatus::Unspecified as i32 {
            return Err(DepositsError::DepositStatusError(format!(
                "{:?}",
                DepositStatus::from_i32(deposit.data.status).unwrap_or_default()
            )));
        }
        let (chain_id, pool_contract_address) = if let Some(peer_contract_config) = &context.peer_contract_config {
            (deposit.data.dst_chain_id, peer_contract_config.pool_contract_address())
        } else {
            (deposit.data.chain_id, context.contract_config.pool_contract_address())
        };
        let contract_address = ethers_address_from_string(pool_contract_address)?;
        let is_historic_commitment_options = IsHistoricCommitmentOptions::builder()
            .chain_id(chain_id)
            .contract_address(contract_address)
            .commitment_hash(biguint_to_u256(&deposit.data.commitment_hash))
            .timeout_ms(options.query_timeout_ms)
            .build();
        if self
            .commitment_pool_contracts
            .is_historic_commitment(is_historic_commitment_options)
            .await?
        {
            return Err(DepositsError::DuplicateCommitmentError(
                deposit.data.commitment_hash.to_string(),
                chain_id,
                pool_contract_address.to_string(),
            ));
        }
        Ok(())
    }
}

fn create_commitment(block_number: u64, deposit: &Document<Deposit>) -> Result<Commitment, DepositsError> {
    let (contract_address, status, src_chain_block_number) = if deposit.data.bridge_type == BridgeType::Loop as i32 {
        (deposit.data.pool_address.clone(), CommitmentStatus::Queued as i32, None)
    } else {
        (
            deposit.data.contract_address.clone(),
            CommitmentStatus::SrcSucceeded as i32,
            Some(block_number),
        )
    };
    Ok(Commitment {
        chain_id: deposit.data.chain_id,
        contract_address,
        bridge_type: deposit.data.bridge_type,
        commitment_hash: deposit.data.commitment_hash.clone(),
        asset_symbol: deposit.data.asset_symbol.clone(),
        asset_decimals: deposit.data.asset_decimals,
        asset_address: deposit.data.asset_address.clone(),
        status,
        spent: false,
        block_number,
        src_chain_block_number,
        included_block_number: None,
        rollup_fee_amount: Some(deposit.data.rollup_fee_decimal_amount.clone()),
        encrypted_note: Some(deposit.data.encrypted_note.clone()),
        leaf_index: None,
        amount: Some(deposit.data.decimal_amount.clone()),
        nullifier: None,
        shielded_address: Some(deposit.data.shielded_address.clone()),
        queued_transaction_hash: deposit.data.queued_transaction_hash.clone(),
        included_transaction_hash: None,
        src_chain_transaction_hash: deposit.data.src_chain_transaction_hash.clone(),
    })
}
