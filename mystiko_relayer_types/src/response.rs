use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder};
use reqwest::Body;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ResponseCode {
    Successful = 0,
    Failed = -1,
    GetMinimumGasFeeFailed = -2,
    DatabaseError = -3,
    RepeatedTransaction = -4,
    ValidateError = -5,
    UnsupportedTransaction = -6,
    TransactionChannelError = -7,
    TransactionNotFound = -8,
    GetGasPriceError = -9,
    ChainIdNotFound = -10,
    AccountNotFoundInDatabase = -11,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    // error message
    pub message: Option<String>,
}

impl<T> Responder for ApiResponse<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}

impl<T> From<ApiResponse<T>> for Body
where
    T: Serialize,
{
    fn from(value: ApiResponse<T>) -> Self {
        Self::from(serde_json::to_string(&value).unwrap())
    }
}

pub fn success<T>(result: T) -> ApiResponse<T>
where
    T: Serialize,
{
    ApiResponse {
        code: ResponseCode::Successful as i32,
        data: Some(result),
        message: None,
    }
}

pub fn failed<T>(result: Option<T>, error: Option<String>) -> ApiResponse<T>
where
    T: Serialize,
{
    ApiResponse {
        code: ResponseCode::Failed as i32,
        data: result,
        message: error,
    }
}
