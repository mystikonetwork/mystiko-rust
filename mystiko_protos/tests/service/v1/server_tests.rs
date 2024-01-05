use mystiko_protos::service::v1::ServerOptions;
use rcgen::{generate_simple_self_signed, Certificate, CertificateParams};
use std::path::PathBuf;
use tempfile::TempDir;
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
fn test_try_into_server() {
    let options = ServerOptions::default();
    let _: Server = options.try_into().unwrap();

    let mut options = ServerOptions::builder()
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
    let _: Server = options.clone().try_into().unwrap();

    let (server_pem, server_key) = generate_tls("example.com").unwrap();
    options.tls_key = Some(server_key);
    options.tls_pem = Some(server_pem);
    let _: Server = options.clone().try_into().unwrap();

    let (_temp_dir, server_pem_path, server_key_path) = generate_tls_files("example.com").unwrap();
    options.tls_key = None;
    options.tls_pem = None;
    options.tls_key_path = Some(server_key_path.to_string_lossy().to_string());
    options.tls_pem_path = Some(server_pem_path.to_string_lossy().to_string());
    let _: Server = options.try_into().unwrap();
}

fn generate_tls(domain_name: &str) -> anyhow::Result<(String, String)> {
    let ca_cert = generate_simple_self_signed(vec![domain_name.to_string()])?;
    let mut server_params = CertificateParams::new(vec![domain_name.to_string()]);
    server_params.is_ca = rcgen::IsCa::NoCa;
    let server_cert = Certificate::from_params(server_params)?;
    let server_pem = server_cert.serialize_pem_with_signer(&ca_cert)?;
    let server_key = server_cert.serialize_private_key_pem();
    Ok((server_pem, server_key))
}

fn generate_tls_files(domain_name: &str) -> anyhow::Result<(TempDir, PathBuf, PathBuf)> {
    let (server_pem, server_key) = generate_tls(domain_name)?;
    let temp_folder = TempDir::new()?;
    let server_pem_path = temp_folder.path().join("server.pem");
    let server_key_path = temp_folder.path().join("server.key");
    std::fs::write(server_pem_path.as_path(), server_pem)?;
    std::fs::write(server_key_path.as_path(), server_key)?;
    Ok((temp_folder, server_pem_path, server_key_path))
}
