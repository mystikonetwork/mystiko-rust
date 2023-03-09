use crate::client::MystikoIndexerClient;
use crate::response::ApiResponse;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use reqwest;
use reqwest::{header, Client};
use std::io::Error;
use std::time::Duration;

use crate::config::ClientConfig;

pub struct MystikoIndexerClientV1 {
    pub base_url: String,
    pub reqwest_client: Client,
}

impl MystikoIndexerClientV1 {
    pub fn new(config: &ClientConfig) -> Self {
        let timeout: Duration = match config.timeout {
            Some(x) => Duration::from_secs(x),
            None => Duration::from_secs(20),
        };
        let mut is_auth: Option<String> = None;
        if config.auth_username.is_some() && config.auth_password.is_some() {
            let username = config.auth_username.clone().unwrap();
            let password = config.auth_password.clone().unwrap();
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
                    .timeout(timeout)
                    .build()
                    .unwrap()
            }
            None => Client::builder().timeout(timeout).build().unwrap(),
        };
        MystikoIndexerClientV1 {
            base_url: config.base_url.clone(),
            reqwest_client,
        }
    }
}

#[async_trait]
impl MystikoIndexerClient for MystikoIndexerClientV1 {
    async fn ping(&self, message: String) -> Result<String, Error> {
        let resp = self
            .reqwest_client
            .get(self.base_url.clone() + "/ping")
            .query(&[("message", message)])
            .send()
            .await
            .unwrap()
            .json::<ApiResponse<String>>()
            .await
            .unwrap();
        Ok(resp.result)
    }

    async fn auth_ping(&self, message: String) -> Result<String, Error> {
        let resp = self
            .reqwest_client
            .get(self.base_url.clone() + "/auth-ping")
            .query(&[("message", message)])
            .send()
            .await
            .unwrap()
            .json::<ApiResponse<String>>()
            .await
            .unwrap();
        Ok(resp.result)
    }
}
