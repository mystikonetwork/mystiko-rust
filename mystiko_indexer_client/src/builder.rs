use crate::client::IndexerClient;
use base64::{engine::general_purpose, Engine as _};
use reqwest::{header, Client};
use std::time::Duration;

pub struct IndexerClientBuilder {
    pub base_url: String,
    pub auth_username: Option<String>,
    pub auth_password: Option<String>,
    pub timeout: Duration,
}

impl IndexerClientBuilder {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            auth_username: None,
            auth_password: None,
            timeout: Duration::from_secs(20),
        }
    }

    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn auth_username(mut self, auth_username: Option<String>) -> Self {
        if let Some(auth_username) = auth_username {
            self.auth_username = Some(auth_username);
        };
        self
    }

    pub fn auth_password(mut self, auth_password: Option<String>) -> Self {
        if let Some(auth_password) = auth_password {
            self.auth_password = Some(auth_password);
        }
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> IndexerClient {
        let IndexerClientBuilder {
            base_url,
            auth_username: _,
            auth_password: _,
            timeout: _,
        } = self;
        let mut is_auth: Option<String> = None;
        if let (Some(auth_username), Some(auth_password)) =
            (&self.auth_username, &self.auth_password)
        {
            let username = auth_username.clone();
            let password = auth_password.clone();
            let auth_str = username + ":" + &password;
            let encoded_str = general_purpose::STANDARD.encode(auth_str);
            is_auth = Some("Basic ".to_owned() + &encoded_str);
        }
        let reqwest_client = match is_auth {
            Some(s) => {
                let mut headers = header::HeaderMap::new();
                headers.insert("Authorization", header::HeaderValue::from_str(&s).unwrap());
                Client::builder()
                    .default_headers(headers)
                    .timeout(self.timeout)
                    .build()
                    .unwrap()
            }
            None => Client::builder().timeout(self.timeout).build().unwrap(),
        };
        IndexerClient {
            base_url,
            reqwest_client,
        }
    }
}
