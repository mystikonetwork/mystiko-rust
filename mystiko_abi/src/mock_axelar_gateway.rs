pub use mock_axelar_gateway::*;
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
pub mod mock_axelar_gateway {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("adminEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("adminEpoch"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("adminThreshold"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("adminThreshold"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("epoch"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("admins"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("admins"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("epoch"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ),),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address[]"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allTokensFrozen"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allTokensFrozen"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("callContract"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("callContract"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("callContractWithToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("callContractWithToken",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
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
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("chainNameA"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("chainNameA"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("chainNameB"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("chainNameB"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("contractAddressA"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("contractAddressA"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("contractAddressB"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("contractAddressB"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("execute"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("freezeAllTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("freezeAllTokens"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("freezeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("freezeToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("implementation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("implementation"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isCommandExecuted"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isCommandExecuted"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("commandId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isContractCallAndMintApproved"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isContractCallAndMintApproved",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("commandId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
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
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isContractCallApproved"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isContractCallApproved",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("commandId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sendToken"),
                        inputs: ::std::vec![
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
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setChainPair"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setChainPair"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainNameA"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_contractAddressA"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainNameB"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_contractAddressB"),
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
                    ::std::borrow::ToOwned::to_owned("setup"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setup"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenAddresses"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenAddresses"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenFrozen"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenFrozen"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unfreezeAllTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unfreezeAllTokens"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unfreezeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unfreezeToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgrade"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("upgrade"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newImplementationCodeHash",),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("setupParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateContractCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateContractCall",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("commandId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateContractCallAndMint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateContractCallAndMint",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("commandId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
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
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccountBlacklisted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AccountBlacklisted"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccountWhitelisted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AccountWhitelisted"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllTokensFrozen"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AllTokensFrozen"),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllTokensUnfrozen"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AllTokensUnfrozen"),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ContractCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ContractCall"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationContractAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ContractCallApproved"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ContractCallApproved",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("commandId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceTxHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceEventIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ContractCallApprovedWithMint"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ContractCallApprovedWithMint",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("commandId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
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
                                name: ::std::borrow::ToOwned::to_owned("sourceTxHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sourceEventIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ContractCallWithToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ContractCallWithToken",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationContractAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
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
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Executed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Executed"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("commandId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenDeployed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TokenDeployed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("symbol"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenAddresses"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenFrozen"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TokenFrozen"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenSent"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TokenSent"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
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
                                name: ::std::borrow::ToOwned::to_owned("symbol"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenUnfrozen"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TokenUnfrozen"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKAXELARGATEWAY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x0E\xC0\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA9W`\x005`\xE0\x1C\x80c\x93[\x13\xF6\x11a\0\xF9W\x80c\xBB\xFBR\xFB\x11a\0\x97W\x80c\xD2\xBC7\xF8\x11a\0qW\x80c\xD2\xBC7\xF8\x14a\x01\xC0W\x80c\xD3\x13|\x85\x14a\x03mW\x80c\xE3\xDF\xA2\x99\x14a\x01\xC0W\x80c\xF6\xA5\xF9\xF5\x14a\x03uW`\0\x80\xFD[\x80c\xBB\xFBR\xFB\x14a\x03.W\x80c\xBC\0\xC2\x16\x14a\x03AW\x80c\xD2o\xF2\x10\x14a\x03_W`\0\x80\xFD[\x80c\xAA\x1E\x1F\n\x11a\0\xD3W\x80c\xAA\x1E\x1F\n\x14a\x02\xE6W\x80c\xAD\xD0\xA5S\x14a\x02\xEDW\x80c\xB5+8\xE1\x14a\x03\0W\x80c\xB5Ap\x84\x14a\x03\x15W`\0\x80\xFD[\x80c\x93[\x13\xF6\x14a\x02\xC4W\x80c\x9D\xED\x06\xDF\x14a\x01\xAEW\x80c\xA3I\x9Cs\x14a\x02\xD2W`\0\x80\xFD[\x80c4\xFFi\x83\x11a\x01fW\x80c_ip\xC3\x11a\x01@W\x80c_ip\xC3\x14a\x02\x82W\x80cdl]4\x14a\x01\xAEW\x80c{\x1Bv\x9E\x14a\x02\x9CW\x80c\x88\xB3\x05\x87\x14a\x02\xB0W`\0\x80\xFD[\x80c4\xFFi\x83\x14a\x01\xAEW\x80c6I@\xD8\x14a\x02jW\x80c\\`\xDA\x1B\x14a\x02{W`\0\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\x01\xAEW\x80c\x10\xEFw\x95\x14a\x01\xC2W\x80c\x14\xBF\xD6\xD0\x14a\x01\xF2W\x80c\x18v\xEE\xD9\x14a\x02\x13W\x80c\x1C\x92\x11_\x14a\x02@W\x80c&\xEFi\x9D\x14a\x02SW[`\0\x80\xFD[a\x01\xC0a\x01\xBC6`\x04a\x05@V[PPV[\0[`\x01Ta\x01\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x06a\x02\x006`\x04a\x05\x81V[P``\x90V[`@Qa\x01\xE9\x91\x90a\x05\x9AV[a\x020a\x02!6`\x04a\x05\xE6V[`\x01\x99\x98PPPPPPPPPV[`@Q\x90\x15\x15\x81R` \x01a\x01\xE9V[a\x01\xC0a\x02N6`\x04a\x06\xA4V[a\x03\x90V[a\x01\xC0a\x02a6`\x04a\x07GV[PPPPPPPV[`\0[`@Q\x90\x81R` \x01a\x01\xE9V[`\0a\x01\xD5V[a\x020a\x02\x906`\x04a\x07\xF3V[`\x01\x96\x95PPPPPPV[a\x020a\x02\xAA6`\x04a\x08\x8EV[P`\x01\x90V[a\x02ma\x02\xBE6`\x04a\x05\x81V[P`\0\x90V[a\x01\xD5a\x02\xBE6`\x04a\x08\x8EV[a\x01\xC0a\x02\xE06`\x04a\t`V[PPPPV[`\x01a\x020V[a\x01\xC0a\x02\xFB6`\x04a\t\xB9V[a\x03\xFFV[a\x03\x08a\x04]V[`@Qa\x01\xE9\x91\x90a\nMV[a\x01\xC0a\x03#6`\x04a\n\x9BV[PPPPPPPPPV[`\x03Ta\x01\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x020a\x03O6`\x04a\x0BvV[`\x01\x9A\x99PPPPPPPPPPV[a\x020a\x02\xAA6`\x04a\x05\x81V[a\x03\x08a\x04\xEBV[a\x020a\x03\x836`\x04a\x0CJV[`\x01\x97\x96PPPPPPPV[`\x03T`@Qc\t\"\xC0\xCB`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cI\x16\x06X\x90a\x03\xC5\x90`\0\x90\x86\x90\x86\x90`\x04\x01a\x0C\xE4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xDFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xF3W=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\0a\x04\x0C\x86\x88\x83a\r\xCBV[P`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x90U`\x02a\x045\x83\x85\x83a\r\xCBV[P`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[`\x02\x80Ta\x04j\x90a\rBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x96\x90a\rBV[\x80\x15a\x04\xE3W\x80`\x1F\x10a\x04\xB8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xE3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xC6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x80Ta\x04j\x90a\rBV[`\0\x80\x83`\x1F\x84\x01\x12a\x05\nW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05!W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x059W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x05SW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05iW`\0\x80\xFD[a\x05u\x85\x82\x86\x01a\x04\xF8V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x05\x93W`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x05\xDBW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x05\xB4V[P\x90\x95\x94PPPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xC0\x8A\x8C\x03\x12\x15a\x06\x04W`\0\x80\xFD[\x895\x98P` \x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06!W`\0\x80\xFD[a\x06-\x8C\x82\x8D\x01a\x04\xF8V[\x90\x99P\x97PP`@\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06LW`\0\x80\xFD[a\x06X\x8C\x82\x8D\x01a\x04\xF8V[\x90\x97P\x95PP``\x8A\x015\x93P`\x80\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06~W`\0\x80\xFD[a\x06\x8A\x8C\x82\x8D\x01a\x04\xF8V[\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x95\x98\x94\x97\x96`\xA0\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a\x06\xBDW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xD3W`\0\x80\xFD[a\x06\xDF\x89\x82\x8A\x01a\x04\xF8V[\x90\x97P\x95PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xFEW`\0\x80\xFD[a\x07\n\x89\x82\x8A\x01a\x04\xF8V[\x90\x95P\x93PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07)W`\0\x80\xFD[a\x075\x89\x82\x8A\x01a\x04\xF8V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\x07bW`\0\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07xW`\0\x80\xFD[a\x07\x84\x8A\x82\x8B\x01a\x04\xF8V[\x90\x98P\x96PP` \x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xA3W`\0\x80\xFD[a\x07\xAF\x8A\x82\x8B\x01a\x04\xF8V[\x90\x96P\x94PP`@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xCEW`\0\x80\xFD[a\x07\xDA\x8A\x82\x8B\x01a\x04\xF8V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96``\x90\x95\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x08\x0CW`\0\x80\xFD[\x865\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08)W`\0\x80\xFD[a\x085\x89\x82\x8A\x01a\x04\xF8V[\x90\x96P\x94PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08TW`\0\x80\xFD[a\x08`\x89\x82\x8A\x01a\x04\xF8V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xA0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xB6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x08\xC7W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xE0Wa\x08\xE0a\x08xV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\t\x0EWa\t\x0Ea\x08xV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\t&W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t[W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\tvW`\0\x80\xFD[a\t\x7F\x85a\tDV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xA1W`\0\x80\xFD[a\t\xAD\x87\x82\x88\x01a\x04\xF8V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\t\xD2W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xE8W`\0\x80\xFD[a\t\xF4\x89\x82\x8A\x01a\x04\xF8V[\x90\x97P\x95Pa\n\x07\x90P` \x88\x01a\tDV[\x93P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\"W`\0\x80\xFD[a\n.\x89\x82\x8A\x01a\x04\xF8V[\x90\x94P\x92Pa\nA\x90P``\x88\x01a\tDV[\x90P\x92\x95P\x92\x95P\x92\x95V[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\n{W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\n^V[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xA0\x8A\x8C\x03\x12\x15a\n\xB9W`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xCFW`\0\x80\xFD[a\n\xDB\x8C\x82\x8D\x01a\x04\xF8V[\x90\x9AP\x98PP` \x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xFAW`\0\x80\xFD[a\x0B\x06\x8C\x82\x8D\x01a\x04\xF8V[\x90\x98P\x96PP`@\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B%W`\0\x80\xFD[a\x0B1\x8C\x82\x8D\x01a\x04\xF8V[\x90\x96P\x94PP``\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BPW`\0\x80\xFD[a\x0B\\\x8C\x82\x8D\x01a\x04\xF8V[\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x95\x98\x94\x97\x96`\x80\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x8B\x8D\x03\x12\x15a\x0B\x95W`\0\x80\xFD[\x8A5\x99P` \x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xB2W`\0\x80\xFD[a\x0B\xBE\x8D\x82\x8E\x01a\x04\xF8V[\x90\x9AP\x98PP`@\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xDDW`\0\x80\xFD[a\x0B\xE9\x8D\x82\x8E\x01a\x04\xF8V[\x90\x98P\x96Pa\x0B\xFC\x90P``\x8C\x01a\tDV[\x94P`\x80\x8B\x015\x93P`\xA0\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x1EW`\0\x80\xFD[a\x0C*\x8D\x82\x8E\x01a\x04\xF8V[\x9B\x9E\x9A\x9DP\x98\x9B\x97\x9A\x96\x99\x95\x98\x94\x97\x94\x96\x95`\xC0\x90\x95\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a\x0CeW`\0\x80\xFD[\x875\x96P` \x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x82W`\0\x80\xFD[a\x0C\x8E\x8A\x82\x8B\x01a\x04\xF8V[\x90\x97P\x95PP`@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xADW`\0\x80\xFD[a\x0C\xB9\x8A\x82\x8B\x01a\x04\xF8V[\x90\x95P\x93Pa\x0C\xCC\x90P``\x89\x01a\tDV[\x96\x99\x95\x98P\x93\x96\x92\x95\x91\x94\x91\x93PP`\x80\x90\x91\x015\x90V[\x83\x81R`\x80` \x82\x01R`\x01`\x80\x82\x01R`\x03`\xFC\x1B`\xA0\x82\x01R`\xC0`@\x82\x01R`\0`\xC0\x82\x01R`\xE0``\x82\x01R\x81`\xE0\x82\x01R\x81\x83a\x01\0\x83\x017`\0\x81\x83\x01a\x01\0\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\rVW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\rvWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\r\xC6W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\r\xA3WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\r\xC3W`\0\x81U`\x01\x01a\r\xAFV[PP[PPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15a\r\xE2Wa\r\xE2a\x08xV[a\r\xF6\x83a\r\xF0\x83Ta\rBV[\x83a\r|V[`\0`\x1F\x84\x11`\x01\x81\x14a\x0E*W`\0\x85\x15a\x0E\x12WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\r\xC3V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x0E[W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x0E;V[P\x86\x82\x10\x15a\x0ExW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV\xFE\xA2dipfsX\"\x12 )\xCB\x1B\xC1B\xED\xB2m\x98\x7F\x9F\nh\xBE\xB2\xF0\x1A\xA7\xDDJ\xA7Y\xCF\x0C\x0B\xB1\xE1\x0C\x06\xFE\r\xBAdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MOCKAXELARGATEWAY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA9W`\x005`\xE0\x1C\x80c\x93[\x13\xF6\x11a\0\xF9W\x80c\xBB\xFBR\xFB\x11a\0\x97W\x80c\xD2\xBC7\xF8\x11a\0qW\x80c\xD2\xBC7\xF8\x14a\x01\xC0W\x80c\xD3\x13|\x85\x14a\x03mW\x80c\xE3\xDF\xA2\x99\x14a\x01\xC0W\x80c\xF6\xA5\xF9\xF5\x14a\x03uW`\0\x80\xFD[\x80c\xBB\xFBR\xFB\x14a\x03.W\x80c\xBC\0\xC2\x16\x14a\x03AW\x80c\xD2o\xF2\x10\x14a\x03_W`\0\x80\xFD[\x80c\xAA\x1E\x1F\n\x11a\0\xD3W\x80c\xAA\x1E\x1F\n\x14a\x02\xE6W\x80c\xAD\xD0\xA5S\x14a\x02\xEDW\x80c\xB5+8\xE1\x14a\x03\0W\x80c\xB5Ap\x84\x14a\x03\x15W`\0\x80\xFD[\x80c\x93[\x13\xF6\x14a\x02\xC4W\x80c\x9D\xED\x06\xDF\x14a\x01\xAEW\x80c\xA3I\x9Cs\x14a\x02\xD2W`\0\x80\xFD[\x80c4\xFFi\x83\x11a\x01fW\x80c_ip\xC3\x11a\x01@W\x80c_ip\xC3\x14a\x02\x82W\x80cdl]4\x14a\x01\xAEW\x80c{\x1Bv\x9E\x14a\x02\x9CW\x80c\x88\xB3\x05\x87\x14a\x02\xB0W`\0\x80\xFD[\x80c4\xFFi\x83\x14a\x01\xAEW\x80c6I@\xD8\x14a\x02jW\x80c\\`\xDA\x1B\x14a\x02{W`\0\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\x01\xAEW\x80c\x10\xEFw\x95\x14a\x01\xC2W\x80c\x14\xBF\xD6\xD0\x14a\x01\xF2W\x80c\x18v\xEE\xD9\x14a\x02\x13W\x80c\x1C\x92\x11_\x14a\x02@W\x80c&\xEFi\x9D\x14a\x02SW[`\0\x80\xFD[a\x01\xC0a\x01\xBC6`\x04a\x05@V[PPV[\0[`\x01Ta\x01\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x06a\x02\x006`\x04a\x05\x81V[P``\x90V[`@Qa\x01\xE9\x91\x90a\x05\x9AV[a\x020a\x02!6`\x04a\x05\xE6V[`\x01\x99\x98PPPPPPPPPV[`@Q\x90\x15\x15\x81R` \x01a\x01\xE9V[a\x01\xC0a\x02N6`\x04a\x06\xA4V[a\x03\x90V[a\x01\xC0a\x02a6`\x04a\x07GV[PPPPPPPV[`\0[`@Q\x90\x81R` \x01a\x01\xE9V[`\0a\x01\xD5V[a\x020a\x02\x906`\x04a\x07\xF3V[`\x01\x96\x95PPPPPPV[a\x020a\x02\xAA6`\x04a\x08\x8EV[P`\x01\x90V[a\x02ma\x02\xBE6`\x04a\x05\x81V[P`\0\x90V[a\x01\xD5a\x02\xBE6`\x04a\x08\x8EV[a\x01\xC0a\x02\xE06`\x04a\t`V[PPPPV[`\x01a\x020V[a\x01\xC0a\x02\xFB6`\x04a\t\xB9V[a\x03\xFFV[a\x03\x08a\x04]V[`@Qa\x01\xE9\x91\x90a\nMV[a\x01\xC0a\x03#6`\x04a\n\x9BV[PPPPPPPPPV[`\x03Ta\x01\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x020a\x03O6`\x04a\x0BvV[`\x01\x9A\x99PPPPPPPPPPV[a\x020a\x02\xAA6`\x04a\x05\x81V[a\x03\x08a\x04\xEBV[a\x020a\x03\x836`\x04a\x0CJV[`\x01\x97\x96PPPPPPPV[`\x03T`@Qc\t\"\xC0\xCB`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cI\x16\x06X\x90a\x03\xC5\x90`\0\x90\x86\x90\x86\x90`\x04\x01a\x0C\xE4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xDFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xF3W=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\0a\x04\x0C\x86\x88\x83a\r\xCBV[P`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x90U`\x02a\x045\x83\x85\x83a\r\xCBV[P`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[`\x02\x80Ta\x04j\x90a\rBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x96\x90a\rBV[\x80\x15a\x04\xE3W\x80`\x1F\x10a\x04\xB8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xE3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xC6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x80Ta\x04j\x90a\rBV[`\0\x80\x83`\x1F\x84\x01\x12a\x05\nW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05!W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x059W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x05SW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05iW`\0\x80\xFD[a\x05u\x85\x82\x86\x01a\x04\xF8V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x05\x93W`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x05\xDBW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x05\xB4V[P\x90\x95\x94PPPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xC0\x8A\x8C\x03\x12\x15a\x06\x04W`\0\x80\xFD[\x895\x98P` \x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06!W`\0\x80\xFD[a\x06-\x8C\x82\x8D\x01a\x04\xF8V[\x90\x99P\x97PP`@\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06LW`\0\x80\xFD[a\x06X\x8C\x82\x8D\x01a\x04\xF8V[\x90\x97P\x95PP``\x8A\x015\x93P`\x80\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06~W`\0\x80\xFD[a\x06\x8A\x8C\x82\x8D\x01a\x04\xF8V[\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x95\x98\x94\x97\x96`\xA0\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a\x06\xBDW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xD3W`\0\x80\xFD[a\x06\xDF\x89\x82\x8A\x01a\x04\xF8V[\x90\x97P\x95PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xFEW`\0\x80\xFD[a\x07\n\x89\x82\x8A\x01a\x04\xF8V[\x90\x95P\x93PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07)W`\0\x80\xFD[a\x075\x89\x82\x8A\x01a\x04\xF8V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\x07bW`\0\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07xW`\0\x80\xFD[a\x07\x84\x8A\x82\x8B\x01a\x04\xF8V[\x90\x98P\x96PP` \x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xA3W`\0\x80\xFD[a\x07\xAF\x8A\x82\x8B\x01a\x04\xF8V[\x90\x96P\x94PP`@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xCEW`\0\x80\xFD[a\x07\xDA\x8A\x82\x8B\x01a\x04\xF8V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96``\x90\x95\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x08\x0CW`\0\x80\xFD[\x865\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08)W`\0\x80\xFD[a\x085\x89\x82\x8A\x01a\x04\xF8V[\x90\x96P\x94PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08TW`\0\x80\xFD[a\x08`\x89\x82\x8A\x01a\x04\xF8V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xA0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xB6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x08\xC7W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xE0Wa\x08\xE0a\x08xV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\t\x0EWa\t\x0Ea\x08xV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\t&W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t[W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\tvW`\0\x80\xFD[a\t\x7F\x85a\tDV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xA1W`\0\x80\xFD[a\t\xAD\x87\x82\x88\x01a\x04\xF8V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\t\xD2W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xE8W`\0\x80\xFD[a\t\xF4\x89\x82\x8A\x01a\x04\xF8V[\x90\x97P\x95Pa\n\x07\x90P` \x88\x01a\tDV[\x93P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\"W`\0\x80\xFD[a\n.\x89\x82\x8A\x01a\x04\xF8V[\x90\x94P\x92Pa\nA\x90P``\x88\x01a\tDV[\x90P\x92\x95P\x92\x95P\x92\x95V[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\n{W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\n^V[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xA0\x8A\x8C\x03\x12\x15a\n\xB9W`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xCFW`\0\x80\xFD[a\n\xDB\x8C\x82\x8D\x01a\x04\xF8V[\x90\x9AP\x98PP` \x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xFAW`\0\x80\xFD[a\x0B\x06\x8C\x82\x8D\x01a\x04\xF8V[\x90\x98P\x96PP`@\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B%W`\0\x80\xFD[a\x0B1\x8C\x82\x8D\x01a\x04\xF8V[\x90\x96P\x94PP``\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BPW`\0\x80\xFD[a\x0B\\\x8C\x82\x8D\x01a\x04\xF8V[\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x95\x98\x94\x97\x96`\x80\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x8B\x8D\x03\x12\x15a\x0B\x95W`\0\x80\xFD[\x8A5\x99P` \x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xB2W`\0\x80\xFD[a\x0B\xBE\x8D\x82\x8E\x01a\x04\xF8V[\x90\x9AP\x98PP`@\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xDDW`\0\x80\xFD[a\x0B\xE9\x8D\x82\x8E\x01a\x04\xF8V[\x90\x98P\x96Pa\x0B\xFC\x90P``\x8C\x01a\tDV[\x94P`\x80\x8B\x015\x93P`\xA0\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x1EW`\0\x80\xFD[a\x0C*\x8D\x82\x8E\x01a\x04\xF8V[\x9B\x9E\x9A\x9DP\x98\x9B\x97\x9A\x96\x99\x95\x98\x94\x97\x94\x96\x95`\xC0\x90\x95\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a\x0CeW`\0\x80\xFD[\x875\x96P` \x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x82W`\0\x80\xFD[a\x0C\x8E\x8A\x82\x8B\x01a\x04\xF8V[\x90\x97P\x95PP`@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xADW`\0\x80\xFD[a\x0C\xB9\x8A\x82\x8B\x01a\x04\xF8V[\x90\x95P\x93Pa\x0C\xCC\x90P``\x89\x01a\tDV[\x96\x99\x95\x98P\x93\x96\x92\x95\x91\x94\x91\x93PP`\x80\x90\x91\x015\x90V[\x83\x81R`\x80` \x82\x01R`\x01`\x80\x82\x01R`\x03`\xFC\x1B`\xA0\x82\x01R`\xC0`@\x82\x01R`\0`\xC0\x82\x01R`\xE0``\x82\x01R\x81`\xE0\x82\x01R\x81\x83a\x01\0\x83\x017`\0\x81\x83\x01a\x01\0\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\rVW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\rvWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\r\xC6W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\r\xA3WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\r\xC3W`\0\x81U`\x01\x01a\r\xAFV[PP[PPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15a\r\xE2Wa\r\xE2a\x08xV[a\r\xF6\x83a\r\xF0\x83Ta\rBV[\x83a\r|V[`\0`\x1F\x84\x11`\x01\x81\x14a\x0E*W`\0\x85\x15a\x0E\x12WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\r\xC3V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x0E[W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x0E;V[P\x86\x82\x10\x15a\x0ExW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV\xFE\xA2dipfsX\"\x12 )\xCB\x1B\xC1B\xED\xB2m\x98\x7F\x9F\nh\xBE\xB2\xF0\x1A\xA7\xDDJ\xA7Y\xCF\x0C\x0B\xB1\xE1\x0C\x06\xFE\r\xBAdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKAXELARGATEWAY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockAxelarGateway<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockAxelarGateway<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockAxelarGateway<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockAxelarGateway<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockAxelarGateway<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockAxelarGateway))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockAxelarGateway<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKAXELARGATEWAY_ABI.clone(),
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
                MOCKAXELARGATEWAY_ABI.clone(),
                MOCKAXELARGATEWAY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `adminEpoch` (0x364940d8) function
        pub fn admin_epoch(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([54, 73, 64, 216], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `adminThreshold` (0x88b30587) function
        pub fn admin_threshold(
            &self,
            epoch: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([136, 179, 5, 135], epoch)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admins` (0x14bfd6d0) function
        pub fn admins(
            &self,
            epoch: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<::ethers::core::types::Address>> {
            self.0
                .method_hash([20, 191, 214, 208], epoch)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allTokensFrozen` (0xaa1e1f0a) function
        pub fn all_tokens_frozen(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([170, 30, 31, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callContract` (0x1c92115f) function
        pub fn call_contract(
            &self,
            destination_chain: ::std::string::String,
            contract_address: ::std::string::String,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 146, 17, 95], (destination_chain, contract_address, payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callContractWithToken` (0xb5417084) function
        pub fn call_contract_with_token(
            &self,
            destination_chain: ::std::string::String,
            contract_address: ::std::string::String,
            payload: ::ethers::core::types::Bytes,
            symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [181, 65, 112, 132],
                    (destination_chain, contract_address, payload, symbol, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainNameA` (0xd3137c85) function
        pub fn chain_name_a(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([211, 19, 124, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainNameB` (0xb52b38e1) function
        pub fn chain_name_b(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([181, 43, 56, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contractAddressA` (0x10ef7795) function
        pub fn contract_address_a(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([16, 239, 119, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contractAddressB` (0xbbfb52fb) function
        pub fn contract_address_b(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([187, 251, 82, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x09c5eabe) function
        pub fn execute(
            &self,
            input: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 197, 234, 190], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `freezeAllTokens` (0xd2bc37f8) function
        pub fn freeze_all_tokens(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 188, 55, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `freezeToken` (0x646c5d34) function
        pub fn freeze_token(&self, symbol: ::std::string::String) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 108, 93, 52], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `implementation` (0x5c60da1b) function
        pub fn implementation(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCommandExecuted` (0xd26ff210) function
        pub fn is_command_executed(&self, command_id: [u8; 32]) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([210, 111, 242, 16], command_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isContractCallAndMintApproved` (0xbc00c216) function
        pub fn is_contract_call_and_mint_approved(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            contract_address: ::ethers::core::types::Address,
            payload_hash: [u8; 32],
            symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [188, 0, 194, 22],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        contract_address,
                        payload_hash,
                        symbol,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isContractCallApproved` (0xf6a5f9f5) function
        pub fn is_contract_call_approved(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            contract_address: ::ethers::core::types::Address,
            payload_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [246, 165, 249, 245],
                    (command_id, source_chain, source_address, contract_address, payload_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendToken` (0x26ef699d) function
        pub fn send_token(
            &self,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 239, 105, 157],
                    (destination_chain, destination_address, symbol, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setChainPair` (0xadd0a553) function
        pub fn set_chain_pair(
            &self,
            chain_name_a: ::std::string::String,
            contract_address_a: ::ethers::core::types::Address,
            chain_name_b: ::std::string::String,
            contract_address_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [173, 208, 165, 83],
                    (chain_name_a, contract_address_a, chain_name_b, contract_address_b),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setup` (0x9ded06df) function
        pub fn setup(&self, params: ::ethers::core::types::Bytes) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 237, 6, 223], params)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenAddresses` (0x935b13f6) function
        pub fn token_addresses(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([147, 91, 19, 246], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenFrozen` (0x7b1b769e) function
        pub fn token_frozen(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([123, 27, 118, 158], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unfreezeAllTokens` (0xe3dfa299) function
        pub fn unfreeze_all_tokens(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 223, 162, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unfreezeToken` (0x34ff6983) function
        pub fn unfreeze_token(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 255, 105, 131], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgrade` (0xa3499c73) function
        pub fn upgrade(
            &self,
            new_implementation: ::ethers::core::types::Address,
            new_implementation_code_hash: [u8; 32],
            setup_params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [163, 73, 156, 115],
                    (new_implementation, new_implementation_code_hash, setup_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateContractCall` (0x5f6970c3) function
        pub fn validate_contract_call(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            payload_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [95, 105, 112, 195],
                    (command_id, source_chain, source_address, payload_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateContractCallAndMint` (0x1876eed9) function
        pub fn validate_contract_call_and_mint(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            payload_hash: [u8; 32],
            symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [24, 118, 238, 217],
                    (command_id, source_chain, source_address, payload_hash, symbol, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AccountBlacklisted` event
        pub fn account_blacklisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountBlacklistedFilter> {
            self.0.event()
        }
        ///Gets the contract's `AccountWhitelisted` event
        pub fn account_whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountWhitelistedFilter> {
            self.0.event()
        }
        ///Gets the contract's `AllTokensFrozen` event
        pub fn all_tokens_frozen_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AllTokensFrozenFilter> {
            self.0.event()
        }
        ///Gets the contract's `AllTokensUnfrozen` event
        pub fn all_tokens_unfrozen_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AllTokensUnfrozenFilter> {
            self.0.event()
        }
        ///Gets the contract's `ContractCall` event
        pub fn contract_call_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ContractCallFilter> {
            self.0.event()
        }
        ///Gets the contract's `ContractCallApproved` event
        pub fn contract_call_approved_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ContractCallApprovedFilter> {
            self.0.event()
        }
        ///Gets the contract's `ContractCallApprovedWithMint` event
        pub fn contract_call_approved_with_mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ContractCallApprovedWithMintFilter> {
            self.0.event()
        }
        ///Gets the contract's `ContractCallWithToken` event
        pub fn contract_call_with_token_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ContractCallWithTokenFilter> {
            self.0.event()
        }
        ///Gets the contract's `Executed` event
        pub fn executed_filter(&self) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ExecutedFilter> {
            self.0.event()
        }
        ///Gets the contract's `TokenDeployed` event
        pub fn token_deployed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TokenDeployedFilter> {
            self.0.event()
        }
        ///Gets the contract's `TokenFrozen` event
        pub fn token_frozen_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TokenFrozenFilter> {
            self.0.event()
        }
        ///Gets the contract's `TokenSent` event
        pub fn token_sent_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TokenSentFilter> {
            self.0.event()
        }
        ///Gets the contract's `TokenUnfrozen` event
        pub fn token_unfrozen_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TokenUnfrozenFilter> {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(&self) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(&self) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MockAxelarGatewayEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockAxelarGateway<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "AccountBlacklisted", abi = "AccountBlacklisted(address)")]
    pub struct AccountBlacklistedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
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
    #[ethevent(name = "AccountWhitelisted", abi = "AccountWhitelisted(address)")]
    pub struct AccountWhitelistedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
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
    #[ethevent(name = "AllTokensFrozen", abi = "AllTokensFrozen()")]
    pub struct AllTokensFrozenFilter;
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
    #[ethevent(name = "AllTokensUnfrozen", abi = "AllTokensUnfrozen()")]
    pub struct AllTokensUnfrozenFilter;
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
    #[ethevent(name = "ContractCall", abi = "ContractCall(address,string,string,bytes32,bytes)")]
    pub struct ContractCallFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_contract_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub payload: ::ethers::core::types::Bytes,
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
        name = "ContractCallApproved",
        abi = "ContractCallApproved(bytes32,string,string,address,bytes32,bytes32,uint256)"
    )]
    pub struct ContractCallApprovedFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        #[ethevent(indexed)]
        pub contract_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub source_tx_hash: [u8; 32],
        pub source_event_index: ::ethers::core::types::U256,
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
        name = "ContractCallApprovedWithMint",
        abi = "ContractCallApprovedWithMint(bytes32,string,string,address,bytes32,string,uint256,bytes32,uint256)"
    )]
    pub struct ContractCallApprovedWithMintFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        #[ethevent(indexed)]
        pub contract_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
        pub source_tx_hash: [u8; 32],
        pub source_event_index: ::ethers::core::types::U256,
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
        name = "ContractCallWithToken",
        abi = "ContractCallWithToken(address,string,string,bytes32,bytes,string,uint256)"
    )]
    pub struct ContractCallWithTokenFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_contract_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub payload: ::ethers::core::types::Bytes,
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "Executed", abi = "Executed(bytes32)")]
    pub struct ExecutedFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
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
    #[ethevent(name = "TokenDeployed", abi = "TokenDeployed(string,address)")]
    pub struct TokenDeployedFilter {
        pub symbol: ::std::string::String,
        pub token_addresses: ::ethers::core::types::Address,
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
    #[ethevent(name = "TokenFrozen", abi = "TokenFrozen(string)")]
    pub struct TokenFrozenFilter {
        pub symbol: ::std::string::String,
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
    #[ethevent(name = "TokenSent", abi = "TokenSent(address,string,string,string,uint256)")]
    pub struct TokenSentFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "TokenUnfrozen", abi = "TokenUnfrozen(string)")]
    pub struct TokenUnfrozenFilter {
        pub symbol: ::std::string::String,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockAxelarGatewayEvents {
        AccountBlacklistedFilter(AccountBlacklistedFilter),
        AccountWhitelistedFilter(AccountWhitelistedFilter),
        AllTokensFrozenFilter(AllTokensFrozenFilter),
        AllTokensUnfrozenFilter(AllTokensUnfrozenFilter),
        ContractCallFilter(ContractCallFilter),
        ContractCallApprovedFilter(ContractCallApprovedFilter),
        ContractCallApprovedWithMintFilter(ContractCallApprovedWithMintFilter),
        ContractCallWithTokenFilter(ContractCallWithTokenFilter),
        ExecutedFilter(ExecutedFilter),
        TokenDeployedFilter(TokenDeployedFilter),
        TokenFrozenFilter(TokenFrozenFilter),
        TokenSentFilter(TokenSentFilter),
        TokenUnfrozenFilter(TokenUnfrozenFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockAxelarGatewayEvents {
        fn decode_log(log: &::ethers::core::abi::RawLog) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AccountBlacklistedFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::AccountBlacklistedFilter(decoded));
            }
            if let Ok(decoded) = AccountWhitelistedFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::AccountWhitelistedFilter(decoded));
            }
            if let Ok(decoded) = AllTokensFrozenFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::AllTokensFrozenFilter(decoded));
            }
            if let Ok(decoded) = AllTokensUnfrozenFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::AllTokensUnfrozenFilter(decoded));
            }
            if let Ok(decoded) = ContractCallFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::ContractCallFilter(decoded));
            }
            if let Ok(decoded) = ContractCallApprovedFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::ContractCallApprovedFilter(decoded));
            }
            if let Ok(decoded) = ContractCallApprovedWithMintFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::ContractCallApprovedWithMintFilter(decoded));
            }
            if let Ok(decoded) = ContractCallWithTokenFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::ContractCallWithTokenFilter(decoded));
            }
            if let Ok(decoded) = ExecutedFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::ExecutedFilter(decoded));
            }
            if let Ok(decoded) = TokenDeployedFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::TokenDeployedFilter(decoded));
            }
            if let Ok(decoded) = TokenFrozenFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::TokenFrozenFilter(decoded));
            }
            if let Ok(decoded) = TokenSentFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::TokenSentFilter(decoded));
            }
            if let Ok(decoded) = TokenUnfrozenFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::TokenUnfrozenFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(MockAxelarGatewayEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockAxelarGatewayEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountBlacklistedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountWhitelistedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTokensFrozenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTokensUnfrozenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractCallFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractCallApprovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractCallApprovedWithMintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractCallWithTokenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenDeployedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenFrozenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenSentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenUnfrozenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountBlacklistedFilter> for MockAxelarGatewayEvents {
        fn from(value: AccountBlacklistedFilter) -> Self {
            Self::AccountBlacklistedFilter(value)
        }
    }
    impl ::core::convert::From<AccountWhitelistedFilter> for MockAxelarGatewayEvents {
        fn from(value: AccountWhitelistedFilter) -> Self {
            Self::AccountWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<AllTokensFrozenFilter> for MockAxelarGatewayEvents {
        fn from(value: AllTokensFrozenFilter) -> Self {
            Self::AllTokensFrozenFilter(value)
        }
    }
    impl ::core::convert::From<AllTokensUnfrozenFilter> for MockAxelarGatewayEvents {
        fn from(value: AllTokensUnfrozenFilter) -> Self {
            Self::AllTokensUnfrozenFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallFilter> for MockAxelarGatewayEvents {
        fn from(value: ContractCallFilter) -> Self {
            Self::ContractCallFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallApprovedFilter> for MockAxelarGatewayEvents {
        fn from(value: ContractCallApprovedFilter) -> Self {
            Self::ContractCallApprovedFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallApprovedWithMintFilter> for MockAxelarGatewayEvents {
        fn from(value: ContractCallApprovedWithMintFilter) -> Self {
            Self::ContractCallApprovedWithMintFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallWithTokenFilter> for MockAxelarGatewayEvents {
        fn from(value: ContractCallWithTokenFilter) -> Self {
            Self::ContractCallWithTokenFilter(value)
        }
    }
    impl ::core::convert::From<ExecutedFilter> for MockAxelarGatewayEvents {
        fn from(value: ExecutedFilter) -> Self {
            Self::ExecutedFilter(value)
        }
    }
    impl ::core::convert::From<TokenDeployedFilter> for MockAxelarGatewayEvents {
        fn from(value: TokenDeployedFilter) -> Self {
            Self::TokenDeployedFilter(value)
        }
    }
    impl ::core::convert::From<TokenFrozenFilter> for MockAxelarGatewayEvents {
        fn from(value: TokenFrozenFilter) -> Self {
            Self::TokenFrozenFilter(value)
        }
    }
    impl ::core::convert::From<TokenSentFilter> for MockAxelarGatewayEvents {
        fn from(value: TokenSentFilter) -> Self {
            Self::TokenSentFilter(value)
        }
    }
    impl ::core::convert::From<TokenUnfrozenFilter> for MockAxelarGatewayEvents {
        fn from(value: TokenUnfrozenFilter) -> Self {
            Self::TokenUnfrozenFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for MockAxelarGatewayEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `adminEpoch` function with signature `adminEpoch()` and selector `0x364940d8`
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
    #[ethcall(name = "adminEpoch", abi = "adminEpoch()")]
    pub struct AdminEpochCall;
    ///Container type for all input parameters for the `adminThreshold` function with signature `adminThreshold(uint256)` and selector `0x88b30587`
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
    #[ethcall(name = "adminThreshold", abi = "adminThreshold(uint256)")]
    pub struct AdminThresholdCall {
        pub epoch: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `admins` function with signature `admins(uint256)` and selector `0x14bfd6d0`
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
    #[ethcall(name = "admins", abi = "admins(uint256)")]
    pub struct AdminsCall {
        pub epoch: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `allTokensFrozen` function with signature `allTokensFrozen()` and selector `0xaa1e1f0a`
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
    #[ethcall(name = "allTokensFrozen", abi = "allTokensFrozen()")]
    pub struct AllTokensFrozenCall;
    ///Container type for all input parameters for the `callContract` function with signature `callContract(string,string,bytes)` and selector `0x1c92115f`
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
    #[ethcall(name = "callContract", abi = "callContract(string,string,bytes)")]
    pub struct CallContractCall {
        pub destination_chain: ::std::string::String,
        pub contract_address: ::std::string::String,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `callContractWithToken` function with signature `callContractWithToken(string,string,bytes,string,uint256)` and selector `0xb5417084`
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
        name = "callContractWithToken",
        abi = "callContractWithToken(string,string,bytes,string,uint256)"
    )]
    pub struct CallContractWithTokenCall {
        pub destination_chain: ::std::string::String,
        pub contract_address: ::std::string::String,
        pub payload: ::ethers::core::types::Bytes,
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `chainNameA` function with signature `chainNameA()` and selector `0xd3137c85`
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
    #[ethcall(name = "chainNameA", abi = "chainNameA()")]
    pub struct ChainNameACall;
    ///Container type for all input parameters for the `chainNameB` function with signature `chainNameB()` and selector `0xb52b38e1`
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
    #[ethcall(name = "chainNameB", abi = "chainNameB()")]
    pub struct ChainNameBCall;
    ///Container type for all input parameters for the `contractAddressA` function with signature `contractAddressA()` and selector `0x10ef7795`
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
    #[ethcall(name = "contractAddressA", abi = "contractAddressA()")]
    pub struct ContractAddressACall;
    ///Container type for all input parameters for the `contractAddressB` function with signature `contractAddressB()` and selector `0xbbfb52fb`
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
    #[ethcall(name = "contractAddressB", abi = "contractAddressB()")]
    pub struct ContractAddressBCall;
    ///Container type for all input parameters for the `execute` function with signature `execute(bytes)` and selector `0x09c5eabe`
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
    #[ethcall(name = "execute", abi = "execute(bytes)")]
    pub struct ExecuteCall {
        pub input: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `freezeAllTokens` function with signature `freezeAllTokens()` and selector `0xd2bc37f8`
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
    #[ethcall(name = "freezeAllTokens", abi = "freezeAllTokens()")]
    pub struct FreezeAllTokensCall;
    ///Container type for all input parameters for the `freezeToken` function with signature `freezeToken(string)` and selector `0x646c5d34`
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
    #[ethcall(name = "freezeToken", abi = "freezeToken(string)")]
    pub struct FreezeTokenCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    ///Container type for all input parameters for the `isCommandExecuted` function with signature `isCommandExecuted(bytes32)` and selector `0xd26ff210`
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
    #[ethcall(name = "isCommandExecuted", abi = "isCommandExecuted(bytes32)")]
    pub struct IsCommandExecutedCall {
        pub command_id: [u8; 32],
    }
    ///Container type for all input parameters for the `isContractCallAndMintApproved` function with signature `isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)` and selector `0xbc00c216`
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
        name = "isContractCallAndMintApproved",
        abi = "isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)"
    )]
    pub struct IsContractCallAndMintApprovedCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub contract_address: ::ethers::core::types::Address,
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isContractCallApproved` function with signature `isContractCallApproved(bytes32,string,string,address,bytes32)` and selector `0xf6a5f9f5`
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
        name = "isContractCallApproved",
        abi = "isContractCallApproved(bytes32,string,string,address,bytes32)"
    )]
    pub struct IsContractCallApprovedCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub contract_address: ::ethers::core::types::Address,
        pub payload_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `sendToken` function with signature `sendToken(string,string,string,uint256)` and selector `0x26ef699d`
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
    #[ethcall(name = "sendToken", abi = "sendToken(string,string,string,uint256)")]
    pub struct SendTokenCall {
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setChainPair` function with signature `setChainPair(string,address,string,address)` and selector `0xadd0a553`
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
    #[ethcall(name = "setChainPair", abi = "setChainPair(string,address,string,address)")]
    pub struct SetChainPairCall {
        pub chain_name_a: ::std::string::String,
        pub contract_address_a: ::ethers::core::types::Address,
        pub chain_name_b: ::std::string::String,
        pub contract_address_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setup` function with signature `setup(bytes)` and selector `0x9ded06df`
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
    #[ethcall(name = "setup", abi = "setup(bytes)")]
    pub struct SetupCall {
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `tokenAddresses` function with signature `tokenAddresses(string)` and selector `0x935b13f6`
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
    #[ethcall(name = "tokenAddresses", abi = "tokenAddresses(string)")]
    pub struct TokenAddressesCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `tokenFrozen` function with signature `tokenFrozen(string)` and selector `0x7b1b769e`
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
    #[ethcall(name = "tokenFrozen", abi = "tokenFrozen(string)")]
    pub struct TokenFrozenCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `unfreezeAllTokens` function with signature `unfreezeAllTokens()` and selector `0xe3dfa299`
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
    #[ethcall(name = "unfreezeAllTokens", abi = "unfreezeAllTokens()")]
    pub struct UnfreezeAllTokensCall;
    ///Container type for all input parameters for the `unfreezeToken` function with signature `unfreezeToken(string)` and selector `0x34ff6983`
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
    #[ethcall(name = "unfreezeToken", abi = "unfreezeToken(string)")]
    pub struct UnfreezeTokenCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `upgrade` function with signature `upgrade(address,bytes32,bytes)` and selector `0xa3499c73`
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
    #[ethcall(name = "upgrade", abi = "upgrade(address,bytes32,bytes)")]
    pub struct UpgradeCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub new_implementation_code_hash: [u8; 32],
        pub setup_params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateContractCall` function with signature `validateContractCall(bytes32,string,string,bytes32)` and selector `0x5f6970c3`
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
        name = "validateContractCall",
        abi = "validateContractCall(bytes32,string,string,bytes32)"
    )]
    pub struct ValidateContractCallCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub payload_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `validateContractCallAndMint` function with signature `validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)` and selector `0x1876eed9`
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
        name = "validateContractCallAndMint",
        abi = "validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)"
    )]
    pub struct ValidateContractCallAndMintCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockAxelarGatewayCalls {
        AdminEpoch(AdminEpochCall),
        AdminThreshold(AdminThresholdCall),
        Admins(AdminsCall),
        AllTokensFrozen(AllTokensFrozenCall),
        CallContract(CallContractCall),
        CallContractWithToken(CallContractWithTokenCall),
        ChainNameA(ChainNameACall),
        ChainNameB(ChainNameBCall),
        ContractAddressA(ContractAddressACall),
        ContractAddressB(ContractAddressBCall),
        Execute(ExecuteCall),
        FreezeAllTokens(FreezeAllTokensCall),
        FreezeToken(FreezeTokenCall),
        Implementation(ImplementationCall),
        IsCommandExecuted(IsCommandExecutedCall),
        IsContractCallAndMintApproved(IsContractCallAndMintApprovedCall),
        IsContractCallApproved(IsContractCallApprovedCall),
        SendToken(SendTokenCall),
        SetChainPair(SetChainPairCall),
        Setup(SetupCall),
        TokenAddresses(TokenAddressesCall),
        TokenFrozen(TokenFrozenCall),
        UnfreezeAllTokens(UnfreezeAllTokensCall),
        UnfreezeToken(UnfreezeTokenCall),
        Upgrade(UpgradeCall),
        ValidateContractCall(ValidateContractCallCall),
        ValidateContractCallAndMint(ValidateContractCallAndMintCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockAxelarGatewayCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdminEpochCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AdminEpoch(decoded));
            }
            if let Ok(decoded) = <AdminThresholdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AdminThreshold(decoded));
            }
            if let Ok(decoded) = <AdminsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admins(decoded));
            }
            if let Ok(decoded) = <AllTokensFrozenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AllTokensFrozen(decoded));
            }
            if let Ok(decoded) = <CallContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallContract(decoded));
            }
            if let Ok(decoded) = <CallContractWithTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallContractWithToken(decoded));
            }
            if let Ok(decoded) = <ChainNameACall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChainNameA(decoded));
            }
            if let Ok(decoded) = <ChainNameBCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChainNameB(decoded));
            }
            if let Ok(decoded) = <ContractAddressACall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ContractAddressA(decoded));
            }
            if let Ok(decoded) = <ContractAddressBCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ContractAddressB(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <FreezeAllTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FreezeAllTokens(decoded));
            }
            if let Ok(decoded) = <FreezeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FreezeToken(decoded));
            }
            if let Ok(decoded) = <ImplementationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Implementation(decoded));
            }
            if let Ok(decoded) = <IsCommandExecutedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsCommandExecuted(decoded));
            }
            if let Ok(decoded) = <IsContractCallAndMintApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsContractCallAndMintApproved(decoded));
            }
            if let Ok(decoded) = <IsContractCallApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsContractCallApproved(decoded));
            }
            if let Ok(decoded) = <SendTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendToken(decoded));
            }
            if let Ok(decoded) = <SetChainPairCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetChainPair(decoded));
            }
            if let Ok(decoded) = <SetupCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Setup(decoded));
            }
            if let Ok(decoded) = <TokenAddressesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenAddresses(decoded));
            }
            if let Ok(decoded) = <TokenFrozenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenFrozen(decoded));
            }
            if let Ok(decoded) = <UnfreezeAllTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnfreezeAllTokens(decoded));
            }
            if let Ok(decoded) = <UnfreezeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnfreezeToken(decoded));
            }
            if let Ok(decoded) = <UpgradeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Upgrade(decoded));
            }
            if let Ok(decoded) = <ValidateContractCallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ValidateContractCall(decoded));
            }
            if let Ok(decoded) = <ValidateContractCallAndMintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ValidateContractCallAndMint(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockAxelarGatewayCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AdminThreshold(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Admins(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllTokensFrozen(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CallContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CallContractWithToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChainNameA(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChainNameB(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ContractAddressA(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ContractAddressB(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FreezeAllTokens(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FreezeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Implementation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsCommandExecuted(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsContractCallAndMintApproved(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsContractCallApproved(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SendToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetChainPair(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Setup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenAddresses(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenFrozen(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnfreezeAllTokens(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnfreezeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Upgrade(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateContractCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateContractCallAndMint(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockAxelarGatewayCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admins(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTokensFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallContractWithToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainNameA(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainNameB(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractAddressA(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractAddressB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreezeAllTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreezeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Implementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsCommandExecuted(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsContractCallAndMintApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsContractCallApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetChainPair(element) => ::core::fmt::Display::fmt(element, f),
                Self::Setup(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnfreezeAllTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnfreezeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Upgrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateContractCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateContractCallAndMint(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminEpochCall> for MockAxelarGatewayCalls {
        fn from(value: AdminEpochCall) -> Self {
            Self::AdminEpoch(value)
        }
    }
    impl ::core::convert::From<AdminThresholdCall> for MockAxelarGatewayCalls {
        fn from(value: AdminThresholdCall) -> Self {
            Self::AdminThreshold(value)
        }
    }
    impl ::core::convert::From<AdminsCall> for MockAxelarGatewayCalls {
        fn from(value: AdminsCall) -> Self {
            Self::Admins(value)
        }
    }
    impl ::core::convert::From<AllTokensFrozenCall> for MockAxelarGatewayCalls {
        fn from(value: AllTokensFrozenCall) -> Self {
            Self::AllTokensFrozen(value)
        }
    }
    impl ::core::convert::From<CallContractCall> for MockAxelarGatewayCalls {
        fn from(value: CallContractCall) -> Self {
            Self::CallContract(value)
        }
    }
    impl ::core::convert::From<CallContractWithTokenCall> for MockAxelarGatewayCalls {
        fn from(value: CallContractWithTokenCall) -> Self {
            Self::CallContractWithToken(value)
        }
    }
    impl ::core::convert::From<ChainNameACall> for MockAxelarGatewayCalls {
        fn from(value: ChainNameACall) -> Self {
            Self::ChainNameA(value)
        }
    }
    impl ::core::convert::From<ChainNameBCall> for MockAxelarGatewayCalls {
        fn from(value: ChainNameBCall) -> Self {
            Self::ChainNameB(value)
        }
    }
    impl ::core::convert::From<ContractAddressACall> for MockAxelarGatewayCalls {
        fn from(value: ContractAddressACall) -> Self {
            Self::ContractAddressA(value)
        }
    }
    impl ::core::convert::From<ContractAddressBCall> for MockAxelarGatewayCalls {
        fn from(value: ContractAddressBCall) -> Self {
            Self::ContractAddressB(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for MockAxelarGatewayCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<FreezeAllTokensCall> for MockAxelarGatewayCalls {
        fn from(value: FreezeAllTokensCall) -> Self {
            Self::FreezeAllTokens(value)
        }
    }
    impl ::core::convert::From<FreezeTokenCall> for MockAxelarGatewayCalls {
        fn from(value: FreezeTokenCall) -> Self {
            Self::FreezeToken(value)
        }
    }
    impl ::core::convert::From<ImplementationCall> for MockAxelarGatewayCalls {
        fn from(value: ImplementationCall) -> Self {
            Self::Implementation(value)
        }
    }
    impl ::core::convert::From<IsCommandExecutedCall> for MockAxelarGatewayCalls {
        fn from(value: IsCommandExecutedCall) -> Self {
            Self::IsCommandExecuted(value)
        }
    }
    impl ::core::convert::From<IsContractCallAndMintApprovedCall> for MockAxelarGatewayCalls {
        fn from(value: IsContractCallAndMintApprovedCall) -> Self {
            Self::IsContractCallAndMintApproved(value)
        }
    }
    impl ::core::convert::From<IsContractCallApprovedCall> for MockAxelarGatewayCalls {
        fn from(value: IsContractCallApprovedCall) -> Self {
            Self::IsContractCallApproved(value)
        }
    }
    impl ::core::convert::From<SendTokenCall> for MockAxelarGatewayCalls {
        fn from(value: SendTokenCall) -> Self {
            Self::SendToken(value)
        }
    }
    impl ::core::convert::From<SetChainPairCall> for MockAxelarGatewayCalls {
        fn from(value: SetChainPairCall) -> Self {
            Self::SetChainPair(value)
        }
    }
    impl ::core::convert::From<SetupCall> for MockAxelarGatewayCalls {
        fn from(value: SetupCall) -> Self {
            Self::Setup(value)
        }
    }
    impl ::core::convert::From<TokenAddressesCall> for MockAxelarGatewayCalls {
        fn from(value: TokenAddressesCall) -> Self {
            Self::TokenAddresses(value)
        }
    }
    impl ::core::convert::From<TokenFrozenCall> for MockAxelarGatewayCalls {
        fn from(value: TokenFrozenCall) -> Self {
            Self::TokenFrozen(value)
        }
    }
    impl ::core::convert::From<UnfreezeAllTokensCall> for MockAxelarGatewayCalls {
        fn from(value: UnfreezeAllTokensCall) -> Self {
            Self::UnfreezeAllTokens(value)
        }
    }
    impl ::core::convert::From<UnfreezeTokenCall> for MockAxelarGatewayCalls {
        fn from(value: UnfreezeTokenCall) -> Self {
            Self::UnfreezeToken(value)
        }
    }
    impl ::core::convert::From<UpgradeCall> for MockAxelarGatewayCalls {
        fn from(value: UpgradeCall) -> Self {
            Self::Upgrade(value)
        }
    }
    impl ::core::convert::From<ValidateContractCallCall> for MockAxelarGatewayCalls {
        fn from(value: ValidateContractCallCall) -> Self {
            Self::ValidateContractCall(value)
        }
    }
    impl ::core::convert::From<ValidateContractCallAndMintCall> for MockAxelarGatewayCalls {
        fn from(value: ValidateContractCallAndMintCall) -> Self {
            Self::ValidateContractCallAndMint(value)
        }
    }
    ///Container type for all return fields from the `adminEpoch` function with signature `adminEpoch()` and selector `0x364940d8`
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
    pub struct AdminEpochReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `adminThreshold` function with signature `adminThreshold(uint256)` and selector `0x88b30587`
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
    pub struct AdminThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `admins` function with signature `admins(uint256)` and selector `0x14bfd6d0`
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
    pub struct AdminsReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `allTokensFrozen` function with signature `allTokensFrozen()` and selector `0xaa1e1f0a`
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
    pub struct AllTokensFrozenReturn(pub bool);
    ///Container type for all return fields from the `chainNameA` function with signature `chainNameA()` and selector `0xd3137c85`
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
    pub struct ChainNameAReturn(pub ::std::string::String);
    ///Container type for all return fields from the `chainNameB` function with signature `chainNameB()` and selector `0xb52b38e1`
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
    pub struct ChainNameBReturn(pub ::std::string::String);
    ///Container type for all return fields from the `contractAddressA` function with signature `contractAddressA()` and selector `0x10ef7795`
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
    pub struct ContractAddressAReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `contractAddressB` function with signature `contractAddressB()` and selector `0xbbfb52fb`
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
    pub struct ContractAddressBReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    pub struct ImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isCommandExecuted` function with signature `isCommandExecuted(bytes32)` and selector `0xd26ff210`
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
    pub struct IsCommandExecutedReturn(pub bool);
    ///Container type for all return fields from the `isContractCallAndMintApproved` function with signature `isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)` and selector `0xbc00c216`
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
    pub struct IsContractCallAndMintApprovedReturn(pub bool);
    ///Container type for all return fields from the `isContractCallApproved` function with signature `isContractCallApproved(bytes32,string,string,address,bytes32)` and selector `0xf6a5f9f5`
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
    pub struct IsContractCallApprovedReturn(pub bool);
    ///Container type for all return fields from the `tokenAddresses` function with signature `tokenAddresses(string)` and selector `0x935b13f6`
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
    pub struct TokenAddressesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenFrozen` function with signature `tokenFrozen(string)` and selector `0x7b1b769e`
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
    pub struct TokenFrozenReturn(pub bool);
    ///Container type for all return fields from the `validateContractCall` function with signature `validateContractCall(bytes32,string,string,bytes32)` and selector `0x5f6970c3`
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
    pub struct ValidateContractCallReturn(pub bool);
    ///Container type for all return fields from the `validateContractCallAndMint` function with signature `validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)` and selector `0x1876eed9`
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
    pub struct ValidateContractCallAndMintReturn(pub bool);
}
