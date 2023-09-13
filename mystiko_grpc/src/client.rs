use anyhow::Result;
use mystiko_protos::service::v1::ClientOptions;
use std::str::FromStr;

pub async fn create_client(options: &ClientOptions) -> Result<tonic::transport::Channel> {
    let host = options.host.clone().unwrap_or("127.0.0.1".to_string());
    let port = options.port.unwrap_or(80u32);
    let schema = if options.is_ssl() { "https" } else { "http" };
    let uri = http::Uri::from_str(&format!("{}/{}:{}", schema, host, port))?;
    if options.is_ssl() {
        let ssl_host = options.ssl_server_name.clone().unwrap_or(host);
        let ssl_uri = http::Uri::from_str(&format!("{}/{}:{}", schema, ssl_host, port))?;
        let mut root_certs = tokio_rustls::rustls::RootCertStore::empty();
        if let Some(ssl_cert) = options.ssl_cert.clone() {
            let cursor = std::io::Cursor::new(ssl_cert);
            let mut buf = std::io::BufReader::new(cursor);
            let certs = rustls_pemfile::certs(&mut buf)?;
            root_certs.add_parsable_certificates(&certs);
        }
        let mut http = hyper::client::HttpConnector::new();
        http.enforce_http(false);
        let tls = tokio_rustls::rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_certs)
            .with_no_client_auth();
        let connector = tower::ServiceBuilder::new()
            .layer_fn(move |s| {
                let tls = tls.clone();
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_tls_config(tls)
                    .https_or_http()
                    .enable_http2()
                    .wrap_connector(s)
            })
            .map_request(move |_| uri.clone())
            .service(http);
        return Ok(tonic::transport::Channel::builder(ssl_uri)
            .connect_with_connector(connector)
            .await?);
    }
    return Ok(tonic::transport::Endpoint::from(uri).connect().await?);
}

fn certs_from_bytes(ssl_cert: Vec<u8>) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let cursor = std::io::Cursor::new(ssl_cert);
    let mut buf = std::io::BufReader::new(cursor);
    rustls_pemfile::certs(&mut buf)
}

#[cfg(feature = "fs")]
fn certs_from_file(ssl_cert_path: &str) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let file = std::fs::File::open(ssl_cert_path)?;
    let mut buf = std::io::BufReader::new(file);
    rustls_pemfile::certs(&mut buf)
}
