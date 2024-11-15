mod account;
mod deposit;
mod spend;
mod wallet;

use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TransactionReceipt, TxHash, U256};
use ethers_signers::{LocalWallet, Signer};
use mockall::mock;
use mystiko_core::{
    AuditorPublicKeysOptions, BalanceOptions, CommitmentPoolContractHandler, CrossChainDepositOptions,
    DepositContractHandler, DepositOptions, DepositQuote, DepositQuoteOptions, Erc20AllowanceOptions,
    Erc20ApproveOptions, Erc20BalanceOptions, Erc20TransferOptions, IsHistoricCommitmentOptions, IsKnownRootOptions,
    IsSpentNullifierOptions, MinRollupFeeOptions, PublicAssetHandler, TransactOptions, TransactionHandler,
    TransactionSigner, TransferOptions, WaitOptions,
};
use mystiko_crypto::zkp::{G16Proof, ZKProveOptions, ZKVerifyOptions};
use mystiko_datapacker_client::{ChainQuery, ChainResponse};
use mystiko_protos::core::v1::Transaction;
use mystiko_protos::data::v1::{ChainData, MerkleTree as ProtoMerkleTree};
use mystiko_relayer_client::types::register::RegisterInfo;
use mystiko_relayer_types::{
    RegisterInfoRequest, RelayTransactRequest, RelayTransactResponse, RelayTransactStatusRequest,
    RelayTransactStatusResponse, WaitingTransactionRequest,
};
use mystiko_screening_client::{ScreeningRequest, ScreeningResponse};
use mystiko_static_cache::GetOptions;
use mystiko_types::TransactionType;
use mystiko_utils::hex::encode_hex;
use std::time::Duration;
use typed_builder::TypedBuilder;

mock! {
    #[derive(Debug, Default)]
    PublicAssets {}

    #[async_trait]
    impl PublicAssetHandler for PublicAssets {
        type Error = anyhow::Error;
        async fn balance_of(&self, options: BalanceOptions) -> Result<U256>;
        async fn transfer<T, S>(&self, options: TransferOptions<T, S>) -> Result<TxHash>
        where
            S: TransactionSigner + 'static,
            T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;
        async fn erc20_balance_of(&self, options: Erc20BalanceOptions) -> Result<U256>;
        async fn erc20_allowance(&self, options: Erc20AllowanceOptions) -> Result<U256>;
        async fn erc20_approve<T, S>(&self, options: Erc20ApproveOptions<T, S>) -> Result<Option<TxHash>>
        where
            S: TransactionSigner + 'static,
            T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;
        async fn erc20_transfer<T, S>(&self, options: Erc20TransferOptions<T, S>) -> Result<TxHash>
        where
            S: TransactionSigner + 'static,
            T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;
    }
}

mock! {
    #[derive(Debug, Default)]
    DepositContracts {}

    #[async_trait]
    impl DepositContractHandler for DepositContracts {
        type Error = anyhow::Error;
        async fn quote(&self, options: DepositQuoteOptions) -> Result<DepositQuote>;

        async fn deposit<T, S>(&self, options: DepositOptions<T, S>) -> Result<TxHash>
        where
            S: TransactionSigner + 'static,
            T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;

        async fn cross_chain_deposit<T, S>(&self, options: CrossChainDepositOptions<T, S>) -> Result<TxHash>
        where
            S: TransactionSigner + 'static,
            T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;
    }
}

mock! {
    #[derive(Debug, Default)]
    CommitmentPoolContracts {}

    #[async_trait]
    impl CommitmentPoolContractHandler for CommitmentPoolContracts {
        type Error = anyhow::Error;
        async fn is_historic_commitment(&self, options: IsHistoricCommitmentOptions) -> Result<bool>;
        async fn is_spent_nullifier(&self, options: IsSpentNullifierOptions) -> Result<bool>;
        async fn is_known_root(&self, options: IsKnownRootOptions) -> Result<bool>;
        async fn min_rollup_fee(&self, options: MinRollupFeeOptions) -> Result<U256>;
        async fn auditor_public_keys(&self, options: AuditorPublicKeysOptions) -> Result<Vec<U256>>;
        async fn transact<T, S>(&self, options: TransactOptions<T, S>) -> Result<TxHash>
        where
            T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
            S: TransactionSigner + 'static;
    }
}

mock! {
    #[derive(Debug, Default)]
    Transactions {}

    #[async_trait]
    impl TransactionHandler<Transaction> for Transactions {
        type Error = anyhow::Error;
        fn create(&self, tx: Option<Transaction>, tx_type: &TransactionType) -> Result<TypedTransaction>;
        async fn wait(&self, options: WaitOptions) -> Result<TransactionReceipt>;
    }
}

