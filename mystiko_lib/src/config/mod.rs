use crate::runtime;
use anyhow::Result;
use mystiko_protos::api::config::v1::{
    FindAssetSymbolsRequest, FindAssetSymbolsResponse, FindBridgeRequest, FindBridgeResponse, FindBridgesRequest,
    FindBridgesResponse, FindChainRequest, FindChainResponse, FindCircuitRequest, FindContractByAddressRequest,
    FindContractByAddressResponse, FindDefaultCircuitRequest, FindDefaultCircuitResponse,
    FindDepositContractByAddressRequest, FindDepositContractByAddressResponse, FindDepositContractRequest,
    FindDepositContractResponse, FindPeerChainsRequest, FindPeerChainsResponse, FindPoolContractByAddressRequest,
    FindPoolContractByAddressResponse, FindPoolContractRequest, FindPoolContractResponse, FindPoolContractsRequest,
    FindPoolContractsResponse, GetConfigResponse, GetTransactionUrlRequest, GetTransactionUrlResponse,
};
use mystiko_protos::api::v1::{ApiResponse, ConfigError};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::config::contract::v1::PoolContractConfig;
use mystiko_protos::config::v1::ChainConfig;

pub fn get() -> ApiResponse {
    runtime().block_on(internal::get())
}

pub fn find_default_circuit<M>(message: M) -> ApiResponse
where
    M: TryInto<FindDefaultCircuitRequest>,
    <M as TryInto<FindDefaultCircuitRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_default_circuit(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_circuit<M>(message: M) -> ApiResponse
where
    M: TryInto<FindCircuitRequest>,
    <M as TryInto<FindCircuitRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_circuit(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_chain<M>(message: M) -> ApiResponse
where
    M: TryInto<FindChainRequest>,
    <M as TryInto<FindChainRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_chain(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_peer_chains<M>(message: M) -> ApiResponse
where
    M: TryInto<FindPeerChainsRequest>,
    <M as TryInto<FindPeerChainsRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_peer_chains(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_asset_symbols<M>(message: M) -> ApiResponse
where
    M: TryInto<FindAssetSymbolsRequest>,
    <M as TryInto<FindAssetSymbolsRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_asset_symbols(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_bridges<M>(message: M) -> ApiResponse
where
    M: TryInto<FindBridgesRequest>,
    <M as TryInto<FindBridgesRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_bridges(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_bridge<M>(message: M) -> ApiResponse
where
    M: TryInto<FindBridgeRequest>,
    <M as TryInto<FindBridgeRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_bridge(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_deposit_contract<M>(message: M) -> ApiResponse
where
    M: TryInto<FindDepositContractRequest>,
    <M as TryInto<FindDepositContractRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_deposit_contract(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_deposit_contract_by_address<M>(message: M) -> ApiResponse
where
    M: TryInto<FindDepositContractByAddressRequest>,
    <M as TryInto<FindDepositContractByAddressRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_deposit_contract_by_address(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_pool_contract<M>(message: M) -> ApiResponse
where
    M: TryInto<FindPoolContractRequest>,
    <M as TryInto<FindPoolContractRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_pool_contract(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_pool_contracts<M>(message: M) -> ApiResponse
where
    M: TryInto<FindPoolContractsRequest>,
    <M as TryInto<FindPoolContractsRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_pool_contracts(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_pool_contract_by_address<M>(message: M) -> ApiResponse
where
    M: TryInto<FindPoolContractByAddressRequest>,
    <M as TryInto<FindPoolContractByAddressRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_pool_contract_by_address(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn find_contract_by_address<M>(message: M) -> ApiResponse
where
    M: TryInto<FindContractByAddressRequest>,
    <M as TryInto<FindContractByAddressRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::find_contract_by_address(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

pub fn get_transaction_url<M>(message: M) -> ApiResponse
where
    M: TryInto<GetTransactionUrlRequest>,
    <M as TryInto<GetTransactionUrlRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(request) => runtime().block_on(internal::get_transaction_url(request)),
        Err(err) => ApiResponse::error(ConfigError::DeserializeMessageError, err),
    }
}

mod internal {
    use super::*;
    use crate::instance;

    pub(crate) async fn get() -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.config.to_proto();
                match result {
                    Ok(mystiko_config) => {
                        ApiResponse::success(GetConfigResponse::builder().config(mystiko_config).build())
                    }
                    Err(err) => ApiResponse::unknown_error(err),
                }
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_default_circuit(request: FindDefaultCircuitRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let config = mystiko
                    .config
                    .find_default_circuit(&request.circuit_type().into())
                    .map(|c| c.to_proto());
                ApiResponse::success(FindDefaultCircuitResponse::builder().config(config).build())
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_circuit(request: FindCircuitRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let config = mystiko.config.find_circuit(&request.circuit_name).map(|c| c.to_proto());
                ApiResponse::success(FindDefaultCircuitResponse::builder().config(config).build())
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_chain(request: FindChainRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko
                    .config
                    .find_chain(request.chain_id)
                    .map(|c| c.to_proto())
                    .transpose();
                match result {
                    Ok(config) => ApiResponse::success(FindChainResponse::builder().config(config).build()),
                    Err(err) => ApiResponse::unknown_error(err),
                }
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_peer_chains(request: FindPeerChainsRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko
                    .config
                    .find_peer_chains(request.chain_id)
                    .into_iter()
                    .map(|c| c.to_proto())
                    .collect::<Result<Vec<ChainConfig>>>();
                match result {
                    Ok(configs) => ApiResponse::success(FindPeerChainsResponse::builder().configs(configs).build()),
                    Err(err) => ApiResponse::unknown_error(err),
                }
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_asset_symbols(request: FindAssetSymbolsRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let symbols = mystiko
                    .config
                    .find_asset_symbols(request.chain_id, request.peer_chain_id)
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                ApiResponse::success(FindAssetSymbolsResponse::builder().asset_symbol(symbols).build())
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_bridges(request: FindBridgesRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let bridges = mystiko
                    .config
                    .find_bridges(request.chain_id, request.peer_chain_id, &request.asset_symbol)
                    .into_iter()
                    .map(|b| Into::<BridgeType>::into(b) as i32)
                    .collect::<Vec<i32>>();
                ApiResponse::success(FindBridgesResponse::builder().bridge_type(bridges).build())
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_bridge(request: FindBridgeRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let config = mystiko
                    .config
                    .find_bridge(&request.bridge_type().into())
                    .map(|c| c.to_proto());
                ApiResponse::success(FindBridgeResponse::builder().config(config).build())
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_deposit_contract(request: FindDepositContractRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko
                    .config
                    .find_deposit_contract(
                        request.chain_id,
                        request.peer_chain_id,
                        &request.asset_symbol,
                        &request.bridge_type().into(),
                    )
                    .map(|c| c.to_proto())
                    .transpose();
                match result {
                    Ok(config) => ApiResponse::success(FindDepositContractResponse::builder().config(config).build()),
                    Err(err) => ApiResponse::unknown_error(err),
                }
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_deposit_contract_by_address(request: FindDepositContractByAddressRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko
                    .config
                    .find_deposit_contract_by_address(request.chain_id, &request.address)
                    .map(|c| c.to_proto())
                    .transpose();
                match result {
                    Ok(config) => {
                        ApiResponse::success(FindDepositContractByAddressResponse::builder().config(config).build())
                    }
                    Err(err) => ApiResponse::unknown_error(err),
                }
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_pool_contract(request: FindPoolContractRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko
                    .config
                    .find_pool_contract(
                        request.chain_id,
                        &request.asset_symbol,
                        &request.bridge_type().into(),
                        request.version,
                    )
                    .map(|c| c.to_proto())
                    .transpose();
                match result {
                    Ok(config) => ApiResponse::success(FindPoolContractResponse::builder().config(config).build()),
                    Err(err) => ApiResponse::unknown_error(err),
                }
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_pool_contracts(request: FindPoolContractsRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko
                    .config
                    .find_pool_contracts(request.chain_id, &request.asset_symbol, &request.bridge_type().into())
                    .into_iter()
                    .map(|c| c.to_proto())
                    .collect::<Result<Vec<PoolContractConfig>>>();
                match result {
                    Ok(config) => ApiResponse::success(FindPoolContractsResponse::builder().config(config).build()),
                    Err(err) => ApiResponse::unknown_error(err),
                }
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_pool_contract_by_address(request: FindPoolContractByAddressRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko
                    .config
                    .find_pool_contract_by_address(request.chain_id, &request.address)
                    .map(|c| c.to_proto())
                    .transpose();
                match result {
                    Ok(config) => {
                        ApiResponse::success(FindPoolContractByAddressResponse::builder().config(config).build())
                    }
                    Err(err) => ApiResponse::unknown_error(err),
                }
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_contract_by_address(request: FindContractByAddressRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko
                    .config
                    .find_contract_by_address(request.chain_id, &request.address)
                    .map(|c| c.to_proto())
                    .transpose();
                match result {
                    Ok(config) => ApiResponse::success(FindContractByAddressResponse::builder().config(config).build()),
                    Err(err) => ApiResponse::unknown_error(err),
                }
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn get_transaction_url(request: GetTransactionUrlRequest) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let url = mystiko.config.transaction_url(request.chain_id, &request.tx_hash);
                ApiResponse::success(GetTransactionUrlResponse::builder().url(url).build())
            }
            Err(err) => ApiResponse::error(ConfigError::GetMystikoGuardError, err),
        }
    }
}
