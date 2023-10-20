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
impl serde::Serialize for GetAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mystiko.core.v1.GetAddressRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAddressRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.v1.GetAddressRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetAddressRequest {
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.v1.GetAddressRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.v1.GetAddressResponse", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAddressResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.v1.GetAddressResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetAddressResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetAddressResponse {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.v1.GetAddressResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for SendTransactionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_id != 0 {
            len += 1;
        }
        if self.transaction.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.v1.SendTransactionRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if let Some(v) = self.transaction.as_ref() {
            struct_ser.serialize_field("transaction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendTransactionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "transaction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            Transaction,
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
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "transaction" => Ok(GeneratedField::Transaction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendTransactionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.v1.SendTransactionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SendTransactionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut transaction__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Transaction => {
                            if transaction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transaction"));
                            }
                            transaction__ = map.next_value()?;
                        }
                    }
                }
                Ok(SendTransactionRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    transaction: transaction__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.v1.SendTransactionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendTransactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_id != 0 {
            len += 1;
        }
        if !self.tx_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.v1.SendTransactionResponse", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.tx_hash.is_empty() {
            struct_ser.serialize_field("txHash", &self.tx_hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendTransactionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "tx_hash",
            "txHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            TxHash,
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
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "txHash" | "tx_hash" => Ok(GeneratedField::TxHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendTransactionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.v1.SendTransactionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SendTransactionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut tx_hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TxHash => {
                            if tx_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHash"));
                            }
                            tx_hash__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SendTransactionResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                    tx_hash: tx_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.v1.SendTransactionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Transaction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.from.is_some() {
            len += 1;
        }
        if self.to.is_some() {
            len += 1;
        }
        if self.gas.is_some() {
            len += 1;
        }
        if self.gas_price.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        if self.nonce.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.v1.Transaction", len)?;
        if let Some(v) = self.from.as_ref() {
            struct_ser.serialize_field("from", v)?;
        }
        if let Some(v) = self.to.as_ref() {
            struct_ser.serialize_field("to", v)?;
        }
        if let Some(v) = self.gas.as_ref() {
            struct_ser.serialize_field("gas", v)?;
        }
        if let Some(v) = self.gas_price.as_ref() {
            struct_ser.serialize_field("gasPrice", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if let Some(v) = self.nonce.as_ref() {
            struct_ser.serialize_field("nonce", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Transaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "to",
            "gas",
            "gas_price",
            "gasPrice",
            "value",
            "data",
            "nonce",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            To,
            Gas,
            GasPrice,
            Value,
            Data,
            Nonce,
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
                            "from" => Ok(GeneratedField::From),
                            "to" => Ok(GeneratedField::To),
                            "gas" => Ok(GeneratedField::Gas),
                            "gasPrice" | "gas_price" => Ok(GeneratedField::GasPrice),
                            "value" => Ok(GeneratedField::Value),
                            "data" => Ok(GeneratedField::Data),
                            "nonce" => Ok(GeneratedField::Nonce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Transaction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.v1.Transaction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Transaction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut to__ = None;
                let mut gas__ = None;
                let mut gas_price__ = None;
                let mut value__ = None;
                let mut data__ = None;
                let mut nonce__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = map.next_value()?;
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = map.next_value()?;
                        }
                        GeneratedField::Gas => {
                            if gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gas"));
                            }
                            gas__ = map.next_value()?;
                        }
                        GeneratedField::GasPrice => {
                            if gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasPrice"));
                            }
                            gas_price__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map.next_value()?;
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = map.next_value()?;
                        }
                    }
                }
                Ok(Transaction {
                    from: from__,
                    to: to__,
                    gas: gas__,
                    gas_price: gas_price__,
                    value: value__,
                    data: data__,
                    nonce: nonce__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.v1.Transaction", FIELDS, GeneratedVisitor)
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
