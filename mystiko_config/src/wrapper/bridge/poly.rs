use crate::raw::bridge::poly::RawPolyBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct PolyBridgeConfig {
    base: BridgeConfig<RawPolyBridgeConfig>,
}

impl PolyBridgeConfig {
    pub fn new(data: RawPolyBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
    }

    pub fn explorer_url(&self) -> &String {
        &self.base.base.data.explorer_url
    }

    pub fn explorer_prefix(&self) -> &String {
        &self.base.base.data.explorer_prefix
    }

    pub fn api_url(&self) -> &String {
        &self.base.base.data.api_url
    }

    pub fn api_prefix(&self) -> &String {
        &self.base.base.data.api_prefix
    }

    pub fn mutate(&self, data: Option<RawPolyBridgeConfig>) -> Self {
        match data {
            None => {
                PolyBridgeConfig::new(self.base.base.data.clone())
            }
            Some(value) => {
                PolyBridgeConfig::new(value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::RawConfig;
    use crate::raw::bridge::poly::RawPolyBridgeConfig;
    use crate::wrapper::bridge::poly::PolyBridgeConfig;

    async fn default_config() -> (RawPolyBridgeConfig, PolyBridgeConfig) {
        let raw_config =
            RawConfig::create_from_file::<RawPolyBridgeConfig>("src/tests/files/bridge/poly.valid.json").await;
        let config = PolyBridgeConfig::new(raw_config.clone());
        (raw_config, config)
    }

    #[tokio::test]
    async fn test_equality() {
        let (raw_config, config) = default_config().await;
        assert_eq!(&raw_config.base.name, config.base.name());
        assert_eq!(&raw_config.bridge_type, config.base.bridge_type());
        assert_eq!(&raw_config.explorer_url, config.explorer_url());
        assert_eq!(&raw_config.explorer_prefix, config.explorer_prefix());
        assert_eq!(&raw_config.api_url, config.api_url());
        assert_eq!(&raw_config.api_prefix, config.api_prefix());
    }

    #[tokio::test]
    async fn test_copy() {
        let (_raw_config, config) = default_config().await;
        let copy = PolyBridgeConfig::new(config.base.base.copy_data());
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
            RawConfig::create_from_json_string::<RawPolyBridgeConfig>(&json_string).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}