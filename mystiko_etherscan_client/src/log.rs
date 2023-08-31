use ethers_core::types::{Address, Bytes, TxHash, H256, U64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Log {
    pub address: Address,

    pub topics: Vec<H256>,

    pub data: Bytes,

    #[serde(rename = "blockHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<H256>,

    #[serde(rename = "blockNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<U64>,

    #[serde(rename = "transactionHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<H256>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogMeta {
    pub address: Address,

    pub block_number: U64,

    pub block_hash: H256,

    pub transaction_hash: TxHash,
}

impl Log {
    pub fn into_raw(self) -> ethers_core::abi::RawLog {
        ethers_core::abi::RawLog {
            topics: self.topics,
            data: self.data.to_vec(),
        }
    }
}

impl From<&Log> for LogMeta {
    fn from(src: &Log) -> Self {
        LogMeta {
            address: src.address,
            block_number: src.block_number.expect("should have a block number"),
            block_hash: src.block_hash.expect("should have a block hash"),
            transaction_hash: src.transaction_hash.expect("should have a tx hash"),
        }
    }
}
