// @generated
impl serde::Serialize for ScreeningClientOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timeout_ms.is_some() {
            len += 1;
        }
        if self.screening_config_api_url.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.screening.v1.ScreeningClientOptions", len)?;
        if let Some(v) = self.timeout_ms.as_ref() {
            struct_ser.serialize_field("timeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.screening_config_api_url.as_ref() {
            struct_ser.serialize_field("screeningConfigApiUrl", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScreeningClientOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timeout_ms",
            "timeoutMs",
            "screening_config_api_url",
            "screeningConfigApiUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TimeoutMs,
            ScreeningConfigApiUrl,
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
                            "timeoutMs" | "timeout_ms" => Ok(GeneratedField::TimeoutMs),
                            "screeningConfigApiUrl" | "screening_config_api_url" => Ok(GeneratedField::ScreeningConfigApiUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScreeningClientOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.screening.v1.ScreeningClientOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScreeningClientOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timeout_ms__ = None;
                let mut screening_config_api_url__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TimeoutMs => {
                            if timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutMs"));
                            }
                            timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ScreeningConfigApiUrl => {
                            if screening_config_api_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("screeningConfigApiUrl"));
                            }
                            screening_config_api_url__ = map.next_value()?;
                        }
                    }
                }
                Ok(ScreeningClientOptions {
                    timeout_ms: timeout_ms__,
                    screening_config_api_url: screening_config_api_url__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.screening.v1.ScreeningClientOptions", FIELDS, GeneratedVisitor)
    }
}
