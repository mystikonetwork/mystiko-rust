use crate::service::v1::ClientOptions;
use mystiko_config::SequencerConfig;
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

    pub fn get_host(&self) -> String {
        self.host.clone().unwrap_or(String::from(DEFAULT_HOST))
    }

    pub fn to_uri(&self) -> Result<http::Uri, http::uri::InvalidUri> {
        self.format_uri(self.get_host())
    }

    fn format_uri(&self, host: String) -> Result<http::Uri, http::uri::InvalidUri> {
        let port = self.port.map(|port| format!(":{}", port)).unwrap_or_default();
        http::Uri::from_str(&format!("{}://{}{}", self.scheme(), host, port))
    }
}

impl From<&SequencerConfig> for ClientOptions {
    fn from(config: &SequencerConfig) -> Self {
        Self::builder()
            .host(config.host().to_string())
            .port(config.port().map(u32::from))
            .is_ssl(config.is_ssl())
            .build()
    }
}

#[cfg(feature = "validate")]
impl validator::Validate for ClientOptions {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let _ = self.to_uri().map_err(|_| {
            let mut errors = validator::ValidationErrors::new();
            errors.add("host", validator::ValidationError::new("invalid uri"));
            errors
        })?;
        Ok(())
    }
}
