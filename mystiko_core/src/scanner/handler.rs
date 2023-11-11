use crate::{
    Account, Accounts, Commitment, CommitmentColumn, Database, Nullifier, NullifierColumn, ScannerError,
    ScannerHandler, WalletHandler, Wallets,
};
use async_trait::async_trait;
use mystiko_crypto::crypto::decrypt_symmetric;
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::commitment::{Commitment as ProtocolCommitment, EncryptedData, EncryptedNote};
use mystiko_protocol::key::separate_secret_keys;
use mystiko_protocol::types::{EncSk, FullSk, VerifySk};
use mystiko_protocol::utils::compute_nullifier;
use mystiko_protos::core::scanner::v1::{
    Balance, BalanceOptions, BalanceResult, ResetOptions, ResetResult, ScanOptions, ScanResult,
};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::{Condition, ConditionOperator, Order, OrderBy, QueryFilter, SubFilter};
use mystiko_storage::{Document, DocumentColumn, StatementFormatter, Storage};
use mystiko_utils::convert::decimal_to_number;
use mystiko_utils::hex::{decode_hex, decode_hex_with_length};
use num_bigint::BigUint;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use typed_builder::TypedBuilder;

const DEFAULT_BATCH_SIZE: u64 = 10000;
const DEFAULT_MAX_QUERY_FILTER_SIZE: u64 = 1000;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Scanner<F: StatementFormatter, S: Storage> {
    db: Arc<Database<F, S>>,
    wallets: Wallets<F, S>,
    accounts: Accounts<F, S>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ScannerOptions<F: StatementFormatter, S: Storage> {
    db: Arc<Database<F, S>>,
}

#[derive(Debug, Clone, TypedBuilder)]
struct ScanAccount {
    shielded_address: ShieldedAddress,
    enc_sk: EncSk,
    v_sk: VerifySk,
}

#[derive(Debug, Clone, TypedBuilder)]
struct ScanRoundParams {
    concurrency: u32,
    scan_batch_size: u64,
    max_query_filter_size: u64,
    start_id: Option<String>,
    accounts: Vec<ScanAccount>,
}

#[derive(Debug, Clone, TypedBuilder)]
struct ScanRoundResult {
    scanned: u64,
    owned: u64,
    end_id: String,
}

#[derive(Debug, Clone, TypedBuilder)]
struct ScanBatchResult {
    scanned: u64,
    updated_commitments: Vec<Document<Commitment>>,
}

impl<F, S> Scanner<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    pub fn new(options: ScannerOptions<F, S>) -> Self {
        Scanner::<F, S>::builder()
            .wallets(Wallets::<F, S>::new(options.db.clone()))
            .accounts(Accounts::<F, S>::new(options.db.clone()))
            .db(options.db)
            .build()
    }
}

