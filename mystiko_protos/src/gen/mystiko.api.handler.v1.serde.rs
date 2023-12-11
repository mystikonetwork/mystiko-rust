// @generated
impl serde::Serialize for CheckCurrentRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CheckCurrentRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckCurrentRequest {
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
            type Value = CheckCurrentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CheckCurrentRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckCurrentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CheckCurrentRequest {
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CheckCurrentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckCurrentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.wallet.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CheckCurrentResponse", len)?;
        if let Some(v) = self.wallet.as_ref() {
            struct_ser.serialize_field("wallet", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckCurrentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wallet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Wallet,
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
                            "wallet" => Ok(GeneratedField::Wallet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckCurrentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CheckCurrentResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckCurrentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Wallet => {
                            if wallet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wallet"));
                            }
                            wallet__ = map.next_value()?;
                        }
                    }
                }
                Ok(CheckCurrentResponse {
                    wallet: wallet__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CheckCurrentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckPasswordRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CheckPasswordRequest", len)?;
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckPasswordRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Password,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckPasswordRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CheckPasswordRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckPasswordRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut password__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CheckPasswordRequest {
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CheckPasswordRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckPasswordResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.wallet.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CheckPasswordResponse", len)?;
        if let Some(v) = self.wallet.as_ref() {
            struct_ser.serialize_field("wallet", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckPasswordResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wallet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Wallet,
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
                            "wallet" => Ok(GeneratedField::Wallet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckPasswordResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CheckPasswordResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckPasswordResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Wallet => {
                            if wallet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wallet"));
                            }
                            wallet__ = map.next_value()?;
                        }
                    }
                }
                Ok(CheckPasswordResponse {
                    wallet: wallet__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CheckPasswordResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CountAccountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CountAccountRequest", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CountAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filter,
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
                            "filter" => Ok(GeneratedField::Filter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CountAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CountAccountRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CountAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map.next_value()?;
                        }
                    }
                }
                Ok(CountAccountRequest {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CountAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CountAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CountAccountResponse", len)?;
        if self.count != 0 {
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CountAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "count",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Count,
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
                            "count" => Ok(GeneratedField::Count),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CountAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CountAccountResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CountAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CountAccountResponse {
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CountAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAccountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CreateAccountRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
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
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CreateAccountRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateAccountRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CreateAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.account.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CreateAccountResponse", len)?;
        if let Some(v) = self.account.as_ref() {
            struct_ser.serialize_field("account", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
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
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CreateAccountResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateAccountResponse {
                    account: account__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CreateAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDepositRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CreateDepositRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDepositRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
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
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDepositRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CreateDepositRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateDepositRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateDepositRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CreateDepositRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDepositResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deposit.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CreateDepositResponse", len)?;
        if let Some(v) = self.deposit.as_ref() {
            struct_ser.serialize_field("deposit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDepositResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deposit,
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
                            "deposit" => Ok(GeneratedField::Deposit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDepositResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CreateDepositResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateDepositResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deposit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Deposit => {
                            if deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposit"));
                            }
                            deposit__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateDepositResponse {
                    deposit: deposit__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CreateDepositResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWalletRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CreateWalletRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWalletRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
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
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWalletRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CreateWalletRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateWalletRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateWalletRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CreateWalletRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWalletResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.wallet.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CreateWalletResponse", len)?;
        if let Some(v) = self.wallet.as_ref() {
            struct_ser.serialize_field("wallet", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWalletResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wallet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Wallet,
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
                            "wallet" => Ok(GeneratedField::Wallet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWalletResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CreateWalletResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateWalletResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Wallet => {
                            if wallet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wallet"));
                            }
                            wallet__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateWalletResponse {
                    wallet: wallet__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CreateWalletResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportMnemonicPhraseRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.ExportMnemonicPhraseRequest", len)?;
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportMnemonicPhraseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Password,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportMnemonicPhraseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.ExportMnemonicPhraseRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExportMnemonicPhraseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut password__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExportMnemonicPhraseRequest {
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.ExportMnemonicPhraseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportMnemonicPhraseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.mnemonic_phrase.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.ExportMnemonicPhraseResponse", len)?;
        if !self.mnemonic_phrase.is_empty() {
            struct_ser.serialize_field("mnemonicPhrase", &self.mnemonic_phrase)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportMnemonicPhraseResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mnemonic_phrase",
            "mnemonicPhrase",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ExportMnemonicPhraseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.ExportMnemonicPhraseResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExportMnemonicPhraseResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mnemonic_phrase__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MnemonicPhrase => {
                            if mnemonic_phrase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mnemonicPhrase"));
                            }
                            mnemonic_phrase__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExportMnemonicPhraseResponse {
                    mnemonic_phrase: mnemonic_phrase__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.ExportMnemonicPhraseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportSecretKeyRequest {
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
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.ExportSecretKeyRequest", len)?;
        if !self.wallet_password.is_empty() {
            struct_ser.serialize_field("walletPassword", &self.wallet_password)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                export_secret_key_request::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                export_secret_key_request::Identifier::PublicKey(v) => {
                    struct_ser.serialize_field("publicKey", v)?;
                }
                export_secret_key_request::Identifier::ShieldedAddress(v) => {
                    struct_ser.serialize_field("shieldedAddress", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportSecretKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wallet_password",
            "walletPassword",
            "id",
            "public_key",
            "publicKey",
            "shielded_address",
            "shieldedAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WalletPassword,
            Id,
            PublicKey,
            ShieldedAddress,
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
                            "id" => Ok(GeneratedField::Id),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "shieldedAddress" | "shielded_address" => Ok(GeneratedField::ShieldedAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportSecretKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.ExportSecretKeyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExportSecretKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet_password__ = None;
                let mut identifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WalletPassword => {
                            if wallet_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletPassword"));
                            }
                            wallet_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(export_secret_key_request::Identifier::Id);
                        }
                        GeneratedField::PublicKey => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(export_secret_key_request::Identifier::PublicKey);
                        }
                        GeneratedField::ShieldedAddress => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddress"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(export_secret_key_request::Identifier::ShieldedAddress);
                        }
                    }
                }
                Ok(ExportSecretKeyRequest {
                    wallet_password: wallet_password__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.ExportSecretKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportSecretKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.secret_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.ExportSecretKeyResponse", len)?;
        if !self.secret_key.is_empty() {
            struct_ser.serialize_field("secretKey", &self.secret_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportSecretKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "secret_key",
            "secretKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ExportSecretKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.ExportSecretKeyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExportSecretKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut secret_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SecretKey => {
                            if secret_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secretKey"));
                            }
                            secret_key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExportSecretKeyResponse {
                    secret_key: secret_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.ExportSecretKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindAccountByIdentifierRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindAccountByIdentifierRequest", len)?;
        if let Some(v) = self.identifier.as_ref() {
            match v {
                find_account_by_identifier_request::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                find_account_by_identifier_request::Identifier::ShieldedAddress(v) => {
                    struct_ser.serialize_field("shieldedAddress", v)?;
                }
                find_account_by_identifier_request::Identifier::PublicKey(v) => {
                    struct_ser.serialize_field("publicKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindAccountByIdentifierRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "shielded_address",
            "shieldedAddress",
            "public_key",
            "publicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ShieldedAddress,
            PublicKey,
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
                            "shieldedAddress" | "shielded_address" => Ok(GeneratedField::ShieldedAddress),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindAccountByIdentifierRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindAccountByIdentifierRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindAccountByIdentifierRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(find_account_by_identifier_request::Identifier::Id);
                        }
                        GeneratedField::ShieldedAddress => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddress"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(find_account_by_identifier_request::Identifier::ShieldedAddress);
                        }
                        GeneratedField::PublicKey => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(find_account_by_identifier_request::Identifier::PublicKey);
                        }
                    }
                }
                Ok(FindAccountByIdentifierRequest {
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindAccountByIdentifierRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindAccountByIdentifierResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.account.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindAccountByIdentifierResponse", len)?;
        if let Some(v) = self.account.as_ref() {
            struct_ser.serialize_field("account", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindAccountByIdentifierResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
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
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindAccountByIdentifierResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindAccountByIdentifierResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindAccountByIdentifierResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = map.next_value()?;
                        }
                    }
                }
                Ok(FindAccountByIdentifierResponse {
                    account: account__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindAccountByIdentifierResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindAccountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindAccountRequest", len)?;
        if let Some(v) = self.condition.as_ref() {
            match v {
                find_account_request::Condition::Filter(v) => {
                    struct_ser.serialize_field("filter", v)?;
                }
                find_account_request::Condition::FindAll(v) => {
                    struct_ser.serialize_field("findAll", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter",
            "find_all",
            "findAll",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filter,
            FindAll,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "findAll" | "find_all" => Ok(GeneratedField::FindAll),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindAccountRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Filter => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            condition__ = map.next_value::<::std::option::Option<_>>()?.map(find_account_request::Condition::Filter)
;
                        }
                        GeneratedField::FindAll => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("findAll"));
                            }
                            condition__ = map.next_value::<::std::option::Option<_>>()?.map(find_account_request::Condition::FindAll);
                        }
                    }
                }
                Ok(FindAccountRequest {
                    condition: condition__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindAccountResponse", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
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
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindAccountResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FindAccountResponse {
                    account: account__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuoteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.QuoteRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuoteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
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
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuoteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.QuoteRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuoteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                    }
                }
                Ok(QuoteRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.QuoteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuoteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.quote.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.QuoteResponse", len)?;
        if let Some(v) = self.quote.as_ref() {
            struct_ser.serialize_field("quote", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuoteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "quote",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Quote,
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
                            "quote" => Ok(GeneratedField::Quote),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuoteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.QuoteResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuoteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut quote__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = map.next_value()?;
                        }
                    }
                }
                Ok(QuoteResponse {
                    quote: quote__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.QuoteResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SendRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
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
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SendRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SendRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                    }
                }
                Ok(SendRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SendRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deposit.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SendResponse", len)?;
        if let Some(v) = self.deposit.as_ref() {
            struct_ser.serialize_field("deposit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deposit,
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
                            "deposit" => Ok(GeneratedField::Deposit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SendResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SendResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deposit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Deposit => {
                            if deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposit"));
                            }
                            deposit__ = map.next_value()?;
                        }
                    }
                }
                Ok(SendResponse {
                    deposit: deposit__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SendResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SummaryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SummaryRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SummaryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
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
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SummaryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SummaryRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SummaryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                    }
                }
                Ok(SummaryRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SummaryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SummaryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SummaryResponse", len)?;
        if let Some(v) = self.summary.as_ref() {
            struct_ser.serialize_field("summary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SummaryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "summary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Summary,
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
                            "summary" => Ok(GeneratedField::Summary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SummaryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SummaryResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SummaryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut summary__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Summary => {
                            if summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summary"));
                            }
                            summary__ = map.next_value()?;
                        }
                    }
                }
                Ok(SummaryResponse {
                    summary: summary__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SummaryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAccountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options.is_some() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateAccountRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                update_account_request::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                update_account_request::Identifier::ShieldedAddress(v) => {
                    struct_ser.serialize_field("shieldedAddress", v)?;
                }
                update_account_request::Identifier::PublicKey(v) => {
                    struct_ser.serialize_field("publicKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "options",
            "id",
            "shielded_address",
            "shieldedAddress",
            "public_key",
            "publicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
            Id,
            ShieldedAddress,
            PublicKey,
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
                            "options" => Ok(GeneratedField::Options),
                            "id" => Ok(GeneratedField::Id),
                            "shieldedAddress" | "shielded_address" => Ok(GeneratedField::ShieldedAddress),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateAccountRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                let mut identifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(update_account_request::Identifier::Id);
                        }
                        GeneratedField::ShieldedAddress => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shieldedAddress"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(update_account_request::Identifier::ShieldedAddress);
                        }
                        GeneratedField::PublicKey => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(update_account_request::Identifier::PublicKey);
                        }
                    }
                }
                Ok(UpdateAccountRequest {
                    options: options__,
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.account.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateAccountResponse", len)?;
        if let Some(v) = self.account.as_ref() {
            struct_ser.serialize_field("account", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
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
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateAccountResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateAccountResponse {
                    account: account__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateEncryptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.old_wallet_password.is_empty() {
            len += 1;
        }
        if !self.new_wallet_password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateEncryptionRequest", len)?;
        if !self.old_wallet_password.is_empty() {
            struct_ser.serialize_field("oldWalletPassword", &self.old_wallet_password)?;
        }
        if !self.new_wallet_password.is_empty() {
            struct_ser.serialize_field("newWalletPassword", &self.new_wallet_password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateEncryptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "old_wallet_password",
            "oldWalletPassword",
            "new_wallet_password",
            "newWalletPassword",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OldWalletPassword,
            NewWalletPassword,
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
                            "oldWalletPassword" | "old_wallet_password" => Ok(GeneratedField::OldWalletPassword),
                            "newWalletPassword" | "new_wallet_password" => Ok(GeneratedField::NewWalletPassword),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateEncryptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateEncryptionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateEncryptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut old_wallet_password__ = None;
                let mut new_wallet_password__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OldWalletPassword => {
                            if old_wallet_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oldWalletPassword"));
                            }
                            old_wallet_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::NewWalletPassword => {
                            if new_wallet_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newWalletPassword"));
                            }
                            new_wallet_password__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateEncryptionRequest {
                    old_wallet_password: old_wallet_password__.unwrap_or_default(),
                    new_wallet_password: new_wallet_password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateEncryptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateEncryptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateEncryptionResponse", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateEncryptionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
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
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateEncryptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateEncryptionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateEncryptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateEncryptionResponse {
                    account: account__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateEncryptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePasswordRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.old_password.is_empty() {
            len += 1;
        }
        if !self.new_password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdatePasswordRequest", len)?;
        if !self.old_password.is_empty() {
            struct_ser.serialize_field("oldPassword", &self.old_password)?;
        }
        if !self.new_password.is_empty() {
            struct_ser.serialize_field("newPassword", &self.new_password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePasswordRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "old_password",
            "oldPassword",
            "new_password",
            "newPassword",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OldPassword,
            NewPassword,
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
                            "oldPassword" | "old_password" => Ok(GeneratedField::OldPassword),
                            "newPassword" | "new_password" => Ok(GeneratedField::NewPassword),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePasswordRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdatePasswordRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdatePasswordRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut old_password__ = None;
                let mut new_password__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OldPassword => {
                            if old_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oldPassword"));
                            }
                            old_password__ = Some(map.next_value()?);
                        }
                        GeneratedField::NewPassword => {
                            if new_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPassword"));
                            }
                            new_password__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdatePasswordRequest {
                    old_password: old_password__.unwrap_or_default(),
                    new_password: new_password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdatePasswordRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePasswordResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.wallet.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdatePasswordResponse", len)?;
        if let Some(v) = self.wallet.as_ref() {
            struct_ser.serialize_field("wallet", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePasswordResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wallet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Wallet,
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
                            "wallet" => Ok(GeneratedField::Wallet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePasswordResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdatePasswordResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdatePasswordResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Wallet => {
                            if wallet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wallet"));
                            }
                            wallet__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdatePasswordResponse {
                    wallet: wallet__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdatePasswordResponse", FIELDS, GeneratedVisitor)
    }
}
