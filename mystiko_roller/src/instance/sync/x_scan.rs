// todo support xscan
use crate::common::env::load_x_scan_api_key;

pub struct XScanInstance {
    url: String,
    key: String,
}

impl XScanInstance {
    pub fn new(url: &str) -> Self {
        let key = load_x_scan_api_key().unwrap();
        XScanInstance {
            url: url.to_string(),
            key,
        }
    }
}
