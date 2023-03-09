pub struct ClientConfig {
    pub base_url: String,
    pub auth_username: Option<String>,
    pub auth_password: Option<String>,
    pub timeout: Option<u64>,
}
