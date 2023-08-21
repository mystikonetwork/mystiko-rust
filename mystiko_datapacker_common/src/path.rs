use std::path::PathBuf;

pub trait PathSchema {
    fn chain_path(&self, chain_id: u64) -> PathBuf;
    fn granularities_path(&self, chain_id: u64) -> PathBuf;
    fn granularity_path(&self, chain_id: u64, granularity: u64) -> PathBuf;
    fn granularity_index(&self, chain_id: u64, granularity: u64) -> PathBuf;
    fn blocks_path(&self, chain_id: u64, granularity: u64) -> PathBuf;
    fn data_path(&self, chain_id: u64, granularity: u64, start_block: u64) -> PathBuf;
    fn checksum_path(&self, chain_id: u64, granularity: u64, start_block: u64) -> PathBuf;
}
