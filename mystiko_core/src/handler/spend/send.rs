use crate::handler::{format_spend_log, format_tx_hash, spend_circuit_type};
use crate::{
    Commitment, CommitmentColumn, CommitmentPoolContractHandler, PublicAssetHandler, Spend, SpendColumn, SpendContext,
    Spends, SpendsError, TransactOptions, TransactionHandler, TransactionSigner, WaitOptions,
};
use ethers_core::abi::AbiEncode;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Bytes, TxHash};
use mystiko_abi::commitment_pool::TransactRequest;
use mystiko_crypto::zkp::{G16Proof, ZKProver};
use mystiko_protocol::error::ProtocolError;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::SendSpendOptions;
use mystiko_protos::core::v1::{SpendStatus, SpendType, Transaction};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_relayer_client::RelayerClient;
use mystiko_storage::{ColumnValues, Document, DocumentColumn, StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::u256_to_biguint;
use mystiko_utils::hex::{encode_fixed_len_hex_with_prefix, encode_hex_with_prefix};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use typed_builder::TypedBuilder;

impl<F, S, A, C, T, P, R, V> Spends<F, S, A, C, T, P, R, V>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    R: RelayerClient,
    V: ZKProver<G16Proof>,
    ProtocolError: From<V::Error>,
    SpendsError: From<A::Error> + From<C::Error> + From<T::Error> + From<R::Error> + From<V::Error>,
{
    pub(crate) async fn execute_send_with_signer<Signer>(
        &self,
        options: &SendSpendOptions,
        spend: Document<Spend>,
        signer: Arc<Signer>,
    ) -> Result<Document<Spend>, SpendsError>
    where
        Signer: TransactionSigner + 'static,
    {
        let SendContext {
            context,
            mut spend,
            request,
            signature,
            out_commitments,
        } = self.prepare_send(options, spend).await?;
        let chain_config = self
            .config
            .find_chain(context.chain_id)
            .ok_or(SpendsError::UnsupportedChainIdError(context.chain_id))?;
        let contract_address = ethers_address_from_string(context.contract_config.address())?;
        let tx = self
            .transactions
            .create(options.tx.clone(), chain_config.transaction_type())?;
        log::info!("submitting transaction for {}", format_spend_log(&spend));
        let transact_options = TransactOptions::<TypedTransaction, Signer>::builder()
            .chain_id(context.chain_id)
            .contract_address(contract_address)
            .request(request)
            .signature(signature)
            .signer(signer)
            .timeout_ms(options.tx_send_timeout_ms)
            .tx(tx)
            .build();
        let tx_hash = self.commitment_pool_contracts.transact(transact_options).await?;
        let spend_tx_url = format_tx_hash(self.config.clone(), context.chain_id, &tx_hash).unwrap_or_default();
        spend.data.transaction_hash = Some(tx_hash.encode_hex());
        spend = self.update_status(spend, SpendStatus::Pending).await?;
        log::info!(
            "successfully submitted {} for {}",
            spend_tx_url,
            format_spend_log(&spend)
        );
        let spend = self
            .wait_send(&context, options, spend, tx_hash, &out_commitments)
            .await?;
        log::info!(
            "successfully confirmed {} for {}",
            spend_tx_url,
            format_spend_log(&spend)
        );
        Ok(spend)
    }

    pub(crate) async fn execute_send_with_relayer(
        &self,
        options: &SendSpendOptions,
        spend: Document<Spend>,
        relayer_url: &str,
    ) -> Result<Document<Spend>, SpendsError> {
        let SendContext {
            context,
            mut spend,
            request,
            signature,
            out_commitments,
        } = self.prepare_send(options, spend).await?;
        let circuit_type = spend_circuit_type(spend.data.input_commitments.len() as u64, out_commitments.len() as u64)?;
        let transact_data = mystiko_relayer_types::TransactRequestData::builder()
            .chain_id(context.chain_id)
            .pool_address(context.contract_config.address().to_string())
            .asset_symbol(spend.data.asset_symbol.clone())
            .asset_decimals(spend.data.asset_decimals)
            .contract_param(request)
            .signature(signature.encode_hex())
            .spend_type(SpendType::from_i32(spend.data.spend_type).unwrap_or_default())
            .bridge_type(context.contract_config.bridge_type().clone())
            .circuit_type(circuit_type)
            .build();
        let relay_transact_request = mystiko_relayer_types::RelayTransactRequest::builder()
            .relayer_url(relayer_url.to_string())
            .data(transact_data)
            .build();
        log::info!(
            "submitting relay_transact to relayer(url={:?}) for {}",
            relayer_url,
            format_spend_log(&spend)
        );
        let relayer_job = self.relayers.relay_transact(relay_transact_request).await?;
        spend = self.update_status(spend, SpendStatus::Pending).await?;
        log::info!(
            "successfully submitted relay_transact(job_id={:?}) to relayer(url={:?}) for {}",
            relayer_job.uuid,
            relayer_url,
            format_spend_log(&spend)
        );
        let wait_relayer_request = mystiko_relayer_types::WaitingTransactionRequest::builder()
            .relayer_url(relayer_url.to_string())
            .uuid(relayer_job.uuid)
            .waiting_status(mystiko_relayer_types::TransactStatus::Succeeded)
            .timeout(options.relayer_wait_timeout_ms.map(Duration::from_millis))
            .interval(options.relayer_wait_interval_ms.map(Duration::from_millis))
            .build();
        let relayer_job = self.relayers.wait_transaction(wait_relayer_request).await?;
        let tx_hash = relayer_job
            .transaction_hash
            .ok_or(SpendsError::MissingTransactionHashFromRelayerJobError(
                relayer_job.uuid.clone(),
            ))?;
        let tx_hash = TxHash::from_str(&tx_hash)?;
        spend.data.transaction_hash = Some(tx_hash.encode_hex());
        spend = self.update_status(spend, SpendStatus::Pending).await?;
        let spend = self
            .wait_send(&context, options, spend, tx_hash, &out_commitments)
            .await?;
        let spend_tx_url = format_tx_hash(self.config.clone(), context.chain_id, &tx_hash).unwrap_or_default();
        log::info!(
            "successfully confirmed relay_transact(job_id={:?}), {} from relayer(url={:?}) for {}",
            relayer_job.uuid,
            spend_tx_url,
            relayer_url,
            format_spend_log(&spend)
        );
        Ok(spend)
    }

    pub(crate) async fn spend_to_send(&self, options: &SendSpendOptions) -> Result<Document<Spend>, SpendsError> {
        let spend = self
            .db
            .spends
            .find_by_id(&options.spend_id)
            .await?
            .ok_or(SpendsError::SpendNotFoundError(options.spend_id.clone()))?;
        if spend.data.status != SpendStatus::Unspecified as i32 {
            return Err(SpendsError::SpendStatusError(format!(
                "{:?}",
                SpendStatus::from_i32(spend.data.status).unwrap_or_default()
            )));
        }
        Ok(spend)
    }

    pub(crate) async fn handle_send_result(
        &self,
        spend: Document<Spend>,
        result: Result<Document<Spend>, SpendsError>,
    ) -> Result<mystiko_protos::core::document::v1::Spend, SpendsError> {
        match result {
            Ok(spend) => Ok(Spend::document_into_proto(spend)),
            Err(err) => {
                log::error!("failed to send {}: {}", format_spend_log(&spend), err);
                let column_values = ColumnValues::new()
                    .set_value(SpendColumn::Status, SpendStatus::Failed as i32)
                    .set_value(SpendColumn::ErrorMessage, err.to_string());
                self.db
                    .spends
                    .update_by_filter(column_values, SubFilter::equal(DocumentColumn::Id, spend.id))
                    .await?;
                Err(err)
            }
        }
    }

    async fn prepare_send(
        &self,
        options: &SendSpendOptions,
        spend: Document<Spend>,
    ) -> Result<SendContext, SpendsError> {
        let context = SpendContext::from_spend(self.config.clone(), &spend)?;
        let spend = self.update_status(spend, SpendStatus::ProofGenerating).await?;
        log::info!("generating zk proof for {}", format_spend_log(&spend));
        let mut proof_context = self.generate_proof(&context, spend, options).await?;
        let (request, signature) = proof_context.to_request().await?;
        proof_context.spend.data.proof = Some(serde_json::to_string(&proof_context.transaction_proof.proof)?);
        proof_context.spend.data.root_hash = Some(u256_to_biguint(&request.root_hash));
        proof_context.spend.data.output_commitments =
            Some(request.out_commitments.iter().map(u256_to_biguint).collect());
        proof_context.spend.data.nullifiers = Some(request.serial_numbers.iter().map(u256_to_biguint).collect());
        proof_context.spend.data.signature_public_key = Some(encode_fixed_len_hex_with_prefix(request.sig_pk, 20));
        proof_context.spend.data.signature_public_key_hashes =
            Some(request.sig_hashes.iter().map(u256_to_biguint).collect());
        proof_context.spend.data.signature = Some(signature.clone().encode_hex());
        proof_context.spend.data.random_auditing_public_key =
            Some(u256_to_biguint(&request.random_auditing_public_key));
        proof_context.spend.data.encrypted_auditor_notes =
            Some(request.encrypted_auditor_notes.iter().map(u256_to_biguint).collect());
        let spend = self
            .update_status(proof_context.spend, SpendStatus::ProofGenerated)
            .await?;
        log::info!("successfully generated zk proof for {}", format_spend_log(&spend));
        Ok(SendContext::builder()
            .context(context)
            .spend(spend)
            .request(request)
            .signature(signature)
            .out_commitments(proof_context.output_commitments)
            .build())
    }

    async fn wait_send(
        &self,
        context: &SpendContext,
        options: &SendSpendOptions,
        mut spend: Document<Spend>,
        tx_hash: TxHash,
        out_commitments: &[mystiko_protocol::commitment::Commitment],
    ) -> Result<Document<Spend>, SpendsError> {
        let wait_options = WaitOptions::builder()
            .chain_id(spend.data.chain_id)
            .tx_hash(tx_hash)
            .confirmations(options.spend_confirmations)
            .interval_ms(options.tx_wait_interval_ms)
            .timeout_ms(options.tx_wait_timeout_ms)
            .build();
        let tx_receipt = self.transactions.wait(wait_options).await?;
        self.update_input_commitments(context, &spend).await?;
        if let Some(tx_receipt) = tx_receipt {
            if let Some(block_number) = tx_receipt.block_number {
                self.create_out_commitments(context, block_number.as_u64(), out_commitments, &spend)
                    .await?;
            }
        }
        spend = self.update_status(spend, SpendStatus::Succeeded).await?;
        Ok(spend)
    }

    async fn update_input_commitments(
        &self,
        context: &SpendContext,
        spend: &Document<Spend>,
    ) -> Result<(), SpendsError> {
        let filters = Condition::and(vec![
            SubFilter::equal(CommitmentColumn::ChainId, context.chain_id),
            SubFilter::equal(CommitmentColumn::ContractAddress, spend.data.contract_address.clone()),
            SubFilter::in_list(CommitmentColumn::CommitmentHash, spend.data.input_commitments.clone()),
        ]);
        let column_values = ColumnValues::new().set_value(CommitmentColumn::Spent, true);
        self.db.commitments.update_by_filter(column_values, filters).await?;
        Ok(())
    }

    async fn create_out_commitments(
        &self,
        context: &SpendContext,
        block_number: u64,
        out_commitments: &[mystiko_protocol::commitment::Commitment],
        spend: &Document<Spend>,
    ) -> Result<Vec<Document<Commitment>>, SpendsError> {
        let tasks = out_commitments
            .iter()
            .map(|out_commitment| self.create_out_commitment(context, block_number, out_commitment, spend))
            .collect::<Vec<_>>();
        futures::future::try_join_all(tasks).await
    }

    async fn create_out_commitment(
        &self,
        context: &SpendContext,
        block_number: u64,
        out_commitment: &mystiko_protocol::commitment::Commitment,
        spend: &Document<Spend>,
    ) -> Result<Document<Commitment>, SpendsError> {
        let bridge_type: BridgeType = context.contract_config.bridge_type().into();
        let commitment = Commitment {
            chain_id: spend.data.chain_id,
            contract_address: spend.data.contract_address.clone(),
            bridge_type: bridge_type as i32,
            commitment_hash: out_commitment.commitment_hash.clone(),
            asset_symbol: spend.data.asset_symbol.clone(),
            asset_decimals: spend.data.asset_decimals,
            asset_address: context
                .contract_config
                .asset_address()
                .map(|address| address.to_string()),
            status: CommitmentStatus::Queued as i32,
            spent: false,
            block_number,
            src_chain_block_number: None,
            included_block_number: None,
            rollup_fee_amount: spend.data.rollup_fee_decimal_amount.clone(),
            encrypted_note: Some(encode_hex_with_prefix(&out_commitment.encrypted_note)),
            leaf_index: None,
            amount: Some(out_commitment.note.amount.clone()),
            nullifier: None,
            shielded_address: Some(out_commitment.shielded_address.address()),
            queued_transaction_hash: spend.data.transaction_hash.clone(),
            included_transaction_hash: None,
            src_chain_transaction_hash: None,
        };
        Ok(self.db.commitments.insert(&commitment).await?)
    }
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct SendContext {
    pub(crate) context: SpendContext,
    pub(crate) spend: Document<Spend>,
    pub(crate) request: TransactRequest,
    pub(crate) signature: Bytes,
    pub(crate) out_commitments: Vec<mystiko_protocol::commitment::Commitment>,
}
