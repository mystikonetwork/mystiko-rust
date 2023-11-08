// @generated
impl serde::Serialize for CreateAccountOptions {
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
        if self.name.is_some() {
            len += 1;
        }
        if self.secret_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.CreateAccountOptions", len)?;
        if !self.wallet_password.is_empty() {
            struct_ser.serialize_field("walletPassword", &self.wallet_password)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.secret_key.as_ref() {
            struct_ser.serialize_field("secretKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAccountOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wallet_password",
            "walletPassword",
            "name",
            "secret_key",
            "secretKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WalletPassword,
            Name,
            SecretKey,
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
                            "name" => Ok(GeneratedField::Name),
                            "secretKey" | "secret_key" => Ok(GeneratedField::SecretKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAccountOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.CreateAccountOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateAccountOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet_password__ = None;
                let mut name__ = None;
                let mut secret_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WalletPassword => {
                            if wallet_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletPassword"));
                            }
                            wallet_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::SecretKey => {
                            if secret_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secretKey"));
                            }
                            secret_key__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateAccountOptions {
                    wallet_password: wallet_password__.unwrap_or_default(),
                    name: name__,
                    secret_key: secret_key__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.CreateAccountOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDepositOptions {
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
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if self.amount != 0. {
            len += 1;
        }
        if !self.shielded_address.is_empty() {
            len += 1;
        }
        if self.rollup_fee_amount != 0. {
            len += 1;
        }
        if self.dst_chain_id.is_some() {
            len += 1;
        }
        if self.bridge_fee_amount.is_some() {
            len += 1;
        }
        if self.executor_fee_amount.is_some() {
            len += 1;
        }
        if self.query_remote_timeout_ms.is_some() {
            len += 1;
        }
        if self.deposit_quote.is_some() {
            len += 1;
        }
        if self.bridge_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.CreateDepositOptions", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.amount != 0. {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.shielded_address.is_empty() {
            struct_ser.serialize_field("shieldedAddress", &self.shielded_address)?;
        }
        if self.rollup_fee_amount != 0. {
            struct_ser.serialize_field("rollupFeeAmount", &self.rollup_fee_amount)?;
        }
        if let Some(v) = self.dst_chain_id.as_ref() {
            struct_ser.serialize_field("dstChainId", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.bridge_fee_amount.as_ref() {
            struct_ser.serialize_field("bridgeFeeAmount", v)?;
        }
        if let Some(v) = self.executor_fee_amount.as_ref() {
            struct_ser.serialize_field("executorFeeAmount", v)?;
        }
        if let Some(v) = self.query_remote_timeout_ms.as_ref() {
            struct_ser.serialize_field("queryRemoteTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.deposit_quote.as_ref() {
            struct_ser.serialize_field("depositQuote", v)?;
        }
        if let Some(v) = self.bridge_type.as_ref() {
            let v = super::super::super::common::v1::BridgeType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDepositOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "asset_symbol",
            "assetSymbol",
            "amount",
            "shielded_address",
            "shieldedAddress",
            "rollup_fee_amount",
            "rollupFeeAmount",
            "dst_chain_id",
            "dstChainId",
            "bridge_fee_amount",
            "bridgeFeeAmount",
            "executor_fee_amount",
            "executorFeeAmount",
            "query_remote_timeout_ms",
            "queryRemoteTimeoutMs",
            "deposit_quote",
            "depositQuote",
            "bridge_type",
            "bridgeType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            AssetSymbol,
            Amount,
            ShieldedAddress,
            RollupFeeAmount,
            DstChainId,
            BridgeFeeAmount,
            ExecutorFeeAmount,
            QueryRemoteTimeoutMs,
            DepositQuote,
            BridgeType,
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
                            "assetSymbol" | "asset_symbol" => Ok(GeneratedField::AssetSymbol),
                            "amount" => Ok(GeneratedField::Amount),
                            "shieldedAddress" | "shielded_address" => Ok(GeneratedField::ShieldedAddress),
                            "rollupFeeAmount" | "rollup_fee_amount" => Ok(GeneratedField::RollupFeeAmount),
                            "dstChainId" | "dst_chain_id" => Ok(GeneratedField::DstChainId),
                            "bridgeFeeAmount" | "bridge_fee_amount" => Ok(GeneratedField::BridgeFeeAmount),
                            "executorFeeAmount" | "executor_fee_amount" => Ok(GeneratedField::ExecutorFeeAmount),
                            "queryRemoteTimeoutMs" | "query_remote_timeout_ms" => Ok(GeneratedField::QueryRemoteTimeoutMs),
                            "depositQuote" | "deposit_quote" => Ok(GeneratedField::DepositQuote),
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDepositOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.CreateDepositOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateDepositOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut asset_symbol__ = None;
                let mut amount__ = None;
                let mut shielded_address__ = None;
                let mut rollup_fee_amount__ = None;
                let mut dst_chain_id__ = None;
                let mut bridge_fee_amount__ = None;
                let mut executor_fee_amount__ = None;
                let mut query_remote_timeout_ms__ = None;
                let mut deposit_quote__ = None;
                let mut bridge_type__ = None;
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
                        GeneratedField::AssetSymbol => {
                            if asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetSymbol"));
                            }
                            asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ShieldedAddress => {
                            if shielded_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddress"));
                            }
                            shielded_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::RollupFeeAmount => {
                            if rollup_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAmount"));
                            }
                            rollup_fee_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DstChainId => {
                            if dst_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dstChainId"));
                            }
                            dst_chain_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BridgeFeeAmount => {
                            if bridge_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeAmount"));
                            }
                            bridge_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ExecutorFeeAmount => {
                            if executor_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorFeeAmount"));
                            }
                            executor_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::QueryRemoteTimeoutMs => {
                            if query_remote_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryRemoteTimeoutMs"));
                            }
                            query_remote_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::DepositQuote => {
                            if deposit_quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositQuote"));
                            }
                            deposit_quote__ = map.next_value()?;
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = map.next_value::<::std::option::Option<super::super::super::common::v1::BridgeType>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(CreateDepositOptions {
                    chain_id: chain_id__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    shielded_address: shielded_address__.unwrap_or_default(),
                    rollup_fee_amount: rollup_fee_amount__.unwrap_or_default(),
                    dst_chain_id: dst_chain_id__,
                    bridge_fee_amount: bridge_fee_amount__,
                    executor_fee_amount: executor_fee_amount__,
                    query_remote_timeout_ms: query_remote_timeout_ms__,
                    deposit_quote: deposit_quote__,
                    bridge_type: bridge_type__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.CreateDepositOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWalletOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.password.is_empty() {
            len += 1;
        }
        if self.mnemonic_phrase.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.CreateWalletOptions", len)?;
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        if let Some(v) = self.mnemonic_phrase.as_ref() {
            struct_ser.serialize_field("mnemonicPhrase", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWalletOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "password",
            "mnemonic_phrase",
            "mnemonicPhrase",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Password,
            MnemonicPhrase,
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
                            "password" => Ok(GeneratedField::Password),
                            "mnemonicPhrase" | "mnemonic_phrase" => Ok(GeneratedField::MnemonicPhrase),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWalletOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.CreateWalletOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateWalletOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut password__ = None;
                let mut mnemonic_phrase__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map.next_value()?);
                        }
                        GeneratedField::MnemonicPhrase => {
                            if mnemonic_phrase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mnemonicPhrase"));
                            }
                            mnemonic_phrase__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateWalletOptions {
                    password: password__.unwrap_or_default(),
                    mnemonic_phrase: mnemonic_phrase__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.CreateWalletOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DepositQuote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_amount != 0. {
            len += 1;
        }
        if self.max_amount != 0. {
            len += 1;
        }
        if self.min_rollup_fee_amount != 0. {
            len += 1;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            len += 1;
        }
        if self.min_bridge_fee_amount.is_some() {
            len += 1;
        }
        if self.bridge_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.min_executor_fee_amount.is_some() {
            len += 1;
        }
        if self.executor_fee_asset_symbol.is_some() {
            len += 1;
        }
        if !self.recommended_amounts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.DepositQuote", len)?;
        if self.min_amount != 0. {
            struct_ser.serialize_field("minAmount", &self.min_amount)?;
        }
        if self.max_amount != 0. {
            struct_ser.serialize_field("maxAmount", &self.max_amount)?;
        }
        if self.min_rollup_fee_amount != 0. {
            struct_ser.serialize_field("minRollupFeeAmount", &self.min_rollup_fee_amount)?;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            struct_ser.serialize_field("rollupFeeAssetSymbol", &self.rollup_fee_asset_symbol)?;
        }
        if let Some(v) = self.min_bridge_fee_amount.as_ref() {
            struct_ser.serialize_field("minBridgeFeeAmount", v)?;
        }
        if let Some(v) = self.bridge_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("bridgeFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.min_executor_fee_amount.as_ref() {
            struct_ser.serialize_field("minExecutorFeeAmount", v)?;
        }
        if let Some(v) = self.executor_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("executorFeeAssetSymbol", v)?;
        }
        if !self.recommended_amounts.is_empty() {
            struct_ser.serialize_field("recommendedAmounts", &self.recommended_amounts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DepositQuote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_amount",
            "minAmount",
            "max_amount",
            "maxAmount",
            "min_rollup_fee_amount",
            "minRollupFeeAmount",
            "rollup_fee_asset_symbol",
            "rollupFeeAssetSymbol",
            "min_bridge_fee_amount",
            "minBridgeFeeAmount",
            "bridge_fee_asset_symbol",
            "bridgeFeeAssetSymbol",
            "min_executor_fee_amount",
            "minExecutorFeeAmount",
            "executor_fee_asset_symbol",
            "executorFeeAssetSymbol",
            "recommended_amounts",
            "recommendedAmounts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinAmount,
            MaxAmount,
            MinRollupFeeAmount,
            RollupFeeAssetSymbol,
            MinBridgeFeeAmount,
            BridgeFeeAssetSymbol,
            MinExecutorFeeAmount,
            ExecutorFeeAssetSymbol,
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
                            "minAmount" | "min_amount" => Ok(GeneratedField::MinAmount),
                            "maxAmount" | "max_amount" => Ok(GeneratedField::MaxAmount),
                            "minRollupFeeAmount" | "min_rollup_fee_amount" => Ok(GeneratedField::MinRollupFeeAmount),
                            "rollupFeeAssetSymbol" | "rollup_fee_asset_symbol" => Ok(GeneratedField::RollupFeeAssetSymbol),
                            "minBridgeFeeAmount" | "min_bridge_fee_amount" => Ok(GeneratedField::MinBridgeFeeAmount),
                            "bridgeFeeAssetSymbol" | "bridge_fee_asset_symbol" => Ok(GeneratedField::BridgeFeeAssetSymbol),
                            "minExecutorFeeAmount" | "min_executor_fee_amount" => Ok(GeneratedField::MinExecutorFeeAmount),
                            "executorFeeAssetSymbol" | "executor_fee_asset_symbol" => Ok(GeneratedField::ExecutorFeeAssetSymbol),
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
            type Value = DepositQuote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.DepositQuote")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DepositQuote, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_amount__ = None;
                let mut max_amount__ = None;
                let mut min_rollup_fee_amount__ = None;
                let mut rollup_fee_asset_symbol__ = None;
                let mut min_bridge_fee_amount__ = None;
                let mut bridge_fee_asset_symbol__ = None;
                let mut min_executor_fee_amount__ = None;
                let mut executor_fee_asset_symbol__ = None;
                let mut recommended_amounts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MinAmount => {
                            if min_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minAmount"));
                            }
                            min_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxAmount => {
                            if max_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAmount"));
                            }
                            max_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinRollupFeeAmount => {
                            if min_rollup_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minRollupFeeAmount"));
                            }
                            min_rollup_fee_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RollupFeeAssetSymbol => {
                            if rollup_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAssetSymbol"));
                            }
                            rollup_fee_asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinBridgeFeeAmount => {
                            if min_bridge_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minBridgeFeeAmount"));
                            }
                            min_bridge_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BridgeFeeAssetSymbol => {
                            if bridge_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeAssetSymbol"));
                            }
                            bridge_fee_asset_symbol__ = map.next_value()?;
                        }
                        GeneratedField::MinExecutorFeeAmount => {
                            if min_executor_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minExecutorFeeAmount"));
                            }
                            min_executor_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ExecutorFeeAssetSymbol => {
                            if executor_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorFeeAssetSymbol"));
                            }
                            executor_fee_asset_symbol__ = map.next_value()?;
                        }
                        GeneratedField::RecommendedAmounts => {
                            if recommended_amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recommendedAmounts"));
                            }
                            recommended_amounts__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(DepositQuote {
                    min_amount: min_amount__.unwrap_or_default(),
                    max_amount: max_amount__.unwrap_or_default(),
                    min_rollup_fee_amount: min_rollup_fee_amount__.unwrap_or_default(),
                    rollup_fee_asset_symbol: rollup_fee_asset_symbol__.unwrap_or_default(),
                    min_bridge_fee_amount: min_bridge_fee_amount__,
                    bridge_fee_asset_symbol: bridge_fee_asset_symbol__,
                    min_executor_fee_amount: min_executor_fee_amount__,
                    executor_fee_asset_symbol: executor_fee_asset_symbol__,
                    recommended_amounts: recommended_amounts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.DepositQuote", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DepositSummary {
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
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if self.amount != 0. {
            len += 1;
        }
        if !self.shielded_address.is_empty() {
            len += 1;
        }
        if self.rollup_fee_amount != 0. {
            len += 1;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            len += 1;
        }
        if self.dst_chain_id.is_some() {
            len += 1;
        }
        if self.bridge_fee_amount.is_some() {
            len += 1;
        }
        if self.bridge_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.executor_fee_amount.is_some() {
            len += 1;
        }
        if self.executor_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.bridge_type.is_some() {
            len += 1;
        }
        if !self.total_amounts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.DepositSummary", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.amount != 0. {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.shielded_address.is_empty() {
            struct_ser.serialize_field("shieldedAddress", &self.shielded_address)?;
        }
        if self.rollup_fee_amount != 0. {
            struct_ser.serialize_field("rollupFeeAmount", &self.rollup_fee_amount)?;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            struct_ser.serialize_field("rollupFeeAssetSymbol", &self.rollup_fee_asset_symbol)?;
        }
        if let Some(v) = self.dst_chain_id.as_ref() {
            struct_ser.serialize_field("dstChainId", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.bridge_fee_amount.as_ref() {
            struct_ser.serialize_field("bridgeFeeAmount", v)?;
        }
        if let Some(v) = self.bridge_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("bridgeFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.executor_fee_amount.as_ref() {
            struct_ser.serialize_field("executorFeeAmount", v)?;
        }
        if let Some(v) = self.executor_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("executorFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.bridge_type.as_ref() {
            let v = super::super::super::common::v1::BridgeType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        if !self.total_amounts.is_empty() {
            struct_ser.serialize_field("totalAmounts", &self.total_amounts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DepositSummary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "asset_symbol",
            "assetSymbol",
            "amount",
            "shielded_address",
            "shieldedAddress",
            "rollup_fee_amount",
            "rollupFeeAmount",
            "rollup_fee_asset_symbol",
            "rollupFeeAssetSymbol",
            "dst_chain_id",
            "dstChainId",
            "bridge_fee_amount",
            "bridgeFeeAmount",
            "bridge_fee_asset_symbol",
            "bridgeFeeAssetSymbol",
            "executor_fee_amount",
            "executorFeeAmount",
            "executor_fee_asset_symbol",
            "executorFeeAssetSymbol",
            "bridge_type",
            "bridgeType",
            "total_amounts",
            "totalAmounts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            AssetSymbol,
            Amount,
            ShieldedAddress,
            RollupFeeAmount,
            RollupFeeAssetSymbol,
            DstChainId,
            BridgeFeeAmount,
            BridgeFeeAssetSymbol,
            ExecutorFeeAmount,
            ExecutorFeeAssetSymbol,
            BridgeType,
            TotalAmounts,
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
                            "assetSymbol" | "asset_symbol" => Ok(GeneratedField::AssetSymbol),
                            "amount" => Ok(GeneratedField::Amount),
                            "shieldedAddress" | "shielded_address" => Ok(GeneratedField::ShieldedAddress),
                            "rollupFeeAmount" | "rollup_fee_amount" => Ok(GeneratedField::RollupFeeAmount),
                            "rollupFeeAssetSymbol" | "rollup_fee_asset_symbol" => Ok(GeneratedField::RollupFeeAssetSymbol),
                            "dstChainId" | "dst_chain_id" => Ok(GeneratedField::DstChainId),
                            "bridgeFeeAmount" | "bridge_fee_amount" => Ok(GeneratedField::BridgeFeeAmount),
                            "bridgeFeeAssetSymbol" | "bridge_fee_asset_symbol" => Ok(GeneratedField::BridgeFeeAssetSymbol),
                            "executorFeeAmount" | "executor_fee_amount" => Ok(GeneratedField::ExecutorFeeAmount),
                            "executorFeeAssetSymbol" | "executor_fee_asset_symbol" => Ok(GeneratedField::ExecutorFeeAssetSymbol),
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
                            "totalAmounts" | "total_amounts" => Ok(GeneratedField::TotalAmounts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DepositSummary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.DepositSummary")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DepositSummary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut asset_symbol__ = None;
                let mut amount__ = None;
                let mut shielded_address__ = None;
                let mut rollup_fee_amount__ = None;
                let mut rollup_fee_asset_symbol__ = None;
                let mut dst_chain_id__ = None;
                let mut bridge_fee_amount__ = None;
                let mut bridge_fee_asset_symbol__ = None;
                let mut executor_fee_amount__ = None;
                let mut executor_fee_asset_symbol__ = None;
                let mut bridge_type__ = None;
                let mut total_amounts__ = None;
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
                        GeneratedField::AssetSymbol => {
                            if asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetSymbol"));
                            }
                            asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ShieldedAddress => {
                            if shielded_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddress"));
                            }
                            shielded_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::RollupFeeAmount => {
                            if rollup_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAmount"));
                            }
                            rollup_fee_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RollupFeeAssetSymbol => {
                            if rollup_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAssetSymbol"));
                            }
                            rollup_fee_asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::DstChainId => {
                            if dst_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dstChainId"));
                            }
                            dst_chain_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BridgeFeeAmount => {
                            if bridge_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeAmount"));
                            }
                            bridge_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BridgeFeeAssetSymbol => {
                            if bridge_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeAssetSymbol"));
                            }
                            bridge_fee_asset_symbol__ = map.next_value()?;
                        }
                        GeneratedField::ExecutorFeeAmount => {
                            if executor_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorFeeAmount"));
                            }
                            executor_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ExecutorFeeAssetSymbol => {
                            if executor_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorFeeAssetSymbol"));
                            }
                            executor_fee_asset_symbol__ = map.next_value()?;
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = map.next_value::<::std::option::Option<super::super::super::common::v1::BridgeType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::TotalAmounts => {
                            if total_amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalAmounts"));
                            }
                            total_amounts__ = Some(
                                map.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<f64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                    }
                }
                Ok(DepositSummary {
                    chain_id: chain_id__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    shielded_address: shielded_address__.unwrap_or_default(),
                    rollup_fee_amount: rollup_fee_amount__.unwrap_or_default(),
                    rollup_fee_asset_symbol: rollup_fee_asset_symbol__.unwrap_or_default(),
                    dst_chain_id: dst_chain_id__,
                    bridge_fee_amount: bridge_fee_amount__,
                    bridge_fee_asset_symbol: bridge_fee_asset_symbol__,
                    executor_fee_amount: executor_fee_amount__,
                    executor_fee_asset_symbol: executor_fee_asset_symbol__,
                    bridge_type: bridge_type__,
                    total_amounts: total_amounts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.DepositSummary", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuoteDepositOptions {
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
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if self.query_remote_timeout_ms.is_some() {
            len += 1;
        }
        if self.dst_chain_id.is_some() {
            len += 1;
        }
        if self.bridge_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.QuoteDepositOptions", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if let Some(v) = self.query_remote_timeout_ms.as_ref() {
            struct_ser.serialize_field("queryRemoteTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.dst_chain_id.as_ref() {
            struct_ser.serialize_field("dstChainId", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.bridge_type.as_ref() {
            let v = super::super::super::common::v1::BridgeType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuoteDepositOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "asset_symbol",
            "assetSymbol",
            "query_remote_timeout_ms",
            "queryRemoteTimeoutMs",
            "dst_chain_id",
            "dstChainId",
            "bridge_type",
            "bridgeType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            AssetSymbol,
            QueryRemoteTimeoutMs,
            DstChainId,
            BridgeType,
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
                            "assetSymbol" | "asset_symbol" => Ok(GeneratedField::AssetSymbol),
                            "queryRemoteTimeoutMs" | "query_remote_timeout_ms" => Ok(GeneratedField::QueryRemoteTimeoutMs),
                            "dstChainId" | "dst_chain_id" => Ok(GeneratedField::DstChainId),
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuoteDepositOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.QuoteDepositOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuoteDepositOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut asset_symbol__ = None;
                let mut query_remote_timeout_ms__ = None;
                let mut dst_chain_id__ = None;
                let mut bridge_type__ = None;
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
                        GeneratedField::AssetSymbol => {
                            if asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetSymbol"));
                            }
                            asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::QueryRemoteTimeoutMs => {
                            if query_remote_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryRemoteTimeoutMs"));
                            }
                            query_remote_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::DstChainId => {
                            if dst_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dstChainId"));
                            }
                            dst_chain_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = map.next_value::<::std::option::Option<super::super::super::common::v1::BridgeType>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(QuoteDepositOptions {
                    chain_id: chain_id__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    query_remote_timeout_ms: query_remote_timeout_ms__,
                    dst_chain_id: dst_chain_id__,
                    bridge_type: bridge_type__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.QuoteDepositOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendDepositOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.deposit_id.is_empty() {
            len += 1;
        }
        if self.private_key.is_some() {
            len += 1;
        }
        if self.query_remote_timeout_ms.is_some() {
            len += 1;
        }
        if self.asset_approve_confirmations.is_some() {
            len += 1;
        }
        if self.deposit_confirmations.is_some() {
            len += 1;
        }
        if self.asset_approve_tx.is_some() {
            len += 1;
        }
        if self.deposit_tx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.SendDepositOptions", len)?;
        if !self.deposit_id.is_empty() {
            struct_ser.serialize_field("depositId", &self.deposit_id)?;
        }
        if let Some(v) = self.private_key.as_ref() {
            struct_ser.serialize_field("privateKey", v)?;
        }
        if let Some(v) = self.query_remote_timeout_ms.as_ref() {
            struct_ser.serialize_field("queryRemoteTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.asset_approve_confirmations.as_ref() {
            struct_ser.serialize_field("assetApproveConfirmations", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.deposit_confirmations.as_ref() {
            struct_ser.serialize_field("depositConfirmations", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.asset_approve_tx.as_ref() {
            struct_ser.serialize_field("assetApproveTx", v)?;
        }
        if let Some(v) = self.deposit_tx.as_ref() {
            struct_ser.serialize_field("depositTx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendDepositOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposit_id",
            "depositId",
            "private_key",
            "privateKey",
            "query_remote_timeout_ms",
            "queryRemoteTimeoutMs",
            "asset_approve_confirmations",
            "assetApproveConfirmations",
            "deposit_confirmations",
            "depositConfirmations",
            "asset_approve_tx",
            "assetApproveTx",
            "deposit_tx",
            "depositTx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DepositId,
            PrivateKey,
            QueryRemoteTimeoutMs,
            AssetApproveConfirmations,
            DepositConfirmations,
            AssetApproveTx,
            DepositTx,
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
                            "depositId" | "deposit_id" => Ok(GeneratedField::DepositId),
                            "privateKey" | "private_key" => Ok(GeneratedField::PrivateKey),
                            "queryRemoteTimeoutMs" | "query_remote_timeout_ms" => Ok(GeneratedField::QueryRemoteTimeoutMs),
                            "assetApproveConfirmations" | "asset_approve_confirmations" => Ok(GeneratedField::AssetApproveConfirmations),
                            "depositConfirmations" | "deposit_confirmations" => Ok(GeneratedField::DepositConfirmations),
                            "assetApproveTx" | "asset_approve_tx" => Ok(GeneratedField::AssetApproveTx),
                            "depositTx" | "deposit_tx" => Ok(GeneratedField::DepositTx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendDepositOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.SendDepositOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SendDepositOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deposit_id__ = None;
                let mut private_key__ = None;
                let mut query_remote_timeout_ms__ = None;
                let mut asset_approve_confirmations__ = None;
                let mut deposit_confirmations__ = None;
                let mut asset_approve_tx__ = None;
                let mut deposit_tx__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DepositId => {
                            if deposit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositId"));
                            }
                            deposit_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::PrivateKey => {
                            if private_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKey"));
                            }
                            private_key__ = map.next_value()?;
                        }
                        GeneratedField::QueryRemoteTimeoutMs => {
                            if query_remote_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryRemoteTimeoutMs"));
                            }
                            query_remote_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AssetApproveConfirmations => {
                            if asset_approve_confirmations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetApproveConfirmations"));
                            }
                            asset_approve_confirmations__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::DepositConfirmations => {
                            if deposit_confirmations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositConfirmations"));
                            }
                            deposit_confirmations__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AssetApproveTx => {
                            if asset_approve_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetApproveTx"));
                            }
                            asset_approve_tx__ = map.next_value()?;
                        }
                        GeneratedField::DepositTx => {
                            if deposit_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositTx"));
                            }
                            deposit_tx__ = map.next_value()?;
                        }
                    }
                }
                Ok(SendDepositOptions {
                    deposit_id: deposit_id__.unwrap_or_default(),
                    private_key: private_key__,
                    query_remote_timeout_ms: query_remote_timeout_ms__,
                    asset_approve_confirmations: asset_approve_confirmations__,
                    deposit_confirmations: deposit_confirmations__,
                    asset_approve_tx: asset_approve_tx__,
                    deposit_tx: deposit_tx__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.SendDepositOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAccountOptions {
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
        if self.name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.UpdateAccountOptions", len)?;
        if !self.wallet_password.is_empty() {
            struct_ser.serialize_field("walletPassword", &self.wallet_password)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAccountOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wallet_password",
            "walletPassword",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WalletPassword,
            Name,
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
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAccountOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.UpdateAccountOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateAccountOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet_password__ = None;
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WalletPassword => {
                            if wallet_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletPassword"));
                            }
                            wallet_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateAccountOptions {
                    wallet_password: wallet_password__.unwrap_or_default(),
                    name: name__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.UpdateAccountOptions", FIELDS, GeneratedVisitor)
    }
}
