use anyhow::Result;
use log::info;
use mystiko_relayer_server::application::{Application, ApplicationOptions};
use mystiko_relayer_server::common::AppStateOptions;

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

    main_inner(relayer_config_path, mystiko_config_path, server_config_path, log_level).await
}

async fn main_inner(
    relayer_config_path: Option<&str>,
    mystiko_config_path: Option<&str>,
    server_config_path: &str,
    log_level: &str,
) -> Result<()> {
    let application = Application::new(
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
    .await?;

    // run transact consumers
    for consumer in application.consumers {
        tokio::spawn(async {
            consumer.run().await;
        });
    }

    // server await
    info!("Application server start at {}:{}", HOST, PORT);
    application.server.await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::main_inner;
    use std::time::Duration;

    #[tokio::test]
    async fn test_main_inner() {
        let _result = tokio::time::timeout(
            Duration::from_secs(15),
            main_inner(
                Some("./tests/files/relayer_config.json"),
                Some("./tests/files/mystiko_config.json"),
                "./tests/files/config_test_testnet.toml",
                "debug",
            ),
        )
        .await;
    }
}
