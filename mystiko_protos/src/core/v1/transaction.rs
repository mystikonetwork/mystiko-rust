#[cfg(feature = "ethers")]
impl From<ethers_core::types::TransactionRequest> for crate::core::v1::Transaction {
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
        crate::core::v1::Transaction::builder()
            .from(from)
            .to(to)
            .value(value)
            .gas(gas)
            .gas_price(gas_price)
            .nonce(nonce)
            .data(data)
            .build()
    }
}
