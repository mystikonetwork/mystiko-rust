extern crate mystiko_roller;
extern crate tracing;

use mystiko_roller::roller::Roller;
use std::process;
use tokio::runtime;
use tracing::error;

async fn run_roller() {
    let roller = Roller::new().await;
    if roller.is_err() {
        error!("roller create failed {:?}", roller.err().unwrap());
        process::exit(-1);
    }
    roller.unwrap().start().await;
}

fn main() {
    let rt = runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(run_roller());
    error!("roller exist");
    process::exit(-1);
}
