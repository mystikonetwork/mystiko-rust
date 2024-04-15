// @generated
impl serde::Serialize for ChainLoadedBlockRequest {
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
        if self.with_contracts.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.ChainLoadedBlockRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if let Some(v) = self.with_contracts.as_ref() {
            struct_ser.serialize_field("withContracts", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainLoadedBlockRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "with_contracts",
            "withContracts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            WithContracts,
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
                            "withContracts" | "with_contracts" => Ok(GeneratedField::WithContracts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainLoadedBlockRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.ChainLoadedBlockRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChainLoadedBlockRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut with_contracts__ = None;
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
                        GeneratedField::WithContracts => {
                            if with_contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withContracts"));
                            }
                            with_contracts__ = map.next_value()?;
                        }
                    }
                }
                Ok(ChainLoadedBlockRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    with_contracts: with_contracts__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.ChainLoadedBlockRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChainLoadedBlockResponse {
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
        if self.block_number != 0 {
            len += 1;
        }
        if !self.contracts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.ChainLoadedBlockResponse", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if self.block_number != 0 {
            struct_ser.serialize_field("blockNumber", ToString::to_string(&self.block_number).as_str())?;
        }
        if !self.contracts.is_empty() {
            struct_ser.serialize_field("contracts", &self.contracts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainLoadedBlockResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "block_number",
            "blockNumber",
            "contracts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            BlockNumber,
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
                            "blockNumber" | "block_number" => Ok(GeneratedField::BlockNumber),
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
            type Value = ChainLoadedBlockResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.ChainLoadedBlockResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChainLoadedBlockResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut block_number__ = None;
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
                        GeneratedField::BlockNumber => {
                            if block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockNumber"));
                            }
                            block_number__ = 
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
                Ok(ChainLoadedBlockResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                    block_number: block_number__.unwrap_or_default(),
                    contracts: contracts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.ChainLoadedBlockResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContractLoadedBlockRequest {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.ContractLoadedBlockRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractLoadedBlockRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddress,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractLoadedBlockRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.ContractLoadedBlockRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContractLoadedBlockRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_address__ = None;
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
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ContractLoadedBlockRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.ContractLoadedBlockRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContractLoadedBlockResponse {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.block_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.ContractLoadedBlockResponse", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if self.block_number != 0 {
            struct_ser.serialize_field("blockNumber", ToString::to_string(&self.block_number).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractLoadedBlockResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "block_number",
            "blockNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddress,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
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
            type Value = ContractLoadedBlockResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.ContractLoadedBlockResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContractLoadedBlockResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_address__ = None;
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
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockNumber => {
                            if block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockNumber"));
                            }
                            block_number__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ContractLoadedBlockResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    block_number: block_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.ContractLoadedBlockResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FetchChainRequest {
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
        if self.start_block != 0 {
            len += 1;
        }
        if self.target_block != 0 {
            len += 1;
        }
        if self.is_full.is_some() {
            len += 1;
        }
        if !self.contracts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.FetchChainRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if self.start_block != 0 {
            struct_ser.serialize_field("startBlock", ToString::to_string(&self.start_block).as_str())?;
        }
        if self.target_block != 0 {
            struct_ser.serialize_field("targetBlock", ToString::to_string(&self.target_block).as_str())?;
        }
        if let Some(v) = self.is_full.as_ref() {
            struct_ser.serialize_field("isFull", v)?;
        }
        if !self.contracts.is_empty() {
            struct_ser.serialize_field("contracts", &self.contracts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FetchChainRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "start_block",
            "startBlock",
            "target_block",
            "targetBlock",
            "is_full",
            "isFull",
            "contracts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            StartBlock,
            TargetBlock,
            IsFull,
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
                            "startBlock" | "start_block" => Ok(GeneratedField::StartBlock),
                            "targetBlock" | "target_block" => Ok(GeneratedField::TargetBlock),
                            "isFull" | "is_full" => Ok(GeneratedField::IsFull),
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
            type Value = FetchChainRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.FetchChainRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FetchChainRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut start_block__ = None;
                let mut target_block__ = None;
                let mut is_full__ = None;
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
                        GeneratedField::StartBlock => {
                            if start_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startBlock"));
                            }
                            start_block__ = 
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
                        GeneratedField::IsFull => {
                            if is_full__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isFull"));
                            }
                            is_full__ = map.next_value()?;
                        }
                        GeneratedField::Contracts => {
                            if contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contracts"));
                            }
                            contracts__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FetchChainRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    start_block: start_block__.unwrap_or_default(),
                    target_block: target_block__.unwrap_or_default(),
                    is_full: is_full__,
                    contracts: contracts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.FetchChainRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FetchChainResponse {
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
        if !self.contracts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.FetchChainResponse", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contracts.is_empty() {
            struct_ser.serialize_field("contracts", &self.contracts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FetchChainResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contracts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
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
            type Value = FetchChainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.FetchChainResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FetchChainResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
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
                        GeneratedField::Contracts => {
                            if contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contracts"));
                            }
                            contracts__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FetchChainResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                    contracts: contracts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.FetchChainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FetchContractRequest {
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
        if self.start_block.is_some() {
            len += 1;
        }
        if self.target_block.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.FetchContractRequest", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if let Some(v) = self.start_block.as_ref() {
            struct_ser.serialize_field("startBlock", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.target_block.as_ref() {
            struct_ser.serialize_field("targetBlock", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FetchContractRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "start_block",
            "startBlock",
            "target_block",
            "targetBlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            StartBlock,
            TargetBlock,
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
                            "startBlock" | "start_block" => Ok(GeneratedField::StartBlock),
                            "targetBlock" | "target_block" => Ok(GeneratedField::TargetBlock),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FetchContractRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.FetchContractRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FetchContractRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut start_block__ = None;
                let mut target_block__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StartBlock => {
                            if start_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startBlock"));
                            }
                            start_block__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TargetBlock => {
                            if target_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetBlock"));
                            }
                            target_block__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(FetchContractRequest {
                    contract_address: contract_address__.unwrap_or_default(),
                    start_block: start_block__,
                    target_block: target_block__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.FetchContractRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FetchContractResponse {
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
        if self.start_block != 0 {
            len += 1;
        }
        if self.end_block != 0 {
            len += 1;
        }
        if !self.commitments.is_empty() {
            len += 1;
        }
        if !self.nullifiers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.FetchContractResponse", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if self.start_block != 0 {
            struct_ser.serialize_field("startBlock", ToString::to_string(&self.start_block).as_str())?;
        }
        if self.end_block != 0 {
            struct_ser.serialize_field("endBlock", ToString::to_string(&self.end_block).as_str())?;
        }
        if !self.commitments.is_empty() {
            struct_ser.serialize_field("commitments", &self.commitments)?;
        }
        if !self.nullifiers.is_empty() {
            struct_ser.serialize_field("nullifiers", &self.nullifiers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FetchContractResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "start_block",
            "startBlock",
            "end_block",
            "endBlock",
            "commitments",
            "nullifiers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            StartBlock,
            EndBlock,
            Commitments,
            Nullifiers,
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
                            "startBlock" | "start_block" => Ok(GeneratedField::StartBlock),
                            "endBlock" | "end_block" => Ok(GeneratedField::EndBlock),
                            "commitments" => Ok(GeneratedField::Commitments),
                            "nullifiers" => Ok(GeneratedField::Nullifiers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FetchContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.FetchContractResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FetchContractResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut start_block__ = None;
                let mut end_block__ = None;
                let mut commitments__ = None;
                let mut nullifiers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StartBlock => {
                            if start_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startBlock"));
                            }
                            start_block__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EndBlock => {
                            if end_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endBlock"));
                            }
                            end_block__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Commitments => {
                            if commitments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitments"));
                            }
                            commitments__ = Some(map.next_value()?);
                        }
                        GeneratedField::Nullifiers => {
                            if nullifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullifiers"));
                            }
                            nullifiers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FetchContractResponse {
                    contract_address: contract_address__.unwrap_or_default(),
                    start_block: start_block__.unwrap_or_default(),
                    end_block: end_block__.unwrap_or_default(),
                    commitments: commitments__.unwrap_or_default(),
                    nullifiers: nullifiers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.FetchContractResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCommitmentHashesRequest {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.from_leaf_index.is_some() {
            len += 1;
        }
        if self.to_leaf_index.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.GetCommitmentHashesRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if let Some(v) = self.from_leaf_index.as_ref() {
            struct_ser.serialize_field("fromLeafIndex", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.to_leaf_index.as_ref() {
            struct_ser.serialize_field("toLeafIndex", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.status.as_ref() {
            let v = super::super::data::v1::CommitmentStatus::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCommitmentHashesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "from_leaf_index",
            "fromLeafIndex",
            "to_leaf_index",
            "toLeafIndex",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddress,
            FromLeafIndex,
            ToLeafIndex,
            Status,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "fromLeafIndex" | "from_leaf_index" => Ok(GeneratedField::FromLeafIndex),
                            "toLeafIndex" | "to_leaf_index" => Ok(GeneratedField::ToLeafIndex),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCommitmentHashesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.GetCommitmentHashesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetCommitmentHashesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_address__ = None;
                let mut from_leaf_index__ = None;
                let mut to_leaf_index__ = None;
                let mut status__ = None;
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
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FromLeafIndex => {
                            if from_leaf_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromLeafIndex"));
                            }
                            from_leaf_index__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ToLeafIndex => {
                            if to_leaf_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toLeafIndex"));
                            }
                            to_leaf_index__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value::<::std::option::Option<super::super::data::v1::CommitmentStatus>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(GetCommitmentHashesRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    from_leaf_index: from_leaf_index__,
                    to_leaf_index: to_leaf_index__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.GetCommitmentHashesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCommitmentHashesResponse {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.from_leaf_index != 0 {
            len += 1;
        }
        if self.to_leaf_index != 0 {
            len += 1;
        }
        if !self.commitment_hashes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.GetCommitmentHashesResponse", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if self.from_leaf_index != 0 {
            struct_ser.serialize_field("fromLeafIndex", ToString::to_string(&self.from_leaf_index).as_str())?;
        }
        if self.to_leaf_index != 0 {
            struct_ser.serialize_field("toLeafIndex", ToString::to_string(&self.to_leaf_index).as_str())?;
        }
        if !self.commitment_hashes.is_empty() {
            struct_ser.serialize_field("commitmentHashes", &self.commitment_hashes.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCommitmentHashesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "from_leaf_index",
            "fromLeafIndex",
            "to_leaf_index",
            "toLeafIndex",
            "commitment_hashes",
            "commitmentHashes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddress,
            FromLeafIndex,
            ToLeafIndex,
            CommitmentHashes,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "fromLeafIndex" | "from_leaf_index" => Ok(GeneratedField::FromLeafIndex),
                            "toLeafIndex" | "to_leaf_index" => Ok(GeneratedField::ToLeafIndex),
                            "commitmentHashes" | "commitment_hashes" => Ok(GeneratedField::CommitmentHashes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCommitmentHashesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.GetCommitmentHashesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetCommitmentHashesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_address__ = None;
                let mut from_leaf_index__ = None;
                let mut to_leaf_index__ = None;
                let mut commitment_hashes__ = None;
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
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FromLeafIndex => {
                            if from_leaf_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromLeafIndex"));
                            }
                            from_leaf_index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ToLeafIndex => {
                            if to_leaf_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toLeafIndex"));
                            }
                            to_leaf_index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CommitmentHashes => {
                            if commitment_hashes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitmentHashes"));
                            }
                            commitment_hashes__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(GetCommitmentHashesResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    from_leaf_index: from_leaf_index__.unwrap_or_default(),
                    to_leaf_index: to_leaf_index__.unwrap_or_default(),
                    commitment_hashes: commitment_hashes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.GetCommitmentHashesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCommitmentsByTxHashRequest {
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
        if !self.tx_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.GetCommitmentsByTxHashRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.tx_hash.is_empty() {
            struct_ser.serialize_field("txHash", pbjson::private::base64::encode(&self.tx_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCommitmentsByTxHashRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "tx_hash",
            "txHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            TxHash,
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
                            "txHash" | "tx_hash" => Ok(GeneratedField::TxHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCommitmentsByTxHashRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.GetCommitmentsByTxHashRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetCommitmentsByTxHashRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut tx_hash__ = None;
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
                        GeneratedField::TxHash => {
                            if tx_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHash"));
                            }
                            tx_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetCommitmentsByTxHashRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    tx_hash: tx_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.GetCommitmentsByTxHashRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCommitmentsByTxHashResponse {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.commitments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.GetCommitmentsByTxHashResponse", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if !self.commitments.is_empty() {
            struct_ser.serialize_field("commitments", &self.commitments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCommitmentsByTxHashResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "commitments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddress,
            Commitments,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "commitments" => Ok(GeneratedField::Commitments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCommitmentsByTxHashResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.GetCommitmentsByTxHashResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetCommitmentsByTxHashResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_address__ = None;
                let mut commitments__ = None;
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
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Commitments => {
                            if commitments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitments"));
                            }
                            commitments__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetCommitmentsByTxHashResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    commitments: commitments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.GetCommitmentsByTxHashResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCommitmentsRequest {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.commitment_hashes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.GetCommitmentsRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if !self.commitment_hashes.is_empty() {
            struct_ser.serialize_field("commitmentHashes", &self.commitment_hashes.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCommitmentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "commitment_hashes",
            "commitmentHashes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddress,
            CommitmentHashes,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "commitmentHashes" | "commitment_hashes" => Ok(GeneratedField::CommitmentHashes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCommitmentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.GetCommitmentsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetCommitmentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_address__ = None;
                let mut commitment_hashes__ = None;
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
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CommitmentHashes => {
                            if commitment_hashes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitmentHashes"));
                            }
                            commitment_hashes__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(GetCommitmentsRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    commitment_hashes: commitment_hashes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.GetCommitmentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCommitmentsResponse {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.commitments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.GetCommitmentsResponse", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if !self.commitments.is_empty() {
            struct_ser.serialize_field("commitments", &self.commitments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCommitmentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "commitments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddress,
            Commitments,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "commitments" => Ok(GeneratedField::Commitments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCommitmentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.GetCommitmentsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetCommitmentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_address__ = None;
                let mut commitments__ = None;
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
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Commitments => {
                            if commitments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitments"));
                            }
                            commitments__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetCommitmentsResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    commitments: commitments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.GetCommitmentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetNullifiersByTxHashRequest {
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
        if !self.tx_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.GetNullifiersByTxHashRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.tx_hash.is_empty() {
            struct_ser.serialize_field("txHash", pbjson::private::base64::encode(&self.tx_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetNullifiersByTxHashRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "tx_hash",
            "txHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            TxHash,
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
                            "txHash" | "tx_hash" => Ok(GeneratedField::TxHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetNullifiersByTxHashRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.GetNullifiersByTxHashRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetNullifiersByTxHashRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut tx_hash__ = None;
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
                        GeneratedField::TxHash => {
                            if tx_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHash"));
                            }
                            tx_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetNullifiersByTxHashRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    tx_hash: tx_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.GetNullifiersByTxHashRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetNullifiersByTxHashResponse {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.nullifiers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.GetNullifiersByTxHashResponse", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if !self.nullifiers.is_empty() {
            struct_ser.serialize_field("nullifiers", &self.nullifiers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetNullifiersByTxHashResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "nullifiers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddress,
            Nullifiers,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "nullifiers" => Ok(GeneratedField::Nullifiers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetNullifiersByTxHashResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.GetNullifiersByTxHashResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetNullifiersByTxHashResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_address__ = None;
                let mut nullifiers__ = None;
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
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nullifiers => {
                            if nullifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullifiers"));
                            }
                            nullifiers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetNullifiersByTxHashResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    nullifiers: nullifiers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.GetNullifiersByTxHashResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetNullifiersRequest {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.nullifier_hashes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.GetNullifiersRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if !self.nullifier_hashes.is_empty() {
            struct_ser.serialize_field("nullifierHashes", &self.nullifier_hashes.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetNullifiersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "nullifier_hashes",
            "nullifierHashes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddress,
            NullifierHashes,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "nullifierHashes" | "nullifier_hashes" => Ok(GeneratedField::NullifierHashes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetNullifiersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.GetNullifiersRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetNullifiersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_address__ = None;
                let mut nullifier_hashes__ = None;
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
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NullifierHashes => {
                            if nullifier_hashes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullifierHashes"));
                            }
                            nullifier_hashes__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(GetNullifiersRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    nullifier_hashes: nullifier_hashes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.GetNullifiersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetNullifiersResponse {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.nullifiers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.GetNullifiersResponse", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
        }
        if !self.nullifiers.is_empty() {
            struct_ser.serialize_field("nullifiers", &self.nullifiers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetNullifiersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "nullifiers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ContractAddress,
            Nullifiers,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "nullifiers" => Ok(GeneratedField::Nullifiers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetNullifiersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.GetNullifiersResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetNullifiersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut contract_address__ = None;
                let mut nullifiers__ = None;
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
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nullifiers => {
                            if nullifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullifiers"));
                            }
                            nullifiers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetNullifiersResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    nullifiers: nullifiers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.GetNullifiersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.HealthCheckRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckRequest {
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
            type Value = HealthCheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.HealthCheckRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheckRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HealthCheckRequest {
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.HealthCheckRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mystiko.sequencer.v1.HealthCheckResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckResponse {
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
            type Value = HealthCheckResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.sequencer.v1.HealthCheckResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheckResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HealthCheckResponse {
                })
            }
        }
        deserializer.deserialize_struct("mystiko.sequencer.v1.HealthCheckResponse", FIELDS, GeneratedVisitor)
    }
}
