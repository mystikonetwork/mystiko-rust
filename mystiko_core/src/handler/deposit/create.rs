use crate::{Deposit, DepositContractHandler, Deposits, DepositsError};
use crate::{DepositContext, WalletHandler};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::CreateDepositOptions;
use mystiko_protos::core::v1::DepositStatus;
use mystiko_storage::{StatementFormatter, Storage};
use mystiko_utils::convert::bytes_to_biguint;
use mystiko_utils::hex::encode_hex_with_prefix;

impl<F, S, A, D, C, T, P> Deposits<F, S, A, D, C, T, P>
where
    F: StatementFormatter,
    S: Storage,
    D: DepositContractHandler,
    DepositsError: From<D::Error>,
{
    pub(crate) async fn execute_create(&self, options: &CreateDepositOptions) -> Result<Deposit, DepositsError> {
        let wallet = self.wallets.check_current().await?;
        let summary = self.execute_summary(options).await?;
        let context = DepositContext::from_create_options(self.config.clone(), options)?;
        let dst_chain_id = context.contract_config.peer_chain_id().unwrap_or(options.chain_id);
        let dst_chain_contract_address = context
            .contract_config
            .peer_contract_address()
            .unwrap_or(context.contract_config.address())
            .to_string();
        let dst_pool_address = context
            .peer_contract_config
            .as_ref()
            .map(|c| c.pool_contract_address())
            .unwrap_or(context.contract_config.pool_contract_address())
            .to_string();
        let commitment = mystiko_protocol::commitment::Commitment::new(
            mystiko_protocol::address::ShieldedAddress::from_string(&options.shielded_address)?,
            Some(mystiko_protocol::commitment::Note::new(
                Some(summary.decimal_amount_as_biguint()?),
                None,
            )?),
            None,
        )?;
        let bridge_type: BridgeType = context.contract_config.bridge_type().into();
        Ok(Deposit {
            chain_id: options.chain_id,
            contract_address: context.contract_config.address().to_string(),
            pool_address: context.contract_config.pool_contract_address().to_string(),
            dst_chain_id,
            dst_chain_contract_address,
            dst_pool_address,
            commitment_hash: commitment.commitment_hash,
            hash_k: commitment.k,
            random_s: bytes_to_biguint(commitment.note.random_s),
            encrypted_note: encode_hex_with_prefix(commitment.encrypted_note),
            asset_symbol: context.contract_config.asset_symbol().to_string(),
            asset_decimals: summary.asset_decimals,
            asset_address: (context.contract_config.asset_type() == &mystiko_types::AssetType::Erc20)
                .then_some(context.contract_config.asset().asset_address().to_string()),
            bridge_type: bridge_type as i32,
            amount: options.amount,
            decimal_amount: summary.decimal_amount_as_biguint()?,
            rollup_fee_amount: summary.rollup_fee_amount,
            rollup_fee_decimal_amount: summary.rollup_fee_decimal_amount_as_biguint()?,
            bridge_fee_amount: summary.bridge_fee_amount,
            bridge_fee_decimal_amount: summary.bridge_fee_decimal_amount_as_biguint()?,
            bridge_fee_asset_address: (context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop
                && context.contract_config.bridge_fee_asset_address().is_some())
            .then_some(context.contract_config.bridge_fee_asset().asset_address().to_string()),
            bridge_fee_asset_symbol: summary.bridge_fee_asset_symbol.clone(),
            bridge_fee_asset_decimals: summary.bridge_fee_asset_decimals,
            executor_fee_amount: summary.executor_fee_amount,
            executor_fee_decimal_amount: summary.executor_fee_decimal_amount_as_biguint()?,
            executor_fee_asset_address: (context.contract_config.bridge_type() != &mystiko_types::BridgeType::Loop
                && context.contract_config.executor_fee_asset_address().is_some())
            .then_some(context.contract_config.executor_fee_asset().asset_address().to_string()),
            executor_fee_asset_symbol: summary.executor_fee_asset_symbol.clone(),
            executor_fee_asset_decimals: summary.executor_fee_asset_decimals,
            shielded_address: options.shielded_address.clone(),
            status: DepositStatus::Unspecified as i32,
            error_message: None,
            wallet_id: wallet.id,
            asset_approve_transaction_hash: None,
            src_chain_transaction_hash: None,
            queued_transaction_hash: None,
            included_transaction_hash: None,
        })
    }
}
