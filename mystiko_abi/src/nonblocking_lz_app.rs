pub use nonblocking_lz_app::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod nonblocking_lz_app {
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
    #[doc = "NonblockingLzApp was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"CallIsNotLzApp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"param\",\"type\":\"string\",\"components\":[]}],\"type\":\"error\",\"name\":\"Invalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NoStoredMessage\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MessageFailed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedRemote\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"failedMessages\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forceResumeReceive\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_chainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_configType\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getConfig\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isTrustedRemote\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"localLayerZeroChainId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lzEndpoint\",\"outputs\":[{\"internalType\":\"contract ILayerZeroEndpoint\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"lzReceive\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"nonblockingLzReceive\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerLayerZeroChainId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"retryMessage\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_chainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_configType\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_config\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setConfig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_lzChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_lzEndpoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReceiveVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSendVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_peerLayerZeroChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_peerAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedRemote\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedRemoteLookup\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static NONBLOCKINGLZAPP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct NonblockingLzApp<M>(ethers::contract::Contract<M>);
    impl<M> Clone for NonblockingLzApp<M> {
        fn clone(&self) -> Self {
            NonblockingLzApp(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for NonblockingLzApp<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for NonblockingLzApp<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(NonblockingLzApp))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> NonblockingLzApp<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), NONBLOCKINGLZAPP_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `failedMessages` (0x5b8c41e6) function"]
        pub fn failed_messages(
            &self,
            p0: u16,
            p1: ethers::core::types::Bytes,
            p2: u64,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([91, 140, 65, 230], (p0, p1, p2))
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
        #[doc = "Calls the contract's `getConfig` (0xf5ecbdbc) function"]
        pub fn get_config(
            &self,
            version: u16,
            chain_id: u16,
            p2: ethers::core::types::Address,
            config_type: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([245, 236, 189, 188], (version, chain_id, p2, config_type))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isTrustedRemote` (0x3d8b38f6) function"]
        pub fn is_trusted_remote(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 139, 56, 246], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `localLayerZeroChainId` (0x302d5f4b) function"]
        pub fn local_layer_zero_chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([48, 45, 95, 75], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lzEndpoint` (0xb353aaa7) function"]
        pub fn lz_endpoint(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([179, 83, 170, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lzReceive` (0x001d3567) function"]
        pub fn lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
            nonce: u64,
            payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 29, 53, 103],
                    (src_chain_id, src_address, nonce, payload),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonblockingLzReceive` (0x66ad5c8a) function"]
        pub fn nonblocking_lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
            nonce: u64,
            payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [102, 173, 92, 138],
                    (src_chain_id, src_address, nonce, payload),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `peerLayerZeroChainId` (0x0097a063) function"]
        pub fn peer_layer_zero_chain_id(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([0, 151, 160, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `retryMessage` (0xd1deba1f) function"]
        pub fn retry_message(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
            nonce: u64,
            payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [209, 222, 186, 31],
                    (src_chain_id, src_address, nonce, payload),
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
        #[doc = "Calls the contract's `setEndpoint` (0x4ee7ded6) function"]
        pub fn set_endpoint(
            &self,
            lz_chain_id: u16,
            lz_endpoint: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 231, 222, 214], (lz_chain_id, lz_endpoint))
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
        #[doc = "Calls the contract's `setTrustedRemote` (0xeb8d72b7) function"]
        pub fn set_trusted_remote(
            &self,
            peer_layer_zero_chain_id: u16,
            peer_address: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [235, 141, 114, 183],
                    (peer_layer_zero_chain_id, peer_address),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedRemoteLookup` (0x7533d788) function"]
        pub fn trusted_remote_lookup(
            &self,
            p0: u16,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([117, 51, 215, 136], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `MessageFailed` event"]
        pub fn message_failed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MessageFailedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetTrustedRemote` event"]
        pub fn set_trusted_remote_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetTrustedRemoteFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, NonblockingLzAppEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for NonblockingLzApp<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `CallIsNotLzApp` with signature `CallIsNotLzApp()` and selector `[227, 234, 29, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "CallIsNotLzApp", abi = "CallIsNotLzApp()")]
    pub struct CallIsNotLzApp;
    #[doc = "Custom Error type `Invalid` with signature `Invalid(string)` and selector `[83, 162, 85, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Invalid", abi = "Invalid(string)")]
    pub struct Invalid {
        pub param: String,
    }
    #[doc = "Custom Error type `NoStoredMessage` with signature `NoStoredMessage()` and selector `[174, 91, 38, 20]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NoStoredMessage", abi = "NoStoredMessage()")]
    pub struct NoStoredMessage;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum NonblockingLzAppErrors {
        CallIsNotLzApp(CallIsNotLzApp),
        Invalid(Invalid),
        NoStoredMessage(NoStoredMessage),
    }
    impl ethers::core::abi::AbiDecode for NonblockingLzAppErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CallIsNotLzApp as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppErrors::CallIsNotLzApp(decoded));
            }
            if let Ok(decoded) = <Invalid as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(NonblockingLzAppErrors::Invalid(decoded));
            }
            if let Ok(decoded) =
                <NoStoredMessage as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppErrors::NoStoredMessage(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for NonblockingLzAppErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                NonblockingLzAppErrors::CallIsNotLzApp(element) => element.encode(),
                NonblockingLzAppErrors::Invalid(element) => element.encode(),
                NonblockingLzAppErrors::NoStoredMessage(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for NonblockingLzAppErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                NonblockingLzAppErrors::CallIsNotLzApp(element) => element.fmt(f),
                NonblockingLzAppErrors::Invalid(element) => element.fmt(f),
                NonblockingLzAppErrors::NoStoredMessage(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CallIsNotLzApp> for NonblockingLzAppErrors {
        fn from(var: CallIsNotLzApp) -> Self {
            NonblockingLzAppErrors::CallIsNotLzApp(var)
        }
    }
    impl ::std::convert::From<Invalid> for NonblockingLzAppErrors {
        fn from(var: Invalid) -> Self {
            NonblockingLzAppErrors::Invalid(var)
        }
    }
    impl ::std::convert::From<NoStoredMessage> for NonblockingLzAppErrors {
        fn from(var: NoStoredMessage) -> Self {
            NonblockingLzAppErrors::NoStoredMessage(var)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "MessageFailed",
        abi = "MessageFailed(uint16,bytes,uint64,bytes)"
    )]
    pub struct MessageFailedFilter {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetTrustedRemote", abi = "SetTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteFilter {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum NonblockingLzAppEvents {
        MessageFailedFilter(MessageFailedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SetTrustedRemoteFilter(SetTrustedRemoteFilter),
    }
    impl ethers::contract::EthLogDecode for NonblockingLzAppEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = MessageFailedFilter::decode_log(log) {
                return Ok(NonblockingLzAppEvents::MessageFailedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(NonblockingLzAppEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedRemoteFilter::decode_log(log) {
                return Ok(NonblockingLzAppEvents::SetTrustedRemoteFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for NonblockingLzAppEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                NonblockingLzAppEvents::MessageFailedFilter(element) => element.fmt(f),
                NonblockingLzAppEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                NonblockingLzAppEvents::SetTrustedRemoteFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `failedMessages` function with signature `failedMessages(uint16,bytes,uint64)` and selector `[91, 140, 65, 230]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "failedMessages", abi = "failedMessages(uint16,bytes,uint64)")]
    pub struct FailedMessagesCall(pub u16, pub ethers::core::types::Bytes, pub u64);
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
        pub p2: ethers::core::types::Address,
        pub config_type: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `[61, 139, 56, 246]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isTrustedRemote", abi = "isTrustedRemote(uint16,bytes)")]
    pub struct IsTrustedRemoteCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `localLayerZeroChainId` function with signature `localLayerZeroChainId()` and selector `[48, 45, 95, 75]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "localLayerZeroChainId", abi = "localLayerZeroChainId()")]
    pub struct LocalLayerZeroChainIdCall;
    #[doc = "Container type for all input parameters for the `lzEndpoint` function with signature `lzEndpoint()` and selector `[179, 83, 170, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lzEndpoint", abi = "lzEndpoint()")]
    pub struct LzEndpointCall;
    #[doc = "Container type for all input parameters for the `lzReceive` function with signature `lzReceive(uint16,bytes,uint64,bytes)` and selector `[0, 29, 53, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lzReceive", abi = "lzReceive(uint16,bytes,uint64,bytes)")]
    pub struct LzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `nonblockingLzReceive` function with signature `nonblockingLzReceive(uint16,bytes,uint64,bytes)` and selector `[102, 173, 92, 138]`"]
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
        name = "nonblockingLzReceive",
        abi = "nonblockingLzReceive(uint16,bytes,uint64,bytes)"
    )]
    pub struct NonblockingLzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `peerLayerZeroChainId` function with signature `peerLayerZeroChainId()` and selector `[0, 151, 160, 99]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "peerLayerZeroChainId", abi = "peerLayerZeroChainId()")]
    pub struct PeerLayerZeroChainIdCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `retryMessage` function with signature `retryMessage(uint16,bytes,uint64,bytes)` and selector `[209, 222, 186, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "retryMessage", abi = "retryMessage(uint16,bytes,uint64,bytes)")]
    pub struct RetryMessageCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `setEndpoint` function with signature `setEndpoint(uint16,address)` and selector `[78, 231, 222, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setEndpoint", abi = "setEndpoint(uint16,address)")]
    pub struct SetEndpointCall {
        pub lz_chain_id: u16,
        pub lz_endpoint: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `setTrustedRemote` function with signature `setTrustedRemote(uint16,bytes)` and selector `[235, 141, 114, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTrustedRemote", abi = "setTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteCall {
        pub peer_layer_zero_chain_id: u16,
        pub peer_address: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `[117, 51, 215, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "trustedRemoteLookup", abi = "trustedRemoteLookup(uint16)")]
    pub struct TrustedRemoteLookupCall(pub u16);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum NonblockingLzAppCalls {
        FailedMessages(FailedMessagesCall),
        ForceResumeReceive(ForceResumeReceiveCall),
        GetConfig(GetConfigCall),
        IsTrustedRemote(IsTrustedRemoteCall),
        LocalLayerZeroChainId(LocalLayerZeroChainIdCall),
        LzEndpoint(LzEndpointCall),
        LzReceive(LzReceiveCall),
        NonblockingLzReceive(NonblockingLzReceiveCall),
        Owner(OwnerCall),
        PeerLayerZeroChainId(PeerLayerZeroChainIdCall),
        RenounceOwnership(RenounceOwnershipCall),
        RetryMessage(RetryMessageCall),
        SetConfig(SetConfigCall),
        SetEndpoint(SetEndpointCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetSendVersion(SetSendVersionCall),
        SetTrustedRemote(SetTrustedRemoteCall),
        TransferOwnership(TransferOwnershipCall),
        TrustedRemoteLookup(TrustedRemoteLookupCall),
    }
    impl ethers::core::abi::AbiDecode for NonblockingLzAppCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FailedMessagesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::FailedMessages(decoded));
            }
            if let Ok(decoded) =
                <ForceResumeReceiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::ForceResumeReceive(decoded));
            }
            if let Ok(decoded) =
                <GetConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::GetConfig(decoded));
            }
            if let Ok(decoded) =
                <IsTrustedRemoteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::IsTrustedRemote(decoded));
            }
            if let Ok(decoded) =
                <LocalLayerZeroChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::LocalLayerZeroChainId(decoded));
            }
            if let Ok(decoded) =
                <LzEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::LzEndpoint(decoded));
            }
            if let Ok(decoded) =
                <LzReceiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::LzReceive(decoded));
            }
            if let Ok(decoded) =
                <NonblockingLzReceiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::NonblockingLzReceive(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PeerLayerZeroChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::PeerLayerZeroChainId(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RetryMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::RetryMessage(decoded));
            }
            if let Ok(decoded) =
                <SetConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::SetConfig(decoded));
            }
            if let Ok(decoded) =
                <SetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::SetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <SetReceiveVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::SetReceiveVersion(decoded));
            }
            if let Ok(decoded) =
                <SetSendVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::SetSendVersion(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedRemoteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::SetTrustedRemote(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TrustedRemoteLookupCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NonblockingLzAppCalls::TrustedRemoteLookup(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for NonblockingLzAppCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                NonblockingLzAppCalls::FailedMessages(element) => element.encode(),
                NonblockingLzAppCalls::ForceResumeReceive(element) => element.encode(),
                NonblockingLzAppCalls::GetConfig(element) => element.encode(),
                NonblockingLzAppCalls::IsTrustedRemote(element) => element.encode(),
                NonblockingLzAppCalls::LocalLayerZeroChainId(element) => element.encode(),
                NonblockingLzAppCalls::LzEndpoint(element) => element.encode(),
                NonblockingLzAppCalls::LzReceive(element) => element.encode(),
                NonblockingLzAppCalls::NonblockingLzReceive(element) => element.encode(),
                NonblockingLzAppCalls::Owner(element) => element.encode(),
                NonblockingLzAppCalls::PeerLayerZeroChainId(element) => element.encode(),
                NonblockingLzAppCalls::RenounceOwnership(element) => element.encode(),
                NonblockingLzAppCalls::RetryMessage(element) => element.encode(),
                NonblockingLzAppCalls::SetConfig(element) => element.encode(),
                NonblockingLzAppCalls::SetEndpoint(element) => element.encode(),
                NonblockingLzAppCalls::SetReceiveVersion(element) => element.encode(),
                NonblockingLzAppCalls::SetSendVersion(element) => element.encode(),
                NonblockingLzAppCalls::SetTrustedRemote(element) => element.encode(),
                NonblockingLzAppCalls::TransferOwnership(element) => element.encode(),
                NonblockingLzAppCalls::TrustedRemoteLookup(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for NonblockingLzAppCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                NonblockingLzAppCalls::FailedMessages(element) => element.fmt(f),
                NonblockingLzAppCalls::ForceResumeReceive(element) => element.fmt(f),
                NonblockingLzAppCalls::GetConfig(element) => element.fmt(f),
                NonblockingLzAppCalls::IsTrustedRemote(element) => element.fmt(f),
                NonblockingLzAppCalls::LocalLayerZeroChainId(element) => element.fmt(f),
                NonblockingLzAppCalls::LzEndpoint(element) => element.fmt(f),
                NonblockingLzAppCalls::LzReceive(element) => element.fmt(f),
                NonblockingLzAppCalls::NonblockingLzReceive(element) => element.fmt(f),
                NonblockingLzAppCalls::Owner(element) => element.fmt(f),
                NonblockingLzAppCalls::PeerLayerZeroChainId(element) => element.fmt(f),
                NonblockingLzAppCalls::RenounceOwnership(element) => element.fmt(f),
                NonblockingLzAppCalls::RetryMessage(element) => element.fmt(f),
                NonblockingLzAppCalls::SetConfig(element) => element.fmt(f),
                NonblockingLzAppCalls::SetEndpoint(element) => element.fmt(f),
                NonblockingLzAppCalls::SetReceiveVersion(element) => element.fmt(f),
                NonblockingLzAppCalls::SetSendVersion(element) => element.fmt(f),
                NonblockingLzAppCalls::SetTrustedRemote(element) => element.fmt(f),
                NonblockingLzAppCalls::TransferOwnership(element) => element.fmt(f),
                NonblockingLzAppCalls::TrustedRemoteLookup(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FailedMessagesCall> for NonblockingLzAppCalls {
        fn from(var: FailedMessagesCall) -> Self {
            NonblockingLzAppCalls::FailedMessages(var)
        }
    }
    impl ::std::convert::From<ForceResumeReceiveCall> for NonblockingLzAppCalls {
        fn from(var: ForceResumeReceiveCall) -> Self {
            NonblockingLzAppCalls::ForceResumeReceive(var)
        }
    }
    impl ::std::convert::From<GetConfigCall> for NonblockingLzAppCalls {
        fn from(var: GetConfigCall) -> Self {
            NonblockingLzAppCalls::GetConfig(var)
        }
    }
    impl ::std::convert::From<IsTrustedRemoteCall> for NonblockingLzAppCalls {
        fn from(var: IsTrustedRemoteCall) -> Self {
            NonblockingLzAppCalls::IsTrustedRemote(var)
        }
    }
    impl ::std::convert::From<LocalLayerZeroChainIdCall> for NonblockingLzAppCalls {
        fn from(var: LocalLayerZeroChainIdCall) -> Self {
            NonblockingLzAppCalls::LocalLayerZeroChainId(var)
        }
    }
    impl ::std::convert::From<LzEndpointCall> for NonblockingLzAppCalls {
        fn from(var: LzEndpointCall) -> Self {
            NonblockingLzAppCalls::LzEndpoint(var)
        }
    }
    impl ::std::convert::From<LzReceiveCall> for NonblockingLzAppCalls {
        fn from(var: LzReceiveCall) -> Self {
            NonblockingLzAppCalls::LzReceive(var)
        }
    }
    impl ::std::convert::From<NonblockingLzReceiveCall> for NonblockingLzAppCalls {
        fn from(var: NonblockingLzReceiveCall) -> Self {
            NonblockingLzAppCalls::NonblockingLzReceive(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for NonblockingLzAppCalls {
        fn from(var: OwnerCall) -> Self {
            NonblockingLzAppCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PeerLayerZeroChainIdCall> for NonblockingLzAppCalls {
        fn from(var: PeerLayerZeroChainIdCall) -> Self {
            NonblockingLzAppCalls::PeerLayerZeroChainId(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for NonblockingLzAppCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            NonblockingLzAppCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RetryMessageCall> for NonblockingLzAppCalls {
        fn from(var: RetryMessageCall) -> Self {
            NonblockingLzAppCalls::RetryMessage(var)
        }
    }
    impl ::std::convert::From<SetConfigCall> for NonblockingLzAppCalls {
        fn from(var: SetConfigCall) -> Self {
            NonblockingLzAppCalls::SetConfig(var)
        }
    }
    impl ::std::convert::From<SetEndpointCall> for NonblockingLzAppCalls {
        fn from(var: SetEndpointCall) -> Self {
            NonblockingLzAppCalls::SetEndpoint(var)
        }
    }
    impl ::std::convert::From<SetReceiveVersionCall> for NonblockingLzAppCalls {
        fn from(var: SetReceiveVersionCall) -> Self {
            NonblockingLzAppCalls::SetReceiveVersion(var)
        }
    }
    impl ::std::convert::From<SetSendVersionCall> for NonblockingLzAppCalls {
        fn from(var: SetSendVersionCall) -> Self {
            NonblockingLzAppCalls::SetSendVersion(var)
        }
    }
    impl ::std::convert::From<SetTrustedRemoteCall> for NonblockingLzAppCalls {
        fn from(var: SetTrustedRemoteCall) -> Self {
            NonblockingLzAppCalls::SetTrustedRemote(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for NonblockingLzAppCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            NonblockingLzAppCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<TrustedRemoteLookupCall> for NonblockingLzAppCalls {
        fn from(var: TrustedRemoteLookupCall) -> Self {
            NonblockingLzAppCalls::TrustedRemoteLookup(var)
        }
    }
    #[doc = "Container type for all return fields from the `failedMessages` function with signature `failedMessages(uint16,bytes,uint64)` and selector `[91, 140, 65, 230]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FailedMessagesReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `[61, 139, 56, 246]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsTrustedRemoteReturn(pub bool);
    #[doc = "Container type for all return fields from the `localLayerZeroChainId` function with signature `localLayerZeroChainId()` and selector `[48, 45, 95, 75]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LocalLayerZeroChainIdReturn(pub u16);
    #[doc = "Container type for all return fields from the `lzEndpoint` function with signature `lzEndpoint()` and selector `[179, 83, 170, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LzEndpointReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `peerLayerZeroChainId` function with signature `peerLayerZeroChainId()` and selector `[0, 151, 160, 99]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PeerLayerZeroChainIdReturn(pub u16);
    #[doc = "Container type for all return fields from the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `[117, 51, 215, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TrustedRemoteLookupReturn(pub ethers::core::types::Bytes);
}
