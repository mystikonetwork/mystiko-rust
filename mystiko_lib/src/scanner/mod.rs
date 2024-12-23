use crate::runtime;
use mystiko_protos::api::scanner::v1::{
    AssetsRequest, BalanceRequest, ChainAssetsRequest, ScannerAssetImportRequest, ScannerResetRequest,
    ScannerScanRequest, ScannerSyncRequest,
};
use mystiko_protos::api::v1::{ApiResponse, ScannerError};

pub fn sync<M>(message: M) -> ApiResponse
where
    M: TryInto<ScannerSyncRequest>,
    <M as TryInto<ScannerSyncRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::sync(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(ScannerError::DeserializeMessageError, err),
    }
}

pub fn scan<M>(message: M) -> ApiResponse
where
    M: TryInto<ScannerScanRequest>,
    <M as TryInto<ScannerScanRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::scan(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(ScannerError::DeserializeMessageError, err),
    }
}

pub fn reset<M>(message: M) -> ApiResponse
where
    M: TryInto<ScannerResetRequest>,
    <M as TryInto<ScannerResetRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::reset(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(ScannerError::DeserializeMessageError, err),
    }
}

pub fn import<M>(message: M) -> ApiResponse
where
    M: TryInto<ScannerAssetImportRequest>,
    <M as TryInto<ScannerAssetImportRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::import(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(ScannerError::DeserializeMessageError, err),
    }
}

pub fn balance<M>(message: M) -> ApiResponse
where
    M: TryInto<BalanceRequest>,
    <M as TryInto<BalanceRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::balance(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(ScannerError::DeserializeMessageError, err),
    }
}

pub fn assets<M>(message: M) -> ApiResponse
where
    M: TryInto<AssetsRequest>,
    <M as TryInto<AssetsRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::assets(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(ScannerError::DeserializeMessageError, err),
    }
}

pub fn chain_assets<M>(message: M) -> ApiResponse
where
    M: TryInto<ChainAssetsRequest>,
    <M as TryInto<ChainAssetsRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::chain_assets(message.chain_id, options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(ScannerError::DeserializeMessageError, err),
    }
}

mod internal {
    use crate::error::parse_scanner_error;
    use crate::instance;
    use mystiko_core::ScannerHandler;
    use mystiko_protos::api::scanner::v1::{
        AssetsResponse, BalanceResponse, ChainAssetsResponse, ScannerAssetImportResponse, ScannerResetResponse,
        ScannerScanResponse,
    };
    use mystiko_protos::api::v1::{ApiResponse, ScannerError};
    use mystiko_protos::core::scanner::v1::{
        AssetImportOptions, AssetsOptions, BalanceOptions, ScannerResetOptions, ScannerScanOptions, ScannerSyncOptions,
    };

    pub(crate) async fn sync(options: ScannerSyncOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.scanner.sync(options).await;
                match result {
                    Ok(balance) => ApiResponse::success(BalanceResponse::builder().result(balance).build()),
                    Err(err) => ApiResponse::error(parse_scanner_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(ScannerError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn scan(options: ScannerScanOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.scanner.scan(options).await;
                match result {
                    Ok(scan) => ApiResponse::success(ScannerScanResponse::builder().result(scan).build()),
                    Err(err) => ApiResponse::error(parse_scanner_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(ScannerError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn reset(options: ScannerResetOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.scanner.reset(options).await;
                match result {
                    Ok(reset) => ApiResponse::success(ScannerResetResponse::builder().result(reset).build()),
                    Err(err) => ApiResponse::error(parse_scanner_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(ScannerError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn import(options: AssetImportOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.scanner.import(options).await;
                match result {
                    Ok(asset) => ApiResponse::success(ScannerAssetImportResponse::builder().result(asset).build()),
                    Err(err) => ApiResponse::error(parse_scanner_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(ScannerError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn balance(options: BalanceOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.scanner.balance(options).await;
                match result {
                    Ok(balance) => ApiResponse::success(BalanceResponse::builder().result(balance).build()),
                    Err(err) => ApiResponse::error(parse_scanner_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(ScannerError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn assets(options: AssetsOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.scanner.assets(options).await;
                match result {
                    Ok(results) => ApiResponse::success(AssetsResponse::builder().results(results).build()),
                    Err(err) => ApiResponse::error(parse_scanner_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(ScannerError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn chain_assets(chain_id: u64, options: AssetsOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.scanner.chain_assets(chain_id, options).await;
                match result {
                    Ok(result) => ApiResponse::success(ChainAssetsResponse::builder().result(result).build()),
                    Err(err) => ApiResponse::error(parse_scanner_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(ScannerError::GetMystikoGuardError, err),
        }
    }
}
