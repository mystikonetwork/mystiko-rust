use crate::runtime;
use mystiko_protos::api::synchronizer::v1::{
    ChainSyncedBlockRequest, ContractSyncedBlockRequest, StatusRequest, SyncRequest, SynchronizerResetRequest,
};
use mystiko_protos::api::v1::{ApiResponse, SynchronizerError};

pub fn chain_synced_block<M>(message: M) -> ApiResponse
where
    M: TryInto<ChainSyncedBlockRequest>,
    <M as TryInto<ChainSyncedBlockRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::chain_synced_block(message.chain_id)),
        Err(err) => ApiResponse::error(SynchronizerError::DeserializeMessageError, err),
    }
}

pub fn contract_synced_block<M>(message: M) -> ApiResponse
where
    M: TryInto<ContractSyncedBlockRequest>,
    <M as TryInto<ContractSyncedBlockRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::contract_synced_block(
            message.chain_id,
            message.contract_address.as_str(),
        )),
        Err(err) => ApiResponse::error(SynchronizerError::DeserializeMessageError, err),
    }
}

pub fn status<M>(message: M) -> ApiResponse
where
    M: TryInto<StatusRequest>,
    <M as TryInto<StatusRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::status(message.with_contracts)),
        Err(err) => ApiResponse::error(SynchronizerError::DeserializeMessageError, err),
    }
}

pub fn sync<M>(message: M) -> ApiResponse
where
    M: TryInto<SyncRequest>,
    <M as TryInto<SyncRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::sync(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SynchronizerError::DeserializeMessageError, err),
    }
}

pub fn reset<M>(message: M) -> ApiResponse
where
    M: TryInto<SynchronizerResetRequest>,
    <M as TryInto<SynchronizerResetRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::reset(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(SynchronizerError::DeserializeMessageError, err),
    }
}

mod internal {
    use crate::error::parse_synchronizer_error;
    use crate::instance;
    use mystiko_core::SynchronizerHandler;
    use mystiko_protos::api::synchronizer::v1::{
        ChainSyncedBlockResponse, ContractSyncedBlockResponse, StatusResponse,
    };
    use mystiko_protos::api::v1::{ApiResponse, SynchronizerError};
    use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerResetOptions};

    pub(crate) async fn chain_synced_block(chain_id: u64) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.synchronizer.chain_synced_block(chain_id).await;
                match result {
                    Ok(result) => ApiResponse::success(ChainSyncedBlockResponse::builder().result(result).build()),
                    Err(err) => ApiResponse::error(parse_synchronizer_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SynchronizerError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn contract_synced_block(chain_id: u64, contract_address: &str) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko
                    .synchronizer
                    .contract_synced_block(chain_id, contract_address)
                    .await;
                match result {
                    Ok(result) => ApiResponse::success(ContractSyncedBlockResponse::builder().result(result).build()),
                    Err(err) => ApiResponse::error(parse_synchronizer_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SynchronizerError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn status(with_contracts: bool) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.synchronizer.status(with_contracts).await;
                match result {
                    Ok(status) => ApiResponse::success(StatusResponse::builder().status(status).build()),
                    Err(err) => ApiResponse::error(parse_synchronizer_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SynchronizerError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn sync(sync_option: SyncOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.synchronizer.sync(sync_option).await;
                match result {
                    Ok(status) => ApiResponse::success(StatusResponse::builder().status(status).build()),
                    Err(err) => ApiResponse::error(parse_synchronizer_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SynchronizerError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn reset(reset_options: SynchronizerResetOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.synchronizer.reset(reset_options).await;
                match result {
                    Ok(_) => ApiResponse::success_with_empty(),
                    Err(err) => ApiResponse::error(parse_synchronizer_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(SynchronizerError::GetMystikoGuardError, err),
        }
    }
}
