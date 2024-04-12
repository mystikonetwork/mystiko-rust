// @generated
impl serde::Serialize for AmountRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min != 0. {
            len += 1;
        }
        if !self.decimal_min.is_empty() {
            len += 1;
        }
        if self.max != 0. {
            len += 1;
        }
        if !self.decimal_max.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.AmountRange", len)?;
        if self.min != 0. {
            struct_ser.serialize_field("min", &self.min)?;
        }
        if !self.decimal_min.is_empty() {
            struct_ser.serialize_field("decimalMin", &self.decimal_min)?;
        }
        if self.max != 0. {
            struct_ser.serialize_field("max", &self.max)?;
        }
        if !self.decimal_max.is_empty() {
            struct_ser.serialize_field("decimalMax", &self.decimal_max)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AmountRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min",
            "decimal_min",
            "decimalMin",
            "max",
            "decimal_max",
            "decimalMax",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Min,
            DecimalMin,
            Max,
            DecimalMax,
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
                            "min" => Ok(GeneratedField::Min),
                            "decimalMin" | "decimal_min" => Ok(GeneratedField::DecimalMin),
                            "max" => Ok(GeneratedField::Max),
                            "decimalMax" | "decimal_max" => Ok(GeneratedField::DecimalMax),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AmountRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.AmountRange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AmountRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min__ = None;
                let mut decimal_min__ = None;
                let mut max__ = None;
                let mut decimal_max__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Min => {
                            if min__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            min__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DecimalMin => {
                            if decimal_min__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decimalMin"));
                            }
                            decimal_min__ = Some(map.next_value()?);
                        }
                        GeneratedField::Max => {
                            if max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            max__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DecimalMax => {
                            if decimal_max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decimalMax"));
                            }
                            decimal_max__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AmountRange {
                    min: min__.unwrap_or_default(),
                    decimal_min: decimal_min__.unwrap_or_default(),
                    max: max__.unwrap_or_default(),
                    decimal_max: decimal_max__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.AmountRange", FIELDS, GeneratedVisitor)
    }
}
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
        if self.dst_chain_id.is_some() {
            len += 1;
        }
        if self.rollup_fee_amount.is_some() {
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
        if let Some(v) = self.dst_chain_id.as_ref() {
            struct_ser.serialize_field("dstChainId", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.rollup_fee_amount.as_ref() {
            struct_ser.serialize_field("rollupFeeAmount", v)?;
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
            "dst_chain_id",
            "dstChainId",
            "rollup_fee_amount",
            "rollupFeeAmount",
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
            DstChainId,
            RollupFeeAmount,
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
                            "dstChainId" | "dst_chain_id" => Ok(GeneratedField::DstChainId),
                            "rollupFeeAmount" | "rollup_fee_amount" => Ok(GeneratedField::RollupFeeAmount),
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
                let mut dst_chain_id__ = None;
                let mut rollup_fee_amount__ = None;
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
                        GeneratedField::DstChainId => {
                            if dst_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dstChainId"));
                            }
                            dst_chain_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RollupFeeAmount => {
                            if rollup_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAmount"));
                            }
                            rollup_fee_amount__ = 
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
                    dst_chain_id: dst_chain_id__,
                    rollup_fee_amount: rollup_fee_amount__,
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
impl serde::Serialize for CreateSpendOptions {
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
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.wallet_password.is_empty() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.rollup_fee_amount.is_some() {
            len += 1;
        }
        if self.gas_relayer.is_some() {
            len += 1;
        }
        if self.query_timeout_ms.is_some() {
            len += 1;
        }
        if self.spend_quote.is_some() {
            len += 1;
        }
        if self.spend_type.is_some() {
            len += 1;
        }
        if self.bridge_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.CreateSpendOptions", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.amount != 0. {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.wallet_password.is_empty() {
            struct_ser.serialize_field("walletPassword", &self.wallet_password)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.rollup_fee_amount.as_ref() {
            struct_ser.serialize_field("rollupFeeAmount", v)?;
        }
        if let Some(v) = self.gas_relayer.as_ref() {
            struct_ser.serialize_field("gasRelayer", v)?;
        }
        if let Some(v) = self.query_timeout_ms.as_ref() {
            struct_ser.serialize_field("queryTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.spend_quote.as_ref() {
            struct_ser.serialize_field("spendQuote", v)?;
        }
        if let Some(v) = self.spend_type.as_ref() {
            let v = super::super::v1::SpendType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("spendType", &v)?;
        }
        if let Some(v) = self.bridge_type.as_ref() {
            let v = super::super::super::common::v1::BridgeType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSpendOptions {
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
            "recipient",
            "wallet_password",
            "walletPassword",
            "version",
            "rollup_fee_amount",
            "rollupFeeAmount",
            "gas_relayer",
            "gasRelayer",
            "query_timeout_ms",
            "queryTimeoutMs",
            "spend_quote",
            "spendQuote",
            "spend_type",
            "spendType",
            "bridge_type",
            "bridgeType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            AssetSymbol,
            Amount,
            Recipient,
            WalletPassword,
            Version,
            RollupFeeAmount,
            GasRelayer,
            QueryTimeoutMs,
            SpendQuote,
            SpendType,
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
                            "recipient" => Ok(GeneratedField::Recipient),
                            "walletPassword" | "wallet_password" => Ok(GeneratedField::WalletPassword),
                            "version" => Ok(GeneratedField::Version),
                            "rollupFeeAmount" | "rollup_fee_amount" => Ok(GeneratedField::RollupFeeAmount),
                            "gasRelayer" | "gas_relayer" => Ok(GeneratedField::GasRelayer),
                            "queryTimeoutMs" | "query_timeout_ms" => Ok(GeneratedField::QueryTimeoutMs),
                            "spendQuote" | "spend_quote" => Ok(GeneratedField::SpendQuote),
                            "spendType" | "spend_type" => Ok(GeneratedField::SpendType),
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
            type Value = CreateSpendOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.CreateSpendOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateSpendOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut asset_symbol__ = None;
                let mut amount__ = None;
                let mut recipient__ = None;
                let mut wallet_password__ = None;
                let mut version__ = None;
                let mut rollup_fee_amount__ = None;
                let mut gas_relayer__ = None;
                let mut query_timeout_ms__ = None;
                let mut spend_quote__ = None;
                let mut spend_type__ = None;
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
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map.next_value()?);
                        }
                        GeneratedField::WalletPassword => {
                            if wallet_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletPassword"));
                            }
                            wallet_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RollupFeeAmount => {
                            if rollup_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeAmount"));
                            }
                            rollup_fee_amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::GasRelayer => {
                            if gas_relayer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayer"));
                            }
                            gas_relayer__ = map.next_value()?;
                        }
                        GeneratedField::QueryTimeoutMs => {
                            if query_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryTimeoutMs"));
                            }
                            query_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SpendQuote => {
                            if spend_quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spendQuote"));
                            }
                            spend_quote__ = map.next_value()?;
                        }
                        GeneratedField::SpendType => {
                            if spend_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spendType"));
                            }
                            spend_type__ = map.next_value::<::std::option::Option<super::super::v1::SpendType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = map.next_value::<::std::option::Option<super::super::super::common::v1::BridgeType>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(CreateSpendOptions {
                    chain_id: chain_id__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    wallet_password: wallet_password__.unwrap_or_default(),
                    version: version__,
                    rollup_fee_amount: rollup_fee_amount__,
                    gas_relayer: gas_relayer__,
                    query_timeout_ms: query_timeout_ms__,
                    spend_quote: spend_quote__,
                    spend_type: spend_type__,
                    bridge_type: bridge_type__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.CreateSpendOptions", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for GasRelayer {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if self.min_gas_fee != 0. {
            len += 1;
        }
        if !self.min_gas_fee_decimal.is_empty() {
            len += 1;
        }
        if self.service_fee_ratio != 0. {
            len += 1;
        }
        if self.service_fee_of_ten_thousandth != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.GasRelayer", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.min_gas_fee != 0. {
            struct_ser.serialize_field("minGasFee", &self.min_gas_fee)?;
        }
        if !self.min_gas_fee_decimal.is_empty() {
            struct_ser.serialize_field("minGasFeeDecimal", &self.min_gas_fee_decimal)?;
        }
        if self.service_fee_ratio != 0. {
            struct_ser.serialize_field("serviceFeeRatio", &self.service_fee_ratio)?;
        }
        if self.service_fee_of_ten_thousandth != 0 {
            struct_ser.serialize_field("serviceFeeOfTenThousandth", &self.service_fee_of_ten_thousandth)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GasRelayer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "name",
            "address",
            "min_gas_fee",
            "minGasFee",
            "min_gas_fee_decimal",
            "minGasFeeDecimal",
            "service_fee_ratio",
            "serviceFeeRatio",
            "service_fee_of_ten_thousandth",
            "serviceFeeOfTenThousandth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            Name,
            Address,
            MinGasFee,
            MinGasFeeDecimal,
            ServiceFeeRatio,
            ServiceFeeOfTenThousandth,
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
                            "name" => Ok(GeneratedField::Name),
                            "address" => Ok(GeneratedField::Address),
                            "minGasFee" | "min_gas_fee" => Ok(GeneratedField::MinGasFee),
                            "minGasFeeDecimal" | "min_gas_fee_decimal" => Ok(GeneratedField::MinGasFeeDecimal),
                            "serviceFeeRatio" | "service_fee_ratio" => Ok(GeneratedField::ServiceFeeRatio),
                            "serviceFeeOfTenThousandth" | "service_fee_of_ten_thousandth" => Ok(GeneratedField::ServiceFeeOfTenThousandth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GasRelayer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.GasRelayer")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GasRelayer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut name__ = None;
                let mut address__ = None;
                let mut min_gas_fee__ = None;
                let mut min_gas_fee_decimal__ = None;
                let mut service_fee_ratio__ = None;
                let mut service_fee_of_ten_thousandth__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map.next_value()?);
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
                        GeneratedField::MinGasFee => {
                            if min_gas_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minGasFee"));
                            }
                            min_gas_fee__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinGasFeeDecimal => {
                            if min_gas_fee_decimal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minGasFeeDecimal"));
                            }
                            min_gas_fee_decimal__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServiceFeeRatio => {
                            if service_fee_ratio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceFeeRatio"));
                            }
                            service_fee_ratio__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ServiceFeeOfTenThousandth => {
                            if service_fee_of_ten_thousandth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceFeeOfTenThousandth"));
                            }
                            service_fee_of_ten_thousandth__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GasRelayer {
                    url: url__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    min_gas_fee: min_gas_fee__.unwrap_or_default(),
                    min_gas_fee_decimal: min_gas_fee_decimal__.unwrap_or_default(),
                    service_fee_ratio: service_fee_ratio__.unwrap_or_default(),
                    service_fee_of_ten_thousandth: service_fee_of_ten_thousandth__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.GasRelayer", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for QuoteSpendOptions {
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
        if self.version.is_some() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.use_relayer.is_some() {
            len += 1;
        }
        if self.query_timeout_ms.is_some() {
            len += 1;
        }
        if self.spend_type.is_some() {
            len += 1;
        }
        if self.bridge_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.QuoteSpendOptions", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if let Some(v) = self.use_relayer.as_ref() {
            struct_ser.serialize_field("useRelayer", v)?;
        }
        if let Some(v) = self.query_timeout_ms.as_ref() {
            struct_ser.serialize_field("queryTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.spend_type.as_ref() {
            let v = super::super::v1::SpendType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("spendType", &v)?;
        }
        if let Some(v) = self.bridge_type.as_ref() {
            let v = super::super::super::common::v1::BridgeType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("bridgeType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuoteSpendOptions {
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
            "version",
            "amount",
            "use_relayer",
            "useRelayer",
            "query_timeout_ms",
            "queryTimeoutMs",
            "spend_type",
            "spendType",
            "bridge_type",
            "bridgeType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            AssetSymbol,
            Version,
            Amount,
            UseRelayer,
            QueryTimeoutMs,
            SpendType,
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
                            "version" => Ok(GeneratedField::Version),
                            "amount" => Ok(GeneratedField::Amount),
                            "useRelayer" | "use_relayer" => Ok(GeneratedField::UseRelayer),
                            "queryTimeoutMs" | "query_timeout_ms" => Ok(GeneratedField::QueryTimeoutMs),
                            "spendType" | "spend_type" => Ok(GeneratedField::SpendType),
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
            type Value = QuoteSpendOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.QuoteSpendOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuoteSpendOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut asset_symbol__ = None;
                let mut version__ = None;
                let mut amount__ = None;
                let mut use_relayer__ = None;
                let mut query_timeout_ms__ = None;
                let mut spend_type__ = None;
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
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::UseRelayer => {
                            if use_relayer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useRelayer"));
                            }
                            use_relayer__ = map.next_value()?;
                        }
                        GeneratedField::QueryTimeoutMs => {
                            if query_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryTimeoutMs"));
                            }
                            query_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SpendType => {
                            if spend_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spendType"));
                            }
                            spend_type__ = map.next_value::<::std::option::Option<super::super::v1::SpendType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::BridgeType => {
                            if bridge_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeType"));
                            }
                            bridge_type__ = map.next_value::<::std::option::Option<super::super::super::common::v1::BridgeType>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(QuoteSpendOptions {
                    chain_id: chain_id__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    version: version__,
                    amount: amount__,
                    use_relayer: use_relayer__,
                    query_timeout_ms: query_timeout_ms__,
                    spend_type: spend_type__,
                    bridge_type: bridge_type__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.QuoteSpendOptions", FIELDS, GeneratedVisitor)
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
        if self.signer_provider.is_some() {
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
        if let Some(v) = self.signer_provider.as_ref() {
            struct_ser.serialize_field("signerProvider", v)?;
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
            "signer_provider",
            "signerProvider",
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
            SignerProvider,
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
                            "signerProvider" | "signer_provider" => Ok(GeneratedField::SignerProvider),
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
                let mut signer_provider__ = None;
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
                        GeneratedField::SignerProvider => {
                            if signer_provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signerProvider"));
                            }
                            signer_provider__ = map.next_value()?;
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
                    signer_provider: signer_provider__,
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
impl serde::Serialize for SendSpendOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.spend_id.is_empty() {
            len += 1;
        }
        if !self.wallet_password.is_empty() {
            len += 1;
        }
        if self.private_key.is_some() {
            len += 1;
        }
        if self.signer_provider.is_some() {
            len += 1;
        }
        if self.query_timeout_ms.is_some() {
            len += 1;
        }
        if self.spend_confirmations.is_some() {
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
        if self.relayer_wait_timeout_ms.is_some() {
            len += 1;
        }
        if self.relayer_wait_interval_ms.is_some() {
            len += 1;
        }
        if self.tx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.SendSpendOptions", len)?;
        if !self.spend_id.is_empty() {
            struct_ser.serialize_field("spendId", &self.spend_id)?;
        }
        if !self.wallet_password.is_empty() {
            struct_ser.serialize_field("walletPassword", &self.wallet_password)?;
        }
        if let Some(v) = self.private_key.as_ref() {
            struct_ser.serialize_field("privateKey", v)?;
        }
        if let Some(v) = self.signer_provider.as_ref() {
            struct_ser.serialize_field("signerProvider", v)?;
        }
        if let Some(v) = self.query_timeout_ms.as_ref() {
            struct_ser.serialize_field("queryTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.spend_confirmations.as_ref() {
            struct_ser.serialize_field("spendConfirmations", ToString::to_string(&v).as_str())?;
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
        if let Some(v) = self.relayer_wait_timeout_ms.as_ref() {
            struct_ser.serialize_field("relayerWaitTimeoutMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.relayer_wait_interval_ms.as_ref() {
            struct_ser.serialize_field("relayerWaitIntervalMs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.tx.as_ref() {
            struct_ser.serialize_field("tx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendSpendOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spend_id",
            "spendId",
            "wallet_password",
            "walletPassword",
            "private_key",
            "privateKey",
            "signer_provider",
            "signerProvider",
            "query_timeout_ms",
            "queryTimeoutMs",
            "spend_confirmations",
            "spendConfirmations",
            "tx_send_timeout_ms",
            "txSendTimeoutMs",
            "tx_wait_timeout_ms",
            "txWaitTimeoutMs",
            "tx_wait_interval_ms",
            "txWaitIntervalMs",
            "relayer_wait_timeout_ms",
            "relayerWaitTimeoutMs",
            "relayer_wait_interval_ms",
            "relayerWaitIntervalMs",
            "tx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SpendId,
            WalletPassword,
            PrivateKey,
            SignerProvider,
            QueryTimeoutMs,
            SpendConfirmations,
            TxSendTimeoutMs,
            TxWaitTimeoutMs,
            TxWaitIntervalMs,
            RelayerWaitTimeoutMs,
            RelayerWaitIntervalMs,
            Tx,
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
                            "spendId" | "spend_id" => Ok(GeneratedField::SpendId),
                            "walletPassword" | "wallet_password" => Ok(GeneratedField::WalletPassword),
                            "privateKey" | "private_key" => Ok(GeneratedField::PrivateKey),
                            "signerProvider" | "signer_provider" => Ok(GeneratedField::SignerProvider),
                            "queryTimeoutMs" | "query_timeout_ms" => Ok(GeneratedField::QueryTimeoutMs),
                            "spendConfirmations" | "spend_confirmations" => Ok(GeneratedField::SpendConfirmations),
                            "txSendTimeoutMs" | "tx_send_timeout_ms" => Ok(GeneratedField::TxSendTimeoutMs),
                            "txWaitTimeoutMs" | "tx_wait_timeout_ms" => Ok(GeneratedField::TxWaitTimeoutMs),
                            "txWaitIntervalMs" | "tx_wait_interval_ms" => Ok(GeneratedField::TxWaitIntervalMs),
                            "relayerWaitTimeoutMs" | "relayer_wait_timeout_ms" => Ok(GeneratedField::RelayerWaitTimeoutMs),
                            "relayerWaitIntervalMs" | "relayer_wait_interval_ms" => Ok(GeneratedField::RelayerWaitIntervalMs),
                            "tx" => Ok(GeneratedField::Tx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendSpendOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.SendSpendOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SendSpendOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spend_id__ = None;
                let mut wallet_password__ = None;
                let mut private_key__ = None;
                let mut signer_provider__ = None;
                let mut query_timeout_ms__ = None;
                let mut spend_confirmations__ = None;
                let mut tx_send_timeout_ms__ = None;
                let mut tx_wait_timeout_ms__ = None;
                let mut tx_wait_interval_ms__ = None;
                let mut relayer_wait_timeout_ms__ = None;
                let mut relayer_wait_interval_ms__ = None;
                let mut tx__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SpendId => {
                            if spend_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spendId"));
                            }
                            spend_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::WalletPassword => {
                            if wallet_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletPassword"));
                            }
                            wallet_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::PrivateKey => {
                            if private_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKey"));
                            }
                            private_key__ = map.next_value()?;
                        }
                        GeneratedField::SignerProvider => {
                            if signer_provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signerProvider"));
                            }
                            signer_provider__ = map.next_value()?;
                        }
                        GeneratedField::QueryTimeoutMs => {
                            if query_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryTimeoutMs"));
                            }
                            query_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SpendConfirmations => {
                            if spend_confirmations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spendConfirmations"));
                            }
                            spend_confirmations__ = 
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
                        GeneratedField::RelayerWaitTimeoutMs => {
                            if relayer_wait_timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayerWaitTimeoutMs"));
                            }
                            relayer_wait_timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RelayerWaitIntervalMs => {
                            if relayer_wait_interval_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayerWaitIntervalMs"));
                            }
                            relayer_wait_interval_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Tx => {
                            if tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tx"));
                            }
                            tx__ = map.next_value()?;
                        }
                    }
                }
                Ok(SendSpendOptions {
                    spend_id: spend_id__.unwrap_or_default(),
                    wallet_password: wallet_password__.unwrap_or_default(),
                    private_key: private_key__,
                    signer_provider: signer_provider__,
                    query_timeout_ms: query_timeout_ms__,
                    spend_confirmations: spend_confirmations__,
                    tx_send_timeout_ms: tx_send_timeout_ms__,
                    tx_wait_timeout_ms: tx_wait_timeout_ms__,
                    tx_wait_interval_ms: tx_wait_interval_ms__,
                    relayer_wait_timeout_ms: relayer_wait_timeout_ms__,
                    relayer_wait_interval_ms: relayer_wait_interval_ms__,
                    tx: tx__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.SendSpendOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpendInvalidCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SPEND_INVALID_CODE_UNSPECIFIED",
            Self::InvalidAmount => "SPEND_INVALID_CODE_INVALID_AMOUNT",
            Self::SplitDisabled => "SPEND_INVALID_CODE_SPLIT_DISABLED",
            Self::NoAvailableAssets => "SPEND_INVALID_CODE_NO_AVAILABLE_ASSETS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SpendInvalidCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SPEND_INVALID_CODE_UNSPECIFIED",
            "SPEND_INVALID_CODE_INVALID_AMOUNT",
            "SPEND_INVALID_CODE_SPLIT_DISABLED",
            "SPEND_INVALID_CODE_NO_AVAILABLE_ASSETS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpendInvalidCode;

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
                    .and_then(SpendInvalidCode::from_i32)
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
                    .and_then(SpendInvalidCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SPEND_INVALID_CODE_UNSPECIFIED" => Ok(SpendInvalidCode::Unspecified),
                    "SPEND_INVALID_CODE_INVALID_AMOUNT" => Ok(SpendInvalidCode::InvalidAmount),
                    "SPEND_INVALID_CODE_SPLIT_DISABLED" => Ok(SpendInvalidCode::SplitDisabled),
                    "SPEND_INVALID_CODE_NO_AVAILABLE_ASSETS" => Ok(SpendInvalidCode::NoAvailableAssets),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SpendQuote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.valid {
            len += 1;
        }
        if !self.asset_symbol.is_empty() {
            len += 1;
        }
        if self.asset_decimals != 0 {
            len += 1;
        }
        if self.current_balance != 0. {
            len += 1;
        }
        if !self.current_decimal_balance.is_empty() {
            len += 1;
        }
        if self.num_of_inputs != 0 {
            len += 1;
        }
        if self.num_of_outputs != 0 {
            len += 1;
        }
        if self.min_rollup_fee != 0. {
            len += 1;
        }
        if !self.min_rollup_fee_decimal.is_empty() {
            len += 1;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            len += 1;
        }
        if self.rollup_fee_asset_decimals != 0 {
            len += 1;
        }
        if !self.fixed_amounts.is_empty() {
            len += 1;
        }
        if !self.fixed_decimal_amounts.is_empty() {
            len += 1;
        }
        if !self.selected_commitments.is_empty() {
            len += 1;
        }
        if !self.gas_relayers.is_empty() {
            len += 1;
        }
        if self.max_gas_relayer_fee.is_some() {
            len += 1;
        }
        if self.max_gas_relayer_fee_decimal.is_some() {
            len += 1;
        }
        if self.gas_relayer_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.gas_relayer_fee_asset_decimals.is_some() {
            len += 1;
        }
        if self.invalid_code.is_some() {
            len += 1;
        }
        if self.amount_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.SpendQuote", len)?;
        if self.valid {
            struct_ser.serialize_field("valid", &self.valid)?;
        }
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.asset_decimals != 0 {
            struct_ser.serialize_field("assetDecimals", &self.asset_decimals)?;
        }
        if self.current_balance != 0. {
            struct_ser.serialize_field("currentBalance", &self.current_balance)?;
        }
        if !self.current_decimal_balance.is_empty() {
            struct_ser.serialize_field("currentDecimalBalance", &self.current_decimal_balance)?;
        }
        if self.num_of_inputs != 0 {
            struct_ser.serialize_field("numOfInputs", ToString::to_string(&self.num_of_inputs).as_str())?;
        }
        if self.num_of_outputs != 0 {
            struct_ser.serialize_field("numOfOutputs", ToString::to_string(&self.num_of_outputs).as_str())?;
        }
        if self.min_rollup_fee != 0. {
            struct_ser.serialize_field("minRollupFee", &self.min_rollup_fee)?;
        }
        if !self.min_rollup_fee_decimal.is_empty() {
            struct_ser.serialize_field("minRollupFeeDecimal", &self.min_rollup_fee_decimal)?;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            struct_ser.serialize_field("rollupFeeAssetSymbol", &self.rollup_fee_asset_symbol)?;
        }
        if self.rollup_fee_asset_decimals != 0 {
            struct_ser.serialize_field("rollupFeeAssetDecimals", &self.rollup_fee_asset_decimals)?;
        }
        if !self.fixed_amounts.is_empty() {
            struct_ser.serialize_field("fixedAmounts", &self.fixed_amounts)?;
        }
        if !self.fixed_decimal_amounts.is_empty() {
            struct_ser.serialize_field("fixedDecimalAmounts", &self.fixed_decimal_amounts)?;
        }
        if !self.selected_commitments.is_empty() {
            struct_ser.serialize_field("selectedCommitments", &self.selected_commitments)?;
        }
        if !self.gas_relayers.is_empty() {
            struct_ser.serialize_field("gasRelayers", &self.gas_relayers)?;
        }
        if let Some(v) = self.max_gas_relayer_fee.as_ref() {
            struct_ser.serialize_field("maxGasRelayerFee", v)?;
        }
        if let Some(v) = self.max_gas_relayer_fee_decimal.as_ref() {
            struct_ser.serialize_field("maxGasRelayerFeeDecimal", v)?;
        }
        if let Some(v) = self.gas_relayer_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("gasRelayerFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.gas_relayer_fee_asset_decimals.as_ref() {
            struct_ser.serialize_field("gasRelayerFeeAssetDecimals", v)?;
        }
        if let Some(v) = self.invalid_code.as_ref() {
            let v = SpendInvalidCode::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("invalidCode", &v)?;
        }
        if let Some(v) = self.amount_range.as_ref() {
            struct_ser.serialize_field("amountRange", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpendQuote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "valid",
            "asset_symbol",
            "assetSymbol",
            "asset_decimals",
            "assetDecimals",
            "current_balance",
            "currentBalance",
            "current_decimal_balance",
            "currentDecimalBalance",
            "num_of_inputs",
            "numOfInputs",
            "num_of_outputs",
            "numOfOutputs",
            "min_rollup_fee",
            "minRollupFee",
            "min_rollup_fee_decimal",
            "minRollupFeeDecimal",
            "rollup_fee_asset_symbol",
            "rollupFeeAssetSymbol",
            "rollup_fee_asset_decimals",
            "rollupFeeAssetDecimals",
            "fixed_amounts",
            "fixedAmounts",
            "fixed_decimal_amounts",
            "fixedDecimalAmounts",
            "selected_commitments",
            "selectedCommitments",
            "gas_relayers",
            "gasRelayers",
            "max_gas_relayer_fee",
            "maxGasRelayerFee",
            "max_gas_relayer_fee_decimal",
            "maxGasRelayerFeeDecimal",
            "gas_relayer_fee_asset_symbol",
            "gasRelayerFeeAssetSymbol",
            "gas_relayer_fee_asset_decimals",
            "gasRelayerFeeAssetDecimals",
            "invalid_code",
            "invalidCode",
            "amount_range",
            "amountRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Valid,
            AssetSymbol,
            AssetDecimals,
            CurrentBalance,
            CurrentDecimalBalance,
            NumOfInputs,
            NumOfOutputs,
            MinRollupFee,
            MinRollupFeeDecimal,
            RollupFeeAssetSymbol,
            RollupFeeAssetDecimals,
            FixedAmounts,
            FixedDecimalAmounts,
            SelectedCommitments,
            GasRelayers,
            MaxGasRelayerFee,
            MaxGasRelayerFeeDecimal,
            GasRelayerFeeAssetSymbol,
            GasRelayerFeeAssetDecimals,
            InvalidCode,
            AmountRange,
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
                            "valid" => Ok(GeneratedField::Valid),
                            "assetSymbol" | "asset_symbol" => Ok(GeneratedField::AssetSymbol),
                            "assetDecimals" | "asset_decimals" => Ok(GeneratedField::AssetDecimals),
                            "currentBalance" | "current_balance" => Ok(GeneratedField::CurrentBalance),
                            "currentDecimalBalance" | "current_decimal_balance" => Ok(GeneratedField::CurrentDecimalBalance),
                            "numOfInputs" | "num_of_inputs" => Ok(GeneratedField::NumOfInputs),
                            "numOfOutputs" | "num_of_outputs" => Ok(GeneratedField::NumOfOutputs),
                            "minRollupFee" | "min_rollup_fee" => Ok(GeneratedField::MinRollupFee),
                            "minRollupFeeDecimal" | "min_rollup_fee_decimal" => Ok(GeneratedField::MinRollupFeeDecimal),
                            "rollupFeeAssetSymbol" | "rollup_fee_asset_symbol" => Ok(GeneratedField::RollupFeeAssetSymbol),
                            "rollupFeeAssetDecimals" | "rollup_fee_asset_decimals" => Ok(GeneratedField::RollupFeeAssetDecimals),
                            "fixedAmounts" | "fixed_amounts" => Ok(GeneratedField::FixedAmounts),
                            "fixedDecimalAmounts" | "fixed_decimal_amounts" => Ok(GeneratedField::FixedDecimalAmounts),
                            "selectedCommitments" | "selected_commitments" => Ok(GeneratedField::SelectedCommitments),
                            "gasRelayers" | "gas_relayers" => Ok(GeneratedField::GasRelayers),
                            "maxGasRelayerFee" | "max_gas_relayer_fee" => Ok(GeneratedField::MaxGasRelayerFee),
                            "maxGasRelayerFeeDecimal" | "max_gas_relayer_fee_decimal" => Ok(GeneratedField::MaxGasRelayerFeeDecimal),
                            "gasRelayerFeeAssetSymbol" | "gas_relayer_fee_asset_symbol" => Ok(GeneratedField::GasRelayerFeeAssetSymbol),
                            "gasRelayerFeeAssetDecimals" | "gas_relayer_fee_asset_decimals" => Ok(GeneratedField::GasRelayerFeeAssetDecimals),
                            "invalidCode" | "invalid_code" => Ok(GeneratedField::InvalidCode),
                            "amountRange" | "amount_range" => Ok(GeneratedField::AmountRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpendQuote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.SpendQuote")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SpendQuote, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut valid__ = None;
                let mut asset_symbol__ = None;
                let mut asset_decimals__ = None;
                let mut current_balance__ = None;
                let mut current_decimal_balance__ = None;
                let mut num_of_inputs__ = None;
                let mut num_of_outputs__ = None;
                let mut min_rollup_fee__ = None;
                let mut min_rollup_fee_decimal__ = None;
                let mut rollup_fee_asset_symbol__ = None;
                let mut rollup_fee_asset_decimals__ = None;
                let mut fixed_amounts__ = None;
                let mut fixed_decimal_amounts__ = None;
                let mut selected_commitments__ = None;
                let mut gas_relayers__ = None;
                let mut max_gas_relayer_fee__ = None;
                let mut max_gas_relayer_fee_decimal__ = None;
                let mut gas_relayer_fee_asset_symbol__ = None;
                let mut gas_relayer_fee_asset_decimals__ = None;
                let mut invalid_code__ = None;
                let mut amount_range__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Valid => {
                            if valid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valid"));
                            }
                            valid__ = Some(map.next_value()?);
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
                        GeneratedField::CurrentBalance => {
                            if current_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentBalance"));
                            }
                            current_balance__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CurrentDecimalBalance => {
                            if current_decimal_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentDecimalBalance"));
                            }
                            current_decimal_balance__ = Some(map.next_value()?);
                        }
                        GeneratedField::NumOfInputs => {
                            if num_of_inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOfInputs"));
                            }
                            num_of_inputs__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumOfOutputs => {
                            if num_of_outputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOfOutputs"));
                            }
                            num_of_outputs__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinRollupFee => {
                            if min_rollup_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minRollupFee"));
                            }
                            min_rollup_fee__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinRollupFeeDecimal => {
                            if min_rollup_fee_decimal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minRollupFeeDecimal"));
                            }
                            min_rollup_fee_decimal__ = Some(map.next_value()?);
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
                        GeneratedField::FixedAmounts => {
                            if fixed_amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAmounts"));
                            }
                            fixed_amounts__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::FixedDecimalAmounts => {
                            if fixed_decimal_amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDecimalAmounts"));
                            }
                            fixed_decimal_amounts__ = Some(map.next_value()?);
                        }
                        GeneratedField::SelectedCommitments => {
                            if selected_commitments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selectedCommitments"));
                            }
                            selected_commitments__ = Some(map.next_value()?);
                        }
                        GeneratedField::GasRelayers => {
                            if gas_relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayers"));
                            }
                            gas_relayers__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxGasRelayerFee => {
                            if max_gas_relayer_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxGasRelayerFee"));
                            }
                            max_gas_relayer_fee__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxGasRelayerFeeDecimal => {
                            if max_gas_relayer_fee_decimal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxGasRelayerFeeDecimal"));
                            }
                            max_gas_relayer_fee_decimal__ = map.next_value()?;
                        }
                        GeneratedField::GasRelayerFeeAssetSymbol => {
                            if gas_relayer_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerFeeAssetSymbol"));
                            }
                            gas_relayer_fee_asset_symbol__ = map.next_value()?;
                        }
                        GeneratedField::GasRelayerFeeAssetDecimals => {
                            if gas_relayer_fee_asset_decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerFeeAssetDecimals"));
                            }
                            gas_relayer_fee_asset_decimals__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::InvalidCode => {
                            if invalid_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invalidCode"));
                            }
                            invalid_code__ = map.next_value::<::std::option::Option<SpendInvalidCode>>()?.map(|x| x as i32);
                        }
                        GeneratedField::AmountRange => {
                            if amount_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountRange"));
                            }
                            amount_range__ = map.next_value()?;
                        }
                    }
                }
                Ok(SpendQuote {
                    valid: valid__.unwrap_or_default(),
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    asset_decimals: asset_decimals__.unwrap_or_default(),
                    current_balance: current_balance__.unwrap_or_default(),
                    current_decimal_balance: current_decimal_balance__.unwrap_or_default(),
                    num_of_inputs: num_of_inputs__.unwrap_or_default(),
                    num_of_outputs: num_of_outputs__.unwrap_or_default(),
                    min_rollup_fee: min_rollup_fee__.unwrap_or_default(),
                    min_rollup_fee_decimal: min_rollup_fee_decimal__.unwrap_or_default(),
                    rollup_fee_asset_symbol: rollup_fee_asset_symbol__.unwrap_or_default(),
                    rollup_fee_asset_decimals: rollup_fee_asset_decimals__.unwrap_or_default(),
                    fixed_amounts: fixed_amounts__.unwrap_or_default(),
                    fixed_decimal_amounts: fixed_decimal_amounts__.unwrap_or_default(),
                    selected_commitments: selected_commitments__.unwrap_or_default(),
                    gas_relayers: gas_relayers__.unwrap_or_default(),
                    max_gas_relayer_fee: max_gas_relayer_fee__,
                    max_gas_relayer_fee_decimal: max_gas_relayer_fee_decimal__,
                    gas_relayer_fee_asset_symbol: gas_relayer_fee_asset_symbol__,
                    gas_relayer_fee_asset_decimals: gas_relayer_fee_asset_decimals__,
                    invalid_code: invalid_code__,
                    amount_range: amount_range__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.SpendQuote", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpendSummary {
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
        if self.current_balance != 0. {
            len += 1;
        }
        if !self.current_decimal_balance.is_empty() {
            len += 1;
        }
        if self.new_balance != 0. {
            len += 1;
        }
        if !self.new_decimal_balance.is_empty() {
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
        if self.rollup_fee_amount != 0. {
            len += 1;
        }
        if !self.rollup_fee_decimal_amount.is_empty() {
            len += 1;
        }
        if self.rollup_fee_total_amount != 0. {
            len += 1;
        }
        if !self.rollup_fee_total_decimal_amount.is_empty() {
            len += 1;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            len += 1;
        }
        if self.rollup_fee_asset_decimals != 0 {
            len += 1;
        }
        if self.gas_relayer_fee_amount.is_some() {
            len += 1;
        }
        if self.gas_relayer_fee_decimal_amount.is_some() {
            len += 1;
        }
        if self.gas_relayer_fee_asset_symbol.is_some() {
            len += 1;
        }
        if self.gas_relayer_fee_asset_decimals.is_some() {
            len += 1;
        }
        if self.gas_relayer_address.is_some() {
            len += 1;
        }
        if self.gas_relayer_name.is_some() {
            len += 1;
        }
        if self.gas_relayer_url.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.SpendSummary", len)?;
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.asset_decimals != 0 {
            struct_ser.serialize_field("assetDecimals", &self.asset_decimals)?;
        }
        if self.current_balance != 0. {
            struct_ser.serialize_field("currentBalance", &self.current_balance)?;
        }
        if !self.current_decimal_balance.is_empty() {
            struct_ser.serialize_field("currentDecimalBalance", &self.current_decimal_balance)?;
        }
        if self.new_balance != 0. {
            struct_ser.serialize_field("newBalance", &self.new_balance)?;
        }
        if !self.new_decimal_balance.is_empty() {
            struct_ser.serialize_field("newDecimalBalance", &self.new_decimal_balance)?;
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
        if self.rollup_fee_amount != 0. {
            struct_ser.serialize_field("rollupFeeAmount", &self.rollup_fee_amount)?;
        }
        if !self.rollup_fee_decimal_amount.is_empty() {
            struct_ser.serialize_field("rollupFeeDecimalAmount", &self.rollup_fee_decimal_amount)?;
        }
        if self.rollup_fee_total_amount != 0. {
            struct_ser.serialize_field("rollupFeeTotalAmount", &self.rollup_fee_total_amount)?;
        }
        if !self.rollup_fee_total_decimal_amount.is_empty() {
            struct_ser.serialize_field("rollupFeeTotalDecimalAmount", &self.rollup_fee_total_decimal_amount)?;
        }
        if !self.rollup_fee_asset_symbol.is_empty() {
            struct_ser.serialize_field("rollupFeeAssetSymbol", &self.rollup_fee_asset_symbol)?;
        }
        if self.rollup_fee_asset_decimals != 0 {
            struct_ser.serialize_field("rollupFeeAssetDecimals", &self.rollup_fee_asset_decimals)?;
        }
        if let Some(v) = self.gas_relayer_fee_amount.as_ref() {
            struct_ser.serialize_field("gasRelayerFeeAmount", v)?;
        }
        if let Some(v) = self.gas_relayer_fee_decimal_amount.as_ref() {
            struct_ser.serialize_field("gasRelayerFeeDecimalAmount", v)?;
        }
        if let Some(v) = self.gas_relayer_fee_asset_symbol.as_ref() {
            struct_ser.serialize_field("gasRelayerFeeAssetSymbol", v)?;
        }
        if let Some(v) = self.gas_relayer_fee_asset_decimals.as_ref() {
            struct_ser.serialize_field("gasRelayerFeeAssetDecimals", v)?;
        }
        if let Some(v) = self.gas_relayer_address.as_ref() {
            struct_ser.serialize_field("gasRelayerAddress", v)?;
        }
        if let Some(v) = self.gas_relayer_name.as_ref() {
            struct_ser.serialize_field("gasRelayerName", v)?;
        }
        if let Some(v) = self.gas_relayer_url.as_ref() {
            struct_ser.serialize_field("gasRelayerUrl", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpendSummary {
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
            "current_balance",
            "currentBalance",
            "current_decimal_balance",
            "currentDecimalBalance",
            "new_balance",
            "newBalance",
            "new_decimal_balance",
            "newDecimalBalance",
            "amount",
            "decimal_amount",
            "decimalAmount",
            "recipient",
            "rollup_fee_amount",
            "rollupFeeAmount",
            "rollup_fee_decimal_amount",
            "rollupFeeDecimalAmount",
            "rollup_fee_total_amount",
            "rollupFeeTotalAmount",
            "rollup_fee_total_decimal_amount",
            "rollupFeeTotalDecimalAmount",
            "rollup_fee_asset_symbol",
            "rollupFeeAssetSymbol",
            "rollup_fee_asset_decimals",
            "rollupFeeAssetDecimals",
            "gas_relayer_fee_amount",
            "gasRelayerFeeAmount",
            "gas_relayer_fee_decimal_amount",
            "gasRelayerFeeDecimalAmount",
            "gas_relayer_fee_asset_symbol",
            "gasRelayerFeeAssetSymbol",
            "gas_relayer_fee_asset_decimals",
            "gasRelayerFeeAssetDecimals",
            "gas_relayer_address",
            "gasRelayerAddress",
            "gas_relayer_name",
            "gasRelayerName",
            "gas_relayer_url",
            "gasRelayerUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetSymbol,
            AssetDecimals,
            CurrentBalance,
            CurrentDecimalBalance,
            NewBalance,
            NewDecimalBalance,
            Amount,
            DecimalAmount,
            Recipient,
            RollupFeeAmount,
            RollupFeeDecimalAmount,
            RollupFeeTotalAmount,
            RollupFeeTotalDecimalAmount,
            RollupFeeAssetSymbol,
            RollupFeeAssetDecimals,
            GasRelayerFeeAmount,
            GasRelayerFeeDecimalAmount,
            GasRelayerFeeAssetSymbol,
            GasRelayerFeeAssetDecimals,
            GasRelayerAddress,
            GasRelayerName,
            GasRelayerUrl,
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
                            "currentBalance" | "current_balance" => Ok(GeneratedField::CurrentBalance),
                            "currentDecimalBalance" | "current_decimal_balance" => Ok(GeneratedField::CurrentDecimalBalance),
                            "newBalance" | "new_balance" => Ok(GeneratedField::NewBalance),
                            "newDecimalBalance" | "new_decimal_balance" => Ok(GeneratedField::NewDecimalBalance),
                            "amount" => Ok(GeneratedField::Amount),
                            "decimalAmount" | "decimal_amount" => Ok(GeneratedField::DecimalAmount),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "rollupFeeAmount" | "rollup_fee_amount" => Ok(GeneratedField::RollupFeeAmount),
                            "rollupFeeDecimalAmount" | "rollup_fee_decimal_amount" => Ok(GeneratedField::RollupFeeDecimalAmount),
                            "rollupFeeTotalAmount" | "rollup_fee_total_amount" => Ok(GeneratedField::RollupFeeTotalAmount),
                            "rollupFeeTotalDecimalAmount" | "rollup_fee_total_decimal_amount" => Ok(GeneratedField::RollupFeeTotalDecimalAmount),
                            "rollupFeeAssetSymbol" | "rollup_fee_asset_symbol" => Ok(GeneratedField::RollupFeeAssetSymbol),
                            "rollupFeeAssetDecimals" | "rollup_fee_asset_decimals" => Ok(GeneratedField::RollupFeeAssetDecimals),
                            "gasRelayerFeeAmount" | "gas_relayer_fee_amount" => Ok(GeneratedField::GasRelayerFeeAmount),
                            "gasRelayerFeeDecimalAmount" | "gas_relayer_fee_decimal_amount" => Ok(GeneratedField::GasRelayerFeeDecimalAmount),
                            "gasRelayerFeeAssetSymbol" | "gas_relayer_fee_asset_symbol" => Ok(GeneratedField::GasRelayerFeeAssetSymbol),
                            "gasRelayerFeeAssetDecimals" | "gas_relayer_fee_asset_decimals" => Ok(GeneratedField::GasRelayerFeeAssetDecimals),
                            "gasRelayerAddress" | "gas_relayer_address" => Ok(GeneratedField::GasRelayerAddress),
                            "gasRelayerName" | "gas_relayer_name" => Ok(GeneratedField::GasRelayerName),
                            "gasRelayerUrl" | "gas_relayer_url" => Ok(GeneratedField::GasRelayerUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpendSummary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.SpendSummary")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SpendSummary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_symbol__ = None;
                let mut asset_decimals__ = None;
                let mut current_balance__ = None;
                let mut current_decimal_balance__ = None;
                let mut new_balance__ = None;
                let mut new_decimal_balance__ = None;
                let mut amount__ = None;
                let mut decimal_amount__ = None;
                let mut recipient__ = None;
                let mut rollup_fee_amount__ = None;
                let mut rollup_fee_decimal_amount__ = None;
                let mut rollup_fee_total_amount__ = None;
                let mut rollup_fee_total_decimal_amount__ = None;
                let mut rollup_fee_asset_symbol__ = None;
                let mut rollup_fee_asset_decimals__ = None;
                let mut gas_relayer_fee_amount__ = None;
                let mut gas_relayer_fee_decimal_amount__ = None;
                let mut gas_relayer_fee_asset_symbol__ = None;
                let mut gas_relayer_fee_asset_decimals__ = None;
                let mut gas_relayer_address__ = None;
                let mut gas_relayer_name__ = None;
                let mut gas_relayer_url__ = None;
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
                        GeneratedField::CurrentBalance => {
                            if current_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentBalance"));
                            }
                            current_balance__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CurrentDecimalBalance => {
                            if current_decimal_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentDecimalBalance"));
                            }
                            current_decimal_balance__ = Some(map.next_value()?);
                        }
                        GeneratedField::NewBalance => {
                            if new_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newBalance"));
                            }
                            new_balance__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NewDecimalBalance => {
                            if new_decimal_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newDecimalBalance"));
                            }
                            new_decimal_balance__ = Some(map.next_value()?);
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
                        GeneratedField::RollupFeeTotalAmount => {
                            if rollup_fee_total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeTotalAmount"));
                            }
                            rollup_fee_total_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RollupFeeTotalDecimalAmount => {
                            if rollup_fee_total_decimal_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollupFeeTotalDecimalAmount"));
                            }
                            rollup_fee_total_decimal_amount__ = Some(map.next_value()?);
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
                        GeneratedField::GasRelayerFeeAssetSymbol => {
                            if gas_relayer_fee_asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerFeeAssetSymbol"));
                            }
                            gas_relayer_fee_asset_symbol__ = map.next_value()?;
                        }
                        GeneratedField::GasRelayerFeeAssetDecimals => {
                            if gas_relayer_fee_asset_decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerFeeAssetDecimals"));
                            }
                            gas_relayer_fee_asset_decimals__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::GasRelayerAddress => {
                            if gas_relayer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerAddress"));
                            }
                            gas_relayer_address__ = map.next_value()?;
                        }
                        GeneratedField::GasRelayerName => {
                            if gas_relayer_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerName"));
                            }
                            gas_relayer_name__ = map.next_value()?;
                        }
                        GeneratedField::GasRelayerUrl => {
                            if gas_relayer_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasRelayerUrl"));
                            }
                            gas_relayer_url__ = map.next_value()?;
                        }
                    }
                }
                Ok(SpendSummary {
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    asset_decimals: asset_decimals__.unwrap_or_default(),
                    current_balance: current_balance__.unwrap_or_default(),
                    current_decimal_balance: current_decimal_balance__.unwrap_or_default(),
                    new_balance: new_balance__.unwrap_or_default(),
                    new_decimal_balance: new_decimal_balance__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    decimal_amount: decimal_amount__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    rollup_fee_amount: rollup_fee_amount__.unwrap_or_default(),
                    rollup_fee_decimal_amount: rollup_fee_decimal_amount__.unwrap_or_default(),
                    rollup_fee_total_amount: rollup_fee_total_amount__.unwrap_or_default(),
                    rollup_fee_total_decimal_amount: rollup_fee_total_decimal_amount__.unwrap_or_default(),
                    rollup_fee_asset_symbol: rollup_fee_asset_symbol__.unwrap_or_default(),
                    rollup_fee_asset_decimals: rollup_fee_asset_decimals__.unwrap_or_default(),
                    gas_relayer_fee_amount: gas_relayer_fee_amount__,
                    gas_relayer_fee_decimal_amount: gas_relayer_fee_decimal_amount__,
                    gas_relayer_fee_asset_symbol: gas_relayer_fee_asset_symbol__,
                    gas_relayer_fee_asset_decimals: gas_relayer_fee_asset_decimals__,
                    gas_relayer_address: gas_relayer_address__,
                    gas_relayer_name: gas_relayer_name__,
                    gas_relayer_url: gas_relayer_url__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.SpendSummary", FIELDS, GeneratedVisitor)
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
