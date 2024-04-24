use mystiko_config::RawSequencerConfig;
use mystiko_protos::service::v1::ClientOptions;
use std::sync::Arc;
use validator::Validate;

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

#[test]
fn test_validate() {
    let client_options = ClientOptions::builder().host("invalid host".to_string()).build();
    assert!(client_options.validate().is_err());
}

#[test]
fn test_sequencer_config_to_client_options() {
    let raw_sequencer_config = RawSequencerConfig::builder()
        .host("example.com".to_string())
        .port(8451)
        .is_ssl(true)
        .build();
    let sequencer_config = mystiko_config::SequencerConfig::new(Arc::new(raw_sequencer_config));
    let client_options: ClientOptions = (&sequencer_config).into();
    assert_eq!(client_options.host(), "example.com");
    assert_eq!(client_options.port(), 8451);
    assert!(client_options.is_ssl());
}
