use crate::{
    Account, AccountHandler, Commitment, CommitmentColumn, Database, Nullifier, NullifierColumn, ScannerError,
    ScannerHandler, WalletHandler,
};
use async_trait::async_trait;
use mystiko_config::MystikoConfig;
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
    config: Arc<MystikoConfig>,
    db: Arc<Database<F, S>>,
    wallets: WalletHandler<F, S>,
    accounts: AccountHandler<F, S>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ScannerOptions<F: StatementFormatter, S: Storage> {
    config: Arc<MystikoConfig>,
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
            .wallets(WalletHandler::<F, S>::new(options.db.clone()))
            .accounts(AccountHandler::<F, S>::new(options.db.clone()))
            .config(options.config)
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
        let from_id = accounts
            .iter()
            .filter_map(|account| account.data.scanned_to_id.as_ref())
            .min()
            .cloned();
        let cms_count = self.query_commitment_count(from_id.clone()).await?;
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
        let mut current_id = from_id;
        for _ in 0..total_round {
            let params = ScanRoundParams::builder()
                .concurrency(concurrency)
                .scan_batch_size(scan_batch_size)
                .max_query_filter_size(DEFAULT_MAX_QUERY_FILTER_SIZE)
                .start_id(current_id.clone())
                .accounts(scan_accounts.clone())
                .build();
            let result = self.scan_one_round(&params).await?;
            current_id = Some(result.end_id);
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
            .to_id(current_id)
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
        let tasks = assets
            .into_iter()
            .map(|asset_symbol| self.calc_asset_balance(asset_symbol, &commitments, options.with_spent));
        let results = futures::future::try_join_all(tasks).await?;
        Ok(BalanceResult::builder().balances(results).build())
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
            return Err(ScannerError::NoAccountFound);
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
            let secret_key = self.accounts.export_secret_key_by_id(password, &account.id).await?;
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
        let end_id = commitments
            .last()
            .ok_or(ScannerError::InternalError("commitment is empty".to_string()))?
            .id
            .clone();
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
            let status_updated = self.update_commitments_spend_status(updated, params).await?;
            owned = status_updated.len() as u64;
            self.db.commitments.update_batch(&status_updated).await?;
        }
        Ok(ScanRoundResult::builder()
            .scanned(scanned)
            .owned(owned)
            .end_id(end_id)
            .build())
    }

    async fn update_commitments_spend_status(
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
                self.update_batch_commitments_spend_status(chunk)
            });
        let results = futures::future::try_join_all(tasks).await?;
        results.into_iter().try_fold(Vec::new(), |mut acc, result| {
            acc.extend(result);
            Ok::<Vec<_>, ScannerError>(acc)
        })
    }

    async fn update_batch_commitments_spend_status(
        &self,
        mut commitments: Vec<Document<Commitment>>,
    ) -> Result<Vec<Document<Commitment>>, ScannerError> {
        let filter_nullifiers = commitments
            .iter()
            .filter_map(|commitment| commitment.data.nullifier.clone())
            .collect::<Vec<_>>();
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

    async fn query_commitment_count(&self, from_id: Option<String>) -> Result<u64, ScannerError> {
        let mut filter = QueryFilter::builder().build();
        if let Some(start) = from_id {
            let condition = Condition::from(SubFilter::greater(DocumentColumn::Id, start));
            filter.conditions = vec![condition];
        }

        let total = self.db.commitments.count(filter).await?;
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

    async fn calc_asset_balance(
        &self,
        asset_symbol: String,
        commitments: &[Document<Commitment>],
        with_spent: Option<bool>,
    ) -> Result<Balance, ScannerError> {
        let filter_cms: Vec<&Document<Commitment>> = commitments
            .iter()
            .filter(|commitment| commitment.data.asset_symbol == asset_symbol)
            .collect();

        let tasks: Vec<_> = filter_cms
            .iter()
            .map(|commitment| &commitment.data.chain_id)
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|chain_id| self.calc_chain_asset_balance(chain_id, &asset_symbol, &filter_cms))
            .collect();

        let all_balances = futures::future::try_join_all(tasks).await?;
        let (pending, unspent, spent) = all_balances
            .into_iter()
            .fold((0f64, 0f64, 0f64), |acc, (u, p, s)| (acc.0 + u, acc.1 + p, acc.2 + s));
        let spend = if with_spent == Some(true) { Some(spent) } else { None };
        Ok(Balance::builder()
            .asset_symbol(asset_symbol)
            .pending(pending)
            .unspent(unspent)
            .spent(spend)
            .build())
    }

    async fn calc_chain_asset_balance(
        &self,
        chain_id: &u64,
        asset_symbol: &str,
        commitments: &[&Document<Commitment>],
    ) -> Result<(f64, f64, f64), ScannerError> {
        let chain_config = self
            .config
            .find_chain(*chain_id)
            .ok_or(ScannerError::ChainConfigNotFoundError(*chain_id))?;
        let asset_decimals = if chain_config.asset_symbol() == asset_symbol {
            chain_config.asset_decimals()
        } else {
            chain_config
                .assets()
                .iter()
                .find(|asset| asset.asset_symbol() == asset_symbol)
                .ok_or(ScannerError::AssetConfigNotFoundError(
                    *chain_id,
                    asset_symbol.to_string(),
                ))?
                .asset_decimals()
        };

        let (pending, unspent, spend) = commitments.iter().try_fold(
            (BigUint::default(), BigUint::default(), BigUint::default()),
            |(pending, unspent, spend), commitment| {
                let amount =
                    commitment.data.amount.as_ref().ok_or_else(|| {
                        ScannerError::MissingAmountError(commitment.data.chain_id, commitment.id.clone())
                    })?;
                if commitment.data.spent {
                    Ok::<(BigUint, BigUint, BigUint), ScannerError>((pending, unspent, spend + amount))
                } else {
                    match commitment.data.status {
                        status if status == CommitmentStatus::Included as i32 => {
                            Ok::<(BigUint, BigUint, BigUint), ScannerError>((pending, unspent + amount, spend))
                        }
                        _ => Ok::<(BigUint, BigUint, BigUint), ScannerError>((pending + amount, unspent, spend)),
                    }
                }
            },
        )?;

        let pending = decimal_to_number::<f64, BigUint>(&pending, Some(asset_decimals))?;
        let unspent = decimal_to_number::<f64, BigUint>(&unspent, Some(asset_decimals))?;
        let spend = decimal_to_number::<f64, BigUint>(&spend, Some(asset_decimals))?;
        Ok((pending, unspent, spend))
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
    match &commitment.data.encrypted_note {
        Some(encrypted_note) => {
            let encrypted_note_bytes: EncryptedNote = decode_hex(encrypted_note)?;
            for account in accounts {
                let protocol_commitment = ProtocolCommitment::new(
                    account.shielded_address.clone(),
                    None,
                    Some(EncryptedData {
                        sk_enc: account.enc_sk,
                        encrypted_note: encrypted_note_bytes.clone(),
                    }),
                );
                if let Ok(pcm) = protocol_commitment {
                    if pcm.commitment_hash == commitment.data.commitment_hash {
                        let nullifier = compute_nullifier(&account.v_sk, &pcm.note.random_p);
                        commitment.data.amount = Some(pcm.note.amount);
                        commitment.data.nullifier = Some(nullifier);
                        commitment.data.shielded_address = Some(account.shielded_address.address());
                        return Ok(Some(commitment));
                    }
                }
            }
            Ok(None)
        }
        None => Err(ScannerError::CommitmentEncryptedNoteIsNone(commitment.id.clone())),
    }
}
