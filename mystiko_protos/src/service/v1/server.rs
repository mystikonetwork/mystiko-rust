use crate::service::v1::ServerOptions;
use std::net::{AddrParseError, SocketAddr};

pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";
pub const DEFAULT_PORT: u32 = 50051;

impl ServerOptions {
    pub fn get_bind_address(&self) -> String {
        self.bind_address.clone().unwrap_or(DEFAULT_BIND_ADDRESS.to_string())
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(DEFAULT_PORT)
    }

    pub fn socket_address(&self) -> Result<SocketAddr, AddrParseError> {
        format!("{}:{}", self.get_bind_address(), self.get_port()).parse()
    }
}

impl From<Option<ServerOptions>> for ServerOptions {
    fn from(option: Option<ServerOptions>) -> Self {
        option.unwrap_or_default()
    }
}

#[cfg(feature = "grpc-server")]
#[derive(thiserror::Error, Debug)]
pub enum ServerOptionsError {
    #[error(transparent)]
    TransportError(#[from] tonic::transport::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[cfg(feature = "grpc-server")]
impl TryFrom<ServerOptions> for tonic::transport::Server {
    type Error = ServerOptionsError;
    fn try_from(options: ServerOptions) -> Result<Self, Self::Error> {
        use std::time::Duration;
        let mut server = tonic::transport::Server::builder();
        let tls_key = if let Some(tls_key) = &options.tls_key {
            Some(tls_key.to_string())
        } else if let Some(tls_key_path) = &options.tls_key_path {
            Some(std::fs::read_to_string(tls_key_path)?)
        } else {
            None
        };
        let tls_pem = if let Some(tls_pem) = &options.tls_pem {
            Some(tls_pem.to_string())
        } else if let Some(tls_pem_path) = &options.tls_pem_path {
            Some(std::fs::read_to_string(tls_pem_path)?)
        } else {
            None
        };
        if let (Some(tls_key), Some(tls_pem)) = (tls_key, tls_pem) {
            let identity = tonic::transport::Identity::from_pem(tls_pem, tls_key);
            server = server.tls_config(tonic::transport::ServerTlsConfig::new().identity(identity))?;
        }
        if let Some(accept_http1) = options.accept_http1 {
            server = server.accept_http1(accept_http1);
        }
        if let Some(concurrency_limit_per_connection) = options.concurrency_limit_per_connection {
            server = server.concurrency_limit_per_connection(concurrency_limit_per_connection as usize);
        }
        if let Some(timeout_ms) = options.timeout_ms {
            server = server.timeout(Duration::from_millis(timeout_ms));
        }
        if let Some(tcp_nodelay) = options.tcp_nodelay {
            server = server.tcp_nodelay(tcp_nodelay);
        }
        Ok(server
            .initial_stream_window_size(options.initial_stream_window_size)
            .initial_connection_window_size(options.initial_connection_window_size)
            .max_concurrent_streams(options.max_concurrent_streams)
            .http2_keepalive_interval(options.http2_keepalive_interval_ms.map(Duration::from_millis))
            .http2_keepalive_timeout(options.http2_keepalive_timeout_ms.map(Duration::from_millis))
            .http2_adaptive_window(options.http2_adaptive_window)
            .tcp_keepalive(options.tcp_keepalive_ms.map(Duration::from_millis))
            .max_frame_size(options.max_frame_size))
    }
}
