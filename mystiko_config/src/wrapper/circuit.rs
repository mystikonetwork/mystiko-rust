use crate::common::CircuitType;
use crate::raw::circuit::RawCircuitConfig;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct CircuitConfig {
    pub(crate) base: BaseConfig<RawCircuitConfig>,
}

impl CircuitConfig {
    pub fn new(data: RawCircuitConfig) -> Self {
        Self {
            base: BaseConfig::new(data, None)
        }
    }

    pub fn name(&self) -> &String {
        return &self.base.data.name;
    }

    pub fn circuit_type(&self) -> &CircuitType {
        return &self.base.data.circuit_type;
    }

    pub fn is_default(&self) -> bool {
        self.base.data.is_default
    }

    pub fn program_file(&self) -> Vec<String> {
        self.base.data.program_file.clone()
    }

    pub fn abi_file(&self) -> Vec<String> {
        self.base.data.abi_file.clone()
    }

    pub fn proving_key_file(&self) -> Vec<String> {
        self.base.data.proving_key_file.clone()
    }

    pub fn verifying_key_file(&self) -> Vec<String> {
        self.base.data.verifying_key_file.clone()
    }

    pub fn mutate(&self, data: Option<RawCircuitConfig>) -> Self {
        let data = match data {
            None => { self.base.data.clone() }
            Some(value) => { value }
        };
        CircuitConfig::new(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::RawConfig;
    use crate::raw::circuit::RawCircuitConfig;
    use crate::wrapper::circuit::CircuitConfig;

    async fn default_config() -> (RawCircuitConfig, CircuitConfig) {
        let raw_config =
            RawConfig::create_from_file::<RawCircuitConfig>("src/tests/files/circuit.valid.json").await;
        let config = CircuitConfig::new(raw_config.clone());
        (raw_config, config)
    }

    #[tokio::test]
    async fn test_equality() {
        let (raw_config, config) = default_config().await;
        assert_eq!(config.name(), &raw_config.name);
        assert_eq!(config.circuit_type(), &raw_config.circuit_type);
        assert_eq!(config.is_default(), raw_config.is_default);
        assert_eq!(config.program_file(), raw_config.program_file);
        assert_eq!(config.abi_file(), raw_config.abi_file);
        assert_eq!(config.proving_key_file(), raw_config.proving_key_file);
        assert_eq!(config.verifying_key_file(), raw_config.verifying_key_file);
    }

    #[tokio::test]
    async fn test_copy() {
        let (_, config) = default_config().await;
        assert_eq!(
            CircuitConfig::new(config.base.copy_data()),
            config
        );
    }

    #[tokio::test]
    async fn test_mutate() {
        let (mut raw_config, config) = default_config().await;
        assert_eq!(config.mutate(None), config);
        raw_config.name = "another name".to_string();
        let new_config = config.mutate(Some(raw_config));
        assert_eq!(new_config.name(), "another name");
    }

    #[tokio::test]
    async fn test_to_json_string() {
        let (raw_config, config) = default_config().await;
        let json_string = config.base.to_json_string();
        let loaded_raw_config =
            RawConfig::create_from_json_string::<RawCircuitConfig>(json_string.as_str()).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}
