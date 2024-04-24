use crate::common::v1::BridgeType;
pub use crate::gen::mystiko::config::bridge::v1::*;

impl TryFrom<&mystiko_config::AxelarBridgeConfig> for BridgeConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::AxelarBridgeConfig) -> anyhow::Result<Self> {
        let bridge_type: BridgeType = config.bridge_type().into();
        Ok(BridgeConfig::builder()
            .name(config.name().to_string())
            .bridge_type(bridge_type as i32)
            .build())
    }
}

impl TryFrom<&mystiko_config::CelerBridgeConfig> for BridgeConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::CelerBridgeConfig) -> anyhow::Result<Self> {
        let bridge_type: BridgeType = config.bridge_type().into();
        Ok(BridgeConfig::builder()
            .name(config.name().to_string())
            .bridge_type(bridge_type as i32)
            .build())
    }
}

impl TryFrom<&mystiko_config::LayerZeroBridgeConfig> for BridgeConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::LayerZeroBridgeConfig) -> anyhow::Result<Self> {
        let bridge_type: BridgeType = config.bridge_type().into();
        Ok(BridgeConfig::builder()
            .name(config.name().to_string())
            .bridge_type(bridge_type as i32)
            .build())
    }
}

impl TryFrom<&mystiko_config::PolyBridgeConfig> for BridgeConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::PolyBridgeConfig) -> anyhow::Result<Self> {
        let bridge_type: BridgeType = config.bridge_type().into();
        Ok(BridgeConfig::builder()
            .name(config.name().to_string())
            .bridge_type(bridge_type as i32)
            .explorer_url(Some(config.explorer_url().to_string()))
            .explorer_prefix(Some(config.explorer_prefix().to_string()))
            .api_url(Some(config.api_url().to_string()))
            .api_prefix(Some(config.api_prefix().to_string()))
            .build())
    }
}

impl TryFrom<&mystiko_config::TBridgeConfig> for BridgeConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::TBridgeConfig) -> anyhow::Result<Self> {
        let bridge_type: BridgeType = config.bridge_type().into();
        Ok(BridgeConfig::builder()
            .name(config.name().to_string())
            .bridge_type(bridge_type as i32)
            .build())
    }
}

impl TryFrom<&mystiko_config::BridgeConfig> for BridgeConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::BridgeConfig) -> anyhow::Result<Self> {
        match config {
            mystiko_config::BridgeConfig::Axelar(config) => config.try_into(),
            mystiko_config::BridgeConfig::Celer(config) => config.try_into(),
            mystiko_config::BridgeConfig::LayerZero(config) => config.try_into(),
            mystiko_config::BridgeConfig::Poly(config) => config.try_into(),
            mystiko_config::BridgeConfig::TBridge(config) => config.try_into(),
        }
    }
}
