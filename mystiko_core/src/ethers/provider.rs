use async_trait::async_trait;
use mystiko_config::MystikoConfig;
use mystiko_ethers::{ChainProvidersOptions, ProviderOptions, ProviderPool, ProvidersOptions, WS_REGEX};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SignerProviderOptions {
    config: Arc<MystikoConfig>,
}

pub type SignerProviderPool = ProviderPool<SignerProviderOptions>;

#[async_trait]
impl ChainProvidersOptions for SignerProviderOptions {
    async fn providers_options(&self, chain_id: u64) -> anyhow::Result<Option<ProvidersOptions>> {
        if let Some(chain_config) = self.config.find_chain(chain_id) {
            let options = ProviderOptions::builder()
                .url(chain_config.signer_endpoint().to_string())
                .build();
            if WS_REGEX.is_match(chain_config.signer_endpoint()) {
                Ok(Some(ProvidersOptions::Ws(options)))
            } else {
                Ok(Some(ProvidersOptions::Http(options)))
            }
        } else {
            Ok(None)
        }
    }
}

impl From<Arc<MystikoConfig>> for SignerProviderOptions {
    fn from(config: Arc<MystikoConfig>) -> Self {
        Self::builder().config(config).build()
    }
}
