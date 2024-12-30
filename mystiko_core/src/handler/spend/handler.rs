use crate::handler::spend::merkle::CacheMerkleTree;
use crate::handler::{format_spend_log, wrap_filter};
use crate::{
    CommitmentPoolContractHandler, CommitmentPoolContracts, Database, FromContext, MystikoContext, MystikoError,
    PrivateKeySigner, PrivateKeySignerOptions, PublicAssetHandler, PublicAssets, Spend, SpendColumn, SpendHandler,
    SpendsError, TransactionHandler, TransactionSigner, Transactions, WalletHandler, Wallets,
};
use async_trait::async_trait;
use mystiko_config::MystikoConfig;
use mystiko_crypto::zkp::{G16Proof, G16Prover, ZKProver};
use mystiko_datapacker_client::v1::DataPackerClient as DataPackerClientV1;
use mystiko_datapacker_client::DataPackerClient;
use mystiko_datapacker_common::{Compression, ZstdCompression};
use mystiko_ethers::Providers;
use mystiko_protocol::error::ProtocolError;
use mystiko_protos::core::document::v1::Spend as ProtoSpend;
use mystiko_protos::core::handler::v1::{
    CreateSpendOptions, FixSpendStatusOptions, QuoteSpendOptions, SendSpendOptions, SpendQuote, SpendSummary,
};
use mystiko_protos::core::v1::Transaction;
use mystiko_protos::data::v1::{ChainData, MerkleTree};
use mystiko_protos::storage::v1::{QueryFilter, SubFilter};
use mystiko_relayer_client::v2::client::RelayerClientV2;
use mystiko_relayer_client::RelayerClient;
use mystiko_static_cache::{GzipStaticCache, StaticCache};
use mystiko_storage::{ColumnValues, StatementFormatter, Storage};
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub const MAX_NUM_INPUTS: usize = 2;
pub const MAX_NUM_OUTPUTS: usize = 2;
pub const EMPTY_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Spends<
    F: StatementFormatter,
    S: Storage,
    A = PublicAssets<Box<dyn Providers>>,
    C = CommitmentPoolContracts<Box<dyn Providers>>,
    T = Transactions<Box<dyn Providers>>,
    P = Box<dyn Providers>,
    R = RelayerClientV2,
    V = G16Prover,
    K = DataPackerClientV1,
    X = Box<dyn Compression>,
