// @generated
impl serde::Serialize for EtherscanFetcherChainConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.url.is_some() {
            len += 1;
        }
        if self.api_key.is_some() {
            len += 1;
        }
        if self.max_requests_per_second.is_some() {
            len += 1;
        }
        if self.page_size.is_some() {
            len += 1;
        }
        if self.url_prefix.is_some() {
            len += 1;
        }
        if self.delay_num_blocks.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.loader.v1.EtherscanFetcherChainConfig", len)?;
        if let Some(v) = self.url.as_ref() {
            struct_ser.serialize_field("url", v)?;
        }
        if let Some(v) = self.api_key.as_ref() {
            struct_ser.serialize_field("apiKey", v)?;
        }
        if let Some(v) = self.max_requests_per_second.as_ref() {
            struct_ser.serialize_field("maxRequestsPerSecond", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.page_size.as_ref() {
            struct_ser.serialize_field("pageSize", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.url_prefix.as_ref() {
            struct_ser.serialize_field("urlPrefix", v)?;
        }
        if let Some(v) = self.delay_num_blocks.as_ref() {
            struct_ser.serialize_field("delayNumBlocks", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EtherscanFetcherChainConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "api_key",
            "apiKey",
            "max_requests_per_second",
            "maxRequestsPerSecond",
            "page_size",
            "pageSize",
            "url_prefix",
            "urlPrefix",
            "delay_num_blocks",
            "delayNumBlocks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            ApiKey,
            MaxRequestsPerSecond,
            PageSize,
            UrlPrefix,
            DelayNumBlocks,
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
                            "url" => Ok(GeneratedField::Url),
                            "apiKey" | "api_key" => Ok(GeneratedField::ApiKey),
                            "maxRequestsPerSecond" | "max_requests_per_second" => Ok(GeneratedField::MaxRequestsPerSecond),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "urlPrefix" | "url_prefix" => Ok(GeneratedField::UrlPrefix),
                            "delayNumBlocks" | "delay_num_blocks" => Ok(GeneratedField::DelayNumBlocks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EtherscanFetcherChainConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.loader.v1.EtherscanFetcherChainConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EtherscanFetcherChainConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut api_key__ = None;
                let mut max_requests_per_second__ = None;
                let mut page_size__ = None;
                let mut url_prefix__ = None;
                let mut delay_num_blocks__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = map.next_value()?;
                        }
                        GeneratedField::ApiKey => {
                            if api_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiKey"));
                            }
                            api_key__ = map.next_value()?;
                        }
                        GeneratedField::MaxRequestsPerSecond => {
                            if max_requests_per_second__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestsPerSecond"));
                            }
                            max_requests_per_second__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::UrlPrefix => {
                            if url_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urlPrefix"));
                            }
                            url_prefix__ = map.next_value()?;
                        }
                        GeneratedField::DelayNumBlocks => {
                            if delay_num_blocks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delayNumBlocks"));
                            }
                            delay_num_blocks__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(EtherscanFetcherChainConfig {
                    url: url__,
                    api_key: api_key__,
                    max_requests_per_second: max_requests_per_second__,
                    page_size: page_size__,
                    url_prefix: url_prefix__,
                    delay_num_blocks: delay_num_blocks__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.loader.v1.EtherscanFetcherChainConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EtherscanFetcherConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.concurrency.is_some() {
            len += 1;
        }
        if !self.chains.is_empty() {
            len += 1;
        }
        if self.skip_validation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.loader.v1.EtherscanFetcherConfig", len)?;
        if let Some(v) = self.concurrency.as_ref() {
            struct_ser.serialize_field("concurrency", v)?;
        }
        if !self.chains.is_empty() {
            struct_ser.serialize_field("chains", &self.chains)?;
        }
        if let Some(v) = self.skip_validation.as_ref() {
            struct_ser.serialize_field("skipValidation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EtherscanFetcherConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "concurrency",
            "chains",
            "skip_validation",
            "skipValidation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Concurrency,
            Chains,
            SkipValidation,
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
                            "concurrency" => Ok(GeneratedField::Concurrency),
                            "chains" => Ok(GeneratedField::Chains),
                            "skipValidation" | "skip_validation" => Ok(GeneratedField::SkipValidation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EtherscanFetcherConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.loader.v1.EtherscanFetcherConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EtherscanFetcherConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut concurrency__ = None;
                let mut chains__ = None;
                let mut skip_validation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Concurrency => {
                            if concurrency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("concurrency"));
                            }
                            concurrency__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Chains => {
                            if chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chains"));
                            }
                            chains__ = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u64>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::SkipValidation => {
                            if skip_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipValidation"));
                            }
                            skip_validation__ = map.next_value()?;
                        }
                    }
                }
                Ok(EtherscanFetcherConfig {
                    concurrency: concurrency__,
                    chains: chains__.unwrap_or_default(),
                    skip_validation: skip_validation__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.loader.v1.EtherscanFetcherConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FetcherConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.packer.is_some() {
            len += 1;
        }
        if self.indexer.is_some() {
            len += 1;
        }
        if self.etherscan.is_some() {
            len += 1;
        }
        if self.provider.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.loader.v1.FetcherConfig", len)?;
        if let Some(v) = self.packer.as_ref() {
            struct_ser.serialize_field("packer", v)?;
        }
        if let Some(v) = self.indexer.as_ref() {
            struct_ser.serialize_field("indexer", v)?;
        }
        if let Some(v) = self.etherscan.as_ref() {
            struct_ser.serialize_field("etherscan", v)?;
        }
        if let Some(v) = self.provider.as_ref() {
            struct_ser.serialize_field("provider", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FetcherConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "packer",
            "indexer",
            "etherscan",
            "provider",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Packer,
            Indexer,
            Etherscan,
            Provider,
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
                            "packer" => Ok(GeneratedField::Packer),
                            "indexer" => Ok(GeneratedField::Indexer),
                            "etherscan" => Ok(GeneratedField::Etherscan),
                            "provider" => Ok(GeneratedField::Provider),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FetcherConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.loader.v1.FetcherConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FetcherConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut packer__ = None;
                let mut indexer__ = None;
                let mut etherscan__ = None;
                let mut provider__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Packer => {
                            if packer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packer"));
                            }
                            packer__ = map.next_value()?;
                        }
                        GeneratedField::Indexer => {
                            if indexer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexer"));
                            }
                            indexer__ = map.next_value()?;
                        }
                        GeneratedField::Etherscan => {
                            if etherscan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("etherscan"));
                            }
                            etherscan__ = map.next_value()?;
                        }
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = map.next_value()?;
                        }
                    }
                }
                Ok(FetcherConfig {
                    packer: packer__,
                    indexer: indexer__,
                    etherscan: etherscan__,
                    provider: provider__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.loader.v1.FetcherConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FetcherType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FETCHER_TYPE_UNSPECIFIED",
            Self::Packer => "FETCHER_TYPE_PACKER",
            Self::Indexer => "FETCHER_TYPE_INDEXER",
            Self::Etherscan => "FETCHER_TYPE_ETHERSCAN",
            Self::Provider => "FETCHER_TYPE_PROVIDER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FetcherType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FETCHER_TYPE_UNSPECIFIED",
            "FETCHER_TYPE_PACKER",
            "FETCHER_TYPE_INDEXER",
            "FETCHER_TYPE_ETHERSCAN",
            "FETCHER_TYPE_PROVIDER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FetcherType;

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
                    .and_then(FetcherType::from_i32)
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
                    .and_then(FetcherType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FETCHER_TYPE_UNSPECIFIED" => Ok(FetcherType::Unspecified),
                    "FETCHER_TYPE_PACKER" => Ok(FetcherType::Packer),
                    "FETCHER_TYPE_INDEXER" => Ok(FetcherType::Indexer),
                    "FETCHER_TYPE_ETHERSCAN" => Ok(FetcherType::Etherscan),
                    "FETCHER_TYPE_PROVIDER" => Ok(FetcherType::Provider),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IndexerFetcherConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.url.is_some() {
            len += 1;
        }
        if self.timeout_ms.is_some() {
            len += 1;
        }
        if self.filter_size.is_some() {
            len += 1;
        }
        if self.skip_validation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.loader.v1.IndexerFetcherConfig", len)?;
        if let Some(v) = self.url.as_ref() {
            struct_ser.serialize_field("url", v)?;
        }
        if let Some(v) = self.timeout_ms.as_ref() {
            struct_ser.serialize_field("timeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.filter_size.as_ref() {
            struct_ser.serialize_field("filterSize", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.skip_validation.as_ref() {
            struct_ser.serialize_field("skipValidation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IndexerFetcherConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "timeout_ms",
            "timeoutMs",
            "filter_size",
            "filterSize",
            "skip_validation",
            "skipValidation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            TimeoutMs,
            FilterSize,
            SkipValidation,
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
                            "url" => Ok(GeneratedField::Url),
                            "timeoutMs" | "timeout_ms" => Ok(GeneratedField::TimeoutMs),
                            "filterSize" | "filter_size" => Ok(GeneratedField::FilterSize),
                            "skipValidation" | "skip_validation" => Ok(GeneratedField::SkipValidation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IndexerFetcherConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.loader.v1.IndexerFetcherConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IndexerFetcherConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut timeout_ms__ = None;
                let mut filter_size__ = None;
                let mut skip_validation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = map.next_value()?;
                        }
                        GeneratedField::TimeoutMs => {
                            if timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutMs"));
                            }
                            timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::FilterSize => {
                            if filter_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterSize"));
                            }
                            filter_size__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SkipValidation => {
                            if skip_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipValidation"));
                            }
                            skip_validation__ = map.next_value()?;
                        }
                    }
                }
                Ok(IndexerFetcherConfig {
                    url: url__,
                    timeout_ms: timeout_ms__,
                    filter_size: filter_size__,
                    skip_validation: skip_validation__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.loader.v1.IndexerFetcherConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoaderConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fetchers.is_empty() {
            len += 1;
        }
        if !self.validators.is_empty() {
            len += 1;
        }
        if self.mystiko_config_options.is_some() {
            len += 1;
        }
        if self.fetcher_config.is_some() {
            len += 1;
        }
        if self.validator_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.loader.v1.LoaderConfig", len)?;
        if !self.fetchers.is_empty() {
            let v: std::collections::HashMap<_, _> = self.fetchers.iter()
                .map(|(k, v)| {
                    let v = FetcherType::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    Ok((k, v))
                }).collect::<Result<_,_>>()?;
            struct_ser.serialize_field("fetchers", &v)?;
        }
        if !self.validators.is_empty() {
            let v: std::collections::HashMap<_, _> = self.validators.iter()
                .map(|(k, v)| {
                    let v = ValidatorType::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    Ok((k, v))
                }).collect::<Result<_,_>>()?;
            struct_ser.serialize_field("validators", &v)?;
        }
        if let Some(v) = self.mystiko_config_options.as_ref() {
            struct_ser.serialize_field("mystikoConfigOptions", v)?;
        }
        if let Some(v) = self.fetcher_config.as_ref() {
            struct_ser.serialize_field("fetcherConfig", v)?;
        }
        if let Some(v) = self.validator_config.as_ref() {
            struct_ser.serialize_field("validatorConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoaderConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fetchers",
            "validators",
            "mystiko_config_options",
            "mystikoConfigOptions",
            "fetcher_config",
            "fetcherConfig",
            "validator_config",
            "validatorConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fetchers,
            Validators,
            MystikoConfigOptions,
            FetcherConfig,
            ValidatorConfig,
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
                            "fetchers" => Ok(GeneratedField::Fetchers),
                            "validators" => Ok(GeneratedField::Validators),
                            "mystikoConfigOptions" | "mystiko_config_options" => Ok(GeneratedField::MystikoConfigOptions),
                            "fetcherConfig" | "fetcher_config" => Ok(GeneratedField::FetcherConfig),
                            "validatorConfig" | "validator_config" => Ok(GeneratedField::ValidatorConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoaderConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.loader.v1.LoaderConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LoaderConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fetchers__ = None;
                let mut validators__ = None;
                let mut mystiko_config_options__ = None;
                let mut fetcher_config__ = None;
                let mut validator_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Fetchers => {
                            if fetchers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fetchers"));
                            }
                            fetchers__ = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u32>, FetcherType>>()?
                                    .into_iter().map(|(k,v)| (k.0, v as i32)).collect()
                            );
                        }
                        GeneratedField::Validators => {
                            if validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validators"));
                            }
                            validators__ = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u32>, ValidatorType>>()?
                                    .into_iter().map(|(k,v)| (k.0, v as i32)).collect()
                            );
                        }
                        GeneratedField::MystikoConfigOptions => {
                            if mystiko_config_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mystikoConfigOptions"));
                            }
                            mystiko_config_options__ = map.next_value()?;
                        }
                        GeneratedField::FetcherConfig => {
                            if fetcher_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fetcherConfig"));
                            }
                            fetcher_config__ = map.next_value()?;
                        }
                        GeneratedField::ValidatorConfig => {
                            if validator_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorConfig"));
                            }
                            validator_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(LoaderConfig {
                    fetchers: fetchers__.unwrap_or_default(),
                    validators: validators__.unwrap_or_default(),
                    mystiko_config_options: mystiko_config_options__,
                    fetcher_config: fetcher_config__,
                    validator_config: validator_config__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.loader.v1.LoaderConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PackerFetcherConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.skip_validation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.loader.v1.PackerFetcherConfig", len)?;
        if let Some(v) = self.skip_validation.as_ref() {
            struct_ser.serialize_field("skipValidation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PackerFetcherConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "skip_validation",
            "skipValidation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SkipValidation,
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
                            "skipValidation" | "skip_validation" => Ok(GeneratedField::SkipValidation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PackerFetcherConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.loader.v1.PackerFetcherConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PackerFetcherConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut skip_validation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SkipValidation => {
                            if skip_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipValidation"));
                            }
                            skip_validation__ = map.next_value()?;
                        }
                    }
                }
                Ok(PackerFetcherConfig {
                    skip_validation: skip_validation__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.loader.v1.PackerFetcherConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProviderFetcherChainConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.urls.is_empty() {
            len += 1;
        }
        if self.delay_num_blocks.is_some() {
            len += 1;
        }
        if self.provider_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.loader.v1.ProviderFetcherChainConfig", len)?;
        if !self.urls.is_empty() {
            struct_ser.serialize_field("urls", &self.urls)?;
        }
        if let Some(v) = self.delay_num_blocks.as_ref() {
            struct_ser.serialize_field("delayNumBlocks", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.provider_type.as_ref() {
            let v = super::super::common::v1::ProviderType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("providerType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProviderFetcherChainConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "urls",
            "delay_num_blocks",
            "delayNumBlocks",
            "provider_type",
            "providerType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Urls,
            DelayNumBlocks,
            ProviderType,
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
                            "urls" => Ok(GeneratedField::Urls),
                            "delayNumBlocks" | "delay_num_blocks" => Ok(GeneratedField::DelayNumBlocks),
                            "providerType" | "provider_type" => Ok(GeneratedField::ProviderType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderFetcherChainConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.loader.v1.ProviderFetcherChainConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProviderFetcherChainConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut urls__ = None;
                let mut delay_num_blocks__ = None;
                let mut provider_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Urls => {
                            if urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urls"));
                            }
                            urls__ = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u32>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::DelayNumBlocks => {
                            if delay_num_blocks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delayNumBlocks"));
                            }
                            delay_num_blocks__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ProviderType => {
                            if provider_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerType"));
                            }
                            provider_type__ = map.next_value::<::std::option::Option<super::super::common::v1::ProviderType>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(ProviderFetcherChainConfig {
                    urls: urls__.unwrap_or_default(),
                    delay_num_blocks: delay_num_blocks__,
                    provider_type: provider_type__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.loader.v1.ProviderFetcherChainConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProviderFetcherConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.concurrency.is_some() {
            len += 1;
        }
        if self.timeout_ms.is_some() {
            len += 1;
        }
        if !self.chains.is_empty() {
            len += 1;
        }
        if self.skip_validation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.loader.v1.ProviderFetcherConfig", len)?;
        if let Some(v) = self.concurrency.as_ref() {
            struct_ser.serialize_field("concurrency", v)?;
        }
        if let Some(v) = self.timeout_ms.as_ref() {
            struct_ser.serialize_field("timeoutMs", ToString::to_string(&v).as_str())?;
        }
        if !self.chains.is_empty() {
            struct_ser.serialize_field("chains", &self.chains)?;
        }
        if let Some(v) = self.skip_validation.as_ref() {
            struct_ser.serialize_field("skipValidation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProviderFetcherConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "concurrency",
            "timeout_ms",
            "timeoutMs",
            "chains",
            "skip_validation",
            "skipValidation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Concurrency,
            TimeoutMs,
            Chains,
            SkipValidation,
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
                            "concurrency" => Ok(GeneratedField::Concurrency),
                            "timeoutMs" | "timeout_ms" => Ok(GeneratedField::TimeoutMs),
                            "chains" => Ok(GeneratedField::Chains),
                            "skipValidation" | "skip_validation" => Ok(GeneratedField::SkipValidation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderFetcherConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.loader.v1.ProviderFetcherConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProviderFetcherConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut concurrency__ = None;
                let mut timeout_ms__ = None;
                let mut chains__ = None;
                let mut skip_validation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Concurrency => {
                            if concurrency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("concurrency"));
                            }
                            concurrency__ = 
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
                        GeneratedField::Chains => {
                            if chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chains"));
                            }
                            chains__ = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u64>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::SkipValidation => {
                            if skip_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipValidation"));
                            }
                            skip_validation__ = map.next_value()?;
                        }
                    }
                }
                Ok(ProviderFetcherConfig {
                    concurrency: concurrency__,
                    timeout_ms: timeout_ms__,
                    chains: chains__.unwrap_or_default(),
                    skip_validation: skip_validation__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.loader.v1.ProviderFetcherConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleValidatorCheckerType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RULE_VALIDATOR_CHECKER_TYPE_UNSPECIFIED",
            Self::Integrity => "RULE_VALIDATOR_CHECKER_TYPE_INTEGRITY",
            Self::Sequence => "RULE_VALIDATOR_CHECKER_TYPE_SEQUENCE",
            Self::Counter => "RULE_VALIDATOR_CHECKER_TYPE_COUNTER",
            Self::Tree => "RULE_VALIDATOR_CHECKER_TYPE_TREE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RuleValidatorCheckerType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RULE_VALIDATOR_CHECKER_TYPE_UNSPECIFIED",
            "RULE_VALIDATOR_CHECKER_TYPE_INTEGRITY",
            "RULE_VALIDATOR_CHECKER_TYPE_SEQUENCE",
            "RULE_VALIDATOR_CHECKER_TYPE_COUNTER",
            "RULE_VALIDATOR_CHECKER_TYPE_TREE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuleValidatorCheckerType;

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
                    .and_then(RuleValidatorCheckerType::from_i32)
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
                    .and_then(RuleValidatorCheckerType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "RULE_VALIDATOR_CHECKER_TYPE_UNSPECIFIED" => Ok(RuleValidatorCheckerType::Unspecified),
                    "RULE_VALIDATOR_CHECKER_TYPE_INTEGRITY" => Ok(RuleValidatorCheckerType::Integrity),
                    "RULE_VALIDATOR_CHECKER_TYPE_SEQUENCE" => Ok(RuleValidatorCheckerType::Sequence),
                    "RULE_VALIDATOR_CHECKER_TYPE_COUNTER" => Ok(RuleValidatorCheckerType::Counter),
                    "RULE_VALIDATOR_CHECKER_TYPE_TREE" => Ok(RuleValidatorCheckerType::Tree),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RuleValidatorConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.checkers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.loader.v1.RuleValidatorConfig", len)?;
        if !self.checkers.is_empty() {
            let v: std::collections::HashMap<_, _> = self.checkers.iter()
                .map(|(k, v)| {
                    let v = RuleValidatorCheckerType::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    Ok((k, v))
                }).collect::<Result<_,_>>()?;
            struct_ser.serialize_field("checkers", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuleValidatorConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "checkers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Checkers,
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
                            "checkers" => Ok(GeneratedField::Checkers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuleValidatorConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.loader.v1.RuleValidatorConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RuleValidatorConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut checkers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Checkers => {
                            if checkers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkers"));
                            }
                            checkers__ = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u32>, RuleValidatorCheckerType>>()?
                                    .into_iter().map(|(k,v)| (k.0, v as i32)).collect()
                            );
                        }
                    }
                }
                Ok(RuleValidatorConfig {
                    checkers: checkers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.loader.v1.RuleValidatorConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidatorConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.loader.v1.ValidatorConfig", len)?;
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidatorConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
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
                            "rule" => Ok(GeneratedField::Rule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.loader.v1.ValidatorConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ValidatorConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map.next_value()?;
                        }
                    }
                }
                Ok(ValidatorConfig {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.loader.v1.ValidatorConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidatorType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VALIDATOR_TYPE_UNSPECIFIED",
            Self::Rule => "VALIDATOR_TYPE_RULE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ValidatorType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VALIDATOR_TYPE_UNSPECIFIED",
            "VALIDATOR_TYPE_RULE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorType;

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
                    .and_then(ValidatorType::from_i32)
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
                    .and_then(ValidatorType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "VALIDATOR_TYPE_UNSPECIFIED" => Ok(ValidatorType::Unspecified),
                    "VALIDATOR_TYPE_RULE" => Ok(ValidatorType::Rule),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
