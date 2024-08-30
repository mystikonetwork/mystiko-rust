use crate::{ScreeningClient, ScreeningRequest, ScreeningResponse};
use anyhow::Result;
use async_trait::async_trait;
use mystiko_protos::screening::v1::ScreeningClientOptions;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typed_builder::TypedBuilder;

pub const DEFAULT_SCREENING_V1_URL: &str = "https://screening.mystiko.network";

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ScreeningClientV1 {
    url: String,
    http_client: reqwest::Client,
}

#[derive(Debug, Error)]
pub enum ScreeningClientV1Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("address screening meet error: {0} {1}")]
    AddressScreeningError(i32, String),
    #[error("empty response error")]
    EmptyResponseError,
}

#[derive(Serialize, Deserialize, Debug, TypedBuilder)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    pub message: Option<String>,
    pub version: String,
}

impl ScreeningClientV1 {
    pub fn new<O>(options: O) -> Self
    where
        O: Into<ScreeningClientOptions>,
    {
        let options: ScreeningClientOptions = options.into();
        let url = options
            .screening_config_api_url
            .unwrap_or(DEFAULT_SCREENING_V1_URL.to_string());
        let http_client = reqwest::Client::new();
        Self { url, http_client }
    }
}

#[async_trait]
impl ScreeningClient for ScreeningClientV1 {
    async fn address_screening(&self, request: &ScreeningRequest) -> Result<ScreeningResponse> {
        let url = format!("{}/v1/screening", self.url);
        let response = self
            .http_client
            .post(&url)
            .json(request)
            .send()
            .await?
            .error_for_status()?;
        let api_response: ApiResponse<ScreeningResponse> = response.json().await.unwrap();
        if api_response.code != 0 {
            return Err(anyhow::anyhow!(ScreeningClientV1Error::AddressScreeningError(
                api_response.code,
                api_response.message.unwrap_or_default()
            )));
        }

        if let Some(data) = api_response.data {
            Ok(data)
        } else {
            Err(anyhow::anyhow!(ScreeningClientV1Error::EmptyResponseError))
        }
    }
}
