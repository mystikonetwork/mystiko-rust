use crate::StatusServerError;
use async_trait::async_trait;
use http::{Request, Response};
use hyper::service::{make_service_fn, service_fn};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot::{channel, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use typed_builder::TypedBuilder;

#[async_trait]
pub trait Status: Send + Sync + Debug {
    async fn status(&self) -> anyhow::Result<(mime::Mime, hyper::Body)>;
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct StatusServer {
    bind_address: String,
    port: u16,
    #[builder(default)]
    status: Option<Arc<Box<dyn Status>>>,
    #[builder(default, setter(skip))]
    join_handle: Mutex<Option<JoinHandle<Result<(), StatusServerError>>>>,
    #[builder(default, setter(skip))]
    shutdown_sender: Mutex<Option<Sender<()>>>,
}

impl StatusServer {
    pub async fn start(&self) -> Result<(), StatusServerError> {
        let mut join_handle = self.join_handle.lock().await;
        if join_handle.is_none() {
            let status = self.status.clone();
            let service = make_service_fn(move |_| {
                let status = status.clone();
                async move {
                    let handler_fn = service_fn(move |request| handle_request(status.clone(), request));
                    Ok::<_, StatusServerError>(handler_fn)
                }
            });
            let socket_address: SocketAddr = format!("{}:{}", self.bind_address, self.port).parse()?;
            let server = hyper::Server::try_bind(&socket_address)?.serve(service);
            let (shutdown_sender, shutdown_receiver) = channel::<()>();
            *self.shutdown_sender.lock().await = Some(shutdown_sender);
            let graceful = server.with_graceful_shutdown(async {
                let _ = shutdown_receiver.await;
                log::info!("scheduler_status_server received graceful shutdown signal");
            });
            let (start_sender, start_receiver) = channel::<()>();
            *join_handle = Some(tokio::spawn(async move {
                match start_sender.send(()) {
                    Ok(_) => {
                        log::info!(
                            "scheduler_status_server has been started listening on {}",
                            socket_address
                        );
                        match graceful.await {
                            Ok(_) => {
                                log::info!("scheduler_status_server has been stopped successfully");
                                Ok(())
                            }
                            Err(err) => {
                                log::error!("scheduler_status_server has been stopped with error: {}", err);
                                Err(StatusServerError::HyperError(err))
                            }
                        }
                    }
                    Err(_) => Err(StatusServerError::SendStatusServerStartSignalError),
                }
            }));
            let _ = start_receiver.await;
        }
        Ok(())
    }

    pub async fn join(&self) -> Result<(), StatusServerError> {
        if let Some(join_handle) = self.join_handle.lock().await.take() {
            return join_handle.await?;
        }
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), StatusServerError> {
        if let Some(shutdown_sender) = self.shutdown_sender.lock().await.take() {
            if !shutdown_sender.is_closed() {
                shutdown_sender
                    .send(())
                    .map_err(|_| StatusServerError::SendStatusServerStopSignalError)?;
            }
        }
        self.join().await
    }
}

async fn handle_request(
    status: Option<Arc<Box<dyn Status>>>,
    request: Request<hyper::Body>,
) -> Result<Response<hyper::Body>, StatusServerError> {
    match (request.method(), request.uri().path()) {
        (&http::Method::GET, "/status") => {
            if let Some(status) = status {
                match status.status().await {
                    Ok((content_type, body)) => Ok(Response::builder()
                        .status(http::StatusCode::OK)
                        .header(http::header::CONTENT_TYPE, content_type.to_string())
                        .body(body)?),
                    Err(err) => {
                        log::error!("failed to query /status of scheduler_status_server: {}", err);
                        Ok(Response::builder()
                            .header(http::header::CONTENT_TYPE, "text/plain")
                            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                            .body(hyper::Body::from(err.to_string()))?)
                    }
                }
            } else {
                Ok(Response::builder()
                    .status(http::StatusCode::OK)
                    .body(hyper::Body::empty())?)
            }
        }
        _ => Ok(Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(hyper::Body::empty())?),
    }
}
