// @generated
impl serde::Serialize for SynchronizerChainStatus {
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
        if self.syncing {
            len += 1;
        }
        if !self.contracts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.synchronizer.v1.SynchronizerChainStatus", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if self.synced_block != 0 {
            struct_ser.serialize_field("syncedBlock", ToString::to_string(&self.synced_block).as_str())?;
        }
        if self.syncing {
            struct_ser.serialize_field("syncing", &self.syncing)?;
        }
        if !self.contracts.is_empty() {
            struct_ser.serialize_field("contracts", &self.contracts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SynchronizerChainStatus {
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
            "syncing",
            "contracts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            SyncedBlock,
            Syncing,
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
                            "syncing" => Ok(GeneratedField::Syncing),
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
            type Value = SynchronizerChainStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.synchronizer.v1.SynchronizerChainStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SynchronizerChainStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut synced_block__ = None;
                let mut syncing__ = None;
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
                        GeneratedField::Syncing => {
                            if syncing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syncing"));
                            }
                            syncing__ = Some(map.next_value()?);
                        }
                        GeneratedField::Contracts => {
                            if contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contracts"));
                            }
                            contracts__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SynchronizerChainStatus {
                    chain_id: chain_id__.unwrap_or_default(),
                    synced_block: synced_block__.unwrap_or_default(),
                    syncing: syncing__.unwrap_or_default(),
                    contracts: contracts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.synchronizer.v1.SynchronizerChainStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SynchronizerContractStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.synced_block != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.synchronizer.v1.SynchronizerContractStatus", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.synced_block != 0 {
            struct_ser.serialize_field("syncedBlock", ToString::to_string(&self.synced_block).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SynchronizerContractStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "synced_block",
            "syncedBlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
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
            type Value = SynchronizerContractStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.synchronizer.v1.SynchronizerContractStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SynchronizerContractStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut synced_block__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
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
                Ok(SynchronizerContractStatus {
                    address: address__.unwrap_or_default(),
                    synced_block: synced_block__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.synchronizer.v1.SynchronizerContractStatus", FIELDS, GeneratedVisitor)
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
