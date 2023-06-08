use ethers_core::types::transaction::eip2930::AccessList;
use ethers_core::types::{Address, Bytes, Transaction, TransactionReceipt, H256, U256};
use mystiko_crypto::zkp::proof::{G1Point, G2Point, Proof, ZKProof};
use mystiko_protocol::rollup::RollupProof;
use mystiko_roller::data::handler::ProofInfo;
use std::str::FromStr;

pub fn get_proof() -> ProofInfo {
    ProofInfo {
        r: RollupProof {
            zk_proof: ZKProof {
                proof: Proof {
                    a: G1Point {
                        x: "0x29fbefd6cb599c09888dd65220052e90f90e748e48cbd56162b916e865853772".to_string(),
                        y: "0x1c6e2baeeb85f42516e5ebdbcb83a2fce361c20bfd62e68dfcfe0e923330b413".to_string(),
                    },
                    b: G2Point {
                        x: [
                            "0x2c56f5e4cb7a464f718514dd868f5f98a5d620250669b941b85a499342f63540".to_string(),
                            "0x14cb5c1abd619455933aeed2553a7c099c579291e509f393ac2de135482c4bb0".to_string(),
                        ],
                        y: [
                            "0x1fe3a4f88757932d6322c4e818463c2bb0ee6abd3122f06d94611dacde460ffa".to_string(),
                            "0x1e4841ca07a44d756c0791c6e18e260953755d0a9a2b78c9a0f253e76d2c1689".to_string(),
                        ],
                    },
                    c: G1Point {
                        x: "0x11da97a1df5fba3cf1e3dcb9502ec4565b256c42a162fffa945a0e9900ba2976".to_string(),
                        y: "0x0a7382541ea8171ff5e7c40ba4b7ee9671aa007282859bd18a259a6796813018".to_string(),
                    },
                },
                inputs: vec![
                    "0x22c21d5862b87489724035886342e313087df95a70b0bd042ef03c638a0bf46c".to_string(),
                    "0x1e75e854260840f462b3de02fd4a96195fe5bce0fe4dbd435c36d48f13f006d6".to_string(),
                    "0x05d3245056163cdb31f2b57bfce5a7a65cf86d4feb0fba04c0ad9c1dafcd3c20".to_string(),
                    "0x0000000000000000000000000000000000000000000000000000000000000001".to_string(),
                ],
            },
            new_root: Default::default(),
            leaves_hash: Default::default(),
        },
    }
}
pub fn get_transaction() -> Transaction {
    Transaction {
        hash: H256::from_str("5e2fc091e15119c97722e9b63d5d32b043d077d834f377b91f80d32872c78109").unwrap(),
        nonce: 65.into(),
        block_hash: Some(H256::from_str("f43869e67c02c57d1f9a07bb897b54bec1cfa1feb704d91a2ee087566de5df2c").unwrap()),
        block_number: Some(6203173.into()),
        transaction_index: Some(10.into()),
        from: Address::from_str("e66b278fa9fbb181522f6916ec2f6d66ab846e04").unwrap(),
        to: Some(Address::from_str("11d7c2ab0d4aa26b7d8502f6a7ef6844908495c2").unwrap()),
        value: 0.into(),
        gas_price: Some(1500000007.into()),
        gas: 106703.into(),
        input: Bytes::from_str("0xe5225381").unwrap(),
        v: 1.into(),
        r: U256::from_str_radix(
            "12010114865104992543118914714169554862963471200433926679648874237672573604889",
            10,
        )
        .unwrap(),
        s: U256::from_str_radix(
            "22830728216401371437656932733690354795366167672037272747970692473382669718804",
            10,
        )
        .unwrap(),
        transaction_type: Some(2.into()),
        access_list: Some(AccessList::default()),
        max_priority_fee_per_gas: Some(1500000000.into()),
        max_fee_per_gas: Some(1500000009.into()),
        chain_id: Some(5.into()),
        other: Default::default(),
    }
}

pub fn get_transaction_receipt() -> TransactionReceipt {
    let v: serde_json::Value = serde_json::from_str(
        r#"{
        "transactionHash": "0x611b173b0e0dfda94da7bfb6cb77c9f1c03e2f2149ba060e6bddfaa219942369",
        "blockHash": "0xa11871d61e0e703ae33b358a6a9653c43e4216f277d4a1c7377b76b4d5b4cbf1",
        "blockNumber": "0xe3c1d8",
        "contractAddress": "0x08f6db30039218894067023a3593baf27d3f4a2b",
        "cumulativeGasUsed": "0x1246047",
        "effectiveGasPrice": "0xa02ffee00",
        "from": "0x0968995a48162a23af60d3ca25cddfa143cd8891",
        "gasUsed": "0x1b9229",
        "logs": [
          {
            "address": "0x08f6db30039218894067023a3593baf27d3f4a2b",
            "topics": [
              "0x40c340f65e17194d14ddddb073d3c9f888e3cb52b5aae0c6c7706b4fbc905fac"
            ],
            "data": "0x0000000000000000000000000968995a48162a23af60d3ca25cddfa143cd88910000000000000000000000000000000000000000000000000000000000002616",
            "blockNumber": "0xe3c1d8",
            "transactionHash": "0x611b173b0e0dfda94da7bfb6cb77c9f1c03e2f2149ba060e6bddfaa219942369",
            "transactionIndex": "0xdf",
            "blockHash": "0xa11871d61e0e703ae33b358a6a9653c43e4216f277d4a1c7377b76b4d5b4cbf1",
            "logIndex": "0x196",
            "removed": false
          },
          {
            "address": "0x08f6db30039218894067023a3593baf27d3f4a2b",
            "topics": [
              "0x40c340f65e17194d14ddddb073d3c9f888e3cb52b5aae0c6c7706b4fbc905fac"
            ],
            "data": "0x00000000000000000000000059750ac0631f63bfdce0f0867618e468e11ee34700000000000000000000000000000000000000000000000000000000000000fa",
            "blockNumber": "0xe3c1d8",
            "transactionHash": "0x611b173b0e0dfda94da7bfb6cb77c9f1c03e2f2149ba060e6bddfaa219942369",
            "transactionIndex": "0xdf",
            "blockHash": "0xa11871d61e0e703ae33b358a6a9653c43e4216f277d4a1c7377b76b4d5b4cbf1",
            "logIndex": "0x197",
            "removed": false
          }
        ],
        "logsBloom": "0x00000000000000800000000040000000000000000000000000000000000000000000008000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "status": "0x1",
        "to": null,
        "transactionIndex": "0xdf",
        "type": "0x2"
        }
        "#,
    ).unwrap();

    let receipt: TransactionReceipt = serde_json::from_value(v).unwrap();
    receipt
}
