use crate::{
    Account, Accounts, Commitment, CommitmentColumn, Database, FromContext, MystikoContext, MystikoError, Nullifier,
    NullifierColumn, ScannerError, ScannerHandler, WalletHandler, Wallets,
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

#[async_trait]
impl<F, S> FromContext<F, S> for Scanner<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        Ok(Scanner::<F, S>::builder()
            .wallets(Wallets::<F, S>::new(context.db.clone()))
            .accounts(Accounts::<F, S>::new(context.db.clone()))
            .db(context.db.clone())
            .build())
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
        if accounts.is_empty() {
            return Ok(self.build_default_scan_result(&[]));
        }

        let scanned_id = accounts
            .iter()
            .filter_map(|account| account.data.scanned_to_id.as_ref())
            .min()
            .cloned();
        let cms_count = self.query_commitment_count(scanned_id.clone()).await?;
        if cms_count == 0 {
            return Ok(self.build_default_scan_result(&accounts));
        }

        let scan_accounts = Arc::new(self.build_scan_accounts(&accounts, &options.wallet_password).await?);
        let rounds = ScanRoundOptions::split_rounds(&options, &cms_count, scan_accounts.clone());
        let mut current_scanned_id = scanned_id;
        let (mut total_scanned, mut total_owned) = (0, 0);
        for mut round in rounds.into_iter() {
            round.start_id = current_scanned_id;
            let result = self.scan_one_round(&round).await?;
            current_scanned_id = Some(result.end_id);
            total_owned += result.owned_count;
            total_scanned += result.scanned_count;
        }

        Ok(ScanResult::builder()
            .total_count(total_scanned)
            .owned_count(total_owned)
            .scanned_shielded_addresses(
                accounts
                    .iter()
                    .map(|account| account.data.shielded_address.clone())
                    .collect::<Vec<_>>(),
            )
            .to_id(current_scanned_id)
            .build())
    }

    async fn reset(&self, options: ResetOptions) -> Result<ResetResult, Self::Error> {
        let mut accounts = self.build_filter_accounts(&options.shielded_addresses).await?;
        if accounts.is_empty() {
            return Ok(ResetResult::default());
        }

        accounts.iter_mut().for_each(|account| {
            account.data.scanned_to_id = options.reset_to_id.clone();
        });
        self.db.accounts.update_batch(&accounts).await?;
        Ok(ResetResult::default())
    }

    async fn balance(&self, options: BalanceOptions) -> Result<BalanceResult, Self::Error> {
        let condition = self.build_balance_filter(&options).await?;
        let commitments = self.db.commitments.find(condition).await?;
        let assets = commitments
            .iter()
            .map(|commitment| commitment.data.asset_symbol.clone())
            .collect::<HashSet<_>>();

        let balances: Result<Vec<_>, _> = assets
            .into_iter()
            .map(|asset_symbol| self.calc_balance_by_asset_symbol(&commitments, &asset_symbol, options.with_spent))
            .collect();

        Ok(BalanceResult::builder().balances(balances?).build())
    }
}

#[derive(Debug, Clone, TypedBuilder)]
struct ScanningAccount {
    shielded_address: ShieldedAddress,
    sk_enc: EncSk,
    sk_verify: VerifySk,
}

impl ScanningAccount {
    fn new(wallet_password: &str, account: &Document<Account>) -> Result<Self, ScannerError> {
        let shielded_address = ShieldedAddress::from_string(&account.data.shielded_address)?;
        let secret_key = decrypt_symmetric(wallet_password, &account.data.encrypted_secret_key)?;
        let secret_key_bytes: FullSk = decode_hex_with_length(secret_key)?;
        let (sk_verify, sk_enc) = separate_secret_keys(&secret_key_bytes);
        Ok(ScanningAccount::builder()
            .shielded_address(shielded_address)
            .sk_enc(sk_enc)
            .sk_verify(sk_verify)
            .build())
    }
}

#[derive(Debug, Clone, TypedBuilder)]
struct ScanRoundOptions {
    concurrency: u32,
    scan_batch_size: u64,
    max_query_filter_size: u64,
    start_id: Option<String>,
    accounts: Arc<Vec<ScanningAccount>>,
}

