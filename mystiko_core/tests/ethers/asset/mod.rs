mod handler_tests;

use async_trait::async_trait;
use ethers_core::abi::{AbiDecode, AbiEncode};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};
use mockall::mock;
use mystiko_core::TransactionSigner;
use mystiko_utils::address::{ethers_address_from_string, ethers_address_to_string};

#[tokio::test]
async fn test_box_impl() {
    let mut signer = MockTransactionSigner::new();
    signer
        .expect_address()
        .returning(|| Ok(ethers_address_from_string("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap()));
    signer.expect_send_transaction().returning(|_, _| {
        Ok(TxHash::decode_hex("0xe872b68aadebb38151ceb1b965db7e0cbd6d1789710aead6360eca4893000c43").unwrap())
    });
    let signer = Box::new(signer) as Box<dyn TransactionSigner>;
    assert_eq!(
        ethers_address_to_string(&signer.address().await.unwrap()),
        "0x388C818CA8B9251b393131C08a736A67ccB19297"
    );
    assert_eq!(
        signer
            .send_transaction(1, TypedTransaction::default())
            .await
            .unwrap()
            .encode_hex(),
        "0xe872b68aadebb38151ceb1b965db7e0cbd6d1789710aead6360eca4893000c43"
    );
}

mock! {
    TransactionSigner {}

    #[async_trait]
    impl mystiko_core::TransactionSigner for TransactionSigner {
        async fn address(&self) -> anyhow::Result<Address>;
        async fn send_transaction(&self, chain_id: u64, tx: TypedTransaction) -> anyhow::Result<TxHash>;
    }
}
