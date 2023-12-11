use ethers_core::abi::AbiEncode;
use ethers_core::types::TxHash;
use mystiko_config::MystikoConfig;
use mystiko_protos::storage::v1::{ColumnValue, QueryFilter, SubFilter};
use std::sync::Arc;

pub(crate) fn wrap_filter<F, C, V>(filter: F, column: C, value: V) -> QueryFilter
where
    F: Into<QueryFilter>,
    C: ToString,
    V: Into<ColumnValue>,
{
    let mut filter = filter.into();
    let sub_filter = SubFilter::equal(column, value);
    filter.additional_condition = Some(sub_filter.into());
    filter
}

pub(crate) fn format_tx_hash(config: Arc<MystikoConfig>, chain_id: u64, tx_hash: &TxHash) -> Option<String> {
    config
        .transaction_url(chain_id, &tx_hash.encode_hex())
        .map(|tx_url| format!("tx({})", tx_url))
}
