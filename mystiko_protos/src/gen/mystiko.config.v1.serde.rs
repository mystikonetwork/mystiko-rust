// @generated
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
        let mut struct_ser = serializer.serialize_struct("mystiko.config.v1.ConfigOptions", len)?;
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
                formatter.write_str("struct mystiko.config.v1.ConfigOptions")
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
        deserializer.deserialize_struct("mystiko.config.v1.ConfigOptions", FIELDS, GeneratedVisitor)
    }
}
