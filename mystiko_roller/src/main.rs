extern crate log;
extern crate mystiko_roller;

use mystiko_roller::roller::Roller;
use std::process;
use tokio::runtime;

async fn run_roller() {
    let roller = Roller::new().await.unwrap();
    roller.start().await;
}

fn main() {
    let rt = runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(run_roller());
    println!("roller exist");
    process::exit(1);
}
