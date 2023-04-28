use anyhow::{Error, Result};
use mystiko_storage::document::*;
use mystiko_storage::testing::*;
use num_traits::{Float, PrimInt};

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
        fn field_integer_value<T: PrimInt>(&self, field: &str) -> Result<Option<T>> {
            if field.eq(DOCUMENT_CREATED_AT_FIELD) {
                Ok(T::from(0xdead))
            } else if field.eq(DOCUMENT_UPDATED_AT_FIELD) {
                Ok(T::from(0xbeef))
            } else if field.eq("field2") {
                Ok(T::from(0xbaad))
            } else {
                Err(Error::msg("wrong field name"))
            }
        }

        fn field_float_value<T: Float>(&self, field: &str) -> Result<Option<T>> {
            if field.eq("field3") {
                Ok(self.field3.and_then(|f| T::from(f)))
            } else {
                Err(Error::msg("wrong field name"))
            }
        }

        fn field_string_value(&self, field: &str) -> Result<Option<String>> {
            if field.eq(DOCUMENT_ID_FIELD) {
                Ok(Some(String::from("1234")))
            } else if field.eq("field1") {
                Ok(Some(String::from("field value")))
            } else {
                Err(Error::msg("wrong field name"))
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
