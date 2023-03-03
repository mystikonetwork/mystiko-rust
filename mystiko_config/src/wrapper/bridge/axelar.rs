use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct AxelarBridgeConfig {
    pub(crate) base: BridgeConfig<RawAxelarBridgeConfig>,
}

impl AxelarBridgeConfig {
    pub fn new(data: RawAxelarBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
    }

    pub fn mutate(&self, data: Option<RawAxelarBridgeConfig>) -> Self {
        match data {
            Some(value) => AxelarBridgeConfig::new(value),
            None => AxelarBridgeConfig::new(self.base.base.data.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::RawConfig;
    use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
    use crate::wrapper::bridge::axelar::AxelarBridgeConfig;

    async fn default_config() -> (RawAxelarBridgeConfig, AxelarBridgeConfig) {
        let raw_config =
            RawConfig::create_from_file::<RawAxelarBridgeConfig>("src/tests/files/bridge/axelar.valid.json").await;
        let config = AxelarBridgeConfig::new(raw_config.clone());
        (raw_config, config)
    }

    #[tokio::test]
    async fn test_equality() {
        let (raw_config, config) = default_config().await;
        assert_eq!(&raw_config.base.name, config.base.name());
        assert_eq!(&raw_config.bridge_type, config.base.bridge_type());
    }

    #[tokio::test]
    async fn test_copy() {
        let (_raw_config, config) = default_config().await;
        let copy = AxelarBridgeConfig::new(config.base.base.copy_data());
        assert_eq!(copy, config);
    }

    #[tokio::test]
    async fn test_mutate() {
        let (mut raw_config, config) = default_config().await;
        let mutate_config = config.mutate(None);
        assert_eq!(config, mutate_config);

        raw_config.base.name = "another name".to_string();
        let new_config = config.mutate(Some(raw_config));
        assert_eq!(new_config.base.name(), &"another name".to_string());
    }

    #[tokio::test]
    async fn test_to_json_string() {
        let (raw_config, config) = default_config().await;
        let json_string = config.base.base.to_json_string();
        let loaded_raw_config =
            RawConfig::create_from_json_string::<RawAxelarBridgeConfig>(&json_string).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}