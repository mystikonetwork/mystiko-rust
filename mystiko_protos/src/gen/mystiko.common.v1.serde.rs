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
            Self::Rollup32 => "CIRCUIT_TYPE_ROLLUP_32",
            Self::Rollup64 => "CIRCUIT_TYPE_ROLLUP_64",
            Self::Rollup128 => "CIRCUIT_TYPE_ROLLUP_128",
            Self::Rollup256 => "CIRCUIT_TYPE_ROLLUP_256",
            Self::Rollup512 => "CIRCUIT_TYPE_ROLLUP_512",
            Self::Rollup1024 => "CIRCUIT_TYPE_ROLLUP_1024",
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
            "CIRCUIT_TYPE_ROLLUP_32",
            "CIRCUIT_TYPE_ROLLUP_64",
            "CIRCUIT_TYPE_ROLLUP_128",
            "CIRCUIT_TYPE_ROLLUP_256",
            "CIRCUIT_TYPE_ROLLUP_512",
            "CIRCUIT_TYPE_ROLLUP_1024",
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
                    "CIRCUIT_TYPE_ROLLUP_32" => Ok(CircuitType::Rollup32),
                    "CIRCUIT_TYPE_ROLLUP_64" => Ok(CircuitType::Rollup64),
                    "CIRCUIT_TYPE_ROLLUP_128" => Ok(CircuitType::Rollup128),
                    "CIRCUIT_TYPE_ROLLUP_256" => Ok(CircuitType::Rollup256),
                    "CIRCUIT_TYPE_ROLLUP_512" => Ok(CircuitType::Rollup512),
                    "CIRCUIT_TYPE_ROLLUP_1024" => Ok(CircuitType::Rollup1024),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.file_path.is_some() {
            len += 1;
        }
        if self.is_testnet.is_some() {
            len += 1;
        }
        if self.is_staging.is_some() {
            len += 1;
        }
        if self.remote_base_url.is_some() {
            len += 1;
        }
        if self.git_revision.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.common.v1.ConfigOptions", len)?;
        if let Some(v) = self.file_path.as_ref() {
            struct_ser.serialize_field("filePath", v)?;
        }
        if let Some(v) = self.is_testnet.as_ref() {
            struct_ser.serialize_field("isTestnet", v)?;
        }
        if let Some(v) = self.is_staging.as_ref() {
            struct_ser.serialize_field("isStaging", v)?;
        }
        if let Some(v) = self.remote_base_url.as_ref() {
            struct_ser.serialize_field("remoteBaseUrl", v)?;
        }
        if let Some(v) = self.git_revision.as_ref() {
            struct_ser.serialize_field("gitRevision", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "is_testnet",
            "isTestnet",
            "is_staging",
            "isStaging",
            "remote_base_url",
            "remoteBaseUrl",
            "git_revision",
            "gitRevision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            IsTestnet,
            IsStaging,
            RemoteBaseUrl,
            GitRevision,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "isTestnet" | "is_testnet" => Ok(GeneratedField::IsTestnet),
                            "isStaging" | "is_staging" => Ok(GeneratedField::IsStaging),
                            "remoteBaseUrl" | "remote_base_url" => Ok(GeneratedField::RemoteBaseUrl),
                            "gitRevision" | "git_revision" => Ok(GeneratedField::GitRevision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.common.v1.ConfigOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfigOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut is_testnet__ = None;
                let mut is_staging__ = None;
                let mut remote_base_url__ = None;
                let mut git_revision__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = map.next_value()?;
                        }
                        GeneratedField::IsTestnet => {
                            if is_testnet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isTestnet"));
                            }
                            is_testnet__ = map.next_value()?;
                        }
                        GeneratedField::IsStaging => {
                            if is_staging__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isStaging"));
                            }
                            is_staging__ = map.next_value()?;
                        }
                        GeneratedField::RemoteBaseUrl => {
                            if remote_base_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteBaseUrl"));
                            }
                            remote_base_url__ = map.next_value()?;
                        }
                        GeneratedField::GitRevision => {
                            if git_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gitRevision"));
                            }
                            git_revision__ = map.next_value()?;
                        }
                    }
                }
                Ok(ConfigOptions {
                    file_path: file_path__,
                    is_testnet: is_testnet__,
                    is_staging: is_staging__,
                    remote_base_url: remote_base_url__,
                    git_revision: git_revision__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.common.v1.ConfigOptions", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for TransactionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TRANSACTION_TYPE_UNSPECIFIED",
            Self::Legacy => "TRANSACTION_TYPE_LEGACY",
            Self::Eip1559 => "TRANSACTION_TYPE_EIP1559",
            Self::Eip2930 => "TRANSACTION_TYPE_EIP2930",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TransactionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TRANSACTION_TYPE_UNSPECIFIED",
            "TRANSACTION_TYPE_LEGACY",
            "TRANSACTION_TYPE_EIP1559",
            "TRANSACTION_TYPE_EIP2930",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransactionType;

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
                    .and_then(TransactionType::from_i32)
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
                    .and_then(TransactionType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TRANSACTION_TYPE_UNSPECIFIED" => Ok(TransactionType::Unspecified),
                    "TRANSACTION_TYPE_LEGACY" => Ok(TransactionType::Legacy),
                    "TRANSACTION_TYPE_EIP1559" => Ok(TransactionType::Eip1559),
                    "TRANSACTION_TYPE_EIP2930" => Ok(TransactionType::Eip2930),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