> {
    pub(crate) db: Arc<Database<F, S>>,
    pub(crate) static_cache: Arc<GzipStaticCache>,
    pub(crate) config: Arc<MystikoConfig>,
    pub(crate) wallets: Wallets<F, S>,
    pub(crate) assets: Arc<A>,
    pub(crate) commitment_pool_contracts: Arc<C>,
    pub(crate) transactions: Arc<T>,
    pub(crate) providers: Arc<P>,
    pub(crate) signer_providers: Arc<P>,
    pub(crate) relayers: Arc<R>,
    pub(crate) prover: Arc<V>,
    pub(crate) packer: Arc<K>,
    pub(crate) cache_tree: CacheMerkleTree,
    pub(crate) compression: Arc<X>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SpendsOptions<
    F: StatementFormatter,
    S: Storage,
    A = PublicAssets<Box<dyn Providers>>,
    C = CommitmentPoolContracts<Box<dyn Providers>>,
    T = Transactions<Box<dyn Providers>>,
    P = Box<dyn Providers>,
    R = RelayerClientV2,
    V = G16Prover,
    K = DataPackerClientV1,
    X = Box<dyn Compression>,
> {
    db: Arc<Database<F, S>>,
    static_cache: Arc<Box<dyn StaticCache>>,
    config: Arc<MystikoConfig>,
    commitment_pool_contracts: Arc<C>,
    assets: Arc<A>,
    transactions: Arc<T>,
    providers: Arc<P>,
    signer_providers: Arc<P>,
    relayers: Arc<R>,
    prover: Arc<V>,
    packer: Arc<K>,
    compression: Arc<X>,
}

type Result<T, E = SpendsError> = std::result::Result<T, E>;

#[async_trait]
impl<F, S, A, C, T, P, R, V, K, X>
    SpendHandler<
        ProtoSpend,
        QuoteSpendOptions,
        SpendQuote,
        CreateSpendOptions,
        SpendSummary,
        SendSpendOptions,
        FixSpendStatusOptions,
    > for Spends<F, S, A, C, T, P, R, V, K, X>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    P: Providers + 'static,
    R: RelayerClient,
    V: ZKProver<G16Proof>,
    K: DataPackerClient<ChainData, MerkleTree>,
    X: Compression + 'static,
    ProtocolError: From<V::Error>,
    SpendsError: From<A::Error> + From<C::Error> + From<T::Error> + From<R::Error> + From<V::Error>,
{
    type Error = SpendsError;

    async fn quote(&self, options: QuoteSpendOptions) -> Result<SpendQuote> {
        self.execute_quote(&options).await
    }

    async fn summary(&self, options: CreateSpendOptions) -> Result<SpendSummary> {
        self.execute_summary(&options).await.map(|(_, summary)| summary)
    }

    async fn create(&self, options: CreateSpendOptions) -> Result<ProtoSpend> {
        let wallet = self.wallets.check_password(&options.wallet_password.clone()).await?;
        let spend = self.execute_create(&wallet, &options).await?;
        let spend = self.db.spends.insert(&spend).await?;
        log::info!("successfully created a {}", format_spend_log(&spend));
        Ok(Spend::document_into_proto(spend))
    }

    async fn send(&self, options: SendSpendOptions) -> Result<ProtoSpend> {
        self.wallets.check_password(&options.wallet_password).await?;
        let spend = self.spend_to_send(&options).await?;
        let result = if let Some(relayer_url) = spend.data.gas_relayer_url.clone() {
            self.execute_send_with_relayer(&options, spend.clone(), &relayer_url)
                .await
        } else {
            let private_key = options.private_key.clone().ok_or(SpendsError::MissingPrivateKeyError)?;
            let signer = PrivateKeySigner::<P>::new(
                PrivateKeySignerOptions::builder()
                    .private_key(private_key)
                    .providers(self.signer_providers.clone())
                    .signer_provider(options.signer_provider.clone())
                    .build(),
            )?;
            self.execute_send_with_signer(&options, spend.clone(), Arc::new(signer))
                .await
        };
        self.handle_send_result(spend, result).await
    }

    async fn send_with_signer<Signer>(&self, options: SendSpendOptions, signer: Arc<Signer>) -> Result<ProtoSpend>
    where
        Signer: TransactionSigner + 'static,
    {
        self.wallets.check_password(&options.wallet_password).await?;
        let spend = self.spend_to_send(&options).await?;
        let result = self.execute_send_with_signer(&options, spend.clone(), signer).await;
        self.handle_send_result(spend, result).await
    }

    async fn fix_status(&self, options: FixSpendStatusOptions) -> Result<ProtoSpend> {
        let spend = self.execute_fix_status(options).await?;
        Ok(Spend::document_into_proto(spend))
    }

    async fn find<Filter>(&self, filter: Filter) -> Result<Vec<ProtoSpend>>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .spends
            .find(wrap_filter::<Filter, _, _>(
                filter,
                SpendColumn::WalletId,
                wallet.id.clone(),
            ))
            .await?
            .into_iter()
            .map(Spend::document_into_proto)
            .collect::<Vec<_>>())
    }

    async fn find_all(&self) -> Result<Vec<ProtoSpend>> {
        let wallet = self.wallets.check_current().await?;
        let filter = SubFilter::equal(SpendColumn::WalletId, wallet.id);
        Ok(self
            .db
            .spends
            .find(filter)
            .await?
            .into_iter()
            .map(Spend::document_into_proto)
            .collect::<Vec<_>>())
    }

    async fn find_one<Filter>(&self, filter: Filter) -> Result<Option<ProtoSpend>>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .spends
            .find_one(wrap_filter::<Filter, _, _>(
                filter,
                SpendColumn::WalletId,
                wallet.id.clone(),
            ))
            .await?
            .map(Spend::document_into_proto))
    }

    async fn find_by_id(&self, id: String) -> Result<Option<ProtoSpend>> {
        Ok(self.db.spends.find_by_id(&id).await?.map(Spend::document_into_proto))
    }

    async fn count<Filter>(&self, filter: Filter) -> Result<u64>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .spends
            .count(wrap_filter::<Filter, _, _>(
                filter,
                SpendColumn::WalletId,
                wallet.id.clone(),
            ))
            .await?)
    }

    async fn count_all(&self) -> Result<u64> {
        let wallet = self.wallets.check_current().await?;
        let filter = SubFilter::equal(SpendColumn::WalletId, wallet.id);
        Ok(self.db.spends.count(filter).await?)
    }

    async fn update(&self, spend: ProtoSpend) -> Result<ProtoSpend> {
        Ok(self
            .db
            .spends
            .update(&Spend::document_from_proto(spend)?)
            .await
            .map(Spend::document_into_proto)?)
    }

    async fn update_batch(&self, spends: Vec<ProtoSpend>) -> Result<Vec<ProtoSpend>> {
        let spends = spends
            .into_iter()
            .map(Spend::document_from_proto)
            .collect::<Result<Vec<_>, _>>()?;
        let spends = self.db.spends.update_batch(&spends).await?;
        Ok(spends.into_iter().map(Spend::document_into_proto).collect::<Vec<_>>())
    }

    async fn update_by_filter<Filter, Values>(&self, column_values: Values, filter: Filter) -> Result<()>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
        Values: Into<ColumnValues> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .spends
            .update_by_filter(
                column_values,
                wrap_filter::<Filter, _, _>(filter, SpendColumn::WalletId, wallet.id.clone()),
            )
            .await?)
    }

    async fn update_all<Values>(&self, column_values: Values) -> Result<()>
    where
        Values: Into<ColumnValues> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        let filter = SubFilter::equal(SpendColumn::WalletId, wallet.id);
        Ok(self.db.spends.update_by_filter(column_values, filter).await?)
    }

    async fn delete(&self, spend: ProtoSpend) -> Result<()> {
        Ok(self.db.spends.delete(&Spend::document_from_proto(spend)?).await?)
    }

    async fn delete_batch(&self, spends: Vec<ProtoSpend>) -> Result<()> {
        let spends = spends
            .into_iter()
            .map(Spend::document_from_proto)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(self.db.spends.delete_batch(&spends).await?)
    }

    async fn delete_by_filter<Filter>(&self, filter: Filter) -> Result<()>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
    {
        let wallet = self.wallets.check_current().await?;
        Ok(self
            .db
            .spends
            .delete_by_filter(wrap_filter::<Filter, _, _>(
                filter,
                SpendColumn::WalletId,
                wallet.id.clone(),
            ))
            .await?)
    }

    async fn delete_all(&self) -> Result<()> {
        let wallet = self.wallets.check_current().await?;
        let filter = SubFilter::equal(SpendColumn::WalletId, wallet.id);
        Ok(self.db.spends.delete_by_filter(filter).await?)
    }
}

