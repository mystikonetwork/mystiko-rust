// @generated
impl serde::Serialize for ApiResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.v1.ApiResponse", len)?;
        if self.code != 0 {
            let v = StatusCode::from_i32(self.code)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
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
                            code__ = Some(map.next_value::<StatusCode>()? as i32);
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
                    code: code__.unwrap_or_default(),
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.v1.ApiResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatusCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATUS_CODE_UNSPECIFIED",
            Self::Success => "STATUS_CODE_SUCCESS",
            Self::UnknownError => "STATUS_CODE_UNKNOWN_ERROR",
            Self::ConfigError => "STATUS_CODE_CONFIG_ERROR",
            Self::DatabaseMigrationError => "STATUS_CODE_DATABASE_MIGRATION_ERROR",
            Self::CryptoError => "STATUS_CODE_CRYPTO_ERROR",
            Self::MnemonicError => "STATUS_CODE_MNEMONIC_ERROR",
            Self::HexStringError => "STATUS_CODE_HEX_STRING_ERROR",
            Self::StorageError => "STATUS_CODE_STORAGE_ERROR",
            Self::InvalidPasswordError => "STATUS_CODE_INVALID_PASSWORD_ERROR",
            Self::MismatchedPasswordError => "STATUS_CODE_MISMATCHED_PASSWORD_ERROR",
            Self::NoExistingWalletError => "STATUS_CODE_NO_EXISTING_WALLET_ERROR",
            Self::NoSuchAccountError => "STATUS_CODE_NO_SUCH_ACCOUNT_ERROR",
            Self::InvalidProviderUrlError => "STATUS_CODE_INVALID_PROVIDER_URL_ERROR",
            Self::DataLoaderError => "STATUS_CODE_DATA_LOADER_ERROR",
            Self::SynchronizerError => "STATUS_CODE_SYNCHRONIZER_ERROR",
            Self::GetMystikoGuardError => "STATUS_CODE_GET_MYSTIKO_GUARD_ERROR",
            Self::DeserializeMessageError => "STATUS_CODE_DESERIALIZE_MESSAGE_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for StatusCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATUS_CODE_UNSPECIFIED",
            "STATUS_CODE_SUCCESS",
            "STATUS_CODE_UNKNOWN_ERROR",
            "STATUS_CODE_CONFIG_ERROR",
            "STATUS_CODE_DATABASE_MIGRATION_ERROR",
            "STATUS_CODE_CRYPTO_ERROR",
            "STATUS_CODE_MNEMONIC_ERROR",
            "STATUS_CODE_HEX_STRING_ERROR",
            "STATUS_CODE_STORAGE_ERROR",
            "STATUS_CODE_INVALID_PASSWORD_ERROR",
            "STATUS_CODE_MISMATCHED_PASSWORD_ERROR",
            "STATUS_CODE_NO_EXISTING_WALLET_ERROR",
            "STATUS_CODE_NO_SUCH_ACCOUNT_ERROR",
            "STATUS_CODE_INVALID_PROVIDER_URL_ERROR",
            "STATUS_CODE_DATA_LOADER_ERROR",
            "STATUS_CODE_SYNCHRONIZER_ERROR",
            "STATUS_CODE_GET_MYSTIKO_GUARD_ERROR",
            "STATUS_CODE_DESERIALIZE_MESSAGE_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusCode;

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
                    .and_then(StatusCode::from_i32)
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
                    .and_then(StatusCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STATUS_CODE_UNSPECIFIED" => Ok(StatusCode::Unspecified),
                    "STATUS_CODE_SUCCESS" => Ok(StatusCode::Success),
                    "STATUS_CODE_UNKNOWN_ERROR" => Ok(StatusCode::UnknownError),
                    "STATUS_CODE_CONFIG_ERROR" => Ok(StatusCode::ConfigError),
                    "STATUS_CODE_DATABASE_MIGRATION_ERROR" => Ok(StatusCode::DatabaseMigrationError),
                    "STATUS_CODE_CRYPTO_ERROR" => Ok(StatusCode::CryptoError),
                    "STATUS_CODE_MNEMONIC_ERROR" => Ok(StatusCode::MnemonicError),
                    "STATUS_CODE_HEX_STRING_ERROR" => Ok(StatusCode::HexStringError),
                    "STATUS_CODE_STORAGE_ERROR" => Ok(StatusCode::StorageError),
                    "STATUS_CODE_INVALID_PASSWORD_ERROR" => Ok(StatusCode::InvalidPasswordError),
                    "STATUS_CODE_MISMATCHED_PASSWORD_ERROR" => Ok(StatusCode::MismatchedPasswordError),
                    "STATUS_CODE_NO_EXISTING_WALLET_ERROR" => Ok(StatusCode::NoExistingWalletError),
                    "STATUS_CODE_NO_SUCH_ACCOUNT_ERROR" => Ok(StatusCode::NoSuchAccountError),
                    "STATUS_CODE_INVALID_PROVIDER_URL_ERROR" => Ok(StatusCode::InvalidProviderUrlError),
                    "STATUS_CODE_DATA_LOADER_ERROR" => Ok(StatusCode::DataLoaderError),
                    "STATUS_CODE_SYNCHRONIZER_ERROR" => Ok(StatusCode::SynchronizerError),
                    "STATUS_CODE_GET_MYSTIKO_GUARD_ERROR" => Ok(StatusCode::GetMystikoGuardError),
                    "STATUS_CODE_DESERIALIZE_MESSAGE_ERROR" => Ok(StatusCode::DeserializeMessageError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
