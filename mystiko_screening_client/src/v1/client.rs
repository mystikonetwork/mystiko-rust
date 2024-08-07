use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typed_builder::TypedBuilder;

use crate::{CertificateRequest, CertificateResponse, ScreeningClient};

pub const DEFAULT_SCREENING_V1_URL: &str = "https://screening.mystiko.network";

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ScreeningClientV1 {
    url: String,
    http_client: reqwest::Client,
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ScreeningClientV1Options {
    #[builder(default, setter(strip_option))]
    url: Option<String>,
    http_client: Option<reqwest::Client>,
}

#[derive(Debug, Error)]
pub enum ScreeningClientV1Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("apply certificate meet error: {0} {1}")]
    ApplyCertificateError(i32, String),
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
        O: Into<ScreeningClientV1Options>,
    {
        let options: ScreeningClientV1Options = options.into();
        let url = options.url.unwrap_or(DEFAULT_SCREENING_V1_URL.to_string());
        let http_client = options.http_client.unwrap_or_default();
        Self { url, http_client }
    }
}

#[async_trait]
impl ScreeningClient for ScreeningClientV1 {
    async fn apply_certificate(&self, request: &CertificateRequest) -> Result<CertificateResponse> {
        let url = format!("{}/v1/certificate/apply", self.url);
        println!("url {:?}", url);
        let response = self
            .http_client
            .post(&url)
            .json(request)
            .send()
            .await?
            .error_for_status()?;
        let api_response: ApiResponse<CertificateResponse> = response.json().await.unwrap();
        if api_response.code != 0 {
            return Err(anyhow::anyhow!(ScreeningClientV1Error::ApplyCertificateError(
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
