use mystiko_protos::service::v1::ClientOptions;

#[test]
fn test_scheme() {
    let client_options = ClientOptions::default();
    assert_eq!(client_options.scheme(), "http");
    let client_options = ClientOptions::builder().is_ssl(true).build();
    assert_eq!(client_options.scheme(), "https");
}

#[test]
fn test_to_uri() {
    let client_options = ClientOptions::default();
    assert_eq!(client_options.to_uri().unwrap().to_string(), "http://127.0.0.1/");
    let client_options = ClientOptions::builder().is_ssl(true).build();
    assert_eq!(client_options.to_uri().unwrap().to_string(), "https://127.0.0.1/");
    let client_options = ClientOptions::builder()
        .host("example.com".to_string())
        .is_ssl(true)
        .build();
    assert_eq!(client_options.to_uri().unwrap().to_string(), "https://example.com/");
    let client_options = ClientOptions::builder()
        .host("example.com".to_string())
        .port(8451)
        .build();
    assert_eq!(client_options.to_uri().unwrap().to_string(), "http://example.com:8451/");
}
