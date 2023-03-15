use crate::builder::IndexerClientBuilder;
use crate::errors::ClientError;
use crate::response::ApiResponse;
use crate::types::commitment_queued::{CommitmentQueuedRequest, CommitmentQueuedResponse};
use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::{RequestBuilder, Response};
use serde::Serialize;

pub struct IndexerClient {
    pub base_url: String,
    pub reqwest_client: reqwest::Client,
}

impl IndexerClient {
    pub fn builder(base_url: &str) -> IndexerClientBuilder {
        IndexerClientBuilder::new(base_url)
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T, ClientError>
    where
        T: serde::de::DeserializeOwned + Serialize,
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
                    _ => Err(ClientError::ApiResponseError {
                        code: parsed_resp.code,
                        message: serde_json::to_string(&parsed_resp.result)?,
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

    async fn get_data<T>(&self, url: &str) -> Result<T, ClientError>
    where
        T: serde::de::DeserializeOwned + Serialize,
    {
        let response = self
            .reqwest_client
            .get(url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .send()
            .await?;
        self.handle_response::<T>(response).await
    }

    async fn post_data<T>(&self, request_builder: RequestBuilder) -> Result<T, ClientError>
    where
        T: serde::de::DeserializeOwned + Serialize,
    {
        let response = request_builder.send().await?;
        self.handle_response::<T>(response).await
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

    fn build_with_block_param(
        &self,
        mut request_builder: RequestBuilder,
        start_block: Option<u32>,
        end_block: Option<u32>,
    ) -> RequestBuilder {
        request_builder = match start_block {
            Some(start_block_num) => request_builder.query(&[("startBlock", start_block_num)]),
            None => request_builder,
        };
        request_builder = match end_block {
            Some(end_block_num) => request_builder.query(&[("endBlock", end_block_num)]),
            None => request_builder,
        };
        request_builder
    }

    pub async fn find_commitment_queued_for_chain(
        &self,
        chain_id: u32,
        start_block: Option<u32>,
        end_block: Option<u32>,
        where_filter: Option<CommitmentQueuedRequest>,
    ) -> Result<Vec<CommitmentQueuedResponse>, ClientError> {
        let mut request_builder = self.reqwest_client.post(format!(
            "{}/chains/{}/events/commitment-queued",
            &self.base_url, chain_id
        ));
        request_builder = self.build_with_block_param(request_builder, start_block, end_block);
        request_builder = request_builder.json(&where_filter);
        let response = self
            .post_data::<Vec<CommitmentQueuedResponse>>(request_builder)
            .await?;
        Ok(response)
    }
}
