use anyhow::Result;
use http::Uri;
use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use mystiko_protos::service::v1::ClientOptions;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tonic::client::Grpc;
use tonic_web::{GrpcWebClientLayer, GrpcWebClientService};
use tower::util::MapRequest;
use tower::ServiceBuilder;

type GrpcClient<B, F> = hyper::Client<HttpsConnector<MapRequest<HttpConnector, F>>, B>;
type WebClient<B, F> = GrpcWebClientService<GrpcClient<B, F>>;

pub async fn grpc_client<B>(options: &ClientOptions) -> Result<Grpc<GrpcClient<B, impl FnMut(Uri) -> Uri + Clone>>>
where
    B: HttpBody + Send,
    <B as HttpBody>::Data: Send,
{
    let uri = options.to_uri()?;
    Ok(Grpc::with_origin(create_client(options).await?, uri))
}

pub async fn grpc_web_client<B>(options: &ClientOptions) -> Result<Grpc<WebClient<B, impl FnMut(Uri) -> Uri + Clone>>>
where
    B: HttpBody + Send,
    <B as HttpBody>::Data: Send,
{
    let uri = options.to_uri()?;
    let svc = ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(create_client(options).await?);
    Ok(Grpc::with_origin(svc, uri))
}

async fn create_client<B>(options: &ClientOptions) -> Result<GrpcClient<B, impl FnMut(Uri) -> Uri + Clone>>
where
    B: HttpBody + Send,
    <B as HttpBody>::Data: Send,
{
    let uri = options.to_uri()?;
    let mut http_connector = HttpConnector::new();
    http_connector.enforce_http(false);
    let tls = tls_config(options).await?;
    let connector = ServiceBuilder::new()
        .layer_fn(move |s| {
            let tls = tls.clone();
            HttpsConnectorBuilder::new()
                .with_tls_config(tls)
                .https_or_http()
                .enable_http2()
                .wrap_connector(s)
        })
        .map_request(move |_| uri.clone())
        .service(http_connector);
    Ok(hyper::Client::builder().build(connector))
}

async fn tls_config(options: &ClientOptions) -> Result<ClientConfig, std::io::Error> {
    let mut root_certs = RootCertStore::empty();
    if let Some(ssl_cert) = options.ssl_cert.clone() {
        let certs = certs_from_bytes(ssl_cert)?;
        root_certs.add_parsable_certificates(&certs);
    }
    #[cfg(feature = "fs")]
    if let Some(ssl_cert_path) = options.ssl_cert_path.clone() {
        let certs = certs_from_file(&ssl_cert_path).await?;
        root_certs.add_parsable_certificates(&certs);
    }
    Ok(ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_certs)
        .with_no_client_auth())
}

fn certs_from_bytes(ssl_cert: Vec<u8>) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let cursor = std::io::Cursor::new(ssl_cert);
    let mut buf = std::io::BufReader::new(cursor);
    rustls_pemfile::certs(&mut buf)
}

#[cfg(feature = "fs")]
async fn certs_from_file(ssl_cert_path: &str) -> Result<Vec<Vec<u8>>, std::io::Error> {
    tokio::fs::read(ssl_cert_path).await.and_then(certs_from_bytes)
}
