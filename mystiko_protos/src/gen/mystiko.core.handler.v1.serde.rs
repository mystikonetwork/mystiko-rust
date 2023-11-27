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
        if self.query_timeout_ms.is_some() {
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
        if let Some(v) = self.query_timeout_ms.as_ref() {
            struct_ser.serialize_field("queryTimeoutMs", ToString::to_string(&v).as_str())?;
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
            "query_timeout_ms",
            "queryTimeoutMs",
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
            QueryTimeoutMs,
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
                            "queryTimeoutMs" | "query_timeout_ms" => Ok(GeneratedField::QueryTimeoutMs),
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
                let mut query_timeout_ms__ = None;
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
                        GeneratedField::QueryTimeoutMs => {
                            if query_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryTimeoutMs"));
                            }
                            query_timeout_ms__ = 
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
                    query_timeout_ms: query_timeout_ms__,
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
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if self.asset_decimals != 0 {
            len += 1;
        }
        if self.min_amount != 0. {
            len += 1;
        }
        if !self.min_decimal_amount.is_empty() {
            len += 1;
        }
        if self.max_amount != 0. {
            len += 1;
        }
        if !self.max_decimal_amount.is_empty() {
            len += 1;
        }
        if self.min_rollup_fee_amount != 0. {
            len += 1;
        }
        if !self.min_rollup_fee_decimal_amount.is_empty() {
            len += 1;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            len += 1;
        }
        if self.rollup_fee_asset_decimals != 0 {
            len += 1;
        }
        if self.min_bridge_fee_amount.is_some() {
            len += 1;
        }
        if self.min_bridge_fee_decimal_amount.is_some() {
            len += 1;
        }
        if self.bridge_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.bridge_fee_asset_decimals.is_some() {
            len += 1;
        }
        if self.min_executor_fee_amount.is_some() {
            len += 1;
        }
        if self.min_executor_fee_decimal_amount.is_some() {
            len += 1;
        }
        if self.executor_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.executor_fee_asset_decimals.is_some() {
            len += 1;
        }
        if !self.recommended_amounts.is_empty() {
            len += 1;
        }
        if !self.recommended_decimal_amounts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.DepositQuote", len)?;
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.asset_decimals != 0 {
            struct_ser.serialize_field("assetDecimals", &self.asset_decimals)?;
        }
        if self.min_amount != 0. {
            struct_ser.serialize_field("minAmount", &self.min_amount)?;
        }
        if !self.min_decimal_amount.is_empty() {
            struct_ser.serialize_field("minDecimalAmount", &self.min_decimal_amount)?;
        }
        if self.max_amount != 0. {
            struct_ser.serialize_field("maxAmount", &self.max_amount)?;
        }
        if !self.max_decimal_amount.is_empty() {
            struct_ser.serialize_field("maxDecimalAmount", &self.max_decimal_amount)?;
        }
        if self.min_rollup_fee_amount != 0. {
            struct_ser.serialize_field("minRollupFeeAmount", &self.min_rollup_fee_amount)?;
        }
        if !self.min_rollup_fee_decimal_amount.is_empty() {
            struct_ser.serialize_field("minRollupFeeDecimalAmount", &self.min_rollup_fee_decimal_amount)?;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            struct_ser.serialize_field("rollupFeeAssetSymbol", &self.rollup_fee_asset_symbol)?;
        }
        if self.rollup_fee_asset_decimals != 0 {
            struct_ser.serialize_field("rollupFeeAssetDecimals", &self.rollup_fee_asset_decimals)?;
        }
        if let Some(v) = self.min_bridge_fee_amount.as_ref() {
            struct_ser.serialize_field("minBridgeFeeAmount", v)?;
        }
        if let Some(v) = self.min_bridge_fee_decimal_amount.as_ref() {
            struct_ser.serialize_field("minBridgeFeeDecimalAmount", v)?;
        }
        if let Some(v) = self.bridge_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("bridgeFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.bridge_fee_asset_decimals.as_ref() {
            struct_ser.serialize_field("bridgeFeeAssetDecimals", v)?;
        }
        if let Some(v) = self.min_executor_fee_amount.as_ref() {
            struct_ser.serialize_field("minExecutorFeeAmount", v)?;
        }
        if let Some(v) = self.min_executor_fee_decimal_amount.as_ref() {
            struct_ser.serialize_field("minExecutorFeeDecimalAmount", v)?;
        }
        if let Some(v) = self.executor_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("executorFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.executor_fee_asset_decimals.as_ref() {
            struct_ser.serialize_field("executorFeeAssetDecimals", v)?;
        }
        if !self.recommended_amounts.is_empty() {
            struct_ser.serialize_field("recommendedAmounts", &self.recommended_amounts)?;
        }
        if !self.recommended_decimal_amounts.is_empty() {
            struct_ser.serialize_field("recommendedDecimalAmounts", &self.recommended_decimal_amounts)?;
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
            "asset_symbol",
            "assetSymbol",
            "asset_decimals",
            "assetDecimals",
            "min_amount",
            "minAmount",
            "min_decimal_amount",
            "minDecimalAmount",
            "max_amount",
            "maxAmount",
            "max_decimal_amount",
            "maxDecimalAmount",
            "min_rollup_fee_amount",
            "minRollupFeeAmount",
            "min_rollup_fee_decimal_amount",
            "minRollupFeeDecimalAmount",
            "rollup_fee_asset_symbol",
            "rollupFeeAssetSymbol",
            "rollup_fee_asset_decimals",
            "rollupFeeAssetDecimals",
            "min_bridge_fee_amount",
            "minBridgeFeeAmount",
            "min_bridge_fee_decimal_amount",
            "minBridgeFeeDecimalAmount",
            "bridge_fee_asset_symbol",
            "bridgeFeeAssetSymbol",
            "bridge_fee_asset_decimals",
            "bridgeFeeAssetDecimals",
            "min_executor_fee_amount",
            "minExecutorFeeAmount",
            "min_executor_fee_decimal_amount",
            "minExecutorFeeDecimalAmount",
            "executor_fee_asset_symbol",
            "executorFeeAssetSymbol",
            "executor_fee_asset_decimals",
            "executorFeeAssetDecimals",
            "recommended_amounts",
            "recommendedAmounts",
            "recommended_decimal_amounts",
            "recommendedDecimalAmounts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetSymbol,
            AssetDecimals,
            MinAmount,
            MinDecimalAmount,
            MaxAmount,
            MaxDecimalAmount,
            MinRollupFeeAmount,
            MinRollupFeeDecimalAmount,
            RollupFeeAssetSymbol,
            RollupFeeAssetDecimals,
            MinBridgeFeeAmount,
            MinBridgeFeeDecimalAmount,
            BridgeFeeAssetSymbol,
            BridgeFeeAssetDecimals,
            MinExecutorFeeAmount,
            MinExecutorFeeDecimalAmount,
            ExecutorFeeAssetSymbol,
            ExecutorFeeAssetDecimals,
            RecommendedAmounts,
            RecommendedDecimalAmounts,
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
                            "assetDecimals" | "asset_decimals" => Ok(GeneratedField::AssetDecimals),
                            "minAmount" | "min_amount" => Ok(GeneratedField::MinAmount),
                            "minDecimalAmount" | "min_decimal_amount" => Ok(GeneratedField::MinDecimalAmount),
                            "maxAmount" | "max_amount" => Ok(GeneratedField::MaxAmount),
                            "maxDecimalAmount" | "max_decimal_amount" => Ok(GeneratedField::MaxDecimalAmount),
                            "minRollupFeeAmount" | "min_rollup_fee_amount" => Ok(GeneratedField::MinRollupFeeAmount),
                            "minRollupFeeDecimalAmount" | "min_rollup_fee_decimal_amount" => Ok(GeneratedField::MinRollupFeeDecimalAmount),
                            "rollupFeeAssetSymbol" | "rollup_fee_asset_symbol" => Ok(GeneratedField::RollupFeeAssetSymbol),
                            "rollupFeeAssetDecimals" | "rollup_fee_asset_decimals" => Ok(GeneratedField::RollupFeeAssetDecimals),
                            "minBridgeFeeAmount" | "min_bridge_fee_amount" => Ok(GeneratedField::MinBridgeFeeAmount),
                            "minBridgeFeeDecimalAmount" | "min_bridge_fee_decimal_amount" => Ok(GeneratedField::MinBridgeFeeDecimalAmount),
                            "bridgeFeeAssetSymbol" | "bridge_fee_asset_symbol" => Ok(GeneratedField::BridgeFeeAssetSymbol),
                            "bridgeFeeAssetDecimals" | "bridge_fee_asset_decimals" => Ok(GeneratedField::BridgeFeeAssetDecimals),
                            "minExecutorFeeAmount" | "min_executor_fee_amount" => Ok(GeneratedField::MinExecutorFeeAmount),
                            "minExecutorFeeDecimalAmount" | "min_executor_fee_decimal_amount" => Ok(GeneratedField::MinExecutorFeeDecimalAmount),
                            "executorFeeAssetSymbol" | "executor_fee_asset_symbol" => Ok(GeneratedField::ExecutorFeeAssetSymbol),
                            "executorFeeAssetDecimals" | "executor_fee_asset_decimals" => Ok(GeneratedField::ExecutorFeeAssetDecimals),
                            "recommendedAmounts" | "recommended_amounts" => Ok(GeneratedField::RecommendedAmounts),
                            "recommendedDecimalAmounts" | "recommended_decimal_amounts" => Ok(GeneratedField::RecommendedDecimalAmounts),
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
                let mut asset_symbol__ = None;
                let mut asset_decimals__ = None;
                let mut min_amount__ = None;
                let mut min_decimal_amount__ = None;
                let mut max_amount__ = None;
                let mut max_decimal_amount__ = None;
                let mut min_rollup_fee_amount__ = None;
                let mut min_rollup_fee_decimal_amount__ = None;
                let mut rollup_fee_asset_symbol__ = None;
                let mut rollup_fee_asset_decimals__ = None;
                let mut min_bridge_fee_amount__ = None;
                let mut min_bridge_fee_decimal_amount__ = None;
                let mut bridge_fee_asset_symbol__ = None;
                let mut bridge_fee_asset_decimals__ = None;
                let mut min_executor_fee_amount__ = None;
                let mut min_executor_fee_decimal_amount__ = None;
                let mut executor_fee_asset_symbol__ = None;
                let mut executor_fee_asset_decimals__ = None;
                let mut recommended_amounts__ = None;
                let mut recommended_decimal_amounts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::MinAmount => {
                            if min_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minAmount"));
                            }
                            min_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinDecimalAmount => {
                            if min_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minDecimalAmount"));
                            }
                            min_decimal_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxAmount => {
                            if max_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAmount"));
                            }
                            max_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxDecimalAmount => {
                            if max_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDecimalAmount"));
                            }
                            max_decimal_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinRollupFeeAmount => {
                            if min_rollup_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minRollupFeeAmount"));
                            }
                            min_rollup_fee_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinRollupFeeDecimalAmount => {
                            if min_rollup_fee_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minRollupFeeDecimalAmount"));
                            }
                            min_rollup_fee_decimal_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RollupFeeAssetSymbol => {
                            if rollup_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAssetSymbol"));
                            }
                            rollup_fee_asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::RollupFeeAssetDecimals => {
                            if rollup_fee_asset_decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAssetDecimals"));
                            }
                            rollup_fee_asset_decimals__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinBridgeFeeAmount => {
                            if min_bridge_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minBridgeFeeAmount"));
                            }
                            min_bridge_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinBridgeFeeDecimalAmount => {
                            if min_bridge_fee_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minBridgeFeeDecimalAmount"));
                            }
                            min_bridge_fee_decimal_amount__ = map.next_value()?;
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
                        GeneratedField::MinExecutorFeeAmount => {
                            if min_executor_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minExecutorFeeAmount"));
                            }
                            min_executor_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinExecutorFeeDecimalAmount => {
                            if min_executor_fee_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minExecutorFeeDecimalAmount"));
                            }
                            min_executor_fee_decimal_amount__ = map.next_value()?;
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
                        GeneratedField::RecommendedAmounts => {
                            if recommended_amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recommendedAmounts"));
                            }
                            recommended_amounts__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::RecommendedDecimalAmounts => {
                            if recommended_decimal_amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recommendedDecimalAmounts"));
                            }
                            recommended_decimal_amounts__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DepositQuote {
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    asset_decimals: asset_decimals__.unwrap_or_default(),
                    min_amount: min_amount__.unwrap_or_default(),
                    min_decimal_amount: min_decimal_amount__.unwrap_or_default(),
                    max_amount: max_amount__.unwrap_or_default(),
                    max_decimal_amount: max_decimal_amount__.unwrap_or_default(),
                    min_rollup_fee_amount: min_rollup_fee_amount__.unwrap_or_default(),
                    min_rollup_fee_decimal_amount: min_rollup_fee_decimal_amount__.unwrap_or_default(),
                    rollup_fee_asset_symbol: rollup_fee_asset_symbol__.unwrap_or_default(),
                    rollup_fee_asset_decimals: rollup_fee_asset_decimals__.unwrap_or_default(),
                    min_bridge_fee_amount: min_bridge_fee_amount__,
                    min_bridge_fee_decimal_amount: min_bridge_fee_decimal_amount__,
                    bridge_fee_asset_symbol: bridge_fee_asset_symbol__,
                    bridge_fee_asset_decimals: bridge_fee_asset_decimals__,
                    min_executor_fee_amount: min_executor_fee_amount__,
                    min_executor_fee_decimal_amount: min_executor_fee_decimal_amount__,
                    executor_fee_asset_symbol: executor_fee_asset_symbol__,
                    executor_fee_asset_decimals: executor_fee_asset_decimals__,
                    recommended_amounts: recommended_amounts__.unwrap_or_default(),
                    recommended_decimal_amounts: recommended_decimal_amounts__.unwrap_or_default(),
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
        if self.asset_decimals != 0 {
            len += 1;
        }
        if self.amount != 0. {
            len += 1;
        }
        if !self.decimal_amount.is_empty() {
            len += 1;
        }
        if !self.shielded_address.is_empty() {
            len += 1;
        }
        if self.rollup_fee_amount != 0. {
            len += 1;
        }
        if !self.rollup_fee_decimal_amount.is_empty() {
            len += 1;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            len += 1;
        }
        if self.rollup_fee_asset_decimals != 0 {
            len += 1;
        }
        if self.dst_chain_id.is_some() {
            len += 1;
        }
        if self.bridge_fee_amount.is_some() {
            len += 1;
        }
        if self.bridge_fee_decimal_amount.is_some() {
            len += 1;
        }
        if self.bridge_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.bridge_fee_asset_decimals.is_some() {
            len += 1;
        }
        if self.executor_fee_amount.is_some() {
            len += 1;
        }
        if self.executor_fee_decimal_amount.is_some() {
            len += 1;
        }
        if self.executor_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.executor_fee_asset_decimals.is_some() {
            len += 1;
        }
        if self.bridge_type.is_some() {
            len += 1;
        }
        if !self.total_amounts.is_empty() {
            len += 1;
        }
        if !self.total_decimal_amounts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.DepositSummary", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
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
        if !self.shielded_address.is_empty() {
            struct_ser.serialize_field("shieldedAddress", &self.shielded_address)?;
        }
        if self.rollup_fee_amount != 0. {
            struct_ser.serialize_field("rollupFeeAmount", &self.rollup_fee_amount)?;
        }
        if !self.rollup_fee_decimal_amount.is_empty() {
            struct_ser.serialize_field("rollupFeeDecimalAmount", &self.rollup_fee_decimal_amount)?;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            struct_ser.serialize_field("rollupFeeAssetSymbol", &self.rollup_fee_asset_symbol)?;
        }
        if self.rollup_fee_asset_decimals != 0 {
            struct_ser.serialize_field("rollupFeeAssetDecimals", &self.rollup_fee_asset_decimals)?;
        }
        if let Some(v) = self.dst_chain_id.as_ref() {
            struct_ser.serialize_field("dstChainId", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.bridge_fee_amount.as_ref() {
            struct_ser.serialize_field("bridgeFeeAmount", v)?;
        }
        if let Some(v) = self.bridge_fee_decimal_amount.as_ref() {
            struct_ser.serialize_field("bridgeFeeDecimalAmount", v)?;
        }
        if let Some(v) = self.bridge_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("bridgeFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.bridge_fee_asset_decimals.as_ref() {
            struct_ser.serialize_field("bridgeFeeAssetDecimals", v)?;
        }
        if let Some(v) = self.executor_fee_amount.as_ref() {
            struct_ser.serialize_field("executorFeeAmount", v)?;
        }
        if let Some(v) = self.executor_fee_decimal_amount.as_ref() {
            struct_ser.serialize_field("executorFeeDecimalAmount", v)?;
        }
        if let Some(v) = self.executor_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("executorFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.executor_fee_asset_decimals.as_ref() {
            struct_ser.serialize_field("executorFeeAssetDecimals", v)?;
        }
        if let Some(v) = self.bridge_type.as_ref() {
            let v = super::super::super::common::v1::BridgeType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        if !self.total_amounts.is_empty() {
            struct_ser.serialize_field("totalAmounts", &self.total_amounts)?;
        }
        if !self.total_decimal_amounts.is_empty() {
            struct_ser.serialize_field("totalDecimalAmounts", &self.total_decimal_amounts)?;
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
            "asset_decimals",
            "assetDecimals",
            "amount",
            "decimal_amount",
            "decimalAmount",
            "shielded_address",
            "shieldedAddress",
            "rollup_fee_amount",
            "rollupFeeAmount",
            "rollup_fee_decimal_amount",
            "rollupFeeDecimalAmount",
            "rollup_fee_asset_symbol",
            "rollupFeeAssetSymbol",
            "rollup_fee_asset_decimals",
            "rollupFeeAssetDecimals",
            "dst_chain_id",
            "dstChainId",
            "bridge_fee_amount",
            "bridgeFeeAmount",
            "bridge_fee_decimal_amount",
            "bridgeFeeDecimalAmount",
            "bridge_fee_asset_symbol",
            "bridgeFeeAssetSymbol",
            "bridge_fee_asset_decimals",
            "bridgeFeeAssetDecimals",
            "executor_fee_amount",
            "executorFeeAmount",
            "executor_fee_decimal_amount",
            "executorFeeDecimalAmount",
            "executor_fee_asset_symbol",
            "executorFeeAssetSymbol",
            "executor_fee_asset_decimals",
            "executorFeeAssetDecimals",
            "bridge_type",
            "bridgeType",
            "total_amounts",
            "totalAmounts",
            "total_decimal_amounts",
            "totalDecimalAmounts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            AssetSymbol,
            AssetDecimals,
            Amount,
            DecimalAmount,
            ShieldedAddress,
            RollupFeeAmount,
            RollupFeeDecimalAmount,
            RollupFeeAssetSymbol,
            RollupFeeAssetDecimals,
            DstChainId,
            BridgeFeeAmount,
            BridgeFeeDecimalAmount,
            BridgeFeeAssetSymbol,
            BridgeFeeAssetDecimals,
            ExecutorFeeAmount,
            ExecutorFeeDecimalAmount,
            ExecutorFeeAssetSymbol,
            ExecutorFeeAssetDecimals,
            BridgeType,
            TotalAmounts,
            TotalDecimalAmounts,
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
                            "assetDecimals" | "asset_decimals" => Ok(GeneratedField::AssetDecimals),
                            "amount" => Ok(GeneratedField::Amount),
                            "decimalAmount" | "decimal_amount" => Ok(GeneratedField::DecimalAmount),
                            "shieldedAddress" | "shielded_address" => Ok(GeneratedField::ShieldedAddress),
                            "rollupFeeAmount" | "rollup_fee_amount" => Ok(GeneratedField::RollupFeeAmount),
                            "rollupFeeDecimalAmount" | "rollup_fee_decimal_amount" => Ok(GeneratedField::RollupFeeDecimalAmount),
                            "rollupFeeAssetSymbol" | "rollup_fee_asset_symbol" => Ok(GeneratedField::RollupFeeAssetSymbol),
                            "rollupFeeAssetDecimals" | "rollup_fee_asset_decimals" => Ok(GeneratedField::RollupFeeAssetDecimals),
                            "dstChainId" | "dst_chain_id" => Ok(GeneratedField::DstChainId),
                            "bridgeFeeAmount" | "bridge_fee_amount" => Ok(GeneratedField::BridgeFeeAmount),
                            "bridgeFeeDecimalAmount" | "bridge_fee_decimal_amount" => Ok(GeneratedField::BridgeFeeDecimalAmount),
                            "bridgeFeeAssetSymbol" | "bridge_fee_asset_symbol" => Ok(GeneratedField::BridgeFeeAssetSymbol),
                            "bridgeFeeAssetDecimals" | "bridge_fee_asset_decimals" => Ok(GeneratedField::BridgeFeeAssetDecimals),
                            "executorFeeAmount" | "executor_fee_amount" => Ok(GeneratedField::ExecutorFeeAmount),
                            "executorFeeDecimalAmount" | "executor_fee_decimal_amount" => Ok(GeneratedField::ExecutorFeeDecimalAmount),
                            "executorFeeAssetSymbol" | "executor_fee_asset_symbol" => Ok(GeneratedField::ExecutorFeeAssetSymbol),
                            "executorFeeAssetDecimals" | "executor_fee_asset_decimals" => Ok(GeneratedField::ExecutorFeeAssetDecimals),
                            "bridgeType" | "bridge_type" => Ok(GeneratedField::BridgeType),
                            "totalAmounts" | "total_amounts" => Ok(GeneratedField::TotalAmounts),
                            "totalDecimalAmounts" | "total_decimal_amounts" => Ok(GeneratedField::TotalDecimalAmounts),
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
                let mut asset_decimals__ = None;
                let mut amount__ = None;
                let mut decimal_amount__ = None;
                let mut shielded_address__ = None;
                let mut rollup_fee_amount__ = None;
                let mut rollup_fee_decimal_amount__ = None;
                let mut rollup_fee_asset_symbol__ = None;
                let mut rollup_fee_asset_decimals__ = None;
                let mut dst_chain_id__ = None;
                let mut bridge_fee_amount__ = None;
                let mut bridge_fee_decimal_amount__ = None;
                let mut bridge_fee_asset_symbol__ = None;
                let mut bridge_fee_asset_decimals__ = None;
                let mut executor_fee_amount__ = None;
                let mut executor_fee_decimal_amount__ = None;
                let mut executor_fee_asset_symbol__ = None;
                let mut executor_fee_asset_decimals__ = None;
                let mut bridge_type__ = None;
                let mut total_amounts__ = None;
                let mut total_decimal_amounts__ = None;
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
                        GeneratedField::RollupFeeDecimalAmount => {
                            if rollup_fee_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeDecimalAmount"));
                            }
                            rollup_fee_decimal_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RollupFeeAssetSymbol => {
                            if rollup_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAssetSymbol"));
                            }
                            rollup_fee_asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::RollupFeeAssetDecimals => {
                            if rollup_fee_asset_decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAssetDecimals"));
                            }
                            rollup_fee_asset_decimals__ = 
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
                        GeneratedField::BridgeFeeDecimalAmount => {
                            if bridge_fee_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFeeDecimalAmount"));
                            }
                            bridge_fee_decimal_amount__ = map.next_value()?;
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
                        GeneratedField::TotalDecimalAmounts => {
                            if total_decimal_amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalDecimalAmounts"));
                            }
                            total_decimal_amounts__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(DepositSummary {
                    chain_id: chain_id__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    asset_decimals: asset_decimals__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    decimal_amount: decimal_amount__.unwrap_or_default(),
                    shielded_address: shielded_address__.unwrap_or_default(),
                    rollup_fee_amount: rollup_fee_amount__.unwrap_or_default(),
                    rollup_fee_decimal_amount: rollup_fee_decimal_amount__.unwrap_or_default(),
                    rollup_fee_asset_symbol: rollup_fee_asset_symbol__.unwrap_or_default(),
                    rollup_fee_asset_decimals: rollup_fee_asset_decimals__.unwrap_or_default(),
                    dst_chain_id: dst_chain_id__,
                    bridge_fee_amount: bridge_fee_amount__,
                    bridge_fee_decimal_amount: bridge_fee_decimal_amount__,
                    bridge_fee_asset_symbol: bridge_fee_asset_symbol__,
                    bridge_fee_asset_decimals: bridge_fee_asset_decimals__,
                    executor_fee_amount: executor_fee_amount__,
                    executor_fee_decimal_amount: executor_fee_decimal_amount__,
                    executor_fee_asset_symbol: executor_fee_asset_symbol__,
                    executor_fee_asset_decimals: executor_fee_asset_decimals__,
                    bridge_type: bridge_type__,
                    total_amounts: total_amounts__.unwrap_or_default(),
                    total_decimal_amounts: total_decimal_amounts__.unwrap_or_default(),
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
        if self.query_timeout_ms.is_some() {
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
        if let Some(v) = self.query_timeout_ms.as_ref() {
            struct_ser.serialize_field("queryTimeoutMs", ToString::to_string(&v).as_str())?;
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
            "query_timeout_ms",
            "queryTimeoutMs",
            "dst_chain_id",
            "dstChainId",
            "bridge_type",
            "bridgeType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            AssetSymbol,
            QueryTimeoutMs,
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
                            "queryTimeoutMs" | "query_timeout_ms" => Ok(GeneratedField::QueryTimeoutMs),
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
                let mut query_timeout_ms__ = None;
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
                        GeneratedField::QueryTimeoutMs => {
                            if query_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryTimeoutMs"));
                            }
                            query_timeout_ms__ = 
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
                    query_timeout_ms: query_timeout_ms__,
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
        if self.query_timeout_ms.is_some() {
            len += 1;
        }
        if self.asset_approve_confirmations.is_some() {
            len += 1;
        }
        if self.deposit_confirmations.is_some() {
            len += 1;
        }
        if self.tx_send_timeout_ms.is_some() {
            len += 1;
        }
        if self.tx_wait_timeout_ms.is_some() {
            len += 1;
        }
        if self.tx_wait_interval_ms.is_some() {
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
        if let Some(v) = self.query_timeout_ms.as_ref() {
            struct_ser.serialize_field("queryTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.asset_approve_confirmations.as_ref() {
            struct_ser.serialize_field("assetApproveConfirmations", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.deposit_confirmations.as_ref() {
            struct_ser.serialize_field("depositConfirmations", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.tx_send_timeout_ms.as_ref() {
            struct_ser.serialize_field("txSendTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.tx_wait_timeout_ms.as_ref() {
            struct_ser.serialize_field("txWaitTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.tx_wait_interval_ms.as_ref() {
            struct_ser.serialize_field("txWaitIntervalMs", ToString::to_string(&v).as_str())?;
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
            "query_timeout_ms",
            "queryTimeoutMs",
            "asset_approve_confirmations",
            "assetApproveConfirmations",
            "deposit_confirmations",
            "depositConfirmations",
            "tx_send_timeout_ms",
            "txSendTimeoutMs",
            "tx_wait_timeout_ms",
            "txWaitTimeoutMs",
            "tx_wait_interval_ms",
            "txWaitIntervalMs",
            "asset_approve_tx",
            "assetApproveTx",
            "deposit_tx",
            "depositTx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DepositId,
            PrivateKey,
            QueryTimeoutMs,
            AssetApproveConfirmations,
            DepositConfirmations,
            TxSendTimeoutMs,
            TxWaitTimeoutMs,
            TxWaitIntervalMs,
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
                            "queryTimeoutMs" | "query_timeout_ms" => Ok(GeneratedField::QueryTimeoutMs),
                            "assetApproveConfirmations" | "asset_approve_confirmations" => Ok(GeneratedField::AssetApproveConfirmations),
                            "depositConfirmations" | "deposit_confirmations" => Ok(GeneratedField::DepositConfirmations),
                            "txSendTimeoutMs" | "tx_send_timeout_ms" => Ok(GeneratedField::TxSendTimeoutMs),
                            "txWaitTimeoutMs" | "tx_wait_timeout_ms" => Ok(GeneratedField::TxWaitTimeoutMs),
                            "txWaitIntervalMs" | "tx_wait_interval_ms" => Ok(GeneratedField::TxWaitIntervalMs),
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
                let mut query_timeout_ms__ = None;
                let mut asset_approve_confirmations__ = None;
                let mut deposit_confirmations__ = None;
                let mut tx_send_timeout_ms__ = None;
                let mut tx_wait_timeout_ms__ = None;
                let mut tx_wait_interval_ms__ = None;
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
                        GeneratedField::QueryTimeoutMs => {
                            if query_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryTimeoutMs"));
                            }
                            query_timeout_ms__ = 
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
                        GeneratedField::TxSendTimeoutMs => {
                            if tx_send_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txSendTimeoutMs"));
                            }
                            tx_send_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TxWaitTimeoutMs => {
                            if tx_wait_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txWaitTimeoutMs"));
                            }
                            tx_wait_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TxWaitIntervalMs => {
                            if tx_wait_interval_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txWaitIntervalMs"));
                            }
                            tx_wait_interval_ms__ = 
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
                    query_timeout_ms: query_timeout_ms__,
                    asset_approve_confirmations: asset_approve_confirmations__,
                    deposit_confirmations: deposit_confirmations__,
                    tx_send_timeout_ms: tx_send_timeout_ms__,
                    tx_wait_timeout_ms: tx_wait_timeout_ms__,
                    tx_wait_interval_ms: tx_wait_interval_ms__,
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
