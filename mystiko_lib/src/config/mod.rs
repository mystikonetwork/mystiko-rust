use crate::runtime;
use anyhow::Result;
use mystiko_protos::api::config::v1::{
    FindAssetSymbolsRequest, FindAssetSymbolsResponse, FindBridgeRequest, FindBridgeResponse, FindBridgesRequest,
    FindBridgesResponse, FindChainRequest, FindChainResponse, FindCircuitRequest, FindCircuitResponse,
    FindContractByAddressRequest, FindContractByAddressResponse, FindDefaultCircuitRequest, FindDefaultCircuitResponse,
    FindDepositContractByAddressRequest, FindDepositContractByAddressResponse, FindDepositContractRequest,
    FindDepositContractResponse, FindPeerChainsRequest, FindPeerChainsResponse, FindPoolContractByAddressRequest,
    FindPoolContractByAddressResponse, FindPoolContractRequest, FindPoolContractResponse, FindPoolContractsRequest,
    FindPoolContractsResponse, GetConfigRequest, GetConfigResponse, GetTransactionUrlRequest,
    GetTransactionUrlResponse,
};
use mystiko_protos::api::v1::{api_response, ApiResponse, StatusCode};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::config::contract::v1::PoolContractConfig;
use mystiko_protos::config::v1::ChainConfig;
use prost::Message;

pub fn get<M>(_message: M) -> ApiResponse
where
    M: TryInto<GetConfigRequest>,
    <M as TryInto<GetConfigRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    runtime().block_on(async {
        match internal::get().await {
            Ok(response) => ApiResponse::builder()
                .code(StatusCode::Success)
                .result(api_response::Result::Data(response.encode_to_vec()))
                .build(),
            Err(err) => ApiResponse::builder()
                .code(StatusCode::UnknownError)
                .result(api_response::Result::ErrorMessage(err.to_string()))
                .build(),
        }
    })
}

