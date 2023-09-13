// @generated
impl serde::Serialize for ContractConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if self.start_block != 0 {
            len += 1;
        }
        if self.disabled {
            len += 1;
        }
        if !self.min_rollup_fee.is_empty() {
            len += 1;
        }
        if self.asset_config.is_some() {
            len += 1;
        }
        if self.bridge_type != 0 {
            len += 1;
        }
        if self.contract_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.contract.v1.ContractConfig", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.start_block != 0 {
            struct_ser.serialize_field("startBlock", ToString::to_string(&self.start_block).as_str())?;
        }
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        if !self.min_rollup_fee.is_empty() {
            struct_ser.serialize_field("minRollupFee", &self.min_rollup_fee)?;
        }
        if let Some(v) = self.asset_config.as_ref() {
            struct_ser.serialize_field("assetConfig", v)?;
        }
        if self.bridge_type != 0 {
            let v = super::super::super::common::v1::BridgeType::from_i32(self.bridge_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.bridge_type)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        if self.contract_type != 0 {
            let v = super::super::super::common::v1::ContractType::from_i32(self.contract_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.contract_type)))?;
            struct_ser.serialize_field("contractType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "name",
            "address",
            "start_block",
            "startBlock",
            "disabled",
            "min_rollup_fee",
            "minRollupFee",
            "asset_config",
            "assetConfig",
            "bridge_type",
            "bridgeType",
            "contract_type",
            "contractType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Name,
            Address,
            StartBlock,
            Disabled,
            MinRollupFee,
            AssetConfig,
            BridgeType,
            ContractType,
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
                            "name" => Ok(GeneratedField::Name),
                            "address" => Ok(GeneratedField::Address),
                            "startBlock" | "start_block" => Ok(GeneratedField::StartBlock),
                            "disabled" => Ok(GeneratedField::Disabled),
                            "minRollupFee" | "min_rollup_fee" => Ok(GeneratedField::MinRollupFee),
                            "assetConfig" | "asset_config" => Ok(GeneratedField::AssetConfig),
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
                            "contractType" | "contract_type" => Ok(GeneratedField::ContractType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.contract.v1.ContractConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContractConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut name__ = None;
                let mut address__ = None;
                let mut start_block__ = None;
                let mut disabled__ = None;
                let mut min_rollup_fee__ = None;
                let mut asset_config__ = None;
                let mut bridge_type__ = None;
                let mut contract_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::StartBlock => {
                            if start_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startBlock"));
                            }
                            start_block__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinRollupFee => {
                            if min_rollup_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minRollupFee"));
                            }
                            min_rollup_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::AssetConfig => {
                            if asset_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetConfig"));
                            }
                            asset_config__ = map.next_value()?;
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = Some(map.next_value::<super::super::super::common::v1::BridgeType>()? as i32);
                        }
                        GeneratedField::ContractType => {
                            if contract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractType"));
                            }
                            contract_type__ = Some(map.next_value::<super::super::super::common::v1::ContractType>()? as i32);
                        }
                    }
                }
                Ok(ContractConfig {
                    version: version__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    start_block: start_block__.unwrap_or_default(),
                    disabled: disabled__.unwrap_or_default(),
                    min_rollup_fee: min_rollup_fee__.unwrap_or_default(),
                    asset_config: asset_config__,
                    bridge_type: bridge_type__.unwrap_or_default(),
                    contract_type: contract_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.contract.v1.ContractConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DepositContractConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if self.start_block != 0 {
            len += 1;
        }
        if self.disabled {
            len += 1;
        }
        if !self.min_amount.is_empty() {
            len += 1;
        }
        if !self.max_amount.is_empty() {
            len += 1;
        }
        if self.pool_contract_config.is_some() {
            len += 1;
        }
        if self.bridge_type != 0 {
            len += 1;
        }
        if self.contract_type != 0 {
            len += 1;
        }
        if self.min_bridge_fee.is_some() {
            len += 1;
        }
        if self.min_executor_fee.is_some() {
            len += 1;
        }
        if self.service_fee.is_some() {
            len += 1;
        }
        if self.service_fee_divider.is_some() {
            len += 1;
        }
        if self.bridge_fee_asset_config.is_some() {
            len += 1;
        }
        if self.executor_fee_asset_config.is_some() {
            len += 1;
        }
        if self.peer_chain_id.is_some() {
            len += 1;
        }
        if self.peer_contract_address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.contract.v1.DepositContractConfig", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.start_block != 0 {
            struct_ser.serialize_field("startBlock", ToString::to_string(&self.start_block).as_str())?;
        }
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        if !self.min_amount.is_empty() {
            struct_ser.serialize_field("minAmount", &self.min_amount)?;
        }
        if !self.max_amount.is_empty() {
            struct_ser.serialize_field("maxAmount", &self.max_amount)?;
        }
        if let Some(v) = self.pool_contract_config.as_ref() {
            struct_ser.serialize_field("poolContractConfig", v)?;
        }
        if self.bridge_type != 0 {
            let v = super::super::super::common::v1::BridgeType::from_i32(self.bridge_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.bridge_type)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        if self.contract_type != 0 {
            let v = super::super::super::common::v1::ContractType::from_i32(self.contract_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.contract_type)))?;
            struct_ser.serialize_field("contractType", &v)?;
        }
        if let Some(v) = self.min_bridge_fee.as_ref() {
            struct_ser.serialize_field("minBridgeFee", v)?;
        }
        if let Some(v) = self.min_executor_fee.as_ref() {
            struct_ser.serialize_field("minExecutorFee", v)?;
        }
        if let Some(v) = self.service_fee.as_ref() {
            struct_ser.serialize_field("serviceFee", v)?;
        }
        if let Some(v) = self.service_fee_divider.as_ref() {
            struct_ser.serialize_field("serviceFeeDivider", v)?;
        }
        if let Some(v) = self.bridge_fee_asset_config.as_ref() {
            struct_ser.serialize_field("bridgeFeeAssetConfig", v)?;
        }
        if let Some(v) = self.executor_fee_asset_config.as_ref() {
            struct_ser.serialize_field("executorFeeAssetConfig", v)?;
        }
        if let Some(v) = self.peer_chain_id.as_ref() {
            struct_ser.serialize_field("peerChainId", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.peer_contract_address.as_ref() {
            struct_ser.serialize_field("peerContractAddress", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DepositContractConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "name",
            "address",
            "start_block",
            "startBlock",
            "disabled",
            "min_amount",
            "minAmount",
            "max_amount",
            "maxAmount",
            "pool_contract_config",
            "poolContractConfig",
            "bridge_type",
            "bridgeType",
            "contract_type",
            "contractType",
            "min_bridge_fee",
            "minBridgeFee",
            "min_executor_fee",
            "minExecutorFee",
            "service_fee",
            "serviceFee",
            "service_fee_divider",
            "serviceFeeDivider",
            "bridge_fee_asset_config",
            "bridgeFeeAssetConfig",
            "executor_fee_asset_config",
            "executorFeeAssetConfig",
            "peer_chain_id",
            "peerChainId",
            "peer_contract_address",
            "peerContractAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Name,
            Address,
            StartBlock,
            Disabled,
            MinAmount,
            MaxAmount,
            PoolContractConfig,
            BridgeType,
            ContractType,
            MinBridgeFee,
            MinExecutorFee,
            ServiceFee,
            ServiceFeeDivider,
            BridgeFeeAssetConfig,
            ExecutorFeeAssetConfig,
            PeerChainId,
            PeerContractAddress,
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
                            "name" => Ok(GeneratedField::Name),
                            "address" => Ok(GeneratedField::Address),
                            "startBlock" | "start_block" => Ok(GeneratedField::StartBlock),
                            "disabled" => Ok(GeneratedField::Disabled),
                            "minAmount" | "min_amount" => Ok(GeneratedField::MinAmount),
                            "maxAmount" | "max_amount" => Ok(GeneratedField::MaxAmount),
                            "poolContractConfig" | "pool_contract_config" => Ok(GeneratedField::PoolContractConfig),
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
                            "contractType" | "contract_type" => Ok(GeneratedField::ContractType),
                            "minBridgeFee" | "min_bridge_fee" => Ok(GeneratedField::MinBridgeFee),
                            "minExecutorFee" | "min_executor_fee" => Ok(GeneratedField::MinExecutorFee),
                            "serviceFee" | "service_fee" => Ok(GeneratedField::ServiceFee),
                            "serviceFeeDivider" | "service_fee_divider" => Ok(GeneratedField::ServiceFeeDivider),
                            "bridgeFeeAssetConfig" | "bridge_fee_asset_config" => Ok(GeneratedField::BridgeFeeAssetConfig),
                            "executorFeeAssetConfig" | "executor_fee_asset_config" => Ok(GeneratedField::ExecutorFeeAssetConfig),
                            "peerChainId" | "peer_chain_id" => Ok(GeneratedField::PeerChainId),
                            "peerContractAddress" | "peer_contract_address" => Ok(GeneratedField::PeerContractAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DepositContractConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.contract.v1.DepositContractConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DepositContractConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut name__ = None;
                let mut address__ = None;
                let mut start_block__ = None;
                let mut disabled__ = None;
                let mut min_amount__ = None;
                let mut max_amount__ = None;
                let mut pool_contract_config__ = None;
                let mut bridge_type__ = None;
                let mut contract_type__ = None;
                let mut min_bridge_fee__ = None;
                let mut min_executor_fee__ = None;
                let mut service_fee__ = None;
                let mut service_fee_divider__ = None;
                let mut bridge_fee_asset_config__ = None;
                let mut executor_fee_asset_config__ = None;
                let mut peer_chain_id__ = None;
                let mut peer_contract_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::StartBlock => {
                            if start_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startBlock"));
                            }
                            start_block__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinAmount => {
                            if min_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minAmount"));
                            }
                            min_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxAmount => {
                            if max_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAmount"));
                            }
                            max_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::PoolContractConfig => {
                            if pool_contract_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolContractConfig"));
                            }
                            pool_contract_config__ = map.next_value()?;
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = Some(map.next_value::<super::super::super::common::v1::BridgeType>()? as i32);
                        }
                        GeneratedField::ContractType => {
                            if contract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractType"));
                            }
                            contract_type__ = Some(map.next_value::<super::super::super::common::v1::ContractType>()? as i32);
                        }
                        GeneratedField::MinBridgeFee => {
                            if min_bridge_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minBridgeFee"));
                            }
                            min_bridge_fee__ = map.next_value()?;
                        }
                        GeneratedField::MinExecutorFee => {
                            if min_executor_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minExecutorFee"));
                            }
                            min_executor_fee__ = map.next_value()?;
                        }
                        GeneratedField::ServiceFee => {
                            if service_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceFee"));
                            }
                            service_fee__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ServiceFeeDivider => {
                            if service_fee_divider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceFeeDivider"));
                            }
                            service_fee_divider__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BridgeFeeAssetConfig => {
                            if bridge_fee_asset_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeAssetConfig"));
                            }
                            bridge_fee_asset_config__ = map.next_value()?;
                        }
                        GeneratedField::ExecutorFeeAssetConfig => {
                            if executor_fee_asset_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorFeeAssetConfig"));
                            }
                            executor_fee_asset_config__ = map.next_value()?;
                        }
                        GeneratedField::PeerChainId => {
                            if peer_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peerChainId"));
                            }
                            peer_chain_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PeerContractAddress => {
                            if peer_contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peerContractAddress"));
                            }
                            peer_contract_address__ = map.next_value()?;
                        }
                    }
                }
                Ok(DepositContractConfig {
                    version: version__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    start_block: start_block__.unwrap_or_default(),
                    disabled: disabled__.unwrap_or_default(),
                    min_amount: min_amount__.unwrap_or_default(),
                    max_amount: max_amount__.unwrap_or_default(),
                    pool_contract_config: pool_contract_config__,
                    bridge_type: bridge_type__.unwrap_or_default(),
                    contract_type: contract_type__.unwrap_or_default(),
                    min_bridge_fee: min_bridge_fee__,
                    min_executor_fee: min_executor_fee__,
                    service_fee: service_fee__,
                    service_fee_divider: service_fee_divider__,
                    bridge_fee_asset_config: bridge_fee_asset_config__,
                    executor_fee_asset_config: executor_fee_asset_config__,
                    peer_chain_id: peer_chain_id__,
                    peer_contract_address: peer_contract_address__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.contract.v1.DepositContractConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PoolContractConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if self.start_block != 0 {
            len += 1;
        }
        if !self.pool_name.is_empty() {
            len += 1;
        }
        if !self.min_rollup_fee.is_empty() {
            len += 1;
        }
        if self.contract_type != 0 {
            len += 1;
        }
        if self.bridge_type != 0 {
            len += 1;
        }
        if self.asset_config.is_some() {
            len += 1;
        }
        if !self.circuit_configs.is_empty() {
            len += 1;
        }
        if !self.circuits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.config.contract.v1.PoolContractConfig", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.start_block != 0 {
            struct_ser.serialize_field("startBlock", ToString::to_string(&self.start_block).as_str())?;
        }
        if !self.pool_name.is_empty() {
            struct_ser.serialize_field("poolName", &self.pool_name)?;
        }
        if !self.min_rollup_fee.is_empty() {
            struct_ser.serialize_field("minRollupFee", &self.min_rollup_fee)?;
        }
        if self.contract_type != 0 {
            let v = super::super::super::common::v1::ContractType::from_i32(self.contract_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.contract_type)))?;
            struct_ser.serialize_field("contractType", &v)?;
        }
        if self.bridge_type != 0 {
            let v = super::super::super::common::v1::BridgeType::from_i32(self.bridge_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.bridge_type)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        if let Some(v) = self.asset_config.as_ref() {
            struct_ser.serialize_field("assetConfig", v)?;
        }
        if !self.circuit_configs.is_empty() {
            struct_ser.serialize_field("circuitConfigs", &self.circuit_configs)?;
        }
        if !self.circuits.is_empty() {
            struct_ser.serialize_field("circuits", &self.circuits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PoolContractConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "name",
            "address",
            "start_block",
            "startBlock",
            "pool_name",
            "poolName",
            "min_rollup_fee",
            "minRollupFee",
            "contract_type",
            "contractType",
            "bridge_type",
            "bridgeType",
            "asset_config",
            "assetConfig",
            "circuit_configs",
            "circuitConfigs",
            "circuits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Name,
            Address,
            StartBlock,
            PoolName,
            MinRollupFee,
            ContractType,
            BridgeType,
            AssetConfig,
            CircuitConfigs,
            Circuits,
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
                            "name" => Ok(GeneratedField::Name),
                            "address" => Ok(GeneratedField::Address),
                            "startBlock" | "start_block" => Ok(GeneratedField::StartBlock),
                            "poolName" | "pool_name" => Ok(GeneratedField::PoolName),
                            "minRollupFee" | "min_rollup_fee" => Ok(GeneratedField::MinRollupFee),
                            "contractType" | "contract_type" => Ok(GeneratedField::ContractType),
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
                            "assetConfig" | "asset_config" => Ok(GeneratedField::AssetConfig),
                            "circuitConfigs" | "circuit_configs" => Ok(GeneratedField::CircuitConfigs),
                            "circuits" => Ok(GeneratedField::Circuits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PoolContractConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.config.contract.v1.PoolContractConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PoolContractConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut name__ = None;
                let mut address__ = None;
                let mut start_block__ = None;
                let mut pool_name__ = None;
                let mut min_rollup_fee__ = None;
                let mut contract_type__ = None;
                let mut bridge_type__ = None;
                let mut asset_config__ = None;
                let mut circuit_configs__ = None;
                let mut circuits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::StartBlock => {
                            if start_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startBlock"));
                            }
                            start_block__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PoolName => {
                            if pool_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolName"));
                            }
                            pool_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinRollupFee => {
                            if min_rollup_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minRollupFee"));
                            }
                            min_rollup_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContractType => {
                            if contract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractType"));
                            }
                            contract_type__ = Some(map.next_value::<super::super::super::common::v1::ContractType>()? as i32);
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = Some(map.next_value::<super::super::super::common::v1::BridgeType>()? as i32);
                        }
                        GeneratedField::AssetConfig => {
                            if asset_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetConfig"));
                            }
                            asset_config__ = map.next_value()?;
                        }
                        GeneratedField::CircuitConfigs => {
                            if circuit_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("circuitConfigs"));
                            }
                            circuit_configs__ = Some(map.next_value()?);
                        }
                        GeneratedField::Circuits => {
                            if circuits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("circuits"));
                            }
                            circuits__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PoolContractConfig {
                    version: version__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    start_block: start_block__.unwrap_or_default(),
                    pool_name: pool_name__.unwrap_or_default(),
                    min_rollup_fee: min_rollup_fee__.unwrap_or_default(),
                    contract_type: contract_type__.unwrap_or_default(),
                    bridge_type: bridge_type__.unwrap_or_default(),
                    asset_config: asset_config__,
                    circuit_configs: circuit_configs__.unwrap_or_default(),
                    circuits: circuits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.config.contract.v1.PoolContractConfig", FIELDS, GeneratedVisitor)
    }
}
