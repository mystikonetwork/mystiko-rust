pub use mystiko_t_bridge_proxy::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mystiko_t_bridge_proxy {
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
    #[doc = "MystikoTBridgeProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CallCrossChainSyncTxError\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOperator\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyRegister\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyWhitelistedExecutor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"WithdrawFailed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"toContract\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"toChainId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"fromContract\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TBridgeCrossChainMessage\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_executor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addExecutorWhitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_register\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addRegisterWhitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newOperator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeOperator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"_fromChainId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_fromContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_toContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_executor\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"crossChainSyncTx\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_executor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeExecutorWhitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_register\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeRegisterWhitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_toContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_toChainId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sendMessage\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MYSTIKOTBRIDGEPROXY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct MystikoTBridgeProxy<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MystikoTBridgeProxy<M> {
        fn clone(&self) -> Self {
            MystikoTBridgeProxy(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MystikoTBridgeProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MystikoTBridgeProxy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MystikoTBridgeProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MystikoTBridgeProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MYSTIKOTBRIDGEPROXY_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `addExecutorWhitelist` (0xa071e9b1) function"]
        pub fn add_executor_whitelist(
            &self,
            executor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 113, 233, 177], executor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addRegisterWhitelist` (0xbfdfd563) function"]
        pub fn add_register_whitelist(
            &self,
            register: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 223, 213, 99], register)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeOperator` (0x06394c9b) function"]
        pub fn change_operator(
            &self,
            new_operator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 57, 76, 155], new_operator)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `crossChainSyncTx` (0x919c1df5) function"]
        pub fn cross_chain_sync_tx(
            &self,
            from_chain_id: u64,
            from_contract: ethers::core::types::Address,
            to_contract: ethers::core::types::Address,
            executor: ethers::core::types::Address,
            message: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [145, 156, 29, 245],
                    (from_chain_id, from_contract, to_contract, executor, message),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeExecutorWhitelist` (0xd1520948) function"]
        pub fn remove_executor_whitelist(
            &self,
            executor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 82, 9, 72], executor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeRegisterWhitelist` (0xf919d469) function"]
        pub fn remove_register_whitelist(
            &self,
            register: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 25, 212, 105], register)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sendMessage` (0xc81739cd) function"]
        pub fn send_message(
            &self,
            to_contract: ethers::core::types::Address,
            to_chain_id: u64,
            message: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 23, 57, 205], (to_contract, to_chain_id, message))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x51cff8d9) function"]
        pub fn withdraw(
            &self,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 207, 248, 217], recipient)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `TBridgeCrossChainMessage` event"]
        pub fn t_bridge_cross_chain_message_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TbridgeCrossChainMessageFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, TbridgeCrossChainMessageFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MystikoTBridgeProxy<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `CallCrossChainSyncTxError` with signature `CallCrossChainSyncTxError()` and selector `[189, 115, 14, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(
        name = "CallCrossChainSyncTxError",
        abi = "CallCrossChainSyncTxError()"
    )]
    pub struct CallCrossChainSyncTxError;
    #[doc = "Custom Error type `OnlyOperator` with signature `OnlyOperator()` and selector `[39, 225, 241, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(name = "OnlyOperator", abi = "OnlyOperator()")]
    pub struct OnlyOperator;
    #[doc = "Custom Error type `OnlyRegister` with signature `OnlyRegister()` and selector `[90, 240, 177, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(name = "OnlyRegister", abi = "OnlyRegister()")]
    pub struct OnlyRegister;
    #[doc = "Custom Error type `OnlyWhitelistedExecutor` with signature `OnlyWhitelistedExecutor()` and selector `[0, 68, 122, 100]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(name = "OnlyWhitelistedExecutor", abi = "OnlyWhitelistedExecutor()")]
    pub struct OnlyWhitelistedExecutor;
    #[doc = "Custom Error type `WithdrawFailed` with signature `WithdrawFailed()` and selector `[117, 11, 33, 156]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(name = "WithdrawFailed", abi = "WithdrawFailed()")]
    pub struct WithdrawFailed;
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum MystikoTBridgeProxyErrors {
        CallCrossChainSyncTxError(CallCrossChainSyncTxError),
        OnlyOperator(OnlyOperator),
        OnlyRegister(OnlyRegister),
        OnlyWhitelistedExecutor(OnlyWhitelistedExecutor),
        WithdrawFailed(WithdrawFailed),
    }
    impl ethers::core::abi::AbiDecode for MystikoTBridgeProxyErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CallCrossChainSyncTxError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyErrors::CallCrossChainSyncTxError(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <OnlyOperator as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyErrors::OnlyOperator(decoded));
            }
            if let Ok(decoded) =
                <OnlyRegister as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyErrors::OnlyRegister(decoded));
            }
            if let Ok(decoded) =
                <OnlyWhitelistedExecutor as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyErrors::OnlyWhitelistedExecutor(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFailed as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyErrors::WithdrawFailed(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MystikoTBridgeProxyErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                MystikoTBridgeProxyErrors::CallCrossChainSyncTxError(element) => element.encode(),
                MystikoTBridgeProxyErrors::OnlyOperator(element) => element.encode(),
                MystikoTBridgeProxyErrors::OnlyRegister(element) => element.encode(),
                MystikoTBridgeProxyErrors::OnlyWhitelistedExecutor(element) => element.encode(),
                MystikoTBridgeProxyErrors::WithdrawFailed(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MystikoTBridgeProxyErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoTBridgeProxyErrors::CallCrossChainSyncTxError(element) => element.fmt(f),
                MystikoTBridgeProxyErrors::OnlyOperator(element) => element.fmt(f),
                MystikoTBridgeProxyErrors::OnlyRegister(element) => element.fmt(f),
                MystikoTBridgeProxyErrors::OnlyWhitelistedExecutor(element) => element.fmt(f),
                MystikoTBridgeProxyErrors::WithdrawFailed(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CallCrossChainSyncTxError> for MystikoTBridgeProxyErrors {
        fn from(var: CallCrossChainSyncTxError) -> Self {
            MystikoTBridgeProxyErrors::CallCrossChainSyncTxError(var)
        }
    }
    impl ::std::convert::From<OnlyOperator> for MystikoTBridgeProxyErrors {
        fn from(var: OnlyOperator) -> Self {
            MystikoTBridgeProxyErrors::OnlyOperator(var)
        }
    }
    impl ::std::convert::From<OnlyRegister> for MystikoTBridgeProxyErrors {
        fn from(var: OnlyRegister) -> Self {
            MystikoTBridgeProxyErrors::OnlyRegister(var)
        }
    }
    impl ::std::convert::From<OnlyWhitelistedExecutor> for MystikoTBridgeProxyErrors {
        fn from(var: OnlyWhitelistedExecutor) -> Self {
            MystikoTBridgeProxyErrors::OnlyWhitelistedExecutor(var)
        }
    }
    impl ::std::convert::From<WithdrawFailed> for MystikoTBridgeProxyErrors {
        fn from(var: WithdrawFailed) -> Self {
            MystikoTBridgeProxyErrors::WithdrawFailed(var)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethevent(
        name = "TBridgeCrossChainMessage",
        abi = "TBridgeCrossChainMessage(address,uint256,address,bytes)"
    )]
    pub struct TbridgeCrossChainMessageFilter {
        pub to_contract: ethers::core::types::Address,
        pub to_chain_id: ethers::core::types::U256,
        pub from_contract: ethers::core::types::Address,
        pub message: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `addExecutorWhitelist` function with signature `addExecutorWhitelist(address)` and selector `[160, 113, 233, 177]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(name = "addExecutorWhitelist", abi = "addExecutorWhitelist(address)")]
    pub struct AddExecutorWhitelistCall {
        pub executor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addRegisterWhitelist` function with signature `addRegisterWhitelist(address)` and selector `[191, 223, 213, 99]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(name = "addRegisterWhitelist", abi = "addRegisterWhitelist(address)")]
    pub struct AddRegisterWhitelistCall {
        pub register: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `changeOperator` function with signature `changeOperator(address)` and selector `[6, 57, 76, 155]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(name = "changeOperator", abi = "changeOperator(address)")]
    pub struct ChangeOperatorCall {
        pub new_operator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `crossChainSyncTx` function with signature `crossChainSyncTx(uint64,address,address,address,bytes)` and selector `[145, 156, 29, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(
        name = "crossChainSyncTx",
        abi = "crossChainSyncTx(uint64,address,address,address,bytes)"
    )]
    pub struct CrossChainSyncTxCall {
        pub from_chain_id: u64,
        pub from_contract: ethers::core::types::Address,
        pub to_contract: ethers::core::types::Address,
        pub executor: ethers::core::types::Address,
        pub message: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `removeExecutorWhitelist` function with signature `removeExecutorWhitelist(address)` and selector `[209, 82, 9, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(
        name = "removeExecutorWhitelist",
        abi = "removeExecutorWhitelist(address)"
    )]
    pub struct RemoveExecutorWhitelistCall {
        pub executor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `removeRegisterWhitelist` function with signature `removeRegisterWhitelist(address)` and selector `[249, 25, 212, 105]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(
        name = "removeRegisterWhitelist",
        abi = "removeRegisterWhitelist(address)"
    )]
    pub struct RemoveRegisterWhitelistCall {
        pub register: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `sendMessage` function with signature `sendMessage(address,uint64,bytes)` and selector `[200, 23, 57, 205]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(name = "sendMessage", abi = "sendMessage(address,uint64,bytes)")]
    pub struct SendMessageCall {
        pub to_contract: ethers::core::types::Address,
        pub to_chain_id: u64,
        pub message: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw(address)` and selector `[81, 207, 248, 217]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address)")]
    pub struct WithdrawCall {
        pub recipient: ethers::core::types::Address,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum MystikoTBridgeProxyCalls {
        AddExecutorWhitelist(AddExecutorWhitelistCall),
        AddRegisterWhitelist(AddRegisterWhitelistCall),
        ChangeOperator(ChangeOperatorCall),
        CrossChainSyncTx(CrossChainSyncTxCall),
        RemoveExecutorWhitelist(RemoveExecutorWhitelistCall),
        RemoveRegisterWhitelist(RemoveRegisterWhitelistCall),
        SendMessage(SendMessageCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for MystikoTBridgeProxyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddExecutorWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyCalls::AddExecutorWhitelist(decoded));
            }
            if let Ok(decoded) =
                <AddRegisterWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyCalls::AddRegisterWhitelist(decoded));
            }
            if let Ok(decoded) =
                <ChangeOperatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyCalls::ChangeOperator(decoded));
            }
            if let Ok(decoded) =
                <CrossChainSyncTxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyCalls::CrossChainSyncTx(decoded));
            }
            if let Ok(decoded) =
                <RemoveExecutorWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyCalls::RemoveExecutorWhitelist(decoded));
            }
            if let Ok(decoded) =
                <RemoveRegisterWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyCalls::RemoveRegisterWhitelist(decoded));
            }
            if let Ok(decoded) =
                <SendMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyCalls::SendMessage(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoTBridgeProxyCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MystikoTBridgeProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MystikoTBridgeProxyCalls::AddExecutorWhitelist(element) => element.encode(),
                MystikoTBridgeProxyCalls::AddRegisterWhitelist(element) => element.encode(),
                MystikoTBridgeProxyCalls::ChangeOperator(element) => element.encode(),
                MystikoTBridgeProxyCalls::CrossChainSyncTx(element) => element.encode(),
                MystikoTBridgeProxyCalls::RemoveExecutorWhitelist(element) => element.encode(),
                MystikoTBridgeProxyCalls::RemoveRegisterWhitelist(element) => element.encode(),
                MystikoTBridgeProxyCalls::SendMessage(element) => element.encode(),
                MystikoTBridgeProxyCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MystikoTBridgeProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoTBridgeProxyCalls::AddExecutorWhitelist(element) => element.fmt(f),
                MystikoTBridgeProxyCalls::AddRegisterWhitelist(element) => element.fmt(f),
                MystikoTBridgeProxyCalls::ChangeOperator(element) => element.fmt(f),
                MystikoTBridgeProxyCalls::CrossChainSyncTx(element) => element.fmt(f),
                MystikoTBridgeProxyCalls::RemoveExecutorWhitelist(element) => element.fmt(f),
                MystikoTBridgeProxyCalls::RemoveRegisterWhitelist(element) => element.fmt(f),
                MystikoTBridgeProxyCalls::SendMessage(element) => element.fmt(f),
                MystikoTBridgeProxyCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddExecutorWhitelistCall> for MystikoTBridgeProxyCalls {
        fn from(var: AddExecutorWhitelistCall) -> Self {
            MystikoTBridgeProxyCalls::AddExecutorWhitelist(var)
        }
    }
    impl ::std::convert::From<AddRegisterWhitelistCall> for MystikoTBridgeProxyCalls {
        fn from(var: AddRegisterWhitelistCall) -> Self {
            MystikoTBridgeProxyCalls::AddRegisterWhitelist(var)
        }
    }
    impl ::std::convert::From<ChangeOperatorCall> for MystikoTBridgeProxyCalls {
        fn from(var: ChangeOperatorCall) -> Self {
            MystikoTBridgeProxyCalls::ChangeOperator(var)
        }
    }
    impl ::std::convert::From<CrossChainSyncTxCall> for MystikoTBridgeProxyCalls {
        fn from(var: CrossChainSyncTxCall) -> Self {
            MystikoTBridgeProxyCalls::CrossChainSyncTx(var)
        }
    }
    impl ::std::convert::From<RemoveExecutorWhitelistCall> for MystikoTBridgeProxyCalls {
        fn from(var: RemoveExecutorWhitelistCall) -> Self {
            MystikoTBridgeProxyCalls::RemoveExecutorWhitelist(var)
        }
    }
    impl ::std::convert::From<RemoveRegisterWhitelistCall> for MystikoTBridgeProxyCalls {
        fn from(var: RemoveRegisterWhitelistCall) -> Self {
            MystikoTBridgeProxyCalls::RemoveRegisterWhitelist(var)
        }
    }
    impl ::std::convert::From<SendMessageCall> for MystikoTBridgeProxyCalls {
        fn from(var: SendMessageCall) -> Self {
            MystikoTBridgeProxyCalls::SendMessage(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for MystikoTBridgeProxyCalls {
        fn from(var: WithdrawCall) -> Self {
            MystikoTBridgeProxyCalls::Withdraw(var)
        }
    }
    #[doc = "Container type for all return fields from the `crossChainSyncTx` function with signature `crossChainSyncTx(uint64,address,address,address,bytes)` and selector `[145, 156, 29, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct CrossChainSyncTxReturn(pub bool);
}