#[async_trait]
impl<F, S> ScannerHandler<ScanOptions, ScanResult, ResetOptions, ResetResult, BalanceOptions, BalanceResult>
    for Scanner<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    type Error = ScannerError;

    async fn scan(&self, options: ScanOptions) -> Result<ScanResult, Self::Error> {
        self.wallets.check_password(&options.wallet_password).await?;
        let accounts = self.build_filter_accounts(&options.shielded_addresses).await?;
        let scanned_id = accounts
            .iter()
            .filter_map(|account| account.data.scanned_to_id.as_ref())
            .min()
            .cloned();
        let cms_count = self.query_commitment_count(scanned_id.clone()).await?;
        if cms_count == 0 {
            return Ok(ScanResult::builder()
                .scanned_shielded_addresses(
                    accounts
                        .iter()
                        .map(|account| account.data.shielded_address.clone())
                        .collect::<Vec<_>>(),
                )
                .build());
        }

        let scan_accounts = self.build_scan_accounts(&accounts, &options.wallet_password).await?;
        let batch_size = options.batch_size.unwrap_or(DEFAULT_BATCH_SIZE) / scan_accounts.len() as u64;
        let batch_size = std::cmp::max(1, batch_size);
        let concurrency = std::cmp::max(1, options.concurrency.unwrap_or(1));
        let scan_batch_size = batch_size * (concurrency as u64);
        let total_round = (cms_count + scan_batch_size - 1) / scan_batch_size;
        let (mut total_scanned, mut total_owned) = (0, 0);
        let mut current_scanned_id = scanned_id;
        for _ in 0..total_round {
            let params = ScanRoundParams::builder()
                .concurrency(concurrency)
                .scan_batch_size(scan_batch_size)
                .max_query_filter_size(DEFAULT_MAX_QUERY_FILTER_SIZE)
                .start_id(current_scanned_id.clone())
                .accounts(scan_accounts.clone())
                .build();
            let result = self.scan_one_round(&params).await?;
            current_scanned_id = Some(result.end_id);
            total_owned += result.owned;
            total_scanned += result.scanned;
        }

        Ok(ScanResult::builder()
            .total_count(total_scanned)
            .owned_count(total_owned)
            .scanned_shielded_addresses(
                scan_accounts
                    .iter()
                    .map(|account| account.shielded_address.address())
                    .collect::<Vec<_>>(),
            )
            .to_id(current_scanned_id)
            .build())
    }

    async fn reset(&self, options: ResetOptions) -> Result<ResetResult, Self::Error> {
        let mut accounts = self.build_filter_accounts(&options.shielded_addresses).await?;
        accounts.iter_mut().for_each(|account| {
            account.data.scanned_to_id = options.reset_to_id.clone();
        });
        self.db.accounts.update_batch(&accounts).await?;
        Ok(ResetResult::default())
    }

    async fn balance(&self, options: BalanceOptions) -> Result<BalanceResult, Self::Error> {
        let shielded_addresses = if !options.shielded_addresses.is_empty() {
            options.shielded_addresses
        } else {
            let accounts = self.build_filter_accounts(&[]).await?;
            accounts
                .iter()
                .map(|account| account.data.shielded_address.clone())
                .collect::<Vec<_>>()
        };

        let mut filters = vec![
            SubFilter::is_not_null(CommitmentColumn::Amount),
            SubFilter::is_not_null(CommitmentColumn::Nullifier),
            SubFilter::in_list(CommitmentColumn::ShieldedAddress, shielded_addresses),
        ];

        if options.with_spent != Some(true) {
            filters.push(SubFilter::equal(CommitmentColumn::Spent, false));
        }

        if !options.chain_ids.is_empty() {
            let sub_filter = SubFilter::in_list(CommitmentColumn::ChainId, options.chain_ids.to_vec());
            filters.push(sub_filter);
        }

        if !options.asset_symbols.is_empty() {
            let sub_filter = SubFilter::in_list(
                CommitmentColumn::AssetSymbol,
                options
                    .asset_symbols
                    .iter()
                    .map(|asset_symbol| asset_symbol.to_string())
                    .collect::<Vec<_>>(),
            );
            filters.push(sub_filter);
        }

        if !options.contract_addresses.is_empty() {
            let sub_filter = SubFilter::in_list(
                CommitmentColumn::ContractAddress,
                options
                    .contract_addresses
                    .iter()
                    .map(|contract_address| contract_address.to_string())
                    .collect::<Vec<_>>(),
            );
            filters.push(sub_filter);
        }

        if !options.bridge_types.is_empty() {
            let sub_filter = SubFilter::in_list(CommitmentColumn::BridgeType, options.bridge_types.to_vec());
            filters.push(sub_filter);
        }

        let condition = Condition::from(filters);
        let commitments = self.db.commitments.find(condition).await?;
        let assets = commitments
            .iter()
            .map(|commitment| commitment.data.asset_symbol.clone())
            .collect::<HashSet<_>>();

        let mut balances = Vec::with_capacity(assets.len());
        for asset_symbol in assets {
            let balance = self.calc_balance_by_asset_symbol(&commitments, &asset_symbol, options.with_spent)?;
            balances.push(balance);
        }
        Ok(BalanceResult::builder().balances(balances).build())
    }
}

