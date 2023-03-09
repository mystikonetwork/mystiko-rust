pub mod v1;

use async_trait::async_trait;
use mystiko_indexer_client::client::MystikoIndexerClient;
use std::io::Error;

pub struct TestMystikoIndexerClient {
    test_base_url: String,
}

#[async_trait]
impl MystikoIndexerClient for TestMystikoIndexerClient {
    async fn ping(&self, message: String) -> Result<String, Error> {
        Ok(message)
    }

    async fn auth_ping(&self, message: String) -> Result<String, Error> {
        Ok(message)
    }
}

#[tokio::test]
async fn test_indexer_client() {
    let test_indexer_client = TestMystikoIndexerClient {
        test_base_url: String::from("http://test_url:test_port"),
    };
    let test_message = String::from("hello");
    assert_eq!(
        test_indexer_client
            .ping(test_message.clone())
            .await
            .unwrap(),
        test_message
    );
    assert_eq!(
        test_indexer_client
            .auth_ping(test_message.clone())
            .await
            .unwrap(),
        test_message
    );
    assert_eq!(
        test_indexer_client.test_base_url,
        String::from("http://test_url:test_port")
    );
}
