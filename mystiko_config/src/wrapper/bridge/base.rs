use serde::Serialize;
use crate::common::BridgeType;
use crate::raw::bridge::base::RawBridgeConfigTrait;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeConfig<T, A = ()>
    where
        T: RawBridgeConfigTrait + Serialize + Clone,
        A: Clone,
{
    pub(crate) base: BaseConfig<T, A>,
}

impl<T, A> BridgeConfig<T, A> where
    T: RawBridgeConfigTrait + Serialize + Clone,
    A: Clone
{
    pub fn new(data: T, aux_data: Option<A>) -> Self {
        Self { base: BaseConfig::new(data, aux_data) }
    }

    pub fn name(&self) -> &String {
        &self.base.data.name()
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.base.data.bridge_type()
    }

    pub fn mutate(&self, data: Option<T>, aux_data: Option<A>) -> BridgeConfig<T, A> {
        let data = match data {
            None => { self.base.data.clone() }
            Some(value) => { value }
        };
        let aux_data = match aux_data {
            None => {
                self.base.aux_data.clone()
            }
            Some(value) => { Some(value) }
        };
        BridgeConfig::<T, A>::new(data, aux_data)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::RawConfig;
    use crate::raw::bridge::base::RawBridgeConfig;
    use crate::wrapper::bridge::base::BridgeConfig;

    async fn default_config() -> (RawBridgeConfig, BridgeConfig<RawBridgeConfig>) {
        let raw_config =
            RawConfig::create_from_file::<RawBridgeConfig>("src/tests/files/bridge/base.valid.json").await;
        let config = BridgeConfig::new(raw_config.clone(), None);
        (raw_config, config)
    }

    #[tokio::test]
    async fn test_equality() {
        let (raw_config, config) = default_config().await;
        assert_eq!(config.name(), &raw_config.name);
        assert_eq!(config.bridge_type(), &raw_config.bridge_type);
    }

    #[tokio::test]
    async fn test_copy() {
        let (raw_config, config) = default_config().await;
        let copy = BridgeConfig::new(config.base.copy_data(), None);
        assert_eq!(copy, config);
    }

    #[tokio::test]
    async fn test_mutate() {
        let (mut raw_config, config) = default_config().await;
        let mutate_config = config.mutate(None, None);
        assert_eq!(config, mutate_config);

        raw_config.name = "another name".to_string();
        let new_config = config.mutate(Some(raw_config), None);
        assert_eq!(new_config.name(), &"another name".to_string());
    }

    #[tokio::test]
    async fn test_to_json_string() {
        let (mut raw_config, config) = default_config().await;
        let json_string = config.base.to_json_string();
        let loaded_raw_config =
            RawConfig::create_from_json_string::<RawBridgeConfig>(&json_string).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}
