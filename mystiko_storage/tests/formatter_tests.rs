use mystiko_storage::document::{Document, DocumentData};
use mystiko_storage::filter::{Condition, Order, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::*;
use mystiko_storage::testing::TestDocumentData;

#[test]
fn test_sql_format_insert() {
    let formatter = SqlFormatter {};
    let mut doc: Document<TestDocumentData> = Document {
        id: String::from("1234"),
        created_at: 0xdead,
        updated_at: 0xbeef,
        data: TestDocumentData {
            field1: String::from("field1 value"),
            field2: 0xbaad,
            field3: None,
        },
    };
    assert_eq!(
        formatter.format_insert(&doc),
        "INSERT INTO `test_documents` \
        (`id`, `created_at`, `updated_at`, `field1`, `field2`, `field3`) VALUES \
        ('1234', '57005', '48879', 'field1 value', '47789', NULL)"
    );
    doc.data.field3 = Some(5.467);
    assert_eq!(
        formatter.format_insert(&doc),
        "INSERT INTO `test_documents` \
        (`id`, `created_at`, `updated_at`, `field1`, `field2`, `field3`) VALUES \
        ('1234', '57005', '48879', 'field1 value', '47789', '5.467')"
    );
}

#[test]
fn test_sql_format_insert_batch() {
    let formatter = SqlFormatter {};
    let documents: Vec<Document<TestDocumentData>> = vec![
        Document {
            id: String::from("1"),
            created_at: 1000,
            updated_at: 1001,
            data: TestDocumentData {
                field1: String::from("doc1 field1"),
                field2: 5000,
                field3: None,
            },
        },
        Document {
            id: String::from("2"),
            created_at: 2000,
            updated_at: 2001,
            data: TestDocumentData {
                field1: String::from("doc2 field1"),
                field2: 5002,
                field3: Some(0.1442),
            },
        },
    ];
    assert_eq!(
        formatter.format_insert_batch(&documents),
        "INSERT INTO `test_documents` \
        (`id`, `created_at`, `updated_at`, `field1`, `field2`, `field3`) VALUES \
        ('1', '1000', '1001', 'doc1 field1', '5000', NULL);\
        INSERT INTO `test_documents` \
        (`id`, `created_at`, `updated_at`, `field1`, `field2`, `field3`) VALUES \
        ('2', '2000', '2001', 'doc2 field1', '5002', '0.1442')"
    );
}

#[test]
fn test_sql_format_update() {
    let formatter = SqlFormatter {};
    let mut doc: Document<TestDocumentData> = Document {
        id: String::from("1234"),
        created_at: 0xdead,
        updated_at: 0xbeef,
        data: TestDocumentData {
            field1: String::from("field1 value"),
            field2: 0xbabe,
            field3: None,
        },
    };
    assert_eq!(
        formatter.format_update(&doc),
        "UPDATE `test_documents` SET \
        `updated_at` = '48879', \
        `field1` = 'field1 value', \
        `field2` = '47806', \
        `field3` = NULL \
        WHERE `id` = '1234'"
    );
    doc.data.field3 = Some(0.234);
    assert_eq!(
        formatter.format_update(&doc),
        "UPDATE `test_documents` SET \
        `updated_at` = '48879', \
        `field1` = 'field1 value', \
        `field2` = '47806', \
        `field3` = '0.234' \
        WHERE `id` = '1234'"
    );
}

#[test]
fn test_sql_format_update_batch() {
    let formatter = SqlFormatter {};
    let documents: Vec<Document<TestDocumentData>> = vec![
        Document {
            id: String::from("1"),
            created_at: 1000,
            updated_at: 1001,
            data: TestDocumentData {
                field1: String::from("doc1 field1"),
                field2: 5000,
                field3: None,
            },
        },
        Document {
            id: String::from("2"),
            created_at: 2000,
            updated_at: 2001,
            data: TestDocumentData {
                field1: String::from("doc2 field1"),
                field2: 5002,
                field3: Some(0.1442),
            },
        },
    ];
    assert_eq!(
        formatter.format_update_batch(&documents),
        "UPDATE `test_documents` SET \
        `updated_at` = '1001', \
        `field1` = 'doc1 field1', \
        `field2` = '5000', \
        `field3` = NULL WHERE `id` = '1';\
        UPDATE `test_documents` SET \
        `updated_at` = '2001', \
        `field1` = 'doc2 field1', \
        `field2` = '5002', \
        `field3` = '0.1442' WHERE `id` = '2'"
    );
}

#[test]
fn test_sql_format_delete() {
    let formatter = SqlFormatter {};
    let doc: Document<TestDocumentData> = Document {
        id: String::from("1234"),
        created_at: 0xdead,
        updated_at: 0xbeef,
        data: TestDocumentData {
            field1: String::from("field1 value"),
            field2: 0xbabe,
            field3: None,
        },
    };
    assert_eq!(
        formatter.format_delete(&doc),
        "DELETE FROM `test_documents` WHERE `id` = '1234'"
    );
}

#[test]
fn test_sql_format_delete_batch() {
    let formatter = SqlFormatter {};
    let documents: Vec<Document<TestDocumentData>> = vec![
        Document {
            id: String::from("1234"),
            created_at: 0xdead,
            updated_at: 0xbeef,
            data: TestDocumentData {
                field1: String::from("field1 value"),
                field2: 0xbabe,
                field3: None,
            },
        },
        Document {
            id: String::from("1235"),
            created_at: 0xdead,
            updated_at: 0xbeef,
            data: TestDocumentData {
                field1: String::from("field1 value"),
                field2: 0xbabe,
                field3: None,
            },
        },
    ];
    assert_eq!(
        formatter.format_delete_batch(&documents),
        "DELETE FROM `test_documents` WHERE `id` = '1234';\
        DELETE FROM `test_documents` WHERE `id` = '1235'"
    );
}

#[test]
fn test_sql_format_find() {
    let formatter = SqlFormatter {};
    assert_eq!(
        formatter.format_find(TestDocumentData::schema(), None),
        "SELECT `id`, `created_at`, `updated_at`, `field1`, `field2`, `field3` \
        FROM `test_documents`",
    );
    assert_eq!(
        formatter.format_find(
            TestDocumentData::schema(),
            Some(QueryFilterBuilder::new().build())
        ),
        "SELECT `id`, `created_at`, `updated_at`, `field1`, `field2`, `field3` \
        FROM `test_documents`",
    );
    assert_eq!(
        formatter.format_find(
            TestDocumentData::schema(),
            Some(
                QueryFilterBuilder::new()
                    .filter(Condition::FILTER(SubFilter::IsNull(String::from("field3"))))
                    .build()
            )
        ),
        "SELECT `id`, `created_at`, `updated_at`, `field1`, `field2`, `field3` \
        FROM `test_documents` WHERE `field3` IS NULL",
    );
    assert_eq!(
        formatter.format_find(
            TestDocumentData::schema(),
            Some(
                QueryFilterBuilder::new()
                    .limit(10)
                    .offset(20)
                    .order_by(vec![String::from("field1")], Order::DESC)
                    .build()
            )
        ),
        "SELECT `id`, `created_at`, `updated_at`, `field1`, `field2`, `field3` \
        FROM `test_documents` ORDER BY `field1` DESC LIMIT 10 OFFSET 20",
    );
}
