use anyhow::Result;
use mystiko_storage::document::*;
use mystiko_storage::error::StorageError;
use mystiko_storage::testing::*;
use num_bigint::BigInt;
use num_traits::{Float, PrimInt};
use std::str::FromStr;

#[test]
fn test_document_definition() {
    let mut doc: Document<TestDocumentData> = Document {
        id: String::from("1234"),
        created_at: 0xdead,
        updated_at: 0xbeef,
        data: TestDocumentData {
            field1: String::from("field value"),
            field2: 0xbaad,
            field3: None,
        },
    };
    assert_eq!(doc.id, "1234");
    assert_eq!(doc.created_at, 0xdead);
    assert_eq!(doc.updated_at, 0xbeef);
    assert_eq!(doc.data.field1, "field value");
    assert_eq!(doc.data.field2, 0xbaad);
    assert!(doc.data.field3.is_none());
    assert_eq!(TestDocumentData::schema().version(), TEST_DOCUMENT_SCHEMA.version());
    assert_eq!(TestDocumentData::schema().field_names, TEST_DOCUMENT_SCHEMA.field_names);
    assert_eq!(
        TestDocumentData::schema().collection_name,
        TEST_DOCUMENT_SCHEMA.collection_name
    );
    assert_eq!(doc.field_value_string("id").unwrap(), "1234");
    assert_eq!(doc.field_value_string("created_at").unwrap(), 0xdead.to_string());
    assert_eq!(doc.field_value_string("updated_at").unwrap(), 0xbeef.to_string());
    assert_eq!(doc.field_value_string("field1").unwrap(), "field value");
    assert_eq!(doc.field_value_string("field2").unwrap(), 0xbaad.to_string());
    assert!(doc.field_value_string("field3").is_none());
    doc.data.field3 = Some(std::f64::consts::PI);
    assert_eq!(
        doc.field_value_string("field3").unwrap(),
        std::f64::consts::PI.to_string()
    );
}

#[test]
fn test_document_deserialize() {
    #[derive(Debug)]
    struct TestDocumentRawData {
        field3: Option<f64>,
    }
    impl DocumentRawData for TestDocumentRawData {
        fn field_integer_value<T: PrimInt>(&self, field: &str) -> Result<Option<T>, StorageError> {
            if field.eq(DOCUMENT_CREATED_AT_FIELD) {
                Ok(T::from(0xdead))
            } else if field.eq(DOCUMENT_UPDATED_AT_FIELD) {
                Ok(T::from(0xbeef))
            } else if field.eq("field2") {
                Ok(T::from(0xbaad))
            } else {
                Err(StorageError::CorruptedDataError("wrong field name".into()))
            }
        }

        fn field_float_value<T: Float>(&self, field: &str) -> Result<Option<T>, StorageError> {
            if field.eq("field3") {
                Ok(self.field3.and_then(|f| T::from(f)))
            } else {
                Err(StorageError::CorruptedDataError("wrong field name".into()))
            }
        }

        fn field_string_value(&self, field: &str) -> Result<Option<String>, StorageError> {
            if field.eq(DOCUMENT_ID_FIELD) {
                Ok(Some(String::from("1234")))
            } else if field.eq("field1") {
                Ok(Some(String::from("field value")))
            } else {
                Err(StorageError::CorruptedDataError("wrong field name".into()))
            }
        }
    }
    let doc1: Document<TestDocumentData> = Document::deserialize(&TestDocumentRawData { field3: None }).unwrap();
    assert_eq!(doc1.id, "1234");
    assert_eq!(doc1.created_at, 0xdead);
    assert_eq!(doc1.updated_at, 0xbeef);
    assert_eq!(doc1.data.field1, "field value");
    assert_eq!(doc1.data.field2, 0xbaad);
    assert!(doc1.data.field3.is_none());
    let doc2: Document<TestDocumentData> = Document::deserialize(&TestDocumentRawData {
        field3: Some(std::f64::consts::PI),
    })
    .unwrap();
    assert_eq!(doc2.id, "1234");
    assert_eq!(doc2.created_at, 0xdead);
    assert_eq!(doc2.updated_at, 0xbeef);
    assert_eq!(doc2.data.field1, "field value");
    assert_eq!(doc2.data.field2, 0xbaad);
    assert_eq!(doc2.data.field3.unwrap(), std::f64::consts::PI);
    assert_ne!(doc1, doc2);
    assert_eq!(doc1, doc1.clone());
}

