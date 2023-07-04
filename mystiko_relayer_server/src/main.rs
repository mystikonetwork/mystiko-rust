use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Result;
use log::info;
use mystiko_ethers::provider::pool::ProviderPool;
use mystiko_relayer_server::channel::transact_channel;
use mystiko_relayer_server::common::{init_app_state, AppStateOptions};
use mystiko_relayer_server::database::init_sqlite_database;
use mystiko_relayer_server::handler::account::AccountHandler;
use mystiko_relayer_server::handler::transaction::TransactionHandler;
use mystiko_relayer_server::service::{info, transact, transaction_status};
use mystiko_server_utils::token_price::config::TokenPriceConfig;
use mystiko_server_utils::token_price::price::TokenPrice;
use std::sync::Arc;
use tokio::sync::RwLock;

pub const DEFAULT_CONFIG_PATH: &str = "./config.toml";
pub const HOST: &str = "127.0.0.1";
pub const PORT: u16 = 8081;
pub const DEFAULT_LOG_LEVEL: &str = "debug";
pub const ARRAY_QUEUE_CAPACITY: usize = 50;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let log_level = args.get(1).map(|level| level.as_str()).unwrap_or(DEFAULT_LOG_LEVEL);
    let server_config_path = args.get(2).map(|path| path.as_str()).unwrap_or(DEFAULT_CONFIG_PATH);
    let relayer_config_path = args.get(3).map(|path| Some(path.as_str())).unwrap_or(None);
    let mystiko_config_path = args.get(4).map(|path| Some(path.as_str())).unwrap_or(None);

    // init app state
    let app_state = init_app_state(
        AppStateOptions::builder()
            .log_level(log_level)
            .server_config_path(server_config_path)
            .relayer_config_path(relayer_config_path)
            .mystiko_config_path(mystiko_config_path)
            .build(),
    )
    .await?;

    // init sqlite db connection
    let db = Arc::new(init_sqlite_database(&app_state.server_config.sqlite_db_path).await?);

    // create account handler
    let account_handler = Arc::new(AccountHandler::new(db.clone(), &app_state.server_config.accounts).await?);

    // create transaction handler
    let transaction_handler = Arc::new(TransactionHandler::new(db.clone()));

    // init token price
    let token_price = Arc::new(RwLock::new(TokenPrice::new(
        &TokenPriceConfig::new(
            serde_json::to_string(&app_state.server_config.network_type)?.as_str(),
            None,
        )?,
        &app_state.server_config.coin_market_cap_api_key,
    )?));

    // init transact channel
    let providers = Arc::new(RwLock::new(
        ProviderPool::builder()
            .chain_providers_options(Box::new(app_state.clone()))
            .build(),
    ));
    let (senders, consumers) = transact_channel::init(
        &app_state.server_config,
        &app_state.relayer_config,
        providers.clone(),
        transaction_handler.clone(),
        token_price.clone(),
        ARRAY_QUEUE_CAPACITY,
    )
    .await?;

    // run transact consumers
    for consumer in consumers {
        tokio::spawn(async {
            consumer.run().await;
        });
    }

    info!("Server startup [host] {} [port] {}", HOST, PORT);

    // run http server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(app_state.clone()))
            .app_data(Data::new(senders.clone()))
            .app_data(Data::new(account_handler.clone()))
            .app_data(Data::new(transaction_handler.clone()))
            .app_data(Data::new(token_price.clone()))
            .app_data(Data::new(providers.clone()))
            .service(info)
            .service(transact)
            .service(transaction_status)
    })
    .bind((HOST, PORT))?
    .run()
    .await?;

    Ok(())
}
