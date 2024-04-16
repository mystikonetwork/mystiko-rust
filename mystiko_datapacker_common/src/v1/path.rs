use std::path::PathBuf;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PathSchema {
    #[builder(default = default_path_base())]
    base: PathBuf,
    #[builder(default = String::from("zst"))]
    data_suffix: String,
    #[builder(default = String::from("sha512"))]
    checksum_suffix: String,
}

impl Default for PathSchema {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl crate::PathSchema for PathSchema {
    fn chain_path(&self, chain_id: u64) -> PathBuf {
        self.base.join("chains").join(format!("{}", chain_id))
    }

    fn contracts_path(&self, chain_id: u64) -> PathBuf {
        self.chain_path(chain_id).join("contracts")
    }

    fn contract_path(&self, chain_id: u64, contract_address: &str) -> PathBuf {
        self.contracts_path(chain_id).join(contract_address)
    }

    fn merkle_tree_path(&self, chain_id: u64, contract_address: &str) -> PathBuf {
        self.contract_path(chain_id, contract_address).join("merkle_tree")
    }

    fn merkle_tree_index_path(&self, chain_id: u64, contract_address: &str) -> PathBuf {
        self.merkle_tree_path(chain_id, contract_address).join("index.json")
    }

    fn merkle_tree_data_path(&self, chain_id: u64, contract_address: &str, last_leaf_index: u64) -> PathBuf {
        self.merkle_tree_path(chain_id, contract_address)
            .join(format!("{}.{}", last_leaf_index, self.data_suffix))
    }

    fn merkle_tree_latest_data_path(&self, chain_id: u64, contract_address: &str) -> PathBuf {
        self.merkle_tree_path(chain_id, contract_address)
            .join(format!("latest.{}", self.data_suffix))
    }

    fn granularities_path(&self, chain_id: u64) -> PathBuf {
        self.chain_path(chain_id).join("granularities")
    }

    fn granularity_path(&self, chain_id: u64, granularity: u64) -> PathBuf {
        self.granularities_path(chain_id).join(format!("{}", granularity))
    }

    fn granularity_index(&self, chain_id: u64, granularity: u64) -> PathBuf {
        self.granularity_path(chain_id, granularity).join("index.json")
    }

    fn blocks_path(&self, chain_id: u64, granularity: u64) -> PathBuf {
        self.granularity_path(chain_id, granularity).join("blocks")
    }

    fn data_path(&self, chain_id: u64, granularity: u64, start_block: u64) -> PathBuf {
        self.blocks_path(chain_id, granularity)
            .join(format!("{}", start_block))
            .join(format!("data.{}", self.data_suffix))
    }

    fn checksum_path(&self, chain_id: u64, granularity: u64, start_block: u64) -> PathBuf {
        self.blocks_path(chain_id, granularity)
            .join(format!("{}", start_block))
            .join(format!("data_checksum.{}", self.checksum_suffix))
    }
}

fn default_path_base() -> PathBuf {
    PathBuf::from("/")
}
