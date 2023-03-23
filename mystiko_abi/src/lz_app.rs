pub use lz_app::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod lz_app {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"param\",\"type\":\"string\",\"components\":[]}],\"type\":\"error\",\"name\":\"Invalid\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedRemote\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forceResumeReceive\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_chainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_configType\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getConfig\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isTrustedRemote\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"localLayerZeroChainId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lzEndpoint\",\"outputs\":[{\"internalType\":\"contract ILayerZeroEndpoint\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"lzReceive\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerLayerZeroChainId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_chainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_configType\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_config\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setConfig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_lzChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_lzEndpoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReceiveVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSendVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_peerLayerZeroChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_peerAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedRemote\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedRemoteLookup\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static LZAPP_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct LzApp<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for LzApp<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LzApp<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LzApp<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LzApp<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(LzApp)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> LzApp<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    LZAPP_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `forceResumeReceive` (0x42d65a8d) function
        pub fn force_resume_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 214, 90, 141], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfig` (0xf5ecbdbc) function
        pub fn get_config(
            &self,
            version: u16,
            chain_id: u16,
            p2: ::ethers_core::types::Address,
            config_type: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Bytes,
        > {
            self.0
                .method_hash([245, 236, 189, 188], (version, chain_id, p2, config_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isTrustedRemote` (0x3d8b38f6) function
        pub fn is_trusted_remote(
            &self,
            src_chain_id: u16,
            src_address: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 139, 56, 246], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `localLayerZeroChainId` (0x302d5f4b) function
        pub fn local_layer_zero_chain_id(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([48, 45, 95, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lzEndpoint` (0xb353aaa7) function
        pub fn lz_endpoint(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([179, 83, 170, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lzReceive` (0x001d3567) function
        pub fn lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers_core::types::Bytes,
            nonce: u64,
            payload: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 29, 53, 103],
                    (src_chain_id, src_address, nonce, payload),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerLayerZeroChainId` (0x0097a063) function
        pub fn peer_layer_zero_chain_id(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([0, 151, 160, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0xcbed8b9c) function
        pub fn set_config(
            &self,
            version: u16,
            chain_id: u16,
            config_type: ::ethers_core::types::U256,
            config: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [203, 237, 139, 156],
                    (version, chain_id, config_type, config),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEndpoint` (0x4ee7ded6) function
        pub fn set_endpoint(
            &self,
            lz_chain_id: u16,
            lz_endpoint: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 231, 222, 214], (lz_chain_id, lz_endpoint))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReceiveVersion` (0x10ddb137) function
        pub fn set_receive_version(
            &self,
            version: u16,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 221, 177, 55], version)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSendVersion` (0x07e0db17) function
        pub fn set_send_version(
            &self,
            version: u16,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 224, 219, 23], version)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTrustedRemote` (0xeb8d72b7) function
        pub fn set_trusted_remote(
            &self,
            peer_layer_zero_chain_id: u16,
            peer_address: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [235, 141, 114, 183],
                    (peer_layer_zero_chain_id, peer_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trustedRemoteLookup` (0x7533d788) function
        pub fn trusted_remote_lookup(
            &self,
            p0: u16,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Bytes,
        > {
            self.0
                .method_hash([117, 51, 215, 136], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetTrustedRemote` event
        pub fn set_trusted_remote_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetTrustedRemoteFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, LzAppEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for LzApp<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Invalid` with signature `Invalid(string)` and selector `0x53a2556c`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Invalid", abi = "Invalid(string)")]
    pub struct Invalid {
        pub param: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers_core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetTrustedRemote", abi = "SetTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum LzAppEvents {
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SetTrustedRemoteFilter(SetTrustedRemoteFilter),
    }
    impl ::ethers_contract::EthLogDecode for LzAppEvents {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(LzAppEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedRemoteFilter::decode_log(log) {
                return Ok(LzAppEvents::SetTrustedRemoteFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LzAppEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetTrustedRemoteFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for LzAppEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteFilter> for LzAppEvents {
        fn from(value: SetTrustedRemoteFilter) -> Self {
            Self::SetTrustedRemoteFilter(value)
        }
    }
    ///Container type for all input parameters for the `forceResumeReceive` function with signature `forceResumeReceive(uint16,bytes)` and selector `0x42d65a8d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "forceResumeReceive", abi = "forceResumeReceive(uint16,bytes)")]
    pub struct ForceResumeReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `0xf5ecbdbc`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getConfig", abi = "getConfig(uint16,uint16,address,uint256)")]
    pub struct GetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub p2: ::ethers_core::types::Address,
        pub config_type: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `0x3d8b38f6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isTrustedRemote", abi = "isTrustedRemote(uint16,bytes)")]
    pub struct IsTrustedRemoteCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `localLayerZeroChainId` function with signature `localLayerZeroChainId()` and selector `0x302d5f4b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "localLayerZeroChainId", abi = "localLayerZeroChainId()")]
    pub struct LocalLayerZeroChainIdCall;
    ///Container type for all input parameters for the `lzEndpoint` function with signature `lzEndpoint()` and selector `0xb353aaa7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lzEndpoint", abi = "lzEndpoint()")]
    pub struct LzEndpointCall;
    ///Container type for all input parameters for the `lzReceive` function with signature `lzReceive(uint16,bytes,uint64,bytes)` and selector `0x001d3567`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lzReceive", abi = "lzReceive(uint16,bytes,uint64,bytes)")]
    pub struct LzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `peerLayerZeroChainId` function with signature `peerLayerZeroChainId()` and selector `0x0097a063`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "peerLayerZeroChainId", abi = "peerLayerZeroChainId()")]
    pub struct PeerLayerZeroChainIdCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig(uint16,uint16,uint256,bytes)` and selector `0xcbed8b9c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setConfig", abi = "setConfig(uint16,uint16,uint256,bytes)")]
    pub struct SetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub config_type: ::ethers_core::types::U256,
        pub config: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `setEndpoint` function with signature `setEndpoint(uint16,address)` and selector `0x4ee7ded6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setEndpoint", abi = "setEndpoint(uint16,address)")]
    pub struct SetEndpointCall {
        pub lz_chain_id: u16,
        pub lz_endpoint: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `setReceiveVersion` function with signature `setReceiveVersion(uint16)` and selector `0x10ddb137`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setReceiveVersion", abi = "setReceiveVersion(uint16)")]
    pub struct SetReceiveVersionCall {
        pub version: u16,
    }
    ///Container type for all input parameters for the `setSendVersion` function with signature `setSendVersion(uint16)` and selector `0x07e0db17`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setSendVersion", abi = "setSendVersion(uint16)")]
    pub struct SetSendVersionCall {
        pub version: u16,
    }
    ///Container type for all input parameters for the `setTrustedRemote` function with signature `setTrustedRemote(uint16,bytes)` and selector `0xeb8d72b7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setTrustedRemote", abi = "setTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteCall {
        pub peer_layer_zero_chain_id: u16,
        pub peer_address: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `0x7533d788`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "trustedRemoteLookup", abi = "trustedRemoteLookup(uint16)")]
    pub struct TrustedRemoteLookupCall(pub u16);
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum LzAppCalls {
        ForceResumeReceive(ForceResumeReceiveCall),
        GetConfig(GetConfigCall),
        IsTrustedRemote(IsTrustedRemoteCall),
        LocalLayerZeroChainId(LocalLayerZeroChainIdCall),
        LzEndpoint(LzEndpointCall),
        LzReceive(LzReceiveCall),
        Owner(OwnerCall),
        PeerLayerZeroChainId(PeerLayerZeroChainIdCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetConfig(SetConfigCall),
        SetEndpoint(SetEndpointCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetSendVersion(SetSendVersionCall),
        SetTrustedRemote(SetTrustedRemoteCall),
        TransferOwnership(TransferOwnershipCall),
        TrustedRemoteLookup(TrustedRemoteLookupCall),
    }
    impl ::ethers_core::abi::AbiDecode for LzAppCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ForceResumeReceiveCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ForceResumeReceive(decoded));
            }
            if let Ok(decoded)
                = <GetConfigCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetConfig(decoded));
            }
            if let Ok(decoded)
                = <IsTrustedRemoteCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsTrustedRemote(decoded));
            }
            if let Ok(decoded)
                = <LocalLayerZeroChainIdCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LocalLayerZeroChainId(decoded));
            }
            if let Ok(decoded)
                = <LzEndpointCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LzEndpoint(decoded));
            }
            if let Ok(decoded)
                = <LzReceiveCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LzReceive(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PeerLayerZeroChainIdCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PeerLayerZeroChainId(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <SetConfigCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded)
                = <SetEndpointCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEndpoint(decoded));
            }
            if let Ok(decoded)
                = <SetReceiveVersionCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReceiveVersion(decoded));
            }
            if let Ok(decoded)
                = <SetSendVersionCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSendVersion(decoded));
            }
            if let Ok(decoded)
                = <SetTrustedRemoteCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetTrustedRemote(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <TrustedRemoteLookupCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TrustedRemoteLookup(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for LzAppCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ForceResumeReceive(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetConfig(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsTrustedRemote(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LocalLayerZeroChainId(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LzEndpoint(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LzReceive(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::PeerLayerZeroChainId(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetConfig(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetEndpoint(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetReceiveVersion(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetSendVersion(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetTrustedRemote(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::TrustedRemoteLookup(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LzAppCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ForceResumeReceive(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsTrustedRemote(element) => ::core::fmt::Display::fmt(element, f),
                Self::LocalLayerZeroChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LzEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::LzReceive(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerLayerZeroChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReceiveVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSendVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTrustedRemote(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustedRemoteLookup(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ForceResumeReceiveCall> for LzAppCalls {
        fn from(value: ForceResumeReceiveCall) -> Self {
            Self::ForceResumeReceive(value)
        }
    }
    impl ::core::convert::From<GetConfigCall> for LzAppCalls {
        fn from(value: GetConfigCall) -> Self {
            Self::GetConfig(value)
        }
    }
    impl ::core::convert::From<IsTrustedRemoteCall> for LzAppCalls {
        fn from(value: IsTrustedRemoteCall) -> Self {
            Self::IsTrustedRemote(value)
        }
    }
    impl ::core::convert::From<LocalLayerZeroChainIdCall> for LzAppCalls {
        fn from(value: LocalLayerZeroChainIdCall) -> Self {
            Self::LocalLayerZeroChainId(value)
        }
    }
    impl ::core::convert::From<LzEndpointCall> for LzAppCalls {
        fn from(value: LzEndpointCall) -> Self {
            Self::LzEndpoint(value)
        }
    }
    impl ::core::convert::From<LzReceiveCall> for LzAppCalls {
        fn from(value: LzReceiveCall) -> Self {
            Self::LzReceive(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LzAppCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PeerLayerZeroChainIdCall> for LzAppCalls {
        fn from(value: PeerLayerZeroChainIdCall) -> Self {
            Self::PeerLayerZeroChainId(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for LzAppCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for LzAppCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetEndpointCall> for LzAppCalls {
        fn from(value: SetEndpointCall) -> Self {
            Self::SetEndpoint(value)
        }
    }
    impl ::core::convert::From<SetReceiveVersionCall> for LzAppCalls {
        fn from(value: SetReceiveVersionCall) -> Self {
            Self::SetReceiveVersion(value)
        }
    }
    impl ::core::convert::From<SetSendVersionCall> for LzAppCalls {
        fn from(value: SetSendVersionCall) -> Self {
            Self::SetSendVersion(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteCall> for LzAppCalls {
        fn from(value: SetTrustedRemoteCall) -> Self {
            Self::SetTrustedRemote(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LzAppCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TrustedRemoteLookupCall> for LzAppCalls {
        fn from(value: TrustedRemoteLookupCall) -> Self {
            Self::TrustedRemoteLookup(value)
        }
    }
    ///Container type for all return fields from the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `0xf5ecbdbc`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetConfigReturn(pub ::ethers_core::types::Bytes);
    ///Container type for all return fields from the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `0x3d8b38f6`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsTrustedRemoteReturn(pub bool);
    ///Container type for all return fields from the `localLayerZeroChainId` function with signature `localLayerZeroChainId()` and selector `0x302d5f4b`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LocalLayerZeroChainIdReturn(pub u16);
    ///Container type for all return fields from the `lzEndpoint` function with signature `lzEndpoint()` and selector `0xb353aaa7`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LzEndpointReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `peerLayerZeroChainId` function with signature `peerLayerZeroChainId()` and selector `0x0097a063`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PeerLayerZeroChainIdReturn(pub u16);
    ///Container type for all return fields from the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `0x7533d788`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TrustedRemoteLookupReturn(pub ::ethers_core::types::Bytes);
}
