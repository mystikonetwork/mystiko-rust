use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug, Serialize, Deserialize)]
pub struct ScreeningRequest {
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    pub account: String,
    pub asset: Option<String>,
    pub message: String,
    pub signature: String,
}

#[derive(TypedBuilder, Debug, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct ScreeningResponse {
    pub deadline: u64,
    pub signature: String,
}

#[async_trait]
pub trait ScreeningClient: Send + Sync {
    async fn address_screening(&self, request: &ScreeningRequest) -> Result<ScreeningResponse>;
}

#[async_trait]
impl ScreeningClient for Box<dyn ScreeningClient> {
    async fn address_screening(&self, request: &ScreeningRequest) -> Result<ScreeningResponse> {
        self.as_ref().address_screening(request).await
    }
}
