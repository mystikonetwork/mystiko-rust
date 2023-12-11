use crate::handler::spend_circuit_type;
use crate::{
    Account, AccountColumn, AuditorPublicKeysOptions, BalanceOptions, Commitment, CommitmentColumn,
    CommitmentPoolContractHandler, Erc20BalanceOptions, IsKnownRootOptions, IsSpentNullifierOptions,
    PublicAssetHandler, Spend, SpendContext, Spends, SpendsError, EMPTY_ADDRESS,
};
use ethers_core::types::Bytes;
use ethers_signers::{LocalWallet, Signer};
use mystiko_abi::commitment_pool::TransactRequest;
use mystiko_config::CircuitConfig;
use mystiko_crypto::crypto::decrypt_symmetric;
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_crypto::zkp::{G16Proof, ZKProver, ZKVerifyOptions};
use mystiko_protocol::commitment::EncryptedNote;
use mystiko_protocol::error::ProtocolError;
use mystiko_protocol::key::{
    encryption_public_key, encryption_secret_key, separate_secret_keys, verification_public_key,
    verification_secret_key,
};
use mystiko_protocol::transact::{TransactionCommitmentInput, TransactionCommitmentOutput, TransactionProof};
use mystiko_protocol::types::{AuditingPk, EncPk, EncSk, FullSk, SigPk, VerifyPk, VerifySk, SIG_PK_SIZE};
use mystiko_protos::core::handler::v1::SendSpendOptions;
use mystiko_protos::core::v1::SpendType;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::{Condition, ConditionOperator, Order, OrderBy, QueryFilter, SubFilter};
use mystiko_static_cache::StaticCache;
use mystiko_storage::{Document, StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{biguint_to_u256, decimal_to_number, u256_to_biguint, u256_to_fixed_bytes};
use mystiko_utils::hex::{decode_hex, decode_hex_with_length};
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::{HashMap, HashSet};
use std::ops::Sub;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub(crate) struct ProofContext {
    pub(crate) spend: Document<Spend>,
    pub(crate) output_commitments: Vec<mystiko_protocol::commitment::Commitment>,
    pub(crate) transaction_proof: TransactionProof<G16Proof>,
    pub(crate) sig_wallet: LocalWallet,
}

impl<F, S, A, C, T, P, R, V> Spends<F, S, A, C, T, P, R, V>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    C: CommitmentPoolContractHandler,
    V: ZKProver<G16Proof>,
    ProtocolError: From<V::Error>,
    SpendsError: From<A::Error> + From<C::Error> + From<V::Error>,
{
    pub(crate) async fn generate_proof(
        &self,
        context: &SpendContext,
        spend: Document<Spend>,
        options: &SendSpendOptions,
    ) -> Result<ProofContext, SpendsError> {
        self.validate_pool_balance(context, &spend, options.query_timeout_ms)
            .await?;
        let input_commitments = self
            .validate_input_commitments(context, &spend, options.query_timeout_ms)
            .await?;
        let output_commitments = build_output_commitments(&spend, &input_commitments).await?;
        let merkle_tree = self.build_merkle_tree(context, options.query_timeout_ms).await?;
        let shielded_addresses = input_commitments
            .iter()
            .filter_map(|commitment| commitment.data.shielded_address.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let condition = Condition::and(vec![
            SubFilter::equal(AccountColumn::WalletId, spend.data.wallet_id.clone()),
            SubFilter::in_list(AccountColumn::ShieldedAddress, shielded_addresses),
        ]);
        let accounts = self
            .db
            .accounts
            .find(condition)
            .await?
            .into_iter()
            .map(|account| {
                AccountWithKeys::new(account, &options.wallet_password)
                    .map(|account| (account.account.data.shielded_address.clone(), account))
            })
            .collect::<Result<HashMap<_, _>, SpendsError>>()?;
        let rollup_fee_amount = spend.data.rollup_fee_decimal_amount.clone().unwrap_or_default();
        let proof_inputs = input_commitments
            .iter()
            .map(|commitment| create_zk_commitment_input(commitment, &accounts, &merkle_tree))
            .collect::<Result<Vec<_>, SpendsError>>()?;
        let proof_outputs = output_commitments
            .iter()
            .map(|commitment| create_zk_commitment_output(commitment, &rollup_fee_amount))
            .collect::<Result<Vec<_>, SpendsError>>()?;
        let sig_wallet = LocalWallet::new(&mut rand::thread_rng());
        let sig_pk: SigPk = sig_wallet.address().to_fixed_bytes();
        let public_amount = if spend.data.spend_type == SpendType::Withdraw as i32 {
            spend.data.decimal_amount.clone()
        } else {
            BigUint::zero()
        };
        let num_inputs = input_commitments.len();
        let num_outputs = output_commitments.len();
        let circuit_type = spend_circuit_type(num_inputs as u64, num_outputs as u64)?;
        let circuit = context
            .contract_config
            .circuit_by_type(&circuit_type)
            .ok_or(SpendsError::MissingCircuitTypeInConfigError(circuit_type))?;
        let (program, abi, proving_key, verifying_key) = self.get_circuit_resources(circuit).await?;
        let auditor_keys_options = AuditorPublicKeysOptions::builder()
            .chain_id(spend.data.chain_id)
            .contract_address(ethers_address_from_string(&spend.data.contract_address)?)
            .timeout_ms(options.query_timeout_ms)
            .build();
        let auditor_keys = self
            .commitment_pool_contracts
            .auditor_public_keys(auditor_keys_options)
            .await?
            .iter()
            .map(u256_to_fixed_bytes)
            .collect::<Vec<AuditingPk>>();
        let transaction = mystiko_protocol::transact::Transaction::builder()
            .inputs(proof_inputs)
            .outputs(proof_outputs)
            .sig_pk(sig_pk)
            .tree_root(merkle_tree.root())
            .public_amount(public_amount)
            .relayer_fee_amount(spend.data.gas_relayer_fee_decimal_amount.clone().unwrap_or_default())
            .auditor_public_keys(auditor_keys)
            .program(program)
            .abi(abi)
            .proving_key(proving_key)
            .random_auditing_secret_key(None)
            .build();
        let proof = transaction.prove(self.prover.clone())?;
        let verify_options = ZKVerifyOptions::builder()
            .proof(&proof.proof)
            .verification_key(&verifying_key)
            .build();
        if !self.prover.verify(verify_options)? {
            return Err(SpendsError::InvalidZKProofError);
        }
        Ok(ProofContext::builder()
            .spend(spend)
            .output_commitments(output_commitments)
            .transaction_proof(proof)
            .sig_wallet(sig_wallet)
            .build())
    }
    async fn validate_pool_balance(
        &self,
        context: &SpendContext,
        spend: &Document<Spend>,
        query_timeout_ms: Option<u64>,
    ) -> Result<(), SpendsError> {
        if context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop {
            let owner = ethers_address_from_string(&spend.data.contract_address)?;
            let balance = if let Some(asset_address) = context.contract_config.asset_address() {
                let asset_address = ethers_address_from_string(asset_address)?;
                let balance_options = Erc20BalanceOptions::builder()
                    .chain_id(spend.data.chain_id)
                    .asset_address(asset_address)
                    .timeout_ms(query_timeout_ms)
                    .owner(owner)
                    .build();
                self.assets.erc20_balance_of(balance_options).await?
            } else {
                let balance_options = BalanceOptions::builder()
                    .chain_id(spend.data.chain_id)
                    .owner(owner)
                    .timeout_ms(query_timeout_ms)
                    .build();
                self.assets.balance_of(balance_options).await?
            };
            let balance = u256_to_biguint(&balance);
            if balance.lt(&spend.data.decimal_amount) {
                let balance = decimal_to_number::<f64, _>(&balance, Some(spend.data.asset_decimals))?;
                return Err(SpendsError::InsufficientPoolBalanceError(balance));
            }
        }
        Ok(())
    }

    async fn validate_input_commitments(
        &self,
        context: &SpendContext,
        spend: &Document<Spend>,
        query_timeout_ms: Option<u64>,
    ) -> Result<Vec<Document<Commitment>>, SpendsError> {
        let conditions = Condition::and(vec![
            SubFilter::equal(CommitmentColumn::ChainId, context.chain_id),
            SubFilter::equal(
                CommitmentColumn::ContractAddress,
                context.contract_config.address().to_string(),
            ),
            SubFilter::in_list(CommitmentColumn::CommitmentHash, spend.data.input_commitments.clone()),
        ]);
        let input_commitments = self.db.commitments.find(conditions).await?;
        let tasks = input_commitments
            .iter()
            .map(|commitment| self.validate_input_commitment(context, commitment, query_timeout_ms))
            .collect::<Vec<_>>();
        futures::future::try_join_all(tasks).await?;
        Ok(input_commitments)
    }

    async fn validate_input_commitment(
        &self,
        context: &SpendContext,
        commitment: &Document<Commitment>,
        query_timeout_ms: Option<u64>,
    ) -> Result<(), SpendsError> {
        let contract_address = ethers_address_from_string(context.contract_config.address())?;
        if let Some(nullifier) = commitment.data.nullifier.clone() {
            let options = IsSpentNullifierOptions::builder()
                .chain_id(context.chain_id)
                .contract_address(contract_address)
                .nullifier(biguint_to_u256(&nullifier))
                .timeout_ms(query_timeout_ms)
                .build();
            if self.commitment_pool_contracts.is_spent_nullifier(options).await? {
                return Err(SpendsError::AlreadySpentCommitmentError(
                    commitment.data.commitment_hash.to_string(),
                ));
            }
        }
        Ok(())
    }

    async fn build_merkle_tree(
        &self,
        context: &SpendContext,
        query_timeout_ms: Option<u64>,
    ) -> Result<MerkleTree, SpendsError> {
        let condition = Condition::and(vec![
            SubFilter::equal(CommitmentColumn::ChainId, context.chain_id),
            SubFilter::equal(
                CommitmentColumn::ContractAddress,
                context.contract_config.address().to_string(),
            ),
            SubFilter::equal(CommitmentColumn::Status, CommitmentStatus::Included as i32),
            SubFilter::is_not_null(CommitmentColumn::LeafIndex),
        ]);
        let order_by = OrderBy::builder()
            .order(Order::Asc)
            .columns(vec![CommitmentColumn::LeafIndex.to_string()])
            .build();
        let filter = QueryFilter::builder()
            .conditions(vec![condition])
            .conditions_operator(ConditionOperator::And)
            .order_by(order_by)
            .build();
        let commitments = self.db.commitments.find(filter).await?;
        let leaves = commitments
            .into_iter()
            .map(|commitment| commitment.data.commitment_hash)
            .collect::<Vec<_>>();
        let merkle_tree = MerkleTree::new(Some(leaves), None, None)?;
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
        Ok(merkle_tree)
    }

    async fn get_circuit_resources(
        &self,
        circuit_config: &CircuitConfig,
    ) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), SpendsError> {
        log::info!(
            "fetching resources for circuit_type={:?}",
            circuit_config.circuit_type()
        );
        let resource_urls = vec![
            circuit_config.program_file().clone(),
            circuit_config.abi_file().clone(),
            circuit_config.proving_key_file().clone(),
            circuit_config.verifying_key_file().clone(),
        ];
        let tasks = resource_urls
            .iter()
            .map(|urls| self.static_cache.get_failover(urls, None))
            .collect::<Vec<_>>();
        let mut files = futures::future::try_join_all(tasks).await?;
        let program = files.remove(0);
        let abi = files.remove(0);
        let proving_key = files.remove(0);
        let verifying_key = files.remove(0);
        log::info!(
            "successfully fetched resources for circuit_type={:?}",
            circuit_config.circuit_type()
        );
        Ok((program, abi, proving_key, verifying_key))
    }
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct AccountWithKeys {
    pub(crate) account: Document<Account>,
    pub(crate) verify_pk: VerifyPk,
    pub(crate) verify_sk: VerifySk,
    pub(crate) enc_pk: EncPk,
    pub(crate) enc_sk: EncSk,
}

impl ProofContext {
    pub(crate) async fn to_request(&self) -> Result<(TransactRequest, Bytes), SpendsError> {
        let public_recipient = if self.spend.data.spend_type == SpendType::Transfer as i32 {
            EMPTY_ADDRESS.to_string()
        } else {
            self.spend.data.recipient.clone()
        };
        let relayer_address = self
            .spend
            .data
            .gas_relayer_address
            .clone()
            .unwrap_or(EMPTY_ADDRESS.to_string());
        let out_encrypted_notes = self
            .output_commitments
            .iter()
            .map(|commitment| commitment.encrypted_note.clone())
            .collect::<Vec<_>>();
        let mut sig_pk = [0_u8; 32];
        sig_pk[(32 - SIG_PK_SIZE)..].copy_from_slice(&self.transaction_proof.zk_input.sig_public_key);
        let request = TransactRequest {
            proof: self.transaction_proof.proof.convert_to()?,
            root_hash: biguint_to_u256(&self.transaction_proof.zk_input.tree_root),
            serial_numbers: self
                .transaction_proof
                .zk_input
                .in_nullifiers
                .iter()
                .map(biguint_to_u256)
                .collect(),
            sig_hashes: self
                .transaction_proof
                .zk_input
                .in_sig_hashes
                .iter()
                .map(biguint_to_u256)
                .collect(),
            sig_pk,
            public_amount: biguint_to_u256(&self.transaction_proof.zk_input.public_amount),
            relayer_fee_amount: biguint_to_u256(&self.transaction_proof.zk_input.relayer_fee_amount),
            out_commitments: self
                .transaction_proof
                .zk_input
                .out_commitments
                .iter()
                .map(biguint_to_u256)
                .collect(),
            out_rollup_fees: self
                .transaction_proof
                .zk_input
                .out_rollup_fee_amounts
                .iter()
                .map(biguint_to_u256)
                .collect(),
            public_recipient: ethers_address_from_string(&public_recipient)?,
            relayer_address: ethers_address_from_string(&relayer_address)?,
            out_encrypted_notes: out_encrypted_notes
                .iter()
                .map(|note| Bytes::from(note.clone()))
                .collect(),
            random_auditing_public_key: biguint_to_u256(&self.transaction_proof.zk_input.random_public_key),
            encrypted_auditor_notes: self
                .transaction_proof
                .zk_input
                .encrypted_commitment_shares
                .iter()
                .flatten()
                .map(biguint_to_u256)
                .collect(),
        };
        let body = vec![
            decode_hex(public_recipient)?,
            decode_hex(relayer_address)?,
            out_encrypted_notes.into_iter().flatten().collect::<Vec<_>>(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        let hash = ethers_core::utils::keccak256(&body);
        let signature = self.sig_wallet.sign_message(hash).await?;
        Ok((request, Bytes::from(signature.to_vec())))
    }
}

impl AccountWithKeys {
    pub(crate) fn new(account: Document<Account>, wallet_password: &str) -> Result<Self, SpendsError> {
        let full_secret_key = decrypt_symmetric(wallet_password, &account.data.encrypted_secret_key)?;
        let full_secret_key: FullSk = decode_hex_with_length(full_secret_key)?;
        let (verify_sk, enc_sk) = separate_secret_keys(&full_secret_key)?;
        Ok(Self::builder()
            .account(account)
            .verify_pk(verification_public_key(&verify_sk)?)
            .verify_sk(verification_secret_key(&verify_sk)?)
            .enc_pk(encryption_public_key(&enc_sk)?)
            .enc_sk(encryption_secret_key(&enc_sk))
            .build())
    }
}

async fn build_output_commitments(
    spend: &Document<Spend>,
    input_commitments: &[Document<Commitment>],
) -> Result<Vec<mystiko_protocol::commitment::Commitment>, SpendsError> {
    let shielded_address = input_commitments
        .first()
        .and_then(|commitment| commitment.data.shielded_address.clone());
    let mut output_commitments = vec![];
    if let Some(shielded_address) = shielded_address {
        let input_sum = input_commitments
            .iter()
            .filter_map(|commitment| commitment.data.amount.clone())
            .sum::<BigUint>();
        let amount = spend.data.decimal_amount.clone();
        let remaining = input_sum.sub(&amount);
        if remaining.gt(&BigUint::zero()) {
            output_commitments.push(build_output_commitment(&shielded_address, remaining)?);
        }
        if spend.data.spend_type == SpendType::Transfer as i32 {
            let output_commitment = build_output_commitment(&spend.data.recipient, amount)?;
            output_commitments.push(output_commitment);
        }
    }
    Ok(output_commitments)
}

fn build_output_commitment(
    shielded_address: &str,
    amount: BigUint,
) -> Result<mystiko_protocol::commitment::Commitment, SpendsError> {
    Ok(mystiko_protocol::commitment::Commitment::new(
        mystiko_protocol::address::ShieldedAddress::from_string(shielded_address)?,
        Some(mystiko_protocol::commitment::Note::new(Some(amount), None)?),
        None,
    )?)
}

fn create_zk_commitment_input(
    commitment: &Document<Commitment>,
    accounts: &HashMap<String, AccountWithKeys>,
    merkle_tree: &MerkleTree,
) -> Result<TransactionCommitmentInput, SpendsError> {
    let shielded_address =
        commitment
            .data
            .shielded_address
            .clone()
            .ok_or(SpendsError::MissingShieldedAddressInCommitmentError(
                commitment.id.clone(),
            ))?;
    let encrypted_note =
        commitment
            .data
            .encrypted_note
            .clone()
            .ok_or(SpendsError::MissingEncryptedNoteInCommitmentError(
                commitment.id.clone(),
            ))?;
    let encrypted_note: EncryptedNote = decode_hex(encrypted_note)?;
    let account = accounts
        .get(&shielded_address)
        .ok_or(SpendsError::NonOwnedShieldedAddressError(shielded_address.clone()))?;
    let path_index = merkle_tree.index_of(&commitment.data.commitment_hash, None).ok_or(
        SpendsError::MissingCommitmentInMerkleTree(commitment.data.commitment_hash.to_string()),
    )?;
    let (path_elements, path_indices) = merkle_tree.path(path_index)?;
    Ok(TransactionCommitmentInput::builder()
        .pk_verify(account.verify_pk)
        .sk_verify(account.verify_sk)
        .pk_enc(account.enc_pk)
        .sk_enc(account.enc_sk)
        .private_note(encrypted_note)
        .commitment(commitment.data.commitment_hash.clone())
        .path_indices(path_indices)
        .path_elements(path_elements)
        .build())
}

fn create_zk_commitment_output(
    commitment: &mystiko_protocol::commitment::Commitment,
    rollup_fee_amount: &BigUint,
) -> Result<TransactionCommitmentOutput, SpendsError> {
    let (pk_verify, _) = commitment.shielded_address.public_key()?;
    Ok(TransactionCommitmentOutput::builder()
        .commitment(commitment.commitment_hash.clone())
        .pk_verify(pk_verify)
        .amount(commitment.note.amount.clone())
        .rollup_fee_amount(rollup_fee_amount.clone())
        .random_s(commitment.note.random_s)
        .random_r(commitment.note.random_r)
        .random_p(commitment.note.random_p)
        .build())
}
