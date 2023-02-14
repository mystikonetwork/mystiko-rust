use std::hash::{Hash, Hasher};
use std::os::macos::raw::stat;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{CircuitType, validate_object};
use crate::raw::base::{RawConfig, RawConfigTrait};
use crate::raw::validator::{string_vec_each_not_empty};

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawCircuitConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(length(min = 1))]
    pub name: String,

    #[serde(rename = "type")]
    pub circuit_type: CircuitType,

    pub is_default: bool,

    #[validate(custom = "string_vec_each_not_empty")]
    pub program_file: Vec<String>,

    #[validate(custom = "string_vec_each_not_empty")]
    pub abi_file: Vec<String>,

    #[validate(custom = "string_vec_each_not_empty")]
    pub proving_key_file: Vec<String>,

    #[validate(custom = "string_vec_each_not_empty")]
    pub verifying_key_file: Vec<String>,
}

impl RawCircuitConfig {
    pub fn new(
        name: String,
        circuit_type: CircuitType,
        is_default: bool,
        program_file: Vec<String>,
        abi_file: Vec<String>,
        proving_key_file: Vec<String>,
        verifying_key_file: Vec<String>,
    ) -> Self {
        Self {
            base: RawConfig::default(),
            name,
            circuit_type,
            is_default,
            program_file,
            abi_file,
            proving_key_file,
            verifying_key_file,
        }
    }
}

impl RawConfigTrait for RawCircuitConfig {
    fn validate(&self) {
        self.base.validate_object(self)
    }
}

impl Hash for RawCircuitConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use serde::de::Unexpected::Str;
    use crate::common::CircuitType;
    use crate::raw::base::{RawConfig, RawConfigTrait};
    use crate::raw::circuit::RawCircuitConfig;

    async fn default_config() -> RawCircuitConfig {
        RawConfig::create_from_object::<RawCircuitConfig>(
            RawCircuitConfig::new(
                "zokrates-1.0-rollup1".to_string(),
                CircuitType::Rollup1,
                true,
                vec![String::from("./Rollup1.program.gz")],
                vec![String::from("./Rollup1.abi.json")],
                vec![String::from("./Rollup1.pkey.gz")],
                vec![String::from("./Rollup1.vkey.gz")],
            )
        ).await
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_name() {
        let mut config = default_config().await;
        config.name = String::from("");
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_program_file() {
        let mut config = default_config().await;
        config.program_file = vec![String::from("")];
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_abi_file() {
        let mut config = default_config().await;
        config.abi_file = vec![String::from("")];
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_proving_key_file() {
        let mut config = default_config().await;
        config.proving_key_file = vec![String::from("")];
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_verifying_key_file() {
        let mut config = default_config().await;
        config.verifying_key_file = vec![String::from("")];
        config.validate();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawCircuitConfig>("src/tests/files/circuit.valid.json").await;
        assert_eq!(file_config, default_config().await);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawCircuitConfig>("src/tests/files/circuit.invalid.json").await;
    }
}