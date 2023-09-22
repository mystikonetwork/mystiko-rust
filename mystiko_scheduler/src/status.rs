use crate::SchedulerError;
use http::{Request, Response};
use hyper::service::{make_service_fn, service_fn};
use hyper::Body;
use std::fmt::Debug;
use std::net::SocketAddr;
use tokio::sync::oneshot::{channel, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct SchedulerStatus {
    pub started: bool,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub(crate) struct StatusServer {
    bind_address: String,
    port: u16,
    #[builder(default, setter(skip))]
    join_handle: Mutex<Option<JoinHandle<Result<(), SchedulerError>>>>,
    #[builder(default, setter(skip))]
    shutdown_sender: Mutex<Option<Sender<()>>>,
}

impl StatusServer {
    pub(crate) async fn start(&self) -> Result<(), SchedulerError> {
        let mut join_handle = self.join_handle.lock().await;
        if join_handle.is_none() {
            let service = make_service_fn(|_| async {
                let handler_fn = service_fn(handle_request);
                Ok::<_, SchedulerError>(handler_fn)
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
                                Err(SchedulerError::HyperError(err))
                            }
                        }
                    }
                    Err(_) => Err(SchedulerError::SendStatusServerStartSignalError),
                }
            }));
            let _ = start_receiver.await;
        }
        Ok(())
    }

    pub(crate) async fn join(&self) -> Result<(), SchedulerError> {
        if let Some(join_handle) = self.join_handle.lock().await.take() {
            return join_handle.await?;
        }
        Ok(())
    }

    pub(crate) async fn stop(&self) -> Result<(), SchedulerError> {
        if let Some(shutdown_sender) = self.shutdown_sender.lock().await.take() {
            if !shutdown_sender.is_closed() {
                shutdown_sender
                    .send(())
                    .map_err(|_| SchedulerError::SendStatusServerStopSignalError)?;
            }
        }
        self.join().await
    }
}

async fn handle_request(request: Request<Body>) -> Result<Response<Body>, SchedulerError> {
    match (request.method(), request.uri().path()) {
        (&http::Method::GET, "/status") => Ok(Response::builder()
            .status(http::StatusCode::OK)
            .body(Body::from("success"))?),
        _ => Ok(Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(Body::empty())?),
    }
}
