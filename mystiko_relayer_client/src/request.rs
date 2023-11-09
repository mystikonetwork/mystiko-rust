use crate::error::RelayerClientError;
use crate::types::result::Result;
use mystiko_relayer_types::response::ApiResponse;
use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn handle_response<T>(response: Response) -> Result<T>
where
    T: DeserializeOwned + Serialize,
{
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
                _ => Err(RelayerClientError::ApiResponseError {
                    code: parsed_resp.code,
                    message: parsed_resp.message.unwrap_or_default(),
                }),
            };
            let res_body = handled_resp?;
            return Ok(res_body.data.unwrap());
        }
    }
    Err(RelayerClientError::UnsupportedContentTypeError(
        content_type.unwrap_or("").to_string(),
    ))
}

pub async fn get_data<T>(reqwest_client: Arc<Client>, url: &str) -> Result<T>
where
    T: DeserializeOwned + Serialize,
{
    let response = reqwest_client
        .get(url)
        .header(ACCEPT, HeaderValue::from_static("application/json"))
        .send()
        .await?;
    handle_response::<T>(response).await
}

pub async fn post_data<T>(request_builder: RequestBuilder) -> Result<T>
where
    T: DeserializeOwned + Serialize,
{
    let response = request_builder.send().await?;
    handle_response::<T>(response).await
}

pub fn build_request_builder<T>(
    mut request_builder: RequestBuilder,
    params: Option<HashMap<String, String>>,
    body: &T,
) -> RequestBuilder
where
    T: Serialize,
{
    match params {
        None => {}
        Some(params) => {
            for (key, value) in params.iter() {
                request_builder = request_builder.query(&[(key, value)]);
            }
        }
    }
    request_builder = request_builder.json(body);
    request_builder
}
