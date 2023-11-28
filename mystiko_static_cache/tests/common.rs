use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use mystiko_static_cache::GetOptions;

mock! {
    #[derive(Debug)]
    pub StaticCache {}

    #[async_trait]
    impl mystiko_static_cache::StaticCache for StaticCache {
        async fn get(&self, url: &str, options: Option<GetOptions>) -> Result<Vec<u8>>;
        async fn get_failover(&self, urls: &[String], options: Option<GetOptions>) -> Result<Vec<u8>>;
    }
}