impl ScanRoundOptions {
    fn split_rounds(options: &ScanOptions, cms_count: &u64, scanning_accounts: Arc<Vec<ScanningAccount>>) -> Vec<Self> {
        let batch_size = std::cmp::max(
            1,
            options.batch_size.unwrap_or(DEFAULT_BATCH_SIZE) / scanning_accounts.len() as u64,
        );
        let concurrency = std::cmp::max(1, options.concurrency.unwrap_or(1));
        let scan_batch_size = batch_size * concurrency as u64;
        let total_round = (cms_count + scan_batch_size - 1) / scan_batch_size;
        (0..total_round)
            .map(|_| {
                ScanRoundOptions::builder()
                    .concurrency(concurrency)
                    .scan_batch_size(scan_batch_size)
                    .max_query_filter_size(DEFAULT_MAX_QUERY_FILTER_SIZE)
                    .start_id(None)
                    .accounts(scanning_accounts.clone())
                    .build()
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, TypedBuilder)]
struct ScanRoundResult {
    scanned_count: u64,
    owned_count: u64,
    end_id: String,
}

#[derive(Debug, Clone, TypedBuilder)]
struct ScanBatchResult {
    scanned_count: u64,
    updated_commitments: Vec<Document<Commitment>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, TypedBuilder)]
struct NullifierCompositeKey {
    chain_id: u64,
    contract_address: String,
    nullifier: BigUint,
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

    async fn scan_one_round(&self, round_options: &ScanRoundOptions) -> Result<ScanRoundResult, ScannerError> {
        let commitments = self.query_commitments(round_options).await?;
        let end_id = commitments.last().ok_or(ScannerError::CommitmentEmptyError)?.id.clone();
        let chunk_size = std::cmp::max(1, commitments.len() / round_options.concurrency as usize);
        let tasks = commitments
            .chunks(chunk_size)
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| {
                let chunk = chunk.to_owned();
                let accounts = round_options.accounts.clone();
                tokio::task::spawn_blocking(move || scan_commitments(chunk, accounts))
            });
        let results = futures::future::try_join_all(tasks).await?;
        let (scanned_count, updated_commitments) = results.into_iter().try_fold(
            (0, Vec::new()),
            |(acc_scanned_count, mut acc_updated_commitments), result| {
                let scan = result?;
                acc_updated_commitments.extend(scan.updated_commitments);
                Ok::<(u64, Vec<_>), ScannerError>((acc_scanned_count + scan.scanned_count, acc_updated_commitments))
            },
        )?;

        let mut owned_count = 0;
        if !updated_commitments.is_empty() {
            let status_updated = self
                .update_commitments_spent_status(updated_commitments, round_options)
                .await?;
            owned_count = status_updated.len() as u64;
            self.db.commitments.update_batch(&status_updated).await?;
        }
        Ok(ScanRoundResult::builder()
            .scanned_count(scanned_count)
            .owned_count(owned_count)
            .end_id(end_id)
            .build())
    }

    async fn build_filter_accounts(
        &self,
        shielded_addresses: &[String],
    ) -> Result<Vec<Document<Account>>, ScannerError> {
        let accounts = self
            .accounts
            .find_all_documents()
            .await?
            .into_iter()
            .filter(|account| {
                shielded_addresses.is_empty() || shielded_addresses.contains(&account.data.shielded_address)
            })
            .collect::<Vec<_>>();

        if !shielded_addresses.is_empty() && accounts.is_empty() {
            return Err(ScannerError::NoSuchAccountError);
        }

        Ok(accounts)
    }

    async fn build_scan_accounts(
        &self,
        accounts: &[Document<Account>],
        password: &str,
    ) -> Result<Vec<ScanningAccount>, ScannerError> {
        accounts
            .iter()
            .map(|account| ScanningAccount::new(password, account))
            .collect()
    }

    fn build_default_scan_result(&self, accounts: &[Document<Account>]) -> ScanResult {
        ScanResult::builder()
            .scanned_shielded_addresses(
                accounts
                    .iter()
                    .map(|account| account.data.shielded_address.clone())
                    .collect::<Vec<_>>(),
            )
            .build()
    }

