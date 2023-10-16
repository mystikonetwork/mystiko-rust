use mystiko_utils::json::{to_safe_json_string, to_safe_json_string_with_keywords};
use serde::Serialize;
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
    assert_eq!(
        json_string,
        "{\"credentials\":[{\"mysql_password\":\"****\",\"mysql_username\":\"root\"}],\
        \"field1\":false,\"private_key\":\"M****y\"}"
    );
    let pretty_json_string = to_safe_json_string(&object, true).unwrap();
    assert_eq!(
        pretty_json_string,
        "{\n  \"credentials\": [\n    {\n      \"mysql_password\": \"****\",\n      \
        \"mysql_username\": \"root\"\n    }\n  ],\n  \"field1\": false,\n  \
        \"private_key\": \"M****y\"\n}"
    );
    let json_string =
        to_safe_json_string_with_keywords(&object, false, &["mysql_username", "mysql_password", "private_key"])
            .unwrap();
    assert_eq!(
        json_string,
        "{\"credentials\":[{\"mysql_password\":\"****\",\"mysql_username\":\"r****t\"}],\
        \"field1\":false,\"private_key\":\"M****y\"}"
    );
}

#[derive(Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct TestStruct1 {
    field1: bool,
    private_key: String,
    credentials: Vec<TestStruct2>,
}

#[derive(Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct TestStruct2 {
    mysql_username: String,
    mysql_password: String,
}