#[test]
fn test_document_deserialize_error() {
    static TEST_DOCUMENT_SCHEMA: DocumentSchema = DocumentSchema {
        collection_name: "test_documents",
        migrations: &["CREATE TABLE `test_documents` (\
            `id` VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at` INT NOT NULL,\
            `updated_at` INT NOT NULL,\
            `field1` VARCHAR(255) NOT NULL,\
            `field2` INT NOT NULL,\
            `field3` REAL NOT NULL,\
            `field4` VARCHAR(128) NOT NULL,\
            `field5` VARCHAR(255),\
            `field6` INT,\
            `field7` REAL,\
            `field8` VARCHAR(128))"],
        field_names: &[
            "field1", "field2", "field3", "field4", "field4", "field5", "field6", "field7",
        ],
    };

    #[derive(Clone, PartialEq, Debug)]
    struct TestDocumentData {
        pub field1: String,
        pub field2: u32,
        pub field3: f32,
        pub field4: BigInt,
        pub field5: Option<String>,
        pub field6: Option<u32>,
        pub field7: Option<f32>,
        pub field8: Option<BigInt>,
    }

    impl DocumentData for TestDocumentData {
        fn schema() -> &'static DocumentSchema {
            &TEST_DOCUMENT_SCHEMA
        }

        fn field_value_string(&self, field: &str) -> Option<String> {
            match field {
                "field1" => Some(self.field1.clone()),
                "field2" => Some(self.field2.to_string()),
                "field3" => Some(self.field3.to_string()),
                "field4" => Some(self.field4.to_string()),
                "field5" => self.field5.clone(),
                "field6" => self.field6.map(|v| v.to_string()),
                "field7" => self.field7.map(|v| v.to_string()),
                "field8" => self.field8.as_ref().map(|v| v.to_string()),
                _ => None,
            }
        }

        fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, StorageError> {
            Ok(TestDocumentData {
                field1: raw.required_field_string_value("field1")?,
                field2: raw.required_field_integer_value::<u32>("field2")?,
                field3: raw.required_field_float_value::<f32>("field3")?,
                field4: raw.required_bigint_value("field4")?,
                field5: raw.field_string_value("field5")?,
                field6: raw.field_integer_value::<u32>("field6")?,
                field7: raw.field_float_value::<f32>("field7")?,
                field8: raw.field_bigint_value("field8")?,
            })
        }
    }

    #[derive(Debug, Clone, typed_builder::TypedBuilder)]
    struct TestDocumentRawData {
        #[builder(default, setter(strip_option))]
        pub field1: Option<String>,
        #[builder(default, setter(strip_option))]
        pub field2: Option<u32>,
        #[builder(default, setter(strip_option))]
        pub field3: Option<f32>,
        #[builder(default, setter(strip_option))]
        pub field4: Option<String>,
        #[builder(default, setter(strip_option))]
        pub field5: Option<String>,
        #[builder(default, setter(strip_option))]
        pub field6: Option<u32>,
        #[builder(default, setter(strip_option))]
        pub field7: Option<f32>,
        #[builder(default, setter(strip_option))]
        pub field8: Option<String>,
    }

    impl DocumentRawData for TestDocumentRawData {
        fn field_integer_value<T: PrimInt + FromStr>(&self, field: &str) -> Result<Option<T>, StorageError> {
            if field.eq(DOCUMENT_CREATED_AT_FIELD) {
                Ok(T::from(0xdead))
            } else if field.eq(DOCUMENT_UPDATED_AT_FIELD) {
                Ok(T::from(0xbeef))
            } else if field.eq("field2") {
                Ok(if let Some(field2) = self.field2 {
                    T::from(field2)
                } else {
                    None
                })
            } else if field.eq("field6") {
                Ok(if let Some(field6) = self.field6 {
                    T::from(field6)
                } else {
                    None
                })
            } else {
                Err(StorageError::CorruptedDataError("wrong field name".into()))
            }
        }

        fn field_float_value<T: Float + FromStr>(&self, field: &str) -> Result<Option<T>, StorageError> {
            if field.eq("field3") {
                Ok(if let Some(field3) = self.field3 {
                    T::from(field3)
                } else {
                    None
                })
            } else if field.eq("field7") {
                Ok(if let Some(field7) = self.field7 {
                    T::from(field7)
                } else {
                    None
                })
            } else {
                Err(StorageError::CorruptedDataError("wrong field name".into()))
            }
        }

        fn field_string_value(&self, field: &str) -> Result<Option<String>, StorageError> {
            if field.eq(DOCUMENT_ID_FIELD) {
                Ok(Some(String::from("1234")))
            } else if field.eq("field1") {
                Ok(self.field1.clone())
            } else if field.eq("field4") {
                Ok(self.field4.clone())
            } else if field.eq("field5") {
                Ok(self.field5.clone())
            } else if field.eq("field8") {
                Ok(self.field8.clone())
            } else {
                Err(StorageError::CorruptedDataError("wrong field name".into()))
            }
        }
    }
    let mut raw_data = TestDocumentRawData::builder()
        .field1("f1_v1".into())
        .field2(1111)
        .field3(2222.3)
        .field4("4444".into())
        .build();
    let mut data = TestDocumentData::deserialize(&raw_data).unwrap();
    assert_eq!(data.field1, "f1_v1");
    assert_eq!(data.field2, 1111);
    assert_eq!(data.field3, 2222.3);
    assert_eq!(data.field4, 4444.into());
    assert!(data.field5.is_none());
    assert!(data.field6.is_none());
    assert!(data.field7.is_none());
    assert!(data.field8.is_none());
    raw_data.field5 = Some("f5_v1".into());
    raw_data.field6 = Some(11111);
    raw_data.field7 = Some(22222.3);
    raw_data.field8 = Some("44444".into());
    data = TestDocumentData::deserialize(&raw_data).unwrap();
    assert_eq!(data.field1, "f1_v1");
    assert_eq!(data.field2, 1111);
    assert_eq!(data.field3, 2222.3);
    assert_eq!(data.field4, 4444.into());
    assert_eq!(data.field5.unwrap(), "f5_v1");
    assert_eq!(data.field6.unwrap(), 11111);
    assert_eq!(data.field7.unwrap(), 22222.3);
    assert_eq!(data.field8.unwrap(), 44444.into());
    raw_data.field1 = None;
    assert!(TestDocumentData::deserialize(&raw_data).is_err());
    raw_data.field1 = Some("f1_v1".into());
    raw_data.field2 = None;
    assert!(TestDocumentData::deserialize(&raw_data).is_err());
    raw_data.field2 = Some(1111);
    raw_data.field3 = None;
    assert!(TestDocumentData::deserialize(&raw_data).is_err());
    raw_data.field3 = Some(2222.3);
    raw_data.field4 = None;
    assert!(TestDocumentData::deserialize(&raw_data).is_err());
}

#[test]
fn test_field_index() {
    assert_eq!(Document::<TestDocumentData>::field_index(DOCUMENT_ID_FIELD).unwrap(), 0);
    assert_eq!(
        Document::<TestDocumentData>::field_index(DOCUMENT_CREATED_AT_FIELD).unwrap(),
        1
    );
    assert_eq!(
        Document::<TestDocumentData>::field_index(DOCUMENT_UPDATED_AT_FIELD).unwrap(),
        2
    );
    assert_eq!(Document::<TestDocumentData>::field_index("field1").unwrap(), 3);
    assert_eq!(Document::<TestDocumentData>::field_index("field2").unwrap(), 4);
    assert_eq!(Document::<TestDocumentData>::field_index("field3").unwrap(), 5);
    assert!(Document::<TestDocumentData>::field_index("wrong field").is_none());
}
