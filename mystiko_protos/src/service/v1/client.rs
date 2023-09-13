use crate::service::v1::ClientOptions;
use std::str::FromStr;

pub const DEFAULT_HOST: &str = "127.0.0.1";

impl ClientOptions {
    pub fn scheme(&self) -> &str {
        if self.is_ssl() {
            "https"
        } else {
            "http"
        }
    }

    pub fn to_uri(&self) -> Result<http::Uri, http::uri::InvalidUri> {
        self.format_uri(self.host.clone())
    }

    pub fn to_ssl_uri(&self) -> Result<http::Uri, http::uri::InvalidUri> {
        if self.is_ssl() {
            self.format_uri(self.ssl_server_name.clone().or(self.host.clone()))
        } else {
            self.to_uri()
        }
    }

    fn format_uri(&self, host: Option<String>) -> Result<http::Uri, http::uri::InvalidUri> {
        let host = host.unwrap_or(String::from(DEFAULT_HOST));
        let port = self.port.map(|port| format!(":{}", port)).unwrap_or_default();
        http::Uri::from_str(&format!("{}://{}{}", self.scheme(), host, port))
    }
}
