use mystiko_indexer_client::builder::IndexerClientBuilder;
use std::time::Duration;
use url::ParseError;

#[test]
fn test_builder() {
    let base_url = "http://test_domain:3098";
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
    let second_base_url = "http://test_domain:3099";
    builder = builder
        .base_url(second_base_url)
        .timeout(Duration::from_secs(30));
    assert_eq!(builder.timeout, Duration::from_secs(30));
    assert_eq!(builder.base_url, second_base_url);
    let client = builder.build().unwrap();
    assert_eq!(client.base_url, second_base_url.to_string());
}

#[test]
fn test_builder_with_error() {
    let base_url = "http://test_domain:error_port";
    let builder = IndexerClientBuilder::new(base_url);
    let client = builder.build();
    assert!(client.is_err());
    assert_eq!(
        client.err().unwrap().to_string(),
        ParseError::InvalidPort.to_string()
    );
    let base_url2 = "error_domain_error_port";
    let builder2 = IndexerClientBuilder::new(base_url2);
    let client2 = builder2.build();
    assert!(client2.is_err());
    assert_eq!(
        client2.err().unwrap().to_string(),
        ParseError::RelativeUrlWithoutBase.to_string()
    );
}
