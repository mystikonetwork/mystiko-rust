// @generated
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
impl serde::Serialize for ColumnValuePair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.column.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.ColumnValuePair", len)?;
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnValuePair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "column",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Column,
            Value,
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
                            "column" => Ok(GeneratedField::Column),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnValuePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.ColumnValuePair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ColumnValuePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut column__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(ColumnValuePair {
                    column: column__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.ColumnValuePair", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for CountDepositRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CountDepositRequest", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CountDepositRequest {
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
            type Value = CountDepositRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CountDepositRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CountDepositRequest, V::Error>
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
                Ok(CountDepositRequest {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CountDepositRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CountDepositResponse {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CountDepositResponse", len)?;
        if self.count != 0 {
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CountDepositResponse {
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
            type Value = CountDepositResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CountDepositResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CountDepositResponse, V::Error>
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
                Ok(CountDepositResponse {
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CountDepositResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CountSpendsRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CountSpendsRequest", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CountSpendsRequest {
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
            type Value = CountSpendsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CountSpendsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CountSpendsRequest, V::Error>
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
                Ok(CountSpendsRequest {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CountSpendsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CountSpendsResponse {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CountSpendsResponse", len)?;
        if self.count != 0 {
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CountSpendsResponse {
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
            type Value = CountSpendsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CountSpendsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CountSpendsResponse, V::Error>
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
                Ok(CountSpendsResponse {
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CountSpendsResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for CreateSpendRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CreateSpendRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSpendRequest {
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
            type Value = CreateSpendRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CreateSpendRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateSpendRequest, V::Error>
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
                Ok(CreateSpendRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CreateSpendRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateSpendResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.spend.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.CreateSpendResponse", len)?;
        if let Some(v) = self.spend.as_ref() {
            struct_ser.serialize_field("spend", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSpendResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spend,
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
                            "spend" => Ok(GeneratedField::Spend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateSpendResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.CreateSpendResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateSpendResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spend__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Spend => {
                            if spend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spend"));
                            }
                            spend__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateSpendResponse {
                    spend: spend__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.CreateSpendResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for DeleteDepositBatchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.deposits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.DeleteDepositBatchRequest", len)?;
        if !self.deposits.is_empty() {
            struct_ser.serialize_field("deposits", &self.deposits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDepositBatchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deposits,
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
                            "deposits" => Ok(GeneratedField::Deposits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteDepositBatchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.DeleteDepositBatchRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteDepositBatchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deposits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Deposits => {
                            if deposits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposits"));
                            }
                            deposits__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeleteDepositBatchRequest {
                    deposits: deposits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.DeleteDepositBatchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteDepositByFilterRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.DeleteDepositByFilterRequest", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDepositByFilterRequest {
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
            type Value = DeleteDepositByFilterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.DeleteDepositByFilterRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteDepositByFilterRequest, V::Error>
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
                Ok(DeleteDepositByFilterRequest {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.DeleteDepositByFilterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteDepositRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.DeleteDepositRequest", len)?;
        if let Some(v) = self.deposit.as_ref() {
            struct_ser.serialize_field("deposit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDepositRequest {
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
            type Value = DeleteDepositRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.DeleteDepositRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteDepositRequest, V::Error>
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
                Ok(DeleteDepositRequest {
                    deposit: deposit__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.DeleteDepositRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSpendBatchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.spends.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.DeleteSpendBatchRequest", len)?;
        if !self.spends.is_empty() {
            struct_ser.serialize_field("spends", &self.spends)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSpendBatchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spends",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spends,
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
                            "spends" => Ok(GeneratedField::Spends),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteSpendBatchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.DeleteSpendBatchRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteSpendBatchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spends__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Spends => {
                            if spends__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spends"));
                            }
                            spends__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeleteSpendBatchRequest {
                    spends: spends__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.DeleteSpendBatchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSpendByFilterRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.DeleteSpendByFilterRequest", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSpendByFilterRequest {
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
            type Value = DeleteSpendByFilterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.DeleteSpendByFilterRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteSpendByFilterRequest, V::Error>
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
                Ok(DeleteSpendByFilterRequest {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.DeleteSpendByFilterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSpendRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.spend.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.DeleteSpendRequest", len)?;
        if let Some(v) = self.spend.as_ref() {
            struct_ser.serialize_field("spend", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSpendRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spend,
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
                            "spend" => Ok(GeneratedField::Spend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteSpendRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.DeleteSpendRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteSpendRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spend__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Spend => {
                            if spend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spend"));
                            }
                            spend__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteSpendRequest {
                    spend: spend__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.DeleteSpendRequest", FIELDS, GeneratedVisitor)
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
        if !self.identifier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.ExportSecretKeyRequest", len)?;
        if !self.wallet_password.is_empty() {
            struct_ser.serialize_field("walletPassword", &self.wallet_password)?;
        }
        if !self.identifier.is_empty() {
            struct_ser.serialize_field("identifier", &self.identifier)?;
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
            "identifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WalletPassword,
            Identifier,
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
                            "identifier" => Ok(GeneratedField::Identifier),
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
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExportSecretKeyRequest {
                    wallet_password: wallet_password__.unwrap_or_default(),
                    identifier: identifier__.unwrap_or_default(),
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
        if !self.identifier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindAccountByIdentifierRequest", len)?;
        if !self.identifier.is_empty() {
            struct_ser.serialize_field("identifier", &self.identifier)?;
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
            "identifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identifier,
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
                            "identifier" => Ok(GeneratedField::Identifier),
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
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FindAccountByIdentifierRequest {
                    identifier: identifier__.unwrap_or_default(),
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
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindAccountRequest", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
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
            type Value = FindAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindAccountRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindAccountRequest, V::Error>
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
                Ok(FindAccountRequest {
                    filter: filter__,
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
impl serde::Serialize for FindDepositByIdRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindDepositByIdRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindDepositByIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindDepositByIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindDepositByIdRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindDepositByIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FindDepositByIdRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindDepositByIdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindDepositRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindDepositRequest", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindDepositRequest {
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
            type Value = FindDepositRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindDepositRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindDepositRequest, V::Error>
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
                Ok(FindDepositRequest {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindDepositRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindDepositResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.deposits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindDepositResponse", len)?;
        if !self.deposits.is_empty() {
            struct_ser.serialize_field("deposits", &self.deposits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindDepositResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deposits,
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
                            "deposits" => Ok(GeneratedField::Deposits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindDepositResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindDepositResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindDepositResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deposits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Deposits => {
                            if deposits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposits"));
                            }
                            deposits__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FindDepositResponse {
                    deposits: deposits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindDepositResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindOneDepositResponse {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindOneDepositResponse", len)?;
        if let Some(v) = self.deposit.as_ref() {
            struct_ser.serialize_field("deposit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindOneDepositResponse {
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
            type Value = FindOneDepositResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindOneDepositResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindOneDepositResponse, V::Error>
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
                Ok(FindOneDepositResponse {
                    deposit: deposit__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindOneDepositResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindOneSpendResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.spend.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindOneSpendResponse", len)?;
        if let Some(v) = self.spend.as_ref() {
            struct_ser.serialize_field("spend", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindOneSpendResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spend,
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
                            "spend" => Ok(GeneratedField::Spend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindOneSpendResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindOneSpendResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindOneSpendResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spend__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Spend => {
                            if spend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spend"));
                            }
                            spend__ = map.next_value()?;
                        }
                    }
                }
                Ok(FindOneSpendResponse {
                    spend: spend__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindOneSpendResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindSpendByIdRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindSpendByIdRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindSpendByIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindSpendByIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindSpendByIdRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindSpendByIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FindSpendByIdRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindSpendByIdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindSpendRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindSpendRequest", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindSpendRequest {
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
            type Value = FindSpendRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindSpendRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindSpendRequest, V::Error>
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
                Ok(FindSpendRequest {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindSpendRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindSpendsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.spends.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.FindSpendsResponse", len)?;
        if !self.spends.is_empty() {
            struct_ser.serialize_field("spends", &self.spends)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindSpendsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spends",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spends,
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
                            "spends" => Ok(GeneratedField::Spends),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindSpendsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.FindSpendsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FindSpendsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spends__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Spends => {
                            if spends__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spends"));
                            }
                            spends__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FindSpendsResponse {
                    spends: spends__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.FindSpendsResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for SendSpendRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SendSpendRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendSpendRequest {
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
            type Value = SendSpendRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SendSpendRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SendSpendRequest, V::Error>
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
                Ok(SendSpendRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SendSpendRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendSpendResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.spend.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SendSpendResponse", len)?;
        if let Some(v) = self.spend.as_ref() {
            struct_ser.serialize_field("spend", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendSpendResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spend,
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
                            "spend" => Ok(GeneratedField::Spend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendSpendResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SendSpendResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SendSpendResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spend__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Spend => {
                            if spend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spend"));
                            }
                            spend__ = map.next_value()?;
                        }
                    }
                }
                Ok(SendSpendResponse {
                    spend: spend__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SendSpendResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendSpendWithGrpcRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.send_options.is_some() {
            len += 1;
        }
        if self.client_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SendSpendWithGrpcRequest", len)?;
        if let Some(v) = self.send_options.as_ref() {
            struct_ser.serialize_field("sendOptions", v)?;
        }
        if let Some(v) = self.client_options.as_ref() {
            struct_ser.serialize_field("clientOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendSpendWithGrpcRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "send_options",
            "sendOptions",
            "client_options",
            "clientOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SendOptions,
            ClientOptions,
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
                            "sendOptions" | "send_options" => Ok(GeneratedField::SendOptions),
                            "clientOptions" | "client_options" => Ok(GeneratedField::ClientOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendSpendWithGrpcRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SendSpendWithGrpcRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SendSpendWithGrpcRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut send_options__ = None;
                let mut client_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SendOptions => {
                            if send_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendOptions"));
                            }
                            send_options__ = map.next_value()?;
                        }
                        GeneratedField::ClientOptions => {
                            if client_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientOptions"));
                            }
                            client_options__ = map.next_value()?;
                        }
                    }
                }
                Ok(SendSpendWithGrpcRequest {
                    send_options: send_options__,
                    client_options: client_options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SendSpendWithGrpcRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendWithGrpcRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.send_options.is_some() {
            len += 1;
        }
        if self.client_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SendWithGrpcRequest", len)?;
        if let Some(v) = self.send_options.as_ref() {
            struct_ser.serialize_field("sendOptions", v)?;
        }
        if let Some(v) = self.client_options.as_ref() {
            struct_ser.serialize_field("clientOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendWithGrpcRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "send_options",
            "sendOptions",
            "client_options",
            "clientOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SendOptions,
            ClientOptions,
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
                            "sendOptions" | "send_options" => Ok(GeneratedField::SendOptions),
                            "clientOptions" | "client_options" => Ok(GeneratedField::ClientOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendWithGrpcRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SendWithGrpcRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SendWithGrpcRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut send_options__ = None;
                let mut client_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SendOptions => {
                            if send_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendOptions"));
                            }
                            send_options__ = map.next_value()?;
                        }
                        GeneratedField::ClientOptions => {
                            if client_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientOptions"));
                            }
                            client_options__ = map.next_value()?;
                        }
                    }
                }
                Ok(SendWithGrpcRequest {
                    send_options: send_options__,
                    client_options: client_options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SendWithGrpcRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpendQuoteRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SpendQuoteRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpendQuoteRequest {
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
            type Value = SpendQuoteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SpendQuoteRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SpendQuoteRequest, V::Error>
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
                Ok(SpendQuoteRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SpendQuoteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpendQuoteResponse {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SpendQuoteResponse", len)?;
        if let Some(v) = self.quote.as_ref() {
            struct_ser.serialize_field("quote", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpendQuoteResponse {
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
            type Value = SpendQuoteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SpendQuoteResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SpendQuoteResponse, V::Error>
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
                Ok(SpendQuoteResponse {
                    quote: quote__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SpendQuoteResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpendSummaryRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SpendSummaryRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpendSummaryRequest {
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
            type Value = SpendSummaryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SpendSummaryRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SpendSummaryRequest, V::Error>
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
                Ok(SpendSummaryRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SpendSummaryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpendSummaryResponse {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.SpendSummaryResponse", len)?;
        if let Some(v) = self.summary.as_ref() {
            struct_ser.serialize_field("summary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpendSummaryResponse {
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
            type Value = SpendSummaryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.SpendSummaryResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SpendSummaryResponse, V::Error>
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
                Ok(SpendSummaryResponse {
                    summary: summary__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.SpendSummaryResponse", FIELDS, GeneratedVisitor)
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
        if !self.identifier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateAccountRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if !self.identifier.is_empty() {
            struct_ser.serialize_field("identifier", &self.identifier)?;
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
            "identifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
            Identifier,
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
                            "identifier" => Ok(GeneratedField::Identifier),
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
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateAccountRequest {
                    options: options__,
                    identifier: identifier__.unwrap_or_default(),
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
impl serde::Serialize for UpdateAllDepositRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.column_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateAllDepositRequest", len)?;
        if !self.column_values.is_empty() {
            struct_ser.serialize_field("columnValues", &self.column_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAllDepositRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "column_values",
            "columnValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ColumnValues,
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
                            "columnValues" | "column_values" => Ok(GeneratedField::ColumnValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAllDepositRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateAllDepositRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateAllDepositRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut column_values__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ColumnValues => {
                            if column_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnValues"));
                            }
                            column_values__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateAllDepositRequest {
                    column_values: column_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateAllDepositRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAllSpendRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.column_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateAllSpendRequest", len)?;
        if !self.column_values.is_empty() {
            struct_ser.serialize_field("columnValues", &self.column_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAllSpendRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "column_values",
            "columnValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ColumnValues,
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
                            "columnValues" | "column_values" => Ok(GeneratedField::ColumnValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAllSpendRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateAllSpendRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateAllSpendRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut column_values__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ColumnValues => {
                            if column_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnValues"));
                            }
                            column_values__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateAllSpendRequest {
                    column_values: column_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateAllSpendRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDepositBatchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.deposits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateDepositBatchRequest", len)?;
        if !self.deposits.is_empty() {
            struct_ser.serialize_field("deposits", &self.deposits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDepositBatchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deposits,
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
                            "deposits" => Ok(GeneratedField::Deposits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateDepositBatchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateDepositBatchRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateDepositBatchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deposits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Deposits => {
                            if deposits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposits"));
                            }
                            deposits__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateDepositBatchRequest {
                    deposits: deposits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateDepositBatchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDepositBatchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.deposits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateDepositBatchResponse", len)?;
        if !self.deposits.is_empty() {
            struct_ser.serialize_field("deposits", &self.deposits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDepositBatchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deposits,
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
                            "deposits" => Ok(GeneratedField::Deposits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateDepositBatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateDepositBatchResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateDepositBatchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deposits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Deposits => {
                            if deposits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposits"));
                            }
                            deposits__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateDepositBatchResponse {
                    deposits: deposits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateDepositBatchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDepositByFilterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.column_values.is_empty() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateDepositByFilterRequest", len)?;
        if !self.column_values.is_empty() {
            struct_ser.serialize_field("columnValues", &self.column_values)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDepositByFilterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "column_values",
            "columnValues",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ColumnValues,
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
                            "columnValues" | "column_values" => Ok(GeneratedField::ColumnValues),
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
            type Value = UpdateDepositByFilterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateDepositByFilterRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateDepositByFilterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut column_values__ = None;
                let mut filter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ColumnValues => {
                            if column_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnValues"));
                            }
                            column_values__ = Some(map.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateDepositByFilterRequest {
                    column_values: column_values__.unwrap_or_default(),
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateDepositByFilterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDepositRequest {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateDepositRequest", len)?;
        if let Some(v) = self.deposit.as_ref() {
            struct_ser.serialize_field("deposit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDepositRequest {
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
            type Value = UpdateDepositRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateDepositRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateDepositRequest, V::Error>
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
                Ok(UpdateDepositRequest {
                    deposit: deposit__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateDepositRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDepositResponse {
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
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateDepositResponse", len)?;
        if let Some(v) = self.deposit.as_ref() {
            struct_ser.serialize_field("deposit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDepositResponse {
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
            type Value = UpdateDepositResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateDepositResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateDepositResponse, V::Error>
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
                Ok(UpdateDepositResponse {
                    deposit: deposit__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateDepositResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for UpdateSpendBatchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.spends.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateSpendBatchRequest", len)?;
        if !self.spends.is_empty() {
            struct_ser.serialize_field("spends", &self.spends)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSpendBatchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spends",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spends,
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
                            "spends" => Ok(GeneratedField::Spends),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateSpendBatchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateSpendBatchRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateSpendBatchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spends__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Spends => {
                            if spends__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spends"));
                            }
                            spends__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateSpendBatchRequest {
                    spends: spends__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateSpendBatchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSpendBatchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.spends.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateSpendBatchResponse", len)?;
        if !self.spends.is_empty() {
            struct_ser.serialize_field("spends", &self.spends)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSpendBatchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spends",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spends,
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
                            "spends" => Ok(GeneratedField::Spends),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateSpendBatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateSpendBatchResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateSpendBatchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spends__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Spends => {
                            if spends__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spends"));
                            }
                            spends__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateSpendBatchResponse {
                    spends: spends__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateSpendBatchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSpendByFilterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.column_values.is_empty() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateSpendByFilterRequest", len)?;
        if !self.column_values.is_empty() {
            struct_ser.serialize_field("columnValues", &self.column_values)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSpendByFilterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "column_values",
            "columnValues",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ColumnValues,
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
                            "columnValues" | "column_values" => Ok(GeneratedField::ColumnValues),
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
            type Value = UpdateSpendByFilterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateSpendByFilterRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateSpendByFilterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut column_values__ = None;
                let mut filter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ColumnValues => {
                            if column_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnValues"));
                            }
                            column_values__ = Some(map.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateSpendByFilterRequest {
                    column_values: column_values__.unwrap_or_default(),
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateSpendByFilterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSpendRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.spend.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateSpendRequest", len)?;
        if let Some(v) = self.spend.as_ref() {
            struct_ser.serialize_field("spend", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSpendRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spend,
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
                            "spend" => Ok(GeneratedField::Spend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateSpendRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateSpendRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateSpendRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spend__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Spend => {
                            if spend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spend"));
                            }
                            spend__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateSpendRequest {
                    spend: spend__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateSpendRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSpendResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.spend.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.api.handler.v1.UpdateSpendResponse", len)?;
        if let Some(v) = self.spend.as_ref() {
            struct_ser.serialize_field("spend", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSpendResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spend,
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
                            "spend" => Ok(GeneratedField::Spend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateSpendResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.api.handler.v1.UpdateSpendResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateSpendResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spend__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Spend => {
                            if spend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spend"));
                            }
                            spend__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateSpendResponse {
                    spend: spend__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.api.handler.v1.UpdateSpendResponse", FIELDS, GeneratedVisitor)
    }
}
