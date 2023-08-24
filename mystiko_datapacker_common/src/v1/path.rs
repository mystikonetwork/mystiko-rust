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
