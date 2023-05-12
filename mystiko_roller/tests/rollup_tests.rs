use std::env;
use std::sync::Once;

static INIT: Once = Once::new();
fn setup() {
    INIT.call_once(|| {
        env::set_var("COIN_MARKET_CAP_API_KEY", "mock");
    });
}
