use crate::common::error::{Result, RollerError};
use crate::db::document::commitment::CommitmentInfo;
use ethers_contract::EthLogDecode;
use ethers_core::abi::{Error as AbiError, RawLog};
use ethers_core::types::Log;
use ethers_core::types::{Address, Bytes, H256, U64};
use mystiko_abi::commitment_pool::CommitmentQueuedFilter;
use mystiko_utils::convert::u256_to_biguint;
use serde::{Deserialize, Serialize};

/// A Intermediate log struct for some explorer response log_index/transactionIndex/transaction_log_index is 0x
/// can't be deserialized to U64/U256
#[derive(Serialize, Deserialize)]
pub struct IntermediateLog {
    /// H160. the contract that emitted the log
    pub address: Address,

    /// topics: Array of 0 to 4 32 Bytes of indexed log arguments.
    /// (In solidity: The first topic is the hash of the signature of the event
    /// (e.g. `Deposit(address,bytes32,uint256)`), except you declared the event
    /// with the anonymous specifier.)
    pub topics: Vec<H256>,

    /// Data
    pub data: Bytes,

    /// Block Hash
    #[serde(rename = "blockHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<H256>,

    /// Block Number
    #[serde(rename = "blockNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<U64>,

    /// Transaction Hash
    #[serde(rename = "transactionHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<H256>,

    // Transaction Index
    // #[serde(rename = "transactionIndex")]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub transaction_index: Option<U64>,
    /// Integer of the log index position in the block. None if it's a pending log.
    // #[serde(rename = "logIndex")]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub log_index: Option<U256>,

    // Integer of the transactions index position log was created from.
    // None when it's a pending log.
    // #[serde(rename = "transactionLogIndex")]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub transaction_log_index: Option<U256>,

    /// Log Type
    #[serde(rename = "logType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,

    /// True when the log was removed, due to a chain reorganization.
    /// false if it's a valid log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<bool>,
}

pub trait EventLog {
    fn block_number(&self) -> Option<U64>;
    fn transaction_hash(&self) -> Option<H256>;
    fn parse_log<D>(&self) -> std::result::Result<D, AbiError>
    where
        D: EthLogDecode;
}

impl EventLog for IntermediateLog {
    fn block_number(&self) -> Option<U64> {
        self.block_number
    }
    fn transaction_hash(&self) -> Option<H256> {
        self.transaction_hash
    }

    fn parse_log<D>(&self) -> std::result::Result<D, AbiError>
    where
        D: EthLogDecode,
    {
        D::decode_log(&RawLog {
            topics: self.topics.clone(),
            data: self.data.to_vec(),
        })
    }
}

impl EventLog for Log {
    fn block_number(&self) -> Option<U64> {
        self.block_number
    }
    fn transaction_hash(&self) -> Option<H256> {
        self.transaction_hash
    }

    fn parse_log<D>(&self) -> std::result::Result<D, AbiError>
    where
        D: EthLogDecode,
    {
        D::decode_log(&RawLog {
            topics: self.topics.clone(),
            data: self.data.to_vec(),
        })
    }
}

pub fn parse_event_logs<T>(chain_id: u64, contract_address: &str, logs: Vec<T>) -> Result<Vec<CommitmentInfo>>
where
    T: EventLog,
{
    let cms = logs
        .into_iter()
        .map(|log| {
            let block_number = log
                .block_number()
                .ok_or_else(|| RollerError::InvalidEventLogs("block number".to_string()))?
                .as_u64();
            let tx_hash = log
                .transaction_hash()
                .ok_or_else(|| RollerError::InvalidEventLogs("transaction hash".to_string()))?;
            let tx_hash = format!("{:#x}", tx_hash);
            let cm = log.parse_log::<CommitmentQueuedFilter>()?;
            Ok(CommitmentInfo {
                chain_id,
                contract_address: contract_address.to_string(),
                commitment_hash: u256_to_biguint(&cm.commitment),
                block_number,
                rollup_fee: cm.rollup_fee.to_string(),
                leaf_index: cm.leaf_index.as_u64(),
                tx_hash,
            })
        })
        .collect::<Result<Vec<CommitmentInfo>>>()?;
    Ok(cms)
}
