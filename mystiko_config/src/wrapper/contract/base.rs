use serde::Serialize;
use crate::common::ContractType;
use crate::raw::contract::base::RawContractConfigTrait;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct ContractConfig<T, A = ()>
    where
        T: RawContractConfigTrait + Serialize + Clone,
        A: Clone,
{
    pub(crate) base: BaseConfig<T, A>,
}

impl<T, A> ContractConfig<T, A> where
    T: RawContractConfigTrait + Serialize + Clone,
    A: Clone
{
    pub fn new(data: T, aux_data: Option<A>) -> Self {
        Self { base: BaseConfig::new(data, aux_data) }
    }

    pub fn version(&self) -> &u32 {
        &self.base.data.version()
    }

    pub fn name(&self) -> &str {
        &self.base.data.name()
    }

    pub fn address(&self) -> &str {
        &self.base.data.address()
    }

    pub fn contract_type(&self) -> &ContractType {
        &self.base.data.contract_type()
    }

    pub fn start_block(&self) -> &u32 {
        &self.base.data.start_block()
    }

    pub fn event_filter_size(&self) -> &Option<u32> {
        &self.base.data.event_filter_size()
    }

    pub fn indexer_filter_size(&self) -> &Option<u32> {
        &self.base.data.indexer_filter_size()
    }

    pub fn mutate(&self, data: Option<T>, aux_data: Option<A>) -> Self {
        let d = match data {
            None => {
                self.base.data.clone()
            }
            Some(value) => {
                value
            }
        };
        let a = match aux_data {
            None => {
                self.base.aux_data.clone()
            }
            Some(_) => {
                aux_data
            }
        };

        ContractConfig::new(d, a)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::RawConfig;
    use crate::raw::contract::base::RawContractConfig;
    use crate::wrapper::contract::base::ContractConfig;

    async fn default_config() -> (RawContractConfig, ContractConfig<RawContractConfig>) {
        let raw_config =
            RawConfig::create_from_file::<RawContractConfig>("src/tests/files/contract/base.valid.json").await;
        let config = ContractConfig::new(raw_config.clone(), None);
        (raw_config, config)
    }

    #[tokio::test]
    async fn test_equality() {
        let (raw_config, config) = default_config().await;
        assert_eq!(config.version(), &raw_config.version);
        assert_eq!(config.address(), &raw_config.address);
        assert_eq!(config.name(), &raw_config.name);
        assert_eq!(config.contract_type(), &raw_config.contract_type);
        assert_eq!(config.start_block(), &raw_config.start_block);
        assert_eq!(config.event_filter_size(), &raw_config.event_filter_size);
        assert_eq!(config.indexer_filter_size(), &raw_config.indexer_filter_size);
    }

    #[tokio::test]
    async fn test_copy() {
        let (raw_config, config) = default_config().await;
        let copy =
            ContractConfig::new(config.base.copy_data(), None);
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

    async fn test_to_json_string() {
        let (mut raw_config, config) = default_config().await;
        let json_string = config.base.to_json_string();
        let loaded_raw_config =
            RawConfig::create_from_json_string::<RawContractConfig>(&json_string).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}