#[async_trait]
impl<F, S, A, C, T> FromContext<F, S> for Spends<F, S, A, C, T>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler + FromContext<F, S>,
    C: CommitmentPoolContractHandler + FromContext<F, S>,
    T: TransactionHandler<Transaction> + FromContext<F, S>,
    SpendsError: From<A::Error> + From<C::Error> + From<T::Error>,
{
    async fn from_context(context: &MystikoContext<F, S>) -> std::result::Result<Self, MystikoError> {
        let relayers = RelayerClientV2::new(context.providers.clone(), context.relayer_client_options.clone()).await?;
        let packer = DataPackerClientV1::new(context.config.clone());
        let compression = Arc::new(Box::new(ZstdCompression) as Box<dyn Compression>);
        let options = SpendsOptions::<F, S, A, C, T>::builder()
            .db(context.db.clone())
            .static_cache(context.static_cache.clone())
            .config(context.config.clone())
            .assets(A::from_context(context).await?)
            .commitment_pool_contracts(C::from_context(context).await?)
            .transactions(T::from_context(context).await?)
            .providers(context.providers.clone())
            .signer_providers(context.signer_providers.clone())
            .relayers(relayers)
            .prover(G16Prover)
            .packer(packer)
            .compression(compression)
            .build();
        Ok(Self::new(options))
    }
}

impl<F, S, A, C, T, P, R, V, K, X> Spends<F, S, A, C, T, P, R, V, K, X>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    P: Providers + 'static,
    R: RelayerClient,
    V: ZKProver<G16Proof>,
    K: DataPackerClient<ChainData, MerkleTree>,
    X: Compression + 'static,
    SpendsError: From<A::Error> + From<C::Error> + From<T::Error> + From<R::Error> + From<V::Error>,
{
    pub fn new(options: SpendsOptions<F, S, A, C, T, P, R, V, K, X>) -> Self {
        let wallets = Wallets::new(options.db.clone());
        Self::builder()
            .db(options.db)
            .static_cache(GzipStaticCache::new(options.static_cache))
            .config(options.config)
            .wallets(wallets)
            .assets(options.assets)
            .commitment_pool_contracts(options.commitment_pool_contracts)
            .transactions(options.transactions)
            .providers(options.providers)
            .signer_providers(options.signer_providers)
            .relayers(options.relayers)
            .prover(options.prover)
            .packer(options.packer)
            .cache_tree(CacheMerkleTree::new())
            .compression(options.compression)
            .build()
    }
}
