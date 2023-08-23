// @generated
impl serde::Serialize for BigInt {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_positive {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.storage.v1.BigInt", len)?;
        if self.is_positive {
            struct_ser.serialize_field("isPositive", &self.is_positive)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigInt {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_positive",
            "isPositive",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsPositive,
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
                            "isPositive" | "is_positive" => Ok(GeneratedField::IsPositive),
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
            type Value = BigInt;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.storage.v1.BigInt")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BigInt, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_positive__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IsPositive => {
                            if is_positive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPositive"));
                            }
                            is_positive__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BigInt {
                    is_positive: is_positive__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.storage.v1.BigInt", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "COLUMN_TYPE_UNSPECIFIED",
            Self::Bool => "COLUMN_TYPE_BOOL",
            Self::Char => "COLUMN_TYPE_CHAR",
            Self::I8 => "COLUMN_TYPE_I8",
            Self::I16 => "COLUMN_TYPE_I16",
            Self::I32 => "COLUMN_TYPE_I32",
            Self::I64 => "COLUMN_TYPE_I64",
            Self::I128 => "COLUMN_TYPE_I128",
            Self::ISize => "COLUMN_TYPE_I_SIZE",
            Self::U8 => "COLUMN_TYPE_U8",
            Self::U16 => "COLUMN_TYPE_U16",
            Self::U32 => "COLUMN_TYPE_U32",
            Self::U64 => "COLUMN_TYPE_U64",
            Self::U128 => "COLUMN_TYPE_U128",
            Self::USize => "COLUMN_TYPE_U_SIZE",
            Self::F32 => "COLUMN_TYPE_F32",
            Self::F64 => "COLUMN_TYPE_F64",
            Self::String => "COLUMN_TYPE_STRING",
            Self::BigInt => "COLUMN_TYPE_BIG_INT",
            Self::BigUint => "COLUMN_TYPE_BIG_UINT",
            Self::Json => "COLUMN_TYPE_JSON",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ColumnType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COLUMN_TYPE_UNSPECIFIED",
            "COLUMN_TYPE_BOOL",
            "COLUMN_TYPE_CHAR",
            "COLUMN_TYPE_I8",
            "COLUMN_TYPE_I16",
            "COLUMN_TYPE_I32",
            "COLUMN_TYPE_I64",
            "COLUMN_TYPE_I128",
            "COLUMN_TYPE_I_SIZE",
            "COLUMN_TYPE_U8",
            "COLUMN_TYPE_U16",
            "COLUMN_TYPE_U32",
            "COLUMN_TYPE_U64",
            "COLUMN_TYPE_U128",
            "COLUMN_TYPE_U_SIZE",
            "COLUMN_TYPE_F32",
            "COLUMN_TYPE_F64",
            "COLUMN_TYPE_STRING",
            "COLUMN_TYPE_BIG_INT",
            "COLUMN_TYPE_BIG_UINT",
            "COLUMN_TYPE_JSON",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnType;

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
                    .and_then(ColumnType::from_i32)
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
                    .and_then(ColumnType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "COLUMN_TYPE_UNSPECIFIED" => Ok(ColumnType::Unspecified),
                    "COLUMN_TYPE_BOOL" => Ok(ColumnType::Bool),
                    "COLUMN_TYPE_CHAR" => Ok(ColumnType::Char),
                    "COLUMN_TYPE_I8" => Ok(ColumnType::I8),
                    "COLUMN_TYPE_I16" => Ok(ColumnType::I16),
                    "COLUMN_TYPE_I32" => Ok(ColumnType::I32),
                    "COLUMN_TYPE_I64" => Ok(ColumnType::I64),
                    "COLUMN_TYPE_I128" => Ok(ColumnType::I128),
                    "COLUMN_TYPE_I_SIZE" => Ok(ColumnType::ISize),
                    "COLUMN_TYPE_U8" => Ok(ColumnType::U8),
                    "COLUMN_TYPE_U16" => Ok(ColumnType::U16),
                    "COLUMN_TYPE_U32" => Ok(ColumnType::U32),
                    "COLUMN_TYPE_U64" => Ok(ColumnType::U64),
                    "COLUMN_TYPE_U128" => Ok(ColumnType::U128),
                    "COLUMN_TYPE_U_SIZE" => Ok(ColumnType::USize),
                    "COLUMN_TYPE_F32" => Ok(ColumnType::F32),
                    "COLUMN_TYPE_F64" => Ok(ColumnType::F64),
                    "COLUMN_TYPE_STRING" => Ok(ColumnType::String),
                    "COLUMN_TYPE_BIG_INT" => Ok(ColumnType::BigInt),
                    "COLUMN_TYPE_BIG_UINT" => Ok(ColumnType::BigUint),
                    "COLUMN_TYPE_JSON" => Ok(ColumnType::Json),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.storage.v1.ColumnValue", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                column_value::Value::BoolValue(v) => {
                    struct_ser.serialize_field("boolValue", v)?;
                }
                column_value::Value::CharValue(v) => {
                    struct_ser.serialize_field("charValue", v)?;
                }
                column_value::Value::I8Value(v) => {
                    struct_ser.serialize_field("i8Value", v)?;
                }
                column_value::Value::I16Value(v) => {
                    struct_ser.serialize_field("i16Value", v)?;
                }
                column_value::Value::I32Value(v) => {
                    struct_ser.serialize_field("i32Value", v)?;
                }
                column_value::Value::I64Value(v) => {
                    struct_ser.serialize_field("i64Value", ToString::to_string(&v).as_str())?;
                }
                column_value::Value::I128Value(v) => {
                    struct_ser.serialize_field("i128Value", pbjson::private::base64::encode(&v).as_str())?;
                }
                column_value::Value::IsizeValue(v) => {
                    struct_ser.serialize_field("isizeValue", ToString::to_string(&v).as_str())?;
                }
                column_value::Value::U8Value(v) => {
                    struct_ser.serialize_field("u8Value", v)?;
                }
                column_value::Value::U16Value(v) => {
                    struct_ser.serialize_field("u16Value", v)?;
                }
                column_value::Value::U32Value(v) => {
                    struct_ser.serialize_field("u32Value", v)?;
                }
                column_value::Value::U64Value(v) => {
                    struct_ser.serialize_field("u64Value", ToString::to_string(&v).as_str())?;
                }
                column_value::Value::U128Value(v) => {
                    struct_ser.serialize_field("u128Value", pbjson::private::base64::encode(&v).as_str())?;
                }
                column_value::Value::UsizeValue(v) => {
                    struct_ser.serialize_field("usizeValue", ToString::to_string(&v).as_str())?;
                }
                column_value::Value::F32Value(v) => {
                    struct_ser.serialize_field("f32Value", v)?;
                }
                column_value::Value::F64Value(v) => {
                    struct_ser.serialize_field("f64Value", v)?;
                }
                column_value::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                column_value::Value::BigIntValue(v) => {
                    struct_ser.serialize_field("bigIntValue", v)?;
                }
                column_value::Value::BigUintValue(v) => {
                    struct_ser.serialize_field("bigUintValue", pbjson::private::base64::encode(&v).as_str())?;
                }
                column_value::Value::JsonValue(v) => {
                    struct_ser.serialize_field("jsonValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bool_value",
            "boolValue",
            "char_value",
            "charValue",
            "i8_value",
            "i8Value",
            "i16_value",
            "i16Value",
            "i32_value",
            "i32Value",
            "i64_value",
            "i64Value",
            "i128_value",
            "i128Value",
            "isize_value",
            "isizeValue",
            "u8_value",
            "u8Value",
            "u16_value",
            "u16Value",
            "u32_value",
            "u32Value",
            "u64_value",
            "u64Value",
            "u128_value",
            "u128Value",
            "usize_value",
            "usizeValue",
            "f32_value",
            "f32Value",
            "f64_value",
            "f64Value",
            "string_value",
            "stringValue",
            "big_int_value",
            "bigIntValue",
            "big_uint_value",
            "bigUintValue",
            "json_value",
            "jsonValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BoolValue,
            CharValue,
            I8Value,
            I16Value,
            I32Value,
            I64Value,
            I128Value,
            IsizeValue,
            U8Value,
            U16Value,
            U32Value,
            U64Value,
            U128Value,
            UsizeValue,
            F32Value,
            F64Value,
            StringValue,
            BigIntValue,
            BigUintValue,
            JsonValue,
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
                            "boolValue" | "bool_value" => Ok(GeneratedField::BoolValue),
                            "charValue" | "char_value" => Ok(GeneratedField::CharValue),
                            "i8Value" | "i8_value" => Ok(GeneratedField::I8Value),
                            "i16Value" | "i16_value" => Ok(GeneratedField::I16Value),
                            "i32Value" | "i32_value" => Ok(GeneratedField::I32Value),
                            "i64Value" | "i64_value" => Ok(GeneratedField::I64Value),
                            "i128Value" | "i128_value" => Ok(GeneratedField::I128Value),
                            "isizeValue" | "isize_value" => Ok(GeneratedField::IsizeValue),
                            "u8Value" | "u8_value" => Ok(GeneratedField::U8Value),
                            "u16Value" | "u16_value" => Ok(GeneratedField::U16Value),
                            "u32Value" | "u32_value" => Ok(GeneratedField::U32Value),
                            "u64Value" | "u64_value" => Ok(GeneratedField::U64Value),
                            "u128Value" | "u128_value" => Ok(GeneratedField::U128Value),
                            "usizeValue" | "usize_value" => Ok(GeneratedField::UsizeValue),
                            "f32Value" | "f32_value" => Ok(GeneratedField::F32Value),
                            "f64Value" | "f64_value" => Ok(GeneratedField::F64Value),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "bigIntValue" | "big_int_value" => Ok(GeneratedField::BigIntValue),
                            "bigUintValue" | "big_uint_value" => Ok(GeneratedField::BigUintValue),
                            "jsonValue" | "json_value" => Ok(GeneratedField::JsonValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.storage.v1.ColumnValue")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ColumnValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BoolValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            value__ = map.next_value::<::std::option::Option<_>>()?.map(column_value::Value::BoolValue);
                        }
                        GeneratedField::CharValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("charValue"));
                            }
                            value__ = map.next_value::<::std::option::Option<_>>()?.map(column_value::Value::CharValue);
                        }
                        GeneratedField::I8Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("i8Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::I8Value(x.0));
                        }
                        GeneratedField::I16Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("i16Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::I16Value(x.0));
                        }
                        GeneratedField::I32Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("i32Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::I32Value(x.0));
                        }
                        GeneratedField::I64Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("i64Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::I64Value(x.0));
                        }
                        GeneratedField::I128Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("i128Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| column_value::Value::I128Value(x.0));
                        }
                        GeneratedField::IsizeValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isizeValue"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::IsizeValue(x.0));
                        }
                        GeneratedField::U8Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u8Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::U8Value(x.0));
                        }
                        GeneratedField::U16Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u16Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::U16Value(x.0));
                        }
                        GeneratedField::U32Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u32Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::U32Value(x.0));
                        }
                        GeneratedField::U64Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u64Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::U64Value(x.0));
                        }
                        GeneratedField::U128Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u128Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| column_value::Value::U128Value(x.0));
                        }
                        GeneratedField::UsizeValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usizeValue"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::UsizeValue(x.0));
                        }
                        GeneratedField::F32Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("f32Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::F32Value(x.0));
                        }
                        GeneratedField::F64Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("f64Value"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| column_value::Value::F64Value(x.0));
                        }
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map.next_value::<::std::option::Option<_>>()?.map(column_value::Value::StringValue);
                        }
                        GeneratedField::BigIntValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bigIntValue"));
                            }
                            value__ = map.next_value::<::std::option::Option<_>>()?.map(column_value::Value::BigIntValue)
