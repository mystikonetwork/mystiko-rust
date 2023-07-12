use anyhow::Result;
use mystiko_relayer_server::application::{run_application, ApplicationOptions};
use mystiko_relayer_server::common::AppStateOptions;

pub const DEFAULT_CONFIG_PATH: &str = "./config.toml";
pub const HOST: &str = "0.0.0.0";
pub const PORT: u16 = 8090;
pub const DEFAULT_LOG_LEVEL: &str = "debug";
pub const ARRAY_QUEUE_CAPACITY: usize = 50;

#[actix_web::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let log_level = args.get(1).map(|level| level.as_str()).unwrap_or(DEFAULT_LOG_LEVEL);
    let server_config_path = args.get(2).map(|path| path.as_str()).unwrap_or(DEFAULT_CONFIG_PATH);
    let relayer_config_path = args.get(3).map(|path| Some(path.as_str())).unwrap_or(None);
    let mystiko_config_path = args.get(4).map(|path| Some(path.as_str())).unwrap_or(None);

    run_application(
        ApplicationOptions::builder()
            .host(HOST)
            .port(PORT)
            .app_state_options(
                AppStateOptions::builder()
                    .log_level(log_level)
                    .server_config_path(server_config_path)
                    .relayer_config_path(relayer_config_path)
                    .mystiko_config_path(mystiko_config_path)
                    .build(),
            )
            .array_queue_capacity(ARRAY_QUEUE_CAPACITY)
            .build(),
    )
    .await
}
