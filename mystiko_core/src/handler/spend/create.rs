use crate::handler::SpendContext;
use crate::{CommitmentPoolContractHandler, Spend, Spends, SpendsError};
use mystiko_datapacker_client::DataPackerClient;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::Wallet;
use mystiko_protos::core::handler::v1::CreateSpendOptions;
use mystiko_protos::core::v1::SpendStatus;
use mystiko_protos::data::v1::{ChainData, MerkleTree};
use mystiko_relayer_client::RelayerClient;
use mystiko_storage::{StatementFormatter, Storage};
use num_bigint::BigUint;
use num_traits::Zero;

impl<F, S, A, C, T, P, R, V, K, X> Spends<F, S, A, C, T, P, R, V, K, X>
where
    F: StatementFormatter,
    S: Storage,
    K: DataPackerClient<ChainData, MerkleTree>,
    C: CommitmentPoolContractHandler,
    R: RelayerClient,
    SpendsError: From<C::Error> + From<R::Error>,
{
    pub(crate) async fn execute_create(
        &self,
        wallet: &Wallet,
        options: &CreateSpendOptions,
    ) -> Result<Spend, SpendsError> {
        let context = SpendContext::from_create_options(self.config.clone(), options)?;
        let bridge_type: BridgeType = context.contract_config.bridge_type().into();
        let (quote, summary) = self.execute_summary_with_context(&context, options).await?;
        Ok(Spend {
            chain_id: options.chain_id,
            contract_address: context.contract_config.address().to_string(),
            bridge_type: bridge_type as i32,
            asset_symbol: quote.asset_symbol.clone(),
            asset_decimals: quote.asset_decimals,
            asset_address: context
                .contract_config
                .asset_address()
                .map(|address| address.to_string()),
            proof: None,
            root_hash: None,
            input_commitments: quote.selected_commitments_as_biguint()?,
            output_commitments: Some(vec![BigUint::zero(); quote.num_of_outputs as usize]),
            nullifiers: None,
            signature_public_key: None,
            signature_public_key_hashes: None,
            amount: summary.amount,
            decimal_amount: summary.decimal_amount_as_biguint()?,
            recipient: summary.recipient.clone(),
            rollup_fee_amount: (quote.num_of_outputs > 0).then_some(summary.rollup_fee_amount),
            rollup_fee_decimal_amount: (quote.num_of_outputs > 0)
                .then_some(summary.rollup_fee_decimal_amount_as_biguint()?),
            rollup_fee_total_amount: (quote.num_of_outputs > 0).then_some(summary.rollup_fee_total_amount),
            rollup_fee_total_decimal_amount: (quote.num_of_outputs > 0)
                .then_some(summary.rollup_fee_total_decimal_amount_as_biguint()?),
            gas_relayer_fee_amount: summary.gas_relayer_fee_amount,
            gas_relayer_fee_decimal_amount: summary.gas_relayer_fee_decimal_amount_as_biguint()?,
            gas_relayer_address: summary.gas_relayer_address,
            gas_relayer_url: summary.gas_relayer_url,
            signature: None,
            random_auditing_public_key: None,
            encrypted_auditor_notes: None,
            spend_type: options.spend_type() as i32,
            status: SpendStatus::Unspecified as i32,
            error_message: None,
            transaction_hash: None,
            wallet_id: wallet.id.to_string(),
        })
    }
}
