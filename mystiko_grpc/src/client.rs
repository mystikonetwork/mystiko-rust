use anyhow::Result;
use mystiko_protos::service::v1::ClientOptions;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint};

pub async fn connect(options: &ClientOptions) -> Result<Channel> {
    let uri = options.to_uri()?;
    let channel = if let Some(tls) = tls_config(options).await? {
        Endpoint::from(uri).tls_config(tls)?.connect_lazy()
    } else {
        Endpoint::from(uri).connect_lazy()
    };
    Ok(channel)
}

async fn tls_config(options: &ClientOptions) -> Result<Option<ClientTlsConfig>, std::io::Error> {
    if options.is_ssl() {
        let mut cert: Option<Certificate> = None;
        if let Some(ssl_cert) = options.ssl_cert.clone() {
            cert = Some(Certificate::from_pem(ssl_cert));
        } else {
            #[cfg(feature = "fs")]
            if let Some(ssl_cert_path) = options.ssl_cert_path.clone() {
                cert = Some(Certificate::from_pem(tokio::fs::read(ssl_cert_path).await?));
            }
        }
        if let Some(cert) = cert {
            let tls = if let Some(domain_name) = &options.ssl_server_name {
                ClientTlsConfig::new().ca_certificate(cert).domain_name(domain_name)
            } else {
                ClientTlsConfig::new()
                    .ca_certificate(cert)
                    .domain_name(options.get_host())
            };
            return Ok(Some(tls));
        }
    }
    Ok(None)
}
