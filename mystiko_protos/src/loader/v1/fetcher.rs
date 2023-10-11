use crate::loader::v1::{
    EtherscanFetcherConfig, FetcherConfig, ProviderFetcherChainConfig, ProviderFetcherConfig, SequencerFetcherConfig,
};

impl From<Option<FetcherConfig>> for FetcherConfig {
    fn from(opt: Option<FetcherConfig>) -> FetcherConfig {
        opt.unwrap_or_default()
    }
}

impl From<Option<SequencerFetcherConfig>> for SequencerFetcherConfig {
    fn from(opt: Option<SequencerFetcherConfig>) -> SequencerFetcherConfig {
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

impl ProviderFetcherChainConfig {
    pub fn urls(&self) -> Vec<String> {
        let mut entries: Vec<_> = self.urls.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries.into_iter().map(|(_, v)| v.clone()).collect()
    }
}
