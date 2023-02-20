use crate::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct LayerZeroBridgeConfig {
    base: BridgeConfig<RawLayerZeroBridgeConfig>,
}

impl LayerZeroBridgeConfig {
    pub fn new(data: RawLayerZeroBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
    }

    pub fn mutate(&self, data: Option<RawLayerZeroBridgeConfig>) -> Self {
        match data {
            Some(value) => LayerZeroBridgeConfig::new(value),
            None => LayerZeroBridgeConfig::new(self.base.base.data.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::RawConfig;
    use crate::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
    use crate::wrapper::bridge::layer_zero::LayerZeroBridgeConfig;

    async fn default_config() -> (RawLayerZeroBridgeConfig, LayerZeroBridgeConfig) {
        let raw_config =
            RawConfig::create_from_file::<RawLayerZeroBridgeConfig>("src/tests/files/bridge/layerZero.valid.json").await;
        let config = LayerZeroBridgeConfig::new(raw_config.clone());
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
        let copy = LayerZeroBridgeConfig::new(config.base.base.copy_data());
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
            RawConfig::create_from_json_string::<RawLayerZeroBridgeConfig>(&json_string).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}