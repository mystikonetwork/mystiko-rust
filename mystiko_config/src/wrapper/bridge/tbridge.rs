use crate::raw::bridge::tbridge::RawTBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct TBridgeConfig {
    pub(crate) base: BridgeConfig<RawTBridgeConfig>,
}

impl TBridgeConfig {
    pub fn new(data: RawTBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
    }

    pub fn mutate(&self, data: Option<RawTBridgeConfig>) -> Self {
        match data {
            Some(value) => TBridgeConfig::new(value),
            None => TBridgeConfig::new(self.base.base.data.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::RawConfig;
    use crate::raw::bridge::tbridge::RawTBridgeConfig;
    use crate::wrapper::bridge::tbridge::TBridgeConfig;

    async fn default_config() -> (RawTBridgeConfig, TBridgeConfig) {
        let raw_config =
            RawConfig::create_from_file::<RawTBridgeConfig>("src/tests/files/bridge/tbridge.valid.json").await;
        let config = TBridgeConfig::new(raw_config.clone());
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
        let copy = TBridgeConfig::new(config.base.base.copy_data());
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
            RawConfig::create_from_json_string::<RawTBridgeConfig>(&json_string).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}