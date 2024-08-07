pub use i_axelar_gateway::*;
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
pub mod i_axelar_gateway {
    const _: () = {
        ::core::include_bytes!(
"../json/IAxelarGateway.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("adminEpoch"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("adminEpoch"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("adminThreshold"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("adminThreshold"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epoch"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("admins"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admins"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epoch"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allTokensFrozen"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allTokensFrozen"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("callContract"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("callContract"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("callContractWithToken"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "callContractWithToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("freezeAllTokens"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("freezeAllTokens"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("freezeToken"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("freezeToken"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("implementation"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isCommandExecuted"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isCommandExecuted"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isContractCallAndMintApproved"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isContractCallAndMintApproved",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isContractCallApproved"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isContractCallApproved",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendToken"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendToken"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationAddress",
                                    ),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setup"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setup"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenAddresses"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenAddresses"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenFrozen"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenFrozen"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unfreezeAllTokens"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unfreezeAllTokens"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unfreezeToken"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unfreezeToken"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgrade"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgrade"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newImplementationCodeHash",
                                    ),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setupParams"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateContractCall"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validateContractCall",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateContractCallAndMint"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validateContractCallAndMint",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccountBlacklisted"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AccountBlacklisted"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccountWhitelisted"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AccountWhitelisted"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllTokensFrozen"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AllTokensFrozen"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllTokensUnfrozen"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AllTokensUnfrozen"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ContractCall"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ContractCall"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationContractAddress",
                                    ),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ContractCallApproved"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ContractCallApproved",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceTxHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceEventIndex"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ContractCallApprovedWithMint"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ContractCallApprovedWithMint",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceTxHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceEventIndex"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ContractCallWithToken"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ContractCallWithToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationContractAddress",
                                    ),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Executed"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Executed"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenDeployed"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenDeployed"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddresses"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenFrozen"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenFrozen"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenSent"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenSent"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationAddress",
                                    ),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenUnfrozen"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenUnfrozen"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IAXELARGATEWAY_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(
        __abi,
    );
    pub struct IAxelarGateway<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAxelarGateway<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAxelarGateway<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IAxelarGateway<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IAxelarGateway<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IAxelarGateway))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IAxelarGateway<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IAXELARGATEWAY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `adminEpoch` (0x364940d8) function
        pub fn admin_epoch(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([54, 73, 64, 216], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `adminThreshold` (0x88b30587) function
        pub fn admin_threshold(
            &self,
            epoch: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([136, 179, 5, 135], epoch)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admins` (0x14bfd6d0) function
        pub fn admins(
            &self,
            epoch: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers_core::types::Address>,
        > {
            self.0
                .method_hash([20, 191, 214, 208], epoch)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allTokensFrozen` (0xaa1e1f0a) function
        pub fn all_tokens_frozen(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([170, 30, 31, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callContract` (0x1c92115f) function
        pub fn call_contract(
            &self,
            destination_chain: ::std::string::String,
            contract_address: ::std::string::String,
            payload: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [28, 146, 17, 95],
                    (destination_chain, contract_address, payload),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callContractWithToken` (0xb5417084) function
        pub fn call_contract_with_token(
            &self,
            destination_chain: ::std::string::String,
            contract_address: ::std::string::String,
            payload: ::ethers_core::types::Bytes,
            symbol: ::std::string::String,
            amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [181, 65, 112, 132],
                    (destination_chain, contract_address, payload, symbol, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x09c5eabe) function
        pub fn execute(
            &self,
            input: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 197, 234, 190], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `freezeAllTokens` (0xd2bc37f8) function
        pub fn freeze_all_tokens(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 188, 55, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `freezeToken` (0x646c5d34) function
        pub fn freeze_token(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 108, 93, 52], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `implementation` (0x5c60da1b) function
        pub fn implementation(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCommandExecuted` (0xd26ff210) function
        pub fn is_command_executed(
            &self,
            command_id: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
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
            contract_address: ::ethers_core::types::Address,
            payload_hash: [u8; 32],
            symbol: ::std::string::String,
            amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
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
            contract_address: ::ethers_core::types::Address,
            payload_hash: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [246, 165, 249, 245],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        contract_address,
                        payload_hash,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendToken` (0x26ef699d) function
        pub fn send_token(
            &self,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            symbol: ::std::string::String,
            amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 239, 105, 157],
                    (destination_chain, destination_address, symbol, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setup` (0x9ded06df) function
        pub fn setup(
            &self,
            params: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 237, 6, 223], params)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenAddresses` (0x935b13f6) function
        pub fn token_addresses(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([147, 91, 19, 246], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenFrozen` (0x7b1b769e) function
        pub fn token_frozen(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([123, 27, 118, 158], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unfreezeAllTokens` (0xe3dfa299) function
        pub fn unfreeze_all_tokens(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 223, 162, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unfreezeToken` (0x34ff6983) function
        pub fn unfreeze_token(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 255, 105, 131], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgrade` (0xa3499c73) function
        pub fn upgrade(
            &self,
            new_implementation: ::ethers_core::types::Address,
            new_implementation_code_hash: [u8; 32],
            setup_params: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
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
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
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
            amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [24, 118, 238, 217],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        payload_hash,
                        symbol,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AccountBlacklisted` event
        pub fn account_blacklisted_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountBlacklistedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountWhitelisted` event
        pub fn account_whitelisted_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountWhitelistedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllTokensFrozen` event
        pub fn all_tokens_frozen_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllTokensFrozenFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllTokensUnfrozen` event
        pub fn all_tokens_unfrozen_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllTokensUnfrozenFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ContractCall` event
        pub fn contract_call_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractCallFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ContractCallApproved` event
        pub fn contract_call_approved_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractCallApprovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ContractCallApprovedWithMint` event
        pub fn contract_call_approved_with_mint_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractCallApprovedWithMintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ContractCallWithToken` event
        pub fn contract_call_with_token_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractCallWithTokenFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Executed` event
        pub fn executed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenDeployed` event
        pub fn token_deployed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenDeployedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenFrozen` event
        pub fn token_frozen_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenFrozenFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenSent` event
        pub fn token_sent_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenSentFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenUnfrozen` event
        pub fn token_unfrozen_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenUnfrozenFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IAxelarGatewayEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IAxelarGateway<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "AccountBlacklisted", abi = "AccountBlacklisted(address)")]
    pub struct AccountBlacklistedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers_core::types::Address,
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
    #[ethevent(name = "AccountWhitelisted", abi = "AccountWhitelisted(address)")]
    pub struct AccountWhitelistedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers_core::types::Address,
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
    #[ethevent(name = "AllTokensFrozen", abi = "AllTokensFrozen()")]
    pub struct AllTokensFrozenFilter;
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
    #[ethevent(name = "AllTokensUnfrozen", abi = "AllTokensUnfrozen()")]
    pub struct AllTokensUnfrozenFilter;
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
        name = "ContractCall",
        abi = "ContractCall(address,string,string,bytes32,bytes)"
    )]
    pub struct ContractCallFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_contract_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub payload: ::ethers_core::types::Bytes,
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
        name = "ContractCallApproved",
        abi = "ContractCallApproved(bytes32,string,string,address,bytes32,bytes32,uint256)"
    )]
    pub struct ContractCallApprovedFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        #[ethevent(indexed)]
        pub contract_address: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub source_tx_hash: [u8; 32],
        pub source_event_index: ::ethers_core::types::U256,
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
        name = "ContractCallApprovedWithMint",
        abi = "ContractCallApprovedWithMint(bytes32,string,string,address,bytes32,string,uint256,bytes32,uint256)"
    )]
    pub struct ContractCallApprovedWithMintFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        #[ethevent(indexed)]
        pub contract_address: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
        pub source_tx_hash: [u8; 32],
        pub source_event_index: ::ethers_core::types::U256,
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
        name = "ContractCallWithToken",
        abi = "ContractCallWithToken(address,string,string,bytes32,bytes,string,uint256)"
    )]
    pub struct ContractCallWithTokenFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_contract_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub payload: ::ethers_core::types::Bytes,
        pub symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
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
    #[ethevent(name = "Executed", abi = "Executed(bytes32)")]
    pub struct ExecutedFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
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
    #[ethevent(name = "TokenDeployed", abi = "TokenDeployed(string,address)")]
    pub struct TokenDeployedFilter {
        pub symbol: ::std::string::String,
        pub token_addresses: ::ethers_core::types::Address,
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
    #[ethevent(name = "TokenFrozen", abi = "TokenFrozen(string)")]
    pub struct TokenFrozenFilter {
        pub symbol: ::std::string::String,
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
        name = "TokenSent",
        abi = "TokenSent(address,string,string,string,uint256)"
    )]
    pub struct TokenSentFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers_core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
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
    #[ethevent(name = "TokenUnfrozen", abi = "TokenUnfrozen(string)")]
    pub struct TokenUnfrozenFilter {
        pub symbol: ::std::string::String,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers_core::types::Address,
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
    pub enum IAxelarGatewayEvents {
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
    impl ::ethers_contract::EthLogDecode for IAxelarGatewayEvents {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = AccountBlacklistedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::AccountBlacklistedFilter(decoded));
            }
            if let Ok(decoded) = AccountWhitelistedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::AccountWhitelistedFilter(decoded));
            }
            if let Ok(decoded) = AllTokensFrozenFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::AllTokensFrozenFilter(decoded));
            }
            if let Ok(decoded) = AllTokensUnfrozenFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::AllTokensUnfrozenFilter(decoded));
            }
            if let Ok(decoded) = ContractCallFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::ContractCallFilter(decoded));
            }
            if let Ok(decoded) = ContractCallApprovedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::ContractCallApprovedFilter(decoded));
            }
            if let Ok(decoded) = ContractCallApprovedWithMintFilter::decode_log(log) {
                return Ok(
                    IAxelarGatewayEvents::ContractCallApprovedWithMintFilter(decoded),
                );
            }
            if let Ok(decoded) = ContractCallWithTokenFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::ContractCallWithTokenFilter(decoded));
            }
            if let Ok(decoded) = ExecutedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::ExecutedFilter(decoded));
            }
            if let Ok(decoded) = TokenDeployedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::TokenDeployedFilter(decoded));
            }
            if let Ok(decoded) = TokenFrozenFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::TokenFrozenFilter(decoded));
            }
            if let Ok(decoded) = TokenSentFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::TokenSentFilter(decoded));
            }
            if let Ok(decoded) = TokenUnfrozenFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::TokenUnfrozenFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::UpgradedFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IAxelarGatewayEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountBlacklistedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountWhitelistedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllTokensFrozenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllTokensUnfrozenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractCallFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractCallApprovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractCallApprovedWithMintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractCallWithTokenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenDeployedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenFrozenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenSentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenUnfrozenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountBlacklistedFilter> for IAxelarGatewayEvents {
        fn from(value: AccountBlacklistedFilter) -> Self {
            Self::AccountBlacklistedFilter(value)
        }
    }
    impl ::core::convert::From<AccountWhitelistedFilter> for IAxelarGatewayEvents {
        fn from(value: AccountWhitelistedFilter) -> Self {
            Self::AccountWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<AllTokensFrozenFilter> for IAxelarGatewayEvents {
        fn from(value: AllTokensFrozenFilter) -> Self {
            Self::AllTokensFrozenFilter(value)
        }
    }
    impl ::core::convert::From<AllTokensUnfrozenFilter> for IAxelarGatewayEvents {
        fn from(value: AllTokensUnfrozenFilter) -> Self {
            Self::AllTokensUnfrozenFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallFilter> for IAxelarGatewayEvents {
        fn from(value: ContractCallFilter) -> Self {
            Self::ContractCallFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallApprovedFilter> for IAxelarGatewayEvents {
        fn from(value: ContractCallApprovedFilter) -> Self {
            Self::ContractCallApprovedFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallApprovedWithMintFilter>
    for IAxelarGatewayEvents {
        fn from(value: ContractCallApprovedWithMintFilter) -> Self {
            Self::ContractCallApprovedWithMintFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallWithTokenFilter> for IAxelarGatewayEvents {
        fn from(value: ContractCallWithTokenFilter) -> Self {
            Self::ContractCallWithTokenFilter(value)
        }
    }
    impl ::core::convert::From<ExecutedFilter> for IAxelarGatewayEvents {
        fn from(value: ExecutedFilter) -> Self {
            Self::ExecutedFilter(value)
        }
    }
    impl ::core::convert::From<TokenDeployedFilter> for IAxelarGatewayEvents {
        fn from(value: TokenDeployedFilter) -> Self {
            Self::TokenDeployedFilter(value)
        }
    }
    impl ::core::convert::From<TokenFrozenFilter> for IAxelarGatewayEvents {
        fn from(value: TokenFrozenFilter) -> Self {
            Self::TokenFrozenFilter(value)
        }
    }
    impl ::core::convert::From<TokenSentFilter> for IAxelarGatewayEvents {
        fn from(value: TokenSentFilter) -> Self {
            Self::TokenSentFilter(value)
        }
    }
    impl ::core::convert::From<TokenUnfrozenFilter> for IAxelarGatewayEvents {
        fn from(value: TokenUnfrozenFilter) -> Self {
            Self::TokenUnfrozenFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for IAxelarGatewayEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `adminEpoch` function with signature `adminEpoch()` and selector `0x364940d8`
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
    #[ethcall(name = "adminEpoch", abi = "adminEpoch()")]
    pub struct AdminEpochCall;
    ///Container type for all input parameters for the `adminThreshold` function with signature `adminThreshold(uint256)` and selector `0x88b30587`
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
    #[ethcall(name = "adminThreshold", abi = "adminThreshold(uint256)")]
    pub struct AdminThresholdCall {
        pub epoch: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `admins` function with signature `admins(uint256)` and selector `0x14bfd6d0`
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
    #[ethcall(name = "admins", abi = "admins(uint256)")]
    pub struct AdminsCall {
        pub epoch: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `allTokensFrozen` function with signature `allTokensFrozen()` and selector `0xaa1e1f0a`
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
    #[ethcall(name = "allTokensFrozen", abi = "allTokensFrozen()")]
    pub struct AllTokensFrozenCall;
    ///Container type for all input parameters for the `callContract` function with signature `callContract(string,string,bytes)` and selector `0x1c92115f`
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
    #[ethcall(name = "callContract", abi = "callContract(string,string,bytes)")]
    pub struct CallContractCall {
        pub destination_chain: ::std::string::String,
        pub contract_address: ::std::string::String,
        pub payload: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `callContractWithToken` function with signature `callContractWithToken(string,string,bytes,string,uint256)` and selector `0xb5417084`
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
        name = "callContractWithToken",
        abi = "callContractWithToken(string,string,bytes,string,uint256)"
    )]
    pub struct CallContractWithTokenCall {
        pub destination_chain: ::std::string::String,
        pub contract_address: ::std::string::String,
        pub payload: ::ethers_core::types::Bytes,
        pub symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(bytes)` and selector `0x09c5eabe`
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
    #[ethcall(name = "execute", abi = "execute(bytes)")]
    pub struct ExecuteCall {
        pub input: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `freezeAllTokens` function with signature `freezeAllTokens()` and selector `0xd2bc37f8`
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
    #[ethcall(name = "freezeAllTokens", abi = "freezeAllTokens()")]
    pub struct FreezeAllTokensCall;
    ///Container type for all input parameters for the `freezeToken` function with signature `freezeToken(string)` and selector `0x646c5d34`
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
    #[ethcall(name = "freezeToken", abi = "freezeToken(string)")]
    pub struct FreezeTokenCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    ///Container type for all input parameters for the `isCommandExecuted` function with signature `isCommandExecuted(bytes32)` and selector `0xd26ff210`
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
    #[ethcall(name = "isCommandExecuted", abi = "isCommandExecuted(bytes32)")]
    pub struct IsCommandExecutedCall {
        pub command_id: [u8; 32],
    }
    ///Container type for all input parameters for the `isContractCallAndMintApproved` function with signature `isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)` and selector `0xbc00c216`
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
        name = "isContractCallAndMintApproved",
        abi = "isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)"
    )]
    pub struct IsContractCallAndMintApprovedCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub contract_address: ::ethers_core::types::Address,
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `isContractCallApproved` function with signature `isContractCallApproved(bytes32,string,string,address,bytes32)` and selector `0xf6a5f9f5`
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
        name = "isContractCallApproved",
        abi = "isContractCallApproved(bytes32,string,string,address,bytes32)"
    )]
    pub struct IsContractCallApprovedCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub contract_address: ::ethers_core::types::Address,
        pub payload_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `sendToken` function with signature `sendToken(string,string,string,uint256)` and selector `0x26ef699d`
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
    #[ethcall(name = "sendToken", abi = "sendToken(string,string,string,uint256)")]
    pub struct SendTokenCall {
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `setup` function with signature `setup(bytes)` and selector `0x9ded06df`
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
    #[ethcall(name = "setup", abi = "setup(bytes)")]
    pub struct SetupCall {
        pub params: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `tokenAddresses` function with signature `tokenAddresses(string)` and selector `0x935b13f6`
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
    #[ethcall(name = "tokenAddresses", abi = "tokenAddresses(string)")]
    pub struct TokenAddressesCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `tokenFrozen` function with signature `tokenFrozen(string)` and selector `0x7b1b769e`
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
    #[ethcall(name = "tokenFrozen", abi = "tokenFrozen(string)")]
    pub struct TokenFrozenCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `unfreezeAllTokens` function with signature `unfreezeAllTokens()` and selector `0xe3dfa299`
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
    #[ethcall(name = "unfreezeAllTokens", abi = "unfreezeAllTokens()")]
    pub struct UnfreezeAllTokensCall;
    ///Container type for all input parameters for the `unfreezeToken` function with signature `unfreezeToken(string)` and selector `0x34ff6983`
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
    #[ethcall(name = "unfreezeToken", abi = "unfreezeToken(string)")]
    pub struct UnfreezeTokenCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `upgrade` function with signature `upgrade(address,bytes32,bytes)` and selector `0xa3499c73`
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
    #[ethcall(name = "upgrade", abi = "upgrade(address,bytes32,bytes)")]
    pub struct UpgradeCall {
        pub new_implementation: ::ethers_core::types::Address,
        pub new_implementation_code_hash: [u8; 32],
        pub setup_params: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateContractCall` function with signature `validateContractCall(bytes32,string,string,bytes32)` and selector `0x5f6970c3`
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
        name = "validateContractCallAndMint",
        abi = "validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)"
    )]
    pub struct ValidateContractCallAndMintCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
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
    pub enum IAxelarGatewayCalls {
        AdminEpoch(AdminEpochCall),
        AdminThreshold(AdminThresholdCall),
        Admins(AdminsCall),
        AllTokensFrozen(AllTokensFrozenCall),
        CallContract(CallContractCall),
        CallContractWithToken(CallContractWithTokenCall),
        Execute(ExecuteCall),
        FreezeAllTokens(FreezeAllTokensCall),
        FreezeToken(FreezeTokenCall),
        Implementation(ImplementationCall),
        IsCommandExecuted(IsCommandExecutedCall),
        IsContractCallAndMintApproved(IsContractCallAndMintApprovedCall),
        IsContractCallApproved(IsContractCallApprovedCall),
        SendToken(SendTokenCall),
        Setup(SetupCall),
        TokenAddresses(TokenAddressesCall),
        TokenFrozen(TokenFrozenCall),
        UnfreezeAllTokens(UnfreezeAllTokensCall),
        UnfreezeToken(UnfreezeTokenCall),
        Upgrade(UpgradeCall),
        ValidateContractCall(ValidateContractCallCall),
        ValidateContractCallAndMint(ValidateContractCallAndMintCall),
    }
    impl ::ethers_core::abi::AbiDecode for IAxelarGatewayCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AdminEpochCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AdminEpoch(decoded));
            }
            if let Ok(decoded)
                = <AdminThresholdCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AdminThreshold(decoded));
            }
            if let Ok(decoded)
                = <AdminsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admins(decoded));
            }
            if let Ok(decoded)
                = <AllTokensFrozenCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AllTokensFrozen(decoded));
            }
            if let Ok(decoded)
                = <CallContractCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallContract(decoded));
            }
            if let Ok(decoded)
                = <CallContractWithTokenCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallContractWithToken(decoded));
            }
            if let Ok(decoded)
                = <ExecuteCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded)
                = <FreezeAllTokensCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FreezeAllTokens(decoded));
            }
            if let Ok(decoded)
                = <FreezeTokenCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FreezeToken(decoded));
            }
            if let Ok(decoded)
                = <ImplementationCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Implementation(decoded));
            }
            if let Ok(decoded)
                = <IsCommandExecutedCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsCommandExecuted(decoded));
            }
            if let Ok(decoded)
                = <IsContractCallAndMintApprovedCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsContractCallAndMintApproved(decoded));
            }
            if let Ok(decoded)
                = <IsContractCallApprovedCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsContractCallApproved(decoded));
            }
            if let Ok(decoded)
                = <SendTokenCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendToken(decoded));
            }
            if let Ok(decoded)
                = <SetupCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Setup(decoded));
            }
            if let Ok(decoded)
                = <TokenAddressesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenAddresses(decoded));
            }
            if let Ok(decoded)
                = <TokenFrozenCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenFrozen(decoded));
            }
            if let Ok(decoded)
                = <UnfreezeAllTokensCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnfreezeAllTokens(decoded));
            }
            if let Ok(decoded)
                = <UnfreezeTokenCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnfreezeToken(decoded));
            }
            if let Ok(decoded)
                = <UpgradeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Upgrade(decoded));
            }
            if let Ok(decoded)
                = <ValidateContractCallCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidateContractCall(decoded));
            }
            if let Ok(decoded)
                = <ValidateContractCallAndMintCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidateContractCallAndMint(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IAxelarGatewayCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminEpoch(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AdminThreshold(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Admins(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AllTokensFrozen(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CallContract(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CallContractWithToken(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Execute(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::FreezeAllTokens(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::FreezeToken(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Implementation(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsCommandExecuted(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsContractCallAndMintApproved(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsContractCallApproved(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SendToken(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Setup(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::TokenAddresses(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::TokenFrozen(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::UnfreezeAllTokens(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::UnfreezeToken(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Upgrade(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ValidateContractCall(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ValidateContractCallAndMint(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IAxelarGatewayCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admins(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTokensFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallContractWithToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreezeAllTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreezeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Implementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsCommandExecuted(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsContractCallAndMintApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsContractCallApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Setup(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnfreezeAllTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnfreezeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Upgrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateContractCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidateContractCallAndMint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AdminEpochCall> for IAxelarGatewayCalls {
        fn from(value: AdminEpochCall) -> Self {
            Self::AdminEpoch(value)
        }
    }
    impl ::core::convert::From<AdminThresholdCall> for IAxelarGatewayCalls {
        fn from(value: AdminThresholdCall) -> Self {
            Self::AdminThreshold(value)
        }
    }
    impl ::core::convert::From<AdminsCall> for IAxelarGatewayCalls {
        fn from(value: AdminsCall) -> Self {
            Self::Admins(value)
        }
    }
    impl ::core::convert::From<AllTokensFrozenCall> for IAxelarGatewayCalls {
        fn from(value: AllTokensFrozenCall) -> Self {
            Self::AllTokensFrozen(value)
        }
    }
    impl ::core::convert::From<CallContractCall> for IAxelarGatewayCalls {
        fn from(value: CallContractCall) -> Self {
            Self::CallContract(value)
        }
    }
    impl ::core::convert::From<CallContractWithTokenCall> for IAxelarGatewayCalls {
        fn from(value: CallContractWithTokenCall) -> Self {
            Self::CallContractWithToken(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for IAxelarGatewayCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<FreezeAllTokensCall> for IAxelarGatewayCalls {
        fn from(value: FreezeAllTokensCall) -> Self {
            Self::FreezeAllTokens(value)
        }
    }
    impl ::core::convert::From<FreezeTokenCall> for IAxelarGatewayCalls {
        fn from(value: FreezeTokenCall) -> Self {
            Self::FreezeToken(value)
        }
    }
    impl ::core::convert::From<ImplementationCall> for IAxelarGatewayCalls {
        fn from(value: ImplementationCall) -> Self {
            Self::Implementation(value)
        }
    }
    impl ::core::convert::From<IsCommandExecutedCall> for IAxelarGatewayCalls {
        fn from(value: IsCommandExecutedCall) -> Self {
            Self::IsCommandExecuted(value)
        }
    }
    impl ::core::convert::From<IsContractCallAndMintApprovedCall>
    for IAxelarGatewayCalls {
        fn from(value: IsContractCallAndMintApprovedCall) -> Self {
            Self::IsContractCallAndMintApproved(value)
        }
    }
    impl ::core::convert::From<IsContractCallApprovedCall> for IAxelarGatewayCalls {
        fn from(value: IsContractCallApprovedCall) -> Self {
            Self::IsContractCallApproved(value)
        }
    }
    impl ::core::convert::From<SendTokenCall> for IAxelarGatewayCalls {
        fn from(value: SendTokenCall) -> Self {
            Self::SendToken(value)
        }
    }
    impl ::core::convert::From<SetupCall> for IAxelarGatewayCalls {
        fn from(value: SetupCall) -> Self {
            Self::Setup(value)
        }
    }
    impl ::core::convert::From<TokenAddressesCall> for IAxelarGatewayCalls {
        fn from(value: TokenAddressesCall) -> Self {
            Self::TokenAddresses(value)
        }
    }
    impl ::core::convert::From<TokenFrozenCall> for IAxelarGatewayCalls {
        fn from(value: TokenFrozenCall) -> Self {
            Self::TokenFrozen(value)
        }
    }
    impl ::core::convert::From<UnfreezeAllTokensCall> for IAxelarGatewayCalls {
        fn from(value: UnfreezeAllTokensCall) -> Self {
            Self::UnfreezeAllTokens(value)
        }
    }
    impl ::core::convert::From<UnfreezeTokenCall> for IAxelarGatewayCalls {
        fn from(value: UnfreezeTokenCall) -> Self {
            Self::UnfreezeToken(value)
        }
    }
    impl ::core::convert::From<UpgradeCall> for IAxelarGatewayCalls {
        fn from(value: UpgradeCall) -> Self {
            Self::Upgrade(value)
        }
    }
    impl ::core::convert::From<ValidateContractCallCall> for IAxelarGatewayCalls {
        fn from(value: ValidateContractCallCall) -> Self {
            Self::ValidateContractCall(value)
        }
    }
    impl ::core::convert::From<ValidateContractCallAndMintCall> for IAxelarGatewayCalls {
        fn from(value: ValidateContractCallAndMintCall) -> Self {
            Self::ValidateContractCallAndMint(value)
        }
    }
    ///Container type for all return fields from the `adminEpoch` function with signature `adminEpoch()` and selector `0x364940d8`
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
    pub struct AdminEpochReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `adminThreshold` function with signature `adminThreshold(uint256)` and selector `0x88b30587`
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
    pub struct AdminThresholdReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `admins` function with signature `admins(uint256)` and selector `0x14bfd6d0`
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
    pub struct AdminsReturn(pub ::std::vec::Vec<::ethers_core::types::Address>);
    ///Container type for all return fields from the `allTokensFrozen` function with signature `allTokensFrozen()` and selector `0xaa1e1f0a`
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
    pub struct AllTokensFrozenReturn(pub bool);
    ///Container type for all return fields from the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    pub struct ImplementationReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `isCommandExecuted` function with signature `isCommandExecuted(bytes32)` and selector `0xd26ff210`
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
    pub struct IsCommandExecutedReturn(pub bool);
    ///Container type for all return fields from the `isContractCallAndMintApproved` function with signature `isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)` and selector `0xbc00c216`
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
    pub struct IsContractCallAndMintApprovedReturn(pub bool);
    ///Container type for all return fields from the `isContractCallApproved` function with signature `isContractCallApproved(bytes32,string,string,address,bytes32)` and selector `0xf6a5f9f5`
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
    pub struct IsContractCallApprovedReturn(pub bool);
    ///Container type for all return fields from the `tokenAddresses` function with signature `tokenAddresses(string)` and selector `0x935b13f6`
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
    pub struct TokenAddressesReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `tokenFrozen` function with signature `tokenFrozen(string)` and selector `0x7b1b769e`
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
    pub struct TokenFrozenReturn(pub bool);
    ///Container type for all return fields from the `validateContractCall` function with signature `validateContractCall(bytes32,string,string,bytes32)` and selector `0x5f6970c3`
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
    pub struct ValidateContractCallReturn(pub bool);
    ///Container type for all return fields from the `validateContractCallAndMint` function with signature `validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)` and selector `0x1876eed9`
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
    pub struct ValidateContractCallAndMintReturn(pub bool);
}
