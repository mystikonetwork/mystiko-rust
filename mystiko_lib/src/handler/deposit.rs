use crate::runtime;
use mystiko_protos::api::handler::v1::{CreateDepositRequest, QuoteRequest, SummaryRequest};
use mystiko_protos::api::v1::{ApiResponse, DepositError};

pub fn quote<M>(message: M) -> ApiResponse
where
    M: TryInto<QuoteRequest>,
    <M as TryInto<QuoteRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::quote(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn summary<M>(message: M) -> ApiResponse
where
    M: TryInto<SummaryRequest>,
    <M as TryInto<SummaryRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::summary(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn create<M>(message: M) -> ApiResponse
where
    M: TryInto<CreateDepositRequest>,
    <M as TryInto<CreateDepositRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::create(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

mod internal {
    use crate::error::parse_deposit_error;
    use crate::instance;
    use mystiko_core::DepositHandler;
    use mystiko_protos::api::handler::v1::{CreateDepositResponse, QuoteResponse, SummaryResponse};
    use mystiko_protos::api::v1::{ApiResponse, DepositError};
    use mystiko_protos::core::handler::v1::{CreateDepositOptions, QuoteDepositOptions};

    pub(crate) async fn quote(options: QuoteDepositOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.quote(options).await;
                match result {
                    Ok(quote) => ApiResponse::success(QuoteResponse::builder().quote(quote).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn summary(options: CreateDepositOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.summary(options).await;
                match result {
                    Ok(summary) => ApiResponse::success(SummaryResponse::builder().summary(summary).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn create(options: CreateDepositOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.create(options).await;
                match result {
                    Ok(deposit) => ApiResponse::success(CreateDepositResponse::builder().deposit(deposit).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }
}
