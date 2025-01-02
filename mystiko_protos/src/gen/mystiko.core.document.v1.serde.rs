// @generated
impl serde::Serialize for Account {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.shielded_address.is_empty() {
            len += 1;
        }
        if !self.public_key.is_empty() {
            len += 1;
        }
        if !self.encrypted_secret_key.is_empty() {
            len += 1;
        }
        if !self.wallet_id.is_empty() {
            len += 1;
        }
        if self.scanned_to_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.document.v1.Account", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.created_at != 0 {
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.shielded_address.is_empty() {
            struct_ser.serialize_field("shieldedAddress", &self.shielded_address)?;
        }
        if !self.public_key.is_empty() {
            struct_ser.serialize_field("publicKey", &self.public_key)?;
        }
        if !self.encrypted_secret_key.is_empty() {
            struct_ser.serialize_field("encryptedSecretKey", &self.encrypted_secret_key)?;
        }
        if !self.wallet_id.is_empty() {
            struct_ser.serialize_field("walletId", &self.wallet_id)?;
        }
        if let Some(v) = self.scanned_to_id.as_ref() {
            struct_ser.serialize_field("scannedToId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Account {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "name",
            "shielded_address",
            "shieldedAddress",
            "public_key",
            "publicKey",
            "encrypted_secret_key",
            "encryptedSecretKey",
            "wallet_id",
            "walletId",
            "scanned_to_id",
            "scannedToId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreatedAt,
            UpdatedAt,
            Name,
            ShieldedAddress,
            PublicKey,
            EncryptedSecretKey,
            WalletId,
            ScannedToId,
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
                            "id" => Ok(GeneratedField::Id),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "name" => Ok(GeneratedField::Name),
                            "shieldedAddress" | "shielded_address" => Ok(GeneratedField::ShieldedAddress),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "encryptedSecretKey" | "encrypted_secret_key" => Ok(GeneratedField::EncryptedSecretKey),
                            "walletId" | "wallet_id" => Ok(GeneratedField::WalletId),
                            "scannedToId" | "scanned_to_id" => Ok(GeneratedField::ScannedToId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Account;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.document.v1.Account")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Account, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut name__ = None;
                let mut shielded_address__ = None;
                let mut public_key__ = None;
                let mut encrypted_secret_key__ = None;
                let mut wallet_id__ = None;
                let mut scanned_to_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ShieldedAddress => {
                            if shielded_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddress"));
                            }
                            shielded_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::EncryptedSecretKey => {
                            if encrypted_secret_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encryptedSecretKey"));
                            }
                            encrypted_secret_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::WalletId => {
                            if wallet_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletId"));
                            }
                            wallet_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ScannedToId => {
                            if scanned_to_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scannedToId"));
                            }
                            scanned_to_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(Account {
                    id: id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    shielded_address: shielded_address__.unwrap_or_default(),
                    public_key: public_key__.unwrap_or_default(),
                    encrypted_secret_key: encrypted_secret_key__.unwrap_or_default(),
                    wallet_id: wallet_id__.unwrap_or_default(),
                    scanned_to_id: scanned_to_id__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.document.v1.Account", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Deposit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if self.chain_id != 0 {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.pool_address.is_empty() {
            len += 1;
        }
        if self.dst_chain_id != 0 {
            len += 1;
        }
        if !self.dst_chain_contract_address.is_empty() {
            len += 1;
        }
        if !self.dst_pool_address.is_empty() {
            len += 1;
        }
        if !self.commitment_hash.is_empty() {
            len += 1;
        }
        if !self.hash_k.is_empty() {
            len += 1;
        }
        if !self.random_s.is_empty() {
            len += 1;
        }
        if !self.encrypted_note.is_empty() {
            len += 1;
        }
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if self.asset_decimals != 0 {
            len += 1;
        }
        if self.amount != 0. {
            len += 1;
        }
        if !self.decimal_amount.is_empty() {
            len += 1;
        }
        if self.rollup_fee_amount != 0. {
            len += 1;
        }
        if !self.rollup_fee_decimal_amount.is_empty() {
            len += 1;
        }
        if !self.shielded_address.is_empty() {
            len += 1;
        }
        if !self.wallet_id.is_empty() {
            len += 1;
        }
        if self.asset_address.is_some() {
            len += 1;
        }
        if self.bridge_fee_amount.is_some() {
            len += 1;
        }
        if self.bridge_fee_decimal_amount.is_some() {
            len += 1;
        }
        if self.executor_fee_amount.is_some() {
            len += 1;
        }
        if self.executor_fee_decimal_amount.is_some() {
            len += 1;
        }
        if self.bridge_fee_asset_address.is_some() {
            len += 1;
        }
        if self.bridge_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.bridge_fee_asset_decimals.is_some() {
            len += 1;
        }
        if self.executor_fee_asset_address.is_some() {
            len += 1;
        }
        if self.executor_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.executor_fee_asset_decimals.is_some() {
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
        if !self.asset_approve_transaction_hash.is_empty() {
            len += 1;
        }
        if self.error_message.is_some() {
            len += 1;
        }
        if self.bridge_type != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.document.v1.Deposit", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.created_at != 0 {
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if !self.pool_address.is_empty() {
            struct_ser.serialize_field("poolAddress", &self.pool_address)?;
        }
        if self.dst_chain_id != 0 {
            struct_ser.serialize_field("dstChainId", ToString::to_string(&self.dst_chain_id).as_str())?;
        }
        if !self.dst_chain_contract_address.is_empty() {
            struct_ser.serialize_field("dstChainContractAddress", &self.dst_chain_contract_address)?;
        }
        if !self.dst_pool_address.is_empty() {
            struct_ser.serialize_field("dstPoolAddress", &self.dst_pool_address)?;
        }
        if !self.commitment_hash.is_empty() {
            struct_ser.serialize_field("commitmentHash", &self.commitment_hash)?;
        }
        if !self.hash_k.is_empty() {
            struct_ser.serialize_field("hashK", &self.hash_k)?;
        }
        if !self.random_s.is_empty() {
            struct_ser.serialize_field("randomS", &self.random_s)?;
        }
        if !self.encrypted_note.is_empty() {
            struct_ser.serialize_field("encryptedNote", &self.encrypted_note)?;
        }
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.asset_decimals != 0 {
            struct_ser.serialize_field("assetDecimals", &self.asset_decimals)?;
        }
        if self.amount != 0. {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.decimal_amount.is_empty() {
            struct_ser.serialize_field("decimalAmount", &self.decimal_amount)?;
        }
        if self.rollup_fee_amount != 0. {
            struct_ser.serialize_field("rollupFeeAmount", &self.rollup_fee_amount)?;
        }
        if !self.rollup_fee_decimal_amount.is_empty() {
            struct_ser.serialize_field("rollupFeeDecimalAmount", &self.rollup_fee_decimal_amount)?;
        }
        if !self.shielded_address.is_empty() {
            struct_ser.serialize_field("shieldedAddress", &self.shielded_address)?;
        }
        if !self.wallet_id.is_empty() {
            struct_ser.serialize_field("walletId", &self.wallet_id)?;
        }
        if let Some(v) = self.asset_address.as_ref() {
            struct_ser.serialize_field("assetAddress", v)?;
        }
        if let Some(v) = self.bridge_fee_amount.as_ref() {
            struct_ser.serialize_field("bridgeFeeAmount", v)?;
        }
        if let Some(v) = self.bridge_fee_decimal_amount.as_ref() {
            struct_ser.serialize_field("bridgeFeeDecimalAmount", v)?;
        }
        if let Some(v) = self.executor_fee_amount.as_ref() {
            struct_ser.serialize_field("executorFeeAmount", v)?;
        }
        if let Some(v) = self.executor_fee_decimal_amount.as_ref() {
            struct_ser.serialize_field("executorFeeDecimalAmount", v)?;
        }
        if let Some(v) = self.bridge_fee_asset_address.as_ref() {
            struct_ser.serialize_field("bridgeFeeAssetAddress", v)?;
        }
        if let Some(v) = self.bridge_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("bridgeFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.bridge_fee_asset_decimals.as_ref() {
            struct_ser.serialize_field("bridgeFeeAssetDecimals", v)?;
        }
        if let Some(v) = self.executor_fee_asset_address.as_ref() {
            struct_ser.serialize_field("executorFeeAssetAddress", v)?;
        }
        if let Some(v) = self.executor_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("executorFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.executor_fee_asset_decimals.as_ref() {
            struct_ser.serialize_field("executorFeeAssetDecimals", v)?;
        }
        if let Some(v) = self.queued_transaction_hash.as_ref() {
            struct_ser.serialize_field("queuedTransactionHash", v)?;
        }
        if let Some(v) = self.included_transaction_hash.as_ref() {
            struct_ser.serialize_field("includedTransactionHash", v)?;
        }
        if let Some(v) = self.src_chain_transaction_hash.as_ref() {
            struct_ser.serialize_field("srcChainTransactionHash", v)?;
        }
        if !self.asset_approve_transaction_hash.is_empty() {
            struct_ser.serialize_field("assetApproveTransactionHash", &self.asset_approve_transaction_hash)?;
        }
        if let Some(v) = self.error_message.as_ref() {
            struct_ser.serialize_field("errorMessage", v)?;
        }
        if self.bridge_type != 0 {
            let v = super::super::super::common::v1::BridgeType::from_i32(self.bridge_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.bridge_type)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        if self.status != 0 {
            let v = super::super::v1::DepositStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Deposit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "pool_address",
            "poolAddress",
            "dst_chain_id",
            "dstChainId",
            "dst_chain_contract_address",
            "dstChainContractAddress",
            "dst_pool_address",
            "dstPoolAddress",
            "commitment_hash",
            "commitmentHash",
            "hash_k",
            "hashK",
            "random_s",
            "randomS",
            "encrypted_note",
            "encryptedNote",
            "asset_symbol",
            "assetSymbol",
            "asset_decimals",
            "assetDecimals",
            "amount",
            "decimal_amount",
            "decimalAmount",
            "rollup_fee_amount",
            "rollupFeeAmount",
            "rollup_fee_decimal_amount",
            "rollupFeeDecimalAmount",
            "shielded_address",
            "shieldedAddress",
            "wallet_id",
            "walletId",
            "asset_address",
            "assetAddress",
            "bridge_fee_amount",
            "bridgeFeeAmount",
            "bridge_fee_decimal_amount",
            "bridgeFeeDecimalAmount",
            "executor_fee_amount",
            "executorFeeAmount",
            "executor_fee_decimal_amount",
            "executorFeeDecimalAmount",
            "bridge_fee_asset_address",
            "bridgeFeeAssetAddress",
            "bridge_fee_asset_symbol",
            "bridgeFeeAssetSymbol",
            "bridge_fee_asset_decimals",
            "bridgeFeeAssetDecimals",
            "executor_fee_asset_address",
            "executorFeeAssetAddress",
            "executor_fee_asset_symbol",
            "executorFeeAssetSymbol",
            "executor_fee_asset_decimals",
            "executorFeeAssetDecimals",
            "queued_transaction_hash",
            "queuedTransactionHash",
            "included_transaction_hash",
            "includedTransactionHash",
            "src_chain_transaction_hash",
            "srcChainTransactionHash",
            "asset_approve_transaction_hash",
            "assetApproveTransactionHash",
            "error_message",
            "errorMessage",
            "bridge_type",
            "bridgeType",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreatedAt,
            UpdatedAt,
            ChainId,
            ContractAddress,
            PoolAddress,
            DstChainId,
            DstChainContractAddress,
            DstPoolAddress,
            CommitmentHash,
            HashK,
            RandomS,
            EncryptedNote,
            AssetSymbol,
            AssetDecimals,
            Amount,
            DecimalAmount,
            RollupFeeAmount,
            RollupFeeDecimalAmount,
            ShieldedAddress,
            WalletId,
            AssetAddress,
            BridgeFeeAmount,
            BridgeFeeDecimalAmount,
            ExecutorFeeAmount,
            ExecutorFeeDecimalAmount,
            BridgeFeeAssetAddress,
            BridgeFeeAssetSymbol,
            BridgeFeeAssetDecimals,
            ExecutorFeeAssetAddress,
            ExecutorFeeAssetSymbol,
            ExecutorFeeAssetDecimals,
            QueuedTransactionHash,
            IncludedTransactionHash,
            SrcChainTransactionHash,
            AssetApproveTransactionHash,
            ErrorMessage,
            BridgeType,
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
                            "id" => Ok(GeneratedField::Id),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "poolAddress" | "pool_address" => Ok(GeneratedField::PoolAddress),
                            "dstChainId" | "dst_chain_id" => Ok(GeneratedField::DstChainId),
                            "dstChainContractAddress" | "dst_chain_contract_address" => Ok(GeneratedField::DstChainContractAddress),
                            "dstPoolAddress" | "dst_pool_address" => Ok(GeneratedField::DstPoolAddress),
                            "commitmentHash" | "commitment_hash" => Ok(GeneratedField::CommitmentHash),
                            "hashK" | "hash_k" => Ok(GeneratedField::HashK),
                            "randomS" | "random_s" => Ok(GeneratedField::RandomS),
                            "encryptedNote" | "encrypted_note" => Ok(GeneratedField::EncryptedNote),
                            "assetSymbol" | "asset_symbol" => Ok(GeneratedField::AssetSymbol),
                            "assetDecimals" | "asset_decimals" => Ok(GeneratedField::AssetDecimals),
                            "amount" => Ok(GeneratedField::Amount),
                            "decimalAmount" | "decimal_amount" => Ok(GeneratedField::DecimalAmount),
                            "rollupFeeAmount" | "rollup_fee_amount" => Ok(GeneratedField::RollupFeeAmount),
                            "rollupFeeDecimalAmount" | "rollup_fee_decimal_amount" => Ok(GeneratedField::RollupFeeDecimalAmount),
                            "shieldedAddress" | "shielded_address" => Ok(GeneratedField::ShieldedAddress),
                            "walletId" | "wallet_id" => Ok(GeneratedField::WalletId),
                            "assetAddress" | "asset_address" => Ok(GeneratedField::AssetAddress),
                            "bridgeFeeAmount" | "bridge_fee_amount" => Ok(GeneratedField::BridgeFeeAmount),
                            "bridgeFeeDecimalAmount" | "bridge_fee_decimal_amount" => Ok(GeneratedField::BridgeFeeDecimalAmount),
                            "executorFeeAmount" | "executor_fee_amount" => Ok(GeneratedField::ExecutorFeeAmount),
                            "executorFeeDecimalAmount" | "executor_fee_decimal_amount" => Ok(GeneratedField::ExecutorFeeDecimalAmount),
                            "bridgeFeeAssetAddress" | "bridge_fee_asset_address" => Ok(GeneratedField::BridgeFeeAssetAddress),
                            "bridgeFeeAssetSymbol" | "bridge_fee_asset_symbol" => Ok(GeneratedField::BridgeFeeAssetSymbol),
                            "bridgeFeeAssetDecimals" | "bridge_fee_asset_decimals" => Ok(GeneratedField::BridgeFeeAssetDecimals),
                            "executorFeeAssetAddress" | "executor_fee_asset_address" => Ok(GeneratedField::ExecutorFeeAssetAddress),
                            "executorFeeAssetSymbol" | "executor_fee_asset_symbol" => Ok(GeneratedField::ExecutorFeeAssetSymbol),
                            "executorFeeAssetDecimals" | "executor_fee_asset_decimals" => Ok(GeneratedField::ExecutorFeeAssetDecimals),
                            "queuedTransactionHash" | "queued_transaction_hash" => Ok(GeneratedField::QueuedTransactionHash),
                            "includedTransactionHash" | "included_transaction_hash" => Ok(GeneratedField::IncludedTransactionHash),
                            "srcChainTransactionHash" | "src_chain_transaction_hash" => Ok(GeneratedField::SrcChainTransactionHash),
                            "assetApproveTransactionHash" | "asset_approve_transaction_hash" => Ok(GeneratedField::AssetApproveTransactionHash),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
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
            type Value = Deposit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.document.v1.Deposit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Deposit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut chain_id__ = None;
                let mut contract_address__ = None;
                let mut pool_address__ = None;
                let mut dst_chain_id__ = None;
                let mut dst_chain_contract_address__ = None;
                let mut dst_pool_address__ = None;
                let mut commitment_hash__ = None;
                let mut hash_k__ = None;
                let mut random_s__ = None;
                let mut encrypted_note__ = None;
                let mut asset_symbol__ = None;
                let mut asset_decimals__ = None;
                let mut amount__ = None;
                let mut decimal_amount__ = None;
                let mut rollup_fee_amount__ = None;
                let mut rollup_fee_decimal_amount__ = None;
                let mut shielded_address__ = None;
                let mut wallet_id__ = None;
                let mut asset_address__ = None;
                let mut bridge_fee_amount__ = None;
                let mut bridge_fee_decimal_amount__ = None;
                let mut executor_fee_amount__ = None;
                let mut executor_fee_decimal_amount__ = None;
                let mut bridge_fee_asset_address__ = None;
                let mut bridge_fee_asset_symbol__ = None;
                let mut bridge_fee_asset_decimals__ = None;
                let mut executor_fee_asset_address__ = None;
                let mut executor_fee_asset_symbol__ = None;
                let mut executor_fee_asset_decimals__ = None;
                let mut queued_transaction_hash__ = None;
                let mut included_transaction_hash__ = None;
                let mut src_chain_transaction_hash__ = None;
                let mut asset_approve_transaction_hash__ = None;
                let mut error_message__ = None;
                let mut bridge_type__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
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
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::PoolAddress => {
                            if pool_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolAddress"));
                            }
                            pool_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::DstChainId => {
                            if dst_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dstChainId"));
                            }
                            dst_chain_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DstChainContractAddress => {
                            if dst_chain_contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dstChainContractAddress"));
                            }
                            dst_chain_contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::DstPoolAddress => {
                            if dst_pool_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dstPoolAddress"));
                            }
                            dst_pool_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::CommitmentHash => {
                            if commitment_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitmentHash"));
                            }
                            commitment_hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::HashK => {
                            if hash_k__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashK"));
                            }
                            hash_k__ = Some(map.next_value()?);
                        }
                        GeneratedField::RandomS => {
                            if random_s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("randomS"));
                            }
                            random_s__ = Some(map.next_value()?);
                        }
                        GeneratedField::EncryptedNote => {
                            if encrypted_note__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encryptedNote"));
                            }
                            encrypted_note__ = Some(map.next_value()?);
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
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DecimalAmount => {
                            if decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decimalAmount"));
                            }
                            decimal_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RollupFeeAmount => {
                            if rollup_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAmount"));
                            }
                            rollup_fee_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RollupFeeDecimalAmount => {
                            if rollup_fee_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeDecimalAmount"));
                            }
                            rollup_fee_decimal_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::ShieldedAddress => {
                            if shielded_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddress"));
                            }
                            shielded_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::WalletId => {
                            if wallet_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletId"));
                            }
                            wallet_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::AssetAddress => {
                            if asset_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetAddress"));
                            }
                            asset_address__ = map.next_value()?;
                        }
                        GeneratedField::BridgeFeeAmount => {
                            if bridge_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeAmount"));
                            }
                            bridge_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BridgeFeeDecimalAmount => {
                            if bridge_fee_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeDecimalAmount"));
                            }
                            bridge_fee_decimal_amount__ = map.next_value()?;
                        }
                        GeneratedField::ExecutorFeeAmount => {
                            if executor_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorFeeAmount"));
                            }
                            executor_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ExecutorFeeDecimalAmount => {
                            if executor_fee_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorFeeDecimalAmount"));
                            }
                            executor_fee_decimal_amount__ = map.next_value()?;
                        }
                        GeneratedField::BridgeFeeAssetAddress => {
                            if bridge_fee_asset_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeAssetAddress"));
                            }
                            bridge_fee_asset_address__ = map.next_value()?;
                        }
                        GeneratedField::BridgeFeeAssetSymbol => {
                            if bridge_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeAssetSymbol"));
                            }
                            bridge_fee_asset_symbol__ = map.next_value()?;
                        }
                        GeneratedField::BridgeFeeAssetDecimals => {
                            if bridge_fee_asset_decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeAssetDecimals"));
                            }
                            bridge_fee_asset_decimals__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ExecutorFeeAssetAddress => {
                            if executor_fee_asset_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorFeeAssetAddress"));
                            }
                            executor_fee_asset_address__ = map.next_value()?;
                        }
                        GeneratedField::ExecutorFeeAssetSymbol => {
                            if executor_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorFeeAssetSymbol"));
                            }
                            executor_fee_asset_symbol__ = map.next_value()?;
                        }
                        GeneratedField::ExecutorFeeAssetDecimals => {
                            if executor_fee_asset_decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorFeeAssetDecimals"));
                            }
                            executor_fee_asset_decimals__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::QueuedTransactionHash => {
                            if queued_transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queuedTransactionHash"));
                            }
                            queued_transaction_hash__ = map.next_value()?;
                        }
                        GeneratedField::IncludedTransactionHash => {
                            if included_transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includedTransactionHash"));
                            }
                            included_transaction_hash__ = map.next_value()?;
                        }
                        GeneratedField::SrcChainTransactionHash => {
                            if src_chain_transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srcChainTransactionHash"));
                            }
                            src_chain_transaction_hash__ = map.next_value()?;
                        }
                        GeneratedField::AssetApproveTransactionHash => {
                            if asset_approve_transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetApproveTransactionHash"));
                            }
                            asset_approve_transaction_hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = map.next_value()?;
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = Some(map.next_value::<super::super::super::common::v1::BridgeType>()? as i32);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<super::super::v1::DepositStatus>()? as i32);
                        }
                    }
                }
                Ok(Deposit {
                    id: id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    pool_address: pool_address__.unwrap_or_default(),
                    dst_chain_id: dst_chain_id__.unwrap_or_default(),
                    dst_chain_contract_address: dst_chain_contract_address__.unwrap_or_default(),
                    dst_pool_address: dst_pool_address__.unwrap_or_default(),
                    commitment_hash: commitment_hash__.unwrap_or_default(),
                    hash_k: hash_k__.unwrap_or_default(),
                    random_s: random_s__.unwrap_or_default(),
                    encrypted_note: encrypted_note__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    asset_decimals: asset_decimals__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    decimal_amount: decimal_amount__.unwrap_or_default(),
                    rollup_fee_amount: rollup_fee_amount__.unwrap_or_default(),
                    rollup_fee_decimal_amount: rollup_fee_decimal_amount__.unwrap_or_default(),
                    shielded_address: shielded_address__.unwrap_or_default(),
                    wallet_id: wallet_id__.unwrap_or_default(),
                    asset_address: asset_address__,
                    bridge_fee_amount: bridge_fee_amount__,
                    bridge_fee_decimal_amount: bridge_fee_decimal_amount__,
                    executor_fee_amount: executor_fee_amount__,
                    executor_fee_decimal_amount: executor_fee_decimal_amount__,
                    bridge_fee_asset_address: bridge_fee_asset_address__,
                    bridge_fee_asset_symbol: bridge_fee_asset_symbol__,
                    bridge_fee_asset_decimals: bridge_fee_asset_decimals__,
                    executor_fee_asset_address: executor_fee_asset_address__,
                    executor_fee_asset_symbol: executor_fee_asset_symbol__,
                    executor_fee_asset_decimals: executor_fee_asset_decimals__,
                    queued_transaction_hash: queued_transaction_hash__,
                    included_transaction_hash: included_transaction_hash__,
                    src_chain_transaction_hash: src_chain_transaction_hash__,
                    asset_approve_transaction_hash: asset_approve_transaction_hash__.unwrap_or_default(),
                    error_message: error_message__,
                    bridge_type: bridge_type__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.document.v1.Deposit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Spend {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if self.chain_id != 0 {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if self.asset_decimals != 0 {
            len += 1;
        }
        if self.amount != 0. {
            len += 1;
        }
        if !self.decimal_amount.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.wallet_id.is_empty() {
            len += 1;
        }
        if !self.input_commitments.is_empty() {
            len += 1;
        }
        if !self.output_commitments.is_empty() {
            len += 1;
        }
        if !self.nullifiers.is_empty() {
            len += 1;
        }
        if !self.signature_public_key_hashes.is_empty() {
            len += 1;
        }
        if !self.encrypted_auditor_notes.is_empty() {
            len += 1;
        }
        if self.rollup_fee_amount.is_some() {
            len += 1;
        }
        if self.rollup_fee_decimal_amount.is_some() {
            len += 1;
        }
        if self.rollup_fee_total_amount.is_some() {
            len += 1;
        }
        if self.rollup_fee_total_decimal_amount.is_some() {
            len += 1;
        }
        if self.gas_relayer_fee_amount.is_some() {
            len += 1;
        }
        if self.gas_relayer_fee_decimal_amount.is_some() {
            len += 1;
        }
        if self.signature_public_key.is_some() {
            len += 1;
        }
        if self.asset_address.is_some() {
            len += 1;
        }
        if self.proof.is_some() {
            len += 1;
        }
        if self.root_hash.is_some() {
            len += 1;
        }
        if self.gas_relayer_address.is_some() {
            len += 1;
        }
        if self.gas_relayer_url.is_some() {
            len += 1;
        }
        if self.signature.is_some() {
            len += 1;
        }
        if self.random_auditing_public_key.is_some() {
            len += 1;
        }
        if self.error_message.is_some() {
            len += 1;
        }
        if self.transaction_hash.is_some() {
            len += 1;
        }
        if self.bridge_type != 0 {
            len += 1;
        }
        if self.spend_type != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.document.v1.Spend", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.created_at != 0 {
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.asset_decimals != 0 {
            struct_ser.serialize_field("assetDecimals", &self.asset_decimals)?;
        }
        if self.amount != 0. {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.decimal_amount.is_empty() {
            struct_ser.serialize_field("decimalAmount", &self.decimal_amount)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.wallet_id.is_empty() {
            struct_ser.serialize_field("walletId", &self.wallet_id)?;
        }
        if !self.input_commitments.is_empty() {
            struct_ser.serialize_field("inputCommitments", &self.input_commitments)?;
        }
        if !self.output_commitments.is_empty() {
            struct_ser.serialize_field("outputCommitments", &self.output_commitments)?;
        }
        if !self.nullifiers.is_empty() {
            struct_ser.serialize_field("nullifiers", &self.nullifiers)?;
        }
        if !self.signature_public_key_hashes.is_empty() {
            struct_ser.serialize_field("signaturePublicKeyHashes", &self.signature_public_key_hashes)?;
        }
        if !self.encrypted_auditor_notes.is_empty() {
            struct_ser.serialize_field("encryptedAuditorNotes", &self.encrypted_auditor_notes)?;
        }
        if let Some(v) = self.rollup_fee_amount.as_ref() {
            struct_ser.serialize_field("rollupFeeAmount", v)?;
        }
        if let Some(v) = self.rollup_fee_decimal_amount.as_ref() {
            struct_ser.serialize_field("rollupFeeDecimalAmount", v)?;
        }
        if let Some(v) = self.rollup_fee_total_amount.as_ref() {
            struct_ser.serialize_field("rollupFeeTotalAmount", v)?;
        }
        if let Some(v) = self.rollup_fee_total_decimal_amount.as_ref() {
            struct_ser.serialize_field("rollupFeeTotalDecimalAmount", v)?;
        }
        if let Some(v) = self.gas_relayer_fee_amount.as_ref() {
            struct_ser.serialize_field("gasRelayerFeeAmount", v)?;
        }
        if let Some(v) = self.gas_relayer_fee_decimal_amount.as_ref() {
            struct_ser.serialize_field("gasRelayerFeeDecimalAmount", v)?;
        }
        if let Some(v) = self.signature_public_key.as_ref() {
            struct_ser.serialize_field("signaturePublicKey", v)?;
        }
        if let Some(v) = self.asset_address.as_ref() {
            struct_ser.serialize_field("assetAddress", v)?;
        }
        if let Some(v) = self.proof.as_ref() {
            struct_ser.serialize_field("proof", v)?;
        }
        if let Some(v) = self.root_hash.as_ref() {
            struct_ser.serialize_field("rootHash", v)?;
        }
        if let Some(v) = self.gas_relayer_address.as_ref() {
            struct_ser.serialize_field("gasRelayerAddress", v)?;
        }
        if let Some(v) = self.gas_relayer_url.as_ref() {
            struct_ser.serialize_field("gasRelayerUrl", v)?;
        }
        if let Some(v) = self.signature.as_ref() {
            struct_ser.serialize_field("signature", v)?;
        }
        if let Some(v) = self.random_auditing_public_key.as_ref() {
            struct_ser.serialize_field("randomAuditingPublicKey", v)?;
        }
        if let Some(v) = self.error_message.as_ref() {
            struct_ser.serialize_field("errorMessage", v)?;
        }
        if let Some(v) = self.transaction_hash.as_ref() {
            struct_ser.serialize_field("transactionHash", v)?;
        }
        if self.bridge_type != 0 {
            let v = super::super::super::common::v1::BridgeType::from_i32(self.bridge_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.bridge_type)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        if self.spend_type != 0 {
            let v = super::super::v1::SpendType::from_i32(self.spend_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.spend_type)))?;
            struct_ser.serialize_field("spendType", &v)?;
        }
        if self.status != 0 {
            let v = super::super::v1::SpendStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Spend {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "chain_id",
            "chainId",
            "contract_address",
            "contractAddress",
            "asset_symbol",
            "assetSymbol",
            "asset_decimals",
            "assetDecimals",
            "amount",
            "decimal_amount",
            "decimalAmount",
            "recipient",
            "wallet_id",
            "walletId",
            "input_commitments",
            "inputCommitments",
            "output_commitments",
            "outputCommitments",
            "nullifiers",
            "signature_public_key_hashes",
            "signaturePublicKeyHashes",
            "encrypted_auditor_notes",
            "encryptedAuditorNotes",
            "rollup_fee_amount",
            "rollupFeeAmount",
            "rollup_fee_decimal_amount",
            "rollupFeeDecimalAmount",
            "rollup_fee_total_amount",
            "rollupFeeTotalAmount",
            "rollup_fee_total_decimal_amount",
            "rollupFeeTotalDecimalAmount",
            "gas_relayer_fee_amount",
            "gasRelayerFeeAmount",
            "gas_relayer_fee_decimal_amount",
            "gasRelayerFeeDecimalAmount",
            "signature_public_key",
            "signaturePublicKey",
            "asset_address",
            "assetAddress",
            "proof",
            "root_hash",
            "rootHash",
            "gas_relayer_address",
            "gasRelayerAddress",
            "gas_relayer_url",
            "gasRelayerUrl",
            "signature",
            "random_auditing_public_key",
            "randomAuditingPublicKey",
            "error_message",
            "errorMessage",
            "transaction_hash",
            "transactionHash",
            "bridge_type",
            "bridgeType",
            "spend_type",
            "spendType",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreatedAt,
            UpdatedAt,
            ChainId,
            ContractAddress,
            AssetSymbol,
            AssetDecimals,
            Amount,
            DecimalAmount,
            Recipient,
            WalletId,
            InputCommitments,
            OutputCommitments,
            Nullifiers,
            SignaturePublicKeyHashes,
            EncryptedAuditorNotes,
            RollupFeeAmount,
            RollupFeeDecimalAmount,
            RollupFeeTotalAmount,
            RollupFeeTotalDecimalAmount,
            GasRelayerFeeAmount,
            GasRelayerFeeDecimalAmount,
            SignaturePublicKey,
            AssetAddress,
            Proof,
            RootHash,
            GasRelayerAddress,
            GasRelayerUrl,
            Signature,
            RandomAuditingPublicKey,
            ErrorMessage,
            TransactionHash,
            BridgeType,
            SpendType,
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
                            "id" => Ok(GeneratedField::Id),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "assetSymbol" | "asset_symbol" => Ok(GeneratedField::AssetSymbol),
                            "assetDecimals" | "asset_decimals" => Ok(GeneratedField::AssetDecimals),
                            "amount" => Ok(GeneratedField::Amount),
                            "decimalAmount" | "decimal_amount" => Ok(GeneratedField::DecimalAmount),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "walletId" | "wallet_id" => Ok(GeneratedField::WalletId),
                            "inputCommitments" | "input_commitments" => Ok(GeneratedField::InputCommitments),
                            "outputCommitments" | "output_commitments" => Ok(GeneratedField::OutputCommitments),
                            "nullifiers" => Ok(GeneratedField::Nullifiers),
                            "signaturePublicKeyHashes" | "signature_public_key_hashes" => Ok(GeneratedField::SignaturePublicKeyHashes),
                            "encryptedAuditorNotes" | "encrypted_auditor_notes" => Ok(GeneratedField::EncryptedAuditorNotes),
                            "rollupFeeAmount" | "rollup_fee_amount" => Ok(GeneratedField::RollupFeeAmount),
                            "rollupFeeDecimalAmount" | "rollup_fee_decimal_amount" => Ok(GeneratedField::RollupFeeDecimalAmount),
                            "rollupFeeTotalAmount" | "rollup_fee_total_amount" => Ok(GeneratedField::RollupFeeTotalAmount),
                            "rollupFeeTotalDecimalAmount" | "rollup_fee_total_decimal_amount" => Ok(GeneratedField::RollupFeeTotalDecimalAmount),
                            "gasRelayerFeeAmount" | "gas_relayer_fee_amount" => Ok(GeneratedField::GasRelayerFeeAmount),
                            "gasRelayerFeeDecimalAmount" | "gas_relayer_fee_decimal_amount" => Ok(GeneratedField::GasRelayerFeeDecimalAmount),
                            "signaturePublicKey" | "signature_public_key" => Ok(GeneratedField::SignaturePublicKey),
                            "assetAddress" | "asset_address" => Ok(GeneratedField::AssetAddress),
                            "proof" => Ok(GeneratedField::Proof),
                            "rootHash" | "root_hash" => Ok(GeneratedField::RootHash),
                            "gasRelayerAddress" | "gas_relayer_address" => Ok(GeneratedField::GasRelayerAddress),
                            "gasRelayerUrl" | "gas_relayer_url" => Ok(GeneratedField::GasRelayerUrl),
                            "signature" => Ok(GeneratedField::Signature),
                            "randomAuditingPublicKey" | "random_auditing_public_key" => Ok(GeneratedField::RandomAuditingPublicKey),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            "transactionHash" | "transaction_hash" => Ok(GeneratedField::TransactionHash),
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
                            "spendType" | "spend_type" => Ok(GeneratedField::SpendType),
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
            type Value = Spend;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.document.v1.Spend")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Spend, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut chain_id__ = None;
                let mut contract_address__ = None;
                let mut asset_symbol__ = None;
                let mut asset_decimals__ = None;
                let mut amount__ = None;
                let mut decimal_amount__ = None;
                let mut recipient__ = None;
                let mut wallet_id__ = None;
                let mut input_commitments__ = None;
                let mut output_commitments__ = None;
                let mut nullifiers__ = None;
                let mut signature_public_key_hashes__ = None;
                let mut encrypted_auditor_notes__ = None;
                let mut rollup_fee_amount__ = None;
                let mut rollup_fee_decimal_amount__ = None;
                let mut rollup_fee_total_amount__ = None;
                let mut rollup_fee_total_decimal_amount__ = None;
                let mut gas_relayer_fee_amount__ = None;
                let mut gas_relayer_fee_decimal_amount__ = None;
                let mut signature_public_key__ = None;
                let mut asset_address__ = None;
                let mut proof__ = None;
                let mut root_hash__ = None;
                let mut gas_relayer_address__ = None;
                let mut gas_relayer_url__ = None;
                let mut signature__ = None;
                let mut random_auditing_public_key__ = None;
                let mut error_message__ = None;
                let mut transaction_hash__ = None;
                let mut bridge_type__ = None;
                let mut spend_type__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
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
                            contract_address__ = Some(map.next_value()?);
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
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DecimalAmount => {
                            if decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decimalAmount"));
                            }
                            decimal_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map.next_value()?);
                        }
                        GeneratedField::WalletId => {
                            if wallet_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletId"));
                            }
                            wallet_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::InputCommitments => {
                            if input_commitments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputCommitments"));
                            }
                            input_commitments__ = Some(map.next_value()?);
                        }
                        GeneratedField::OutputCommitments => {
                            if output_commitments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputCommitments"));
                            }
                            output_commitments__ = Some(map.next_value()?);
                        }
                        GeneratedField::Nullifiers => {
                            if nullifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullifiers"));
                            }
                            nullifiers__ = Some(map.next_value()?);
                        }
                        GeneratedField::SignaturePublicKeyHashes => {
                            if signature_public_key_hashes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signaturePublicKeyHashes"));
                            }
                            signature_public_key_hashes__ = Some(map.next_value()?);
                        }
                        GeneratedField::EncryptedAuditorNotes => {
                            if encrypted_auditor_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encryptedAuditorNotes"));
                            }
                            encrypted_auditor_notes__ = Some(map.next_value()?);
                        }
                        GeneratedField::RollupFeeAmount => {
                            if rollup_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAmount"));
                            }
                            rollup_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RollupFeeDecimalAmount => {
                            if rollup_fee_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeDecimalAmount"));
                            }
                            rollup_fee_decimal_amount__ = map.next_value()?;
                        }
                        GeneratedField::RollupFeeTotalAmount => {
                            if rollup_fee_total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeTotalAmount"));
                            }
                            rollup_fee_total_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RollupFeeTotalDecimalAmount => {
                            if rollup_fee_total_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeTotalDecimalAmount"));
                            }
                            rollup_fee_total_decimal_amount__ = map.next_value()?;
                        }
                        GeneratedField::GasRelayerFeeAmount => {
                            if gas_relayer_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerFeeAmount"));
                            }
                            gas_relayer_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::GasRelayerFeeDecimalAmount => {
                            if gas_relayer_fee_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerFeeDecimalAmount"));
                            }
                            gas_relayer_fee_decimal_amount__ = map.next_value()?;
                        }
                        GeneratedField::SignaturePublicKey => {
                            if signature_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signaturePublicKey"));
                            }
                            signature_public_key__ = map.next_value()?;
                        }
                        GeneratedField::AssetAddress => {
                            if asset_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetAddress"));
                            }
                            asset_address__ = map.next_value()?;
                        }
                        GeneratedField::Proof => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proof"));
                            }
                            proof__ = map.next_value()?;
                        }
                        GeneratedField::RootHash => {
                            if root_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rootHash"));
                            }
                            root_hash__ = map.next_value()?;
                        }
                        GeneratedField::GasRelayerAddress => {
                            if gas_relayer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerAddress"));
                            }
                            gas_relayer_address__ = map.next_value()?;
                        }
                        GeneratedField::GasRelayerUrl => {
                            if gas_relayer_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerUrl"));
                            }
                            gas_relayer_url__ = map.next_value()?;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = map.next_value()?;
                        }
                        GeneratedField::RandomAuditingPublicKey => {
                            if random_auditing_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("randomAuditingPublicKey"));
                            }
                            random_auditing_public_key__ = map.next_value()?;
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = map.next_value()?;
                        }
                        GeneratedField::TransactionHash => {
                            if transaction_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionHash"));
                            }
                            transaction_hash__ = map.next_value()?;
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = Some(map.next_value::<super::super::super::common::v1::BridgeType>()? as i32);
                        }
                        GeneratedField::SpendType => {
                            if spend_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spendType"));
                            }
                            spend_type__ = Some(map.next_value::<super::super::v1::SpendType>()? as i32);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<super::super::v1::SpendStatus>()? as i32);
                        }
                    }
                }
                Ok(Spend {
                    id: id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    chain_id: chain_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    asset_decimals: asset_decimals__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    decimal_amount: decimal_amount__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    wallet_id: wallet_id__.unwrap_or_default(),
                    input_commitments: input_commitments__.unwrap_or_default(),
                    output_commitments: output_commitments__.unwrap_or_default(),
                    nullifiers: nullifiers__.unwrap_or_default(),
                    signature_public_key_hashes: signature_public_key_hashes__.unwrap_or_default(),
                    encrypted_auditor_notes: encrypted_auditor_notes__.unwrap_or_default(),
                    rollup_fee_amount: rollup_fee_amount__,
                    rollup_fee_decimal_amount: rollup_fee_decimal_amount__,
                    rollup_fee_total_amount: rollup_fee_total_amount__,
                    rollup_fee_total_decimal_amount: rollup_fee_total_decimal_amount__,
                    gas_relayer_fee_amount: gas_relayer_fee_amount__,
                    gas_relayer_fee_decimal_amount: gas_relayer_fee_decimal_amount__,
                    signature_public_key: signature_public_key__,
                    asset_address: asset_address__,
                    proof: proof__,
                    root_hash: root_hash__,
                    gas_relayer_address: gas_relayer_address__,
                    gas_relayer_url: gas_relayer_url__,
                    signature: signature__,
                    random_auditing_public_key: random_auditing_public_key__,
                    error_message: error_message__,
                    transaction_hash: transaction_hash__,
                    bridge_type: bridge_type__.unwrap_or_default(),
                    spend_type: spend_type__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.document.v1.Spend", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Wallet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if !self.encrypted_entropy.is_empty() {
            len += 1;
        }
        if !self.hashed_password.is_empty() {
            len += 1;
        }
        if self.account_nonce != 0 {
            len += 1;
        }
        if self.mnemonic_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.document.v1.Wallet", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.created_at != 0 {
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        if !self.encrypted_entropy.is_empty() {
            struct_ser.serialize_field("encryptedEntropy", &self.encrypted_entropy)?;
        }
        if !self.hashed_password.is_empty() {
            struct_ser.serialize_field("hashedPassword", &self.hashed_password)?;
        }
        if self.account_nonce != 0 {
            struct_ser.serialize_field("accountNonce", &self.account_nonce)?;
        }
        if self.mnemonic_type != 0 {
            let v = super::super::v1::MnemonicType::from_i32(self.mnemonic_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.mnemonic_type)))?;
            struct_ser.serialize_field("mnemonicType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Wallet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "encrypted_entropy",
            "encryptedEntropy",
            "hashed_password",
            "hashedPassword",
            "account_nonce",
            "accountNonce",
            "mnemonic_type",
            "mnemonicType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreatedAt,
            UpdatedAt,
            EncryptedEntropy,
            HashedPassword,
            AccountNonce,
            MnemonicType,
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
                            "id" => Ok(GeneratedField::Id),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "encryptedEntropy" | "encrypted_entropy" => Ok(GeneratedField::EncryptedEntropy),
                            "hashedPassword" | "hashed_password" => Ok(GeneratedField::HashedPassword),
                            "accountNonce" | "account_nonce" => Ok(GeneratedField::AccountNonce),
                            "mnemonicType" | "mnemonic_type" => Ok(GeneratedField::MnemonicType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Wallet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.document.v1.Wallet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Wallet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut encrypted_entropy__ = None;
                let mut hashed_password__ = None;
                let mut account_nonce__ = None;
                let mut mnemonic_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EncryptedEntropy => {
                            if encrypted_entropy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encryptedEntropy"));
                            }
                            encrypted_entropy__ = Some(map.next_value()?);
                        }
                        GeneratedField::HashedPassword => {
                            if hashed_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashedPassword"));
                            }
                            hashed_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::AccountNonce => {
                            if account_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountNonce"));
                            }
                            account_nonce__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MnemonicType => {
                            if mnemonic_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mnemonicType"));
                            }
                            mnemonic_type__ = Some(map.next_value::<super::super::v1::MnemonicType>()? as i32);
                        }
                    }
                }
                Ok(Wallet {
                    id: id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    encrypted_entropy: encrypted_entropy__.unwrap_or_default(),
                    hashed_password: hashed_password__.unwrap_or_default(),
                    account_nonce: account_nonce__.unwrap_or_default(),
                    mnemonic_type: mnemonic_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.document.v1.Wallet", FIELDS, GeneratedVisitor)
    }
}