impl<F, S> Scanner<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    async fn build_filter_accounts(
        &self,
        shielded_addresses: &[String],
    ) -> Result<Vec<Document<Account>>, ScannerError> {
        let mut accounts = self.accounts.find_all_documents().await?;
        if !shielded_addresses.is_empty() {
            accounts.retain(|account| shielded_addresses.contains(&account.data.shielded_address));
        }

        if accounts.is_empty() {
            return Err(ScannerError::NoAccountFoundError);
        }
        Ok(accounts)
    }

    async fn build_scan_accounts(
        &self,
        accounts: &[Document<Account>],
        password: &str,
    ) -> Result<Vec<ScanAccount>, ScannerError> {
        let mut scan_accounts = Vec::new();
        for account in accounts {
            let shielded_address = ShieldedAddress::from_string(&account.data.shielded_address)?;
            let secret_key = decrypt_symmetric(password, &account.data.encrypted_secret_key)?;
            let secret_key_bytes: FullSk = decode_hex_with_length(secret_key)?;
            let (v_sk, enc_sk) = separate_secret_keys(&secret_key_bytes);
            scan_accounts.push(
                ScanAccount::builder()
                    .shielded_address(shielded_address)
                    .enc_sk(enc_sk)
                    .v_sk(v_sk)
                    .build(),
            );
        }
        Ok(scan_accounts)
    }

    async fn scan_one_round(&self, params: &ScanRoundParams) -> Result<ScanRoundResult, ScannerError> {
        let commitments = self.query_commitments(params).await?;
        let end_id = commitments.last().ok_or(ScannerError::CommitmentEmptyError)?.id.clone();
        let chunk_size = std::cmp::max(1, commitments.len() / params.concurrency as usize);
        let tasks = commitments
            .chunks(chunk_size)
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| {
                let chunk = chunk.to_owned();
                let accounts = params.accounts.clone();
                tokio::spawn(async move { scan_commitments(chunk, accounts).await })
            });
        let results = futures::future::try_join_all(tasks).await?;
        let (scanned, updated) =
            results
                .into_iter()
                .try_fold((0, Vec::new()), |(acc_scanned, mut acc_updated), result| {
                    let scan = result?;
                    acc_updated.extend(scan.updated_commitments);
                    Ok::<(u64, Vec<_>), ScannerError>((acc_scanned + scan.scanned, acc_updated))
                })?;

        let mut owned = 0;
        if !updated.is_empty() {
            let status_updated = self.update_commitments_spent_status(updated, params).await?;
            owned = status_updated.len() as u64;
            self.db.commitments.update_batch(&status_updated).await?;
        }
        Ok(ScanRoundResult::builder()
            .scanned(scanned)
            .owned(owned)
            .end_id(end_id)
            .build())
    }

    async fn update_commitments_spent_status(
        &self,
        commitments: Vec<Document<Commitment>>,
        params: &ScanRoundParams,
    ) -> Result<Vec<Document<Commitment>>, ScannerError> {
        let chunk_size = std::cmp::max(1, params.max_query_filter_size) as usize;
        let tasks = commitments
            .chunks(chunk_size)
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| {
                let chunk = chunk.to_owned();
                self.update_batch_commitments_spent_status(chunk)
            });
        let results = futures::future::try_join_all(tasks).await?;
        results.into_iter().try_fold(Vec::new(), |mut acc, result| {
            acc.extend(result);
            Ok::<Vec<_>, ScannerError>(acc)
        })
    }

    async fn update_batch_commitments_spent_status(
        &self,
        mut commitments: Vec<Document<Commitment>>,
    ) -> Result<Vec<Document<Commitment>>, ScannerError> {
        let filter_nullifiers: Vec<_> = commitments
            .iter()
            .filter_map(|commitment| commitment.data.nullifier.as_ref())
            .cloned()
            .collect();

        let nullifiers = self.query_nullifiers(filter_nullifiers).await?;
        if !nullifiers.is_empty() {
            for commitment in commitments.iter_mut() {
                if let Some(data_nullifier) = &commitment.data.nullifier {
                    if let Some(db_nullifier) = nullifiers.get(data_nullifier) {
                        if db_nullifier.data.chain_id == commitment.data.chain_id
                            && db_nullifier.data.contract_address == commitment.data.contract_address
                        {
                            commitment.data.status = CommitmentStatus::Included as i32;
                            commitment.data.spent = true;
                        }
                    }
                }
            }
        }

        Ok(commitments)
    }

    async fn query_commitment_count(&self, scanned_id: Option<String>) -> Result<u64, ScannerError> {
        let total = if let Some(scanned) = scanned_id {
            self.db
                .commitments
                .count(SubFilter::greater(DocumentColumn::Id, scanned.clone()))
                .await?
        } else {
            self.db.commitments.count_all().await?
        };
        Ok(total)
    }

    async fn query_commitments(&self, params: &ScanRoundParams) -> Result<Vec<Document<Commitment>>, ScannerError> {
        let order = OrderBy::builder()
            .columns(vec![DocumentColumn::Id.to_string()])
            .order(Order::Asc)
            .build();

        let filter = if let Some(start) = &params.start_id {
            QueryFilter::builder()
                .limit(params.scan_batch_size)
                .order_by(order)
                .conditions(vec![Condition::from(SubFilter::greater(
                    DocumentColumn::Id,
                    start.clone(),
                ))])
                .conditions_operator(ConditionOperator::And as i32)
                .build()
        } else {
            QueryFilter::builder()
                .limit(params.scan_batch_size)
                .order_by(order)
                .build()
        };
        let commitments = self.db.commitments.find(filter).await?;
        Ok(commitments)
    }

    async fn query_nullifiers(
        &self,
        filter_nullifiers: Vec<BigUint>,
    ) -> Result<HashMap<BigUint, Document<Nullifier>>, ScannerError> {
        if !filter_nullifiers.is_empty() {
            let condition = Condition::from(SubFilter::in_list(NullifierColumn::Nullifier, filter_nullifiers));
            let nullifiers = self.db.nullifiers.find(condition).await?;
            Ok(nullifiers
                .into_iter()
                .map(|nullifier| (nullifier.data.nullifier.clone(), nullifier))
                .collect::<HashMap<_, _>>())
        } else {
            Ok(HashMap::new())
        }
    }

    fn calc_balance_by_asset_symbol(
        &self,
        commitments: &[Document<Commitment>],
        asset_symbol: &str,
        with_spent: Option<bool>,
    ) -> Result<Balance, ScannerError> {
        let default_amount = &BigUint::default();
        let (pending, unspent, spent) = commitments
            .iter()
            .filter(|commitment| commitment.data.asset_symbol == asset_symbol)
            .try_fold(
                (0f64, 0f64, 0f64),
                |(mut pending, mut unspent, mut spent), commitment| {
                    let amount = decimal_to_number::<f64, BigUint>(
                        commitment.data.amount.as_ref().unwrap_or(default_amount),
                        Some(commitment.data.asset_decimals),
                    )?;
                    match commitment.data {
                        Commitment { spent: true, .. } => spent += amount,
                        Commitment { status, .. } if status == CommitmentStatus::Included as i32 => unspent += amount,
                        _ => pending += amount,
                    };
                    Ok::<(f64, f64, f64), ScannerError>((pending, unspent, spent))
                },
            )?;

        let spent_option = with_spent.filter(|&b| b).map(|_| spent);
        Ok(Balance::builder()
            .asset_symbol(asset_symbol.to_string())
            .pending(pending)
            .unspent(unspent)
            .spent(spent_option)
            .build())
    }
}

