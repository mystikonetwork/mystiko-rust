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
        if self.config_options.is_some() {
            len += 1;
        }
        if self.db_path.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.v1.MystikoOptions", len)?;
        if let Some(v) = self.config_options.as_ref() {
            struct_ser.serialize_field("configOptions", v)?;
        }
        if let Some(v) = self.db_path.as_ref() {
            struct_ser.serialize_field("dbPath", v)?;
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
            "config_options",
            "configOptions",
            "db_path",
            "dbPath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigOptions,
            DbPath,
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
                            "configOptions" | "config_options" => Ok(GeneratedField::ConfigOptions),
                            "dbPath" | "db_path" => Ok(GeneratedField::DbPath),
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
                let mut config_options__ = None;
                let mut db_path__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ConfigOptions => {
                            if config_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configOptions"));
                            }
                            config_options__ = map.next_value()?;
                        }
                        GeneratedField::DbPath => {
                            if db_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbPath"));
                            }
                            db_path__ = map.next_value()?;
                        }
                    }
                }
                Ok(MystikoOptions {
                    config_options: config_options__,
                    db_path: db_path__,
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
