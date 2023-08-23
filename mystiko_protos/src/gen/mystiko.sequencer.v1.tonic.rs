// @generated
/// Generated client implementations.
#[cfg(feature = "grpc-client")]
pub mod sequencer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct SequencerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SequencerServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SequencerServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SequencerServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SequencerServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        pub async fn fetch_chain(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchChainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchChainResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mystiko.sequencer.v1.SequencerService/FetchChain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mystiko.sequencer.v1.SequencerService",
                        "FetchChain",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn query_contract_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryContractCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryContractCommitmentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mystiko.sequencer.v1.SequencerService/QueryContractCommitment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mystiko.sequencer.v1.SequencerService",
                        "QueryContractCommitment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn count_contract_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::CountContractCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountContractCommitmentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mystiko.sequencer.v1.SequencerService/CountContractCommitment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mystiko.sequencer.v1.SequencerService",
                        "CountContractCommitment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn query_contract_nullifier(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryContractNullifierRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryContractNullifierResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mystiko.sequencer.v1.SequencerService/QueryContractNullifier",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mystiko.sequencer.v1.SequencerService",
                        "QueryContractNullifier",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn count_contract_nullifier(
            &mut self,
            request: impl tonic::IntoRequest<super::CountContractNullifierRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountContractNullifierResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mystiko.sequencer.v1.SequencerService/CountContractNullifier",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mystiko.sequencer.v1.SequencerService",
                        "CountContractNullifier",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn chain_loaded_block(
            &mut self,
            request: impl tonic::IntoRequest<super::ChainLoadedBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChainLoadedBlockResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mystiko.sequencer.v1.SequencerService/ChainLoadedBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mystiko.sequencer.v1.SequencerService",
                        "ChainLoadedBlock",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn contract_loaded_block(
            &mut self,
            request: impl tonic::IntoRequest<super::ContractLoadedBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ContractLoadedBlockResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mystiko.sequencer.v1.SequencerService/ContractLoadedBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mystiko.sequencer.v1.SequencerService",
                        "ContractLoadedBlock",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn health_check(
            &mut self,
            request: impl tonic::IntoRequest<super::HealthCheckRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HealthCheckResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mystiko.sequencer.v1.SequencerService/HealthCheck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mystiko.sequencer.v1.SequencerService",
                        "HealthCheck",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc-server")]
pub mod sequencer_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SequencerServiceServer.
    #[async_trait]
    pub trait SequencerService: Send + Sync + 'static {
        ///
        async fn fetch_chain(
            &self,
            request: tonic::Request<super::FetchChainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchChainResponse>,
            tonic::Status,
        >;
        ///
        async fn query_contract_commitment(
            &self,
            request: tonic::Request<super::QueryContractCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryContractCommitmentResponse>,
            tonic::Status,
        >;
        ///
        async fn count_contract_commitment(
            &self,
            request: tonic::Request<super::CountContractCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountContractCommitmentResponse>,
            tonic::Status,
        >;
        ///
        async fn query_contract_nullifier(
            &self,
            request: tonic::Request<super::QueryContractNullifierRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryContractNullifierResponse>,
            tonic::Status,
        >;
        ///
        async fn count_contract_nullifier(
            &self,
            request: tonic::Request<super::CountContractNullifierRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountContractNullifierResponse>,
            tonic::Status,
        >;
        ///
        async fn chain_loaded_block(
            &self,
            request: tonic::Request<super::ChainLoadedBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChainLoadedBlockResponse>,
            tonic::Status,
        >;
        ///
        async fn contract_loaded_block(
            &self,
            request: tonic::Request<super::ContractLoadedBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ContractLoadedBlockResponse>,
            tonic::Status,
        >;
        ///
        async fn health_check(
            &self,
            request: tonic::Request<super::HealthCheckRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HealthCheckResponse>,
            tonic::Status,
        >;
    }
    ///
    #[derive(Debug)]
    pub struct SequencerServiceServer<T: SequencerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SequencerService> SequencerServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SequencerServiceServer<T>
    where
        T: SequencerService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/mystiko.sequencer.v1.SequencerService/FetchChain" => {
                    #[allow(non_camel_case_types)]
                    struct FetchChainSvc<T: SequencerService>(pub Arc<T>);
                    impl<
                        T: SequencerService,
                    > tonic::server::UnaryService<super::FetchChainRequest>
                    for FetchChainSvc<T> {
                        type Response = super::FetchChainResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FetchChainRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).fetch_chain(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchChainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mystiko.sequencer.v1.SequencerService/QueryContractCommitment" => {
                    #[allow(non_camel_case_types)]
                    struct QueryContractCommitmentSvc<T: SequencerService>(pub Arc<T>);
                    impl<
                        T: SequencerService,
                    > tonic::server::UnaryService<super::QueryContractCommitmentRequest>
                    for QueryContractCommitmentSvc<T> {
                        type Response = super::QueryContractCommitmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryContractCommitmentRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_contract_commitment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryContractCommitmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mystiko.sequencer.v1.SequencerService/CountContractCommitment" => {
                    #[allow(non_camel_case_types)]
                    struct CountContractCommitmentSvc<T: SequencerService>(pub Arc<T>);
                    impl<
                        T: SequencerService,
                    > tonic::server::UnaryService<super::CountContractCommitmentRequest>
                    for CountContractCommitmentSvc<T> {
                        type Response = super::CountContractCommitmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CountContractCommitmentRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).count_contract_commitment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CountContractCommitmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mystiko.sequencer.v1.SequencerService/QueryContractNullifier" => {
                    #[allow(non_camel_case_types)]
                    struct QueryContractNullifierSvc<T: SequencerService>(pub Arc<T>);
                    impl<
                        T: SequencerService,
                    > tonic::server::UnaryService<super::QueryContractNullifierRequest>
                    for QueryContractNullifierSvc<T> {
                        type Response = super::QueryContractNullifierResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryContractNullifierRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_contract_nullifier(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryContractNullifierSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mystiko.sequencer.v1.SequencerService/CountContractNullifier" => {
                    #[allow(non_camel_case_types)]
                    struct CountContractNullifierSvc<T: SequencerService>(pub Arc<T>);
                    impl<
                        T: SequencerService,
                    > tonic::server::UnaryService<super::CountContractNullifierRequest>
                    for CountContractNullifierSvc<T> {
                        type Response = super::CountContractNullifierResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CountContractNullifierRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).count_contract_nullifier(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CountContractNullifierSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mystiko.sequencer.v1.SequencerService/ChainLoadedBlock" => {
                    #[allow(non_camel_case_types)]
                    struct ChainLoadedBlockSvc<T: SequencerService>(pub Arc<T>);
                    impl<
                        T: SequencerService,
                    > tonic::server::UnaryService<super::ChainLoadedBlockRequest>
                    for ChainLoadedBlockSvc<T> {
                        type Response = super::ChainLoadedBlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChainLoadedBlockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).chain_loaded_block(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChainLoadedBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mystiko.sequencer.v1.SequencerService/ContractLoadedBlock" => {
                    #[allow(non_camel_case_types)]
                    struct ContractLoadedBlockSvc<T: SequencerService>(pub Arc<T>);
                    impl<
                        T: SequencerService,
                    > tonic::server::UnaryService<super::ContractLoadedBlockRequest>
                    for ContractLoadedBlockSvc<T> {
                        type Response = super::ContractLoadedBlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContractLoadedBlockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).contract_loaded_block(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ContractLoadedBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mystiko.sequencer.v1.SequencerService/HealthCheck" => {
                    #[allow(non_camel_case_types)]
                    struct HealthCheckSvc<T: SequencerService>(pub Arc<T>);
                    impl<
                        T: SequencerService,
                    > tonic::server::UnaryService<super::HealthCheckRequest>
                    for HealthCheckSvc<T> {
                        type Response = super::HealthCheckResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HealthCheckRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).health_check(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HealthCheckSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: SequencerService> Clone for SequencerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: SequencerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SequencerService> tonic::server::NamedService for SequencerServiceServer<T> {
        const NAME: &'static str = "mystiko.sequencer.v1.SequencerService";
    }
}
