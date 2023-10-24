#[cfg(feature = "ethers")]
impl From<ethers_core::types::TransactionRequest> for crate::core::v1::LegacyTransaction {
    fn from(ether_tx: ethers_core::types::TransactionRequest) -> Self {
        let from = ether_tx
            .from
            .as_ref()
            .map(mystiko_utils::address::ethers_address_to_string);
        let to = ether_tx
            .to
            .as_ref()
            .and_then(|to| to.as_address())
            .map(mystiko_utils::address::ethers_address_to_string);
        let value = ether_tx.value.as_ref().map(|value| format!("0x{:x}", value));
        let gas = ether_tx.gas.as_ref().map(|gas| format!("0x{:x}", gas));
        let gas_price = ether_tx
            .gas_price
            .as_ref()
            .map(|gas_price| format!("0x{:x}", gas_price));
        let nonce = ether_tx.nonce.as_ref().map(|nonce| format!("0x{:x}", nonce));
        let data = ether_tx.data.map(|data| format!("{:x}", data));
        crate::core::v1::LegacyTransaction::builder()
            .from(from)
            .to(to)
            .value(value)
            .gas(gas)
            .gas_price(gas_price)
            .nonce(nonce)
            .data(data)
            .chain_id(ether_tx.chain_id.map(|chain_id| chain_id.as_u64()))
            .build()
    }
}

#[cfg(feature = "ethers")]
impl From<ethers_core::types::transaction::eip1559::Eip1559TransactionRequest> for crate::core::v1::Eip1559Transaction {
    fn from(ether_tx: ethers_core::types::transaction::eip1559::Eip1559TransactionRequest) -> Self {
        let from = ether_tx
            .from
            .as_ref()
            .map(mystiko_utils::address::ethers_address_to_string);
        let to = ether_tx
            .to
            .as_ref()
            .and_then(|to| to.as_address())
            .map(mystiko_utils::address::ethers_address_to_string);
        let value = ether_tx.value.as_ref().map(|value| format!("0x{:x}", value));
        let gas = ether_tx.gas.as_ref().map(|gas| format!("0x{:x}", gas));
        let max_fee_per_gas = ether_tx.max_fee_per_gas.as_ref().map(|max| format!("0x{:x}", max));
        let max_priority_fee_per_gas = ether_tx
            .max_priority_fee_per_gas
            .as_ref()
            .map(|max| format!("0x{:x}", max));
        let nonce = ether_tx.nonce.as_ref().map(|nonce| format!("0x{:x}", nonce));
        let data = ether_tx.data.map(|data| format!("{:x}", data));
        crate::core::v1::Eip1559Transaction::builder()
            .from(from)
            .to(to)
            .value(value)
            .gas(gas)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .nonce(nonce)
            .data(data)
            .chain_id(ether_tx.chain_id.map(|chain_id| chain_id.as_u64()))
            .access_list(into_access_list(ether_tx.access_list))
            .build()
    }
}

#[cfg(feature = "ethers")]
impl From<ethers_core::types::transaction::eip2930::Eip2930TransactionRequest> for crate::core::v1::Eip2930Transaction {
    fn from(ethers_tx: ethers_core::types::transaction::eip2930::Eip2930TransactionRequest) -> Self {
        crate::core::v1::Eip2930Transaction::builder()
            .tx(Some(ethers_tx.tx.into()))
            .access_list(into_access_list(ethers_tx.access_list))
            .build()
    }
}

#[cfg(feature = "ethers")]
impl From<ethers_core::types::transaction::eip2718::TypedTransaction> for crate::core::v1::Transaction {
    fn from(ethers_tx: ethers_core::types::transaction::eip2718::TypedTransaction) -> Self {
        let tx = match ethers_tx {
            ethers_core::types::transaction::eip2718::TypedTransaction::Legacy(tx) => {
                crate::core::v1::transaction::Transaction::LegacyTransaction(tx.into())
            }
            ethers_core::types::transaction::eip2718::TypedTransaction::Eip1559(tx) => {
                crate::core::v1::transaction::Transaction::Eip1559Transaction(tx.into())
            }
            ethers_core::types::transaction::eip2718::TypedTransaction::Eip2930(tx) => {
                crate::core::v1::transaction::Transaction::Eip2930Transaction(tx.into())
            }
        };
        crate::core::v1::Transaction::builder().transaction(tx).build()
    }
}

#[cfg(feature = "ethers")]
impl From<ethers_core::types::transaction::eip2930::AccessListItem> for crate::core::v1::AccessListItem {
    fn from(item: ethers_core::types::transaction::eip2930::AccessListItem) -> Self {
        let storage_keys = item
            .storage_keys
            .into_iter()
            .map(|key| format!("0x{:x}", key))
            .collect::<Vec<_>>();
        crate::core::v1::AccessListItem::builder()
            .address(mystiko_utils::address::ethers_address_to_string(&item.address))
            .storage_keys(storage_keys)
            .build()
    }
}

#[cfg(feature = "ethers")]
fn into_access_list(
    access_list: ethers_core::types::transaction::eip2930::AccessList,
) -> Vec<crate::core::v1::AccessListItem> {
    access_list.0.into_iter().map(|item| item.into()).collect::<Vec<_>>()
}
