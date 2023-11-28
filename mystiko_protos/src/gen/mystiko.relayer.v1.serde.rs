// @generated
impl serde::Serialize for RelayerClientOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_testnet.is_some() {
            len += 1;
        }
        if self.is_staging.is_some() {
            len += 1;
        }
        if self.timeout_ms.is_some() {
            len += 1;
        }
        if self.relayer_config_file_path.is_some() {
            len += 1;
        }
        if self.relayer_config_remote_base_url.is_some() {
            len += 1;
        }
        if self.relayer_config_git_revision.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.relayer.v1.RelayerClientOptions", len)?;
        if let Some(v) = self.is_testnet.as_ref() {
            struct_ser.serialize_field("isTestnet", v)?;
        }
        if let Some(v) = self.is_staging.as_ref() {
            struct_ser.serialize_field("isStaging", v)?;
        }
        if let Some(v) = self.timeout_ms.as_ref() {
            struct_ser.serialize_field("timeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.relayer_config_file_path.as_ref() {
            struct_ser.serialize_field("relayerConfigFilePath", v)?;
        }
        if let Some(v) = self.relayer_config_remote_base_url.as_ref() {
            struct_ser.serialize_field("relayerConfigRemoteBaseUrl", v)?;
        }
        if let Some(v) = self.relayer_config_git_revision.as_ref() {
            struct_ser.serialize_field("relayerConfigGitRevision", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelayerClientOptions {
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
            "timeout_ms",
            "timeoutMs",
            "relayer_config_file_path",
            "relayerConfigFilePath",
            "relayer_config_remote_base_url",
            "relayerConfigRemoteBaseUrl",
            "relayer_config_git_revision",
            "relayerConfigGitRevision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsTestnet,
            IsStaging,
            TimeoutMs,
            RelayerConfigFilePath,
            RelayerConfigRemoteBaseUrl,
            RelayerConfigGitRevision,
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
                            "timeoutMs" | "timeout_ms" => Ok(GeneratedField::TimeoutMs),
                            "relayerConfigFilePath" | "relayer_config_file_path" => Ok(GeneratedField::RelayerConfigFilePath),
                            "relayerConfigRemoteBaseUrl" | "relayer_config_remote_base_url" => Ok(GeneratedField::RelayerConfigRemoteBaseUrl),
                            "relayerConfigGitRevision" | "relayer_config_git_revision" => Ok(GeneratedField::RelayerConfigGitRevision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelayerClientOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.relayer.v1.RelayerClientOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RelayerClientOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_testnet__ = None;
                let mut is_staging__ = None;
                let mut timeout_ms__ = None;
                let mut relayer_config_file_path__ = None;
                let mut relayer_config_remote_base_url__ = None;
                let mut relayer_config_git_revision__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::TimeoutMs => {
                            if timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutMs"));
                            }
                            timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RelayerConfigFilePath => {
                            if relayer_config_file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayerConfigFilePath"));
                            }
                            relayer_config_file_path__ = map.next_value()?;
                        }
                        GeneratedField::RelayerConfigRemoteBaseUrl => {
                            if relayer_config_remote_base_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayerConfigRemoteBaseUrl"));
                            }
                            relayer_config_remote_base_url__ = map.next_value()?;
                        }
                        GeneratedField::RelayerConfigGitRevision => {
                            if relayer_config_git_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayerConfigGitRevision"));
                            }
                            relayer_config_git_revision__ = map.next_value()?;
                        }
                    }
                }
                Ok(RelayerClientOptions {
                    is_testnet: is_testnet__,
                    is_staging: is_staging__,
                    timeout_ms: timeout_ms__,
                    relayer_config_file_path: relayer_config_file_path__,
                    relayer_config_remote_base_url: relayer_config_remote_base_url__,
                    relayer_config_git_revision: relayer_config_git_revision__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.relayer.v1.RelayerClientOptions", FIELDS, GeneratedVisitor)
    }
}
