// @generated
impl serde::Serialize for AssetConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.asset_type != 0 {
            len += 1;
        }
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if self.asset_decimals != 0 {
            len += 1;
        }
        if self.asset_address.is_some() {
            len += 1;
        }
        if !self.recommended_amounts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.v1.AssetConfig", len)?;
        if self.asset_type != 0 {
            let v = super::super::common::v1::AssetType::from_i32(self.asset_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.asset_type)))?;
            struct_ser.serialize_field("assetType", &v)?;
        }
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.asset_decimals != 0 {
            struct_ser.serialize_field("assetDecimals", &self.asset_decimals)?;
        }
        if let Some(v) = self.asset_address.as_ref() {
            struct_ser.serialize_field("assetAddress", v)?;
        }
        if !self.recommended_amounts.is_empty() {
            struct_ser.serialize_field("recommendedAmounts", &self.recommended_amounts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_type",
            "assetType",
            "asset_symbol",
            "assetSymbol",
            "asset_decimals",
            "assetDecimals",
            "asset_address",
            "assetAddress",
            "recommended_amounts",
            "recommendedAmounts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetType,
            AssetSymbol,
            AssetDecimals,
            AssetAddress,
            RecommendedAmounts,
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
                            "assetType" | "asset_type" => Ok(GeneratedField::AssetType),
                            "assetSymbol" | "asset_symbol" => Ok(GeneratedField::AssetSymbol),
                            "assetDecimals" | "asset_decimals" => Ok(GeneratedField::AssetDecimals),
                            "assetAddress" | "asset_address" => Ok(GeneratedField::AssetAddress),
                            "recommendedAmounts" | "recommended_amounts" => Ok(GeneratedField::RecommendedAmounts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.v1.AssetConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssetConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_type__ = None;
                let mut asset_symbol__ = None;
                let mut asset_decimals__ = None;
                let mut asset_address__ = None;
                let mut recommended_amounts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AssetType => {
                            if asset_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetType"));
                            }
                            asset_type__ = Some(map.next_value::<super::super::common::v1::AssetType>()? as i32);
                        }
                        GeneratedField::AssetSymbol => {
                            if asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetSymbol"));
                            }
                            asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::AssetDecimals => {
                            if asset_decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetDecimals"));
                            }
                            asset_decimals__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AssetAddress => {
                            if asset_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetAddress"));
                            }
                            asset_address__ = map.next_value()?;
                        }
                        GeneratedField::RecommendedAmounts => {
                            if recommended_amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recommendedAmounts"));
                            }
                            recommended_amounts__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AssetConfig {
                    asset_type: asset_type__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    asset_decimals: asset_decimals__.unwrap_or_default(),
                    asset_address: asset_address__,
                    recommended_amounts: recommended_amounts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.v1.AssetConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChainConfig {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if self.asset_decimals != 0 {
            len += 1;
        }
        if !self.explorer_url.is_empty() {
            len += 1;
        }
        if !self.explorer_api_url.is_empty() {
            len += 1;
        }
        if !self.explorer_prefix.is_empty() {
            len += 1;
        }
        if self.provider_quorum_percentage != 0 {
            len += 1;
        }
        if !self.signer_endpoint.is_empty() {
            len += 1;
        }
        if self.event_delay_blocks != 0 {
            len += 1;
        }
        if self.event_filter_size != 0 {
            len += 1;
        }
        if self.indexer_filter_size != 0 {
            len += 1;
        }
        if self.main_asset_config.is_some() {
            len += 1;
        }
        if self.provider_type != 0 {
            len += 1;
        }
        if !self.asset_configs.is_empty() {
            len += 1;
        }
        if !self.deposit_contract_configs.is_empty() {
            len += 1;
        }
        if !self.pool_contract_configs.is_empty() {
            len += 1;
        }
        if !self.recommended_amounts.is_empty() {
            len += 1;
        }
        if !self.provider_configs.is_empty() {
            len += 1;
        }
        if !self.granularities.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.v1.ChainConfig", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.asset_decimals != 0 {
            struct_ser.serialize_field("assetDecimals", &self.asset_decimals)?;
        }
        if !self.explorer_url.is_empty() {
            struct_ser.serialize_field("explorerUrl", &self.explorer_url)?;
        }
        if !self.explorer_api_url.is_empty() {
            struct_ser.serialize_field("explorerApiUrl", &self.explorer_api_url)?;
        }
        if !self.explorer_prefix.is_empty() {
            struct_ser.serialize_field("explorerPrefix", &self.explorer_prefix)?;
        }
        if self.provider_quorum_percentage != 0 {
            struct_ser.serialize_field("providerQuorumPercentage", &self.provider_quorum_percentage)?;
        }
        if !self.signer_endpoint.is_empty() {
            struct_ser.serialize_field("signerEndpoint", &self.signer_endpoint)?;
        }
        if self.event_delay_blocks != 0 {
            struct_ser.serialize_field("eventDelayBlocks", ToString::to_string(&self.event_delay_blocks).as_str())?;
        }
        if self.event_filter_size != 0 {
            struct_ser.serialize_field("eventFilterSize", ToString::to_string(&self.event_filter_size).as_str())?;
        }
        if self.indexer_filter_size != 0 {
            struct_ser.serialize_field("indexerFilterSize", ToString::to_string(&self.indexer_filter_size).as_str())?;
        }
        if let Some(v) = self.main_asset_config.as_ref() {
            struct_ser.serialize_field("mainAssetConfig", v)?;
        }
        if self.provider_type != 0 {
            let v = super::super::common::v1::ProviderType::from_i32(self.provider_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.provider_type)))?;
            struct_ser.serialize_field("providerType", &v)?;
        }
        if !self.asset_configs.is_empty() {
            struct_ser.serialize_field("assetConfigs", &self.asset_configs)?;
        }
        if !self.deposit_contract_configs.is_empty() {
            struct_ser.serialize_field("depositContractConfigs", &self.deposit_contract_configs)?;
        }
        if !self.pool_contract_configs.is_empty() {
            struct_ser.serialize_field("poolContractConfigs", &self.pool_contract_configs)?;
        }
        if !self.recommended_amounts.is_empty() {
            struct_ser.serialize_field("recommendedAmounts", &self.recommended_amounts)?;
        }
        if !self.provider_configs.is_empty() {
            struct_ser.serialize_field("providerConfigs", &self.provider_configs)?;
        }
        if !self.granularities.is_empty() {
            struct_ser.serialize_field("granularities", &self.granularities.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "name",
            "asset_symbol",
            "assetSymbol",
            "asset_decimals",
            "assetDecimals",
            "explorer_url",
            "explorerUrl",
            "explorer_api_url",
            "explorerApiUrl",
            "explorer_prefix",
            "explorerPrefix",
            "provider_quorum_percentage",
            "providerQuorumPercentage",
            "signer_endpoint",
            "signerEndpoint",
            "event_delay_blocks",
            "eventDelayBlocks",
            "event_filter_size",
            "eventFilterSize",
            "indexer_filter_size",
            "indexerFilterSize",
            "main_asset_config",
            "mainAssetConfig",
            "provider_type",
            "providerType",
            "asset_configs",
            "assetConfigs",
            "deposit_contract_configs",
            "depositContractConfigs",
            "pool_contract_configs",
            "poolContractConfigs",
            "recommended_amounts",
            "recommendedAmounts",
            "provider_configs",
            "providerConfigs",
            "granularities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            Name,
            AssetSymbol,
            AssetDecimals,
            ExplorerUrl,
            ExplorerApiUrl,
            ExplorerPrefix,
            ProviderQuorumPercentage,
            SignerEndpoint,
            EventDelayBlocks,
            EventFilterSize,
            IndexerFilterSize,
            MainAssetConfig,
            ProviderType,
            AssetConfigs,
            DepositContractConfigs,
            PoolContractConfigs,
            RecommendedAmounts,
            ProviderConfigs,
            Granularities,
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
                            "name" => Ok(GeneratedField::Name),
                            "assetSymbol" | "asset_symbol" => Ok(GeneratedField::AssetSymbol),
                            "assetDecimals" | "asset_decimals" => Ok(GeneratedField::AssetDecimals),
                            "explorerUrl" | "explorer_url" => Ok(GeneratedField::ExplorerUrl),
                            "explorerApiUrl" | "explorer_api_url" => Ok(GeneratedField::ExplorerApiUrl),
                            "explorerPrefix" | "explorer_prefix" => Ok(GeneratedField::ExplorerPrefix),
                            "providerQuorumPercentage" | "provider_quorum_percentage" => Ok(GeneratedField::ProviderQuorumPercentage),
                            "signerEndpoint" | "signer_endpoint" => Ok(GeneratedField::SignerEndpoint),
                            "eventDelayBlocks" | "event_delay_blocks" => Ok(GeneratedField::EventDelayBlocks),
                            "eventFilterSize" | "event_filter_size" => Ok(GeneratedField::EventFilterSize),
                            "indexerFilterSize" | "indexer_filter_size" => Ok(GeneratedField::IndexerFilterSize),
                            "mainAssetConfig" | "main_asset_config" => Ok(GeneratedField::MainAssetConfig),
                            "providerType" | "provider_type" => Ok(GeneratedField::ProviderType),
                            "assetConfigs" | "asset_configs" => Ok(GeneratedField::AssetConfigs),
                            "depositContractConfigs" | "deposit_contract_configs" => Ok(GeneratedField::DepositContractConfigs),
                            "poolContractConfigs" | "pool_contract_configs" => Ok(GeneratedField::PoolContractConfigs),
                            "recommendedAmounts" | "recommended_amounts" => Ok(GeneratedField::RecommendedAmounts),
                            "providerConfigs" | "provider_configs" => Ok(GeneratedField::ProviderConfigs),
                            "granularities" => Ok(GeneratedField::Granularities),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.v1.ChainConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChainConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut name__ = None;
                let mut asset_symbol__ = None;
                let mut asset_decimals__ = None;
                let mut explorer_url__ = None;
                let mut explorer_api_url__ = None;
                let mut explorer_prefix__ = None;
                let mut provider_quorum_percentage__ = None;
                let mut signer_endpoint__ = None;
                let mut event_delay_blocks__ = None;
                let mut event_filter_size__ = None;
                let mut indexer_filter_size__ = None;
                let mut main_asset_config__ = None;
                let mut provider_type__ = None;
                let mut asset_configs__ = None;
                let mut deposit_contract_configs__ = None;
                let mut pool_contract_configs__ = None;
                let mut recommended_amounts__ = None;
                let mut provider_configs__ = None;
                let mut granularities__ = None;
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
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::AssetSymbol => {
                            if asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetSymbol"));
                            }
                            asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::AssetDecimals => {
                            if asset_decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetDecimals"));
                            }
                            asset_decimals__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExplorerUrl => {
                            if explorer_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("explorerUrl"));
                            }
                            explorer_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExplorerApiUrl => {
                            if explorer_api_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("explorerApiUrl"));
                            }
                            explorer_api_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExplorerPrefix => {
                            if explorer_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("explorerPrefix"));
                            }
                            explorer_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProviderQuorumPercentage => {
                            if provider_quorum_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerQuorumPercentage"));
                            }
                            provider_quorum_percentage__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SignerEndpoint => {
                            if signer_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signerEndpoint"));
                            }
                            signer_endpoint__ = Some(map.next_value()?);
                        }
                        GeneratedField::EventDelayBlocks => {
                            if event_delay_blocks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventDelayBlocks"));
                            }
                            event_delay_blocks__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EventFilterSize => {
                            if event_filter_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventFilterSize"));
                            }
                            event_filter_size__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IndexerFilterSize => {
                            if indexer_filter_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexerFilterSize"));
                            }
                            indexer_filter_size__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MainAssetConfig => {
                            if main_asset_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mainAssetConfig"));
                            }
                            main_asset_config__ = map.next_value()?;
                        }
                        GeneratedField::ProviderType => {
                            if provider_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerType"));
                            }
                            provider_type__ = Some(map.next_value::<super::super::common::v1::ProviderType>()? as i32);
                        }
                        GeneratedField::AssetConfigs => {
                            if asset_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetConfigs"));
                            }
                            asset_configs__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::DepositContractConfigs => {
                            if deposit_contract_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositContractConfigs"));
                            }
                            deposit_contract_configs__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::PoolContractConfigs => {
                            if pool_contract_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolContractConfigs"));
                            }
                            pool_contract_configs__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::RecommendedAmounts => {
                            if recommended_amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recommendedAmounts"));
                            }
                            recommended_amounts__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProviderConfigs => {
                            if provider_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerConfigs"));
                            }
                            provider_configs__ = Some(map.next_value()?);
                        }
                        GeneratedField::Granularities => {
                            if granularities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("granularities"));
                            }
                            granularities__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(ChainConfig {
                    chain_id: chain_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    asset_decimals: asset_decimals__.unwrap_or_default(),
                    explorer_url: explorer_url__.unwrap_or_default(),
                    explorer_api_url: explorer_api_url__.unwrap_or_default(),
                    explorer_prefix: explorer_prefix__.unwrap_or_default(),
                    provider_quorum_percentage: provider_quorum_percentage__.unwrap_or_default(),
                    signer_endpoint: signer_endpoint__.unwrap_or_default(),
                    event_delay_blocks: event_delay_blocks__.unwrap_or_default(),
                    event_filter_size: event_filter_size__.unwrap_or_default(),
                    indexer_filter_size: indexer_filter_size__.unwrap_or_default(),
                    main_asset_config: main_asset_config__,
                    provider_type: provider_type__.unwrap_or_default(),
                    asset_configs: asset_configs__.unwrap_or_default(),
                    deposit_contract_configs: deposit_contract_configs__.unwrap_or_default(),
                    pool_contract_configs: pool_contract_configs__.unwrap_or_default(),
                    recommended_amounts: recommended_amounts__.unwrap_or_default(),
                    provider_configs: provider_configs__.unwrap_or_default(),
                    granularities: granularities__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.v1.ChainConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CircuitConfig {
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
        if self.circuit_type != 0 {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        if !self.program_file.is_empty() {
            len += 1;
        }
        if !self.abi_file.is_empty() {
            len += 1;
        }
        if !self.proving_key_file.is_empty() {
            len += 1;
        }
        if !self.verifying_key_file.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.v1.CircuitConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.circuit_type != 0 {
            let v = super::super::common::v1::CircuitType::from_i32(self.circuit_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.circuit_type)))?;
            struct_ser.serialize_field("circuitType", &v)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        if !self.program_file.is_empty() {
            struct_ser.serialize_field("programFile", &self.program_file)?;
        }
        if !self.abi_file.is_empty() {
            struct_ser.serialize_field("abiFile", &self.abi_file)?;
        }
        if !self.proving_key_file.is_empty() {
            struct_ser.serialize_field("provingKeyFile", &self.proving_key_file)?;
        }
        if !self.verifying_key_file.is_empty() {
            struct_ser.serialize_field("verifyingKeyFile", &self.verifying_key_file)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CircuitConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "circuit_type",
            "circuitType",
            "is_default",
            "isDefault",
            "program_file",
            "programFile",
            "abi_file",
            "abiFile",
            "proving_key_file",
            "provingKeyFile",
            "verifying_key_file",
            "verifyingKeyFile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CircuitType,
            IsDefault,
            ProgramFile,
            AbiFile,
            ProvingKeyFile,
            VerifyingKeyFile,
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
                            "circuitType" | "circuit_type" => Ok(GeneratedField::CircuitType),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            "programFile" | "program_file" => Ok(GeneratedField::ProgramFile),
                            "abiFile" | "abi_file" => Ok(GeneratedField::AbiFile),
                            "provingKeyFile" | "proving_key_file" => Ok(GeneratedField::ProvingKeyFile),
                            "verifyingKeyFile" | "verifying_key_file" => Ok(GeneratedField::VerifyingKeyFile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CircuitConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.v1.CircuitConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CircuitConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut circuit_type__ = None;
                let mut is_default__ = None;
                let mut program_file__ = None;
                let mut abi_file__ = None;
                let mut proving_key_file__ = None;
                let mut verifying_key_file__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::CircuitType => {
                            if circuit_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("circuitType"));
                            }
                            circuit_type__ = Some(map.next_value::<super::super::common::v1::CircuitType>()? as i32);
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProgramFile => {
                            if program_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("programFile"));
                            }
                            program_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::AbiFile => {
                            if abi_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abiFile"));
                            }
                            abi_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProvingKeyFile => {
                            if proving_key_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provingKeyFile"));
                            }
                            proving_key_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::VerifyingKeyFile => {
                            if verifying_key_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyingKeyFile"));
                            }
                            verifying_key_file__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CircuitConfig {
                    name: name__.unwrap_or_default(),
                    circuit_type: circuit_type__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
                    program_file: program_file__.unwrap_or_default(),
                    abi_file: abi_file__.unwrap_or_default(),
                    proving_key_file: proving_key_file__.unwrap_or_default(),
                    verifying_key_file: verifying_key_file__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.v1.CircuitConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IndexerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        if self.timeout_ms != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.v1.IndexerConfig", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if self.timeout_ms != 0 {
            struct_ser.serialize_field("timeoutMs", ToString::to_string(&self.timeout_ms).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IndexerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "timeout_ms",
            "timeoutMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            TimeoutMs,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IndexerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.v1.IndexerConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IndexerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut timeout_ms__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimeoutMs => {
                            if timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutMs"));
                            }
                            timeout_ms__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(IndexerConfig {
                    url: url__.unwrap_or_default(),
                    timeout_ms: timeout_ms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.v1.IndexerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MystikoConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.chain_configs.is_empty() {
            len += 1;
        }
        if !self.bridge_configs.is_empty() {
            len += 1;
        }
        if self.git_revision.is_some() {
            len += 1;
        }
        if self.indexer_config.is_some() {
            len += 1;
        }
        if self.packer_config.is_some() {
            len += 1;
        }
        if !self.country_blacklist.is_empty() {
            len += 1;
        }
        if !self.circuit_configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.v1.MystikoConfig", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.chain_configs.is_empty() {
            struct_ser.serialize_field("chainConfigs", &self.chain_configs)?;
        }
        if !self.bridge_configs.is_empty() {
            struct_ser.serialize_field("bridgeConfigs", &self.bridge_configs)?;
        }
        if let Some(v) = self.git_revision.as_ref() {
            struct_ser.serialize_field("gitRevision", v)?;
        }
        if let Some(v) = self.indexer_config.as_ref() {
            struct_ser.serialize_field("indexerConfig", v)?;
        }
        if let Some(v) = self.packer_config.as_ref() {
            struct_ser.serialize_field("packerConfig", v)?;
        }
        if !self.country_blacklist.is_empty() {
            struct_ser.serialize_field("countryBlacklist", &self.country_blacklist)?;
        }
        if !self.circuit_configs.is_empty() {
            struct_ser.serialize_field("circuitConfigs", &self.circuit_configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MystikoConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "chain_configs",
            "chainConfigs",
            "bridge_configs",
            "bridgeConfigs",
            "git_revision",
            "gitRevision",
            "indexer_config",
            "indexerConfig",
            "packer_config",
            "packerConfig",
            "country_blacklist",
            "countryBlacklist",
            "circuit_configs",
            "circuitConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            ChainConfigs,
            BridgeConfigs,
            GitRevision,
            IndexerConfig,
            PackerConfig,
            CountryBlacklist,
            CircuitConfigs,
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
                            "version" => Ok(GeneratedField::Version),
                            "chainConfigs" | "chain_configs" => Ok(GeneratedField::ChainConfigs),
                            "bridgeConfigs" | "bridge_configs" => Ok(GeneratedField::BridgeConfigs),
                            "gitRevision" | "git_revision" => Ok(GeneratedField::GitRevision),
                            "indexerConfig" | "indexer_config" => Ok(GeneratedField::IndexerConfig),
                            "packerConfig" | "packer_config" => Ok(GeneratedField::PackerConfig),
                            "countryBlacklist" | "country_blacklist" => Ok(GeneratedField::CountryBlacklist),
                            "circuitConfigs" | "circuit_configs" => Ok(GeneratedField::CircuitConfigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MystikoConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.v1.MystikoConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MystikoConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut chain_configs__ = None;
                let mut bridge_configs__ = None;
                let mut git_revision__ = None;
                let mut indexer_config__ = None;
                let mut packer_config__ = None;
                let mut country_blacklist__ = None;
                let mut circuit_configs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainConfigs => {
                            if chain_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainConfigs"));
                            }
                            chain_configs__ = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u64>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::BridgeConfigs => {
                            if bridge_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeConfigs"));
                            }
                            bridge_configs__ = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<i32>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::GitRevision => {
                            if git_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gitRevision"));
                            }
                            git_revision__ = map.next_value()?;
                        }
                        GeneratedField::IndexerConfig => {
                            if indexer_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexerConfig"));
                            }
                            indexer_config__ = map.next_value()?;
                        }
                        GeneratedField::PackerConfig => {
                            if packer_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packerConfig"));
                            }
                            packer_config__ = map.next_value()?;
                        }
                        GeneratedField::CountryBlacklist => {
                            if country_blacklist__.is_some() {
                                return Err(serde::de::Error::duplicate_field("countryBlacklist"));
                            }
                            country_blacklist__ = Some(map.next_value()?);
                        }
                        GeneratedField::CircuitConfigs => {
                            if circuit_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("circuitConfigs"));
                            }
                            circuit_configs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MystikoConfig {
                    version: version__.unwrap_or_default(),
                    chain_configs: chain_configs__.unwrap_or_default(),
                    bridge_configs: bridge_configs__.unwrap_or_default(),
                    git_revision: git_revision__,
                    indexer_config: indexer_config__,
                    packer_config: packer_config__,
                    country_blacklist: country_blacklist__.unwrap_or_default(),
                    circuit_configs: circuit_configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.v1.MystikoConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PackerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        if self.client_timeout_ms != 0 {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if self.checksum != 0 {
            len += 1;
        }
        if self.compression != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.v1.PackerConfig", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if self.client_timeout_ms != 0 {
            struct_ser.serialize_field("clientTimeoutMs", ToString::to_string(&self.client_timeout_ms).as_str())?;
        }
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.checksum != 0 {
            let v = super::super::core::v1::PackerChecksum::from_i32(self.checksum)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.checksum)))?;
            struct_ser.serialize_field("checksum", &v)?;
        }
        if self.compression != 0 {
            let v = super::super::core::v1::PackerCompression::from_i32(self.compression)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.compression)))?;
            struct_ser.serialize_field("compression", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PackerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "client_timeout_ms",
            "clientTimeoutMs",
            "version",
            "checksum",
            "compression",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            ClientTimeoutMs,
            Version,
            Checksum,
            Compression,
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
                            "clientTimeoutMs" | "client_timeout_ms" => Ok(GeneratedField::ClientTimeoutMs),
                            "version" => Ok(GeneratedField::Version),
                            "checksum" => Ok(GeneratedField::Checksum),
                            "compression" => Ok(GeneratedField::Compression),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PackerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.v1.PackerConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PackerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut client_timeout_ms__ = None;
                let mut version__ = None;
                let mut checksum__ = None;
                let mut compression__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClientTimeoutMs => {
                            if client_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientTimeoutMs"));
                            }
                            client_timeout_ms__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Checksum => {
                            if checksum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checksum"));
                            }
                            checksum__ = Some(map.next_value::<super::super::core::v1::PackerChecksum>()? as i32);
                        }
                        GeneratedField::Compression => {
                            if compression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compression"));
                            }
                            compression__ = Some(map.next_value::<super::super::core::v1::PackerCompression>()? as i32);
                        }
                    }
                }
                Ok(PackerConfig {
                    url: url__.unwrap_or_default(),
                    client_timeout_ms: client_timeout_ms__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    checksum: checksum__.unwrap_or_default(),
                    compression: compression__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.v1.PackerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProviderConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        if self.timeout_ms != 0 {
            len += 1;
        }
        if self.max_try_count != 0 {
            len += 1;
        }
        if self.quorum_weight != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.v1.ProviderConfig", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if self.timeout_ms != 0 {
            struct_ser.serialize_field("timeoutMs", &self.timeout_ms)?;
        }
        if self.max_try_count != 0 {
            struct_ser.serialize_field("maxTryCount", &self.max_try_count)?;
        }
        if self.quorum_weight != 0 {
            struct_ser.serialize_field("quorumWeight", &self.quorum_weight)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProviderConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "timeout_ms",
            "timeoutMs",
            "max_try_count",
            "maxTryCount",
            "quorum_weight",
            "quorumWeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            TimeoutMs,
            MaxTryCount,
            QuorumWeight,
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
                            "maxTryCount" | "max_try_count" => Ok(GeneratedField::MaxTryCount),
                            "quorumWeight" | "quorum_weight" => Ok(GeneratedField::QuorumWeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.v1.ProviderConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProviderConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut timeout_ms__ = None;
                let mut max_try_count__ = None;
                let mut quorum_weight__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimeoutMs => {
                            if timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutMs"));
                            }
                            timeout_ms__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxTryCount => {
                            if max_try_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTryCount"));
                            }
                            max_try_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::QuorumWeight => {
                            if quorum_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quorumWeight"));
                            }
                            quorum_weight__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProviderConfig {
                    url: url__.unwrap_or_default(),
                    timeout_ms: timeout_ms__.unwrap_or_default(),
                    max_try_count: max_try_count__.unwrap_or_default(),
                    quorum_weight: quorum_weight__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.v1.ProviderConfig", FIELDS, GeneratedVisitor)
    }
}
