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
        if let Some(v) = self.is_ssl.as_ref() {
            struct_ser.serialize_field("isSsl", v)?;
        }
        if let Some(v) = self.ssl_cert.as_ref() {
            struct_ser.serialize_field("sslCert", v)?;
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
                            ssl_cert__ = map.next_value()?;
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
impl serde::Serialize for ServerOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bind_address.is_some() {
            len += 1;
        }
        if self.port.is_some() {
            len += 1;
        }
        if self.tls_key.is_some() {
            len += 1;
        }
        if self.tls_key_path.is_some() {
            len += 1;
        }
        if self.tls_pem.is_some() {
            len += 1;
        }
        if self.tls_pem_path.is_some() {
            len += 1;
        }
        if self.accept_http1.is_some() {
            len += 1;
        }
        if self.enable_web.is_some() {
            len += 1;
        }
        if self.concurrency_limit_per_connection.is_some() {
            len += 1;
        }
        if self.timeout_ms.is_some() {
            len += 1;
        }
        if self.initial_stream_window_size.is_some() {
            len += 1;
        }
        if self.initial_connection_window_size.is_some() {
            len += 1;
        }
        if self.max_concurrent_streams.is_some() {
            len += 1;
        }
        if self.http2_keepalive_interval_ms.is_some() {
            len += 1;
        }
        if self.http2_keepalive_timeout_ms.is_some() {
            len += 1;
        }
        if self.http2_adaptive_window.is_some() {
            len += 1;
        }
        if self.tcp_keepalive_ms.is_some() {
            len += 1;
        }
        if self.tcp_nodelay.is_some() {
            len += 1;
        }
        if self.max_frame_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.service.v1.ServerOptions", len)?;
        if let Some(v) = self.bind_address.as_ref() {
            struct_ser.serialize_field("bindAddress", v)?;
        }
        if let Some(v) = self.port.as_ref() {
            struct_ser.serialize_field("port", v)?;
        }
        if let Some(v) = self.tls_key.as_ref() {
            struct_ser.serialize_field("tlsKey", v)?;
        }
        if let Some(v) = self.tls_key_path.as_ref() {
            struct_ser.serialize_field("tlsKeyPath", v)?;
        }
        if let Some(v) = self.tls_pem.as_ref() {
            struct_ser.serialize_field("tlsPem", v)?;
        }
        if let Some(v) = self.tls_pem_path.as_ref() {
            struct_ser.serialize_field("tlsPemPath", v)?;
        }
        if let Some(v) = self.accept_http1.as_ref() {
            struct_ser.serialize_field("acceptHttp1", v)?;
        }
        if let Some(v) = self.enable_web.as_ref() {
            struct_ser.serialize_field("enableWeb", v)?;
        }
        if let Some(v) = self.concurrency_limit_per_connection.as_ref() {
            struct_ser.serialize_field("concurrencyLimitPerConnection", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.timeout_ms.as_ref() {
            struct_ser.serialize_field("timeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.initial_stream_window_size.as_ref() {
            struct_ser.serialize_field("initialStreamWindowSize", v)?;
        }
        if let Some(v) = self.initial_connection_window_size.as_ref() {
            struct_ser.serialize_field("initialConnectionWindowSize", v)?;
        }
        if let Some(v) = self.max_concurrent_streams.as_ref() {
            struct_ser.serialize_field("maxConcurrentStreams", v)?;
        }
        if let Some(v) = self.http2_keepalive_interval_ms.as_ref() {
            struct_ser.serialize_field("http2KeepaliveIntervalMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.http2_keepalive_timeout_ms.as_ref() {
            struct_ser.serialize_field("http2KeepaliveTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.http2_adaptive_window.as_ref() {
            struct_ser.serialize_field("http2AdaptiveWindow", v)?;
        }
        if let Some(v) = self.tcp_keepalive_ms.as_ref() {
            struct_ser.serialize_field("tcpKeepaliveMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.tcp_nodelay.as_ref() {
            struct_ser.serialize_field("tcpNodelay", v)?;
        }
        if let Some(v) = self.max_frame_size.as_ref() {
            struct_ser.serialize_field("maxFrameSize", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bind_address",
            "bindAddress",
            "port",
            "tls_key",
            "tlsKey",
            "tls_key_path",
            "tlsKeyPath",
            "tls_pem",
            "tlsPem",
            "tls_pem_path",
            "tlsPemPath",
            "accept_http1",
            "acceptHttp1",
            "enable_web",
            "enableWeb",
            "concurrency_limit_per_connection",
            "concurrencyLimitPerConnection",
            "timeout_ms",
            "timeoutMs",
            "initial_stream_window_size",
            "initialStreamWindowSize",
            "initial_connection_window_size",
            "initialConnectionWindowSize",
            "max_concurrent_streams",
            "maxConcurrentStreams",
            "http2_keepalive_interval_ms",
            "http2KeepaliveIntervalMs",
            "http2_keepalive_timeout_ms",
            "http2KeepaliveTimeoutMs",
            "http2_adaptive_window",
            "http2AdaptiveWindow",
            "tcp_keepalive_ms",
            "tcpKeepaliveMs",
            "tcp_nodelay",
            "tcpNodelay",
            "max_frame_size",
            "maxFrameSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BindAddress,
            Port,
            TlsKey,
            TlsKeyPath,
            TlsPem,
            TlsPemPath,
            AcceptHttp1,
            EnableWeb,
            ConcurrencyLimitPerConnection,
            TimeoutMs,
            InitialStreamWindowSize,
            InitialConnectionWindowSize,
            MaxConcurrentStreams,
            Http2KeepaliveIntervalMs,
            Http2KeepaliveTimeoutMs,
            Http2AdaptiveWindow,
            TcpKeepaliveMs,
            TcpNodelay,
            MaxFrameSize,
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
                            "bindAddress" | "bind_address" => Ok(GeneratedField::BindAddress),
                            "port" => Ok(GeneratedField::Port),
                            "tlsKey" | "tls_key" => Ok(GeneratedField::TlsKey),
                            "tlsKeyPath" | "tls_key_path" => Ok(GeneratedField::TlsKeyPath),
                            "tlsPem" | "tls_pem" => Ok(GeneratedField::TlsPem),
                            "tlsPemPath" | "tls_pem_path" => Ok(GeneratedField::TlsPemPath),
                            "acceptHttp1" | "accept_http1" => Ok(GeneratedField::AcceptHttp1),
                            "enableWeb" | "enable_web" => Ok(GeneratedField::EnableWeb),
                            "concurrencyLimitPerConnection" | "concurrency_limit_per_connection" => Ok(GeneratedField::ConcurrencyLimitPerConnection),
                            "timeoutMs" | "timeout_ms" => Ok(GeneratedField::TimeoutMs),
                            "initialStreamWindowSize" | "initial_stream_window_size" => Ok(GeneratedField::InitialStreamWindowSize),
                            "initialConnectionWindowSize" | "initial_connection_window_size" => Ok(GeneratedField::InitialConnectionWindowSize),
                            "maxConcurrentStreams" | "max_concurrent_streams" => Ok(GeneratedField::MaxConcurrentStreams),
                            "http2KeepaliveIntervalMs" | "http2_keepalive_interval_ms" => Ok(GeneratedField::Http2KeepaliveIntervalMs),
                            "http2KeepaliveTimeoutMs" | "http2_keepalive_timeout_ms" => Ok(GeneratedField::Http2KeepaliveTimeoutMs),
                            "http2AdaptiveWindow" | "http2_adaptive_window" => Ok(GeneratedField::Http2AdaptiveWindow),
                            "tcpKeepaliveMs" | "tcp_keepalive_ms" => Ok(GeneratedField::TcpKeepaliveMs),
                            "tcpNodelay" | "tcp_nodelay" => Ok(GeneratedField::TcpNodelay),
                            "maxFrameSize" | "max_frame_size" => Ok(GeneratedField::MaxFrameSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.service.v1.ServerOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServerOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bind_address__ = None;
                let mut port__ = None;
                let mut tls_key__ = None;
                let mut tls_key_path__ = None;
                let mut tls_pem__ = None;
                let mut tls_pem_path__ = None;
                let mut accept_http1__ = None;
                let mut enable_web__ = None;
                let mut concurrency_limit_per_connection__ = None;
                let mut timeout_ms__ = None;
                let mut initial_stream_window_size__ = None;
                let mut initial_connection_window_size__ = None;
                let mut max_concurrent_streams__ = None;
                let mut http2_keepalive_interval_ms__ = None;
                let mut http2_keepalive_timeout_ms__ = None;
                let mut http2_adaptive_window__ = None;
                let mut tcp_keepalive_ms__ = None;
                let mut tcp_nodelay__ = None;
                let mut max_frame_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BindAddress => {
                            if bind_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bindAddress"));
                            }
                            bind_address__ = map.next_value()?;
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TlsKey => {
                            if tls_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsKey"));
                            }
                            tls_key__ = map.next_value()?;
                        }
                        GeneratedField::TlsKeyPath => {
                            if tls_key_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsKeyPath"));
                            }
                            tls_key_path__ = map.next_value()?;
                        }
                        GeneratedField::TlsPem => {
                            if tls_pem__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsPem"));
                            }
                            tls_pem__ = map.next_value()?;
                        }
                        GeneratedField::TlsPemPath => {
                            if tls_pem_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsPemPath"));
                            }
                            tls_pem_path__ = map.next_value()?;
                        }
                        GeneratedField::AcceptHttp1 => {
                            if accept_http1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("acceptHttp1"));
                            }
                            accept_http1__ = map.next_value()?;
                        }
                        GeneratedField::EnableWeb => {
                            if enable_web__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableWeb"));
                            }
                            enable_web__ = map.next_value()?;
                        }
                        GeneratedField::ConcurrencyLimitPerConnection => {
                            if concurrency_limit_per_connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("concurrencyLimitPerConnection"));
                            }
                            concurrency_limit_per_connection__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TimeoutMs => {
                            if timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutMs"));
                            }
                            timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::InitialStreamWindowSize => {
                            if initial_stream_window_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialStreamWindowSize"));
                            }
                            initial_stream_window_size__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::InitialConnectionWindowSize => {
                            if initial_connection_window_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialConnectionWindowSize"));
                            }
                            initial_connection_window_size__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxConcurrentStreams => {
                            if max_concurrent_streams__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConcurrentStreams"));
                            }
                            max_concurrent_streams__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Http2KeepaliveIntervalMs => {
                            if http2_keepalive_interval_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http2KeepaliveIntervalMs"));
                            }
                            http2_keepalive_interval_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Http2KeepaliveTimeoutMs => {
                            if http2_keepalive_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http2KeepaliveTimeoutMs"));
                            }
                            http2_keepalive_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Http2AdaptiveWindow => {
                            if http2_adaptive_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http2AdaptiveWindow"));
                            }
                            http2_adaptive_window__ = map.next_value()?;
                        }
                        GeneratedField::TcpKeepaliveMs => {
                            if tcp_keepalive_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpKeepaliveMs"));
                            }
                            tcp_keepalive_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TcpNodelay => {
                            if tcp_nodelay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpNodelay"));
                            }
                            tcp_nodelay__ = map.next_value()?;
                        }
                        GeneratedField::MaxFrameSize => {
                            if max_frame_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxFrameSize"));
                            }
                            max_frame_size__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(ServerOptions {
                    bind_address: bind_address__,
                    port: port__,
                    tls_key: tls_key__,
                    tls_key_path: tls_key_path__,
                    tls_pem: tls_pem__,
                    tls_pem_path: tls_pem_path__,
                    accept_http1: accept_http1__,
                    enable_web: enable_web__,
                    concurrency_limit_per_connection: concurrency_limit_per_connection__,
                    timeout_ms: timeout_ms__,
                    initial_stream_window_size: initial_stream_window_size__,
                    initial_connection_window_size: initial_connection_window_size__,
                    max_concurrent_streams: max_concurrent_streams__,
                    http2_keepalive_interval_ms: http2_keepalive_interval_ms__,
                    http2_keepalive_timeout_ms: http2_keepalive_timeout_ms__,
                    http2_adaptive_window: http2_adaptive_window__,
                    tcp_keepalive_ms: tcp_keepalive_ms__,
                    tcp_nodelay: tcp_nodelay__,
                    max_frame_size: max_frame_size__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.service.v1.ServerOptions", FIELDS, GeneratedVisitor)
    }
}
