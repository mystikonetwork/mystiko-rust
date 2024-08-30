pub use mock_axelar_gas_service::*;
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
pub mod mock_axelar_gas_service {
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
                    ::std::borrow::ToOwned::to_owned("contractId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("contractId"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static MOCKAXELARGASSERVICE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x0E|\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0pW`\x005`\xE0\x1C\x80c\xC6, \x02\x11a\0NW\x80c\xC6, \x02\x14a\0\xEAW\x80c\xDA\x85Mu\x14a\0\xFDW\x80c\xED\xB6\xB3\xA5\x14a\x01\x1DW\x80c\xFD\t\xE3\xBD\x14a\x01=W`\0\x80\xFD[\x80c\x0C\x93\xE3\xBB\x14a\0uW\x80c\x82\x91(l\x14a\0\x8AW\x80c\x82\xADo5\x14a\0\xCAW[`\0\x80\xFD[a\0\x88a\0\x836`\x04a\x06\x9DV[a\x01]V[\0[4\x80\x15a\0\x96W`\0\x80\xFD[P\x7F\xFA\xA2\xF0\x15\xF2\xCEZ\xEE\"Y\x04r\x8D\xE2\xDE\xF8n\xB8\x83t\x91\xEF\xD2\x1F\x1A\x04\xFC \xD8\xE9#\xF6`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xD6W`\0\x80\xFD[Pa\0\x88a\0\xE56`\x04a\x07fV[a\x01\xEBV[a\0\x88a\0\xF86`\x04a\x07\xA7V[a\x02EV[4\x80\x15a\x01\tW`\0\x80\xFD[Pa\0\x88a\x01\x186`\x04a\x08\xA4V[a\x02\xDCV[4\x80\x15a\x01)W`\0\x80\xFD[Pa\0\x88a\x0186`\x04a\t\xCFV[a\x03\xDEV[4\x80\x15a\x01IW`\0\x80\xFD[Pa\0\x88a\x01X6`\x04a\n\xE9V[a\x04`V[4`\0\x03a\x01~W`@Qc\xB5\xC7J'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x82`@Qa\x01\x8E\x92\x91\x90a\x0B\xC9V[`@Q\x80\x91\x03\x90 \x88`\x01`\x01`\xA0\x1B\x03\x16\x7Fas2\xC1\x83 X\xDFn\xE4_\xCB\xDFG\x12QGL\x99E\xA8\xE5\xD2)(z!\xA5\xF6|\xCF\n\x89\x89\x89\x894\x88`@Qa\x01\xD9\x96\x95\x94\x93\x92\x91\x90a\x0C\x02V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x025W`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02/W=`\0\x80>=`\0\xFD[PPPPV[a\x02@\x82\x84\x83a\x04\xDCV[PPPV[4`\0\x03a\x02fW`@Qc\xB5\xC7J'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x85`@Qa\x02v\x92\x91\x90a\x0B\xC9V[`@Q\x80\x91\x03\x90 \x8B`\x01`\x01`\xA0\x1B\x03\x16\x7F\x99\x9DC\x1BXv\x12\x13\xCFS\xAF\x96&+g\xA0i\xCB\xD9cI\x9F\xD8\xEF\xFD\x1E!Ub\x17\xB8A\x8C\x8C\x8C\x8C\x8A\x8A\x8A4\x8B`@Qa\x02\xC7\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0CMV[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPV[`\0[\x81\x81\x10\x15a\x02/W`\0\x83\x83\x83\x81\x81\x10a\x02\xFBWa\x02\xFBa\x0C\xB4V[\x90P` \x02\x01` \x81\x01\x90a\x03\x10\x91\x90a\x0C\xCAV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03[W`@Q`\x01`\x01`\xA0\x1B\x03\x86\x16\x90G\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x03UW=`\0\x80>=`\0\xFD[Pa\x03\xD5V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xC6\x91\x90a\x0C\xEEV[\x90Pa\x03\xD3\x82\x87\x83a\x04\xDCV[P[P`\x01\x01a\x02\xDFV[a\x03\xE9\x833\x84a\x05\xF4V[\x86\x86`@Qa\x03\xF9\x92\x91\x90a\x0B\xC9V[`@Q\x80\x91\x03\x90 \x8C`\x01`\x01`\xA0\x1B\x03\x16\x7F\x88u\xF9vO(\xFA\x82\xD3\xE7\xFF\x1B\x80\xBD\\\x8Ff^\x1FB\xFC\xD8\xC2\xFA\xEB\xC7\xC4\0\xA4\xBA\x1B\xBD\x8D\x8D\x8D\x8D\x8B\x8B\x8B\x8B\x8B`@Qa\x04J\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\r+V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPPV[a\x04k\x833\x84a\x05\xF4V[\x84\x84`@Qa\x04{\x92\x91\x90a\x0B\xC9V[`@Q\x80\x91\x03\x90 \x8A`\x01`\x01`\xA0\x1B\x03\x16\x7F\x99 g`\xF0\xBE\x19\xDD\t7)\xBD5\xE5\x92M\xAF\xF5\xE2\x17\xBC\xED\xC5\">\xD0g\xB6\0\x08\xCF\x8A\x8B\x8B\x8B\x8B\x89\x89\x89`@Qa\x04\xC8\x97\x96\x95\x94\x93\x92\x91\x90a\r\xB4V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R`\0\x91\x82\x91\x86\x16\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x05I\x91\x90a\x0E\x08V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x05\x86W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\x8BV[``\x91P[P\x91P\x91P`\0\x82\x80\x15a\x05\xB7WP\x81Q\x15\x80a\x05\xB7WP\x81\x80` \x01\x90Q\x81\x01\x90a\x05\xB7\x91\x90a\x0E$V[\x90P\x80\x15\x80a\x05\xCEWP`\x01`\x01`\xA0\x1B\x03\x86\x16;\x15[\x15a\x05\xECW`@Qc\x12\x17\x1D\x83`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R0`D\x83\x01R`d\x82\x01\x83\x90R`\0\x91\x82\x91\x86\x16\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01a\x05\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06BW`\0\x80\xFD[PV[\x805a\x06P\x81a\x06-V[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x06gW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06~W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x06\x96W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15a\x06\xB9W`\0\x80\xFD[\x885a\x06\xC4\x81a\x06-V[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xDFW`\0\x80\xFD[a\x06\xEB\x8B\x82\x8C\x01a\x06UV[\x90\x98P\x96PP`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\nW`\0\x80\xFD[a\x07\x16\x8B\x82\x8C\x01a\x06UV[\x90\x96P\x94PP``\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x075W`\0\x80\xFD[a\x07A\x8B\x82\x8C\x01a\x06UV[\x90\x94P\x92PP`\x80\x89\x015a\x07U\x81a\x06-V[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x07{W`\0\x80\xFD[\x835a\x07\x86\x81a\x06-V[\x92P` \x84\x015a\x07\x96\x81a\x06-V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xE0\x8C\x8E\x03\x12\x15a\x07\xC8W`\0\x80\xFD[a\x07\xD1\x8Ca\x06EV[\x9AP` \x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xECW`\0\x80\xFD[a\x07\xF8\x8E\x82\x8F\x01a\x06UV[\x90\x9BP\x99PP`@\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x17W`\0\x80\xFD[a\x08#\x8E\x82\x8F\x01a\x06UV[\x90\x99P\x97PP``\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08BW`\0\x80\xFD[a\x08N\x8E\x82\x8F\x01a\x06UV[\x90\x97P\x95PP`\x80\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08mW`\0\x80\xFD[a\x08y\x8E\x82\x8F\x01a\x06UV[\x90\x95P\x93PP`\xA0\x8C\x015\x91Pa\x08\x92`\xC0\x8D\x01a\x06EV[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x08\xB9W`\0\x80\xFD[\x835a\x08\xC4\x81a\x06-V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xDFW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x08\xF0W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\x06W`\0\x80\xFD[\x86` \x82`\x05\x1B\x84\x01\x01\x11\x15a\t\x1BW`\0\x80\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\tSW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\tlWa\tla\t,V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\t\x9AWa\t\x9Aa\t,V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\t\xB2W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01 \x8D\x8F\x03\x12\x15a\t\xF2W`\0\x80\xFD[a\t\xFB\x8Da\x06EV[\x9BP`\x01`\x01`@\x1B\x03` \x8E\x015\x11\x15a\n\x15W`\0\x80\xFD[a\n%\x8E` \x8F\x015\x8F\x01a\x06UV[\x90\x9BP\x99P`\x01`\x01`@\x1B\x03`@\x8E\x015\x11\x15a\nBW`\0\x80\xFD[a\nR\x8E`@\x8F\x015\x8F\x01a\x06UV[\x90\x99P\x97P`\x01`\x01`@\x1B\x03``\x8E\x015\x11\x15a\noW`\0\x80\xFD[a\n\x7F\x8E``\x8F\x015\x8F\x01a\x06UV[\x90\x97P\x95P`\x01`\x01`@\x1B\x03`\x80\x8E\x015\x11\x15a\n\x9CW`\0\x80\xFD[a\n\xAC\x8E`\x80\x8F\x015\x8F\x01a\tBV[\x94P`\xA0\x8D\x015\x93Pa\n\xC1`\xC0\x8E\x01a\x06EV[\x92P`\xE0\x8D\x015\x91Pa\n\xD7a\x01\0\x8E\x01a\x06EV[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9BP\x92\x95\x98\x9BV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x8B\x8D\x03\x12\x15a\x0B\x08W`\0\x80\xFD[\x8A5a\x0B\x13\x81a\x06-V[\x99P` \x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B.W`\0\x80\xFD[a\x0B:\x8D\x82\x8E\x01a\x06UV[\x90\x9AP\x98PP`@\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BYW`\0\x80\xFD[a\x0Be\x8D\x82\x8E\x01a\x06UV[\x90\x98P\x96PP``\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x84W`\0\x80\xFD[a\x0B\x90\x8D\x82\x8E\x01a\x06UV[\x90\x96P\x94Pa\x0B\xA3\x90P`\x80\x8C\x01a\x06EV[\x92P`\xA0\x8B\x015\x91Pa\x0B\xB8`\xC0\x8C\x01a\x06EV[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x80\x81R`\0a\x0C\x16`\x80\x83\x01\x88\x8Aa\x0B\xD9V[\x82\x81\x03` \x84\x01Ra\x0C)\x81\x87\x89a\x0B\xD9V[`@\x84\x01\x95\x90\x95RPP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16``\x90\x91\x01R\x94\x93PPPPV[`\xC0\x81R`\0a\x0Ca`\xC0\x83\x01\x8B\x8Da\x0B\xD9V[\x82\x81\x03` \x84\x01Ra\x0Ct\x81\x8A\x8Ca\x0B\xD9V[\x90P\x82\x81\x03`@\x84\x01Ra\x0C\x89\x81\x88\x8Aa\x0B\xD9V[``\x84\x01\x96\x90\x96RPP`\x80\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x90\x91\x01R\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\xDCW`\0\x80\xFD[\x815a\x0C\xE7\x81a\x06-V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\r\0W`\0\x80\xFD[PQ\x91\x90PV[`\0[\x83\x81\x10\x15a\r\"W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\nV[PP`\0\x91\x01RV[`\xE0\x81R`\0a\r?`\xE0\x83\x01\x8B\x8Da\x0B\xD9V[\x82\x81\x03` \x84\x01Ra\rR\x81\x8A\x8Ca\x0B\xD9V[\x90P\x82\x81\x03`@\x84\x01R\x87Q\x80\x82Ra\rr\x81` \x84\x01` \x8C\x01a\r\x07V[``\x84\x01\x97\x90\x97R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x80\x84\x01R`\xA0\x83\x01\x94\x90\x94RP\x92\x16`\xC0\x90\x92\x01\x91\x90\x91R` `\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x95\x94PPPPPV[`\xA0\x81R`\0a\r\xC8`\xA0\x83\x01\x89\x8Ba\x0B\xD9V[\x82\x81\x03` \x84\x01Ra\r\xDB\x81\x88\x8Aa\x0B\xD9V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`@\x85\x01R``\x84\x01\x95\x90\x95RPP\x92\x16`\x80\x90\x92\x01\x91\x90\x91R\x94\x93PPPPV[`\0\x82Qa\x0E\x1A\x81\x84` \x87\x01a\r\x07V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0E6W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0C\xE7W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xA2\xD00\x8D\xD6\x9F\x14\x12\xB3s\xADy\x15D\xBD\x8A*h|\xFEu\xA3r8\x8F\x04\xB7\x92\xFE\x0Fr\xB3dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MOCKAXELARGASSERVICE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0pW`\x005`\xE0\x1C\x80c\xC6, \x02\x11a\0NW\x80c\xC6, \x02\x14a\0\xEAW\x80c\xDA\x85Mu\x14a\0\xFDW\x80c\xED\xB6\xB3\xA5\x14a\x01\x1DW\x80c\xFD\t\xE3\xBD\x14a\x01=W`\0\x80\xFD[\x80c\x0C\x93\xE3\xBB\x14a\0uW\x80c\x82\x91(l\x14a\0\x8AW\x80c\x82\xADo5\x14a\0\xCAW[`\0\x80\xFD[a\0\x88a\0\x836`\x04a\x06\x9DV[a\x01]V[\0[4\x80\x15a\0\x96W`\0\x80\xFD[P\x7F\xFA\xA2\xF0\x15\xF2\xCEZ\xEE\"Y\x04r\x8D\xE2\xDE\xF8n\xB8\x83t\x91\xEF\xD2\x1F\x1A\x04\xFC \xD8\xE9#\xF6`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xD6W`\0\x80\xFD[Pa\0\x88a\0\xE56`\x04a\x07fV[a\x01\xEBV[a\0\x88a\0\xF86`\x04a\x07\xA7V[a\x02EV[4\x80\x15a\x01\tW`\0\x80\xFD[Pa\0\x88a\x01\x186`\x04a\x08\xA4V[a\x02\xDCV[4\x80\x15a\x01)W`\0\x80\xFD[Pa\0\x88a\x0186`\x04a\t\xCFV[a\x03\xDEV[4\x80\x15a\x01IW`\0\x80\xFD[Pa\0\x88a\x01X6`\x04a\n\xE9V[a\x04`V[4`\0\x03a\x01~W`@Qc\xB5\xC7J'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x82`@Qa\x01\x8E\x92\x91\x90a\x0B\xC9V[`@Q\x80\x91\x03\x90 \x88`\x01`\x01`\xA0\x1B\x03\x16\x7Fas2\xC1\x83 X\xDFn\xE4_\xCB\xDFG\x12QGL\x99E\xA8\xE5\xD2)(z!\xA5\xF6|\xCF\n\x89\x89\x89\x894\x88`@Qa\x01\xD9\x96\x95\x94\x93\x92\x91\x90a\x0C\x02V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x025W`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02/W=`\0\x80>=`\0\xFD[PPPPV[a\x02@\x82\x84\x83a\x04\xDCV[PPPV[4`\0\x03a\x02fW`@Qc\xB5\xC7J'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x85`@Qa\x02v\x92\x91\x90a\x0B\xC9V[`@Q\x80\x91\x03\x90 \x8B`\x01`\x01`\xA0\x1B\x03\x16\x7F\x99\x9DC\x1BXv\x12\x13\xCFS\xAF\x96&+g\xA0i\xCB\xD9cI\x9F\xD8\xEF\xFD\x1E!Ub\x17\xB8A\x8C\x8C\x8C\x8C\x8A\x8A\x8A4\x8B`@Qa\x02\xC7\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0CMV[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPV[`\0[\x81\x81\x10\x15a\x02/W`\0\x83\x83\x83\x81\x81\x10a\x02\xFBWa\x02\xFBa\x0C\xB4V[\x90P` \x02\x01` \x81\x01\x90a\x03\x10\x91\x90a\x0C\xCAV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03[W`@Q`\x01`\x01`\xA0\x1B\x03\x86\x16\x90G\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x03UW=`\0\x80>=`\0\xFD[Pa\x03\xD5V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xC6\x91\x90a\x0C\xEEV[\x90Pa\x03\xD3\x82\x87\x83a\x04\xDCV[P[P`\x01\x01a\x02\xDFV[a\x03\xE9\x833\x84a\x05\xF4V[\x86\x86`@Qa\x03\xF9\x92\x91\x90a\x0B\xC9V[`@Q\x80\x91\x03\x90 \x8C`\x01`\x01`\xA0\x1B\x03\x16\x7F\x88u\xF9vO(\xFA\x82\xD3\xE7\xFF\x1B\x80\xBD\\\x8Ff^\x1FB\xFC\xD8\xC2\xFA\xEB\xC7\xC4\0\xA4\xBA\x1B\xBD\x8D\x8D\x8D\x8D\x8B\x8B\x8B\x8B\x8B`@Qa\x04J\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\r+V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPPV[a\x04k\x833\x84a\x05\xF4V[\x84\x84`@Qa\x04{\x92\x91\x90a\x0B\xC9V[`@Q\x80\x91\x03\x90 \x8A`\x01`\x01`\xA0\x1B\x03\x16\x7F\x99 g`\xF0\xBE\x19\xDD\t7)\xBD5\xE5\x92M\xAF\xF5\xE2\x17\xBC\xED\xC5\">\xD0g\xB6\0\x08\xCF\x8A\x8B\x8B\x8B\x8B\x89\x89\x89`@Qa\x04\xC8\x97\x96\x95\x94\x93\x92\x91\x90a\r\xB4V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R`\0\x91\x82\x91\x86\x16\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x05I\x91\x90a\x0E\x08V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x05\x86W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\x8BV[``\x91P[P\x91P\x91P`\0\x82\x80\x15a\x05\xB7WP\x81Q\x15\x80a\x05\xB7WP\x81\x80` \x01\x90Q\x81\x01\x90a\x05\xB7\x91\x90a\x0E$V[\x90P\x80\x15\x80a\x05\xCEWP`\x01`\x01`\xA0\x1B\x03\x86\x16;\x15[\x15a\x05\xECW`@Qc\x12\x17\x1D\x83`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R0`D\x83\x01R`d\x82\x01\x83\x90R`\0\x91\x82\x91\x86\x16\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01a\x05\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06BW`\0\x80\xFD[PV[\x805a\x06P\x81a\x06-V[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x06gW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06~W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x06\x96W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15a\x06\xB9W`\0\x80\xFD[\x885a\x06\xC4\x81a\x06-V[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xDFW`\0\x80\xFD[a\x06\xEB\x8B\x82\x8C\x01a\x06UV[\x90\x98P\x96PP`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\nW`\0\x80\xFD[a\x07\x16\x8B\x82\x8C\x01a\x06UV[\x90\x96P\x94PP``\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x075W`\0\x80\xFD[a\x07A\x8B\x82\x8C\x01a\x06UV[\x90\x94P\x92PP`\x80\x89\x015a\x07U\x81a\x06-V[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x07{W`\0\x80\xFD[\x835a\x07\x86\x81a\x06-V[\x92P` \x84\x015a\x07\x96\x81a\x06-V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xE0\x8C\x8E\x03\x12\x15a\x07\xC8W`\0\x80\xFD[a\x07\xD1\x8Ca\x06EV[\x9AP` \x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xECW`\0\x80\xFD[a\x07\xF8\x8E\x82\x8F\x01a\x06UV[\x90\x9BP\x99PP`@\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x17W`\0\x80\xFD[a\x08#\x8E\x82\x8F\x01a\x06UV[\x90\x99P\x97PP``\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08BW`\0\x80\xFD[a\x08N\x8E\x82\x8F\x01a\x06UV[\x90\x97P\x95PP`\x80\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08mW`\0\x80\xFD[a\x08y\x8E\x82\x8F\x01a\x06UV[\x90\x95P\x93PP`\xA0\x8C\x015\x91Pa\x08\x92`\xC0\x8D\x01a\x06EV[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x08\xB9W`\0\x80\xFD[\x835a\x08\xC4\x81a\x06-V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xDFW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x08\xF0W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\x06W`\0\x80\xFD[\x86` \x82`\x05\x1B\x84\x01\x01\x11\x15a\t\x1BW`\0\x80\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\tSW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\tlWa\tla\t,V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\t\x9AWa\t\x9Aa\t,V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\t\xB2W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01 \x8D\x8F\x03\x12\x15a\t\xF2W`\0\x80\xFD[a\t\xFB\x8Da\x06EV[\x9BP`\x01`\x01`@\x1B\x03` \x8E\x015\x11\x15a\n\x15W`\0\x80\xFD[a\n%\x8E` \x8F\x015\x8F\x01a\x06UV[\x90\x9BP\x99P`\x01`\x01`@\x1B\x03`@\x8E\x015\x11\x15a\nBW`\0\x80\xFD[a\nR\x8E`@\x8F\x015\x8F\x01a\x06UV[\x90\x99P\x97P`\x01`\x01`@\x1B\x03``\x8E\x015\x11\x15a\noW`\0\x80\xFD[a\n\x7F\x8E``\x8F\x015\x8F\x01a\x06UV[\x90\x97P\x95P`\x01`\x01`@\x1B\x03`\x80\x8E\x015\x11\x15a\n\x9CW`\0\x80\xFD[a\n\xAC\x8E`\x80\x8F\x015\x8F\x01a\tBV[\x94P`\xA0\x8D\x015\x93Pa\n\xC1`\xC0\x8E\x01a\x06EV[\x92P`\xE0\x8D\x015\x91Pa\n\xD7a\x01\0\x8E\x01a\x06EV[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9BP\x92\x95\x98\x9BV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x8B\x8D\x03\x12\x15a\x0B\x08W`\0\x80\xFD[\x8A5a\x0B\x13\x81a\x06-V[\x99P` \x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B.W`\0\x80\xFD[a\x0B:\x8D\x82\x8E\x01a\x06UV[\x90\x9AP\x98PP`@\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BYW`\0\x80\xFD[a\x0Be\x8D\x82\x8E\x01a\x06UV[\x90\x98P\x96PP``\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x84W`\0\x80\xFD[a\x0B\x90\x8D\x82\x8E\x01a\x06UV[\x90\x96P\x94Pa\x0B\xA3\x90P`\x80\x8C\x01a\x06EV[\x92P`\xA0\x8B\x015\x91Pa\x0B\xB8`\xC0\x8C\x01a\x06EV[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x80\x81R`\0a\x0C\x16`\x80\x83\x01\x88\x8Aa\x0B\xD9V[\x82\x81\x03` \x84\x01Ra\x0C)\x81\x87\x89a\x0B\xD9V[`@\x84\x01\x95\x90\x95RPP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16``\x90\x91\x01R\x94\x93PPPPV[`\xC0\x81R`\0a\x0Ca`\xC0\x83\x01\x8B\x8Da\x0B\xD9V[\x82\x81\x03` \x84\x01Ra\x0Ct\x81\x8A\x8Ca\x0B\xD9V[\x90P\x82\x81\x03`@\x84\x01Ra\x0C\x89\x81\x88\x8Aa\x0B\xD9V[``\x84\x01\x96\x90\x96RPP`\x80\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x90\x91\x01R\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\xDCW`\0\x80\xFD[\x815a\x0C\xE7\x81a\x06-V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\r\0W`\0\x80\xFD[PQ\x91\x90PV[`\0[\x83\x81\x10\x15a\r\"W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\nV[PP`\0\x91\x01RV[`\xE0\x81R`\0a\r?`\xE0\x83\x01\x8B\x8Da\x0B\xD9V[\x82\x81\x03` \x84\x01Ra\rR\x81\x8A\x8Ca\x0B\xD9V[\x90P\x82\x81\x03`@\x84\x01R\x87Q\x80\x82Ra\rr\x81` \x84\x01` \x8C\x01a\r\x07V[``\x84\x01\x97\x90\x97R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x80\x84\x01R`\xA0\x83\x01\x94\x90\x94RP\x92\x16`\xC0\x90\x92\x01\x91\x90\x91R` `\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x95\x94PPPPPV[`\xA0\x81R`\0a\r\xC8`\xA0\x83\x01\x89\x8Ba\x0B\xD9V[\x82\x81\x03` \x84\x01Ra\r\xDB\x81\x88\x8Aa\x0B\xD9V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`@\x85\x01R``\x84\x01\x95\x90\x95RPP\x92\x16`\x80\x90\x92\x01\x91\x90\x91R\x94\x93PPPPV[`\0\x82Qa\x0E\x1A\x81\x84` \x87\x01a\r\x07V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0E6W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0C\xE7W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xA2\xD00\x8D\xD6\x9F\x14\x12\xB3s\xADy\x15D\xBD\x8A*h|\xFEu\xA3r8\x8F\x04\xB7\x92\xFE\x0Fr\xB3dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKAXELARGASSERVICE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockAxelarGasService<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockAxelarGasService<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockAxelarGasService<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockAxelarGasService<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockAxelarGasService<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockAxelarGasService))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockAxelarGasService<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKAXELARGASSERVICE_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MOCKAXELARGASSERVICE_ABI.clone(),
                MOCKAXELARGASSERVICE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `contractId` (0x8291286c) function
        pub fn contract_id(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([130, 145, 40, 108], ())
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
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MockAxelarGasServiceEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockAxelarGasService<M> {
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
    pub enum MockAxelarGasServiceErrors {
        NothingReceived(NothingReceived),
        TransferFailed(TransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockAxelarGasServiceErrors {
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
    impl ::ethers::core::abi::AbiEncode for MockAxelarGasServiceErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::NothingReceived(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockAxelarGasServiceErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <NothingReceived as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <TransferFailed as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockAxelarGasServiceErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NothingReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockAxelarGasServiceErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<NothingReceived> for MockAxelarGasServiceErrors {
        fn from(value: NothingReceived) -> Self {
            Self::NothingReceived(value)
        }
    }
    impl ::core::convert::From<TransferFailed> for MockAxelarGasServiceErrors {
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
    pub enum MockAxelarGasServiceEvents {
        GasPaidForContractCallFilter(GasPaidForContractCallFilter),
        GasPaidForContractCallWithTokenFilter(GasPaidForContractCallWithTokenFilter),
        NativeGasPaidForContractCallFilter(NativeGasPaidForContractCallFilter),
        NativeGasPaidForContractCallWithTokenFilter(NativeGasPaidForContractCallWithTokenFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockAxelarGasServiceEvents {
        fn decode_log(log: &::ethers::core::abi::RawLog) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = GasPaidForContractCallFilter::decode_log(log) {
                return Ok(MockAxelarGasServiceEvents::GasPaidForContractCallFilter(decoded));
            }
            if let Ok(decoded) = GasPaidForContractCallWithTokenFilter::decode_log(log) {
                return Ok(MockAxelarGasServiceEvents::GasPaidForContractCallWithTokenFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NativeGasPaidForContractCallFilter::decode_log(log) {
                return Ok(MockAxelarGasServiceEvents::NativeGasPaidForContractCallFilter(decoded));
            }
            if let Ok(decoded) = NativeGasPaidForContractCallWithTokenFilter::decode_log(log) {
                return Ok(MockAxelarGasServiceEvents::NativeGasPaidForContractCallWithTokenFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockAxelarGasServiceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GasPaidForContractCallFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GasPaidForContractCallWithTokenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeGasPaidForContractCallFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeGasPaidForContractCallWithTokenFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GasPaidForContractCallFilter> for MockAxelarGasServiceEvents {
        fn from(value: GasPaidForContractCallFilter) -> Self {
            Self::GasPaidForContractCallFilter(value)
        }
    }
    impl ::core::convert::From<GasPaidForContractCallWithTokenFilter> for MockAxelarGasServiceEvents {
        fn from(value: GasPaidForContractCallWithTokenFilter) -> Self {
            Self::GasPaidForContractCallWithTokenFilter(value)
        }
    }
    impl ::core::convert::From<NativeGasPaidForContractCallFilter> for MockAxelarGasServiceEvents {
        fn from(value: NativeGasPaidForContractCallFilter) -> Self {
            Self::NativeGasPaidForContractCallFilter(value)
        }
    }
    impl ::core::convert::From<NativeGasPaidForContractCallWithTokenFilter> for MockAxelarGasServiceEvents {
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
    ///Container type for all input parameters for the `contractId` function with signature `contractId()` and selector `0x8291286c`
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
    #[ethcall(name = "contractId", abi = "contractId()")]
    pub struct ContractIdCall;
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
    pub enum MockAxelarGasServiceCalls {
        CollectFees(CollectFeesCall),
        ContractId(ContractIdCall),
        PayGasForContractCall(PayGasForContractCallCall),
        PayGasForContractCallWithToken(PayGasForContractCallWithTokenCall),
        PayNativeGasForContractCall(PayNativeGasForContractCallCall),
        PayNativeGasForContractCallWithToken(PayNativeGasForContractCallWithTokenCall),
        Refund(RefundCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockAxelarGasServiceCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CollectFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CollectFees(decoded));
            }
            if let Ok(decoded) = <ContractIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ContractId(decoded));
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
    impl ::ethers::core::abi::AbiEncode for MockAxelarGasServiceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CollectFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ContractId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayGasForContractCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayGasForContractCallWithToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayNativeGasForContractCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayNativeGasForContractCallWithToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Refund(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockAxelarGasServiceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CollectFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractId(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayGasForContractCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayGasForContractCallWithToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayNativeGasForContractCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayNativeGasForContractCallWithToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Refund(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CollectFeesCall> for MockAxelarGasServiceCalls {
        fn from(value: CollectFeesCall) -> Self {
            Self::CollectFees(value)
        }
    }
    impl ::core::convert::From<ContractIdCall> for MockAxelarGasServiceCalls {
        fn from(value: ContractIdCall) -> Self {
            Self::ContractId(value)
        }
    }
    impl ::core::convert::From<PayGasForContractCallCall> for MockAxelarGasServiceCalls {
        fn from(value: PayGasForContractCallCall) -> Self {
            Self::PayGasForContractCall(value)
        }
    }
    impl ::core::convert::From<PayGasForContractCallWithTokenCall> for MockAxelarGasServiceCalls {
        fn from(value: PayGasForContractCallWithTokenCall) -> Self {
            Self::PayGasForContractCallWithToken(value)
        }
    }
    impl ::core::convert::From<PayNativeGasForContractCallCall> for MockAxelarGasServiceCalls {
        fn from(value: PayNativeGasForContractCallCall) -> Self {
            Self::PayNativeGasForContractCall(value)
        }
    }
    impl ::core::convert::From<PayNativeGasForContractCallWithTokenCall> for MockAxelarGasServiceCalls {
        fn from(value: PayNativeGasForContractCallWithTokenCall) -> Self {
            Self::PayNativeGasForContractCallWithToken(value)
        }
    }
    impl ::core::convert::From<RefundCall> for MockAxelarGasServiceCalls {
        fn from(value: RefundCall) -> Self {
            Self::Refund(value)
        }
    }
    ///Container type for all return fields from the `contractId` function with signature `contractId()` and selector `0x8291286c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ContractIdReturn(pub [u8; 32]);
}
