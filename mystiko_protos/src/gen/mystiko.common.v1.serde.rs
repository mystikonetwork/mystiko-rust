// @generated
impl serde::Serialize for AssetType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ASSET_TYPE_UNSPECIFIED",
            Self::Erc20 => "ASSET_TYPE_ERC20",
            Self::Main => "ASSET_TYPE_MAIN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AssetType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ASSET_TYPE_UNSPECIFIED",
            "ASSET_TYPE_ERC20",
            "ASSET_TYPE_MAIN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(AssetType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(AssetType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ASSET_TYPE_UNSPECIFIED" => Ok(AssetType::Unspecified),
                    "ASSET_TYPE_ERC20" => Ok(AssetType::Erc20),
                    "ASSET_TYPE_MAIN" => Ok(AssetType::Main),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for BridgeType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "BRIDGE_TYPE_UNSPECIFIED",
            Self::Loop => "BRIDGE_TYPE_LOOP",
            Self::Poly => "BRIDGE_TYPE_POLY",
            Self::Tbridge => "BRIDGE_TYPE_TBRIDGE",
            Self::Celer => "BRIDGE_TYPE_CELER",
            Self::LayerZero => "BRIDGE_TYPE_LAYER_ZERO",
            Self::Axelar => "BRIDGE_TYPE_AXELAR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for BridgeType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "BRIDGE_TYPE_UNSPECIFIED",
            "BRIDGE_TYPE_LOOP",
            "BRIDGE_TYPE_POLY",
            "BRIDGE_TYPE_TBRIDGE",
            "BRIDGE_TYPE_CELER",
            "BRIDGE_TYPE_LAYER_ZERO",
            "BRIDGE_TYPE_AXELAR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BridgeType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(BridgeType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(BridgeType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "BRIDGE_TYPE_UNSPECIFIED" => Ok(BridgeType::Unspecified),
                    "BRIDGE_TYPE_LOOP" => Ok(BridgeType::Loop),
                    "BRIDGE_TYPE_POLY" => Ok(BridgeType::Poly),
                    "BRIDGE_TYPE_TBRIDGE" => Ok(BridgeType::Tbridge),
                    "BRIDGE_TYPE_CELER" => Ok(BridgeType::Celer),
                    "BRIDGE_TYPE_LAYER_ZERO" => Ok(BridgeType::LayerZero),
                    "BRIDGE_TYPE_AXELAR" => Ok(BridgeType::Axelar),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CircuitType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CIRCUIT_TYPE_UNSPECIFIED",
            Self::Rollup1 => "CIRCUIT_TYPE_ROLLUP_1",
            Self::Rollup2 => "CIRCUIT_TYPE_ROLLUP_2",
            Self::Rollup4 => "CIRCUIT_TYPE_ROLLUP_4",
            Self::Rollup8 => "CIRCUIT_TYPE_ROLLUP_8",
            Self::Rollup16 => "CIRCUIT_TYPE_ROLLUP_16",
            Self::Transaction1x0 => "CIRCUIT_TYPE_TRANSACTION1X0",
            Self::Transaction1x1 => "CIRCUIT_TYPE_TRANSACTION1X1",
            Self::Transaction1x2 => "CIRCUIT_TYPE_TRANSACTION1X2",
            Self::Transaction2x0 => "CIRCUIT_TYPE_TRANSACTION2X0",
            Self::Transaction2x1 => "CIRCUIT_TYPE_TRANSACTION2X1",
            Self::Transaction2x2 => "CIRCUIT_TYPE_TRANSACTION2X2",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CircuitType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CIRCUIT_TYPE_UNSPECIFIED",
            "CIRCUIT_TYPE_ROLLUP_1",
            "CIRCUIT_TYPE_ROLLUP_2",
            "CIRCUIT_TYPE_ROLLUP_4",
            "CIRCUIT_TYPE_ROLLUP_8",
            "CIRCUIT_TYPE_ROLLUP_16",
            "CIRCUIT_TYPE_TRANSACTION1X0",
            "CIRCUIT_TYPE_TRANSACTION1X1",
            "CIRCUIT_TYPE_TRANSACTION1X2",
            "CIRCUIT_TYPE_TRANSACTION2X0",
            "CIRCUIT_TYPE_TRANSACTION2X1",
            "CIRCUIT_TYPE_TRANSACTION2X2",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CircuitType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(CircuitType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(CircuitType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CIRCUIT_TYPE_UNSPECIFIED" => Ok(CircuitType::Unspecified),
                    "CIRCUIT_TYPE_ROLLUP_1" => Ok(CircuitType::Rollup1),
                    "CIRCUIT_TYPE_ROLLUP_2" => Ok(CircuitType::Rollup2),
                    "CIRCUIT_TYPE_ROLLUP_4" => Ok(CircuitType::Rollup4),
                    "CIRCUIT_TYPE_ROLLUP_8" => Ok(CircuitType::Rollup8),
                    "CIRCUIT_TYPE_ROLLUP_16" => Ok(CircuitType::Rollup16),
                    "CIRCUIT_TYPE_TRANSACTION1X0" => Ok(CircuitType::Transaction1x0),
                    "CIRCUIT_TYPE_TRANSACTION1X1" => Ok(CircuitType::Transaction1x1),
                    "CIRCUIT_TYPE_TRANSACTION1X2" => Ok(CircuitType::Transaction1x2),
                    "CIRCUIT_TYPE_TRANSACTION2X0" => Ok(CircuitType::Transaction2x0),
                    "CIRCUIT_TYPE_TRANSACTION2X1" => Ok(CircuitType::Transaction2x1),
                    "CIRCUIT_TYPE_TRANSACTION2X2" => Ok(CircuitType::Transaction2x2),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ContractType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CONTRACT_TYPE_UNSPECIFIED",
            Self::Deposit => "CONTRACT_TYPE_DEPOSIT",
            Self::Pool => "CONTRACT_TYPE_POOL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ContractType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONTRACT_TYPE_UNSPECIFIED",
            "CONTRACT_TYPE_DEPOSIT",
            "CONTRACT_TYPE_POOL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ContractType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ContractType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CONTRACT_TYPE_UNSPECIFIED" => Ok(ContractType::Unspecified),
                    "CONTRACT_TYPE_DEPOSIT" => Ok(ContractType::Deposit),
                    "CONTRACT_TYPE_POOL" => Ok(ContractType::Pool),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ProviderType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PROVIDER_TYPE_UNSPECIFIED",
            Self::Failover => "PROVIDER_TYPE_FAILOVER",
            Self::Quorum => "PROVIDER_TYPE_QUORUM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ProviderType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROVIDER_TYPE_UNSPECIFIED",
            "PROVIDER_TYPE_FAILOVER",
            "PROVIDER_TYPE_QUORUM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ProviderType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ProviderType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PROVIDER_TYPE_UNSPECIFIED" => Ok(ProviderType::Unspecified),
                    "PROVIDER_TYPE_FAILOVER" => Ok(ProviderType::Failover),
                    "PROVIDER_TYPE_QUORUM" => Ok(ProviderType::Quorum),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
