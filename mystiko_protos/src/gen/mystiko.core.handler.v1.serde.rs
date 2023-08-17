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
        if self.scan_size.is_some() {
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
        if let Some(v) = self.scan_size.as_ref() {
            struct_ser.serialize_field("scanSize", v)?;
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
            "scan_size",
            "scanSize",
            "secret_key",
            "secretKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WalletPassword,
            Name,
            ScanSize,
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
                            "scanSize" | "scan_size" => Ok(GeneratedField::ScanSize),
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
                let mut scan_size__ = None;
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
                        GeneratedField::ScanSize => {
                            if scan_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scanSize"));
                            }
                            scan_size__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
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
                    scan_size: scan_size__,
                    secret_key: secret_key__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.CreateAccountOptions", FIELDS, GeneratedVisitor)
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
        if self.scan_size.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.UpdateAccountOptions", len)?;
        if !self.wallet_password.is_empty() {
            struct_ser.serialize_field("walletPassword", &self.wallet_password)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.scan_size.as_ref() {
            struct_ser.serialize_field("scanSize", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            let v = super::super::v1::AccountStatus::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("status", &v)?;
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
            "scan_size",
            "scanSize",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WalletPassword,
            Name,
            ScanSize,
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
                            "walletPassword" | "wallet_password" => Ok(GeneratedField::WalletPassword),
                            "name" => Ok(GeneratedField::Name),
                            "scanSize" | "scan_size" => Ok(GeneratedField::ScanSize),
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
                let mut scan_size__ = None;
                let mut status__ = None;
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
                        GeneratedField::ScanSize => {
                            if scan_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scanSize"));
                            }
                            scan_size__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value::<::std::option::Option<super::super::v1::AccountStatus>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(UpdateAccountOptions {
                    wallet_password: wallet_password__.unwrap_or_default(),
                    name: name__,
                    scan_size: scan_size__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.UpdateAccountOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateChainOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.providers.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.UpdateChainOptions", len)?;
        if !self.providers.is_empty() {
            struct_ser.serialize_field("providers", &self.providers)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateChainOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "providers",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Providers,
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
                            "providers" => Ok(GeneratedField::Providers),
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
            type Value = UpdateChainOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.UpdateChainOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateChainOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut providers__ = None;
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Providers => {
                            if providers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providers"));
                            }
                            providers__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateChainOptions {
                    providers: providers__.unwrap_or_default(),
                    name: name__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.UpdateChainOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProviderOptions {
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
        if self.timeout_ms.is_some() {
            len += 1;
        }
        if self.max_try_count.is_some() {
            len += 1;
        }
        if self.quorum_weight.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.core.handler.v1.UpdateProviderOptions", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if let Some(v) = self.timeout_ms.as_ref() {
            struct_ser.serialize_field("timeoutMs", v)?;
        }
        if let Some(v) = self.max_try_count.as_ref() {
            struct_ser.serialize_field("maxTryCount", v)?;
        }
        if let Some(v) = self.quorum_weight.as_ref() {
            struct_ser.serialize_field("quorumWeight", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProviderOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "timeout_ms",
            "timeoutMs",
            "max_try_count",
            "maxTryCount",
            "quorum_weight",
            "quorumWeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            TimeoutMs,
            MaxTryCount,
            QuorumWeight,
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
                            "timeoutMs" | "timeout_ms" => Ok(GeneratedField::TimeoutMs),
                            "maxTryCount" | "max_try_count" => Ok(GeneratedField::MaxTryCount),
                            "quorumWeight" | "quorum_weight" => Ok(GeneratedField::QuorumWeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProviderOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.core.handler.v1.UpdateProviderOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateProviderOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut timeout_ms__ = None;
                let mut max_try_count__ = None;
                let mut quorum_weight__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimeoutMs => {
                            if timeout_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutMs"));
                            }
                            timeout_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxTryCount => {
                            if max_try_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTryCount"));
                            }
                            max_try_count__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::QuorumWeight => {
                            if quorum_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quorumWeight"));
                            }
                            quorum_weight__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(UpdateProviderOptions {
                    url: url__.unwrap_or_default(),
                    timeout_ms: timeout_ms__,
                    max_try_count: max_try_count__,
                    quorum_weight: quorum_weight__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.core.handler.v1.UpdateProviderOptions", FIELDS, GeneratedVisitor)
    }
}
