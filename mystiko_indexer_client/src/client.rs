use crate::builder::IndexerClientBuilder;
use crate::errors::ClientError;
use crate::response::ApiResponse;
use crate::types::{
    commitment::{CommitmentResponse, CommitmentsForContractRequest},
    commitment_included::{
        CommitmentIncludedForChainRequest, CommitmentIncludedForContractRequest, CommitmentIncludedResponse,
    },
    commitment_queued::{
        CommitmentQueuedForChainRequest, CommitmentQueuedForContractRequest, CommitmentQueuedResponse,
    },
    commitment_spent::{CommitmentSpentForChainRequest, CommitmentSpentForContractRequest, CommitmentSpentResponse},
    sync_response::{ChainSyncRepsonse, ContractSyncResponse},
};
use anyhow::{anyhow, Result};
use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::{RequestBuilder, Response};
use serde::Serialize;
use std::collections::HashMap;

pub struct IndexerClient {
    pub base_url: String,
    pub reqwest_client: reqwest::Client,
}

impl IndexerClient {
    pub fn builder(base_url: &str) -> IndexerClientBuilder {
        IndexerClientBuilder::new(base_url)
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T>
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
                    _ => Err(anyhow!(ClientError::ApiResponseError {
                        code: parsed_resp.code,
                        message: serde_json::to_string(&parsed_resp.result)?,
                    })),
                };
                let res_body = handled_resp?;
                return Ok(res_body.result);
            }
        }
        Err(anyhow!(ClientError::UnsupportedContentTypeError(
            content_type.unwrap_or("").to_string(),
        )))
    }

    async fn get_data<T>(&self, url: &str) -> Result<T>
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

    async fn post_data<T>(&self, request_builder: RequestBuilder) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Serialize,
    {
        let response = request_builder.send().await?;
        self.handle_response::<T>(response).await
    }

    fn build_block_params_map(
        &self,
        mut params_map: HashMap<String, String>,
        start_block: &Option<u32>,
        end_block: &Option<u32>,
    ) -> HashMap<String, String> {
        if let Some(start_block_num) = start_block {
            params_map.insert(String::from("startBlock"), start_block_num.to_string());
        }
        if let Some(end_block_num) = end_block {
            params_map.insert(String::from("endBlock"), end_block_num.to_string());
        }
        params_map
    }

    fn build_request_builder<T>(
        &self,
        mut request_builder: RequestBuilder,
        params: HashMap<String, String>,
        body: &T,
    ) -> RequestBuilder
    where
        T: Serialize,
    {
        for (key, value) in params.iter() {
            request_builder = request_builder.query(&[(key, value)]);
        }
        request_builder = request_builder.json(body);
        request_builder
    }

    pub async fn ping(&self, message: &str) -> Result<String> {
        let resp = self
            .get_data::<String>(&format!("{}{}{}", &self.base_url, "/ping?message=", message))
            .await?;
        Ok(resp)
    }

    pub async fn auth_ping(&self, message: &str) -> Result<String> {
        let resp = self
            .get_data::<String>(&format!("{}{}{}", &self.base_url, "/auth-ping?message=", message))
            .await?;
        Ok(resp)
    }

    pub async fn query_chain_sync_repsonse_by_id(&self, chain_id: u32) -> Result<ChainSyncRepsonse> {
        let resp = self
            .get_data::<ChainSyncRepsonse>(&format!("{}/chains/{}/block-number", &self.base_url, chain_id))
            .await?;
        Ok(resp)
    }

    pub async fn query_contract_sync_response(
        &self,
        chain_id: u32,
        contract_address: &str,
    ) -> Result<ContractSyncResponse> {
        let resp = self
            .get_data::<ContractSyncResponse>(&format!(
                "{}/chains/{}/contracts/{}/block-number",
                &self.base_url, chain_id, contract_address
            ))
            .await?;
        Ok(resp)
    }

    pub async fn find_commitment_queued_for_chain(
        &self,
        request: &CommitmentQueuedForChainRequest,
    ) -> Result<Vec<CommitmentQueuedResponse>> {
        let mut request_builder = self.reqwest_client.post(format!(
            "{}/chains/{}/events/commitment-queued",
            &self.base_url, &request.chain_id
        ));
        let params_map: HashMap<String, String> = HashMap::new();
        let params_map = self.build_block_params_map(params_map, &request.start_block, &request.end_block);
        request_builder = self.build_request_builder(request_builder, params_map, &request.where_filter);
        let response = self.post_data::<Vec<CommitmentQueuedResponse>>(request_builder).await?;
        Ok(response)
    }

    pub async fn find_commitment_queued_for_contract(
        &self,
        request: &CommitmentQueuedForContractRequest,
    ) -> Result<Vec<CommitmentQueuedResponse>> {
        let mut request_builder = self.reqwest_client.post(format!(
            "{}/chains/{}/contracts/{}/events/commitment-queued",
            &self.base_url, &request.chain_id, &request.address
        ));
        let params_map: HashMap<String, String> = HashMap::new();
        let params_map = self.build_block_params_map(params_map, &request.start_block, &request.end_block);
        request_builder = self.build_request_builder(request_builder, params_map, &request.where_filter);
        let response = self.post_data::<Vec<CommitmentQueuedResponse>>(request_builder).await?;
        Ok(response)
    }

    pub async fn find_commitment_included_for_chain(
        &self,
        request: &CommitmentIncludedForChainRequest,
    ) -> Result<Vec<CommitmentIncludedResponse>> {
        let mut request_builder = self.reqwest_client.post(format!(
            "{}/chains/{}/events/commitment-included",
            &self.base_url, &request.chain_id
        ));
        let params_map: HashMap<String, String> = HashMap::new();
        let params_map = self.build_block_params_map(params_map, &request.start_block, &request.end_block);
        request_builder = self.build_request_builder(request_builder, params_map, &request.where_filter);
        let response = self
            .post_data::<Vec<CommitmentIncludedResponse>>(request_builder)
            .await?;
        Ok(response)
    }

    pub async fn find_commitment_included_for_contract(
        &self,
        request: &CommitmentIncludedForContractRequest,
    ) -> Result<Vec<CommitmentIncludedResponse>> {
        let mut request_builder = self.reqwest_client.post(format!(
            "{}/chains/{}/contracts/{}/events/commitment-included",
            &self.base_url, &request.chain_id, &request.address
        ));
        let params_map: HashMap<String, String> = HashMap::new();
        let params_map = self.build_block_params_map(params_map, &request.start_block, &request.end_block);
        request_builder = self.build_request_builder(request_builder, params_map, &request.where_filter);
        let response = self
            .post_data::<Vec<CommitmentIncludedResponse>>(request_builder)
            .await?;
        Ok(response)
    }
    pub async fn find_commitment_spent_for_chain(
        &self,
        request: &CommitmentSpentForChainRequest,
    ) -> Result<Vec<CommitmentSpentResponse>> {
        let mut request_builder = self.reqwest_client.post(format!(
            "{}/chains/{}/events/commitment-spent",
            &self.base_url, &request.chain_id
        ));
        let params_map: HashMap<String, String> = HashMap::new();
        let params_map = self.build_block_params_map(params_map, &request.start_block, &request.end_block);
        request_builder = self.build_request_builder(request_builder, params_map, &request.where_filter);
        let response = self.post_data::<Vec<CommitmentSpentResponse>>(request_builder).await?;
        Ok(response)
    }
    pub async fn find_commitment_spent_for_contract(
        &self,
        request: &CommitmentSpentForContractRequest,
    ) -> Result<Vec<CommitmentSpentResponse>> {
        let mut request_builder = self.reqwest_client.post(format!(
            "{}/chains/{}/contracts/{}/events/commitment-spent",
            &self.base_url, &request.chain_id, &request.address
        ));
        let params_map: HashMap<String, String> = HashMap::new();
        let params_map = self.build_block_params_map(params_map, &request.start_block, &request.end_block);
        request_builder = self.build_request_builder(request_builder, params_map, &request.where_filter);
        let response = self.post_data::<Vec<CommitmentSpentResponse>>(request_builder).await?;
        Ok(response)
    }

    pub async fn find_commitments_for_contract(
        &self,
        request: &CommitmentsForContractRequest,
    ) -> Result<Vec<CommitmentResponse>> {
        let mut request_builder = self.reqwest_client.post(format!(
            "{}/chains/{}/contracts/{}/commitments",
            &self.base_url, &request.chain_id, &request.contract_address
        ));
        let params_map: HashMap<String, String> = HashMap::new();
        let params_map = self.build_block_params_map(params_map, &request.start_block, &request.end_block);
        request_builder = self.build_request_builder(request_builder, params_map, &request.where_filter);
        let response = self.post_data::<Vec<CommitmentResponse>>(request_builder).await?;
        Ok(response)
    }

    pub async fn count_commitment_included_for_contract(
        &self,
        chain_id: u32,
        contract_address: String,
        end_block: u32,
    ) -> Result<u32> {
        let resp = self
            .get_data::<u32>(&format!(
                "{}/chains/{}/contracts/{}/count/commitment-included?endBlock={}",
                &self.base_url, chain_id, contract_address, end_block
            ))
            .await?;
        Ok(resp)
    }
}
