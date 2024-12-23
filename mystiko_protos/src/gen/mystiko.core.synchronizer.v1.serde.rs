// @generated
impl serde::Serialize for ChainStatus {
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
        if self.synced_block != 0 {
            len += 1;
        }
        if self.target_block != 0 {
            len += 1;
        }
        if !self.contracts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.synchronizer.v1.ChainStatus", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if self.synced_block != 0 {
            struct_ser.serialize_field("syncedBlock", ToString::to_string(&self.synced_block).as_str())?;
        }
        if self.target_block != 0 {
            struct_ser.serialize_field("targetBlock", ToString::to_string(&self.target_block).as_str())?;
        }
        if !self.contracts.is_empty() {
            struct_ser.serialize_field("contracts", &self.contracts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "synced_block",
            "syncedBlock",
            "target_block",
            "targetBlock",
            "contracts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            SyncedBlock,
            TargetBlock,
            Contracts,
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
                            "syncedBlock" | "synced_block" => Ok(GeneratedField::SyncedBlock),
                            "targetBlock" | "target_block" => Ok(GeneratedField::TargetBlock),
                            "contracts" => Ok(GeneratedField::Contracts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.synchronizer.v1.ChainStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChainStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut synced_block__ = None;
                let mut target_block__ = None;
                let mut contracts__ = None;
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
                        GeneratedField::SyncedBlock => {
                            if synced_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syncedBlock"));
                            }
                            synced_block__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TargetBlock => {
                            if target_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetBlock"));
                            }
                            target_block__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Contracts => {
                            if contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contracts"));
                            }
                            contracts__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ChainStatus {
                    chain_id: chain_id__.unwrap_or_default(),
                    synced_block: synced_block__.unwrap_or_default(),
                    target_block: target_block__.unwrap_or_default(),
                    contracts: contracts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.synchronizer.v1.ChainStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContractStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.synced_block != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.synchronizer.v1.ContractStatus", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.synced_block != 0 {
            struct_ser.serialize_field("syncedBlock", ToString::to_string(&self.synced_block).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "synced_block",
            "syncedBlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            SyncedBlock,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "syncedBlock" | "synced_block" => Ok(GeneratedField::SyncedBlock),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.synchronizer.v1.ContractStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContractStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut synced_block__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::SyncedBlock => {
                            if synced_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syncedBlock"));
                            }
                            synced_block__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ContractStatus {
                    contract_address: contract_address__.unwrap_or_default(),
                    synced_block: synced_block__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.synchronizer.v1.ContractStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetChainOptions {
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
        if !self.contract_addresses.is_empty() {
            len += 1;
        }
        if self.block_number.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.synchronizer.v1.ResetChainOptions", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_addresses.is_empty() {
            struct_ser.serialize_field("contractAddresses", &self.contract_addresses)?;
        }
        if let Some(v) = self.block_number.as_ref() {
            struct_ser.serialize_field("blockNumber", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetChainOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_addresses",
            "contractAddresses",
            "block_number",
            "blockNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddresses,
            BlockNumber,
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
                            "contractAddresses" | "contract_addresses" => Ok(GeneratedField::ContractAddresses),
                            "blockNumber" | "block_number" => Ok(GeneratedField::BlockNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResetChainOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.synchronizer.v1.ResetChainOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResetChainOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_addresses__ = None;
                let mut block_number__ = None;
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
                        GeneratedField::ContractAddresses => {
                            if contract_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddresses"));
                            }
                            contract_addresses__ = Some(map.next_value()?);
                        }
                        GeneratedField::BlockNumber => {
                            if block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockNumber"));
                            }
                            block_number__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(ResetChainOptions {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_addresses: contract_addresses__.unwrap_or_default(),
                    block_number: block_number__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.synchronizer.v1.ResetChainOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SynchronizerResetOptions {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.core.synchronizer.v1.SynchronizerResetOptions", len)?;
        if !self.chains.is_empty() {
            struct_ser.serialize_field("chains", &self.chains)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SynchronizerResetOptions {
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
            type Value = SynchronizerResetOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.synchronizer.v1.SynchronizerResetOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SynchronizerResetOptions, V::Error>
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
                Ok(SynchronizerResetOptions {
                    chains: chains__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.synchronizer.v1.SynchronizerResetOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SynchronizerStatus {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.core.synchronizer.v1.SynchronizerStatus", len)?;
        if !self.chains.is_empty() {
            struct_ser.serialize_field("chains", &self.chains)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SynchronizerStatus {
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
            type Value = SynchronizerStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.synchronizer.v1.SynchronizerStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SynchronizerStatus, V::Error>
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
                Ok(SynchronizerStatus {
                    chains: chains__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.synchronizer.v1.SynchronizerStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SynchronizerSyncOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.disable_datapacker_fetcher.is_some() {
            len += 1;
        }
        if self.enable_datapacker_fetcher_validate.is_some() {
            len += 1;
        }
        if self.disable_sequencer_fetcher.is_some() {
            len += 1;
        }
        if self.enable_sequencer_fetcher_validate.is_some() {
            len += 1;
        }
        if self.disable_provider_fetcher.is_some() {
            len += 1;
        }
        if self.disable_provider_fetcher_validate.is_some() {
            len += 1;
        }
        if self.disable_rule_validator.is_some() {
            len += 1;
        }
        if self.disable_rule_validator_integrity_check.is_some() {
            len += 1;
        }
        if self.disable_rule_validator_sequence_check.is_some() {
            len += 1;
        }
        if self.disable_rule_validator_counter_check.is_some() {
            len += 1;
        }
        if self.disable_rule_validator_tree_check.is_some() {
            len += 1;
        }
        if self.fetcher_fetch_timeout_ms.is_some() {
            len += 1;
        }
        if self.fetcher_query_loaded_block_timeout_ms.is_some() {
            len += 1;
        }
        if self.validator_validate_concurrency.is_some() {
            len += 1;
        }
        if !self.chain_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.synchronizer.v1.SynchronizerSyncOptions", len)?;
        if let Some(v) = self.disable_datapacker_fetcher.as_ref() {
            struct_ser.serialize_field("disableDatapackerFetcher", v)?;
        }
        if let Some(v) = self.enable_datapacker_fetcher_validate.as_ref() {
            struct_ser.serialize_field("enableDatapackerFetcherValidate", v)?;
        }
        if let Some(v) = self.disable_sequencer_fetcher.as_ref() {
            struct_ser.serialize_field("disableSequencerFetcher", v)?;
        }
        if let Some(v) = self.enable_sequencer_fetcher_validate.as_ref() {
            struct_ser.serialize_field("enableSequencerFetcherValidate", v)?;
        }
        if let Some(v) = self.disable_provider_fetcher.as_ref() {
            struct_ser.serialize_field("disableProviderFetcher", v)?;
        }
        if let Some(v) = self.disable_provider_fetcher_validate.as_ref() {
            struct_ser.serialize_field("disableProviderFetcherValidate", v)?;
        }
        if let Some(v) = self.disable_rule_validator.as_ref() {
            struct_ser.serialize_field("disableRuleValidator", v)?;
        }
        if let Some(v) = self.disable_rule_validator_integrity_check.as_ref() {
            struct_ser.serialize_field("disableRuleValidatorIntegrityCheck", v)?;
        }
        if let Some(v) = self.disable_rule_validator_sequence_check.as_ref() {
            struct_ser.serialize_field("disableRuleValidatorSequenceCheck", v)?;
        }
        if let Some(v) = self.disable_rule_validator_counter_check.as_ref() {
            struct_ser.serialize_field("disableRuleValidatorCounterCheck", v)?;
        }
        if let Some(v) = self.disable_rule_validator_tree_check.as_ref() {
            struct_ser.serialize_field("disableRuleValidatorTreeCheck", v)?;
        }
        if let Some(v) = self.fetcher_fetch_timeout_ms.as_ref() {
            struct_ser.serialize_field("fetcherFetchTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.fetcher_query_loaded_block_timeout_ms.as_ref() {
            struct_ser.serialize_field("fetcherQueryLoadedBlockTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.validator_validate_concurrency.as_ref() {
            struct_ser.serialize_field("validatorValidateConcurrency", ToString::to_string(&v).as_str())?;
        }
        if !self.chain_ids.is_empty() {
            struct_ser.serialize_field("chainIds", &self.chain_ids.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SynchronizerSyncOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disable_datapacker_fetcher",
            "disableDatapackerFetcher",
            "enable_datapacker_fetcher_validate",
            "enableDatapackerFetcherValidate",
            "disable_sequencer_fetcher",
            "disableSequencerFetcher",
            "enable_sequencer_fetcher_validate",
            "enableSequencerFetcherValidate",
            "disable_provider_fetcher",
            "disableProviderFetcher",
            "disable_provider_fetcher_validate",
            "disableProviderFetcherValidate",
            "disable_rule_validator",
            "disableRuleValidator",
            "disable_rule_validator_integrity_check",
            "disableRuleValidatorIntegrityCheck",
            "disable_rule_validator_sequence_check",
            "disableRuleValidatorSequenceCheck",
            "disable_rule_validator_counter_check",
            "disableRuleValidatorCounterCheck",
            "disable_rule_validator_tree_check",
            "disableRuleValidatorTreeCheck",
            "fetcher_fetch_timeout_ms",
            "fetcherFetchTimeoutMs",
            "fetcher_query_loaded_block_timeout_ms",
            "fetcherQueryLoadedBlockTimeoutMs",
            "validator_validate_concurrency",
            "validatorValidateConcurrency",
            "chain_ids",
            "chainIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DisableDatapackerFetcher,
            EnableDatapackerFetcherValidate,
            DisableSequencerFetcher,
            EnableSequencerFetcherValidate,
            DisableProviderFetcher,
            DisableProviderFetcherValidate,
            DisableRuleValidator,
            DisableRuleValidatorIntegrityCheck,
            DisableRuleValidatorSequenceCheck,
            DisableRuleValidatorCounterCheck,
            DisableRuleValidatorTreeCheck,
            FetcherFetchTimeoutMs,
            FetcherQueryLoadedBlockTimeoutMs,
            ValidatorValidateConcurrency,
            ChainIds,
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
                            "disableDatapackerFetcher" | "disable_datapacker_fetcher" => Ok(GeneratedField::DisableDatapackerFetcher),
                            "enableDatapackerFetcherValidate" | "enable_datapacker_fetcher_validate" => Ok(GeneratedField::EnableDatapackerFetcherValidate),
                            "disableSequencerFetcher" | "disable_sequencer_fetcher" => Ok(GeneratedField::DisableSequencerFetcher),
                            "enableSequencerFetcherValidate" | "enable_sequencer_fetcher_validate" => Ok(GeneratedField::EnableSequencerFetcherValidate),
                            "disableProviderFetcher" | "disable_provider_fetcher" => Ok(GeneratedField::DisableProviderFetcher),
                            "disableProviderFetcherValidate" | "disable_provider_fetcher_validate" => Ok(GeneratedField::DisableProviderFetcherValidate),
                            "disableRuleValidator" | "disable_rule_validator" => Ok(GeneratedField::DisableRuleValidator),
                            "disableRuleValidatorIntegrityCheck" | "disable_rule_validator_integrity_check" => Ok(GeneratedField::DisableRuleValidatorIntegrityCheck),
                            "disableRuleValidatorSequenceCheck" | "disable_rule_validator_sequence_check" => Ok(GeneratedField::DisableRuleValidatorSequenceCheck),
                            "disableRuleValidatorCounterCheck" | "disable_rule_validator_counter_check" => Ok(GeneratedField::DisableRuleValidatorCounterCheck),
                            "disableRuleValidatorTreeCheck" | "disable_rule_validator_tree_check" => Ok(GeneratedField::DisableRuleValidatorTreeCheck),
                            "fetcherFetchTimeoutMs" | "fetcher_fetch_timeout_ms" => Ok(GeneratedField::FetcherFetchTimeoutMs),
                            "fetcherQueryLoadedBlockTimeoutMs" | "fetcher_query_loaded_block_timeout_ms" => Ok(GeneratedField::FetcherQueryLoadedBlockTimeoutMs),
                            "validatorValidateConcurrency" | "validator_validate_concurrency" => Ok(GeneratedField::ValidatorValidateConcurrency),
                            "chainIds" | "chain_ids" => Ok(GeneratedField::ChainIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SynchronizerSyncOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.synchronizer.v1.SynchronizerSyncOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SynchronizerSyncOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut disable_datapacker_fetcher__ = None;
                let mut enable_datapacker_fetcher_validate__ = None;
                let mut disable_sequencer_fetcher__ = None;
                let mut enable_sequencer_fetcher_validate__ = None;
                let mut disable_provider_fetcher__ = None;
                let mut disable_provider_fetcher_validate__ = None;
                let mut disable_rule_validator__ = None;
                let mut disable_rule_validator_integrity_check__ = None;
                let mut disable_rule_validator_sequence_check__ = None;
                let mut disable_rule_validator_counter_check__ = None;
                let mut disable_rule_validator_tree_check__ = None;
                let mut fetcher_fetch_timeout_ms__ = None;
                let mut fetcher_query_loaded_block_timeout_ms__ = None;
                let mut validator_validate_concurrency__ = None;
                let mut chain_ids__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DisableDatapackerFetcher => {
                            if disable_datapacker_fetcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableDatapackerFetcher"));
                            }
                            disable_datapacker_fetcher__ = map.next_value()?;
                        }
                        GeneratedField::EnableDatapackerFetcherValidate => {
                            if enable_datapacker_fetcher_validate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableDatapackerFetcherValidate"));
                            }
                            enable_datapacker_fetcher_validate__ = map.next_value()?;
                        }
                        GeneratedField::DisableSequencerFetcher => {
                            if disable_sequencer_fetcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableSequencerFetcher"));
                            }
                            disable_sequencer_fetcher__ = map.next_value()?;
                        }
                        GeneratedField::EnableSequencerFetcherValidate => {
                            if enable_sequencer_fetcher_validate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableSequencerFetcherValidate"));
                            }
                            enable_sequencer_fetcher_validate__ = map.next_value()?;
                        }
                        GeneratedField::DisableProviderFetcher => {
                            if disable_provider_fetcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableProviderFetcher"));
                            }
                            disable_provider_fetcher__ = map.next_value()?;
                        }
                        GeneratedField::DisableProviderFetcherValidate => {
                            if disable_provider_fetcher_validate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableProviderFetcherValidate"));
                            }
                            disable_provider_fetcher_validate__ = map.next_value()?;
                        }
                        GeneratedField::DisableRuleValidator => {
                            if disable_rule_validator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableRuleValidator"));
                            }
                            disable_rule_validator__ = map.next_value()?;
                        }
                        GeneratedField::DisableRuleValidatorIntegrityCheck => {
                            if disable_rule_validator_integrity_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableRuleValidatorIntegrityCheck"));
                            }
                            disable_rule_validator_integrity_check__ = map.next_value()?;
                        }
                        GeneratedField::DisableRuleValidatorSequenceCheck => {
                            if disable_rule_validator_sequence_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableRuleValidatorSequenceCheck"));
                            }
                            disable_rule_validator_sequence_check__ = map.next_value()?;
                        }
                        GeneratedField::DisableRuleValidatorCounterCheck => {
                            if disable_rule_validator_counter_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableRuleValidatorCounterCheck"));
                            }
                            disable_rule_validator_counter_check__ = map.next_value()?;
                        }
                        GeneratedField::DisableRuleValidatorTreeCheck => {
                            if disable_rule_validator_tree_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableRuleValidatorTreeCheck"));
                            }
                            disable_rule_validator_tree_check__ = map.next_value()?;
                        }
                        GeneratedField::FetcherFetchTimeoutMs => {
                            if fetcher_fetch_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fetcherFetchTimeoutMs"));
                            }
                            fetcher_fetch_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::FetcherQueryLoadedBlockTimeoutMs => {
                            if fetcher_query_loaded_block_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fetcherQueryLoadedBlockTimeoutMs"));
                            }
                            fetcher_query_loaded_block_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ValidatorValidateConcurrency => {
                            if validator_validate_concurrency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorValidateConcurrency"));
                            }
                            validator_validate_concurrency__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
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
                    }
                }
                Ok(SynchronizerSyncOptions {
                    disable_datapacker_fetcher: disable_datapacker_fetcher__,
                    enable_datapacker_fetcher_validate: enable_datapacker_fetcher_validate__,
                    disable_sequencer_fetcher: disable_sequencer_fetcher__,
                    enable_sequencer_fetcher_validate: enable_sequencer_fetcher_validate__,
                    disable_provider_fetcher: disable_provider_fetcher__,
                    disable_provider_fetcher_validate: disable_provider_fetcher_validate__,
                    disable_rule_validator: disable_rule_validator__,
                    disable_rule_validator_integrity_check: disable_rule_validator_integrity_check__,
                    disable_rule_validator_sequence_check: disable_rule_validator_sequence_check__,
                    disable_rule_validator_counter_check: disable_rule_validator_counter_check__,
                    disable_rule_validator_tree_check: disable_rule_validator_tree_check__,
                    fetcher_fetch_timeout_ms: fetcher_fetch_timeout_ms__,
                    fetcher_query_loaded_block_timeout_ms: fetcher_query_loaded_block_timeout_ms__,
                    validator_validate_concurrency: validator_validate_concurrency__,
                    chain_ids: chain_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.synchronizer.v1.SynchronizerSyncOptions", FIELDS, GeneratedVisitor)
    }
}
