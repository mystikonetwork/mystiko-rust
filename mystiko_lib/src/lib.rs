#[cfg(target_os = "android")]
extern crate android_logger;
#[cfg(not(target_os = "android"))]
extern crate env_logger;
extern crate log;

pub mod config;
mod handler;

pub use handler::*;

use anyhow::{Error, Result};
use lazy_static::lazy_static;
use mystiko_core::Mystiko;
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

pub fn initialize<T>(options: T) -> Result<()>
where
    T: TryInto<ProtoMystikoOptions>,
    <T as TryInto<ProtoMystikoOptions>>::Error: std::error::Error + Send + Sync + 'static,
{
    let options: ProtoMystikoOptions = options.try_into()?;
    runtime().block_on(internal::initialize(options))
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
    use mystiko_core::MystikoOptions;
    use mystiko_database::Database;
    use mystiko_storage_sqlite::SqliteStorageBuilder;

    pub(crate) async fn initialize(options: ProtoMystikoOptions) -> Result<()> {
        if !is_initialized().await {
            let mut mystiko_guard = MYSTIKO.write().await;
            if !mystiko_guard.is_initialized() {
                init_logger();
                let mut storage_builder = SqliteStorageBuilder::new();
                if let Some(db_path) = options.db_path.as_ref() {
                    storage_builder = storage_builder.path(db_path);
                }
                let storage = storage_builder.build().await?;
                let database = Database::new(SqlStatementFormatter::sqlite(), storage);
                let mystiko = Mystiko::new(
                    database,
                    Some(MystikoOptions::builder().config_options(options.config_options).build()),
                )
                .await?;
                mystiko_guard.initialize(mystiko);
            }
        }
        Ok(())
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
