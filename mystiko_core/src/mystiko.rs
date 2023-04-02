use crate::handler::wallet::WalletHandler;
use anyhow::Result;
use futures::lock::Mutex;
use mystiko_config::wrapper::mystiko::{MystikoConfig, RemoteOptions};
use mystiko_database::database::Database;
use mystiko_storage::document::DocumentRawData;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::storage::Storage;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

pub struct Mystiko<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    db: Arc<Mutex<Database<F, R, S>>>,
    config: Arc<MystikoConfig>,
    wallets: WalletHandler<F, R, S>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct MystikoOptions {
    #[builder(default, setter(strip_option))]
    pub config_file_path: Option<String>,
    #[builder(default, setter(strip_option))]
    pub config_remote_base_url: Option<String>,
    #[builder(default, setter(strip_option))]
    pub config_git_revision: Option<String>,
    #[builder(default = false)]
    pub is_testnet: bool,
    #[builder(default = false)]
    pub is_staging: bool,
}

#[derive(Error, Debug)]
pub enum MystikoError {
    #[error("config raised error: {0:?}")]
    ConfigError(#[source] anyhow::Error),
    #[error("failed to migrate database: {0:?}")]
    MigrationError(#[source] anyhow::Error),
}

impl<F, R, S> Mystiko<F, R, S>
where
    F: StatementFormatter,
    R: DocumentRawData,
    S: Storage<R>,
{
    pub async fn new(
        database: Database<F, R, S>,
        options: Option<MystikoOptions>,
    ) -> Result<Self, MystikoError> {
        let mystiko_options = options.clone().unwrap_or(MystikoOptions::builder().build());
        database
            .migrate()
            .await
            .map_err(MystikoError::MigrationError)?;
        let db = Arc::new(Mutex::new(database));
        let config = create_mystiko_config(&mystiko_options).await?;
        let wallets = WalletHandler::new(db.clone());
        let mystiko = Self {
            db,
            config: config.clone(),
            wallets,
        };
        log::info!(
            "mystiko on {} has been initialized, config git revision {}",
            if mystiko_options.is_testnet {
                "testnet"
            } else {
                "mainnet"
            },
            config.git_revision().unwrap_or("unknown")
        );
        Ok(mystiko)
    }

    pub fn db(&self) -> &Mutex<Database<F, R, S>> {
        self.db.as_ref()
    }

    pub fn config(&self) -> &MystikoConfig {
        self.config.as_ref()
    }

    pub fn wallets(&mut self) -> &mut WalletHandler<F, R, S> {
        &mut self.wallets
    }
}

async fn create_mystiko_config(
    mystiko_options: &MystikoOptions,
) -> Result<Arc<MystikoConfig>, MystikoError> {
    let config = if let Some(config_file_path) = &mystiko_options.config_file_path {
        MystikoConfig::from_json_file(config_file_path)
            .await
            .map_err(MystikoError::ConfigError)?
    } else {
        let mut remote_options = RemoteOptions::builder()
            .is_testnet(mystiko_options.is_testnet)
            .is_staging(mystiko_options.is_staging)
            .build();
        remote_options.base_url = mystiko_options.config_remote_base_url.clone();
        remote_options.git_revision = mystiko_options.config_git_revision.clone();
        MystikoConfig::from_remote(&remote_options)
            .await
            .map_err(MystikoError::ConfigError)?
    };
    Ok(Arc::new(config))
}
