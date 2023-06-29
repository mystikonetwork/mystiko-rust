extern crate mystiko_roller;
extern crate tracing;

use mystiko_roller::roller::Roller;
use std::process;
use tracing::error;

async fn run_roller() {
    let roller = Roller::new().await;
    match roller {
        Ok(mut r) => {
            r.start().await;
        }
        Err(e) => {
            error!("roller start failed {:?}", e);
            process::exit(-1);
        }
    }
}

#[tokio::main]
async fn main() {
    run_roller().await;
}
