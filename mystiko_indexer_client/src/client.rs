use crate::builder::IndexerClientBuilder;
use crate::errors::ClientError;
use crate::response::ApiResponse;
use reqwest;

pub struct IndexerClient {
    pub base_url: String,
    pub reqwest_client: reqwest::Client,
}

impl IndexerClient {
    pub fn builder(base_url: String) -> IndexerClientBuilder {
        IndexerClientBuilder::new(base_url)
    }

    pub async fn ping(&self, message: String) -> Result<String, ClientError> {
        let resp = self
            .reqwest_client
            .get(self.base_url.clone() + "/ping")
            .query(&[("message", message)])
            .send()
            .await?;
        let status_code = resp.status().as_u16();
        let handled_resp = match resp.error_for_status() {
            Ok(res) => match res.json::<ApiResponse<String>>().await {
                Ok(resp_body) => match resp_body.code {
                    0 => Ok(resp_body),
                    _ => Err(ClientError::ApiResponseError {
                        code: resp_body.code,
                        message: resp_body.result,
                    }),
                },
                Err(err) => Err(ClientError::ReqwestError(err)),
            },
            Err(err) => Err(ClientError::HttpResponseError {
                code: status_code,
                message: err.to_string(),
            }),
        };

        let resp_body = handled_resp?;
        Ok(resp_body.result)
    }

    pub async fn auth_ping(&self, message: String) -> Result<String, ClientError> {
        let resp = self
            .reqwest_client
            .get(self.base_url.clone() + "/auth-ping")
            .query(&[("message", message)])
            .send()
            .await?;
        let status_code = resp.status().as_u16();
        let handled_resp = match resp.error_for_status() {
            Ok(res) => match res.json::<ApiResponse<String>>().await {
                Ok(resp_body) => match resp_body.code {
                    0 => Ok(resp_body),
                    _ => Err(ClientError::ApiResponseError {
                        code: resp_body.code,
                        message: resp_body.result,
                    }),
                },
                Err(err) => Err(ClientError::ReqwestError(err)),
            },
            Err(err) => Err(ClientError::HttpResponseError {
                code: status_code,
                message: err.to_string(),
            }),
        };

        let resp_body = handled_resp?;
        Ok(resp_body.result)
    }
}
