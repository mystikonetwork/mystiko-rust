use crate::api::v1::status_code::Error;
pub use crate::gen::mystiko::api::v1::*;

impl ApiResponse {
    pub fn success<T>(data: T) -> Self
    where
        T: prost::Message,
    {
        ApiResponse::builder()
            .code(StatusCode::builder().success(true).build())
            .result(api_response::Result::Data(data.encode_to_vec()))
            .build()
    }

    pub fn success_with_empty() -> Self {
        ApiResponse::builder()
            .code(StatusCode::builder().success(true).build())
            .build()
    }

    pub fn error<T, E>(code: E, error_message: T) -> Self
    where
        E: Into<StatusCode>,
        T: ToString,
    {
        ApiResponse::builder()
            .code(code.into())
            .result(api_response::Result::ErrorMessage(error_message.to_string()))
            .build()
    }

    pub fn unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        ApiResponse::builder()
            .code(StatusCode::builder().success(false).build())
            .result(api_response::Result::ErrorMessage(error_message.to_string()))
            .build()
    }
}

impl From<MystikoError> for StatusCode {
    fn from(value: MystikoError) -> Self {
        StatusCode::builder().error(Error::Mystiko(value as i32)).build()
    }
}

impl From<ConfigError> for StatusCode {
    fn from(value: ConfigError) -> Self {
        StatusCode::builder().error(Error::Config(value as i32)).build()
    }
}

impl From<AccountError> for StatusCode {
    fn from(value: AccountError) -> Self {
        StatusCode::builder().error(Error::Account(value as i32)).build()
    }
}

impl From<WalletError> for StatusCode {
    fn from(value: WalletError) -> Self {
        StatusCode::builder().error(Error::Wallet(value as i32)).build()
    }
}

impl From<ScannerError> for StatusCode {
    fn from(value: ScannerError) -> Self {
        StatusCode::builder().error(Error::Scanner(value as i32)).build()
    }
}

impl From<SynchronizerError> for StatusCode {
    fn from(value: SynchronizerError) -> Self {
        StatusCode::builder().error(Error::Synchronizer(value as i32)).build()
    }
}

impl From<DepositError> for StatusCode {
    fn from(value: DepositError) -> Self {
        StatusCode::builder().error(Error::Deposit(value as i32)).build()
    }
}