async fn scan_commitments(
    commitments: Vec<Document<Commitment>>,
    accounts: Vec<ScanAccount>,
) -> Result<ScanBatchResult, ScannerError> {
    let total = commitments.len() as u64;
    let mut updated_commitments = Vec::new();
    for commitment in commitments {
        let updated = scan_commitment_by_accounts(commitment, &accounts).await?;
        if let Some(updated) = updated {
            updated_commitments.push(updated);
        }
    }

    Ok(ScanBatchResult::builder()
        .scanned(total)
        .updated_commitments(updated_commitments)
        .build())
}

async fn scan_commitment_by_accounts(
    mut commitment: Document<Commitment>,
    accounts: &[ScanAccount],
) -> Result<Option<Document<Commitment>>, ScannerError> {
    if let Some(encrypted_note) = &commitment.data.encrypted_note {
        let encrypted_note_bytes: EncryptedNote = decode_hex(encrypted_note)?;
        for account in accounts {
            if let Ok(pcm) = ProtocolCommitment::new(
                account.shielded_address.clone(),
                None,
                Some(EncryptedData {
                    sk_enc: account.enc_sk,
                    encrypted_note: encrypted_note_bytes.clone(),
                }),
            ) {
                if pcm.commitment_hash == commitment.data.commitment_hash {
                    let nullifier = compute_nullifier(&account.v_sk, &pcm.note.random_p);
                    commitment.data.amount = Some(pcm.note.amount);
                    commitment.data.nullifier = Some(nullifier);
                    commitment.data.shielded_address = Some(account.shielded_address.address());
                    return Ok(Some(commitment));
                }
            }
        }
    }
    Ok(None)
}
