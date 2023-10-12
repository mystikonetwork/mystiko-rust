use crate::ChainSynchronizer;
use mystiko_dataloader::loader::DataLoader;

#[derive(Debug)]
pub struct Synchronizer<L: DataLoader> {
    chains: Vec<ChainSynchronizer<L>>,
}
