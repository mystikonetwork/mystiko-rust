use anyhow::Result;
use bytes::Bytes;
use http::{Request, Response};
use mystiko_protos::service::v1::ServerOptions;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use tonic::body::BoxBody;
use tonic::codegen::Service;
use tonic::server::NamedService;
use tonic::transport::server::{Router, Routes};
use tonic::transport::{Body, Server};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
use tower_layer::Layer;
use typed_builder::TypedBuilder;

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct GrpcServer {
    shutdown: Option<oneshot::Sender<()>>,
    server_task: Option<tokio::task::JoinHandle<Result<()>>>,
}

type TonicError = Box<dyn std::error::Error + Send + Sync>;

impl GrpcServer {
    pub async fn start<S, O>(&mut self, service: S, options: O) -> Result<()>
    where
        S: Service<Request<Body>, Response = Response<BoxBody>, Error = Infallible>
            + NamedService
            + Clone
            + Send
            + 'static,
        S::Future: Send + 'static,
        O: Into<ServerOptions>,
    {
        let options = options.into();
        let address = options.socket_address()?;
        let enable_web = options.enable_web();
        let mut server: Server = options.try_into()?;
        if enable_web {
            let cors = CorsLayer::new()
                .allow_headers(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_origin(tower_http::cors::Any);
            let router = server.layer(cors).layer(GrpcWebLayer::new()).add_service(service);
            self.start_with_router(router, address).await
        } else {
            let router = server.add_service(service);
            self.start_with_router(router, address).await
        }
    }

    pub async fn start_with_router<L, ResBody>(&mut self, router: Router<L>, address: SocketAddr) -> Result<()>
    where
        L: Layer<Routes> + Send + 'static,
        L::Service: Service<Request<Body>, Response = Response<ResBody>> + Clone + Send + 'static,
        <<L as Layer<Routes>>::Service as Service<Request<Body>>>::Future: Send + 'static,
        <<L as Layer<Routes>>::Service as Service<Request<Body>>>::Error: Into<TonicError> + Send,
        ResBody: http_body::Body<Data = Bytes> + Send + 'static,
        ResBody::Error: Into<TonicError>,
    {
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let (server_ready_tx, server_ready_rx) = oneshot::channel::<()>();
        self.shutdown = Some(shutdown_tx);
        let server_task = tokio::spawn(async move {
            let server = router.serve_with_shutdown(address, async {
                let _ = shutdown_rx.await;
            });
            if server_ready_tx.send(()).is_err() {
                return Err(anyhow::anyhow!("failed to send server ready notification"));
            }
            match server.await {
                Err(e) => Err(anyhow::anyhow!("failed to await server to stop: {:?}", e)),
                Ok(_) => Ok(()),
            }
        });
        self.server_task = Some(server_task);
        server_ready_rx
            .await
            .map_err(|_| anyhow::anyhow!("failed to wait on server ready signal"))?;
        Ok(())
    }

    pub async fn join(&mut self) -> Result<()> {
        if let Some(server_task) = self.server_task.take() {
            let _ = server_task.await?;
        }
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(shutdown) = self.shutdown.take() {
            shutdown
                .send(())
                .map_err(|_| anyhow::anyhow!("failed to send shutdown signal"))?;
            return self.join().await;
        }
        Ok(())
    }
}
