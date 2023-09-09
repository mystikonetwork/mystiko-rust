use crate::loader::v1::{EtherscanFetcherConfig, FetcherConfig, IndexerFetcherConfig, ProviderFetcherConfig};

impl From<Option<FetcherConfig>> for FetcherConfig {
    fn from(opt: Option<FetcherConfig>) -> FetcherConfig {
        opt.unwrap_or_default()
    }
}

impl From<Option<IndexerFetcherConfig>> for IndexerFetcherConfig {
    fn from(opt: Option<IndexerFetcherConfig>) -> IndexerFetcherConfig {
        opt.unwrap_or_default()
    }
}

impl From<Option<EtherscanFetcherConfig>> for EtherscanFetcherConfig {
    fn from(opt: Option<EtherscanFetcherConfig>) -> EtherscanFetcherConfig {
        opt.unwrap_or_default()
    }
}

impl From<Option<ProviderFetcherConfig>> for ProviderFetcherConfig {
    fn from(opt: Option<ProviderFetcherConfig>) -> ProviderFetcherConfig {
        opt.unwrap_or_default()
    }
}
