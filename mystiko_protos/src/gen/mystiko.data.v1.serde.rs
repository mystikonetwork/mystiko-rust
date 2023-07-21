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
        if self.leaf_index.is_some() {
            len += 1;
        }
        if self.rollup_fee.is_some() {
            len += 1;
        }
        if self.encrypted_note.is_some() {
            len += 1;
        }
        if self.creation_transaction_hash.is_some() {
            len += 1;
        }
        if self.rollup_transaction_hash.is_some() {
            len += 1;
        }
        if self.relay_transaction_hash.is_some() {
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
        if let Some(v) = self.leaf_index.as_ref() {
            struct_ser.serialize_field("leafIndex", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.rollup_fee.as_ref() {
            struct_ser.serialize_field("rollupFee", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.encrypted_note.as_ref() {
            struct_ser.serialize_field("encryptedNote", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.creation_transaction_hash.as_ref() {
            struct_ser.serialize_field("creationTransactionHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.rollup_transaction_hash.as_ref() {
            struct_ser.serialize_field("rollupTransactionHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.relay_transaction_hash.as_ref() {
            struct_ser.serialize_field("relayTransactionHash", pbjson::private::base64::encode(&v).as_str())?;
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
            "leaf_index",
            "leafIndex",
            "rollup_fee",
            "rollupFee",
            "encrypted_note",
            "encryptedNote",
            "creation_transaction_hash",
            "creationTransactionHash",
            "rollup_transaction_hash",
            "rollupTransactionHash",
            "relay_transaction_hash",
            "relayTransactionHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommitmentHash,
            Status,
            LeafIndex,
            RollupFee,
            EncryptedNote,
            CreationTransactionHash,
            RollupTransactionHash,
            RelayTransactionHash,
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
                            "leafIndex" | "leaf_index" => Ok(GeneratedField::LeafIndex),
                            "rollupFee" | "rollup_fee" => Ok(GeneratedField::RollupFee),
                            "encryptedNote" | "encrypted_note" => Ok(GeneratedField::EncryptedNote),
                            "creationTransactionHash" | "creation_transaction_hash" => Ok(GeneratedField::CreationTransactionHash),
                            "rollupTransactionHash" | "rollup_transaction_hash" => Ok(GeneratedField::RollupTransactionHash),
                            "relayTransactionHash" | "relay_transaction_hash" => Ok(GeneratedField::RelayTransactionHash),
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
                let mut leaf_index__ = None;
                let mut rollup_fee__ = None;
                let mut encrypted_note__ = None;
                let mut creation_transaction_hash__ = None;
                let mut rollup_transaction_hash__ = None;
                let mut relay_transaction_hash__ = None;
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
                        GeneratedField::CreationTransactionHash => {
                            if creation_transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTransactionHash"));
                            }
                            creation_transaction_hash__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RollupTransactionHash => {
                            if rollup_transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupTransactionHash"));
                            }
                            rollup_transaction_hash__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RelayTransactionHash => {
                            if relay_transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayTransactionHash"));
                            }
                            relay_transaction_hash__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Commitment {
                    commitment_hash: commitment_hash__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    leaf_index: leaf_index__,
                    rollup_fee: rollup_fee__,
                    encrypted_note: encrypted_note__,
                    creation_transaction_hash: creation_transaction_hash__,
                    rollup_transaction_hash: rollup_transaction_hash__,
                    relay_transaction_hash: relay_transaction_hash__,
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
        if !self.transaction_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.data.v1.Nullifier", len)?;
        if !self.nullifier.is_empty() {
            struct_ser.serialize_field("nullifier", pbjson::private::base64::encode(&self.nullifier).as_str())?;
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
            "transaction_hash",
            "transactionHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nullifier,
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
                    transaction_hash: transaction_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.data.v1.Nullifier", FIELDS, GeneratedVisitor)
    }
}
