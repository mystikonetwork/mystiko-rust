use mystiko_utils::json::{to_safe_json_string, to_safe_json_string_with_keywords};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[test]
fn test_to_safe_json_string() {
    let object = TestStruct1::builder()
        .private_key("MyAwesomePrivateKey".to_string())
        .credentials(vec![TestStruct2::builder()
            .mysql_username("root".to_string())
            .mysql_password("PW".to_string())
            .build()])
        .build();
    let json_string = to_safe_json_string(&object, false).unwrap();
    let deserialized = serde_json::from_str::<TestStruct1>(&json_string).unwrap();
    assert_eq!(deserialized.private_key, "M****y");
    assert_eq!(deserialized.credentials[0].mysql_password, "****");
    let pretty_json_string = to_safe_json_string(&object, true).unwrap();
    let deserialized = serde_json::from_str::<TestStruct1>(&pretty_json_string).unwrap();
    assert_eq!(deserialized.private_key, "M****y");
    assert_eq!(deserialized.credentials[0].mysql_password, "****");
    let json_string =
        to_safe_json_string_with_keywords(&object, false, &["mysql_username", "mysql_password", "private_key"])
            .unwrap();
    let deserialized = serde_json::from_str::<TestStruct1>(&json_string).unwrap();
    assert_eq!(deserialized.private_key, "M****y");
    assert_eq!(deserialized.credentials[0].mysql_username, "r****t");
    assert_eq!(deserialized.credentials[0].mysql_password, "****");
}

#[derive(Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct TestStruct1 {
    field1: bool,
    private_key: String,
    credentials: Vec<TestStruct2>,
}

#[derive(Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct TestStruct2 {
    mysql_username: String,
    mysql_password: String,
}
