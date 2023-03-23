pub use i_axelar_gas_service::*;
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
pub mod i_axelar_gas_service {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"NothingReceived\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferFailed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sourceAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GasPaidForContractCall\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sourceAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GasPaidForContractCallWithToken\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sourceAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NativeGasPaidForContractCall\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sourceAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NativeGasPaidForContractCallWithToken\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"collectFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"payGasForContractCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasFeeAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"payGasForContractCallWithToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"payNativeGasForContractCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refundAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"payNativeGasForContractCallWithToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"refund\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IAXELARGASSERVICE_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IAxelarGasService<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAxelarGasService<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAxelarGasService<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IAxelarGasService<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IAxelarGasService<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IAxelarGasService)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IAxelarGasService<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IAXELARGASSERVICE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `collectFees` (0xda854d75) function
        pub fn collect_fees(
            &self,
            receiver: ::ethers_core::types::Address,
            tokens: ::std::vec::Vec<::ethers_core::types::Address>,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 133, 77, 117], (receiver, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payGasForContractCall` (0xfd09e3bd) function
        pub fn pay_gas_for_contract_call(
            &self,
            sender: ::ethers_core::types::Address,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            payload: ::ethers_core::types::Bytes,
            gas_token: ::ethers_core::types::Address,
            gas_fee_amount: ::ethers_core::types::U256,
            refund_address: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
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
        ///Calls the contract's `payGasForContractCallWithToken` (0xedb6b3a5) function
        pub fn pay_gas_for_contract_call_with_token(
            &self,
            sender: ::ethers_core::types::Address,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            payload: ::ethers_core::types::Bytes,
            symbol: ::std::string::String,
            amount: ::ethers_core::types::U256,
            gas_token: ::ethers_core::types::Address,
            gas_fee_amount: ::ethers_core::types::U256,
            refund_address: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
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
        ///Calls the contract's `payNativeGasForContractCall` (0x0c93e3bb) function
        pub fn pay_native_gas_for_contract_call(
            &self,
            sender: ::ethers_core::types::Address,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            payload: ::ethers_core::types::Bytes,
            refund_address: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
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
        ///Calls the contract's `payNativeGasForContractCallWithToken` (0xc62c2002) function
        pub fn pay_native_gas_for_contract_call_with_token(
            &self,
            sender: ::ethers_core::types::Address,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            payload: ::ethers_core::types::Bytes,
            symbol: ::std::string::String,
            amount: ::ethers_core::types::U256,
            refund_address: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
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
        ///Calls the contract's `refund` (0x82ad6f35) function
        pub fn refund(
            &self,
            receiver: ::ethers_core::types::Address,
            token: ::ethers_core::types::Address,
            amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 173, 111, 53], (receiver, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `GasPaidForContractCall` event
        pub fn gas_paid_for_contract_call_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasPaidForContractCallFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GasPaidForContractCallWithToken` event
        pub fn gas_paid_for_contract_call_with_token_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasPaidForContractCallWithTokenFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NativeGasPaidForContractCall` event
        pub fn native_gas_paid_for_contract_call_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NativeGasPaidForContractCallFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NativeGasPaidForContractCallWithToken` event
        pub fn native_gas_paid_for_contract_call_with_token_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NativeGasPaidForContractCallWithTokenFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IAxelarGasServiceEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IAxelarGasService<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NothingReceived` with signature `NothingReceived()` and selector `0xb5c74a27`
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
    #[etherror(name = "NothingReceived", abi = "NothingReceived()")]
    pub struct NothingReceived;
    ///Custom Error type `TransferFailed` with signature `TransferFailed()` and selector `0x90b8ec18`
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
    #[etherror(name = "TransferFailed", abi = "TransferFailed()")]
    pub struct TransferFailed;
    ///Container type for all of the contract's custom errors
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
    pub enum IAxelarGasServiceErrors {
        NothingReceived(NothingReceived),
        TransferFailed(TransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for IAxelarGasServiceErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <NothingReceived as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NothingReceived(decoded));
            }
            if let Ok(decoded)
                = <TransferFailed as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFailed(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IAxelarGasServiceErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::NothingReceived(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::TransferFailed(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for IAxelarGasServiceErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <NothingReceived as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFailed as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IAxelarGasServiceErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NothingReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IAxelarGasServiceErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<NothingReceived> for IAxelarGasServiceErrors {
        fn from(value: NothingReceived) -> Self {
            Self::NothingReceived(value)
        }
    }
    impl ::core::convert::From<TransferFailed> for IAxelarGasServiceErrors {
        fn from(value: TransferFailed) -> Self {
            Self::TransferFailed(value)
        }
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
        name = "GasPaidForContractCall",
        abi = "GasPaidForContractCall(address,string,string,bytes32,address,uint256,address)"
    )]
    pub struct GasPaidForContractCallFilter {
        #[ethevent(indexed)]
        pub source_address: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub gas_token: ::ethers_core::types::Address,
        pub gas_fee_amount: ::ethers_core::types::U256,
        pub refund_address: ::ethers_core::types::Address,
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
        name = "GasPaidForContractCallWithToken",
        abi = "GasPaidForContractCallWithToken(address,string,string,bytes32,string,uint256,address,uint256,address)"
    )]
    pub struct GasPaidForContractCallWithTokenFilter {
        #[ethevent(indexed)]
        pub source_address: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
        pub gas_token: ::ethers_core::types::Address,
        pub gas_fee_amount: ::ethers_core::types::U256,
        pub refund_address: ::ethers_core::types::Address,
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
        name = "NativeGasPaidForContractCall",
        abi = "NativeGasPaidForContractCall(address,string,string,bytes32,uint256,address)"
    )]
    pub struct NativeGasPaidForContractCallFilter {
        #[ethevent(indexed)]
        pub source_address: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub gas_fee_amount: ::ethers_core::types::U256,
        pub refund_address: ::ethers_core::types::Address,
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
        name = "NativeGasPaidForContractCallWithToken",
        abi = "NativeGasPaidForContractCallWithToken(address,string,string,bytes32,string,uint256,uint256,address)"
    )]
    pub struct NativeGasPaidForContractCallWithTokenFilter {
        #[ethevent(indexed)]
        pub source_address: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
        pub gas_fee_amount: ::ethers_core::types::U256,
        pub refund_address: ::ethers_core::types::Address,
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
    pub enum IAxelarGasServiceEvents {
        GasPaidForContractCallFilter(GasPaidForContractCallFilter),
        GasPaidForContractCallWithTokenFilter(GasPaidForContractCallWithTokenFilter),
        NativeGasPaidForContractCallFilter(NativeGasPaidForContractCallFilter),
        NativeGasPaidForContractCallWithTokenFilter(
            NativeGasPaidForContractCallWithTokenFilter,
        ),
    }
    impl ::ethers_contract::EthLogDecode for IAxelarGasServiceEvents {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = GasPaidForContractCallFilter::decode_log(log) {
                return Ok(
                    IAxelarGasServiceEvents::GasPaidForContractCallFilter(decoded),
                );
            }
            if let Ok(decoded) = GasPaidForContractCallWithTokenFilter::decode_log(log) {
                return Ok(
                    IAxelarGasServiceEvents::GasPaidForContractCallWithTokenFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = NativeGasPaidForContractCallFilter::decode_log(log) {
                return Ok(
                    IAxelarGasServiceEvents::NativeGasPaidForContractCallFilter(decoded),
                );
            }
            if let Ok(decoded)
                = NativeGasPaidForContractCallWithTokenFilter::decode_log(log) {
                return Ok(
                    IAxelarGasServiceEvents::NativeGasPaidForContractCallWithTokenFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IAxelarGasServiceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GasPaidForContractCallFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GasPaidForContractCallWithTokenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NativeGasPaidForContractCallFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NativeGasPaidForContractCallWithTokenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GasPaidForContractCallFilter>
    for IAxelarGasServiceEvents {
        fn from(value: GasPaidForContractCallFilter) -> Self {
            Self::GasPaidForContractCallFilter(value)
        }
    }
    impl ::core::convert::From<GasPaidForContractCallWithTokenFilter>
    for IAxelarGasServiceEvents {
        fn from(value: GasPaidForContractCallWithTokenFilter) -> Self {
            Self::GasPaidForContractCallWithTokenFilter(value)
        }
    }
    impl ::core::convert::From<NativeGasPaidForContractCallFilter>
    for IAxelarGasServiceEvents {
        fn from(value: NativeGasPaidForContractCallFilter) -> Self {
            Self::NativeGasPaidForContractCallFilter(value)
        }
    }
    impl ::core::convert::From<NativeGasPaidForContractCallWithTokenFilter>
    for IAxelarGasServiceEvents {
        fn from(value: NativeGasPaidForContractCallWithTokenFilter) -> Self {
            Self::NativeGasPaidForContractCallWithTokenFilter(value)
        }
    }
    ///Container type for all input parameters for the `collectFees` function with signature `collectFees(address,address[])` and selector `0xda854d75`
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
    #[ethcall(name = "collectFees", abi = "collectFees(address,address[])")]
    pub struct CollectFeesCall {
        pub receiver: ::ethers_core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers_core::types::Address>,
    }
    ///Container type for all input parameters for the `payGasForContractCall` function with signature `payGasForContractCall(address,string,string,bytes,address,uint256,address)` and selector `0xfd09e3bd`
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
    #[ethcall(
        name = "payGasForContractCall",
        abi = "payGasForContractCall(address,string,string,bytes,address,uint256,address)"
    )]
    pub struct PayGasForContractCallCall {
        pub sender: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub payload: ::ethers_core::types::Bytes,
        pub gas_token: ::ethers_core::types::Address,
        pub gas_fee_amount: ::ethers_core::types::U256,
        pub refund_address: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `payGasForContractCallWithToken` function with signature `payGasForContractCallWithToken(address,string,string,bytes,string,uint256,address,uint256,address)` and selector `0xedb6b3a5`
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
    #[ethcall(
        name = "payGasForContractCallWithToken",
        abi = "payGasForContractCallWithToken(address,string,string,bytes,string,uint256,address,uint256,address)"
    )]
    pub struct PayGasForContractCallWithTokenCall {
        pub sender: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub payload: ::ethers_core::types::Bytes,
        pub symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
        pub gas_token: ::ethers_core::types::Address,
        pub gas_fee_amount: ::ethers_core::types::U256,
        pub refund_address: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `payNativeGasForContractCall` function with signature `payNativeGasForContractCall(address,string,string,bytes,address)` and selector `0x0c93e3bb`
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
    #[ethcall(
        name = "payNativeGasForContractCall",
        abi = "payNativeGasForContractCall(address,string,string,bytes,address)"
    )]
    pub struct PayNativeGasForContractCallCall {
        pub sender: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub payload: ::ethers_core::types::Bytes,
        pub refund_address: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `payNativeGasForContractCallWithToken` function with signature `payNativeGasForContractCallWithToken(address,string,string,bytes,string,uint256,address)` and selector `0xc62c2002`
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
    #[ethcall(
        name = "payNativeGasForContractCallWithToken",
        abi = "payNativeGasForContractCallWithToken(address,string,string,bytes,string,uint256,address)"
    )]
    pub struct PayNativeGasForContractCallWithTokenCall {
        pub sender: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub payload: ::ethers_core::types::Bytes,
        pub symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
        pub refund_address: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `refund` function with signature `refund(address,address,uint256)` and selector `0x82ad6f35`
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
    #[ethcall(name = "refund", abi = "refund(address,address,uint256)")]
    pub struct RefundCall {
        pub receiver: ::ethers_core::types::Address,
        pub token: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
    }
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
    pub enum IAxelarGasServiceCalls {
        CollectFees(CollectFeesCall),
        PayGasForContractCall(PayGasForContractCallCall),
        PayGasForContractCallWithToken(PayGasForContractCallWithTokenCall),
        PayNativeGasForContractCall(PayNativeGasForContractCallCall),
        PayNativeGasForContractCallWithToken(PayNativeGasForContractCallWithTokenCall),
        Refund(RefundCall),
    }
    impl ::ethers_core::abi::AbiDecode for IAxelarGasServiceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CollectFeesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CollectFees(decoded));
            }
            if let Ok(decoded)
                = <PayGasForContractCallCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PayGasForContractCall(decoded));
            }
            if let Ok(decoded)
                = <PayGasForContractCallWithTokenCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PayGasForContractCallWithToken(decoded));
            }
            if let Ok(decoded)
                = <PayNativeGasForContractCallCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PayNativeGasForContractCall(decoded));
            }
            if let Ok(decoded)
                = <PayNativeGasForContractCallWithTokenCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PayNativeGasForContractCallWithToken(decoded));
            }
            if let Ok(decoded)
                = <RefundCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Refund(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IAxelarGasServiceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CollectFees(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PayGasForContractCall(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PayGasForContractCallWithToken(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PayNativeGasForContractCall(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PayNativeGasForContractCallWithToken(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Refund(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IAxelarGasServiceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CollectFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayGasForContractCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PayGasForContractCallWithToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PayNativeGasForContractCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PayNativeGasForContractCallWithToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Refund(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CollectFeesCall> for IAxelarGasServiceCalls {
        fn from(value: CollectFeesCall) -> Self {
            Self::CollectFees(value)
        }
    }
    impl ::core::convert::From<PayGasForContractCallCall> for IAxelarGasServiceCalls {
        fn from(value: PayGasForContractCallCall) -> Self {
            Self::PayGasForContractCall(value)
        }
    }
    impl ::core::convert::From<PayGasForContractCallWithTokenCall>
    for IAxelarGasServiceCalls {
        fn from(value: PayGasForContractCallWithTokenCall) -> Self {
            Self::PayGasForContractCallWithToken(value)
        }
    }
    impl ::core::convert::From<PayNativeGasForContractCallCall>
    for IAxelarGasServiceCalls {
        fn from(value: PayNativeGasForContractCallCall) -> Self {
            Self::PayNativeGasForContractCall(value)
        }
    }
    impl ::core::convert::From<PayNativeGasForContractCallWithTokenCall>
    for IAxelarGasServiceCalls {
        fn from(value: PayNativeGasForContractCallWithTokenCall) -> Self {
            Self::PayNativeGasForContractCallWithToken(value)
        }
    }
    impl ::core::convert::From<RefundCall> for IAxelarGasServiceCalls {
        fn from(value: RefundCall) -> Self {
            Self::Refund(value)
        }
    }
}
