pub use i_layer_zero_endpoint::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_layer_zero_endpoint {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ILayerZeroEndpoint was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_dstChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_userApplication\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_payInZRO\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_adapterParam\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"estimateFees\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nativeFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"zroFee\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forceResumeReceive\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_chainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_userApplication\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_configType\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getConfig\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInboundNonce\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_dstChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_srcAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOutboundNonce\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_userApplication\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReceiveLibraryAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_userApplication\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReceiveVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_userApplication\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSendLibraryAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_userApplication\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSendVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasStoredPayload\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isReceivingPayload\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSendingPayload\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_dstAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"receivePayload\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"retryPayload\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_dstChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_destination\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"_refundAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_zroPaymentAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_adapterParams\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"send\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_chainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_configType\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_config\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setConfig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReceiveVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSendVersion\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ILAYERZEROENDPOINT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ILayerZeroEndpoint<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ILayerZeroEndpoint<M> {
        fn clone(&self) -> Self {
            ILayerZeroEndpoint(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ILayerZeroEndpoint<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ILayerZeroEndpoint<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ILayerZeroEndpoint))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ILayerZeroEndpoint<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ILAYERZEROENDPOINT_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `estimateFees` (0x40a7bb10) function"]
        pub fn estimate_fees(
            &self,
            dst_chain_id: u16,
            user_application: ethers::core::types::Address,
            payload: ethers::core::types::Bytes,
            pay_in_zro: bool,
            adapter_param: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [64, 167, 187, 16],
                    (
                        dst_chain_id,
                        user_application,
                        payload,
                        pay_in_zro,
                        adapter_param,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forceResumeReceive` (0x42d65a8d) function"]
        pub fn force_resume_receive(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 214, 90, 141], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChainId` (0x3408e470) function"]
        pub fn get_chain_id(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getConfig` (0xf5ecbdbc) function"]
        pub fn get_config(
            &self,
            version: u16,
            chain_id: u16,
            user_application: ethers::core::types::Address,
            config_type: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash(
                    [245, 236, 189, 188],
                    (version, chain_id, user_application, config_type),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getInboundNonce` (0xfdc07c70) function"]
        pub fn get_inbound_nonce(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([253, 192, 124, 112], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOutboundNonce` (0x7a145748) function"]
        pub fn get_outbound_nonce(
            &self,
            dst_chain_id: u16,
            src_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([122, 20, 87, 72], (dst_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReceiveLibraryAddress` (0x71ba2fd6) function"]
        pub fn get_receive_library_address(
            &self,
            user_application: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([113, 186, 47, 214], user_application)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReceiveVersion` (0xda1a7c9a) function"]
        pub fn get_receive_version(
            &self,
            user_application: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([218, 26, 124, 154], user_application)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSendLibraryAddress` (0x9c729da1) function"]
        pub fn get_send_library_address(
            &self,
            user_application: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([156, 114, 157, 161], user_application)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSendVersion` (0x096568f6) function"]
        pub fn get_send_version(
            &self,
            user_application: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([9, 101, 104, 246], user_application)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasStoredPayload` (0x0eaf6ea6) function"]
        pub fn has_stored_payload(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 175, 110, 166], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isReceivingPayload` (0xca066b35) function"]
        pub fn is_receiving_payload(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([202, 6, 107, 53], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSendingPayload` (0xe97a448a) function"]
        pub fn is_sending_payload(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 122, 68, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `receivePayload` (0xc2fa4813) function"]
        pub fn receive_payload(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
            dst_address: ethers::core::types::Address,
            nonce: u64,
            gas_limit: ethers::core::types::U256,
            payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [194, 250, 72, 19],
                    (
                        src_chain_id,
                        src_address,
                        dst_address,
                        nonce,
                        gas_limit,
                        payload,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `retryPayload` (0xaaff5f16) function"]
        pub fn retry_payload(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
            payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 255, 95, 22], (src_chain_id, src_address, payload))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `send` (0xc5803100) function"]
        pub fn send(
            &self,
            dst_chain_id: u16,
            destination: ethers::core::types::Bytes,
            payload: ethers::core::types::Bytes,
            refund_address: ethers::core::types::Address,
            zro_payment_address: ethers::core::types::Address,
            adapter_params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [197, 128, 49, 0],
                    (
                        dst_chain_id,
                        destination,
                        payload,
                        refund_address,
                        zro_payment_address,
                        adapter_params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setConfig` (0xcbed8b9c) function"]
        pub fn set_config(
            &self,
            version: u16,
            chain_id: u16,
            config_type: ethers::core::types::U256,
            config: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [203, 237, 139, 156],
                    (version, chain_id, config_type, config),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReceiveVersion` (0x10ddb137) function"]
        pub fn set_receive_version(
            &self,
            version: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 221, 177, 55], version)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSendVersion` (0x07e0db17) function"]
        pub fn set_send_version(
            &self,
            version: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 224, 219, 23], version)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ILayerZeroEndpoint<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `estimateFees` function with signature `estimateFees(uint16,address,bytes,bool,bytes)` and selector `[64, 167, 187, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "estimateFees",
        abi = "estimateFees(uint16,address,bytes,bool,bytes)"
    )]
    pub struct EstimateFeesCall {
        pub dst_chain_id: u16,
        pub user_application: ethers::core::types::Address,
        pub payload: ethers::core::types::Bytes,
        pub pay_in_zro: bool,
        pub adapter_param: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `forceResumeReceive` function with signature `forceResumeReceive(uint16,bytes)` and selector `[66, 214, 90, 141]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "forceResumeReceive", abi = "forceResumeReceive(uint16,bytes)")]
    pub struct ForceResumeReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    #[doc = "Container type for all input parameters for the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `[245, 236, 189, 188]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getConfig", abi = "getConfig(uint16,uint16,address,uint256)")]
    pub struct GetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub user_application: ethers::core::types::Address,
        pub config_type: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getInboundNonce` function with signature `getInboundNonce(uint16,bytes)` and selector `[253, 192, 124, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getInboundNonce", abi = "getInboundNonce(uint16,bytes)")]
    pub struct GetInboundNonceCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getOutboundNonce` function with signature `getOutboundNonce(uint16,address)` and selector `[122, 20, 87, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getOutboundNonce", abi = "getOutboundNonce(uint16,address)")]
    pub struct GetOutboundNonceCall {
        pub dst_chain_id: u16,
        pub src_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReceiveLibraryAddress` function with signature `getReceiveLibraryAddress(address)` and selector `[113, 186, 47, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getReceiveLibraryAddress",
        abi = "getReceiveLibraryAddress(address)"
    )]
    pub struct GetReceiveLibraryAddressCall {
        pub user_application: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReceiveVersion` function with signature `getReceiveVersion(address)` and selector `[218, 26, 124, 154]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getReceiveVersion", abi = "getReceiveVersion(address)")]
    pub struct GetReceiveVersionCall {
        pub user_application: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getSendLibraryAddress` function with signature `getSendLibraryAddress(address)` and selector `[156, 114, 157, 161]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getSendLibraryAddress", abi = "getSendLibraryAddress(address)")]
    pub struct GetSendLibraryAddressCall {
        pub user_application: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getSendVersion` function with signature `getSendVersion(address)` and selector `[9, 101, 104, 246]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getSendVersion", abi = "getSendVersion(address)")]
    pub struct GetSendVersionCall {
        pub user_application: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hasStoredPayload` function with signature `hasStoredPayload(uint16,bytes)` and selector `[14, 175, 110, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hasStoredPayload", abi = "hasStoredPayload(uint16,bytes)")]
    pub struct HasStoredPayloadCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `isReceivingPayload` function with signature `isReceivingPayload()` and selector `[202, 6, 107, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isReceivingPayload", abi = "isReceivingPayload()")]
    pub struct IsReceivingPayloadCall;
    #[doc = "Container type for all input parameters for the `isSendingPayload` function with signature `isSendingPayload()` and selector `[233, 122, 68, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isSendingPayload", abi = "isSendingPayload()")]
    pub struct IsSendingPayloadCall;
    #[doc = "Container type for all input parameters for the `receivePayload` function with signature `receivePayload(uint16,bytes,address,uint64,uint256,bytes)` and selector `[194, 250, 72, 19]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "receivePayload",
        abi = "receivePayload(uint16,bytes,address,uint64,uint256,bytes)"
    )]
    pub struct ReceivePayloadCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub dst_address: ethers::core::types::Address,
        pub nonce: u64,
        pub gas_limit: ethers::core::types::U256,
        pub payload: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `retryPayload` function with signature `retryPayload(uint16,bytes,bytes)` and selector `[170, 255, 95, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "retryPayload", abi = "retryPayload(uint16,bytes,bytes)")]
    pub struct RetryPayloadCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub payload: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `send` function with signature `send(uint16,bytes,bytes,address,address,bytes)` and selector `[197, 128, 49, 0]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "send", abi = "send(uint16,bytes,bytes,address,address,bytes)")]
    pub struct SendCall {
        pub dst_chain_id: u16,
        pub destination: ethers::core::types::Bytes,
        pub payload: ethers::core::types::Bytes,
        pub refund_address: ethers::core::types::Address,
        pub zro_payment_address: ethers::core::types::Address,
        pub adapter_params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setConfig` function with signature `setConfig(uint16,uint16,uint256,bytes)` and selector `[203, 237, 139, 156]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setConfig", abi = "setConfig(uint16,uint16,uint256,bytes)")]
    pub struct SetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub config_type: ethers::core::types::U256,
        pub config: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setReceiveVersion` function with signature `setReceiveVersion(uint16)` and selector `[16, 221, 177, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setReceiveVersion", abi = "setReceiveVersion(uint16)")]
    pub struct SetReceiveVersionCall {
        pub version: u16,
    }
    #[doc = "Container type for all input parameters for the `setSendVersion` function with signature `setSendVersion(uint16)` and selector `[7, 224, 219, 23]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setSendVersion", abi = "setSendVersion(uint16)")]
    pub struct SetSendVersionCall {
        pub version: u16,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ILayerZeroEndpointCalls {
        EstimateFees(EstimateFeesCall),
        ForceResumeReceive(ForceResumeReceiveCall),
        GetChainId(GetChainIdCall),
        GetConfig(GetConfigCall),
        GetInboundNonce(GetInboundNonceCall),
        GetOutboundNonce(GetOutboundNonceCall),
        GetReceiveLibraryAddress(GetReceiveLibraryAddressCall),
        GetReceiveVersion(GetReceiveVersionCall),
        GetSendLibraryAddress(GetSendLibraryAddressCall),
        GetSendVersion(GetSendVersionCall),
        HasStoredPayload(HasStoredPayloadCall),
        IsReceivingPayload(IsReceivingPayloadCall),
        IsSendingPayload(IsSendingPayloadCall),
        ReceivePayload(ReceivePayloadCall),
        RetryPayload(RetryPayloadCall),
        Send(SendCall),
        SetConfig(SetConfigCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetSendVersion(SetSendVersionCall),
    }
    impl ethers::core::abi::AbiDecode for ILayerZeroEndpointCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <EstimateFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::EstimateFees(decoded));
            }
            if let Ok(decoded) =
                <ForceResumeReceiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::ForceResumeReceive(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::GetConfig(decoded));
            }
            if let Ok(decoded) =
                <GetInboundNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::GetInboundNonce(decoded));
            }
            if let Ok(decoded) =
                <GetOutboundNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::GetOutboundNonce(decoded));
            }
            if let Ok(decoded) =
                <GetReceiveLibraryAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ILayerZeroEndpointCalls::GetReceiveLibraryAddress(decoded));
            }
            if let Ok(decoded) =
                <GetReceiveVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::GetReceiveVersion(decoded));
            }
            if let Ok(decoded) =
                <GetSendLibraryAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::GetSendLibraryAddress(decoded));
            }
            if let Ok(decoded) =
                <GetSendVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::GetSendVersion(decoded));
            }
            if let Ok(decoded) =
                <HasStoredPayloadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::HasStoredPayload(decoded));
            }
            if let Ok(decoded) =
                <IsReceivingPayloadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::IsReceivingPayload(decoded));
            }
            if let Ok(decoded) =
                <IsSendingPayloadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::IsSendingPayload(decoded));
            }
            if let Ok(decoded) =
                <ReceivePayloadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::ReceivePayload(decoded));
            }
            if let Ok(decoded) =
                <RetryPayloadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::RetryPayload(decoded));
            }
            if let Ok(decoded) = <SendCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ILayerZeroEndpointCalls::Send(decoded));
            }
            if let Ok(decoded) =
                <SetConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::SetConfig(decoded));
            }
            if let Ok(decoded) =
                <SetReceiveVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::SetReceiveVersion(decoded));
            }
            if let Ok(decoded) =
                <SetSendVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroEndpointCalls::SetSendVersion(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ILayerZeroEndpointCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ILayerZeroEndpointCalls::EstimateFees(element) => element.encode(),
                ILayerZeroEndpointCalls::ForceResumeReceive(element) => element.encode(),
                ILayerZeroEndpointCalls::GetChainId(element) => element.encode(),
                ILayerZeroEndpointCalls::GetConfig(element) => element.encode(),
                ILayerZeroEndpointCalls::GetInboundNonce(element) => element.encode(),
                ILayerZeroEndpointCalls::GetOutboundNonce(element) => element.encode(),
                ILayerZeroEndpointCalls::GetReceiveLibraryAddress(element) => element.encode(),
                ILayerZeroEndpointCalls::GetReceiveVersion(element) => element.encode(),
                ILayerZeroEndpointCalls::GetSendLibraryAddress(element) => element.encode(),
                ILayerZeroEndpointCalls::GetSendVersion(element) => element.encode(),
                ILayerZeroEndpointCalls::HasStoredPayload(element) => element.encode(),
                ILayerZeroEndpointCalls::IsReceivingPayload(element) => element.encode(),
                ILayerZeroEndpointCalls::IsSendingPayload(element) => element.encode(),
                ILayerZeroEndpointCalls::ReceivePayload(element) => element.encode(),
                ILayerZeroEndpointCalls::RetryPayload(element) => element.encode(),
                ILayerZeroEndpointCalls::Send(element) => element.encode(),
                ILayerZeroEndpointCalls::SetConfig(element) => element.encode(),
                ILayerZeroEndpointCalls::SetReceiveVersion(element) => element.encode(),
                ILayerZeroEndpointCalls::SetSendVersion(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ILayerZeroEndpointCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ILayerZeroEndpointCalls::EstimateFees(element) => element.fmt(f),
                ILayerZeroEndpointCalls::ForceResumeReceive(element) => element.fmt(f),
                ILayerZeroEndpointCalls::GetChainId(element) => element.fmt(f),
                ILayerZeroEndpointCalls::GetConfig(element) => element.fmt(f),
                ILayerZeroEndpointCalls::GetInboundNonce(element) => element.fmt(f),
                ILayerZeroEndpointCalls::GetOutboundNonce(element) => element.fmt(f),
                ILayerZeroEndpointCalls::GetReceiveLibraryAddress(element) => element.fmt(f),
                ILayerZeroEndpointCalls::GetReceiveVersion(element) => element.fmt(f),
                ILayerZeroEndpointCalls::GetSendLibraryAddress(element) => element.fmt(f),
                ILayerZeroEndpointCalls::GetSendVersion(element) => element.fmt(f),
                ILayerZeroEndpointCalls::HasStoredPayload(element) => element.fmt(f),
                ILayerZeroEndpointCalls::IsReceivingPayload(element) => element.fmt(f),
                ILayerZeroEndpointCalls::IsSendingPayload(element) => element.fmt(f),
                ILayerZeroEndpointCalls::ReceivePayload(element) => element.fmt(f),
                ILayerZeroEndpointCalls::RetryPayload(element) => element.fmt(f),
                ILayerZeroEndpointCalls::Send(element) => element.fmt(f),
                ILayerZeroEndpointCalls::SetConfig(element) => element.fmt(f),
                ILayerZeroEndpointCalls::SetReceiveVersion(element) => element.fmt(f),
                ILayerZeroEndpointCalls::SetSendVersion(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<EstimateFeesCall> for ILayerZeroEndpointCalls {
        fn from(var: EstimateFeesCall) -> Self {
            ILayerZeroEndpointCalls::EstimateFees(var)
        }
    }
    impl ::std::convert::From<ForceResumeReceiveCall> for ILayerZeroEndpointCalls {
        fn from(var: ForceResumeReceiveCall) -> Self {
            ILayerZeroEndpointCalls::ForceResumeReceive(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for ILayerZeroEndpointCalls {
        fn from(var: GetChainIdCall) -> Self {
            ILayerZeroEndpointCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GetConfigCall> for ILayerZeroEndpointCalls {
        fn from(var: GetConfigCall) -> Self {
            ILayerZeroEndpointCalls::GetConfig(var)
        }
    }
    impl ::std::convert::From<GetInboundNonceCall> for ILayerZeroEndpointCalls {
        fn from(var: GetInboundNonceCall) -> Self {
            ILayerZeroEndpointCalls::GetInboundNonce(var)
        }
    }
    impl ::std::convert::From<GetOutboundNonceCall> for ILayerZeroEndpointCalls {
        fn from(var: GetOutboundNonceCall) -> Self {
            ILayerZeroEndpointCalls::GetOutboundNonce(var)
        }
    }
    impl ::std::convert::From<GetReceiveLibraryAddressCall> for ILayerZeroEndpointCalls {
        fn from(var: GetReceiveLibraryAddressCall) -> Self {
            ILayerZeroEndpointCalls::GetReceiveLibraryAddress(var)
        }
    }
    impl ::std::convert::From<GetReceiveVersionCall> for ILayerZeroEndpointCalls {
        fn from(var: GetReceiveVersionCall) -> Self {
            ILayerZeroEndpointCalls::GetReceiveVersion(var)
        }
    }
    impl ::std::convert::From<GetSendLibraryAddressCall> for ILayerZeroEndpointCalls {
        fn from(var: GetSendLibraryAddressCall) -> Self {
            ILayerZeroEndpointCalls::GetSendLibraryAddress(var)
        }
    }
    impl ::std::convert::From<GetSendVersionCall> for ILayerZeroEndpointCalls {
        fn from(var: GetSendVersionCall) -> Self {
            ILayerZeroEndpointCalls::GetSendVersion(var)
        }
    }
    impl ::std::convert::From<HasStoredPayloadCall> for ILayerZeroEndpointCalls {
        fn from(var: HasStoredPayloadCall) -> Self {
            ILayerZeroEndpointCalls::HasStoredPayload(var)
        }
    }
    impl ::std::convert::From<IsReceivingPayloadCall> for ILayerZeroEndpointCalls {
        fn from(var: IsReceivingPayloadCall) -> Self {
            ILayerZeroEndpointCalls::IsReceivingPayload(var)
        }
    }
    impl ::std::convert::From<IsSendingPayloadCall> for ILayerZeroEndpointCalls {
        fn from(var: IsSendingPayloadCall) -> Self {
            ILayerZeroEndpointCalls::IsSendingPayload(var)
        }
    }
    impl ::std::convert::From<ReceivePayloadCall> for ILayerZeroEndpointCalls {
        fn from(var: ReceivePayloadCall) -> Self {
            ILayerZeroEndpointCalls::ReceivePayload(var)
        }
    }
    impl ::std::convert::From<RetryPayloadCall> for ILayerZeroEndpointCalls {
        fn from(var: RetryPayloadCall) -> Self {
            ILayerZeroEndpointCalls::RetryPayload(var)
        }
    }
    impl ::std::convert::From<SendCall> for ILayerZeroEndpointCalls {
        fn from(var: SendCall) -> Self {
            ILayerZeroEndpointCalls::Send(var)
        }
    }
    impl ::std::convert::From<SetConfigCall> for ILayerZeroEndpointCalls {
        fn from(var: SetConfigCall) -> Self {
            ILayerZeroEndpointCalls::SetConfig(var)
        }
    }
    impl ::std::convert::From<SetReceiveVersionCall> for ILayerZeroEndpointCalls {
        fn from(var: SetReceiveVersionCall) -> Self {
            ILayerZeroEndpointCalls::SetReceiveVersion(var)
        }
    }
    impl ::std::convert::From<SetSendVersionCall> for ILayerZeroEndpointCalls {
        fn from(var: SetSendVersionCall) -> Self {
            ILayerZeroEndpointCalls::SetSendVersion(var)
        }
    }
    #[doc = "Container type for all return fields from the `estimateFees` function with signature `estimateFees(uint16,address,bytes,bool,bytes)` and selector `[64, 167, 187, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct EstimateFeesReturn {
        pub native_fee: ethers::core::types::U256,
        pub zro_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetChainIdReturn(pub u16);
    #[doc = "Container type for all return fields from the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `[245, 236, 189, 188]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetConfigReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `getInboundNonce` function with signature `getInboundNonce(uint16,bytes)` and selector `[253, 192, 124, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetInboundNonceReturn(pub u64);
    #[doc = "Container type for all return fields from the `getOutboundNonce` function with signature `getOutboundNonce(uint16,address)` and selector `[122, 20, 87, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetOutboundNonceReturn(pub u64);
    #[doc = "Container type for all return fields from the `getReceiveLibraryAddress` function with signature `getReceiveLibraryAddress(address)` and selector `[113, 186, 47, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReceiveLibraryAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getReceiveVersion` function with signature `getReceiveVersion(address)` and selector `[218, 26, 124, 154]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReceiveVersionReturn(pub u16);
    #[doc = "Container type for all return fields from the `getSendLibraryAddress` function with signature `getSendLibraryAddress(address)` and selector `[156, 114, 157, 161]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetSendLibraryAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getSendVersion` function with signature `getSendVersion(address)` and selector `[9, 101, 104, 246]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetSendVersionReturn(pub u16);
    #[doc = "Container type for all return fields from the `hasStoredPayload` function with signature `hasStoredPayload(uint16,bytes)` and selector `[14, 175, 110, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HasStoredPayloadReturn(pub bool);
    #[doc = "Container type for all return fields from the `isReceivingPayload` function with signature `isReceivingPayload()` and selector `[202, 6, 107, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsReceivingPayloadReturn(pub bool);
    #[doc = "Container type for all return fields from the `isSendingPayload` function with signature `isSendingPayload()` and selector `[233, 122, 68, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsSendingPayloadReturn(pub bool);
}
