extern crate mystiko_roller;
extern crate tracing;

use mystiko_roller::roller::run_roller;

#[tokio::main]
async fn main() {
    run_roller().await;
}
