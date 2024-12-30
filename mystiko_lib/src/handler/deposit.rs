use crate::runtime;
use mystiko_protos::api::handler::v1::{
    CountDepositRequest, CreateDepositRequest, DeleteDepositBatchRequest, DeleteDepositByFilterRequest,
    DeleteDepositRequest, FindDepositByIdRequest, FindDepositRequest, FixDepositStatusRequest, QuoteRequest,
    SendRequest, SendWithGrpcRequest, SummaryRequest, UpdateAllDepositRequest, UpdateDepositBatchRequest,
    UpdateDepositByFilterRequest, UpdateDepositRequest,
};
use mystiko_protos::api::v1::{ApiResponse, DepositError};
use mystiko_protos::storage::v1::ColumnValue;
use mystiko_storage::ColumnValues;

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

pub fn send<M>(message: M) -> ApiResponse
where
    M: TryInto<SendRequest>,
    <M as TryInto<SendRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::send(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn fix_status<M>(message: M) -> ApiResponse
where
    M: TryInto<FixDepositStatusRequest>,
    <M as TryInto<FixDepositStatusRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::fix_status(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn send_with_grpc<M>(message: M) -> ApiResponse
where
    M: TryInto<SendWithGrpcRequest>,
    <M as TryInto<SendWithGrpcRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if message.send_options.is_some() && message.client_options.is_some() {
                return runtime().block_on(internal::send_with_grpc(
                    message.send_options.unwrap(),
                    message.client_options.unwrap(),
                ));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn find<M>(message: M) -> ApiResponse
where
    M: TryInto<FindDepositRequest>,
    <M as TryInto<FindDepositRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                return runtime().block_on(internal::find(filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn find_all() -> ApiResponse {
    runtime().block_on(internal::find_all())
}

pub fn find_one<M>(message: M) -> ApiResponse
where
    M: TryInto<FindDepositRequest>,
    <M as TryInto<FindDepositRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                return runtime().block_on(internal::find_one(filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn find_by_id<M>(message: M) -> ApiResponse
where
    M: TryInto<FindDepositByIdRequest>,
    <M as TryInto<FindDepositByIdRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::find_by_id(message.id)),
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn count<M>(message: M) -> ApiResponse
where
    M: TryInto<CountDepositRequest>,
    <M as TryInto<CountDepositRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                return runtime().block_on(internal::count(filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn count_all() -> ApiResponse {
    runtime().block_on(internal::count_all())
}

pub fn update<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateDepositRequest>,
    <M as TryInto<UpdateDepositRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(deposit) = message.deposit {
                return runtime().block_on(internal::update(deposit));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn update_batch<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateDepositBatchRequest>,
    <M as TryInto<UpdateDepositBatchRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::update_batch(message.deposits)),
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn update_by_filter<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateDepositByFilterRequest>,
    <M as TryInto<UpdateDepositByFilterRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                let column_values = ColumnValues::builder()
                    .column_values(
                        message
                            .column_values
                            .into_iter()
                            .map(|pair| (pair.column, pair.value))
                            .collect::<Vec<(String, Option<ColumnValue>)>>(),
                    )
                    .build();
                return runtime().block_on(internal::update_by_filter(column_values, filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn update_all<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateAllDepositRequest>,
    <M as TryInto<UpdateAllDepositRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            let column_values = ColumnValues::builder()
                .column_values(
                    message
                        .column_values
                        .into_iter()
                        .map(|pair| (pair.column, pair.value))
                        .collect::<Vec<(String, Option<ColumnValue>)>>(),
                )
                .build();
            runtime().block_on(internal::update_all(column_values))
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn delete<M>(message: M) -> ApiResponse
where
    M: TryInto<DeleteDepositRequest>,
    <M as TryInto<DeleteDepositRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(deposit) = message.deposit {
                return runtime().block_on(internal::delete(deposit));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn delete_batch<M>(message: M) -> ApiResponse
where
    M: TryInto<DeleteDepositBatchRequest>,
    <M as TryInto<DeleteDepositBatchRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::delete_batch(message.deposits)),
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn delete_by_filter<M>(message: M) -> ApiResponse
where
    M: TryInto<DeleteDepositByFilterRequest>,
    <M as TryInto<DeleteDepositByFilterRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                return runtime().block_on(internal::delete_by_filter(filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(DepositError::DeserializeMessageError, err),
    }
}

pub fn delete_all() -> ApiResponse {
    runtime().block_on(internal::delete_all())
}

mod internal {
    use super::*;
    use crate::error::parse_deposit_error;
    use crate::instance;
    use mystiko_core::{DepositHandler, GrpcSigner};
    use mystiko_protos::api::handler::v1::{
        CountDepositResponse, CreateDepositResponse, FindDepositResponse, FindOneDepositResponse,
        FixDepositStatusResponse, QuoteResponse, SendResponse, SummaryResponse, UpdateDepositBatchResponse,
        UpdateDepositResponse,
    };
    use mystiko_protos::core::document::v1::Deposit;
    use mystiko_protos::core::handler::v1::{
        CreateDepositOptions, FixDepositStatusOptions, QuoteDepositOptions, SendDepositOptions,
    };
    use mystiko_protos::service::v1::ClientOptions;
    use mystiko_protos::storage::v1::QueryFilter;
    use mystiko_storage::ColumnValues;
    use std::sync::Arc;

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

    pub(crate) async fn send(options: SendDepositOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.send(options).await;
                match result {
                    Ok(deposit) => ApiResponse::success(SendResponse::builder().deposit(deposit).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn send_with_grpc(send_options: SendDepositOptions, client_options: ClientOptions) -> ApiResponse {
        // grpc client to call dart grpc server
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => match GrpcSigner::connect(&client_options).await {
                Ok(signer) => match mystiko.deposits.send_with_signer(send_options, Arc::new(signer)).await {
                    Ok(deposit) => ApiResponse::success(SendResponse::builder().deposit(deposit).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                },
                Err(err) => ApiResponse::error(DepositError::GrpcConnectError, err),
            },
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }
    pub(crate) async fn fix_status(options: FixDepositStatusOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.fix_status(options).await;
                match result {
                    Ok(deposit) => ApiResponse::success(FixDepositStatusResponse::builder().deposit(deposit).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find(filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.find(filter).await;
                match result {
                    Ok(deposits) => ApiResponse::success(FindDepositResponse::builder().deposits(deposits).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_all() -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.find_all().await;
                match result {
                    Ok(deposits) => ApiResponse::success(FindDepositResponse::builder().deposits(deposits).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_one(filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.find_one(filter).await;
                match result {
                    Ok(deposit) => ApiResponse::success(FindOneDepositResponse::builder().deposit(deposit).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_by_id(id: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.find_by_id(id).await;
                match result {
                    Ok(deposit) => ApiResponse::success(FindOneDepositResponse::builder().deposit(deposit).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn count(filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.count(filter).await;
                match result {
                    Ok(count) => ApiResponse::success(CountDepositResponse::builder().count(count).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn count_all() -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.count_all().await;
                match result {
                    Ok(count) => ApiResponse::success(CountDepositResponse::builder().count(count).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update(deposit: Deposit) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.update(deposit).await;
                match result {
                    Ok(deposit) => ApiResponse::success(UpdateDepositResponse::builder().deposit(deposit).build()),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_batch(deposits: Vec<Deposit>) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.update_batch(deposits).await;
                match result {
                    Ok(deposits) => {
                        ApiResponse::success(UpdateDepositBatchResponse::builder().deposits(deposits).build())
                    }
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_by_filter(column_values: ColumnValues, filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.update_by_filter(column_values, filter).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_all(column_values: ColumnValues) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.update_all(column_values).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn delete(deposit: Deposit) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.delete(deposit).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn delete_batch(deposits: Vec<Deposit>) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.delete_batch(deposits).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn delete_by_filter(filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.delete_by_filter(filter).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn delete_all() -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.deposits.delete_all().await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_deposit_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(DepositError::GetMystikoGuardError, err),
        }
    }
}
