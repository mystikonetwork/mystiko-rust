pub use i_axelar_executable::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_axelar_executable {
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
    #[doc = "IAxelarExecutable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"NotApprovedByGateway\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commandId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commandId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"tokenSymbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeWithToken\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gateway\",\"outputs\":[{\"internalType\":\"contract IAxelarGateway\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IAXELAREXECUTABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IAxelarExecutable<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IAxelarExecutable<M> {
        fn clone(&self) -> Self {
            IAxelarExecutable(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IAxelarExecutable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IAxelarExecutable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAxelarExecutable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IAxelarExecutable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IAXELAREXECUTABLE_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `execute` (0x49160658) function"]
        pub fn execute(
            &self,
            command_id: [u8; 32],
            source_chain: String,
            source_address: String,
            payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [73, 22, 6, 88],
                    (command_id, source_chain, source_address, payload),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeWithToken` (0x1a98b2e0) function"]
        pub fn execute_with_token(
            &self,
            command_id: [u8; 32],
            source_chain: String,
            source_address: String,
            payload: ethers::core::types::Bytes,
            token_symbol: String,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [26, 152, 178, 224],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        payload,
                        token_symbol,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `gateway` (0x116191b6) function"]
        pub fn gateway(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([17, 97, 145, 182], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IAxelarExecutable<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `NotApprovedByGateway` with signature `NotApprovedByGateway()` and selector `[80, 12, 68, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NotApprovedByGateway", abi = "NotApprovedByGateway()")]
    pub struct NotApprovedByGateway;
    #[doc = "Container type for all input parameters for the `execute` function with signature `execute(bytes32,string,string,bytes)` and selector `[73, 22, 6, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "execute", abi = "execute(bytes32,string,string,bytes)")]
    pub struct ExecuteCall {
        pub command_id: [u8; 32],
        pub source_chain: String,
        pub source_address: String,
        pub payload: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `executeWithToken` function with signature `executeWithToken(bytes32,string,string,bytes,string,uint256)` and selector `[26, 152, 178, 224]`"]
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
        name = "executeWithToken",
        abi = "executeWithToken(bytes32,string,string,bytes,string,uint256)"
    )]
    pub struct ExecuteWithTokenCall {
        pub command_id: [u8; 32],
        pub source_chain: String,
        pub source_address: String,
        pub payload: ethers::core::types::Bytes,
        pub token_symbol: String,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `gateway` function with signature `gateway()` and selector `[17, 97, 145, 182]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "gateway", abi = "gateway()")]
    pub struct GatewayCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAxelarExecutableCalls {
        Execute(ExecuteCall),
        ExecuteWithToken(ExecuteWithTokenCall),
        Gateway(GatewayCall),
    }
    impl ethers::core::abi::AbiDecode for IAxelarExecutableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ExecuteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarExecutableCalls::Execute(decoded));
            }
            if let Ok(decoded) =
                <ExecuteWithTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarExecutableCalls::ExecuteWithToken(decoded));
            }
            if let Ok(decoded) =
                <GatewayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarExecutableCalls::Gateway(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAxelarExecutableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAxelarExecutableCalls::Execute(element) => element.encode(),
                IAxelarExecutableCalls::ExecuteWithToken(element) => element.encode(),
                IAxelarExecutableCalls::Gateway(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAxelarExecutableCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAxelarExecutableCalls::Execute(element) => element.fmt(f),
                IAxelarExecutableCalls::ExecuteWithToken(element) => element.fmt(f),
                IAxelarExecutableCalls::Gateway(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ExecuteCall> for IAxelarExecutableCalls {
        fn from(var: ExecuteCall) -> Self {
            IAxelarExecutableCalls::Execute(var)
        }
    }
    impl ::std::convert::From<ExecuteWithTokenCall> for IAxelarExecutableCalls {
        fn from(var: ExecuteWithTokenCall) -> Self {
            IAxelarExecutableCalls::ExecuteWithToken(var)
        }
    }
    impl ::std::convert::From<GatewayCall> for IAxelarExecutableCalls {
        fn from(var: GatewayCall) -> Self {
            IAxelarExecutableCalls::Gateway(var)
        }
    }
    #[doc = "Container type for all return fields from the `gateway` function with signature `gateway()` and selector `[17, 97, 145, 182]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GatewayReturn(pub ethers::core::types::Address);
}
