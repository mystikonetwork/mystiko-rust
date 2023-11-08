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
        let value = ether_tx.value.as_ref().map(mystiko_utils::convert::u256_to_hex_string);
        let gas = ether_tx.gas.as_ref().map(mystiko_utils::convert::u256_to_hex_string);
        let gas_price = ether_tx
            .gas_price
            .as_ref()
            .map(mystiko_utils::convert::u256_to_hex_string);
        let nonce = ether_tx.nonce.as_ref().map(mystiko_utils::convert::u256_to_hex_string);
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
impl TryFrom<crate::core::v1::LegacyTransaction> for ethers_core::types::TransactionRequest {
    type Error = anyhow::Error;

    fn try_from(tx: crate::core::v1::LegacyTransaction) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let from = tx
            .from
            .as_ref()
            .map(mystiko_utils::address::ethers_address_from_string)
            .transpose()?;
        let to = tx
            .to
            .as_ref()
            .map(mystiko_utils::address::ethers_address_from_string)
            .transpose()?;
        let value = tx
            .value
            .as_ref()
            .map(mystiko_utils::convert::hex_string_to_u256)
            .transpose()?;
        let gas = tx
            .gas
            .as_ref()
            .map(mystiko_utils::convert::hex_string_to_u256)
            .transpose()?;
        let gas_price = tx
            .gas_price
            .as_ref()
            .map(mystiko_utils::convert::hex_string_to_u256)
            .transpose()?;
        let nonce = tx
            .nonce
            .as_ref()
            .map(mystiko_utils::convert::hex_string_to_u256)
            .transpose()?;
        let data = tx
            .data
            .as_ref()
            .map(|data| ethers_core::types::Bytes::from_str(data))
            .transpose()?;
        Ok(Self {
            from,
            to: to.map(ethers_core::types::NameOrAddress::Address),
            value,
            gas,
            gas_price,
            nonce,
            data,
            chain_id: tx.chain_id.map(ethers_core::types::U64::from),
        })
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
        let value = ether_tx.value.as_ref().map(mystiko_utils::convert::u256_to_hex_string);
        let gas = ether_tx.gas.as_ref().map(mystiko_utils::convert::u256_to_hex_string);
        let max_fee_per_gas = ether_tx
            .max_fee_per_gas
            .as_ref()
            .map(mystiko_utils::convert::u256_to_hex_string);
        let max_priority_fee_per_gas = ether_tx
            .max_priority_fee_per_gas
            .as_ref()
            .map(mystiko_utils::convert::u256_to_hex_string);
        let nonce = ether_tx.nonce.as_ref().map(mystiko_utils::convert::u256_to_hex_string);
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
impl TryFrom<crate::core::v1::Eip1559Transaction>
    for ethers_core::types::transaction::eip1559::Eip1559TransactionRequest
{
    type Error = anyhow::Error;

    fn try_from(tx: crate::core::v1::Eip1559Transaction) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let from = tx
            .from
            .as_ref()
            .map(mystiko_utils::address::ethers_address_from_string)
            .transpose()?;
        let to = tx
            .to
            .as_ref()
            .map(mystiko_utils::address::ethers_address_from_string)
            .transpose()?;
        let value = tx
            .value
            .as_ref()
            .map(mystiko_utils::convert::hex_string_to_u256)
            .transpose()?;
        let gas = tx
            .gas
            .as_ref()
            .map(mystiko_utils::convert::hex_string_to_u256)
            .transpose()?;
        let max_fee_per_gas = tx
            .max_fee_per_gas
            .as_ref()
            .map(mystiko_utils::convert::hex_string_to_u256)
            .transpose()?;
        let max_priority_fee_per_gas = tx
            .max_priority_fee_per_gas
            .as_ref()
            .map(mystiko_utils::convert::hex_string_to_u256)
            .transpose()?;
        let nonce = tx
            .nonce
            .as_ref()
            .map(mystiko_utils::convert::hex_string_to_u256)
            .transpose()?;
        let data = tx
            .data
            .as_ref()
            .map(|data| ethers_core::types::Bytes::from_str(data))
            .transpose()?;
        let access_list = from_access_list(tx.access_list)?;
        Ok(Self {
            from,
            to: to.map(ethers_core::types::NameOrAddress::Address),
            value,
            gas,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            nonce,
            data,
            chain_id: tx.chain_id.map(ethers_core::types::U64::from),
            access_list,
        })
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
impl TryFrom<crate::core::v1::Eip2930Transaction>
    for ethers_core::types::transaction::eip2930::Eip2930TransactionRequest
{
    type Error = anyhow::Error;

    fn try_from(tx: crate::core::v1::Eip2930Transaction) -> Result<Self, Self::Error> {
        Ok(Self {
            tx: tx.tx.unwrap_or_default().try_into()?,
            access_list: from_access_list(tx.access_list)?,
        })
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
impl TryFrom<crate::core::v1::Transaction> for ethers_core::types::transaction::eip2718::TypedTransaction {
    type Error = anyhow::Error;

    fn try_from(tx: crate::core::v1::Transaction) -> Result<Self, Self::Error> {
        match tx.transaction {
            Some(crate::core::v1::transaction::Transaction::LegacyTransaction(tx)) => Ok(
                ethers_core::types::transaction::eip2718::TypedTransaction::Legacy(tx.try_into()?),
            ),
            Some(crate::core::v1::transaction::Transaction::Eip1559Transaction(tx)) => Ok(
                ethers_core::types::transaction::eip2718::TypedTransaction::Eip1559(tx.try_into()?),
            ),
            Some(crate::core::v1::transaction::Transaction::Eip2930Transaction(tx)) => Ok(
                ethers_core::types::transaction::eip2718::TypedTransaction::Eip2930(tx.try_into()?),
            ),
            None => Err(anyhow::anyhow!("Transaction is missing")),
        }
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
impl TryFrom<crate::core::v1::AccessListItem> for ethers_core::types::transaction::eip2930::AccessListItem {
    type Error = anyhow::Error;

    fn try_from(item: crate::core::v1::AccessListItem) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let storage_keys = item
            .storage_keys
            .into_iter()
            .map(|key| ethers_core::types::H256::from_str(&key))
            .collect::<Result<Vec<_>, _>>()?;
        let address = mystiko_utils::address::ethers_address_from_string(&item.address)?;
        Ok(Self { address, storage_keys })
    }
}

#[cfg(feature = "ethers")]
fn into_access_list(
    access_list: ethers_core::types::transaction::eip2930::AccessList,
) -> Vec<crate::core::v1::AccessListItem> {
    access_list.0.into_iter().map(|item| item.into()).collect::<Vec<_>>()
}

#[cfg(feature = "ethers")]
fn from_access_list(
    access_list: Vec<crate::core::v1::AccessListItem>,
) -> anyhow::Result<ethers_core::types::transaction::eip2930::AccessList> {
    let access_list = access_list
        .into_iter()
        .map(ethers_core::types::transaction::eip2930::AccessListItem::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ethers_core::types::transaction::eip2930::AccessList(access_list))
}
