// @generated
impl serde::Serialize for BridgeConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.bridge_type != 0 {
            len += 1;
        }
        if self.explorer_url.is_some() {
            len += 1;
        }
        if self.explorer_prefix.is_some() {
            len += 1;
        }
        if self.api_url.is_some() {
            len += 1;
        }
        if self.api_prefix.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.bridge.v1.BridgeConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.bridge_type != 0 {
            let v = super::super::super::common::v1::BridgeType::from_i32(self.bridge_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.bridge_type)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        if let Some(v) = self.explorer_url.as_ref() {
            struct_ser.serialize_field("explorerUrl", v)?;
        }
        if let Some(v) = self.explorer_prefix.as_ref() {
            struct_ser.serialize_field("explorerPrefix", v)?;
        }
        if let Some(v) = self.api_url.as_ref() {
            struct_ser.serialize_field("apiUrl", v)?;
        }
        if let Some(v) = self.api_prefix.as_ref() {
            struct_ser.serialize_field("apiPrefix", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BridgeConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "bridge_type",
            "bridgeType",
            "explorer_url",
            "explorerUrl",
            "explorer_prefix",
            "explorerPrefix",
            "api_url",
            "apiUrl",
            "api_prefix",
            "apiPrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            BridgeType,
            ExplorerUrl,
            ExplorerPrefix,
            ApiUrl,
            ApiPrefix,
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
                            "name" => Ok(GeneratedField::Name),
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
                            "explorerUrl" | "explorer_url" => Ok(GeneratedField::ExplorerUrl),
                            "explorerPrefix" | "explorer_prefix" => Ok(GeneratedField::ExplorerPrefix),
                            "apiUrl" | "api_url" => Ok(GeneratedField::ApiUrl),
                            "apiPrefix" | "api_prefix" => Ok(GeneratedField::ApiPrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BridgeConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.bridge.v1.BridgeConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BridgeConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut bridge_type__ = None;
                let mut explorer_url__ = None;
                let mut explorer_prefix__ = None;
                let mut api_url__ = None;
                let mut api_prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = Some(map.next_value::<super::super::super::common::v1::BridgeType>()? as i32);
                        }
                        GeneratedField::ExplorerUrl => {
                            if explorer_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("explorerUrl"));
                            }
                            explorer_url__ = map.next_value()?;
                        }
                        GeneratedField::ExplorerPrefix => {
                            if explorer_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("explorerPrefix"));
                            }
                            explorer_prefix__ = map.next_value()?;
                        }
                        GeneratedField::ApiUrl => {
                            if api_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiUrl"));
                            }
                            api_url__ = map.next_value()?;
                        }
                        GeneratedField::ApiPrefix => {
                            if api_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiPrefix"));
                            }
                            api_prefix__ = map.next_value()?;
                        }
                    }
                }
                Ok(BridgeConfig {
                    name: name__.unwrap_or_default(),
                    bridge_type: bridge_type__.unwrap_or_default(),
                    explorer_url: explorer_url__,
                    explorer_prefix: explorer_prefix__,
                    api_url: api_url__,
                    api_prefix: api_prefix__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.bridge.v1.BridgeConfig", FIELDS, GeneratedVisitor)
    }
}
