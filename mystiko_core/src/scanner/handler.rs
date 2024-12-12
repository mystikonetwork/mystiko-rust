use crate::scanner::scan::{calc_balance_details, ScanRoundOptions};
use crate::{
    Accounts, CommitmentPoolContractHandler, CommitmentPoolContracts, Database, FromContext, MystikoContext,
    MystikoError, ScannerError, ScannerHandler, WalletHandler, Wallets,
};
use anyhow::anyhow;
use async_trait::async_trait;
use itertools::Itertools;
use mystiko_config::MystikoConfig;
use mystiko_ethers::Providers;
use mystiko_protos::core::scanner::v1::{
    AssetImportOptions, AssetImportResult, AssetsByChain, AssetsOptions, BalanceOptions, BalanceResult, ResetResult,
    ScanOptions, ScanResult, ScannerResetOptions, SyncOptions,
};
use mystiko_protos::data::v1::{Commitment as ProtosCommitment, Nullifier as ProtosNullifier};
use mystiko_protos::sequencer::v1::{FetchChainRequest, FetchChainResponse};
use mystiko_sequencer_client::v1::SequencerClient as SequencerClientV1;
use mystiko_sequencer_client::SequencerClient;
use mystiko_storage::{StatementFormatter, Storage};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Scanner<
    F: StatementFormatter,
    S: Storage,
    C = CommitmentPoolContracts<Box<dyn Providers>>,
    Q = SequencerClientV1,
    P = Box<dyn Providers>,
