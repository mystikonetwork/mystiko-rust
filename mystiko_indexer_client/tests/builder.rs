use mystiko_indexer_client::builder::IndexerClientBuilder;
use std::time::Duration;

#[test]
fn test_builder() {
    let base_url = "http://test_domain:test_port";
    let mut builder = IndexerClientBuilder::new(base_url);
    assert_eq!(builder.base_url, base_url);
    assert_eq!(builder.timeout, Duration::from_secs(20));
    assert_eq!(builder.auth_username, None);
    assert_eq!(builder.auth_password, None);
    builder = builder
        .auth_username(Some(String::from("test_username")))
        .auth_password(Some(String::from("test_password")))
        .auth_password(Some(String::from("test_password")));
    assert_eq!(builder.auth_username, Some(String::from("test_username")));
    assert_eq!(builder.auth_password, Some(String::from("test_password")));
    let second_base_url = String::from("http://test_domain:test_port2");
    builder = builder
        .base_url(second_base_url.clone())
        .timeout(Duration::from_secs(30));
    assert_eq!(builder.timeout, Duration::from_secs(30));
    assert_eq!(builder.base_url, second_base_url);
    let client = builder.build();
    assert_eq!(client.base_url, second_base_url);
}
