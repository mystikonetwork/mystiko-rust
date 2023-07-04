use crate::channel::consumer::TransactionConsumer;
use crate::channel::transact_channel;
use crate::common::{init_app_state, AppStateOptions};
use crate::database::init_sqlite_database;
use crate::handler::account::AccountHandler;
use crate::handler::transaction::TransactionHandler;
use crate::service::{info, transact, transaction_status};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Result;
use mystiko_ethers::provider::pool::ProviderPool;
use mystiko_server_utils::token_price::config::TokenPriceConfig;
use mystiko_server_utils::token_price::price::TokenPrice;
use std::sync::Arc;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Application {
    pub server: Server,
    pub consumers: Vec<TransactionConsumer>,
}

#[derive(TypedBuilder)]
pub struct ApplicationOptions<'a> {
    host: &'a str,
    port: u16,
    array_queue_capacity: usize,
    app_state_options: AppStateOptions<'a>,
}

impl Application {
    #[allow(clippy::needless_lifetimes)]
    pub async fn new<'a>(options: ApplicationOptions<'a>) -> Result<Self> {
        // init app state
        let app_state = init_app_state(options.app_state_options).await?;

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
            options.array_queue_capacity,
        )
        .await?;

        let http_server = HttpServer::new(move || {
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
        .bind((options.host, options.port))?
        .run();

        Ok(Application::builder().server(http_server).consumers(consumers).build())
    }
}
