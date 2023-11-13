mod account_tests;
mod deposit_tests;
mod wallet_tests;

use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{TransactionReceipt, TxHash, U256};
use mockall::mock;
use mystiko_core::{
    BalanceOptions, CommitmentPoolContractHandler, CrossChainDepositOptions, DepositContractHandler, DepositOptions,
    DepositQuote, DepositQuoteOptions, Erc20AllowanceOptions, Erc20ApproveOptions, Erc20BalanceOptions,
    Erc20TransferOptions, IsHistoricCommitmentOptions, IsKnownRootOptions, IsSpentNullifierOptions, PublicAssetHandler,
    TransactionHandler, TransactionSigner, TransferOptions, WaitOptions,
};
use mystiko_protos::core::v1::Transaction;
use mystiko_types::TransactionType;

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
    }
}

mock! {
    #[derive(Debug, Default)]
    Transactions {}

    #[async_trait]
    impl TransactionHandler<Transaction> for Transactions {
        type Error = anyhow::Error;
        fn create(&self, tx: Option<Transaction>, tx_type: &TransactionType) -> Result<TypedTransaction>;
        async fn wait(&self, options: WaitOptions) -> Result<Option<TransactionReceipt>>;
    }
}
