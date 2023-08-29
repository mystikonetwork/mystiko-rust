use anyhow::{Error, Result};
use lazy_static::lazy_static;
use mystiko_core::mystiko::{Mystiko, MystikoOptions};
use mystiko_database::database::Database;
use mystiko_protos::core::v1::MystikoOptions as ProtoMystikoOptions;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use prost::Message;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;

type MystikoType = Mystiko<SqlStatementFormatter, SqliteStorage>;
type OptionalMystiko = Option<MystikoType>;

lazy_static! {
    pub(crate) static ref MYSTIKO: RwLock<MystikoLib> = RwLock::new(MystikoLib::new());
    pub(crate) static ref MYSTIKO_RUNTIME: Runtime = Runtime::new().unwrap();
}

pub(crate) struct MystikoLib {
    mystiko: OptionalMystiko,
}

impl MystikoLib {
    pub(crate) fn new() -> Self {
        MystikoLib { mystiko: None }
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

async fn initialize(options: &[u8]) -> Result<()> {
    let options = ProtoMystikoOptions::decode(options)?;
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
                Some(MystikoOptions {
                    config_file_path: options.config_file_path.clone(),
                    config_remote_base_url: options.config_remote_base_url.clone(),
                    config_git_revision: options.config_git_revision.clone(),
                    is_testnet: options.is_testnet,
                    is_staging: options.is_staging,
                    provider_factory: None,
                }),
            )
            .await?;
            mystiko_guard.initialize(mystiko);
        }
    }
    Ok(())
}

async fn is_initialized() -> bool {
    MYSTIKO.read().await.is_initialized()
}

async fn destroy() {
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