pub fn find_default_circuit<M>(message: M) -> ApiResponse
where
    M: TryInto<FindDefaultCircuitRequest>,
    <M as TryInto<FindDefaultCircuitRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_default_circuit(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_circuit<M>(message: M) -> ApiResponse
where
    M: TryInto<FindCircuitRequest>,
    <M as TryInto<FindCircuitRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_circuit(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_chain<M>(message: M) -> ApiResponse
where
    M: TryInto<FindChainRequest>,
    <M as TryInto<FindChainRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_chain(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_peer_chains<M>(message: M) -> ApiResponse
where
    M: TryInto<FindPeerChainsRequest>,
    <M as TryInto<FindPeerChainsRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_peer_chains(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_asset_symbols<M>(message: M) -> ApiResponse
where
    M: TryInto<FindAssetSymbolsRequest>,
    <M as TryInto<FindAssetSymbolsRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_asset_symbols(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_bridges<M>(message: M) -> ApiResponse
where
    M: TryInto<FindBridgesRequest>,
    <M as TryInto<FindBridgesRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_bridges(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_bridge<M>(message: M) -> ApiResponse
where
    M: TryInto<FindBridgeRequest>,
    <M as TryInto<FindBridgeRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_bridge(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_deposit_contract<M>(message: M) -> ApiResponse
where
    M: TryInto<FindDepositContractRequest>,
    <M as TryInto<FindDepositContractRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_deposit_contract(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_deposit_contract_by_address<M>(message: M) -> ApiResponse
where
    M: TryInto<FindDepositContractByAddressRequest>,
    <M as TryInto<FindDepositContractByAddressRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_deposit_contract_by_address(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_pool_contract<M>(message: M) -> ApiResponse
where
    M: TryInto<FindPoolContractRequest>,
    <M as TryInto<FindPoolContractRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_pool_contract(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_pool_contracts<M>(message: M) -> ApiResponse
where
    M: TryInto<FindPoolContractsRequest>,
    <M as TryInto<FindPoolContractsRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_pool_contracts(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_pool_contract_by_address<M>(message: M) -> ApiResponse
where
    M: TryInto<FindPoolContractByAddressRequest>,
    <M as TryInto<FindPoolContractByAddressRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_pool_contract_by_address(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn find_contract_by_address<M>(message: M) -> ApiResponse
where
    M: TryInto<FindContractByAddressRequest>,
    <M as TryInto<FindContractByAddressRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::find_contract_by_address(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

pub fn get_transaction_url<M>(message: M) -> ApiResponse
where
    M: TryInto<GetTransactionUrlRequest>,
    <M as TryInto<GetTransactionUrlRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    let response = match message.try_into() {
        Ok(message) => runtime().block_on(async {
            internal::get_transaction_url(message).await.map(|data| {
                ApiResponse::builder()
                    .code(StatusCode::Success)
                    .result(api_response::Result::Data(data.encode_to_vec()))
                    .build()
            })
        }),
        Err(err) => Ok(ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()),
    };

    response.unwrap_or_else(|err| {
        ApiResponse::builder()
            .code(StatusCode::UnknownError)
            .result(api_response::Result::ErrorMessage(err.to_string()))
            .build()
    })
}

mod internal {
    use super::*;
    use crate::instance;

    pub(crate) async fn get() -> Result<GetConfigResponse> {
        let mystiko_guard = instance().read().await;
        mystiko_guard
            .get()?
            .config
            .to_proto()
            .map(|c| GetConfigResponse::builder().config(c).build())
    }

    pub(crate) async fn find_default_circuit(request: FindDefaultCircuitRequest) -> Result<FindDefaultCircuitResponse> {
        let mystiko_guard = instance().read().await;
        let config = mystiko_guard
            .get()?
            .config
            .find_default_circuit(&request.circuit_type().into())
            .map(|c| c.to_proto());
        Ok(FindDefaultCircuitResponse::builder().config(config).build())
    }

    pub(crate) async fn find_circuit(request: FindCircuitRequest) -> Result<FindCircuitResponse> {
        let mystiko_guard = instance().read().await;
        let config = mystiko_guard
            .get()?
            .config
            .find_circuit(&request.circuit_name)
            .map(|c| c.to_proto());
        Ok(FindCircuitResponse::builder().config(config).build())
    }

    pub(crate) async fn find_chain(request: FindChainRequest) -> Result<FindChainResponse> {
        let mystiko_guard = instance().read().await;
        let config = mystiko_guard
            .get()?
            .config
            .find_chain(request.chain_id)
            .map(|c| c.to_proto())
            .transpose()?;
        Ok(FindChainResponse::builder().config(config).build())
    }

    pub(crate) async fn find_peer_chains(request: FindPeerChainsRequest) -> Result<FindPeerChainsResponse> {
        let mystiko_guard = instance().read().await;
        Ok(FindPeerChainsResponse::builder()
            .configs(
                mystiko_guard
                    .get()?
                    .config
                    .find_peer_chains(request.chain_id)
                    .into_iter()
                    .map(|c| c.to_proto())
                    .collect::<Result<Vec<ChainConfig>>>()?,
            )
            .build())
    }

    pub(crate) async fn find_asset_symbols(request: FindAssetSymbolsRequest) -> Result<FindAssetSymbolsResponse> {
        let mystiko_guard = instance().read().await;
        Ok(FindAssetSymbolsResponse::builder()
            .asset_symbol(
                mystiko_guard
                    .get()?
                    .config
                    .find_asset_symbols(request.chain_id, request.peer_chain_id)
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
            .build())
    }

    pub(crate) async fn find_bridges(request: FindBridgesRequest) -> Result<FindBridgesResponse> {
        let mystiko_guard = instance().read().await;
        Ok(FindBridgesResponse::builder()
            .bridge_type(
                mystiko_guard
                    .get()?
                    .config
                    .find_bridges(request.chain_id, request.peer_chain_id, &request.asset_symbol)
                    .into_iter()
                    .map(|b| Into::<BridgeType>::into(b) as i32)
                    .collect::<Vec<i32>>(),
            )
            .build())
    }

    pub(crate) async fn find_bridge(request: FindBridgeRequest) -> Result<FindBridgeResponse> {
        let mystiko_guard = instance().read().await;
        Ok(FindBridgeResponse::builder()
            .config(
                mystiko_guard
                    .get()?
                    .config
                    .find_bridge(&request.bridge_type().into())
                    .map(|c| c.to_proto()),
            )
            .build())
    }

    pub(crate) async fn find_deposit_contract(
        request: FindDepositContractRequest,
    ) -> Result<FindDepositContractResponse> {
        let mystiko_guard = instance().read().await;
        let config = mystiko_guard
            .get()?
            .config
            .find_deposit_contract(
                request.chain_id,
                request.peer_chain_id,
                &request.asset_symbol,
                &request.bridge_type().into(),
            )
            .map(|c| c.to_proto())
            .transpose()?;
        Ok(FindDepositContractResponse::builder().config(config).build())
    }

    pub(crate) async fn find_deposit_contract_by_address(
        request: FindDepositContractByAddressRequest,
    ) -> Result<FindDepositContractByAddressResponse> {
        let mystiko_guard = instance().read().await;
        let config = mystiko_guard
            .get()?
            .config
            .find_deposit_contract_by_address(request.chain_id, &request.address)
            .map(|c| c.to_proto())
            .transpose()?;
        Ok(FindDepositContractByAddressResponse::builder().config(config).build())
    }

    pub(crate) async fn find_pool_contract(request: FindPoolContractRequest) -> Result<FindPoolContractResponse> {
        let mystiko_guard = instance().read().await;
        let config = mystiko_guard
            .get()?
            .config
            .find_pool_contract(
                request.chain_id,
                &request.asset_symbol,
                &request.bridge_type().into(),
                request.version,
            )
            .map(|c| c.to_proto())
            .transpose()?;
        Ok(FindPoolContractResponse::builder().config(config).build())
    }

    pub(crate) async fn find_pool_contracts(request: FindPoolContractsRequest) -> Result<FindPoolContractsResponse> {
        let mystiko_guard = instance().read().await;
        let config = mystiko_guard
            .get()?
            .config
            .find_pool_contracts(request.chain_id, &request.asset_symbol, &request.bridge_type().into())
            .into_iter()
            .map(|c| c.to_proto())
            .collect::<Result<Vec<PoolContractConfig>>>()?;
        Ok(FindPoolContractsResponse::builder().config(config).build())
    }

    pub(crate) async fn find_pool_contract_by_address(
        request: FindPoolContractByAddressRequest,
    ) -> Result<FindPoolContractByAddressResponse> {
        let mystiko_guard = instance().read().await;
        let config = mystiko_guard
            .get()?
            .config
            .find_pool_contract_by_address(request.chain_id, &request.address)
            .map(|c| c.to_proto())
            .transpose()?;
        Ok(FindPoolContractByAddressResponse::builder().config(config).build())
    }

    pub(crate) async fn find_contract_by_address(
        request: FindContractByAddressRequest,
    ) -> Result<FindContractByAddressResponse> {
        let mystiko_guard = instance().read().await;
        let config = mystiko_guard
            .get()?
            .config
            .find_contract_by_address(request.chain_id, &request.address)
            .map(|c| c.to_proto())
            .transpose()?;
        Ok(FindContractByAddressResponse::builder().config(config).build())
    }

    pub(crate) async fn get_transaction_url(request: GetTransactionUrlRequest) -> Result<GetTransactionUrlResponse> {
        let mystiko_guard = instance().read().await;
        Ok(GetTransactionUrlResponse::builder()
            .url(
                mystiko_guard
                    .get()?
                    .config
                    .transaction_url(request.chain_id, &request.tx_hash),
            )
            .build())
    }
}