mock! {
    #[derive(Debug, Default)]
    StaticCache {}

    #[async_trait]
    impl mystiko_static_cache::StaticCache for StaticCache {
        async fn get(&self, url: &str, options: Option<GetOptions>) -> Result<Vec<u8>>;
        async fn get_failover(&self, urls: &[String], options: Option<GetOptions>) -> Result<Vec<u8>>;
    }
}

mock! {
    #[derive(Debug, Default)]
    RelayerClient {}

    #[async_trait]
    impl mystiko_relayer_client::RelayerClient for RelayerClient {
        type Error = anyhow::Error;
        async fn register_info(&self, request: RegisterInfoRequest) -> Result<Vec<RegisterInfo>>;
        async fn relay_transact(&self, request: RelayTransactRequest) -> Result<RelayTransactResponse>;
        async fn relay_transaction_status(
            &self,
            request: RelayTransactStatusRequest,
        ) -> Result<RelayTransactStatusResponse>;
        async fn wait_transaction(
            &self,
            request: WaitingTransactionRequest,
        ) -> Result<RelayTransactStatusResponse>;
        async fn handshake(&self, url: &str) -> Result<bool>;
    }
}

mock! {
    #[derive(Debug, Default)]
    ScreeningClient {}

    #[async_trait]
    impl mystiko_screening_client::ScreeningClient for ScreeningClient {
        async fn address_screening(&self, request: &ScreeningRequest) -> Result<ScreeningResponse>;
    }
}

mock! {
    #[derive(Debug, Default)]
    ZKProver {}

    impl mystiko_crypto::zkp::ZKProver<G16Proof> for ZKProver {
        type Error = anyhow::Error;
        fn prove<'a>(&self, options: ZKProveOptions<'a>) -> Result<G16Proof>;
        fn verify<'a>(&self, options: ZKVerifyOptions<'a, G16Proof>) -> Result<bool>;
    }
}

// type ConcreteDataPackerClient = dyn DataPackerClient<ChainData, MerkleTree>;
mock! {
   #[derive(Debug, Default)]
    DataPackerClient{}

    #[async_trait]
    impl mystiko_datapacker_client::DataPackerClient<ChainData, ProtoMerkleTree> for DataPackerClient {
        async fn query_chain(&self, query: &ChainQuery) -> Result<ChainResponse<ChainData>>;
        async fn query_chain_loaded_block(&self, chain_id: u64) -> Result<u64>;
        async fn query_merkle_tree(&self, chain_id: u64, address: &Address) -> Result<Option<ProtoMerkleTree>>;
    }

}

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct TimeoutRelayerClient<R: Default = MockRelayerClient> {
    timeout_ms: Option<u64>,
    raw: R,
}

#[async_trait]
impl mystiko_relayer_client::RelayerClient for TimeoutRelayerClient {
    type Error = anyhow::Error;

    async fn register_info(&self, request: RegisterInfoRequest) -> Result<Vec<RegisterInfo>> {
        tokio::time::sleep(Duration::from_millis(self.timeout_ms.unwrap_or(1))).await;
        self.raw.register_info(request).await
    }

    async fn relay_transact(&self, request: RelayTransactRequest) -> Result<RelayTransactResponse> {
        tokio::time::sleep(Duration::from_millis(self.timeout_ms.unwrap_or(1))).await;
        self.raw.relay_transact(request).await
    }

    async fn relay_transaction_status(
        &self,
        request: RelayTransactStatusRequest,
    ) -> Result<RelayTransactStatusResponse> {
        tokio::time::sleep(Duration::from_millis(self.timeout_ms.unwrap_or(1))).await;
        self.raw.relay_transaction_status(request).await
    }

    async fn wait_transaction(&self, request: WaitingTransactionRequest) -> Result<RelayTransactStatusResponse> {
        tokio::time::sleep(Duration::from_millis(self.timeout_ms.unwrap_or(1))).await;
        self.raw.wait_transaction(request).await
    }

    async fn handshake(&self, url: &str) -> Result<bool> {
        tokio::time::sleep(Duration::from_millis(self.timeout_ms.unwrap_or(1))).await;
        self.raw.handshake(url).await
    }
}

pub(crate) fn generate_private_key() -> (Address, String) {
    let local_wallet = LocalWallet::new(&mut rand::thread_rng());
    let address = local_wallet.address();
    let key = local_wallet.signer().to_bytes();
    (address, encode_hex(key))
}
