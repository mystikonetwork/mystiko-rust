// @generated
impl serde::Serialize for AssetChainImportOptions {
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
        if !self.tx_hashes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.AssetChainImportOptions", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.tx_hashes.is_empty() {
            struct_ser.serialize_field("txHashes", &self.tx_hashes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetChainImportOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "tx_hashes",
            "txHashes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            TxHashes,
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
                            "txHashes" | "tx_hashes" => Ok(GeneratedField::TxHashes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetChainImportOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.AssetChainImportOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssetChainImportOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut tx_hashes__ = None;
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
                        GeneratedField::TxHashes => {
                            if tx_hashes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHashes"));
                            }
                            tx_hashes__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AssetChainImportOptions {
                    chain_id: chain_id__.unwrap_or_default(),
                    tx_hashes: tx_hashes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.AssetChainImportOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetChainImportResult {
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
        if self.imported_count != 0 {
            len += 1;
        }
        if self.found_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.AssetChainImportResult", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if self.imported_count != 0 {
            struct_ser.serialize_field("importedCount", &self.imported_count)?;
        }
        if self.found_count != 0 {
            struct_ser.serialize_field("foundCount", &self.found_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetChainImportResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "imported_count",
            "importedCount",
            "found_count",
            "foundCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ImportedCount,
            FoundCount,
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
                            "importedCount" | "imported_count" => Ok(GeneratedField::ImportedCount),
                            "foundCount" | "found_count" => Ok(GeneratedField::FoundCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetChainImportResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.AssetChainImportResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssetChainImportResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut imported_count__ = None;
                let mut found_count__ = None;
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
                        GeneratedField::ImportedCount => {
                            if imported_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("importedCount"));
                            }
                            imported_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FoundCount => {
                            if found_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("foundCount"));
                            }
                            found_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AssetChainImportResult {
                    chain_id: chain_id__.unwrap_or_default(),
                    imported_count: imported_count__.unwrap_or_default(),
                    found_count: found_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.AssetChainImportResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetImportOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.wallet_password.is_empty() {
            len += 1;
        }
        if !self.chains.is_empty() {
            len += 1;
        }
        if self.query_timeout_ms.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.AssetImportOptions", len)?;
        if !self.wallet_password.is_empty() {
            struct_ser.serialize_field("walletPassword", &self.wallet_password)?;
        }
        if !self.chains.is_empty() {
            struct_ser.serialize_field("chains", &self.chains)?;
        }
        if let Some(v) = self.query_timeout_ms.as_ref() {
            struct_ser.serialize_field("queryTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetImportOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wallet_password",
            "walletPassword",
            "chains",
            "query_timeout_ms",
            "queryTimeoutMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WalletPassword,
            Chains,
            QueryTimeoutMs,
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
                            "walletPassword" | "wallet_password" => Ok(GeneratedField::WalletPassword),
                            "chains" => Ok(GeneratedField::Chains),
                            "queryTimeoutMs" | "query_timeout_ms" => Ok(GeneratedField::QueryTimeoutMs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetImportOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.AssetImportOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssetImportOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet_password__ = None;
                let mut chains__ = None;
                let mut query_timeout_ms__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WalletPassword => {
                            if wallet_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletPassword"));
                            }
                            wallet_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::Chains => {
                            if chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chains"));
                            }
                            chains__ = Some(map.next_value()?);
                        }
                        GeneratedField::QueryTimeoutMs => {
                            if query_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryTimeoutMs"));
                            }
                            query_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(AssetImportOptions {
                    wallet_password: wallet_password__.unwrap_or_default(),
                    chains: chains__.unwrap_or_default(),
                    query_timeout_ms: query_timeout_ms__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.AssetImportOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetImportResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chains.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.AssetImportResult", len)?;
        if !self.chains.is_empty() {
            struct_ser.serialize_field("chains", &self.chains)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetImportResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chains",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Chains,
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
                            "chains" => Ok(GeneratedField::Chains),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetImportResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.AssetImportResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssetImportResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chains__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Chains => {
                            if chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chains"));
                            }
                            chains__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AssetImportResult {
                    chains: chains__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.AssetImportResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetsByBridge {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bridge_type != 0 {
            len += 1;
        }
        if !self.symbols.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.AssetsByBridge", len)?;
        if self.bridge_type != 0 {
            let v = super::super::super::common::v1::BridgeType::from_i32(self.bridge_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.bridge_type)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        if !self.symbols.is_empty() {
            struct_ser.serialize_field("symbols", &self.symbols)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetsByBridge {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bridge_type",
            "bridgeType",
            "symbols",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BridgeType,
            Symbols,
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
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
                            "symbols" => Ok(GeneratedField::Symbols),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetsByBridge;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.AssetsByBridge")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssetsByBridge, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bridge_type__ = None;
                let mut symbols__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = Some(map.next_value::<super::super::super::common::v1::BridgeType>()? as i32);
                        }
                        GeneratedField::Symbols => {
                            if symbols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbols"));
                            }
                            symbols__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AssetsByBridge {
                    bridge_type: bridge_type__.unwrap_or_default(),
                    symbols: symbols__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.AssetsByBridge", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetsByChain {
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
        if !self.bridges.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.AssetsByChain", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.bridges.is_empty() {
            struct_ser.serialize_field("bridges", &self.bridges)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetsByChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "bridges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            Bridges,
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
                            "bridges" => Ok(GeneratedField::Bridges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetsByChain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.AssetsByChain")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssetsByChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut bridges__ = None;
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
                        GeneratedField::Bridges => {
                            if bridges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridges"));
                            }
                            bridges__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AssetsByChain {
                    chain_id: chain_id__.unwrap_or_default(),
                    bridges: bridges__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.AssetsByChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetsBySymbol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if !self.versions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.AssetsBySymbol", len)?;
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if !self.versions.is_empty() {
            struct_ser.serialize_field("versions", &self.versions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetsBySymbol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_symbol",
            "assetSymbol",
            "versions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetSymbol,
            Versions,
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
                            "assetSymbol" | "asset_symbol" => Ok(GeneratedField::AssetSymbol),
                            "versions" => Ok(GeneratedField::Versions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetsBySymbol;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.AssetsBySymbol")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssetsBySymbol, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_symbol__ = None;
                let mut versions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AssetSymbol => {
                            if asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetSymbol"));
                            }
                            asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::Versions => {
                            if versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            versions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AssetsBySymbol {
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    versions: versions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.AssetsBySymbol", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetsByVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pool_version != 0 {
            len += 1;
        }
        if !self.pool_name.is_empty() {
            len += 1;
        }
        if self.balance != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.AssetsByVersion", len)?;
        if self.pool_version != 0 {
            struct_ser.serialize_field("poolVersion", &self.pool_version)?;
        }
        if !self.pool_name.is_empty() {
            struct_ser.serialize_field("poolName", &self.pool_name)?;
        }
        if self.balance != 0. {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetsByVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pool_version",
            "poolVersion",
            "pool_name",
            "poolName",
            "balance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PoolVersion,
            PoolName,
            Balance,
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
                            "poolVersion" | "pool_version" => Ok(GeneratedField::PoolVersion),
                            "poolName" | "pool_name" => Ok(GeneratedField::PoolName),
                            "balance" => Ok(GeneratedField::Balance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetsByVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.AssetsByVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssetsByVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pool_version__ = None;
                let mut pool_name__ = None;
                let mut balance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PoolVersion => {
                            if pool_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolVersion"));
                            }
                            pool_version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PoolName => {
                            if pool_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolName"));
                            }
                            pool_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AssetsByVersion {
                    pool_version: pool_version__.unwrap_or_default(),
                    pool_name: pool_name__.unwrap_or_default(),
                    balance: balance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.AssetsByVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetsOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shielded_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.AssetsOptions", len)?;
        if !self.shielded_addresses.is_empty() {
            struct_ser.serialize_field("shieldedAddresses", &self.shielded_addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetsOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shielded_addresses",
            "shieldedAddresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShieldedAddresses,
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
                            "shieldedAddresses" | "shielded_addresses" => Ok(GeneratedField::ShieldedAddresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetsOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.AssetsOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssetsOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shielded_addresses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ShieldedAddresses => {
                            if shielded_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddresses"));
                            }
                            shielded_addresses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AssetsOptions {
                    shielded_addresses: shielded_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.AssetsOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Balance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if self.unspent != 0. {
            len += 1;
        }
        if self.pending != 0. {
            len += 1;
        }
        if self.spent.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.Balance", len)?;
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.unspent != 0. {
            struct_ser.serialize_field("unspent", &self.unspent)?;
        }
        if self.pending != 0. {
            struct_ser.serialize_field("pending", &self.pending)?;
        }
        if let Some(v) = self.spent.as_ref() {
            struct_ser.serialize_field("spent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Balance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_symbol",
            "assetSymbol",
            "unspent",
            "pending",
            "spent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetSymbol,
            Unspent,
            Pending,
            Spent,
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
                            "assetSymbol" | "asset_symbol" => Ok(GeneratedField::AssetSymbol),
                            "unspent" => Ok(GeneratedField::Unspent),
                            "pending" => Ok(GeneratedField::Pending),
                            "spent" => Ok(GeneratedField::Spent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Balance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.Balance")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Balance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_symbol__ = None;
                let mut unspent__ = None;
                let mut pending__ = None;
                let mut spent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AssetSymbol => {
                            if asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetSymbol"));
                            }
                            asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::Unspent => {
                            if unspent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unspent"));
                            }
                            unspent__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pending => {
                            if pending__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pending"));
                            }
                            pending__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Spent => {
                            if spent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spent"));
                            }
                            spent__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Balance {
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    unspent: unspent__.unwrap_or_default(),
                    pending: pending__.unwrap_or_default(),
                    spent: spent__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.Balance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BalanceOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.with_spent.is_some() {
            len += 1;
        }
        if !self.shielded_addresses.is_empty() {
            len += 1;
        }
        if !self.chain_ids.is_empty() {
            len += 1;
        }
        if !self.asset_symbols.is_empty() {
            len += 1;
        }
        if !self.contract_addresses.is_empty() {
            len += 1;
        }
        if !self.bridge_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.BalanceOptions", len)?;
        if let Some(v) = self.with_spent.as_ref() {
            struct_ser.serialize_field("withSpent", v)?;
        }
        if !self.shielded_addresses.is_empty() {
            struct_ser.serialize_field("shieldedAddresses", &self.shielded_addresses)?;
        }
        if !self.chain_ids.is_empty() {
            struct_ser.serialize_field("chainIds", &self.chain_ids.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.asset_symbols.is_empty() {
            struct_ser.serialize_field("assetSymbols", &self.asset_symbols)?;
        }
        if !self.contract_addresses.is_empty() {
            struct_ser.serialize_field("contractAddresses", &self.contract_addresses)?;
        }
        if !self.bridge_types.is_empty() {
            let v = self.bridge_types.iter().cloned().map(|v| {
                super::super::super::common::v1::BridgeType::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("bridgeTypes", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BalanceOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "with_spent",
            "withSpent",
            "shielded_addresses",
            "shieldedAddresses",
            "chain_ids",
            "chainIds",
            "asset_symbols",
            "assetSymbols",
            "contract_addresses",
            "contractAddresses",
            "bridge_types",
            "bridgeTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithSpent,
            ShieldedAddresses,
            ChainIds,
            AssetSymbols,
            ContractAddresses,
            BridgeTypes,
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
                            "withSpent" | "with_spent" => Ok(GeneratedField::WithSpent),
                            "shieldedAddresses" | "shielded_addresses" => Ok(GeneratedField::ShieldedAddresses),
                            "chainIds" | "chain_ids" => Ok(GeneratedField::ChainIds),
                            "assetSymbols" | "asset_symbols" => Ok(GeneratedField::AssetSymbols),
                            "contractAddresses" | "contract_addresses" => Ok(GeneratedField::ContractAddresses),
                            "bridgeTypes" | "bridge_types" => Ok(GeneratedField::BridgeTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BalanceOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.BalanceOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BalanceOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut with_spent__ = None;
                let mut shielded_addresses__ = None;
                let mut chain_ids__ = None;
                let mut asset_symbols__ = None;
                let mut contract_addresses__ = None;
                let mut bridge_types__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WithSpent => {
                            if with_spent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withSpent"));
                            }
                            with_spent__ = map.next_value()?;
                        }
                        GeneratedField::ShieldedAddresses => {
                            if shielded_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddresses"));
                            }
                            shielded_addresses__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainIds => {
                            if chain_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainIds"));
                            }
                            chain_ids__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::AssetSymbols => {
                            if asset_symbols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetSymbols"));
                            }
                            asset_symbols__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContractAddresses => {
                            if contract_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddresses"));
                            }
                            contract_addresses__ = Some(map.next_value()?);
                        }
                        GeneratedField::BridgeTypes => {
                            if bridge_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeTypes"));
                            }
                            bridge_types__ = Some(map.next_value::<Vec<super::super::super::common::v1::BridgeType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(BalanceOptions {
                    with_spent: with_spent__,
                    shielded_addresses: shielded_addresses__.unwrap_or_default(),
                    chain_ids: chain_ids__.unwrap_or_default(),
                    asset_symbols: asset_symbols__.unwrap_or_default(),
                    contract_addresses: contract_addresses__.unwrap_or_default(),
                    bridge_types: bridge_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.BalanceOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BalanceResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.balances.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.BalanceResult", len)?;
        if !self.balances.is_empty() {
            struct_ser.serialize_field("balances", &self.balances)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BalanceResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "balances",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balances,
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
                            "balances" => Ok(GeneratedField::Balances),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BalanceResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.BalanceResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BalanceResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balances__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Balances => {
                            if balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balances"));
                            }
                            balances__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BalanceResult {
                    balances: balances__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.BalanceResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.ResetResult", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetResult {
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
            type Value = ResetResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.ResetResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResetResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ResetResult {
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.ResetResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScanOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.wallet_password.is_empty() {
            len += 1;
        }
        if self.batch_size.is_some() {
            len += 1;
        }
        if self.concurrency.is_some() {
            len += 1;
        }
        if !self.shielded_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.ScanOptions", len)?;
        if !self.wallet_password.is_empty() {
            struct_ser.serialize_field("walletPassword", &self.wallet_password)?;
        }
        if let Some(v) = self.batch_size.as_ref() {
            struct_ser.serialize_field("batchSize", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.concurrency.as_ref() {
            struct_ser.serialize_field("concurrency", v)?;
        }
        if !self.shielded_addresses.is_empty() {
            struct_ser.serialize_field("shieldedAddresses", &self.shielded_addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScanOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wallet_password",
            "walletPassword",
            "batch_size",
            "batchSize",
            "concurrency",
            "shielded_addresses",
            "shieldedAddresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WalletPassword,
            BatchSize,
            Concurrency,
            ShieldedAddresses,
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
                            "walletPassword" | "wallet_password" => Ok(GeneratedField::WalletPassword),
                            "batchSize" | "batch_size" => Ok(GeneratedField::BatchSize),
                            "concurrency" => Ok(GeneratedField::Concurrency),
                            "shieldedAddresses" | "shielded_addresses" => Ok(GeneratedField::ShieldedAddresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScanOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.ScanOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScanOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet_password__ = None;
                let mut batch_size__ = None;
                let mut concurrency__ = None;
                let mut shielded_addresses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WalletPassword => {
                            if wallet_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletPassword"));
                            }
                            wallet_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchSize => {
                            if batch_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchSize"));
                            }
                            batch_size__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Concurrency => {
                            if concurrency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("concurrency"));
                            }
                            concurrency__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ShieldedAddresses => {
                            if shielded_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddresses"));
                            }
                            shielded_addresses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ScanOptions {
                    wallet_password: wallet_password__.unwrap_or_default(),
                    batch_size: batch_size__,
                    concurrency: concurrency__,
                    shielded_addresses: shielded_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.ScanOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScanResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total_count != 0 {
            len += 1;
        }
        if self.owned_count != 0 {
            len += 1;
        }
        if !self.scanned_shielded_addresses.is_empty() {
            len += 1;
        }
        if self.to_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.ScanResult", len)?;
        if self.total_count != 0 {
            struct_ser.serialize_field("totalCount", ToString::to_string(&self.total_count).as_str())?;
        }
        if self.owned_count != 0 {
            struct_ser.serialize_field("ownedCount", ToString::to_string(&self.owned_count).as_str())?;
        }
        if !self.scanned_shielded_addresses.is_empty() {
            struct_ser.serialize_field("scannedShieldedAddresses", &self.scanned_shielded_addresses)?;
        }
        if let Some(v) = self.to_id.as_ref() {
            struct_ser.serialize_field("toId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScanResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total_count",
            "totalCount",
            "owned_count",
            "ownedCount",
            "scanned_shielded_addresses",
            "scannedShieldedAddresses",
            "to_id",
            "toId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalCount,
            OwnedCount,
            ScannedShieldedAddresses,
            ToId,
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
                            "totalCount" | "total_count" => Ok(GeneratedField::TotalCount),
                            "ownedCount" | "owned_count" => Ok(GeneratedField::OwnedCount),
                            "scannedShieldedAddresses" | "scanned_shielded_addresses" => Ok(GeneratedField::ScannedShieldedAddresses),
                            "toId" | "to_id" => Ok(GeneratedField::ToId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScanResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.ScanResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScanResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total_count__ = None;
                let mut owned_count__ = None;
                let mut scanned_shielded_addresses__ = None;
                let mut to_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TotalCount => {
                            if total_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalCount"));
                            }
                            total_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OwnedCount => {
                            if owned_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownedCount"));
                            }
                            owned_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ScannedShieldedAddresses => {
                            if scanned_shielded_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scannedShieldedAddresses"));
                            }
                            scanned_shielded_addresses__ = Some(map.next_value()?);
                        }
                        GeneratedField::ToId => {
                            if to_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toId"));
                            }
                            to_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(ScanResult {
                    total_count: total_count__.unwrap_or_default(),
                    owned_count: owned_count__.unwrap_or_default(),
                    scanned_shielded_addresses: scanned_shielded_addresses__.unwrap_or_default(),
                    to_id: to_id__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.ScanResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScannerResetOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reset_to_id.is_some() {
            len += 1;
        }
        if !self.shielded_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.ScannerResetOptions", len)?;
        if let Some(v) = self.reset_to_id.as_ref() {
            struct_ser.serialize_field("resetToId", v)?;
        }
        if !self.shielded_addresses.is_empty() {
            struct_ser.serialize_field("shieldedAddresses", &self.shielded_addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScannerResetOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reset_to_id",
            "resetToId",
            "shielded_addresses",
            "shieldedAddresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResetToId,
            ShieldedAddresses,
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
                            "resetToId" | "reset_to_id" => Ok(GeneratedField::ResetToId),
                            "shieldedAddresses" | "shielded_addresses" => Ok(GeneratedField::ShieldedAddresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScannerResetOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.ScannerResetOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScannerResetOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reset_to_id__ = None;
                let mut shielded_addresses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResetToId => {
                            if reset_to_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resetToId"));
                            }
                            reset_to_id__ = map.next_value()?;
                        }
                        GeneratedField::ShieldedAddresses => {
                            if shielded_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddresses"));
                            }
                            shielded_addresses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ScannerResetOptions {
                    reset_to_id: reset_to_id__,
                    shielded_addresses: shielded_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.ScannerResetOptions", FIELDS, GeneratedVisitor)
    }
}
