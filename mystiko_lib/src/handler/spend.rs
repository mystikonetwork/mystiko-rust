use crate::runtime;
use mystiko_protos::api::handler::v1::{
    CountSpendsRequest, CreateSpendRequest, DeleteSpendBatchRequest, DeleteSpendByFilterRequest, DeleteSpendRequest,
    FindSpendByIdRequest, FindSpendRequest, FixSpendStatusRequest, SendSpendRequest, SendSpendWithGrpcRequest,
    SpendQuoteRequest, SpendSummaryRequest, UpdateAllSpendRequest, UpdateSpendBatchRequest, UpdateSpendByFilterRequest,
    UpdateSpendRequest,
};
use mystiko_protos::api::v1::{ApiResponse, SpendError};
use mystiko_protos::storage::v1::ColumnValue;
use mystiko_storage::ColumnValues;

pub fn quote<M>(message: M) -> ApiResponse
where
    M: TryInto<SpendQuoteRequest>,
    <M as TryInto<SpendQuoteRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::quote(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn summary<M>(message: M) -> ApiResponse
where
    M: TryInto<SpendSummaryRequest>,
    <M as TryInto<SpendSummaryRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::summary(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn create<M>(message: M) -> ApiResponse
where
    M: TryInto<CreateSpendRequest>,
    <M as TryInto<CreateSpendRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::create(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn send<M>(message: M) -> ApiResponse
where
    M: TryInto<SendSpendRequest>,
    <M as TryInto<SendSpendRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::send(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn send_with_grpc<M>(message: M) -> ApiResponse
where
    M: TryInto<SendSpendWithGrpcRequest>,
    <M as TryInto<SendSpendWithGrpcRequest>>::Error: std::error::Error + Send + Sync + 'static,
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
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn fix_status<M>(message: M) -> ApiResponse
where
    M: TryInto<FixSpendStatusRequest>,
    <M as TryInto<FixSpendStatusRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::fix_status(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn find<M>(message: M) -> ApiResponse
where
    M: TryInto<FindSpendRequest>,
    <M as TryInto<FindSpendRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                return runtime().block_on(internal::find(filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn find_all() -> ApiResponse {
    runtime().block_on(internal::find_all())
}

pub fn find_one<M>(message: M) -> ApiResponse
where
    M: TryInto<FindSpendRequest>,
    <M as TryInto<FindSpendRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                return runtime().block_on(internal::find_one(filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn find_by_id<M>(message: M) -> ApiResponse
where
    M: TryInto<FindSpendByIdRequest>,
    <M as TryInto<FindSpendByIdRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::find_by_id(message.id)),
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn count<M>(message: M) -> ApiResponse
where
    M: TryInto<CountSpendsRequest>,
    <M as TryInto<CountSpendsRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                return runtime().block_on(internal::count(filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn count_all() -> ApiResponse {
    runtime().block_on(internal::count_all())
}

pub fn update<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateSpendRequest>,
    <M as TryInto<UpdateSpendRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(spend) = message.spend {
                return runtime().block_on(internal::update(spend));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn update_batch<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateSpendBatchRequest>,
    <M as TryInto<UpdateSpendBatchRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::update_batch(message.spends)),
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn update_by_filter<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateSpendByFilterRequest>,
    <M as TryInto<UpdateSpendByFilterRequest>>::Error: std::error::Error + Send + Sync + 'static,
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
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn update_all<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateAllSpendRequest>,
    <M as TryInto<UpdateAllSpendRequest>>::Error: std::error::Error + Send + Sync + 'static,
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
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn delete<M>(message: M) -> ApiResponse
where
    M: TryInto<DeleteSpendRequest>,
    <M as TryInto<DeleteSpendRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(spend) = message.spend {
                return runtime().block_on(internal::delete(spend));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn delete_batch<M>(message: M) -> ApiResponse
where
    M: TryInto<DeleteSpendBatchRequest>,
    <M as TryInto<DeleteSpendBatchRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::delete_batch(message.spends)),
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn delete_by_filter<M>(message: M) -> ApiResponse
where
    M: TryInto<DeleteSpendByFilterRequest>,
    <M as TryInto<DeleteSpendByFilterRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                return runtime().block_on(internal::delete_by_filter(filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SpendError::DeserializeMessageError, err),
    }
}

pub fn delete_all() -> ApiResponse {
    runtime().block_on(internal::delete_all())
}

mod internal {
    use super::*;
    use crate::error::parse_spends_error;
    use crate::instance;
    use mystiko_core::{GrpcSigner, SpendHandler};
    use mystiko_protos::api::handler::v1::{
        CountSpendsResponse, CreateSpendResponse, FindOneSpendResponse, FindSpendsResponse, FixSpendStatusResponse,
        SendSpendResponse, SpendQuoteResponse, SpendSummaryResponse, UpdateSpendBatchResponse, UpdateSpendResponse,
    };
    use mystiko_protos::core::document::v1::Spend;
    use mystiko_protos::core::handler::v1::{
        CreateSpendOptions, FixSpendStatusOptions, QuoteSpendOptions, SendSpendOptions,
    };
    use mystiko_protos::service::v1::ClientOptions;
    use mystiko_protos::storage::v1::QueryFilter;
    use mystiko_storage::ColumnValues;
    use std::sync::Arc;

    pub(crate) async fn quote(options: QuoteSpendOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.quote(options).await;
                match result {
                    Ok(quote) => ApiResponse::success(SpendQuoteResponse::builder().quote(quote).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn summary(options: CreateSpendOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.summary(options).await;
                match result {
                    Ok(summary) => ApiResponse::success(SpendSummaryResponse::builder().summary(summary).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn create(options: CreateSpendOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.create(options).await;
                match result {
                    Ok(spend) => ApiResponse::success(CreateSpendResponse::builder().spend(spend).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn send(options: SendSpendOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.send(options).await;
                match result {
                    Ok(spend) => ApiResponse::success(SendSpendResponse::builder().spend(spend).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn send_with_grpc(send_options: SendSpendOptions, client_options: ClientOptions) -> ApiResponse {
        // grpc client to call dart grpc server
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => match GrpcSigner::connect(&client_options).await {
                Ok(signer) => match mystiko.spends.send_with_signer(send_options, Arc::new(signer)).await {
                    Ok(spend) => ApiResponse::success(SendSpendResponse::builder().spend(spend).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                },
                Err(err) => ApiResponse::error(SpendError::GrpcConnectError, err),
            },
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn fix_status(options: FixSpendStatusOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.fix_status(options).await;
                match result {
                    Ok(spend) => ApiResponse::success(FixSpendStatusResponse::builder().spend(spend).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find(filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.find(filter).await;
                match result {
                    Ok(spends) => ApiResponse::success(FindSpendsResponse::builder().spends(spends).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_all() -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.find_all().await;
                match result {
                    Ok(spends) => ApiResponse::success(FindSpendsResponse::builder().spends(spends).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_one(filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.find_one(filter).await;
                match result {
                    Ok(spend) => ApiResponse::success(FindOneSpendResponse::builder().spend(spend).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_by_id(id: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.find_by_id(id).await;
                match result {
                    Ok(spend) => ApiResponse::success(FindOneSpendResponse::builder().spend(spend).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn count(filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.count(filter).await;
                match result {
                    Ok(count) => ApiResponse::success(CountSpendsResponse::builder().count(count).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn count_all() -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.count_all().await;
                match result {
                    Ok(count) => ApiResponse::success(CountSpendsResponse::builder().count(count).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update(spend: Spend) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.update(spend).await;
                match result {
                    Ok(spend) => ApiResponse::success(UpdateSpendResponse::builder().spend(spend).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_batch(spends: Vec<Spend>) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.update_batch(spends).await;
                match result {
                    Ok(spends) => ApiResponse::success(UpdateSpendBatchResponse::builder().spends(spends).build()),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_by_filter(column_values: ColumnValues, filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.update_by_filter(column_values, filter).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_all(column_values: ColumnValues) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.update_all(column_values).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn delete(spend: Spend) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.delete(spend).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn delete_batch(spends: Vec<Spend>) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.delete_batch(spends).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn delete_by_filter(filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.delete_by_filter(filter).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn delete_all() -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.spends.delete_all().await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_spends_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SpendError::GetMystikoGuardError, err),
        }
    }
}
