// @generated
impl serde::Serialize for ChainData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_block != 0 {
            len += 1;
        }
        if self.end_block != 0 {
            len += 1;
        }
        if !self.contract_data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.data.v1.ChainData", len)?;
        if self.start_block != 0 {
            struct_ser.serialize_field("startBlock", ToString::to_string(&self.start_block).as_str())?;
        }
        if self.end_block != 0 {
            struct_ser.serialize_field("endBlock", ToString::to_string(&self.end_block).as_str())?;
        }
        if !self.contract_data.is_empty() {
            struct_ser.serialize_field("contractData", &self.contract_data)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_block",
            "startBlock",
            "end_block",
            "endBlock",
            "contract_data",
            "contractData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartBlock,
            EndBlock,
            ContractData,
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
                            "startBlock" | "start_block" => Ok(GeneratedField::StartBlock),
                            "endBlock" | "end_block" => Ok(GeneratedField::EndBlock),
                            "contractData" | "contract_data" => Ok(GeneratedField::ContractData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.data.v1.ChainData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChainData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_block__ = None;
                let mut end_block__ = None;
                let mut contract_data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::ContractData => {
                            if contract_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractData"));
                            }
                            contract_data__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ChainData {
                    start_block: start_block__.unwrap_or_default(),
                    end_block: end_block__.unwrap_or_default(),
                    contract_data: contract_data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.data.v1.ChainData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Commitment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.commitment_hash.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.block_number != 0 {
            len += 1;
        }
        if self.included_block_number.is_some() {
            len += 1;
        }
        if self.src_chain_block_number.is_some() {
            len += 1;
        }
        if self.leaf_index.is_some() {
            len += 1;
        }
        if self.rollup_fee.is_some() {
            len += 1;
        }
        if self.encrypted_note.is_some() {
            len += 1;
        }
        if self.queued_transaction_hash.is_some() {
            len += 1;
        }
        if self.included_transaction_hash.is_some() {
            len += 1;
        }
        if self.src_chain_transaction_hash.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.data.v1.Commitment", len)?;
        if !self.commitment_hash.is_empty() {
            struct_ser.serialize_field("commitmentHash", pbjson::private::base64::encode(&self.commitment_hash).as_str())?;
        }
        if self.status != 0 {
            let v = CommitmentStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if self.block_number != 0 {
            struct_ser.serialize_field("blockNumber", ToString::to_string(&self.block_number).as_str())?;
        }
        if let Some(v) = self.included_block_number.as_ref() {
            struct_ser.serialize_field("includedBlockNumber", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.src_chain_block_number.as_ref() {
            struct_ser.serialize_field("srcChainBlockNumber", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.leaf_index.as_ref() {
            struct_ser.serialize_field("leafIndex", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.rollup_fee.as_ref() {
            struct_ser.serialize_field("rollupFee", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.encrypted_note.as_ref() {
            struct_ser.serialize_field("encryptedNote", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.queued_transaction_hash.as_ref() {
            struct_ser.serialize_field("queuedTransactionHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.included_transaction_hash.as_ref() {
            struct_ser.serialize_field("includedTransactionHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.src_chain_transaction_hash.as_ref() {
            struct_ser.serialize_field("srcChainTransactionHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Commitment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commitment_hash",
            "commitmentHash",
            "status",
            "block_number",
            "blockNumber",
            "included_block_number",
            "includedBlockNumber",
            "src_chain_block_number",
            "srcChainBlockNumber",
            "leaf_index",
            "leafIndex",
            "rollup_fee",
            "rollupFee",
            "encrypted_note",
            "encryptedNote",
            "queued_transaction_hash",
            "queuedTransactionHash",
            "included_transaction_hash",
            "includedTransactionHash",
            "src_chain_transaction_hash",
            "srcChainTransactionHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommitmentHash,
            Status,
            BlockNumber,
            IncludedBlockNumber,
            SrcChainBlockNumber,
            LeafIndex,
            RollupFee,
            EncryptedNote,
            QueuedTransactionHash,
            IncludedTransactionHash,
            SrcChainTransactionHash,
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
                            "commitmentHash" | "commitment_hash" => Ok(GeneratedField::CommitmentHash),
                            "status" => Ok(GeneratedField::Status),
                            "blockNumber" | "block_number" => Ok(GeneratedField::BlockNumber),
                            "includedBlockNumber" | "included_block_number" => Ok(GeneratedField::IncludedBlockNumber),
                            "srcChainBlockNumber" | "src_chain_block_number" => Ok(GeneratedField::SrcChainBlockNumber),
                            "leafIndex" | "leaf_index" => Ok(GeneratedField::LeafIndex),
                            "rollupFee" | "rollup_fee" => Ok(GeneratedField::RollupFee),
                            "encryptedNote" | "encrypted_note" => Ok(GeneratedField::EncryptedNote),
                            "queuedTransactionHash" | "queued_transaction_hash" => Ok(GeneratedField::QueuedTransactionHash),
                            "includedTransactionHash" | "included_transaction_hash" => Ok(GeneratedField::IncludedTransactionHash),
                            "srcChainTransactionHash" | "src_chain_transaction_hash" => Ok(GeneratedField::SrcChainTransactionHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Commitment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.data.v1.Commitment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Commitment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut commitment_hash__ = None;
                let mut status__ = None;
                let mut block_number__ = None;
                let mut included_block_number__ = None;
                let mut src_chain_block_number__ = None;
                let mut leaf_index__ = None;
                let mut rollup_fee__ = None;
                let mut encrypted_note__ = None;
                let mut queued_transaction_hash__ = None;
                let mut included_transaction_hash__ = None;
                let mut src_chain_transaction_hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommitmentHash => {
                            if commitment_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitmentHash"));
                            }
                            commitment_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<CommitmentStatus>()? as i32);
                        }
                        GeneratedField::BlockNumber => {
                            if block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockNumber"));
                            }
                            block_number__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IncludedBlockNumber => {
                            if included_block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includedBlockNumber"));
                            }
                            included_block_number__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SrcChainBlockNumber => {
                            if src_chain_block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srcChainBlockNumber"));
                            }
                            src_chain_block_number__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LeafIndex => {
                            if leaf_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leafIndex"));
                            }
                            leaf_index__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RollupFee => {
                            if rollup_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFee"));
                            }
                            rollup_fee__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::EncryptedNote => {
                            if encrypted_note__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encryptedNote"));
                            }
                            encrypted_note__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::QueuedTransactionHash => {
                            if queued_transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queuedTransactionHash"));
                            }
                            queued_transaction_hash__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IncludedTransactionHash => {
                            if included_transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includedTransactionHash"));
                            }
                            included_transaction_hash__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SrcChainTransactionHash => {
                            if src_chain_transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srcChainTransactionHash"));
                            }
                            src_chain_transaction_hash__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Commitment {
                    commitment_hash: commitment_hash__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    block_number: block_number__.unwrap_or_default(),
                    included_block_number: included_block_number__,
                    src_chain_block_number: src_chain_block_number__,
                    leaf_index: leaf_index__,
                    rollup_fee: rollup_fee__,
                    encrypted_note: encrypted_note__,
                    queued_transaction_hash: queued_transaction_hash__,
                    included_transaction_hash: included_transaction_hash__,
                    src_chain_transaction_hash: src_chain_transaction_hash__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.data.v1.Commitment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommitmentStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "COMMITMENT_STATUS_UNSPECIFIED",
            Self::SrcSucceeded => "COMMITMENT_STATUS_SRC_SUCCEEDED",
            Self::Queued => "COMMITMENT_STATUS_QUEUED",
            Self::Included => "COMMITMENT_STATUS_INCLUDED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CommitmentStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COMMITMENT_STATUS_UNSPECIFIED",
            "COMMITMENT_STATUS_SRC_SUCCEEDED",
            "COMMITMENT_STATUS_QUEUED",
            "COMMITMENT_STATUS_INCLUDED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommitmentStatus;

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
                    .and_then(CommitmentStatus::from_i32)
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
                    .and_then(CommitmentStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "COMMITMENT_STATUS_UNSPECIFIED" => Ok(CommitmentStatus::Unspecified),
                    "COMMITMENT_STATUS_SRC_SUCCEEDED" => Ok(CommitmentStatus::SrcSucceeded),
                    "COMMITMENT_STATUS_QUEUED" => Ok(CommitmentStatus::Queued),
                    "COMMITMENT_STATUS_INCLUDED" => Ok(CommitmentStatus::Included),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ContractData {
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
        if !self.commitments.is_empty() {
            len += 1;
        }
        if !self.nullifiers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.data.v1.ContractData", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", pbjson::private::base64::encode(&self.contract_address).as_str())?;
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
impl<'de> serde::Deserialize<'de> for ContractData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "commitments",
            "nullifiers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
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
            type Value = ContractData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.data.v1.ContractData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContractData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
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
                Ok(ContractData {
                    contract_address: contract_address__.unwrap_or_default(),
                    commitments: commitments__.unwrap_or_default(),
                    nullifiers: nullifiers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.data.v1.ContractData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MerkleTree {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.root_hash.is_empty() {
            len += 1;
        }
        if !self.commitment_hashes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.data.v1.MerkleTree", len)?;
        if !self.root_hash.is_empty() {
            struct_ser.serialize_field("rootHash", pbjson::private::base64::encode(&self.root_hash).as_str())?;
        }
        if !self.commitment_hashes.is_empty() {
            struct_ser.serialize_field("commitmentHashes", &self.commitment_hashes.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleTree {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "root_hash",
            "rootHash",
            "commitment_hashes",
            "commitmentHashes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RootHash,
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
                            "rootHash" | "root_hash" => Ok(GeneratedField::RootHash),
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
            type Value = MerkleTree;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.data.v1.MerkleTree")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MerkleTree, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut root_hash__ = None;
                let mut commitment_hashes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RootHash => {
                            if root_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rootHash"));
                            }
                            root_hash__ = 
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
                Ok(MerkleTree {
                    root_hash: root_hash__.unwrap_or_default(),
                    commitment_hashes: commitment_hashes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.data.v1.MerkleTree", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Nullifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nullifier.is_empty() {
            len += 1;
        }
        if self.block_number != 0 {
            len += 1;
        }
        if !self.transaction_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.data.v1.Nullifier", len)?;
        if !self.nullifier.is_empty() {
            struct_ser.serialize_field("nullifier", pbjson::private::base64::encode(&self.nullifier).as_str())?;
        }
        if self.block_number != 0 {
            struct_ser.serialize_field("blockNumber", ToString::to_string(&self.block_number).as_str())?;
        }
        if !self.transaction_hash.is_empty() {
            struct_ser.serialize_field("transactionHash", pbjson::private::base64::encode(&self.transaction_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Nullifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nullifier",
            "block_number",
            "blockNumber",
            "transaction_hash",
            "transactionHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nullifier,
            BlockNumber,
            TransactionHash,
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
                            "nullifier" => Ok(GeneratedField::Nullifier),
                            "blockNumber" | "block_number" => Ok(GeneratedField::BlockNumber),
                            "transactionHash" | "transaction_hash" => Ok(GeneratedField::TransactionHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Nullifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.data.v1.Nullifier")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Nullifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nullifier__ = None;
                let mut block_number__ = None;
                let mut transaction_hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Nullifier => {
                            if nullifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullifier"));
                            }
                            nullifier__ = 
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
                        GeneratedField::TransactionHash => {
                            if transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionHash"));
                            }
                            transaction_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Nullifier {
                    nullifier: nullifier__.unwrap_or_default(),
                    block_number: block_number__.unwrap_or_default(),
                    transaction_hash: transaction_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.data.v1.Nullifier", FIELDS, GeneratedVisitor)
    }
}
