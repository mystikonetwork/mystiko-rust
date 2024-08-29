use crate::handler::DepositContext;
use crate::{
    CommitmentPoolContractHandler, DepositContractHandler, Deposits, DepositsError, PublicAssetHandler,
    ScreeningOptions, TransactionHandler, TransactionSigner,
};
use ethers_core::types::Address;
use mystiko_protos::core::handler::v1::SendDepositOptions;
use mystiko_protos::core::v1::Transaction;
use mystiko_screening_client::{ScreeningClient, ScreeningRequest};
use mystiko_storage::{StatementFormatter, Storage};
use mystiko_utils::address::ethers_address_to_string;
use mystiko_utils::hex::decode_hex;
use std::sync::Arc;

pub const DEFAULT_ADDRESS_SCREENING_MESSAGE: &str = "For address screening purposes, please sign this message to confirm ownership of your connected wallet. No additional permissions are needed.";

impl<F, S, A, D, C, T, P, N> Deposits<F, S, A, D, C, T, P, N>
where
    F: StatementFormatter,
    S: Storage,
    A: PublicAssetHandler,
    D: DepositContractHandler,
    C: CommitmentPoolContractHandler,
    T: TransactionHandler<Transaction>,
    N: ScreeningClient + 'static,
    DepositsError: From<A::Error> + From<D::Error> + From<C::Error> + From<T::Error>,
{
    pub(crate) async fn address_screening<Signer>(
        &self,
        context: &DepositContext,
        options: &SendDepositOptions,
        signer: Arc<Signer>,
        owner: Address,
    ) -> Result<Option<ScreeningOptions>, DepositsError>
    where
        Signer: TransactionSigner + 'static,
    {
        if !self.is_contract_support_screening(context) {
            return Ok(None);
        }

        let account = ethers_address_to_string(&owner);
        let message = options
            .screening_message
            .clone()
            .unwrap_or(DEFAULT_ADDRESS_SCREENING_MESSAGE.to_string());
        let signature = signer.sign_message(account.clone(), message.clone()).await?;
        let request = ScreeningRequest::builder()
            .chain_id(context.chain_config.chain_id())
            .account(account)
            .message(message)
            .signature(signature)
            .asset(Some(context.contract_config.asset().asset_address().to_string()))
            .build();
        let response = self.screening.address_screening(&request).await?;
        let signature = decode_hex(response.signature)?;
        Ok(Some(
            ScreeningOptions::builder()
                .deadline(response.deadline)
                .signature(signature)
                .build(),
        ))
    }

    fn is_contract_support_screening(&self, context: &DepositContext) -> bool {
        context.contract_config.version() > 6
    }
}
