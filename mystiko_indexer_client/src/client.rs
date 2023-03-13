use crate::builder::IndexerClientBuilder;
use crate::errors::ClientError;
use crate::response::ApiResponse;
use reqwest::header::{HeaderValue, ACCEPT};
use serde;

pub struct IndexerClient {
    pub base_url: String,
    pub reqwest_client: reqwest::Client,
}

impl IndexerClient {
    pub fn builder(base_url: &str) -> IndexerClientBuilder {
        IndexerClientBuilder::new(base_url)
    }

    async fn get_data<T>(&self, url: &str) -> Result<T, ClientError>
    where
        T: serde::de::DeserializeOwned + ToString,
    {
        let response = self
            .reqwest_client
            .get(url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok());
        if let Some(s) = content_type {
            if s.contains("application/json") {
                let parsed_resp = response.json::<ApiResponse<T>>().await?;
                let handled_resp = match parsed_resp.code {
                    0 => Ok(parsed_resp),
                    _ => Err(ClientError::ApiResponseError {
                        code: parsed_resp.code,
                        message: parsed_resp.result.to_string(),
                    }),
                };
                let res_body = handled_resp?;
                return Ok(res_body.result);
            }
        }
        Err(ClientError::UnsupportedContentTypeError(
            content_type.unwrap_or("").to_string(),
        ))
    }

    pub async fn ping(&self, message: &str) -> Result<String, ClientError> {
        let resp = self
            .get_data::<String>(&format!(
                "{}{}{}",
                &self.base_url, "/ping?message=", message
            ))
            .await?;
        Ok(resp)
    }

    pub async fn auth_ping(&self, message: &str) -> Result<String, ClientError> {
        let resp = self
            .get_data::<String>(&format!(
                "{}{}{}",
                &self.base_url, "/auth-ping?message=", message
            ))
            .await?;
        Ok(resp)
    }
}
