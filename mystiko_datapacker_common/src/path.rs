use std::path::PathBuf;

pub trait PathSchema: Send + Sync {
    fn chain_path(&self, chain_id: u64) -> PathBuf;
    fn granularities_path(&self, chain_id: u64) -> PathBuf;
    fn granularity_path(&self, chain_id: u64, granularity: u64) -> PathBuf;
    fn granularity_index(&self, chain_id: u64, granularity: u64) -> PathBuf;
    fn blocks_path(&self, chain_id: u64, granularity: u64) -> PathBuf;
    fn data_path(&self, chain_id: u64, granularity: u64, start_block: u64) -> PathBuf;
    fn checksum_path(&self, chain_id: u64, granularity: u64, start_block: u64) -> PathBuf;
}

impl PathSchema for Box<dyn PathSchema> {
    fn chain_path(&self, chain_id: u64) -> PathBuf {
        self.as_ref().chain_path(chain_id)
    }

    fn granularities_path(&self, chain_id: u64) -> PathBuf {
        self.as_ref().granularities_path(chain_id)
    }

    fn granularity_path(&self, chain_id: u64, granularity: u64) -> PathBuf {
        self.as_ref().granularity_path(chain_id, granularity)
    }

    fn granularity_index(&self, chain_id: u64, granularity: u64) -> PathBuf {
        self.as_ref().granularity_index(chain_id, granularity)
    }

    fn blocks_path(&self, chain_id: u64, granularity: u64) -> PathBuf {
        self.as_ref().blocks_path(chain_id, granularity)
    }

    fn data_path(&self, chain_id: u64, granularity: u64, start_block: u64) -> PathBuf {
        self.as_ref().data_path(chain_id, granularity, start_block)
    }

    fn checksum_path(&self, chain_id: u64, granularity: u64, start_block: u64) -> PathBuf {
        self.as_ref().checksum_path(chain_id, granularity, start_block)
    }
}
