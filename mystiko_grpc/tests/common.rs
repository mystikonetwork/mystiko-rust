use anyhow::Result;
use mystiko_protos::testing::v1::testing_service_server::TestingServiceServer;
use mystiko_protos::testing::v1::{EchoRequest, EchoResponse};
use rcgen::{generate_simple_self_signed, Certificate, CertificateParams};
use tokio::sync::oneshot;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};
use tonic_web::GrpcWebLayer;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Default)]
pub struct TestingService;

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TestingServer {
    pub shutdown: Option<oneshot::Sender<()>>,
    pub server_task: Option<tokio::task::JoinHandle<()>>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct StartOptions {
    pub port: u16,
    #[builder(default)]
    pub tls_keys: Option<TlsKeys>,
}

#[derive(Debug, Clone)]
pub struct TlsKeys {
    pub ca_pem: String,
    pub server_key: String,
    pub server_pem: String,
}

#[tonic::async_trait]
impl mystiko_protos::testing::v1::testing_service_server::TestingService for TestingService {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let message = request.get_ref().message.to_string();
        Ok(Response::new(EchoResponse::builder().message(message).build()))
    }
}

impl TestingServer {
    pub async fn start(&mut self, options: StartOptions) -> Result<oneshot::Receiver<()>> {
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let (server_ready_tx, server_ready_rx) = oneshot::channel::<()>();
        let address = format!("127.0.0.1:{}", options.port).parse()?;
        let tls_config = options.tls_keys.map(|tls_keys| {
            let identity = Identity::from_pem(tls_keys.server_pem, tls_keys.server_key);
            ServerTlsConfig::new().identity(identity)
        });
        self.shutdown = Some(shutdown_tx);
        let server_task = tokio::spawn(async move {
            let service = TestingServiceServer::new(TestingService);
            let mut server = Server::builder().accept_http1(true);
            if let Some(tls_config) = tls_config {
                server = server.tls_config(tls_config).expect("tls_config failed");
            }
            let server = server.layer(GrpcWebLayer::new()).add_service(service);
            let server = server.serve_with_shutdown(address, async {
                let _ = shutdown_rx.await;
            });
            server_ready_tx.send(()).expect("server_ready_tx send failed");
            server.await.expect("server failed")
        });
        self.server_task = Some(server_task);
        Ok(server_ready_rx)
    }

    pub async fn stop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            shutdown.send(()).expect("shutdown send failed");
            if let Some(server_task) = self.server_task.take() {
                server_task.await.expect("server task failed");
            }
        }
    }
}

impl TlsKeys {
    pub fn generate(domain_name: &str) -> Result<Self> {
        let ca_cert = generate_simple_self_signed(vec![domain_name.to_string()])?;
        let ca_pem = ca_cert.serialize_pem()?;
        let mut server_params = CertificateParams::new(vec![domain_name.to_string()]);
        server_params.is_ca = rcgen::IsCa::NoCa;
        let server_cert = Certificate::from_params(server_params)?;
        let server_pem = server_cert.serialize_pem_with_signer(&ca_cert)?;
        let server_key = server_cert.serialize_private_key_pem();
        Ok(Self {
            ca_pem,
            server_key,
            server_pem,
        })
    }
}