;
                        }
                        GeneratedField::BigUintValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bigUintValue"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| column_value::Value::BigUintValue(x.0));
                        }
                        GeneratedField::JsonValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonValue"));
                            }
                            value__ = map.next_value::<::std::option::Option<_>>()?.map(column_value::Value::JsonValue);
                        }
                    }
                }
                Ok(ColumnValue {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.storage.v1.ColumnValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Condition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operator != 0 {
            len += 1;
        }
        if !self.sub_filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.storage.v1.Condition", len)?;
        if self.operator != 0 {
            let v = ConditionOperator::from_i32(self.operator)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.operator)))?;
            struct_ser.serialize_field("operator", &v)?;
        }
        if !self.sub_filters.is_empty() {
            struct_ser.serialize_field("subFilters", &self.sub_filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Condition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operator",
            "sub_filters",
            "subFilters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operator,
            SubFilters,
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
                            "operator" => Ok(GeneratedField::Operator),
                            "subFilters" | "sub_filters" => Ok(GeneratedField::SubFilters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Condition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.storage.v1.Condition")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Condition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operator__ = None;
                let mut sub_filters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operator => {
                            if operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            operator__ = Some(map.next_value::<ConditionOperator>()? as i32);
                        }
                        GeneratedField::SubFilters => {
                            if sub_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subFilters"));
                            }
                            sub_filters__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Condition {
                    operator: operator__.unwrap_or_default(),
                    sub_filters: sub_filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.storage.v1.Condition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConditionOperator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CONDITION_OPERATOR_UNSPECIFIED",
            Self::And => "CONDITION_OPERATOR_AND",
            Self::Or => "CONDITION_OPERATOR_OR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ConditionOperator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONDITION_OPERATOR_UNSPECIFIED",
            "CONDITION_OPERATOR_AND",
            "CONDITION_OPERATOR_OR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConditionOperator;

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
                    .and_then(ConditionOperator::from_i32)
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
                    .and_then(ConditionOperator::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CONDITION_OPERATOR_UNSPECIFIED" => Ok(ConditionOperator::Unspecified),
                    "CONDITION_OPERATOR_AND" => Ok(ConditionOperator::And),
                    "CONDITION_OPERATOR_OR" => Ok(ConditionOperator::Or),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Order {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ORDER_UNSPECIFIED",
            Self::Asc => "ORDER_ASC",
            Self::Desc => "ORDER_DESC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Order {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ORDER_UNSPECIFIED",
            "ORDER_ASC",
            "ORDER_DESC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Order;

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
                    .and_then(Order::from_i32)
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
                    .and_then(Order::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ORDER_UNSPECIFIED" => Ok(Order::Unspecified),
                    "ORDER_ASC" => Ok(Order::Asc),
                    "ORDER_DESC" => Ok(Order::Desc),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OrderBy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.columns.is_empty() {
            len += 1;
        }
        if self.order != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.storage.v1.OrderBy", len)?;
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        if self.order != 0 {
            let v = Order::from_i32(self.order)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.order)))?;
            struct_ser.serialize_field("order", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderBy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "columns",
            "order",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Columns,
            Order,
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
                            "columns" => Ok(GeneratedField::Columns),
                            "order" => Ok(GeneratedField::Order),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderBy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.storage.v1.OrderBy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OrderBy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut columns__ = None;
                let mut order__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map.next_value()?);
                        }
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = Some(map.next_value::<Order>()? as i32);
                        }
                    }
                }
                Ok(OrderBy {
                    columns: columns__.unwrap_or_default(),
                    order: order__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.storage.v1.OrderBy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conditions.is_empty() {
            len += 1;
        }
        if self.conditions_operator != 0 {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.offset.is_some() {
            len += 1;
        }
        if self.order_by.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.storage.v1.QueryFilter", len)?;
        if !self.conditions.is_empty() {
            struct_ser.serialize_field("conditions", &self.conditions)?;
        }
        if self.conditions_operator != 0 {
            let v = ConditionOperator::from_i32(self.conditions_operator)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.conditions_operator)))?;
            struct_ser.serialize_field("conditionsOperator", &v)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.offset.as_ref() {
            struct_ser.serialize_field("offset", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.order_by.as_ref() {
            struct_ser.serialize_field("orderBy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conditions",
            "conditions_operator",
            "conditionsOperator",
            "limit",
            "offset",
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Conditions,
            ConditionsOperator,
            Limit,
            Offset,
            OrderBy,
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
                            "conditions" => Ok(GeneratedField::Conditions),
                            "conditionsOperator" | "conditions_operator" => Ok(GeneratedField::ConditionsOperator),
                            "limit" => Ok(GeneratedField::Limit),
                            "offset" => Ok(GeneratedField::Offset),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.storage.v1.QueryFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conditions__ = None;
                let mut conditions_operator__ = None;
                let mut limit__ = None;
                let mut offset__ = None;
                let mut order_by__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Conditions => {
                            if conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            conditions__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConditionsOperator => {
                            if conditions_operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditionsOperator"));
                            }
                            conditions_operator__ = Some(map.next_value::<ConditionOperator>()? as i32);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryFilter {
                    conditions: conditions__.unwrap_or_default(),
                    conditions_operator: conditions_operator__.unwrap_or_default(),
                    limit: limit__,
                    offset: offset__,
                    order_by: order_by__,
                })
            }
        }
        deserializer.deserialize_struct("mystiko.storage.v1.QueryFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operator != 0 {
            len += 1;
        }
        if !self.column.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mystiko.storage.v1.SubFilter", len)?;
        if self.operator != 0 {
            let v = SubFilterOperator::from_i32(self.operator)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.operator)))?;
            struct_ser.serialize_field("operator", &v)?;
        }
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operator",
            "column",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operator,
            Column,
            Values,
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
                            "operator" => Ok(GeneratedField::Operator),
                            "column" => Ok(GeneratedField::Column),
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mystiko.storage.v1.SubFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operator__ = None;
                let mut column__ = None;
                let mut values__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operator => {
                            if operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            operator__ = Some(map.next_value::<SubFilterOperator>()? as i32);
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = Some(map.next_value()?);
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SubFilter {
                    operator: operator__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mystiko.storage.v1.SubFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubFilterOperator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SUB_FILTER_OPERATOR_UNSPECIFIED",
            Self::IsNull => "SUB_FILTER_OPERATOR_IS_NULL",
            Self::IsNotNull => "SUB_FILTER_OPERATOR_IS_NOT_NULL",
            Self::Equal => "SUB_FILTER_OPERATOR_EQUAL",
            Self::NotEqual => "SUB_FILTER_OPERATOR_NOT_EQUAL",
            Self::Greater => "SUB_FILTER_OPERATOR_GREATER",
            Self::GreaterEqual => "SUB_FILTER_OPERATOR_GREATER_EQUAL",
            Self::Less => "SUB_FILTER_OPERATOR_LESS",
            Self::LessEqual => "SUB_FILTER_OPERATOR_LESS_EQUAL",
            Self::BetweenAnd => "SUB_FILTER_OPERATOR_BETWEEN_AND",
            Self::In => "SUB_FILTER_OPERATOR_IN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SubFilterOperator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SUB_FILTER_OPERATOR_UNSPECIFIED",
            "SUB_FILTER_OPERATOR_IS_NULL",
            "SUB_FILTER_OPERATOR_IS_NOT_NULL",
            "SUB_FILTER_OPERATOR_EQUAL",
            "SUB_FILTER_OPERATOR_NOT_EQUAL",
            "SUB_FILTER_OPERATOR_GREATER",
            "SUB_FILTER_OPERATOR_GREATER_EQUAL",
            "SUB_FILTER_OPERATOR_LESS",
            "SUB_FILTER_OPERATOR_LESS_EQUAL",
            "SUB_FILTER_OPERATOR_BETWEEN_AND",
            "SUB_FILTER_OPERATOR_IN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubFilterOperator;

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
                    .and_then(SubFilterOperator::from_i32)
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
                    .and_then(SubFilterOperator::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SUB_FILTER_OPERATOR_UNSPECIFIED" => Ok(SubFilterOperator::Unspecified),
                    "SUB_FILTER_OPERATOR_IS_NULL" => Ok(SubFilterOperator::IsNull),
                    "SUB_FILTER_OPERATOR_IS_NOT_NULL" => Ok(SubFilterOperator::IsNotNull),
                    "SUB_FILTER_OPERATOR_EQUAL" => Ok(SubFilterOperator::Equal),
                    "SUB_FILTER_OPERATOR_NOT_EQUAL" => Ok(SubFilterOperator::NotEqual),
                    "SUB_FILTER_OPERATOR_GREATER" => Ok(SubFilterOperator::Greater),
                    "SUB_FILTER_OPERATOR_GREATER_EQUAL" => Ok(SubFilterOperator::GreaterEqual),
                    "SUB_FILTER_OPERATOR_LESS" => Ok(SubFilterOperator::Less),
                    "SUB_FILTER_OPERATOR_LESS_EQUAL" => Ok(SubFilterOperator::LessEqual),
                    "SUB_FILTER_OPERATOR_BETWEEN_AND" => Ok(SubFilterOperator::BetweenAnd),
                    "SUB_FILTER_OPERATOR_IN" => Ok(SubFilterOperator::In),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
