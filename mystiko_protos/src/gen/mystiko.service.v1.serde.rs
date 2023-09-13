// @generated
impl serde::Serialize for ClientOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host.is_some() {
            len += 1;
        }
        if self.port.is_some() {
            len += 1;
        }
        if self.is_web.is_some() {
            len += 1;
        }
        if self.is_ssl.is_some() {
            len += 1;
        }
        if self.ssl_cert.is_some() {
            len += 1;
        }
        if self.ssl_cert_path.is_some() {
            len += 1;
        }
        if self.ssl_server_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.service.v1.ClientOptions", len)?;
        if let Some(v) = self.host.as_ref() {
            struct_ser.serialize_field("host", v)?;
        }
        if let Some(v) = self.port.as_ref() {
            struct_ser.serialize_field("port", v)?;
        }
        if let Some(v) = self.is_web.as_ref() {
            struct_ser.serialize_field("isWeb", v)?;
        }
        if let Some(v) = self.is_ssl.as_ref() {
            struct_ser.serialize_field("isSsl", v)?;
        }
        if let Some(v) = self.ssl_cert.as_ref() {
            struct_ser.serialize_field("sslCert", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.ssl_cert_path.as_ref() {
            struct_ser.serialize_field("sslCertPath", v)?;
        }
        if let Some(v) = self.ssl_server_name.as_ref() {
            struct_ser.serialize_field("sslServerName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host",
            "port",
            "is_web",
            "isWeb",
            "is_ssl",
            "isSsl",
            "ssl_cert",
            "sslCert",
            "ssl_cert_path",
            "sslCertPath",
            "ssl_server_name",
            "sslServerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Host,
            Port,
            IsWeb,
            IsSsl,
            SslCert,
            SslCertPath,
            SslServerName,
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
                            "host" => Ok(GeneratedField::Host),
                            "port" => Ok(GeneratedField::Port),
                            "isWeb" | "is_web" => Ok(GeneratedField::IsWeb),
                            "isSsl" | "is_ssl" => Ok(GeneratedField::IsSsl),
                            "sslCert" | "ssl_cert" => Ok(GeneratedField::SslCert),
                            "sslCertPath" | "ssl_cert_path" => Ok(GeneratedField::SslCertPath),
                            "sslServerName" | "ssl_server_name" => Ok(GeneratedField::SslServerName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.service.v1.ClientOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host__ = None;
                let mut port__ = None;
                let mut is_web__ = None;
                let mut is_ssl__ = None;
                let mut ssl_cert__ = None;
                let mut ssl_cert_path__ = None;
                let mut ssl_server_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = map.next_value()?;
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IsWeb => {
                            if is_web__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isWeb"));
                            }
                            is_web__ = map.next_value()?;
                        }
                        GeneratedField::IsSsl => {
                            if is_ssl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSsl"));
                            }
                            is_ssl__ = map.next_value()?;
                        }
                        GeneratedField::SslCert => {
                            if ssl_cert__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sslCert"));
                            }
                            ssl_cert__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SslCertPath => {
                            if ssl_cert_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sslCertPath"));
                            }
                            ssl_cert_path__ = map.next_value()?;
                        }
                        GeneratedField::SslServerName => {
                            if ssl_server_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sslServerName"));
                            }
                            ssl_server_name__ = map.next_value()?;
                        }
                    }
                }
                Ok(ClientOptions {
                    host: host__,
                    port: port__,
                    is_web: is_web__,
                    is_ssl: is_ssl__,
                    ssl_cert: ssl_cert__,
                    ssl_cert_path: ssl_cert_path__,
                    ssl_server_name: ssl_server_name__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.service.v1.ClientOptions", FIELDS, GeneratedVisitor)
    }
}
