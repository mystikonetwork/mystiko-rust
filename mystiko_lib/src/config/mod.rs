use crate::runtime;
use anyhow::Result;
use mystiko_protos::common::v1::{BridgeType, CircuitType};
use mystiko_protos::config::api::v1::{
    FindAssetSymbolsRequest, FindAssetSymbolsResponse, FindBridgeRequest, FindBridgeResponse, FindBridgesRequest,
    FindBridgesResponse, FindChainRequest, FindChainResponse, FindCircuitRequest, FindCircuitResponse,
    FindContractByAddressRequest, FindContractByAddressResponse, FindDefaultCircuitRequest, FindDefaultCircuitResponse,
    FindDepositContractByAddressRequest, FindDepositContractByAddressResponse, FindDepositContractRequest,
    FindDepositContractResponse, FindPeerChainsRequest, FindPeerChainsResponse, FindPoolContractByAddressRequest,
    FindPoolContractByAddressResponse, FindPoolContractRequest, FindPoolContractResponse, FindPoolContractsRequest,
    FindPoolContractsResponse, GetConfigResponse, GetTransactionUrlRequest, GetTransactionUrlResponse,
};
use mystiko_protos::config::bridge::v1::BridgeConfig;
use mystiko_protos::config::contract::v1::{ContractConfig, DepositContractConfig, PoolContractConfig};
use mystiko_protos::config::v1::{ChainConfig, CircuitConfig, MystikoConfig};

pub fn get() -> Result<GetConfigResponse> {
    runtime().block_on(internal::get())
}

pub fn find_default_circuit(request: FindDefaultCircuitRequest) -> Result<FindDefaultCircuitResponse> {
    runtime().block_on(internal::find_default_circuit(request))
}

pub fn find_circuit(request: FindCircuitRequest) -> Result<FindCircuitResponse> {
    runtime().block_on(internal::find_circuit(request))
}

pub fn find_chain(request: FindChainRequest) -> Result<FindChainResponse> {
    runtime().block_on(internal::find_chain(request))
}

pub fn find_peer_chains(request: FindPeerChainsRequest) -> Result<FindPeerChainsResponse> {
    runtime().block_on(internal::find_peer_chains(request))
}

pub fn find_asset_symbols(request: FindAssetSymbolsRequest) -> Result<FindAssetSymbolsResponse> {
    runtime().block_on(internal::find_asset_symbols(request))
}

pub fn find_bridges(request: FindBridgesRequest) -> Result<FindBridgesResponse> {
    runtime().block_on(internal::find_bridges(request))
}

pub fn find_bridge(request: FindBridgeRequest) -> Result<FindBridgeResponse> {
    runtime().block_on(internal::find_bridge(request))
}

pub fn find_deposit_contract(request: FindDepositContractRequest) -> Result<FindDepositContractResponse> {
    runtime().block_on(internal::find_deposit_contract(request))
}

pub fn find_deposit_contract_by_address(
    request: FindDepositContractByAddressRequest,
) -> Result<FindDepositContractByAddressResponse> {
    runtime().block_on(internal::find_deposit_contract_by_address(request))
}

pub fn find_pool_contract(request: FindPoolContractRequest) -> Result<FindPoolContractResponse> {
    runtime().block_on(internal::find_pool_contract(request))
}

pub fn find_pool_contracts(request: FindPoolContractsRequest) -> Result<FindPoolContractsResponse> {
    runtime().block_on(internal::find_pool_contracts(request))
}

pub fn find_pool_contract_by_address(
    request: FindPoolContractByAddressRequest,
) -> Result<FindPoolContractByAddressResponse> {
    runtime().block_on(internal::find_pool_contract_by_address(request))
}

pub fn find_contract_by_address(request: FindContractByAddressRequest) -> Result<FindContractByAddressResponse> {
    runtime().block_on(internal::find_contract_by_address(request))
}

pub fn get_transaction_url(request: GetTransactionUrlRequest) -> Result<GetTransactionUrlResponse> {
    runtime().block_on(internal::get_transaction_url(request))
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