> {
    pub(crate) config: Arc<MystikoConfig>,
    pub(crate) db: Arc<Database<F, S>>,
    pub(crate) wallets: Wallets<F, S>,
    pub(crate) accounts: Accounts<F, S>,
    pub(crate) commitment_pool_contracts: Arc<C>,
    pub(crate) providers: Arc<P>,
    pub(crate) sequencer: Arc<Q>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ScannerOptions<
    F: StatementFormatter,
    S: Storage,
    C = CommitmentPoolContracts<Box<dyn Providers>>,
    Q = SequencerClientV1,
    P = Box<dyn Providers>,
> {
    db: Arc<Database<F, S>>,
    commitment_pool_contracts: Arc<C>,
    providers: Arc<P>,
    sequencer: Arc<Q>,
}

#[async_trait]
impl<F, S, C> FromContext<F, S> for Scanner<F, S, C>
where
    F: StatementFormatter,
    S: Storage,
    C: CommitmentPoolContractHandler + FromContext<F, S>,
    ScannerError: From<C::Error>,
{
    async fn from_context(context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        let sequencer_config = context
            .config
            .sequencer()
            .ok_or(MystikoError::ConfigError(anyhow!("sequencer config not found")))?;
        let client = SequencerClientV1::connect(&sequencer_config.into())
            .await
            .map_err(|e| ScannerError::AnyhowError(anyhow!("sequencer error {}", e)))?;
        let options = ScannerOptions::<F, S, C>::builder()
            .db(context.db.clone())
            .commitment_pool_contracts(C::from_context(context).await?)
            .providers(context.providers.clone())
            .sequencer(client)
            .build();
        Ok(Scanner::new(context.config.clone(), options))
    }
}

#[async_trait]
impl<F, S, C, Q, P>
    ScannerHandler<
        SyncOptions,
        ScanOptions,
        ScanResult,
        ScannerResetOptions,
        ResetResult,
        AssetImportOptions,
        AssetImportResult,
        BalanceOptions,
        BalanceResult,
        AssetsOptions,
        AssetsByChain,
    > for Scanner<F, S, C, Q, P>
where
    F: StatementFormatter,
    S: Storage,
    C: CommitmentPoolContractHandler,
    Q: SequencerClient<FetchChainRequest, FetchChainResponse, ProtosCommitment, ProtosNullifier>,
    P: Providers + 'static,
    ScannerError: From<C::Error>,
{
    type Error = ScannerError;
    async fn sync(&self, options: SyncOptions) -> Result<(), Self::Error> {
        self.asset_sync(options).await
    }

    async fn scan(&self, options: ScanOptions) -> Result<ScanResult, Self::Error> {
        self.wallets.check_password(&options.wallet_password).await?;
        let mut accounts = self.build_filter_accounts(&options.shielded_addresses).await?;
        if accounts.is_empty() {
            return Ok(self.build_default_scan_result(&[], None));
        }

        let scanned_id = accounts
            .iter()
            .map(|accounts| accounts.data.scanned_to_id.clone())
            .min()
            .flatten();
        let cms_count = self.query_commitment_count(scanned_id.clone()).await?;
        if cms_count == 0 {
            return Ok(self.build_default_scan_result(&accounts, scanned_id));
        }

        let scan_accounts = Arc::new(self.build_scan_accounts(&accounts, &options.wallet_password).await?);
        let rounds = ScanRoundOptions::split_rounds(&options, &cms_count, scan_accounts.clone());
        let mut current_scanned_id = scanned_id;
        let (mut total_scanned, mut total_owned) = (0, 0);
        for mut round in rounds.into_iter() {
            round.start_id = current_scanned_id;
            let result = self.scan_one_round(&round).await?;
            self.update_accounts_scanned_to_id(&mut accounts, &result.end_id)
                .await?;
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

    async fn reset(&self, options: ScannerResetOptions) -> Result<ResetResult, Self::Error> {
        let mut accounts = self.build_filter_accounts(&options.shielded_addresses).await?;
        if accounts.is_empty() {
            return Ok(ResetResult::default());
        }

        accounts.iter_mut().for_each(|account| {
            account.data.scanned_to_id.clone_from(&options.reset_to_id);
        });
        self.db.accounts.update_batch(&accounts).await?;
        let shielded_addresses = accounts
            .iter()
            .map(|account| account.data.shielded_address.clone())
            .collect::<Vec<_>>();
        log::info!(
            "successfully reset scanner to commitment id={:?} \
            for shielded_addresses={:?}",
            options.reset_to_id,
            shielded_addresses
        );
        Ok(ResetResult::default())
    }

    async fn import(&self, options: AssetImportOptions) -> Result<AssetImportResult, Self::Error> {
        self.asset_import(options).await
    }

    async fn balance(&self, options: BalanceOptions) -> Result<BalanceResult, Self::Error> {
        let filter = self.build_balance_filter(&options).await?;
        let commitments = self.db.commitments.find(filter).await?;
        let balances = commitments
            .iter()
            .map(|commitment| commitment.data.asset_symbol.clone())
            .unique()
            .map(|asset_symbol| calc_balance_details(commitments.as_slice(), &asset_symbol, options.with_spent))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(BalanceResult::builder().balances(balances).build())
    }

    async fn assets(&self, options: AssetsOptions) -> Result<Vec<AssetsByChain>, Self::Error> {
        self.assets_balance(&options, None).await
    }

    async fn chain_assets(&self, chain_id: u64, options: AssetsOptions) -> Result<Option<AssetsByChain>, Self::Error> {
        let result = self.assets_balance(&options, Some(chain_id)).await?;
        Ok(result.into_iter().next())
    }
}

impl<F, S, C, Q, P> Scanner<F, S, C, Q, P>
where
    F: StatementFormatter,
    S: Storage,
    C: CommitmentPoolContractHandler,
    Q: SequencerClient<FetchChainRequest, FetchChainResponse, ProtosCommitment, ProtosNullifier>,
    P: Providers + 'static,
    ScannerError: From<C::Error>,
{
    pub fn new(config: Arc<MystikoConfig>, options: ScannerOptions<F, S, C, Q, P>) -> Self {
        Scanner::builder()
            .config(config)
            .wallets(Wallets::<F, S>::new(options.db.clone()))
            .accounts(Accounts::<F, S>::new(options.db.clone()))
            .db(options.db)
            .commitment_pool_contracts(options.commitment_pool_contracts)
            .providers(options.providers)
            .sequencer(options.sequencer)
            .build()
    }
}
