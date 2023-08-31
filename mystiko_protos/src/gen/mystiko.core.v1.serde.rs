// @generated
impl serde::Serialize for AccountStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ACCOUNT_STATUS_UNSPECIFIED",
            Self::Created => "ACCOUNT_STATUS_CREATED",
            Self::Scanning => "ACCOUNT_STATUS_SCANNING",
            Self::Scanned => "ACCOUNT_STATUS_SCANNED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AccountStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACCOUNT_STATUS_UNSPECIFIED",
            "ACCOUNT_STATUS_CREATED",
            "ACCOUNT_STATUS_SCANNING",
            "ACCOUNT_STATUS_SCANNED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccountStatus;

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
                    .and_then(AccountStatus::from_i32)
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
                    .and_then(AccountStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ACCOUNT_STATUS_UNSPECIFIED" => Ok(AccountStatus::Unspecified),
                    "ACCOUNT_STATUS_CREATED" => Ok(AccountStatus::Created),
                    "ACCOUNT_STATUS_SCANNING" => Ok(AccountStatus::Scanning),
                    "ACCOUNT_STATUS_SCANNED" => Ok(AccountStatus::Scanned),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
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
impl serde::Serialize for CommitmentStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "COMMITMENT_STATUS_UNSPECIFIED",
            Self::Init => "COMMITMENT_STATUS_INIT",
            Self::SrcSucceeded => "COMMITMENT_STATUS_SRC_SUCCEEDED",
            Self::Queued => "COMMITMENT_STATUS_QUEUED",
            Self::Included => "COMMITMENT_STATUS_INCLUDED",
            Self::Spent => "COMMITMENT_STATUS_SPENT",
            Self::Failed => "COMMITMENT_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CommitmentStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COMMITMENT_STATUS_UNSPECIFIED",
            "COMMITMENT_STATUS_INIT",
            "COMMITMENT_STATUS_SRC_SUCCEEDED",
            "COMMITMENT_STATUS_QUEUED",
            "COMMITMENT_STATUS_INCLUDED",
            "COMMITMENT_STATUS_SPENT",
            "COMMITMENT_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommitmentStatus;

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
                    .and_then(CommitmentStatus::from_i32)
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
                    .and_then(CommitmentStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "COMMITMENT_STATUS_UNSPECIFIED" => Ok(CommitmentStatus::Unspecified),
                    "COMMITMENT_STATUS_INIT" => Ok(CommitmentStatus::Init),
                    "COMMITMENT_STATUS_SRC_SUCCEEDED" => Ok(CommitmentStatus::SrcSucceeded),
                    "COMMITMENT_STATUS_QUEUED" => Ok(CommitmentStatus::Queued),
                    "COMMITMENT_STATUS_INCLUDED" => Ok(CommitmentStatus::Included),
                    "COMMITMENT_STATUS_SPENT" => Ok(CommitmentStatus::Spent),
                    "COMMITMENT_STATUS_FAILED" => Ok(CommitmentStatus::Failed),
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
impl serde::Serialize for DepositStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DEPOSIT_STATUS_UNSPECIFIED",
            Self::Init => "DEPOSIT_STATUS_INIT",
            Self::AssetApproving => "DEPOSIT_STATUS_ASSET_APPROVING",
            Self::AssetApproved => "DEPOSIT_STATUS_ASSET_APPROVED",
            Self::SrcPending => "DEPOSIT_STATUS_SRC_PENDING",
            Self::SrcSucceeded => "DEPOSIT_STATUS_SRC_SUCCEEDED",
            Self::Queued => "DEPOSIT_STATUS_QUEUED",
            Self::Included => "DEPOSIT_STATUS_INCLUDED",
            Self::Failed => "DEPOSIT_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for DepositStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEPOSIT_STATUS_UNSPECIFIED",
            "DEPOSIT_STATUS_INIT",
            "DEPOSIT_STATUS_ASSET_APPROVING",
            "DEPOSIT_STATUS_ASSET_APPROVED",
            "DEPOSIT_STATUS_SRC_PENDING",
            "DEPOSIT_STATUS_SRC_SUCCEEDED",
            "DEPOSIT_STATUS_QUEUED",
            "DEPOSIT_STATUS_INCLUDED",
            "DEPOSIT_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DepositStatus;

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
                    .and_then(DepositStatus::from_i32)
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
                    .and_then(DepositStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEPOSIT_STATUS_UNSPECIFIED" => Ok(DepositStatus::Unspecified),
                    "DEPOSIT_STATUS_INIT" => Ok(DepositStatus::Init),
                    "DEPOSIT_STATUS_ASSET_APPROVING" => Ok(DepositStatus::AssetApproving),
                    "DEPOSIT_STATUS_ASSET_APPROVED" => Ok(DepositStatus::AssetApproved),
                    "DEPOSIT_STATUS_SRC_PENDING" => Ok(DepositStatus::SrcPending),
                    "DEPOSIT_STATUS_SRC_SUCCEEDED" => Ok(DepositStatus::SrcSucceeded),
                    "DEPOSIT_STATUS_QUEUED" => Ok(DepositStatus::Queued),
                    "DEPOSIT_STATUS_INCLUDED" => Ok(DepositStatus::Included),
                    "DEPOSIT_STATUS_FAILED" => Ok(DepositStatus::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MystikoOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_testnet {
            len += 1;
        }
        if self.is_staging {
            len += 1;
        }
        if self.db_path.is_some() {
            len += 1;
        }
        if self.config_file_path.is_some() {
            len += 1;
        }
        if self.config_remote_base_url.is_some() {
            len += 1;
        }
        if self.config_git_revision.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.v1.MystikoOptions", len)?;
        if self.is_testnet {
            struct_ser.serialize_field("isTestnet", &self.is_testnet)?;
        }
        if self.is_staging {
            struct_ser.serialize_field("isStaging", &self.is_staging)?;
        }
        if let Some(v) = self.db_path.as_ref() {
            struct_ser.serialize_field("dbPath", v)?;
        }
        if let Some(v) = self.config_file_path.as_ref() {
            struct_ser.serialize_field("configFilePath", v)?;
        }
        if let Some(v) = self.config_remote_base_url.as_ref() {
            struct_ser.serialize_field("configRemoteBaseUrl", v)?;
        }
        if let Some(v) = self.config_git_revision.as_ref() {
            struct_ser.serialize_field("configGitRevision", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MystikoOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_testnet",
            "isTestnet",
            "is_staging",
            "isStaging",
            "db_path",
            "dbPath",
            "config_file_path",
            "configFilePath",
            "config_remote_base_url",
            "configRemoteBaseUrl",
            "config_git_revision",
            "configGitRevision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsTestnet,
            IsStaging,
            DbPath,
            ConfigFilePath,
            ConfigRemoteBaseUrl,
            ConfigGitRevision,
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
                            "isTestnet" | "is_testnet" => Ok(GeneratedField::IsTestnet),
                            "isStaging" | "is_staging" => Ok(GeneratedField::IsStaging),
                            "dbPath" | "db_path" => Ok(GeneratedField::DbPath),
                            "configFilePath" | "config_file_path" => Ok(GeneratedField::ConfigFilePath),
                            "configRemoteBaseUrl" | "config_remote_base_url" => Ok(GeneratedField::ConfigRemoteBaseUrl),
                            "configGitRevision" | "config_git_revision" => Ok(GeneratedField::ConfigGitRevision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MystikoOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.v1.MystikoOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MystikoOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_testnet__ = None;
                let mut is_staging__ = None;
                let mut db_path__ = None;
                let mut config_file_path__ = None;
                let mut config_remote_base_url__ = None;
                let mut config_git_revision__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IsTestnet => {
                            if is_testnet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isTestnet"));
                            }
                            is_testnet__ = Some(map.next_value()?);
                        }
                        GeneratedField::IsStaging => {
                            if is_staging__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isStaging"));
                            }
                            is_staging__ = Some(map.next_value()?);
                        }
                        GeneratedField::DbPath => {
                            if db_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbPath"));
                            }
                            db_path__ = map.next_value()?;
                        }
                        GeneratedField::ConfigFilePath => {
                            if config_file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configFilePath"));
                            }
                            config_file_path__ = map.next_value()?;
                        }
                        GeneratedField::ConfigRemoteBaseUrl => {
                            if config_remote_base_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configRemoteBaseUrl"));
                            }
                            config_remote_base_url__ = map.next_value()?;
                        }
                        GeneratedField::ConfigGitRevision => {
                            if config_git_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configGitRevision"));
                            }
                            config_git_revision__ = map.next_value()?;
                        }
                    }
                }
                Ok(MystikoOptions {
                    is_testnet: is_testnet__.unwrap_or_default(),
                    is_staging: is_staging__.unwrap_or_default(),
                    db_path: db_path__,
                    config_file_path: config_file_path__,
                    config_remote_base_url: config_remote_base_url__,
                    config_git_revision: config_git_revision__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.v1.MystikoOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "NETWORK_TYPE_UNSPECIFIED",
            Self::Testnet => "NETWORK_TYPE_TESTNET",
            Self::Mainnet => "NETWORK_TYPE_MAINNET",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for NetworkType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NETWORK_TYPE_UNSPECIFIED",
            "NETWORK_TYPE_TESTNET",
            "NETWORK_TYPE_MAINNET",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetworkType;

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
                    .and_then(NetworkType::from_i32)
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
                    .and_then(NetworkType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NETWORK_TYPE_UNSPECIFIED" => Ok(NetworkType::Unspecified),
                    "NETWORK_TYPE_TESTNET" => Ok(NetworkType::Testnet),
                    "NETWORK_TYPE_MAINNET" => Ok(NetworkType::Mainnet),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PackerChecksum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PACKER_CHECKSUM_UNSPECIFIED",
            Self::Sha512 => "PACKER_CHECKSUM_SHA512",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PackerChecksum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PACKER_CHECKSUM_UNSPECIFIED",
            "PACKER_CHECKSUM_SHA512",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PackerChecksum;

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
                    .and_then(PackerChecksum::from_i32)
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
                    .and_then(PackerChecksum::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PACKER_CHECKSUM_UNSPECIFIED" => Ok(PackerChecksum::Unspecified),
                    "PACKER_CHECKSUM_SHA512" => Ok(PackerChecksum::Sha512),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PackerCompression {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PACKER_COMPRESSION_UNSPECIFIED",
            Self::Zstd => "PACKER_COMPRESSION_ZSTD",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PackerCompression {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PACKER_COMPRESSION_UNSPECIFIED",
            "PACKER_COMPRESSION_ZSTD",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PackerCompression;

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
                    .and_then(PackerCompression::from_i32)
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
                    .and_then(PackerCompression::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PACKER_COMPRESSION_UNSPECIFIED" => Ok(PackerCompression::Unspecified),
                    "PACKER_COMPRESSION_ZSTD" => Ok(PackerCompression::Zstd),
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
impl serde::Serialize for TransactionStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TRANSACTION_STATUS_UNSPECIFIED",
            Self::Init => "TRANSACTION_STATUS_INIT",
            Self::ProofGenerating => "TRANSACTION_STATUS_PROOF_GENERATING",
            Self::ProofGenerated => "TRANSACTION_STATUS_PROOF_GENERATED",
            Self::Pending => "TRANSACTION_STATUS_PENDING",
            Self::Succeeded => "TRANSACTION_STATUS_SUCCEEDED",
            Self::Failed => "TRANSACTION_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TransactionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TRANSACTION_STATUS_UNSPECIFIED",
            "TRANSACTION_STATUS_INIT",
            "TRANSACTION_STATUS_PROOF_GENERATING",
            "TRANSACTION_STATUS_PROOF_GENERATED",
            "TRANSACTION_STATUS_PENDING",
            "TRANSACTION_STATUS_SUCCEEDED",
            "TRANSACTION_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransactionStatus;

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
                    .and_then(TransactionStatus::from_i32)
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
                    .and_then(TransactionStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TRANSACTION_STATUS_UNSPECIFIED" => Ok(TransactionStatus::Unspecified),
                    "TRANSACTION_STATUS_INIT" => Ok(TransactionStatus::Init),
                    "TRANSACTION_STATUS_PROOF_GENERATING" => Ok(TransactionStatus::ProofGenerating),
                    "TRANSACTION_STATUS_PROOF_GENERATED" => Ok(TransactionStatus::ProofGenerated),
                    "TRANSACTION_STATUS_PENDING" => Ok(TransactionStatus::Pending),
                    "TRANSACTION_STATUS_SUCCEEDED" => Ok(TransactionStatus::Succeeded),
                    "TRANSACTION_STATUS_FAILED" => Ok(TransactionStatus::Failed),
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
            Self::Transfer => "TRANSACTION_TYPE_TRANSFER",
            Self::Withdraw => "TRANSACTION_TYPE_WITHDRAW",
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
            "TRANSACTION_TYPE_TRANSFER",
            "TRANSACTION_TYPE_WITHDRAW",
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
                    "TRANSACTION_TYPE_TRANSFER" => Ok(TransactionType::Transfer),
                    "TRANSACTION_TYPE_WITHDRAW" => Ok(TransactionType::Withdraw),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
