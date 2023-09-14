use anyhow::Result;
use mockall::mock;
use mystiko_protos::testing::v1::{EchoRequest, EchoResponse};
use tonic::{Request, Response, Status};

mock! {
    pub TestingService {}

    #[tonic::async_trait]
    impl mystiko_protos::testing::v1::testing_service_server::TestingService for TestingService {
        async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status>;
    }
}