    async fn update_commitments_spent_status(
        &self,
        commitments: Vec<Document<Commitment>>,
        round_options: &ScanRoundOptions,
    ) -> Result<Vec<Document<Commitment>>, ScannerError> {
        let chunk_size = std::cmp::max(1, round_options.max_query_filter_size) as usize;
        let tasks = commitments
            .chunks(chunk_size)
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| {
                let chunk = chunk.to_owned();
                self.update_batch_commitments_spent_status(chunk)
            });
        let results = futures::future::try_join_all(tasks).await?;
        Ok(results.into_iter().flatten().collect())
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
            commitments.iter_mut().for_each(|commitment| {
                if let Some(data_nullifier) = &commitment.data.nullifier {
                    let key = NullifierCompositeKey::builder()
                        .chain_id(commitment.data.chain_id)
                        .contract_address(commitment.data.contract_address.clone())
                        .nullifier(data_nullifier.clone())
                        .build();
                    if let Some(db_nullifier) = nullifiers.get(&key) {
                        if db_nullifier.data.chain_id == commitment.data.chain_id
                            && db_nullifier.data.contract_address == commitment.data.contract_address
                        {
                            commitment.data.status = CommitmentStatus::Included as i32;
                            commitment.data.spent = true;
                        }
                    }
                }
            });
        }

        Ok(commitments)
    }

    async fn query_commitment_count(&self, scanned_id: Option<String>) -> Result<u64, ScannerError> {
        let total = if let Some(id) = scanned_id {
            self.db
                .commitments
                .count(SubFilter::greater(DocumentColumn::Id, id.clone()))
                .await?
        } else {
            self.db.commitments.count_all().await?
        };
        Ok(total)
    }

    async fn query_commitments(&self, options: &ScanRoundOptions) -> Result<Vec<Document<Commitment>>, ScannerError> {
        let order = OrderBy::builder()
            .columns(vec![DocumentColumn::Id.to_string()])
            .order(Order::Asc)
            .build();

        let filter = if let Some(start) = &options.start_id {
            let condition: Condition = SubFilter::greater(DocumentColumn::Id, start.clone()).into();
            QueryFilter::builder()
                .limit(options.scan_batch_size)
                .order_by(order)
                .conditions(vec![condition])
                .conditions_operator(ConditionOperator::And as i32)
                .build()
        } else {
            QueryFilter::builder()
                .limit(options.scan_batch_size)
                .order_by(order)
                .build()
        };
        let commitments = self.db.commitments.find(filter).await?;
        Ok(commitments)
    }

    async fn query_nullifiers(
        &self,
        filter_nullifiers: Vec<BigUint>,
    ) -> Result<HashMap<NullifierCompositeKey, Document<Nullifier>>, ScannerError> {
        if !filter_nullifiers.is_empty() {
            let filter = SubFilter::in_list(NullifierColumn::Nullifier, filter_nullifiers);
            let nullifiers = self.db.nullifiers.find(filter).await?;
            Ok(nullifiers
                .into_iter()
                .map(|nullifier| {
                    (
                        NullifierCompositeKey::builder()
                            .chain_id(nullifier.data.chain_id)
                            .contract_address(nullifier.data.contract_address.clone())
                            .nullifier(nullifier.data.nullifier.clone())
                            .build(),
                        nullifier,
                    )
                })
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
        let (pending, unspent, spent) = commitments
            .iter()
            .filter(|commitment| commitment.data.asset_symbol == asset_symbol)
            .try_fold(
                (0f64, 0f64, 0f64),
                |(mut pending_amount, mut unspent_amount, mut spent_aount), commitment| {
                    let amount = decimal_to_number::<f64, BigUint>(
                        commitment.data.amount.as_ref().unwrap_or(&BigUint::default()),
                        Some(commitment.data.asset_decimals),
                    )?;
                    match commitment.data {
                        Commitment { spent: true, .. } => spent_aount += amount,
                        Commitment { status, .. } if status == CommitmentStatus::Included as i32 => {
                            unspent_amount += amount
                        }
                        _ => pending_amount += amount,
                    };
                    Ok::<(f64, f64, f64), ScannerError>((pending_amount, unspent_amount, spent_aount))
                },
            )?;

        let spent_amount = with_spent.filter(|&b| b).map(|_| spent);
        Ok(Balance::builder()
            .asset_symbol(asset_symbol.to_string())
            .pending(pending)
            .unspent(unspent)
            .spent(spent_amount)
            .build())
    }

    async fn build_balance_filter(&self, options: &BalanceOptions) -> Result<Condition, ScannerError> {
        let shielded_addresses = self
            .build_filter_accounts(&options.shielded_addresses)
            .await?
            .into_iter()
            .map(|account| account.data.shielded_address)
            .collect::<Vec<_>>();
        println!("shielded_addresses: {:?}", shielded_addresses);
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

        Ok(Condition::from(filters))
    }
}

fn scan_commitments(
    commitments: Vec<Document<Commitment>>,
    accounts: Arc<Vec<ScanningAccount>>,
) -> Result<ScanBatchResult, ScannerError> {
    let scanned_count = commitments.len() as u64;
    let updated_commitments: Result<Vec<_>, _> = commitments
        .into_iter()
        .filter_map(|commitment| scan_commitment_by_accounts(commitment, &accounts).transpose())
        .collect();
    Ok(ScanBatchResult::builder()
        .scanned_count(scanned_count)
        .updated_commitments(updated_commitments?)
        .build())
}

fn scan_commitment_by_accounts(
    mut commitment: Document<Commitment>,
    accounts: &[ScanningAccount],
) -> Result<Option<Document<Commitment>>, ScannerError> {
    if let Some(encrypted_note) = &commitment.data.encrypted_note {
        let encrypted_note_bytes: EncryptedNote = decode_hex(encrypted_note)?;
        for account in accounts {
            if let Ok(pcm) = ProtocolCommitment::new(
                account.shielded_address.clone(),
                None,
                Some(EncryptedData {
                    sk_enc: account.sk_enc,
                    encrypted_note: encrypted_note_bytes.clone(),
                }),
            ) {
                if pcm.commitment_hash == commitment.data.commitment_hash {
                    let nullifier = compute_nullifier(&account.sk_verify, &pcm.note.random_p);
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
