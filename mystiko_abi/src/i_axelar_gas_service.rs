pub use i_axelar_gas_service::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod i_axelar_gas_service {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("collectFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("collectFees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("receiver"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address payable"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokens"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address[]"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payGasForContractCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("payGasForContractCall",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destinationAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("gasFeeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payGasForContractCallWithToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("payGasForContractCallWithToken",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destinationAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("symbol"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("gasFeeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payNativeGasForContractCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("payNativeGasForContractCall",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destinationAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payNativeGasForContractCallWithToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("payNativeGasForContractCallWithToken",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destinationAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("symbol"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("refund"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("refund"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("receiver"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address payable"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("GasPaidForContractCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("GasPaidForContractCall",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("gasFeeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GasPaidForContractCallWithToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("GasPaidForContractCallWithToken",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("symbol"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("gasFeeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NativeGasPaidForContractCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NativeGasPaidForContractCall",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("gasFeeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NativeGasPaidForContractCallWithToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NativeGasPaidForContractCallWithToken",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("symbol"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("gasFeeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NothingReceived"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NothingReceived"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TransferFailed"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IAXELARGASSERVICE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IAxelarGasService<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAxelarGasService<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAxelarGasService<M> {
        type Target = ::ethers::contract::Contract<M>;
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
            f.debug_tuple(::core::stringify!(IAxelarGasService))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IAxelarGasService<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IAXELARGASSERVICE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `collectFees` (0xda854d75) function
        pub fn collect_fees(
            &self,
            receiver: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 133, 77, 117], (receiver, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payGasForContractCall` (0xfd09e3bd) function
        pub fn pay_gas_for_contract_call(
            &self,
            sender: ::ethers::core::types::Address,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            payload: ::ethers::core::types::Bytes,
            gas_token: ::ethers::core::types::Address,
            gas_fee_amount: ::ethers::core::types::U256,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
            sender: ::ethers::core::types::Address,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            payload: ::ethers::core::types::Bytes,
            symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            gas_fee_amount: ::ethers::core::types::U256,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
            sender: ::ethers::core::types::Address,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            payload: ::ethers::core::types::Bytes,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [12, 147, 227, 187],
                    (sender, destination_chain, destination_address, payload, refund_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payNativeGasForContractCallWithToken` (0xc62c2002) function
        pub fn pay_native_gas_for_contract_call_with_token(
            &self,
            sender: ::ethers::core::types::Address,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            payload: ::ethers::core::types::Bytes,
            symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
            receiver: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 173, 111, 53], (receiver, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `GasPaidForContractCall` event
        pub fn gas_paid_for_contract_call_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GasPaidForContractCallFilter> {
            self.0.event()
        }
        ///Gets the contract's `GasPaidForContractCallWithToken` event
        pub fn gas_paid_for_contract_call_with_token_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GasPaidForContractCallWithTokenFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NativeGasPaidForContractCall` event
        pub fn native_gas_paid_for_contract_call_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NativeGasPaidForContractCallFilter> {
            self.0.event()
        }
        ///Gets the contract's `NativeGasPaidForContractCallWithToken` event
        pub fn native_gas_paid_for_contract_call_with_token_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NativeGasPaidForContractCallWithTokenFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(&self) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IAxelarGasServiceEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for IAxelarGasService<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NothingReceived` with signature `NothingReceived()` and selector `0xb5c74a27`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NothingReceived", abi = "NothingReceived()")]
    pub struct NothingReceived;
    ///Custom Error type `TransferFailed` with signature `TransferFailed()` and selector `0x90b8ec18`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TransferFailed", abi = "TransferFailed()")]
    pub struct TransferFailed;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum IAxelarGasServiceErrors {
        NothingReceived(NothingReceived),
        TransferFailed(TransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IAxelarGasServiceErrors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <NothingReceived as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NothingReceived(decoded));
            }
            if let Ok(decoded) = <TransferFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IAxelarGasServiceErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::NothingReceived(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IAxelarGasServiceErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <NothingReceived as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <TransferFailed as ::ethers::contract::EthError>::selector() => true,
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
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GasPaidForContractCall",
        abi = "GasPaidForContractCall(address,string,string,bytes32,address,uint256,address)"
    )]
    pub struct GasPaidForContractCallFilter {
        #[ethevent(indexed)]
        pub source_address: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub gas_token: ::ethers::core::types::Address,
        pub gas_fee_amount: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GasPaidForContractCallWithToken",
        abi = "GasPaidForContractCallWithToken(address,string,string,bytes32,string,uint256,address,uint256,address)"
    )]
    pub struct GasPaidForContractCallWithTokenFilter {
        #[ethevent(indexed)]
        pub source_address: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub gas_fee_amount: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "NativeGasPaidForContractCall",
        abi = "NativeGasPaidForContractCall(address,string,string,bytes32,uint256,address)"
    )]
    pub struct NativeGasPaidForContractCallFilter {
        #[ethevent(indexed)]
        pub source_address: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub gas_fee_amount: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "NativeGasPaidForContractCallWithToken",
        abi = "NativeGasPaidForContractCallWithToken(address,string,string,bytes32,string,uint256,uint256,address)"
    )]
    pub struct NativeGasPaidForContractCallWithTokenFilter {
        #[ethevent(indexed)]
        pub source_address: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
        pub gas_fee_amount: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum IAxelarGasServiceEvents {
        GasPaidForContractCallFilter(GasPaidForContractCallFilter),
        GasPaidForContractCallWithTokenFilter(GasPaidForContractCallWithTokenFilter),
        NativeGasPaidForContractCallFilter(NativeGasPaidForContractCallFilter),
        NativeGasPaidForContractCallWithTokenFilter(NativeGasPaidForContractCallWithTokenFilter),
    }
    impl ::ethers::contract::EthLogDecode for IAxelarGasServiceEvents {
        fn decode_log(log: &::ethers::core::abi::RawLog) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = GasPaidForContractCallFilter::decode_log(log) {
                return Ok(IAxelarGasServiceEvents::GasPaidForContractCallFilter(decoded));
            }
            if let Ok(decoded) = GasPaidForContractCallWithTokenFilter::decode_log(log) {
                return Ok(IAxelarGasServiceEvents::GasPaidForContractCallWithTokenFilter(decoded));
            }
            if let Ok(decoded) = NativeGasPaidForContractCallFilter::decode_log(log) {
                return Ok(IAxelarGasServiceEvents::NativeGasPaidForContractCallFilter(decoded));
            }
            if let Ok(decoded) = NativeGasPaidForContractCallWithTokenFilter::decode_log(log) {
                return Ok(IAxelarGasServiceEvents::NativeGasPaidForContractCallWithTokenFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IAxelarGasServiceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GasPaidForContractCallFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GasPaidForContractCallWithTokenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeGasPaidForContractCallFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeGasPaidForContractCallWithTokenFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GasPaidForContractCallFilter> for IAxelarGasServiceEvents {
        fn from(value: GasPaidForContractCallFilter) -> Self {
            Self::GasPaidForContractCallFilter(value)
        }
    }
    impl ::core::convert::From<GasPaidForContractCallWithTokenFilter> for IAxelarGasServiceEvents {
        fn from(value: GasPaidForContractCallWithTokenFilter) -> Self {
            Self::GasPaidForContractCallWithTokenFilter(value)
        }
    }
    impl ::core::convert::From<NativeGasPaidForContractCallFilter> for IAxelarGasServiceEvents {
        fn from(value: NativeGasPaidForContractCallFilter) -> Self {
            Self::NativeGasPaidForContractCallFilter(value)
        }
    }
    impl ::core::convert::From<NativeGasPaidForContractCallWithTokenFilter> for IAxelarGasServiceEvents {
        fn from(value: NativeGasPaidForContractCallWithTokenFilter) -> Self {
            Self::NativeGasPaidForContractCallWithTokenFilter(value)
        }
    }
    ///Container type for all input parameters for the `collectFees` function with signature `collectFees(address,address[])` and selector `0xda854d75`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "collectFees", abi = "collectFees(address,address[])")]
    pub struct CollectFeesCall {
        pub receiver: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `payGasForContractCall` function with signature `payGasForContractCall(address,string,string,bytes,address,uint256,address)` and selector `0xfd09e3bd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "payGasForContractCall",
        abi = "payGasForContractCall(address,string,string,bytes,address,uint256,address)"
    )]
    pub struct PayGasForContractCallCall {
        pub sender: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub payload: ::ethers::core::types::Bytes,
        pub gas_token: ::ethers::core::types::Address,
        pub gas_fee_amount: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `payGasForContractCallWithToken` function with signature `payGasForContractCallWithToken(address,string,string,bytes,string,uint256,address,uint256,address)` and selector `0xedb6b3a5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "payGasForContractCallWithToken",
        abi = "payGasForContractCallWithToken(address,string,string,bytes,string,uint256,address,uint256,address)"
    )]
    pub struct PayGasForContractCallWithTokenCall {
        pub sender: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub payload: ::ethers::core::types::Bytes,
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub gas_fee_amount: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `payNativeGasForContractCall` function with signature `payNativeGasForContractCall(address,string,string,bytes,address)` and selector `0x0c93e3bb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "payNativeGasForContractCall",
        abi = "payNativeGasForContractCall(address,string,string,bytes,address)"
    )]
    pub struct PayNativeGasForContractCallCall {
        pub sender: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub payload: ::ethers::core::types::Bytes,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `payNativeGasForContractCallWithToken` function with signature `payNativeGasForContractCallWithToken(address,string,string,bytes,string,uint256,address)` and selector `0xc62c2002`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "payNativeGasForContractCallWithToken",
        abi = "payNativeGasForContractCallWithToken(address,string,string,bytes,string,uint256,address)"
    )]
    pub struct PayNativeGasForContractCallWithTokenCall {
        pub sender: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub payload: ::ethers::core::types::Bytes,
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `refund` function with signature `refund(address,address,uint256)` and selector `0x82ad6f35`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "refund", abi = "refund(address,address,uint256)")]
    pub struct RefundCall {
        pub receiver: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum IAxelarGasServiceCalls {
        CollectFees(CollectFeesCall),
        PayGasForContractCall(PayGasForContractCallCall),
        PayGasForContractCallWithToken(PayGasForContractCallWithTokenCall),
        PayNativeGasForContractCall(PayNativeGasForContractCallCall),
        PayNativeGasForContractCallWithToken(PayNativeGasForContractCallWithTokenCall),
        Refund(RefundCall),
    }
    impl ::ethers::core::abi::AbiDecode for IAxelarGasServiceCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CollectFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CollectFees(decoded));
            }
            if let Ok(decoded) = <PayGasForContractCallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PayGasForContractCall(decoded));
            }
            if let Ok(decoded) = <PayGasForContractCallWithTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PayGasForContractCallWithToken(decoded));
            }
            if let Ok(decoded) = <PayNativeGasForContractCallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PayNativeGasForContractCall(decoded));
            }
            if let Ok(decoded) =
                <PayNativeGasForContractCallWithTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PayNativeGasForContractCallWithToken(decoded));
            }
            if let Ok(decoded) = <RefundCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Refund(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IAxelarGasServiceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CollectFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayGasForContractCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayGasForContractCallWithToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayNativeGasForContractCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayNativeGasForContractCallWithToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Refund(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IAxelarGasServiceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CollectFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayGasForContractCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayGasForContractCallWithToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayNativeGasForContractCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayNativeGasForContractCallWithToken(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<PayGasForContractCallWithTokenCall> for IAxelarGasServiceCalls {
        fn from(value: PayGasForContractCallWithTokenCall) -> Self {
            Self::PayGasForContractCallWithToken(value)
        }
    }
    impl ::core::convert::From<PayNativeGasForContractCallCall> for IAxelarGasServiceCalls {
        fn from(value: PayNativeGasForContractCallCall) -> Self {
            Self::PayNativeGasForContractCall(value)
        }
    }
    impl ::core::convert::From<PayNativeGasForContractCallWithTokenCall> for IAxelarGasServiceCalls {
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
