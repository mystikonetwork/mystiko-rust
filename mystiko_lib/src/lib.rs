#[cfg(target_os = "android")]
extern crate android_logger;
#[cfg(not(target_os = "android"))]
extern crate env_logger;
extern crate log;

pub mod config;
pub mod error;
pub mod handler;

pub use handler::*;

use anyhow::{Error, Result};
use lazy_static::lazy_static;
use mystiko_core::Mystiko;
use mystiko_protos::api::v1::ApiResponse;
use mystiko_protos::core::v1::MystikoOptions as ProtoMystikoOptions;
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;

type MystikoType = Mystiko<SqlStatementFormatter, SqliteStorage>;
type OptionalMystiko = Option<MystikoType>;

lazy_static! {
    static ref MYSTIKO: RwLock<MystikoStatic> = RwLock::new(MystikoStatic::new());
    static ref MYSTIKO_RUNTIME: Runtime = Runtime::new().unwrap();
}

pub fn initialize<T>(options: T) -> ApiResponse
where
    T: TryInto<ProtoMystikoOptions>,
    <T as TryInto<ProtoMystikoOptions>>::Error: std::error::Error + Send + Sync + 'static,
{
    match options.try_into() {
        Ok(options) => runtime().block_on(internal::initialize(options)),
        Err(err) => ApiResponse::unknown_error(err),
    }
}

pub fn is_initialized() -> bool {
    runtime().block_on(internal::is_initialized())
}

pub fn destroy() {
    runtime().block_on(internal::destroy())
}

pub(crate) fn instance() -> &'static RwLock<MystikoStatic> {
    &MYSTIKO
}

pub(crate) fn runtime() -> &'static Runtime {
    &MYSTIKO_RUNTIME
}

pub(crate) struct MystikoStatic {
    mystiko: OptionalMystiko,
}

impl MystikoStatic {
    pub(crate) fn new() -> Self {
        MystikoStatic { mystiko: None }
    }

    pub(crate) fn initialize(&mut self, mystiko: MystikoType) {
        self.mystiko = Some(mystiko);
    }

    pub(crate) fn is_initialized(&self) -> bool {
        self.mystiko.is_some()
    }

    pub(crate) fn destroy(&mut self) {
        self.mystiko = None;
    }

    pub(crate) fn get(&self) -> Result<&MystikoType> {
        self.mystiko
            .as_ref()
            .ok_or_else(|| Error::msg("Mystiko not initialized"))
    }
}

mod internal {
    use super::*;
    use crate::error::parse_mystiko_error;
    use mystiko_core::{Database, MystikoOptions};
    use mystiko_protos::api::v1::MystikoError;
    use mystiko_static_cache::{FileStaticCache, StaticCache};
    use mystiko_storage_sqlite::SqliteStorageOptions;

    pub(crate) async fn initialize(options: ProtoMystikoOptions) -> ApiResponse {
        match is_initialized().await {
            true => ApiResponse::success_with_empty(),
            false => {
                let mut mystiko_guard = MYSTIKO.write().await;

                if mystiko_guard.is_initialized() {
                    ApiResponse::success_with_empty()
                } else {
                    init_logger();

                    let storage_options = SqliteStorageOptions::builder().path(options.db_path).build();
                    let storage = match SqliteStorage::new(storage_options).await {
                        Ok(storage) => storage,
                        Err(err) => return ApiResponse::error(MystikoError::StorageError, err),
                    };

                    let database = Database::new(SqlStatementFormatter::sqlite(), storage);
                    let static_cache = if let Some(static_cache_path) = options.static_cache_path {
                        match FileStaticCache::new(static_cache_path).await {
                            Ok(static_cache) => Some(Box::new(static_cache) as Box<dyn StaticCache>),
                            Err(err) => return ApiResponse::error(MystikoError::FileStaticCacheError, err),
                        }
                    } else {
                        None
                    };
                    let mystiko_options = MystikoOptions::builder()
                        .config_options(options.config_options)
                        .relayer_client_options(options.relayer_client_options)
                        .loader_config(options.loader_config)
                        .static_cache(static_cache)
                        .build();

                    let mystiko = match MystikoType::new(database, Some(mystiko_options)).await {
                        Ok(mystiko) => mystiko,
                        Err(err) => return ApiResponse::error(parse_mystiko_error(&err), err),
                    };

                    mystiko_guard.initialize(mystiko);
                    ApiResponse::success_with_empty()
                }
            }
        }
    }

    pub(crate) async fn is_initialized() -> bool {
        MYSTIKO.read().await.is_initialized()
    }

    pub(crate) async fn destroy() {
        MYSTIKO.write().await.destroy()
    }

    #[cfg(target_os = "android")]
    fn init_logger() {
        android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Trace));
    }

    #[cfg(not(target_os = "android"))]
    fn init_logger() {
        if let Err(e) = env_logger::try_init() {
            log::warn!("Failed to initialize logger: {}", e);
        }
    }
}
