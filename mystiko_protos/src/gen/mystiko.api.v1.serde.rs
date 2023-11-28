// @generated
impl serde::Serialize for AccountError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ACCOUNT_ERROR_UNSPECIFIED",
            Self::StorageError => "ACCOUNT_ERROR_STORAGE_ERROR",
            Self::CryptoError => "ACCOUNT_ERROR_CRYPTO_ERROR",
            Self::MnemonicError => "ACCOUNT_ERROR_MNEMONIC_ERROR",
            Self::HexStringError => "ACCOUNT_ERROR_HEX_STRING_ERROR",
            Self::NoSuchAccountError => "ACCOUNT_ERROR_NO_SUCH_ACCOUNT_ERROR",
            Self::WalletsError => "ACCOUNT_ERROR_WALLETS_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AccountError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACCOUNT_ERROR_UNSPECIFIED",
            "ACCOUNT_ERROR_STORAGE_ERROR",
            "ACCOUNT_ERROR_CRYPTO_ERROR",
            "ACCOUNT_ERROR_MNEMONIC_ERROR",
            "ACCOUNT_ERROR_HEX_STRING_ERROR",
            "ACCOUNT_ERROR_NO_SUCH_ACCOUNT_ERROR",
            "ACCOUNT_ERROR_WALLETS_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccountError;

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
                    .and_then(AccountError::from_i32)
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
                    .and_then(AccountError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ACCOUNT_ERROR_UNSPECIFIED" => Ok(AccountError::Unspecified),
                    "ACCOUNT_ERROR_STORAGE_ERROR" => Ok(AccountError::StorageError),
                    "ACCOUNT_ERROR_CRYPTO_ERROR" => Ok(AccountError::CryptoError),
                    "ACCOUNT_ERROR_MNEMONIC_ERROR" => Ok(AccountError::MnemonicError),
                    "ACCOUNT_ERROR_HEX_STRING_ERROR" => Ok(AccountError::HexStringError),
                    "ACCOUNT_ERROR_NO_SUCH_ACCOUNT_ERROR" => Ok(AccountError::NoSuchAccountError),
                    "ACCOUNT_ERROR_WALLETS_ERROR" => Ok(AccountError::WalletsError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ApiResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.v1.ApiResponse", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            match v {
                api_response::Result::Data(v) => {
                    struct_ser.serialize_field("data", pbjson::private::base64::encode(&v).as_str())?;
                }
                api_response::Result::ErrorMessage(v) => {
                    struct_ser.serialize_field("errorMessage", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "data",
            "error_message",
            "errorMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Data,
            ErrorMessage,
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
                            "code" => Ok(GeneratedField::Code),
                            "data" => Ok(GeneratedField::Data),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.v1.ApiResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApiResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map.next_value()?;
                        }
                        GeneratedField::Data => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            result__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| api_response::Result::Data(x.0));
                        }
                        GeneratedField::ErrorMessage => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            result__ = map.next_value::<::std::option::Option<_>>()?.map(api_response::Result::ErrorMessage);
                        }
                    }
                }
                Ok(ApiResponse {
                    code: code__,
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.v1.ApiResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DepositError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DEPOSIT_ERROR_UNSPECIFIED",
            Self::HexStringError => "DEPOSIT_ERROR_HEX_STRING_ERROR",
            Self::FromDecStrError => "DEPOSIT_ERROR_FROM_DEC_STR_ERROR",
            Self::ParseBytesError => "DEPOSIT_ERROR_PARSE_BYTES_ERROR",
            Self::ProviderError => "DEPOSIT_ERROR_PROVIDER_ERROR",
            Self::AnyhowError => "DEPOSIT_ERROR_ANYHOW_ERROR",
            Self::PublicAssetsError => "DEPOSIT_ERROR_PUBLIC_ASSETS_ERROR",
            Self::DepositContractsError => "DEPOSIT_ERROR_DEPOSIT_CONTRACTS_ERROR",
            Self::CommitmentPoolContractsError => "DEPOSIT_ERROR_COMMITMENT_POOL_CONTRACTS_ERROR",
            Self::TransactionsError => "DEPOSIT_ERROR_TRANSACTIONS_ERROR",
            Self::ProtocolError => "DEPOSIT_ERROR_PROTOCOL_ERROR",
            Self::StorageError => "DEPOSIT_ERROR_STORAGE_ERROR",
            Self::ParseBigIntError => "DEPOSIT_ERROR_PARSE_BIG_INT_ERROR",
            Self::UnsupportedChainIdError => "DEPOSIT_ERROR_UNSUPPORTED_CHAIN_ID_ERROR",
            Self::NoDepositContractFoundError => "DEPOSIT_ERROR_NO_DEPOSIT_CONTRACT_FOUND_ERROR",
            Self::InvalidDepositAmountError => "DEPOSIT_ERROR_INVALID_DEPOSIT_AMOUNT_ERROR",
            Self::InvalidRollupFeeAmountError => "DEPOSIT_ERROR_INVALID_ROLLUP_FEE_AMOUNT_ERROR",
            Self::InvalidBridgeFeeAmountError => "DEPOSIT_ERROR_INVALID_BRIDGE_FEE_AMOUNT_ERROR",
            Self::InvalidExecutorFeeAmountError => "DEPOSIT_ERROR_INVALID_EXECUTOR_FEE_AMOUNT_ERROR",
            Self::InsufficientBalanceError => "DEPOSIT_ERROR_INSUFFICIENT_BALANCE_ERROR",
            Self::IdNotFoundError => "DEPOSIT_ERROR_ID_NOT_FOUND_ERROR",
            Self::MissingPrivateKeyError => "DEPOSIT_ERROR_MISSING_PRIVATE_KEY_ERROR",
            Self::DepositStatusError => "DEPOSIT_ERROR_DEPOSIT_STATUS_ERROR",
            Self::DuplicateCommitmentError => "DEPOSIT_ERROR_DUPLICATE_COMMITMENT_ERROR",
            Self::WalletsError => "DEPOSIT_ERROR_WALLETS_ERROR",
            Self::AccountsError => "DEPOSIT_ERROR_ACCOUNTS_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for DepositError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEPOSIT_ERROR_UNSPECIFIED",
            "DEPOSIT_ERROR_HEX_STRING_ERROR",
            "DEPOSIT_ERROR_FROM_DEC_STR_ERROR",
            "DEPOSIT_ERROR_PARSE_BYTES_ERROR",
            "DEPOSIT_ERROR_PROVIDER_ERROR",
            "DEPOSIT_ERROR_ANYHOW_ERROR",
            "DEPOSIT_ERROR_PUBLIC_ASSETS_ERROR",
            "DEPOSIT_ERROR_DEPOSIT_CONTRACTS_ERROR",
            "DEPOSIT_ERROR_COMMITMENT_POOL_CONTRACTS_ERROR",
            "DEPOSIT_ERROR_TRANSACTIONS_ERROR",
            "DEPOSIT_ERROR_PROTOCOL_ERROR",
            "DEPOSIT_ERROR_STORAGE_ERROR",
            "DEPOSIT_ERROR_PARSE_BIG_INT_ERROR",
            "DEPOSIT_ERROR_UNSUPPORTED_CHAIN_ID_ERROR",
            "DEPOSIT_ERROR_NO_DEPOSIT_CONTRACT_FOUND_ERROR",
            "DEPOSIT_ERROR_INVALID_DEPOSIT_AMOUNT_ERROR",
            "DEPOSIT_ERROR_INVALID_ROLLUP_FEE_AMOUNT_ERROR",
            "DEPOSIT_ERROR_INVALID_BRIDGE_FEE_AMOUNT_ERROR",
            "DEPOSIT_ERROR_INVALID_EXECUTOR_FEE_AMOUNT_ERROR",
            "DEPOSIT_ERROR_INSUFFICIENT_BALANCE_ERROR",
            "DEPOSIT_ERROR_ID_NOT_FOUND_ERROR",
            "DEPOSIT_ERROR_MISSING_PRIVATE_KEY_ERROR",
            "DEPOSIT_ERROR_DEPOSIT_STATUS_ERROR",
            "DEPOSIT_ERROR_DUPLICATE_COMMITMENT_ERROR",
            "DEPOSIT_ERROR_WALLETS_ERROR",
            "DEPOSIT_ERROR_ACCOUNTS_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DepositError;

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
                    .and_then(DepositError::from_i32)
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
                    .and_then(DepositError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEPOSIT_ERROR_UNSPECIFIED" => Ok(DepositError::Unspecified),
                    "DEPOSIT_ERROR_HEX_STRING_ERROR" => Ok(DepositError::HexStringError),
                    "DEPOSIT_ERROR_FROM_DEC_STR_ERROR" => Ok(DepositError::FromDecStrError),
                    "DEPOSIT_ERROR_PARSE_BYTES_ERROR" => Ok(DepositError::ParseBytesError),
                    "DEPOSIT_ERROR_PROVIDER_ERROR" => Ok(DepositError::ProviderError),
                    "DEPOSIT_ERROR_ANYHOW_ERROR" => Ok(DepositError::AnyhowError),
                    "DEPOSIT_ERROR_PUBLIC_ASSETS_ERROR" => Ok(DepositError::PublicAssetsError),
                    "DEPOSIT_ERROR_DEPOSIT_CONTRACTS_ERROR" => Ok(DepositError::DepositContractsError),
                    "DEPOSIT_ERROR_COMMITMENT_POOL_CONTRACTS_ERROR" => Ok(DepositError::CommitmentPoolContractsError),
                    "DEPOSIT_ERROR_TRANSACTIONS_ERROR" => Ok(DepositError::TransactionsError),
                    "DEPOSIT_ERROR_PROTOCOL_ERROR" => Ok(DepositError::ProtocolError),
                    "DEPOSIT_ERROR_STORAGE_ERROR" => Ok(DepositError::StorageError),
                    "DEPOSIT_ERROR_PARSE_BIG_INT_ERROR" => Ok(DepositError::ParseBigIntError),
                    "DEPOSIT_ERROR_UNSUPPORTED_CHAIN_ID_ERROR" => Ok(DepositError::UnsupportedChainIdError),
                    "DEPOSIT_ERROR_NO_DEPOSIT_CONTRACT_FOUND_ERROR" => Ok(DepositError::NoDepositContractFoundError),
                    "DEPOSIT_ERROR_INVALID_DEPOSIT_AMOUNT_ERROR" => Ok(DepositError::InvalidDepositAmountError),
                    "DEPOSIT_ERROR_INVALID_ROLLUP_FEE_AMOUNT_ERROR" => Ok(DepositError::InvalidRollupFeeAmountError),
                    "DEPOSIT_ERROR_INVALID_BRIDGE_FEE_AMOUNT_ERROR" => Ok(DepositError::InvalidBridgeFeeAmountError),
                    "DEPOSIT_ERROR_INVALID_EXECUTOR_FEE_AMOUNT_ERROR" => Ok(DepositError::InvalidExecutorFeeAmountError),
                    "DEPOSIT_ERROR_INSUFFICIENT_BALANCE_ERROR" => Ok(DepositError::InsufficientBalanceError),
                    "DEPOSIT_ERROR_ID_NOT_FOUND_ERROR" => Ok(DepositError::IdNotFoundError),
                    "DEPOSIT_ERROR_MISSING_PRIVATE_KEY_ERROR" => Ok(DepositError::MissingPrivateKeyError),
                    "DEPOSIT_ERROR_DEPOSIT_STATUS_ERROR" => Ok(DepositError::DepositStatusError),
                    "DEPOSIT_ERROR_DUPLICATE_COMMITMENT_ERROR" => Ok(DepositError::DuplicateCommitmentError),
                    "DEPOSIT_ERROR_WALLETS_ERROR" => Ok(DepositError::WalletsError),
                    "DEPOSIT_ERROR_ACCOUNTS_ERROR" => Ok(DepositError::AccountsError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MystikoError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MYSTIKO_ERROR_UNSPECIFIED",
            Self::SynchronizerError => "MYSTIKO_ERROR_SYNCHRONIZER_ERROR",
            Self::ScannerError => "MYSTIKO_ERROR_SCANNER_ERROR",
            Self::DataloaderError => "MYSTIKO_ERROR_DATALOADER_ERROR",
            Self::ConfigError => "MYSTIKO_ERROR_CONFIG_ERROR",
            Self::DatabaseMigrationError => "MYSTIKO_ERROR_DATABASE_MIGRATION_ERROR",
            Self::InvalidProviderUrlError => "MYSTIKO_ERROR_INVALID_PROVIDER_URL_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MystikoError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MYSTIKO_ERROR_UNSPECIFIED",
            "MYSTIKO_ERROR_SYNCHRONIZER_ERROR",
            "MYSTIKO_ERROR_SCANNER_ERROR",
            "MYSTIKO_ERROR_DATALOADER_ERROR",
            "MYSTIKO_ERROR_CONFIG_ERROR",
            "MYSTIKO_ERROR_DATABASE_MIGRATION_ERROR",
            "MYSTIKO_ERROR_INVALID_PROVIDER_URL_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MystikoError;

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
                    .and_then(MystikoError::from_i32)
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
                    .and_then(MystikoError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "MYSTIKO_ERROR_UNSPECIFIED" => Ok(MystikoError::Unspecified),
                    "MYSTIKO_ERROR_SYNCHRONIZER_ERROR" => Ok(MystikoError::SynchronizerError),
                    "MYSTIKO_ERROR_SCANNER_ERROR" => Ok(MystikoError::ScannerError),
                    "MYSTIKO_ERROR_DATALOADER_ERROR" => Ok(MystikoError::DataloaderError),
                    "MYSTIKO_ERROR_CONFIG_ERROR" => Ok(MystikoError::ConfigError),
                    "MYSTIKO_ERROR_DATABASE_MIGRATION_ERROR" => Ok(MystikoError::DatabaseMigrationError),
                    "MYSTIKO_ERROR_INVALID_PROVIDER_URL_ERROR" => Ok(MystikoError::InvalidProviderUrlError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MystikoLibError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MYSTIKO_LIB_ERROR_UNSPECIFIED",
            Self::GetMystikoGuardError => "MYSTIKO_LIB_ERROR_GET_MYSTIKO_GUARD_ERROR",
            Self::DeserializeMessageError => "MYSTIKO_LIB_ERROR_DESERIALIZE_MESSAGE_ERROR",
            Self::StorageError => "MYSTIKO_LIB_ERROR_STORAGE_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MystikoLibError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MYSTIKO_LIB_ERROR_UNSPECIFIED",
            "MYSTIKO_LIB_ERROR_GET_MYSTIKO_GUARD_ERROR",
            "MYSTIKO_LIB_ERROR_DESERIALIZE_MESSAGE_ERROR",
            "MYSTIKO_LIB_ERROR_STORAGE_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MystikoLibError;

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
                    .and_then(MystikoLibError::from_i32)
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
                    .and_then(MystikoLibError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "MYSTIKO_LIB_ERROR_UNSPECIFIED" => Ok(MystikoLibError::Unspecified),
                    "MYSTIKO_LIB_ERROR_GET_MYSTIKO_GUARD_ERROR" => Ok(MystikoLibError::GetMystikoGuardError),
                    "MYSTIKO_LIB_ERROR_DESERIALIZE_MESSAGE_ERROR" => Ok(MystikoLibError::DeserializeMessageError),
                    "MYSTIKO_LIB_ERROR_STORAGE_ERROR" => Ok(MystikoLibError::StorageError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ScannerError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SCANNER_ERROR_UNSPECIFIED",
            Self::NoSuchAccountError => "SCANNER_ERROR_NO_SUCH_ACCOUNT_ERROR",
            Self::NoSuchContractConfigError => "SCANNER_ERROR_NO_SUCH_CONTRACT_CONFIG_ERROR",
            Self::CommitmentEmptyError => "SCANNER_ERROR_COMMITMENT_EMPTY_ERROR",
            Self::CryptoError => "SCANNER_ERROR_CRYPTO_ERROR",
            Self::StorageError => "SCANNER_ERROR_STORAGE_ERROR",
            Self::JoinError => "SCANNER_ERROR_JOIN_ERROR",
            Self::ProtocolError => "SCANNER_ERROR_PROTOCOL_ERROR",
            Self::FromHexError => "SCANNER_ERROR_FROM_HEX_ERROR",
            Self::AnyhowError => "SCANNER_ERROR_ANYHOW_ERROR",
            Self::AccountHandlerError => "SCANNER_ERROR_ACCOUNT_HANDLER_ERROR",
            Self::WalletHandlerError => "SCANNER_ERROR_WALLET_HANDLER_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ScannerError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SCANNER_ERROR_UNSPECIFIED",
            "SCANNER_ERROR_NO_SUCH_ACCOUNT_ERROR",
            "SCANNER_ERROR_NO_SUCH_CONTRACT_CONFIG_ERROR",
            "SCANNER_ERROR_COMMITMENT_EMPTY_ERROR",
            "SCANNER_ERROR_CRYPTO_ERROR",
            "SCANNER_ERROR_STORAGE_ERROR",
            "SCANNER_ERROR_JOIN_ERROR",
            "SCANNER_ERROR_PROTOCOL_ERROR",
            "SCANNER_ERROR_FROM_HEX_ERROR",
            "SCANNER_ERROR_ANYHOW_ERROR",
            "SCANNER_ERROR_ACCOUNT_HANDLER_ERROR",
            "SCANNER_ERROR_WALLET_HANDLER_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScannerError;

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
                    .and_then(ScannerError::from_i32)
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
                    .and_then(ScannerError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SCANNER_ERROR_UNSPECIFIED" => Ok(ScannerError::Unspecified),
                    "SCANNER_ERROR_NO_SUCH_ACCOUNT_ERROR" => Ok(ScannerError::NoSuchAccountError),
                    "SCANNER_ERROR_NO_SUCH_CONTRACT_CONFIG_ERROR" => Ok(ScannerError::NoSuchContractConfigError),
                    "SCANNER_ERROR_COMMITMENT_EMPTY_ERROR" => Ok(ScannerError::CommitmentEmptyError),
                    "SCANNER_ERROR_CRYPTO_ERROR" => Ok(ScannerError::CryptoError),
                    "SCANNER_ERROR_STORAGE_ERROR" => Ok(ScannerError::StorageError),
                    "SCANNER_ERROR_JOIN_ERROR" => Ok(ScannerError::JoinError),
                    "SCANNER_ERROR_PROTOCOL_ERROR" => Ok(ScannerError::ProtocolError),
                    "SCANNER_ERROR_FROM_HEX_ERROR" => Ok(ScannerError::FromHexError),
                    "SCANNER_ERROR_ANYHOW_ERROR" => Ok(ScannerError::AnyhowError),
                    "SCANNER_ERROR_ACCOUNT_HANDLER_ERROR" => Ok(ScannerError::AccountHandlerError),
                    "SCANNER_ERROR_WALLET_HANDLER_ERROR" => Ok(ScannerError::WalletHandlerError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StatusCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.v1.StatusCode", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if let Some(v) = self.error.as_ref() {
            match v {
                status_code::Error::Lib(v) => {
                    let v = MystikoLibError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("lib", &v)?;
                }
                status_code::Error::Mystiko(v) => {
                    let v = MystikoError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("mystiko", &v)?;
                }
                status_code::Error::Wallet(v) => {
                    let v = WalletError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("wallet", &v)?;
                }
                status_code::Error::Account(v) => {
                    let v = AccountError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("account", &v)?;
                }
                status_code::Error::Deposit(v) => {
                    let v = DepositError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("deposit", &v)?;
                }
                status_code::Error::Scanner(v) => {
                    let v = ScannerError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("scanner", &v)?;
                }
                status_code::Error::Synchronize(v) => {
                    let v = SynchronizeError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("synchronize", &v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatusCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "lib",
            "mystiko",
            "wallet",
            "account",
            "deposit",
            "scanner",
            "synchronize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            Lib,
            Mystiko,
            Wallet,
            Account,
            Deposit,
            Scanner,
            Synchronize,
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
                            "success" => Ok(GeneratedField::Success),
                            "lib" => Ok(GeneratedField::Lib),
                            "mystiko" => Ok(GeneratedField::Mystiko),
                            "wallet" => Ok(GeneratedField::Wallet),
                            "account" => Ok(GeneratedField::Account),
                            "deposit" => Ok(GeneratedField::Deposit),
                            "scanner" => Ok(GeneratedField::Scanner),
                            "synchronize" => Ok(GeneratedField::Synchronize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.v1.StatusCode")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatusCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map.next_value()?);
                        }
                        GeneratedField::Lib => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lib"));
                            }
                            error__ = map.next_value::<::std::option::Option<MystikoLibError>>()?.map(|x| status_code::Error::Lib(x as i32));
                        }
                        GeneratedField::Mystiko => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mystiko"));
                            }
                            error__ = map.next_value::<::std::option::Option<MystikoError>>()?.map(|x| status_code::Error::Mystiko(x as i32));
                        }
                        GeneratedField::Wallet => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wallet"));
                            }
                            error__ = map.next_value::<::std::option::Option<WalletError>>()?.map(|x| status_code::Error::Wallet(x as i32));
                        }
                        GeneratedField::Account => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            error__ = map.next_value::<::std::option::Option<AccountError>>()?.map(|x| status_code::Error::Account(x as i32));
                        }
                        GeneratedField::Deposit => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposit"));
                            }
                            error__ = map.next_value::<::std::option::Option<DepositError>>()?.map(|x| status_code::Error::Deposit(x as i32));
                        }
                        GeneratedField::Scanner => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scanner"));
                            }
                            error__ = map.next_value::<::std::option::Option<ScannerError>>()?.map(|x| status_code::Error::Scanner(x as i32));
                        }
                        GeneratedField::Synchronize => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("synchronize"));
                            }
                            error__ = map.next_value::<::std::option::Option<SynchronizeError>>()?.map(|x| status_code::Error::Synchronize(x as i32));
                        }
                    }
                }
                Ok(StatusCode {
                    success: success__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.v1.StatusCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SynchronizeError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SYNCHRONIZE_ERROR_UNSPECIFIED",
            Self::UnsupportedChainError => "SYNCHRONIZE_ERROR_UNSUPPORTED_CHAIN_ERROR",
            Self::DataloaderError => "SYNCHRONIZE_ERROR_DATALOADER_ERROR",
            Self::DataloaderConfigError => "SYNCHRONIZE_ERROR_DATALOADER_CONFIG_ERROR",
            Self::AnyhowError => "SYNCHRONIZE_ERROR_ANYHOW_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SynchronizeError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SYNCHRONIZE_ERROR_UNSPECIFIED",
            "SYNCHRONIZE_ERROR_UNSUPPORTED_CHAIN_ERROR",
            "SYNCHRONIZE_ERROR_DATALOADER_ERROR",
            "SYNCHRONIZE_ERROR_DATALOADER_CONFIG_ERROR",
            "SYNCHRONIZE_ERROR_ANYHOW_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SynchronizeError;

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
                    .and_then(SynchronizeError::from_i32)
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
                    .and_then(SynchronizeError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SYNCHRONIZE_ERROR_UNSPECIFIED" => Ok(SynchronizeError::Unspecified),
                    "SYNCHRONIZE_ERROR_UNSUPPORTED_CHAIN_ERROR" => Ok(SynchronizeError::UnsupportedChainError),
                    "SYNCHRONIZE_ERROR_DATALOADER_ERROR" => Ok(SynchronizeError::DataloaderError),
                    "SYNCHRONIZE_ERROR_DATALOADER_CONFIG_ERROR" => Ok(SynchronizeError::DataloaderConfigError),
                    "SYNCHRONIZE_ERROR_ANYHOW_ERROR" => Ok(SynchronizeError::AnyhowError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WalletError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "WALLET_ERROR_UNSPECIFIED",
            Self::StorageError => "WALLET_ERROR_STORAGE_ERROR",
            Self::CryptoError => "WALLET_ERROR_CRYPTO_ERROR",
            Self::HexStringError => "WALLET_ERROR_HEX_STRING_ERROR",
            Self::MnemonicError => "WALLET_ERROR_MNEMONIC_ERROR",
            Self::InvalidPasswordError => "WALLET_ERROR_INVALID_PASSWORD_ERROR",
            Self::MismatchedPasswordError => "WALLET_ERROR_MISMATCHED_PASSWORD_ERROR",
            Self::NoExistingWalletError => "WALLET_ERROR_NO_EXISTING_WALLET_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for WalletError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "WALLET_ERROR_UNSPECIFIED",
            "WALLET_ERROR_STORAGE_ERROR",
            "WALLET_ERROR_CRYPTO_ERROR",
            "WALLET_ERROR_HEX_STRING_ERROR",
            "WALLET_ERROR_MNEMONIC_ERROR",
            "WALLET_ERROR_INVALID_PASSWORD_ERROR",
            "WALLET_ERROR_MISMATCHED_PASSWORD_ERROR",
            "WALLET_ERROR_NO_EXISTING_WALLET_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WalletError;

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
                    .and_then(WalletError::from_i32)
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
                    .and_then(WalletError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "WALLET_ERROR_UNSPECIFIED" => Ok(WalletError::Unspecified),
                    "WALLET_ERROR_STORAGE_ERROR" => Ok(WalletError::StorageError),
                    "WALLET_ERROR_CRYPTO_ERROR" => Ok(WalletError::CryptoError),
                    "WALLET_ERROR_HEX_STRING_ERROR" => Ok(WalletError::HexStringError),
                    "WALLET_ERROR_MNEMONIC_ERROR" => Ok(WalletError::MnemonicError),
                    "WALLET_ERROR_INVALID_PASSWORD_ERROR" => Ok(WalletError::InvalidPasswordError),
                    "WALLET_ERROR_MISMATCHED_PASSWORD_ERROR" => Ok(WalletError::MismatchedPasswordError),
                    "WALLET_ERROR_NO_EXISTING_WALLET_ERROR" => Ok(WalletError::NoExistingWalletError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
