pub use crate::gen::mystiko::api::v1::*;

impl ApiResponse {
    pub fn success<T>(data: T) -> Self
    where
        T: prost::Message,
    {
        ApiResponse::builder()
            .code(StatusCode::Success)
            .result(api_response::Result::Data(data.encode_to_vec()))
            .build()
    }

    pub fn success_with_empty() -> Self {
        ApiResponse::builder().code(StatusCode::Success).build()
    }

    pub fn error<T>(code: StatusCode, error_message: T) -> Self
    where
        T: ToString,
    {
        ApiResponse::builder()
            .code(code)
            .result(api_response::Result::ErrorMessage(error_message.to_string()))
            .build()
    }

    pub fn unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(error_message.to_string()))
            .build()
    }
}
