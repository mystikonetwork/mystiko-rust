use ethers_core::types::Log;
use mystiko_fs::read_file_bytes;
use mystiko_indexer_client::types::commitment_queued::CommitmentQueuedResponse;
use mystiko_roller::db::document::commitment::CommitmentInfo;
use serde_json::from_reader;

pub async fn load_commitments(
    file: &str,
    replace_chain_id: Option<u64>,
    replace_contract_address: Option<&str>,
) -> Vec<CommitmentInfo> {
    let bytes = read_file_bytes(file).await.unwrap();
    let mut commitments: Vec<CommitmentInfo> = from_reader(bytes.as_slice()).unwrap();
    if let Some(chain_id) = replace_chain_id {
        commitments.iter_mut().for_each(|cm| {
            cm.chain_id = chain_id;
        });
    }
    if let Some(contract_address) = replace_contract_address {
        commitments.iter_mut().for_each(|cm| {
            cm.contract_address = contract_address.to_string();
        });
    }

    commitments
}

pub async fn load_commitment_logs(file: &str) -> Vec<Log> {
    let bytes = read_file_bytes(file).await.unwrap();
    let commitments: Vec<Log> = from_reader(bytes.as_slice()).unwrap();
    commitments
}

pub async fn load_indexer_commitments(
    file: &str,
    replace_chain_id: Option<u64>,
    replace_contract_address: Option<&str>,
) -> Vec<CommitmentQueuedResponse> {
    let cms = load_commitments(file, replace_chain_id, replace_contract_address).await;
    cms.iter()
        .map(|cm| CommitmentQueuedResponse {
            id: 0,
            chain_id: cm.chain_id,
            contract_address: cm.contract_address.clone(),
            commit_hash: cm.commitment_hash.to_string(),
            tx_hash: cm.tx_hash.to_string(),
            block_num: cm.block_number,
            encrypted_note: "".to_string(),
            rollup_fee: cm.rollup_fee.to_string(),
            leaf_index: cm.leaf_index as u32,
            create_at: None,
            status: None,
            contract_id: 0,
        })
        .collect()
}
