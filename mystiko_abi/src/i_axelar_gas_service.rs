pub use i_axelar_gas_service::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_axelar_gas_service {
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
    #[doc = "IAxelarGasService was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"NothingReceived\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferFailed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sourceAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GasPaidForContractCall\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sourceAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GasPaidForContractCallWithToken\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sourceAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NativeGasPaidForContractCall\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sourceAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NativeGasPaidForContractCallWithToken\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"collectFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"payGasForContractCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"payGasForContractCallWithToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"payNativeGasForContractCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"payNativeGasForContractCallWithToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"refund\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IAXELARGASSERVICE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IAxelarGasService<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IAxelarGasService<M> {
        fn clone(&self) -> Self {
            IAxelarGasService(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IAxelarGasService<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IAxelarGasService<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAxelarGasService))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IAxelarGasService<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IAXELARGASSERVICE_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `collectFees` (0xda854d75) function"]
        pub fn collect_fees(
            &self,
            receiver: ethers::core::types::Address,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 133, 77, 117], (receiver, tokens))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `payGasForContractCall` (0xfd09e3bd) function"]
        pub fn pay_gas_for_contract_call(
            &self,
            sender: ethers::core::types::Address,
            destination_chain: String,
            destination_address: String,
            payload: ethers::core::types::Bytes,
            gas_token: ethers::core::types::Address,
            gas_fee_amount: ethers::core::types::U256,
            refund_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [253, 9, 227, 189],
                    (
                        sender,
                        destination_chain,
                        destination_address,
                        payload,
                        gas_token,
                        gas_fee_amount,
                        refund_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `payGasForContractCallWithToken` (0xedb6b3a5) function"]
        pub fn pay_gas_for_contract_call_with_token(
            &self,
            sender: ethers::core::types::Address,
            destination_chain: String,
            destination_address: String,
            payload: ethers::core::types::Bytes,
            symbol: String,
            amount: ethers::core::types::U256,
            gas_token: ethers::core::types::Address,
            gas_fee_amount: ethers::core::types::U256,
            refund_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [237, 182, 179, 165],
                    (
                        sender,
                        destination_chain,
                        destination_address,
                        payload,
                        symbol,
                        amount,
                        gas_token,
                        gas_fee_amount,
                        refund_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `payNativeGasForContractCall` (0x0c93e3bb) function"]
        pub fn pay_native_gas_for_contract_call(
            &self,
            sender: ethers::core::types::Address,
            destination_chain: String,
            destination_address: String,
            payload: ethers::core::types::Bytes,
            refund_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [12, 147, 227, 187],
                    (
                        sender,
                        destination_chain,
                        destination_address,
                        payload,
                        refund_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `payNativeGasForContractCallWithToken` (0xc62c2002) function"]
        pub fn pay_native_gas_for_contract_call_with_token(
            &self,
            sender: ethers::core::types::Address,
            destination_chain: String,
            destination_address: String,
            payload: ethers::core::types::Bytes,
            symbol: String,
            amount: ethers::core::types::U256,
            refund_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [198, 44, 32, 2],
                    (
                        sender,
                        destination_chain,
                        destination_address,
                        payload,
                        symbol,
                        amount,
                        refund_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refund` (0x82ad6f35) function"]
        pub fn refund(
            &self,
            receiver: ethers::core::types::Address,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 173, 111, 53], (receiver, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `GasPaidForContractCall` event"]
        pub fn gas_paid_for_contract_call_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, GasPaidForContractCallFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `GasPaidForContractCallWithToken` event"]
        pub fn gas_paid_for_contract_call_with_token_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, GasPaidForContractCallWithTokenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NativeGasPaidForContractCall` event"]
        pub fn native_gas_paid_for_contract_call_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NativeGasPaidForContractCallFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NativeGasPaidForContractCallWithToken` event"]
        pub fn native_gas_paid_for_contract_call_with_token_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NativeGasPaidForContractCallWithTokenFilter>
        {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IAxelarGasServiceEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IAxelarGasService<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `NothingReceived` with signature `NothingReceived()` and selector `[181, 199, 74, 39]`"]
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
    #[etherror(name = "NothingReceived", abi = "NothingReceived()")]
    pub struct NothingReceived;
    #[doc = "Custom Error type `TransferFailed` with signature `TransferFailed()` and selector `[144, 184, 236, 24]`"]
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
    #[etherror(name = "TransferFailed", abi = "TransferFailed()")]
    pub struct TransferFailed;
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum IAxelarGasServiceErrors {
        NothingReceived(NothingReceived),
        TransferFailed(TransferFailed),
    }
    impl ethers::core::abi::AbiDecode for IAxelarGasServiceErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <NothingReceived as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGasServiceErrors::NothingReceived(decoded));
            }
            if let Ok(decoded) =
                <TransferFailed as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGasServiceErrors::TransferFailed(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAxelarGasServiceErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IAxelarGasServiceErrors::NothingReceived(element) => element.encode(),
                IAxelarGasServiceErrors::TransferFailed(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAxelarGasServiceErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAxelarGasServiceErrors::NothingReceived(element) => element.fmt(f),
                IAxelarGasServiceErrors::TransferFailed(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<NothingReceived> for IAxelarGasServiceErrors {
        fn from(var: NothingReceived) -> Self {
            IAxelarGasServiceErrors::NothingReceived(var)
        }
    }
    impl ::std::convert::From<TransferFailed> for IAxelarGasServiceErrors {
        fn from(var: TransferFailed) -> Self {
            IAxelarGasServiceErrors::TransferFailed(var)
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
        name = "GasPaidForContractCall",
        abi = "GasPaidForContractCall(address,string,string,bytes32,address,uint256,address)"
    )]
    pub struct GasPaidForContractCallFilter {
        #[ethevent(indexed)]
        pub source_address: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_address: String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub gas_token: ethers::core::types::Address,
        pub gas_fee_amount: ethers::core::types::U256,
        pub refund_address: ethers::core::types::Address,
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
        name = "GasPaidForContractCallWithToken",
        abi = "GasPaidForContractCallWithToken(address,string,string,bytes32,string,uint256,address,uint256,address)"
    )]
    pub struct GasPaidForContractCallWithTokenFilter {
        #[ethevent(indexed)]
        pub source_address: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_address: String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub symbol: String,
        pub amount: ethers::core::types::U256,
        pub gas_token: ethers::core::types::Address,
        pub gas_fee_amount: ethers::core::types::U256,
        pub refund_address: ethers::core::types::Address,
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
        name = "NativeGasPaidForContractCall",
        abi = "NativeGasPaidForContractCall(address,string,string,bytes32,uint256,address)"
    )]
    pub struct NativeGasPaidForContractCallFilter {
        #[ethevent(indexed)]
        pub source_address: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_address: String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub gas_fee_amount: ethers::core::types::U256,
        pub refund_address: ethers::core::types::Address,
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
        name = "NativeGasPaidForContractCallWithToken",
        abi = "NativeGasPaidForContractCallWithToken(address,string,string,bytes32,string,uint256,uint256,address)"
    )]
    pub struct NativeGasPaidForContractCallWithTokenFilter {
        #[ethevent(indexed)]
        pub source_address: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_address: String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub symbol: String,
        pub amount: ethers::core::types::U256,
        pub gas_fee_amount: ethers::core::types::U256,
        pub refund_address: ethers::core::types::Address,
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
    pub enum IAxelarGasServiceEvents {
        GasPaidForContractCallFilter(GasPaidForContractCallFilter),
        GasPaidForContractCallWithTokenFilter(GasPaidForContractCallWithTokenFilter),
        NativeGasPaidForContractCallFilter(NativeGasPaidForContractCallFilter),
        NativeGasPaidForContractCallWithTokenFilter(NativeGasPaidForContractCallWithTokenFilter),
    }
    impl ethers::contract::EthLogDecode for IAxelarGasServiceEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = GasPaidForContractCallFilter::decode_log(log) {
                return Ok(IAxelarGasServiceEvents::GasPaidForContractCallFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = GasPaidForContractCallWithTokenFilter::decode_log(log) {
                return Ok(IAxelarGasServiceEvents::GasPaidForContractCallWithTokenFilter(decoded));
            }
            if let Ok(decoded) = NativeGasPaidForContractCallFilter::decode_log(log) {
                return Ok(IAxelarGasServiceEvents::NativeGasPaidForContractCallFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NativeGasPaidForContractCallWithTokenFilter::decode_log(log) {
                return Ok(
                    IAxelarGasServiceEvents::NativeGasPaidForContractCallWithTokenFilter(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IAxelarGasServiceEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAxelarGasServiceEvents::GasPaidForContractCallFilter(element) => element.fmt(f),
                IAxelarGasServiceEvents::GasPaidForContractCallWithTokenFilter(element) => {
                    element.fmt(f)
                }
                IAxelarGasServiceEvents::NativeGasPaidForContractCallFilter(element) => {
                    element.fmt(f)
                }
                IAxelarGasServiceEvents::NativeGasPaidForContractCallWithTokenFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `collectFees` function with signature `collectFees(address,address[])` and selector `[218, 133, 77, 117]`"]
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
    #[ethcall(name = "collectFees", abi = "collectFees(address,address[])")]
    pub struct CollectFeesCall {
        pub receiver: ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `payGasForContractCall` function with signature `payGasForContractCall(address,string,string,bytes,address,uint256,address)` and selector `[253, 9, 227, 189]`"]
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
        name = "payGasForContractCall",
        abi = "payGasForContractCall(address,string,string,bytes,address,uint256,address)"
    )]
    pub struct PayGasForContractCallCall {
        pub sender: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_address: String,
        pub payload: ethers::core::types::Bytes,
        pub gas_token: ethers::core::types::Address,
        pub gas_fee_amount: ethers::core::types::U256,
        pub refund_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `payGasForContractCallWithToken` function with signature `payGasForContractCallWithToken(address,string,string,bytes,string,uint256,address,uint256,address)` and selector `[237, 182, 179, 165]`"]
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
        name = "payGasForContractCallWithToken",
        abi = "payGasForContractCallWithToken(address,string,string,bytes,string,uint256,address,uint256,address)"
    )]
    pub struct PayGasForContractCallWithTokenCall {
        pub sender: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_address: String,
        pub payload: ethers::core::types::Bytes,
        pub symbol: String,
        pub amount: ethers::core::types::U256,
        pub gas_token: ethers::core::types::Address,
        pub gas_fee_amount: ethers::core::types::U256,
        pub refund_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `payNativeGasForContractCall` function with signature `payNativeGasForContractCall(address,string,string,bytes,address)` and selector `[12, 147, 227, 187]`"]
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
        name = "payNativeGasForContractCall",
        abi = "payNativeGasForContractCall(address,string,string,bytes,address)"
    )]
    pub struct PayNativeGasForContractCallCall {
        pub sender: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_address: String,
        pub payload: ethers::core::types::Bytes,
        pub refund_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `payNativeGasForContractCallWithToken` function with signature `payNativeGasForContractCallWithToken(address,string,string,bytes,string,uint256,address)` and selector `[198, 44, 32, 2]`"]
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
        name = "payNativeGasForContractCallWithToken",
        abi = "payNativeGasForContractCallWithToken(address,string,string,bytes,string,uint256,address)"
    )]
    pub struct PayNativeGasForContractCallWithTokenCall {
        pub sender: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_address: String,
        pub payload: ethers::core::types::Bytes,
        pub symbol: String,
        pub amount: ethers::core::types::U256,
        pub refund_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `refund` function with signature `refund(address,address,uint256)` and selector `[130, 173, 111, 53]`"]
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
    #[ethcall(name = "refund", abi = "refund(address,address,uint256)")]
    pub struct RefundCall {
        pub receiver: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    pub enum IAxelarGasServiceCalls {
        CollectFees(CollectFeesCall),
        PayGasForContractCall(PayGasForContractCallCall),
        PayGasForContractCallWithToken(PayGasForContractCallWithTokenCall),
        PayNativeGasForContractCall(PayNativeGasForContractCallCall),
        PayNativeGasForContractCallWithToken(PayNativeGasForContractCallWithTokenCall),
        Refund(RefundCall),
    }
    impl ethers::core::abi::AbiDecode for IAxelarGasServiceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CollectFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGasServiceCalls::CollectFees(decoded));
            }
            if let Ok(decoded) =
                <PayGasForContractCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGasServiceCalls::PayGasForContractCall(decoded));
            }
            if let Ok(decoded) =
                <PayGasForContractCallWithTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAxelarGasServiceCalls::PayGasForContractCallWithToken(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PayNativeGasForContractCallCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAxelarGasServiceCalls::PayNativeGasForContractCall(decoded));
            }
            if let Ok(decoded) =
                <PayNativeGasForContractCallWithTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAxelarGasServiceCalls::PayNativeGasForContractCallWithToken(decoded));
            }
            if let Ok(decoded) = <RefundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGasServiceCalls::Refund(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAxelarGasServiceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAxelarGasServiceCalls::CollectFees(element) => element.encode(),
                IAxelarGasServiceCalls::PayGasForContractCall(element) => element.encode(),
                IAxelarGasServiceCalls::PayGasForContractCallWithToken(element) => element.encode(),
                IAxelarGasServiceCalls::PayNativeGasForContractCall(element) => element.encode(),
                IAxelarGasServiceCalls::PayNativeGasForContractCallWithToken(element) => {
                    element.encode()
                }
                IAxelarGasServiceCalls::Refund(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAxelarGasServiceCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAxelarGasServiceCalls::CollectFees(element) => element.fmt(f),
                IAxelarGasServiceCalls::PayGasForContractCall(element) => element.fmt(f),
                IAxelarGasServiceCalls::PayGasForContractCallWithToken(element) => element.fmt(f),
                IAxelarGasServiceCalls::PayNativeGasForContractCall(element) => element.fmt(f),
                IAxelarGasServiceCalls::PayNativeGasForContractCallWithToken(element) => {
                    element.fmt(f)
                }
                IAxelarGasServiceCalls::Refund(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CollectFeesCall> for IAxelarGasServiceCalls {
        fn from(var: CollectFeesCall) -> Self {
            IAxelarGasServiceCalls::CollectFees(var)
        }
    }
    impl ::std::convert::From<PayGasForContractCallCall> for IAxelarGasServiceCalls {
        fn from(var: PayGasForContractCallCall) -> Self {
            IAxelarGasServiceCalls::PayGasForContractCall(var)
        }
    }
    impl ::std::convert::From<PayGasForContractCallWithTokenCall> for IAxelarGasServiceCalls {
        fn from(var: PayGasForContractCallWithTokenCall) -> Self {
            IAxelarGasServiceCalls::PayGasForContractCallWithToken(var)
        }
    }
    impl ::std::convert::From<PayNativeGasForContractCallCall> for IAxelarGasServiceCalls {
        fn from(var: PayNativeGasForContractCallCall) -> Self {
            IAxelarGasServiceCalls::PayNativeGasForContractCall(var)
        }
    }
    impl ::std::convert::From<PayNativeGasForContractCallWithTokenCall> for IAxelarGasServiceCalls {
        fn from(var: PayNativeGasForContractCallWithTokenCall) -> Self {
            IAxelarGasServiceCalls::PayNativeGasForContractCallWithToken(var)
        }
    }
    impl ::std::convert::From<RefundCall> for IAxelarGasServiceCalls {
        fn from(var: RefundCall) -> Self {
            IAxelarGasServiceCalls::Refund(var)
        }
    }
}
