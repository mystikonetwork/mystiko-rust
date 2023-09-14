use mystiko_protos::service::v1::ServerOptions;
use tonic::transport::Server;

#[test]
fn test_from_option() {
    let options: ServerOptions = None.into();
    assert_eq!(options, ServerOptions::default());
    let options = ServerOptions::builder().port(50051u32).build();
    let another: ServerOptions = Some(options.clone()).into();
    assert_eq!(another, options);
}

#[test]
fn test_into_server() {
    let options = ServerOptions::default();
    let _: Server = options.into();
    let options = ServerOptions::builder()
        .accept_http1(true)
        .concurrency_limit_per_connection(1)
        .timeout_ms(1000)
        .tcp_nodelay(true)
        .initial_stream_window_size(1)
        .initial_connection_window_size(1)
        .max_concurrent_streams(1)
        .http2_keepalive_interval_ms(1000)
        .http2_keepalive_timeout_ms(1000)
        .http2_adaptive_window(true)
        .tcp_keepalive_ms(1000)
        .max_frame_size(1)
        .build();
    let _: Server = options.into();
}
