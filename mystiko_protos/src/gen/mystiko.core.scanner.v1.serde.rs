// @generated
impl serde::Serialize for Balance {
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
        if self.unspent != 0. {
            len += 1;
        }
        if self.pending != 0. {
            len += 1;
        }
        if self.spent.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.Balance", len)?;
        if !self.asset_symbol.is_empty() {
            struct_ser.serialize_field("assetSymbol", &self.asset_symbol)?;
        }
        if self.unspent != 0. {
            struct_ser.serialize_field("unspent", &self.unspent)?;
        }
        if self.pending != 0. {
            struct_ser.serialize_field("pending", &self.pending)?;
        }
        if let Some(v) = self.spent.as_ref() {
            struct_ser.serialize_field("spent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Balance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_symbol",
            "assetSymbol",
            "unspent",
            "pending",
            "spent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetSymbol,
            Unspent,
            Pending,
            Spent,
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
                            "unspent" => Ok(GeneratedField::Unspent),
                            "pending" => Ok(GeneratedField::Pending),
                            "spent" => Ok(GeneratedField::Spent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Balance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.Balance")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Balance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_symbol__ = None;
                let mut unspent__ = None;
                let mut pending__ = None;
                let mut spent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AssetSymbol => {
                            if asset_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetSymbol"));
                            }
                            asset_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::Unspent => {
                            if unspent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unspent"));
                            }
                            unspent__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pending => {
                            if pending__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pending"));
                            }
                            pending__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Spent => {
                            if spent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spent"));
                            }
                            spent__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Balance {
                    asset_symbol: asset_symbol__.unwrap_or_default(),
                    unspent: unspent__.unwrap_or_default(),
                    pending: pending__.unwrap_or_default(),
                    spent: spent__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.Balance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BalanceOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.with_spent.is_some() {
            len += 1;
        }
        if !self.shielded_addresses.is_empty() {
            len += 1;
        }
        if !self.chain_ids.is_empty() {
            len += 1;
        }
        if !self.asset_symbols.is_empty() {
            len += 1;
        }
        if !self.contract_addresses.is_empty() {
            len += 1;
        }
        if !self.bridge_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.BalanceOptions", len)?;
        if let Some(v) = self.with_spent.as_ref() {
            struct_ser.serialize_field("withSpent", v)?;
        }
        if !self.shielded_addresses.is_empty() {
            struct_ser.serialize_field("shieldedAddresses", &self.shielded_addresses)?;
        }
        if !self.chain_ids.is_empty() {
            struct_ser.serialize_field("chainIds", &self.chain_ids.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.asset_symbols.is_empty() {
            struct_ser.serialize_field("assetSymbols", &self.asset_symbols)?;
        }
        if !self.contract_addresses.is_empty() {
            struct_ser.serialize_field("contractAddresses", &self.contract_addresses)?;
        }
        if !self.bridge_types.is_empty() {
            let v = self.bridge_types.iter().cloned().map(|v| {
                super::super::super::common::v1::BridgeType::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("bridgeTypes", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BalanceOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "with_spent",
            "withSpent",
            "shielded_addresses",
            "shieldedAddresses",
            "chain_ids",
            "chainIds",
            "asset_symbols",
            "assetSymbols",
            "contract_addresses",
            "contractAddresses",
            "bridge_types",
            "bridgeTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithSpent,
            ShieldedAddresses,
            ChainIds,
            AssetSymbols,
            ContractAddresses,
            BridgeTypes,
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
                            "withSpent" | "with_spent" => Ok(GeneratedField::WithSpent),
                            "shieldedAddresses" | "shielded_addresses" => Ok(GeneratedField::ShieldedAddresses),
                            "chainIds" | "chain_ids" => Ok(GeneratedField::ChainIds),
                            "assetSymbols" | "asset_symbols" => Ok(GeneratedField::AssetSymbols),
                            "contractAddresses" | "contract_addresses" => Ok(GeneratedField::ContractAddresses),
                            "bridgeTypes" | "bridge_types" => Ok(GeneratedField::BridgeTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BalanceOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.BalanceOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BalanceOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut with_spent__ = None;
                let mut shielded_addresses__ = None;
                let mut chain_ids__ = None;
                let mut asset_symbols__ = None;
                let mut contract_addresses__ = None;
                let mut bridge_types__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WithSpent => {
                            if with_spent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withSpent"));
                            }
                            with_spent__ = map.next_value()?;
                        }
                        GeneratedField::ShieldedAddresses => {
                            if shielded_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddresses"));
                            }
                            shielded_addresses__ = Some(map.next_value()?);
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
                        GeneratedField::AssetSymbols => {
                            if asset_symbols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetSymbols"));
                            }
                            asset_symbols__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContractAddresses => {
                            if contract_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddresses"));
                            }
                            contract_addresses__ = Some(map.next_value()?);
                        }
                        GeneratedField::BridgeTypes => {
                            if bridge_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeTypes"));
                            }
                            bridge_types__ = Some(map.next_value::<Vec<super::super::super::common::v1::BridgeType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(BalanceOptions {
                    with_spent: with_spent__,
                    shielded_addresses: shielded_addresses__.unwrap_or_default(),
                    chain_ids: chain_ids__.unwrap_or_default(),
                    asset_symbols: asset_symbols__.unwrap_or_default(),
                    contract_addresses: contract_addresses__.unwrap_or_default(),
                    bridge_types: bridge_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.BalanceOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BalanceResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.balances.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.BalanceResult", len)?;
        if !self.balances.is_empty() {
            struct_ser.serialize_field("balances", &self.balances)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BalanceResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "balances",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balances,
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
                            "balances" => Ok(GeneratedField::Balances),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BalanceResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.BalanceResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BalanceResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balances__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Balances => {
                            if balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balances"));
                            }
                            balances__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BalanceResult {
                    balances: balances__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.BalanceResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reset_to_id.is_some() {
            len += 1;
        }
        if !self.shielded_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.ResetOptions", len)?;
        if let Some(v) = self.reset_to_id.as_ref() {
            struct_ser.serialize_field("resetToId", v)?;
        }
        if !self.shielded_addresses.is_empty() {
            struct_ser.serialize_field("shieldedAddresses", &self.shielded_addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reset_to_id",
            "resetToId",
            "shielded_addresses",
            "shieldedAddresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResetToId,
            ShieldedAddresses,
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
                            "resetToId" | "reset_to_id" => Ok(GeneratedField::ResetToId),
                            "shieldedAddresses" | "shielded_addresses" => Ok(GeneratedField::ShieldedAddresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResetOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.ResetOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResetOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reset_to_id__ = None;
                let mut shielded_addresses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResetToId => {
                            if reset_to_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resetToId"));
                            }
                            reset_to_id__ = map.next_value()?;
                        }
                        GeneratedField::ShieldedAddresses => {
                            if shielded_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddresses"));
                            }
                            shielded_addresses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ResetOptions {
                    reset_to_id: reset_to_id__,
                    shielded_addresses: shielded_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.ResetOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.ResetResult", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetResult {
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
            type Value = ResetResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.ResetResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResetResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ResetResult {
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.ResetResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScanOptions {
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
        if self.batch_size.is_some() {
            len += 1;
        }
        if self.concurrency.is_some() {
            len += 1;
        }
        if !self.shielded_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.ScanOptions", len)?;
        if !self.wallet_password.is_empty() {
            struct_ser.serialize_field("walletPassword", &self.wallet_password)?;
        }
        if let Some(v) = self.batch_size.as_ref() {
            struct_ser.serialize_field("batchSize", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.concurrency.as_ref() {
            struct_ser.serialize_field("concurrency", v)?;
        }
        if !self.shielded_addresses.is_empty() {
            struct_ser.serialize_field("shieldedAddresses", &self.shielded_addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScanOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wallet_password",
            "walletPassword",
            "batch_size",
            "batchSize",
            "concurrency",
            "shielded_addresses",
            "shieldedAddresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WalletPassword,
            BatchSize,
            Concurrency,
            ShieldedAddresses,
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
                            "batchSize" | "batch_size" => Ok(GeneratedField::BatchSize),
                            "concurrency" => Ok(GeneratedField::Concurrency),
                            "shieldedAddresses" | "shielded_addresses" => Ok(GeneratedField::ShieldedAddresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScanOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.ScanOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScanOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet_password__ = None;
                let mut batch_size__ = None;
                let mut concurrency__ = None;
                let mut shielded_addresses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WalletPassword => {
                            if wallet_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletPassword"));
                            }
                            wallet_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchSize => {
                            if batch_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchSize"));
                            }
                            batch_size__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Concurrency => {
                            if concurrency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("concurrency"));
                            }
                            concurrency__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ShieldedAddresses => {
                            if shielded_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddresses"));
                            }
                            shielded_addresses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ScanOptions {
                    wallet_password: wallet_password__.unwrap_or_default(),
                    batch_size: batch_size__,
                    concurrency: concurrency__,
                    shielded_addresses: shielded_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.ScanOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScanResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from_id.is_empty() {
            len += 1;
        }
        if !self.to_id.is_empty() {
            len += 1;
        }
        if self.total_count != 0 {
            len += 1;
        }
        if self.owned_count != 0 {
            len += 1;
        }
        if !self.scanned_shielded_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.scanner.v1.ScanResult", len)?;
        if !self.from_id.is_empty() {
            struct_ser.serialize_field("fromId", &self.from_id)?;
        }
        if !self.to_id.is_empty() {
            struct_ser.serialize_field("toId", &self.to_id)?;
        }
        if self.total_count != 0 {
            struct_ser.serialize_field("totalCount", ToString::to_string(&self.total_count).as_str())?;
        }
        if self.owned_count != 0 {
            struct_ser.serialize_field("ownedCount", ToString::to_string(&self.owned_count).as_str())?;
        }
        if !self.scanned_shielded_addresses.is_empty() {
            struct_ser.serialize_field("scannedShieldedAddresses", &self.scanned_shielded_addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScanResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from_id",
            "fromId",
            "to_id",
            "toId",
            "total_count",
            "totalCount",
            "owned_count",
            "ownedCount",
            "scanned_shielded_addresses",
            "scannedShieldedAddresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FromId,
            ToId,
            TotalCount,
            OwnedCount,
            ScannedShieldedAddresses,
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
                            "fromId" | "from_id" => Ok(GeneratedField::FromId),
                            "toId" | "to_id" => Ok(GeneratedField::ToId),
                            "totalCount" | "total_count" => Ok(GeneratedField::TotalCount),
                            "ownedCount" | "owned_count" => Ok(GeneratedField::OwnedCount),
                            "scannedShieldedAddresses" | "scanned_shielded_addresses" => Ok(GeneratedField::ScannedShieldedAddresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScanResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.scanner.v1.ScanResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScanResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from_id__ = None;
                let mut to_id__ = None;
                let mut total_count__ = None;
                let mut owned_count__ = None;
                let mut scanned_shielded_addresses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FromId => {
                            if from_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromId"));
                            }
                            from_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ToId => {
                            if to_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toId"));
                            }
                            to_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::TotalCount => {
                            if total_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalCount"));
                            }
                            total_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OwnedCount => {
                            if owned_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownedCount"));
                            }
                            owned_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ScannedShieldedAddresses => {
                            if scanned_shielded_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scannedShieldedAddresses"));
                            }
                            scanned_shielded_addresses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ScanResult {
                    from_id: from_id__.unwrap_or_default(),
                    to_id: to_id__.unwrap_or_default(),
                    total_count: total_count__.unwrap_or_default(),
                    owned_count: owned_count__.unwrap_or_default(),
                    scanned_shielded_addresses: scanned_shielded_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.scanner.v1.ScanResult", FIELDS, GeneratedVisitor)
    }
}
