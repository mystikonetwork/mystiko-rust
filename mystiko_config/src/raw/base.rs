pub trait RawConfig {
    fn validate(&self) -> Result<(), Vec<String>>;
}