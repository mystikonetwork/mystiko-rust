#[cfg(test)]
mod tests {

    use base64::{engine::general_purpose, Engine as _};
    use mockito;
    use mockito::*;
    use mystiko_indexer_client::client::MystikoIndexerClient;
    use mystiko_indexer_client::config::ClientConfig;
    use mystiko_indexer_client::response::ApiResponse;
    use mystiko_indexer_client::v1::implement::MystikoIndexerClientV1;
    use serde_json;

    static AUTH_USERNAME: &str = "test_username";
    static AUTH_PASSWORD: &str = "110110";

    fn generate_auth_header() -> String {
        let str = AUTH_USERNAME.to_owned() + ":" + AUTH_PASSWORD;
        let encoded = general_purpose::STANDARD.encode(&str);
        String::from("Basic ".to_owned() + &encoded)
    }

    fn create_indexer_client(base_url: &str) -> MystikoIndexerClientV1 {
        let config = &ClientConfig {
            base_url: String::from(base_url),
            auth_username: None,
            auth_password: None,
            timeout: None,
        };
        MystikoIndexerClientV1::new(config)
    }

    fn create_indexer_client_with_auth(base_url: &str) -> MystikoIndexerClientV1 {
        let config = &ClientConfig {
            base_url: String::from(base_url),
            auth_username: Some(String::from(AUTH_USERNAME)),
            auth_password: Some(String::from(AUTH_PASSWORD)),
            timeout: None,
        };
        MystikoIndexerClientV1::new(config)
    }

    #[tokio::test]
    async fn test_ping() {
        let mut server = Server::new_async().await;
        let url = server.url();
        let message = "hello";
        let api_resp = ApiResponse {
            code: 0,
            result: String::from(message),
        };
        let m = server
            .mock("get", "/ping")
            .match_query(Matcher::Regex("message=hello".into()))
            .with_status(200)
            .with_body(serde_json::to_string(&api_resp).unwrap())
            .create_async()
            .await;
        let client = create_indexer_client(&url);
        let resp = client.ping(String::from(message)).await.unwrap();
        assert_eq!(resp, message);
        m.assert_async().await;
    }

    #[tokio::test]
    async fn test_auth_ping() {
        let mut server = Server::new_async().await;
        let url = server.url();
        let message = "helloauth";
        let api_resp = ApiResponse {
            code: 0,
            result: String::from(message),
        };
        let auth_header = generate_auth_header();
        let a: &str = &auth_header;
        let m = server
            .mock("get", "/auth-ping")
            .match_query(Matcher::Regex("message=helloauth".into()))
            .match_header("authorization", a)
            .with_body(serde_json::to_string(&api_resp).unwrap())
            .with_status(200)
            .create_async()
            .await;
        let client = create_indexer_client_with_auth(&url);
        let resp = client.auth_ping(String::from(message)).await.unwrap();
        assert_eq!(resp, message);
        m.assert_async().await;
    }
}
