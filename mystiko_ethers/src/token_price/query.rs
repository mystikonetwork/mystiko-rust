use crate::token_price::error::TokenPriceError;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Status {
    error_code: u64,
    error_message: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CurrencyMap {
    id: u32,
    symbol: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CurrencyMapResponse {
    data: Vec<CurrencyMap>,
    status: Status,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct CurrencyQuoteData {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub quote: Quote,
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Quote {
    pub USD: CurrencyUsdPrice,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct CurrencyUsdPrice {
    pub price: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CurrencyQuoteResponse {
    data: HashMap<String, CurrencyQuoteData>,
    status: Status,
}

pub struct QueryApiInstance {
    base_url: String,
    client: Client,
}

impl QueryApiInstance {
    pub fn new(api_key: &str, base_url: String, timeout_secs: u32) -> Result<QueryApiInstance, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_str(api_key).unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(timeout_secs as u64))
            .build()?;

        Ok(QueryApiInstance { client, base_url })
    }

    pub async fn get_token_id(&self, symbol: &str) -> Result<Vec<u32>, TokenPriceError> {
        let mut map = HashMap::new();
        map.insert("symbol", symbol);
        let response = self
            .client
            .get(&format!("{}{}", self.base_url, "/v1/cryptocurrency/map"))
            .query(&map)
            .send()
            .await?
            .json::<CurrencyMapResponse>()
            .await?;

        if response.status.error_code != 0 {
            return Err(TokenPriceError::ResponseError(response.status.error_code));
        }

        Ok(response.data.iter().map(|d| d.id).collect())
    }

    pub async fn get_latest_price(&self, ids: &[u32]) -> Result<HashMap<u32, f64>, TokenPriceError> {
        let ids_str = ids.iter().map(|&id| id.to_string()).collect::<Vec<_>>().join(",");
        let response = self
            .client
            .get(&format!("{}{}", self.base_url, "/v2/cryptocurrency/quotes/latest"))
            .query(&[("id", ids_str)])
            .send()
            .await?
            .json::<CurrencyQuoteResponse>()
            .await?;

        if response.status.error_code != 0 {
            return Err(TokenPriceError::ResponseError(response.status.error_code));
        }

        let mut prices = HashMap::new();
        for (_, price) in response.data {
            prices.insert(price.id, price.quote.USD.price);
        }

        Ok(prices)
    }
}
