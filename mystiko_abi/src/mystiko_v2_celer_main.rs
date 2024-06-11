pub use mystiko_v2_celer_main::*;
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
pub mod mystiko_v2_celer_main {
    const _: () = {
        ::core::include_bytes!(
"../json/MystikoV2CelerMain.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers_core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_hasher3"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IHasher3"),
                        ),
                    },
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_bridgeProxyAddress"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_settingsCenter"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_localConfig"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                            ::std::vec![
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                            ],
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "struct IMystikoBridge.LocalConfig",
                            ),
                        ),
                    },
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_peerConfig"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                            ::std::vec![
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                            ],
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "struct IMystikoBridge.PeerConfig",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("assetAddress"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetAddress"),
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
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assetType"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetType"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum AssetPool.AssetType"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bridgeProxyAddress"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bridgeProxyAddress"),
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
                    ::std::borrow::ToOwned::to_owned("bridgeType"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bridgeType"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("certDeposit"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("certDeposit"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_request"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IMystikoBridge.DepositRequest",
                                        ),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_certificateDeadline",
                                    ),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_certificateSignature",
                                    ),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultMaxAmount"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultMaxAmount"),
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
                    ::std::borrow::ToOwned::to_owned("defaultMinAmount"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultMinAmount"),
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
                    ::std::borrow::ToOwned::to_owned("defaultMinBridgeFee"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultMinBridgeFee",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("defaultPeerMinExecutorFee"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultPeerMinExecutorFee",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("defaultPeerMinRollupFee"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultPeerMinRollupFee",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_request"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IMystikoBridge.DepositRequest",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeMessage"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeMessage"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_executor"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAssociatedCommitmentPool"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAssociatedCommitmentPool",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getMaxAmount"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMaxAmount"),
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
                    ::std::borrow::ToOwned::to_owned("getMinAmount"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMinAmount"),
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
                    ::std::borrow::ToOwned::to_owned("getMinBridgeFee"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMinBridgeFee"),
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
                    ::std::borrow::ToOwned::to_owned("getMinExecutorFee"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMinExecutorFee"),
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
                    ::std::borrow::ToOwned::to_owned("getPeerMinRollupFee"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPeerMinRollupFee",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("isDepositsDisabled"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isDepositsDisabled"),
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
                    ::std::borrow::ToOwned::to_owned("isPeerContractSet"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPeerContractSet"),
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
                    ::std::borrow::ToOwned::to_owned("peerChainId"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("peerChainId"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peerChainName"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("peerChainName"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peerContract"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("peerContract"),
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
                    ::std::borrow::ToOwned::to_owned("setPeerContract"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPeerContract"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_peerContract"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers_core::abi::ethabi::ParamType::String,
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IMystikoBridge.PeerContract",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("settings"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("settings"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract MystikoBridgeSettings",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentCrossChain"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CommitmentCrossChain",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("commitment"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AmountLessThanZero"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AmountLessThanZero"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AmountTooLarge"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AmountTooLarge"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AmountTooSmall"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AmountTooSmall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssociatedPoolNotSet"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AssociatedPoolNotSet",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BridgeFeeTooFew"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BridgeFeeTooFew"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CertificateInvalid"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CertificateInvalid"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentHashIncorrect"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CommitmentHashIncorrect",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositsDisabled"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DepositsDisabled"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutorFeeTooFew"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ExecutorFeeTooFew"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HashKGreaterThanFieldSize"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HashKGreaterThanFieldSize",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotSupport"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotSupport"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PeerChainIdNotMatched"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PeerChainIdNotMatched",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PeerContractAlreadySet"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PeerContractAlreadySet",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PeerContractNotMatched"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PeerContractNotMatched",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RandomSGreaterThanFieldSize"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RandomSGreaterThanFieldSize",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RollupFeeToFew"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RollupFeeToFew"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SanctionedAddress"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SanctionedAddress"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SenderIsNotBridgeProxy"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SenderIsNotBridgeProxy",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MYSTIKOV2CELERMAIN_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90U4\x80\x15a\0\x1DW`\0\x80\xFD[P`@Qa'\08\x03\x80a'\0\x839\x81\x01`@\x81\x90Ra\0<\x91a\x01\x1DV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x08\x80T\x95\x87\x16\x95\x82\x16\x95\x90\x95\x17\x90\x94U\x81Q`\x03U` \x80\x83\x01Q`\x04U`@\x90\x92\x01Q`\x05U\x80Q`\x06U\x01Q`\x07U`\t\x80T\x91\x90\x93\x16\x91\x16\x17\x90Ua\x01\xD7V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xB4W`\0\x80\xFD[PV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xE7WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xE7WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\0\x85\x87\x03a\x01\0\x81\x12\x15a\x017W`\0\x80\xFD[\x86Qa\x01B\x81a\0\x9FV[` \x88\x01Q\x90\x96Pa\x01S\x81a\0\x9FV[`@\x88\x01Q\x90\x95Pa\x01d\x81a\0\x9FV[\x93P```_\x19\x82\x01\x12\x15a\x01xW`\0\x80\xFD[a\x01\x80a\0\xB7V[``\x88\x01Q\x81R`\x80\x88\x01Q` \x82\x01R`\xA0\x88\x01Q`@\x80\x83\x01\x91\x90\x91R\x90\x93P`\xBF\x19\x82\x01\x12\x15a\x01\xB2W`\0\x80\xFD[Pa\x01\xBBa\0\xEDV[`\xC0\x87\x01Q\x81R`\xE0\x90\x96\x01Q` \x87\x01RP\x92\x95\x91\x94P\x92\x90V[a%\x1A\x80a\x01\xE6`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x96W`\x005`\xE0\x1C\x80c\x9Cd\x9F\xDF\x11a\0\xE1W\x80c\xDD\xAC]\xC1\x11a\0\x8AW\x80c\xEF\xBF\xB2\xAE\x11a\0dW\x80c\xEF\xBF\xB2\xAE\x14a\x04#W\x80c\xF4\xAD\x17\xC6\x14a\x048W\x80c\xFAu\x0FV\x14a\x04MW\x80c\xFB>=s\x14a\x04nW`\0\x80\xFD[\x80c\xDD\xAC]\xC1\x14a\x03\xD9W\x80c\xE0at\xE4\x14a\x03\xEEW\x80c\xEDn\xA3:\x14a\x04\x0EW`\0\x80\xFD[\x80c\xCD\xFC\xEE\xBA\x11a\0\xBBW\x80c\xCD\xFC\xEE\xBA\x14a\x03mW\x80c\xCF\xC7\xE2\xDA\x14a\x03\xAEW\x80c\xD0\xB46\xBD\x14a\x03\xC3W`\0\x80\xFD[\x80c\x9Cd\x9F\xDF\x14a\x03!W\x80c\xCB\\\x02\x9A\x14a\x03DW\x80c\xCB\xE3B\x85\x14a\x03WW`\0\x80\xFD[\x80cB.\0(\x11a\x01CW\x80cd\x0C\x0B6\x11a\x01\x1DW\x80cd\x0C\x0B6\x14a\x02\xE3W\x80c\x82[_\x8D\x14a\x02\xF9W\x80c\x9A\x03cl\x14a\x03\x0EW`\0\x80\xFD[\x80cB.\0(\x14a\x02\x96W\x80cM\xDEo\xBC\x14a\x02\xB8W\x80cN<\x10\xB7\x14a\x02\xCEW`\0\x80\xFD[\x80c$!\xE1U\x11a\x01tW\x80c$!\xE1U\x14a\x02\x0BW\x80c,\xD2mE\x14a\x02ZW\x80c?\xE34z\x14a\x02zW`\0\x80\xFD[\x80c\x0B\xA9Y\t\x14a\x01\x9BW\x80c\x1B\xA4l\xFD\x14a\x01\xC3W\x80c!\xE3-U\x14a\x01\xEBW[`\0\x80\xFD[4\x80\x15a\x01\xA7W`\0\x80\xFD[Pa\x01\xB0a\x04\x84V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xCFW`\0\x80\xFD[P`\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xBAV[4\x80\x15a\x01\xF7W`\0\x80\xFD[P`\x02Ta\x01\xD3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\x17W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7Fceler\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x01\xBA\x91\x90a\x1DNV[4\x80\x15a\x02fW`\0\x80\xFD[P`\x08Ta\x01\xD3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\x86W`\0\x80\xFD[P`\x01`@Qa\x01\xBA\x91\x90a\x1DhV[4\x80\x15a\x02\xA2W`\0\x80\xFD[Pa\x02\xB6a\x02\xB16`\x04a\x1E\x9DV[a\x05\rV[\0[4\x80\x15a\x02\xC4W`\0\x80\xFD[Pa\x01\xB0`\x05T\x81V[4\x80\x15a\x02\xDAW`\0\x80\xFD[Pa\x02Ma\x05\xFAV[4\x80\x15a\x02\xEFW`\0\x80\xFD[Pa\x01\xB0`\x06T\x81V[4\x80\x15a\x03\x05W`\0\x80\xFD[Pa\x01\xB0a\x06\x88V[a\x02\xB6a\x03\x1C6`\x04a \"V[a\x07\x0FV[a\x034a\x03/6`\x04a _V[a\x07(V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBAV[a\x02\xB6a\x03R6`\x04a!\x0CV[a\x07\xB2V[4\x80\x15a\x03cW`\0\x80\xFD[Pa\x01\xB0`\x07T\x81V[4\x80\x15a\x03yW`\0\x80\xFD[P`\0Ta\x03\x95\x90`\x01`\xA8\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xBAV[4\x80\x15a\x03\xBAW`\0\x80\xFD[Pa\x01\xB0a\x0B\xDDV[4\x80\x15a\x03\xCFW`\0\x80\xFD[Pa\x01\xB0`\x04T\x81V[4\x80\x15a\x03\xE5W`\0\x80\xFD[Pa\x01\xD3a\x0CdV[4\x80\x15a\x03\xFAW`\0\x80\xFD[P`\tTa\x01\xD3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\x1AW`\0\x80\xFD[Pa\x034a\r\x03V[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x01\xB0a\ruV[4\x80\x15a\x04DW`\0\x80\xFD[Pa\x01\xB0a\r\xFCV[4\x80\x15a\x04YW`\0\x80\xFD[P`\0Ta\x034\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x04zW`\0\x80\xFD[Pa\x01\xB0`\x03T\x81V[`\tT`@QcG:\x061`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG:\x061\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF5\x91\x90a!~V[\x90P\x80\x15a\x05\x03W\x80a\x05\x07V[`\x04T[\x91PP\x90V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x058W`@Qcm\x8F\x11Q`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA8\x1B\x02\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U` \x81\x01Q`\x01\x90a\x05\x8C\x90\x82a\" V[P`@\x01Q`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90U`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x17\x90UV[`\x01\x80Ta\x06\x07\x90a!\x97V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x063\x90a!\x97V[\x80\x15a\x06\x80W\x80`\x1F\x10a\x06UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06cW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\tT`@Qca\x86<\x03`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC3\x0Cx\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xF9\x91\x90a!~V[\x90P\x80\x15a\x07\x07W\x80a\x05\x07V[PP`\x07T\x90V[`@Qc\xE7\xA2O\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07VW`@Qc=\xCA\x01\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\x97\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0E\x83\x92PPPV[\x90Pa\x07\xA5\x86\x88\x85\x84a\x0FKV[P`\x01\x96\x95PPPPPPV[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1E\x91\x90a\"\xDFV[\x15a\x08<W`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xBCXw\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xB3\x91\x90a\"\xDFV[\x15a\t\x80W`\0`@Q\x80`\x80\x01`@R\x802`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x08\xDC`\0\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x86\x90R`@\x91\x82\x01\x85\x90R`\tT\x91Qc\x84\x9E\x8B\x9F`\xE0\x1B\x81R\x92\x93P\x16\x90c\x84\x9E\x8B\x9F\x90a\t \x90\x84\x90`\x04\x01a#\x01V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ta\x91\x90a\"\xDFV[a\t~W`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[a\t\x88a\x0B\xDDV[\x83Q\x10\x15a\t\xA9W`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xB1a\x04\x84V[\x83Q\x11\x15a\t\xD2W`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xDAa\ruV[\x83`\xA0\x01Q\x10\x15a\t\xFEW`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x06a\r\xFCV[\x83`\xC0\x01Q\x10\x15a\n*W`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n2a\x06\x88V[\x83`\xE0\x01Q\x10\x15a\nVW`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\no\x84`@\x01Q\x85`\0\x01Q\x86``\x01Qa\x10>V[\x90P\x80\x84` \x01Q\x14a\n\x95W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT`@Qc\xDFY/}`\xE0\x1B\x81R2`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x01\x91\x90a\"\xDFV[\x15a\x0B\x1FW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x90\x82\x01R`\xC0\x86\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x85\x01Q``\x82\x01R`\x80\x80\x86\x01Q\x90\x82\x01R`\0a\x0Bb\x82a\x11UV[\x90Pa\x0Br\x86`\xA0\x01Q\x82a\x11\xC4V[a\x0B\xA7a\x0B}a\x0CdV[`\xE0\x88\x01Q`\xC0\x89\x01Q\x89Qa\x0B\x93\x91\x90a#LV[a\x0B\x9D\x91\x90a#LV[\x88`\xA0\x01Qa\x12JV[` \x86\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPPPV[`\tT`@QcU%\x98I`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cU%\x98I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CN\x91\x90a!~V[\x90P\x80\x15a\x0C\\W\x80a\x05\x07V[PP`\x03T\x90V[`\tT`@QcA\xFBiy`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cA\xFBiy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xD5\x91\x90a#mV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xFEW`@Qc\x06\xF3\xD63`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rp\x91\x90a\"\xDFV[\x90P\x90V[`\tT`@Qc{R\xB0\r`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c{R\xB0\r\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xE6\x91\x90a!~V[\x90P\x80\x15a\r\xF4W\x80a\x05\x07V[PP`\x05T\x90V[`\tT`@QcO1O\x9D`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9Eb\x9F:\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EIW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Em\x91\x90a!~V[\x90P\x80\x15a\x0E{W\x80a\x05\x07V[PP`\x06T\x90V[a\x0E\xB5`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\x0E\xE7`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\x0E\xF3\x84\x82a\x13PV[\x90\x83R\x90Pa\x0F\x02\x84\x82a\x13PV[` \x84\x01\x91\x90\x91R\x90Pa\x0F\x16\x84\x82a\x13PV[`@\x84\x01\x91\x90\x91R\x90Pa\x0F*\x84\x82a\x13PV[``\x84\x01\x91\x90\x91R\x90Pa\x0F>\x84\x82a\x14\x81V[P`\x80\x83\x01RP\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x0FyW`@Qc\x1BI^\xCF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xA8\x1B\x90\x92\x04\x16\x14a\x0F\xAFW`@Qc7;\xC1!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x03a\x0F\xD1W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xD9a\x0CdV[`\x01`\x01`\xA0\x1B\x03\x16cx\xD6\x0C\xD7\x82\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x06\x92\x91\x90a#\x8AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10 W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x104W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x10\x81W`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x10\xB3W`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x11\x0B\x91`\x04\x01a#\xE7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11L\x91\x90a!~V[\x95\x94PPPPPV[``\x80a\x11e\x83`\0\x01Qa\x15\x8EV[a\x11r\x84` \x01Qa\x15\x8EV[a\x11\x7F\x85`@\x01Qa\x15\x8EV[a\x11\x8C\x86``\x01Qa\x15\x8EV[a\x11\x99\x87`\x80\x01Qa\x16&V[`@Q` \x01a\x11\xAD\x95\x94\x93\x92\x91\x90a$\x18V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[`\x08T`\x02T`\0T`@QcO\x9Er\xAD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93c\x9F<\xE5Z\x93\x87\x93a\x12\x14\x93\x91\x90\x92\x16\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\xA8\x1B\x90\x91\x04\x16\x90\x87\x90`\x04\x01a$\x83V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x12-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12AW=`\0\x80>=`\0\xFD[PPPPPPPV[a\x12T\x81\x83a#LV[4\x14a\x12\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Finsufficient token\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x12\xF4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x12\xF9V[``\x91P[PP\x90P\x80a\x13JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Famount transfer failed\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x12\x9EV[PPPPV[`\0\x80\x83Q\x83` a\x13b\x91\x90a#LV[\x11\x15\x80\x15a\x13yWPa\x13v\x83` a#LV[\x83\x10[a\x13\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\x12\x9EV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x14\x06W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x13\xE6V[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x14hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x12\x9EV[\x80a\x14t\x85` a#LV[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a\x14\x90\x85\x85a\x16]V[\x86Q\x90\x95P\x90\x91Pa\x14\xA2\x82\x86a#LV[\x11\x15\x80\x15a\x14\xB8WPa\x14\xB5\x81\x85a#LV[\x84\x10[a\x15\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\x12\x9EV[``\x81\x15\x80\x15a\x15+W`@Q\x91P` \x82\x01`@Ra\x15uV[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a\x15dW\x80Q\x83R` \x92\x83\x01\x92\x01a\x15LV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a\x15\x81\x83\x87a#LV[\x93P\x93PPP\x92P\x92\x90PV[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x15\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\x12\x9EV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\x16\x16W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x15\xF4V[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\x164\x81a\x18aV[\x83`@Q` \x01a\x16F\x92\x91\x90a$\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`\0\x80`\0a\x16l\x85\x85a\x19+V[\x94P\x90P`\0`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\xFD`\xF8\x1B\x03a\x17\x04Wa\x16\x91\x86\x86a\x19\xB3V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a\x16\xACWPa\xFF\xFF\x81\x11\x15[a\x16\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\x12\x9EV[\x92P\x83\x91Pa\x14z\x90PV[`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\x7F`\xF9\x1B\x03a\x17\x8EWa\x17#\x86\x86a\x1AlV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a\x17BWPc\xFF\xFF\xFF\xFF\x81\x11\x15[a\x16\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x12\x9EV[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x90\x03a\x18\x0BWa\x17\xAA\x86\x86a\x1B=V[\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a\x16\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x12\x9EV[P`\xF8\x81\x90\x1C`\xFD\x81\x10a\x16\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x12\x9EV[```\xFD\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x18\x96W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91R[\x92\x91PPV[a\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x18\xE6Wa\x18\xB6`\xFD`\xF8\x1Ba\x1C\x0EV[a\x18\xBF\x83a\x1C5V[`@Q` \x01a\x18\xD0\x92\x91\x90a$\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x19\x11Wa\x19\x08`\x7F`\xF9\x1Ba\x1C\x0EV[a\x18\xBF\x83a\x1CxV[a\x19\"`\x01`\x01`\xF8\x1B\x03\x19a\x1C\x0EV[a\x18\xBF\x83a\x1C\xBBV[`\0\x80\x83Q\x83`\x01a\x19=\x91\x90a#LV[\x11\x15\x80\x15a\x19TWPa\x19Q\x83`\x01a#LV[\x83\x10[a\x19\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\x12\x9EV[\x83\x83\x01` \x01Q\x80a\x14t\x85`\x01a#LV[`\0\x80\x83Q\x83`\x02a\x19\xC5\x91\x90a#LV[\x11\x15\x80\x15a\x19\xDCWPa\x19\xD9\x83`\x02a#LV[\x83\x10[a\x1A3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12\x9EV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a\x14t\x91\x90a#LV[`\0\x80\x83Q\x83`\x04a\x1A~\x91\x90a#LV[\x11\x15\x80\x15a\x1A\x95WPa\x1A\x92\x83`\x04a#LV[\x83\x10[a\x1A\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12\x9EV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1B!W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1B\x01V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x14t\x85`\x04a#LV[`\0\x80\x83Q\x83`\x08a\x1BO\x91\x90a#LV[\x11\x15\x80\x15a\x1BfWPa\x1Bc\x83`\x08a#LV[\x83\x10[a\x1B\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12\x9EV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1B\xF2W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1B\xD2V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x14t\x85`\x08a#LV[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a\x18\x90V[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1ChW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1CFV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1C\xABW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1C\x89V[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1C\xEEW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1C\xCCV[PPP`(\x81\x01`@R\x92\x91PPV[`\0[\x83\x81\x10\x15a\x1D\x19W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1D\x01V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1D:\x81` \x86\x01` \x86\x01a\x1C\xFEV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1Da` \x83\x01\x84a\x1D\"V[\x93\x92PPPV[` \x81\x01`\x02\x83\x10a\x1D\x8AWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xC9Wa\x1D\xC9a\x1D\x90V[`@R\x90V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xC9Wa\x1D\xC9a\x1D\x90V[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\xFEW`\0\x80\xFD[`\0\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a\x1E&Wa\x1E&a\x1D\x90V[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1EUWa\x1EUa\x1D\x90V[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x1EmW`\0\x80\xFD[\x83\x83` \x83\x017`\0` \x85\x83\x01\x01RP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E\x9AW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x1E\xAFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xC6W`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a\x1E\xD8W`\0\x80\xFD[a\x1E\xE0a\x1D\xA6V[a\x1E\xE9\x82a\x1D\xF3V[\x81R` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\x05W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x86\x13a\x1F\x16W`\0\x80\xFD[a\x1F%\x86\x825` \x84\x01a\x1E\x0BV[` \x83\x01RP`@\x82\x015\x91Pa\x1F;\x82a\x1E\x85V[`@\x81\x01\x91\x90\x91R\x93\x92PPPV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\xFEW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x1F{W`\0\x80\xFD[a\x1Da\x83\x835` \x85\x01a\x1E\x0BV[`\0a\x01\0\x82\x84\x03\x12\x15a\x1F\x9DW`\0\x80\xFD[a\x1F\xA5a\x1D\xCFV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R\x90Pa\x1F\xCB``\x83\x01a\x1FJV[``\x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xEAW`\0\x80\xFD[a\x1F\xF6\x84\x82\x85\x01a\x1FjV[`\x80\x83\x01RP`\xA0\x82\x81\x015\x90\x82\x01R`\xC0\x80\x83\x015\x90\x82\x01R`\xE0\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a 4W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a KW`\0\x80\xFD[a W\x84\x82\x85\x01a\x1F\x8AV[\x94\x93PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a wW`\0\x80\xFD[\x855a \x82\x81a\x1E\x85V[\x94Pa \x90` \x87\x01a\x1D\xF3V[\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xACW`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a \xBDW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xD4W`\0\x80\xFD[\x88` \x82\x84\x01\x01\x11\x15a \xE6W`\0\x80\xFD[` \x91\x90\x91\x01\x93P\x91P``\x86\x015a \xFE\x81a\x1E\x85V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!!W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!8W`\0\x80\xFD[a!D\x86\x82\x87\x01a\x1F\x8AV[\x93PP` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!hW`\0\x80\xFD[a!t\x86\x82\x87\x01a\x1FjV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a!\x90W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a!\xABW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a!\xCBWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\"\x1BW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a!\xF8WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\"\x18W`\0\x81U`\x01\x01a\"\x04V[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\":Wa\":a\x1D\x90V[a\"N\x81a\"H\x84Ta!\x97V[\x84a!\xD1V[` `\x1F\x82\x11`\x01\x81\x14a\"\x82W`\0\x83\x15a\"jWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\"\x18V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\"\xB2W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\"\x92V[P\x84\x82\x10\x15a\"\xD0W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15a\"\xF1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1DaW`\0\x80\xFD[` \x81R`\x01`\x01`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q``\x82\x01R`\0``\x83\x01Q`\x80\x80\x84\x01Ra W`\xA0\x84\x01\x82a\x1D\"V[\x80\x82\x01\x80\x82\x11\x15a\x18\x90WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a#\x7FW`\0\x80\xFD[\x81Qa\x1Da\x81a\x1E\x85V[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra#\xCE`\xE0\x84\x01\x82a\x1D\"V[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a$\x0FW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a#\xF0V[PPP\x92\x91PPV[`\0\x86Qa$*\x81\x84` \x8B\x01a\x1C\xFEV[\x86Q\x90\x83\x01\x90a$>\x81\x83` \x8B\x01a\x1C\xFEV[\x86Q\x91\x01\x90a$Q\x81\x83` \x8A\x01a\x1C\xFEV[\x85Q\x91\x01\x90a$d\x81\x83` \x89\x01a\x1C\xFEV[\x84Q\x91\x01\x90a$w\x81\x83` \x88\x01a\x1C\xFEV[\x01\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R```@\x82\x01R`\0a\x11L``\x83\x01\x84a\x1D\"V[`\0\x83Qa$\xC7\x81\x84` \x88\x01a\x1C\xFEV[\x83Q\x90\x83\x01\x90a$\xDB\x81\x83` \x88\x01a\x1C\xFEV[\x01\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 ]1\xF1/\x90\xAE\xCF\xC8\xBA\x8C\x1B\x9C\xBB5\xB8\x93\xE2S\x88dq\x19EM\xA1\xE3\xDE:\xC9(z\xFAdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MYSTIKOV2CELERMAIN_BYTECODE: ::ethers_core::types::Bytes = ::ethers_core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x96W`\x005`\xE0\x1C\x80c\x9Cd\x9F\xDF\x11a\0\xE1W\x80c\xDD\xAC]\xC1\x11a\0\x8AW\x80c\xEF\xBF\xB2\xAE\x11a\0dW\x80c\xEF\xBF\xB2\xAE\x14a\x04#W\x80c\xF4\xAD\x17\xC6\x14a\x048W\x80c\xFAu\x0FV\x14a\x04MW\x80c\xFB>=s\x14a\x04nW`\0\x80\xFD[\x80c\xDD\xAC]\xC1\x14a\x03\xD9W\x80c\xE0at\xE4\x14a\x03\xEEW\x80c\xEDn\xA3:\x14a\x04\x0EW`\0\x80\xFD[\x80c\xCD\xFC\xEE\xBA\x11a\0\xBBW\x80c\xCD\xFC\xEE\xBA\x14a\x03mW\x80c\xCF\xC7\xE2\xDA\x14a\x03\xAEW\x80c\xD0\xB46\xBD\x14a\x03\xC3W`\0\x80\xFD[\x80c\x9Cd\x9F\xDF\x14a\x03!W\x80c\xCB\\\x02\x9A\x14a\x03DW\x80c\xCB\xE3B\x85\x14a\x03WW`\0\x80\xFD[\x80cB.\0(\x11a\x01CW\x80cd\x0C\x0B6\x11a\x01\x1DW\x80cd\x0C\x0B6\x14a\x02\xE3W\x80c\x82[_\x8D\x14a\x02\xF9W\x80c\x9A\x03cl\x14a\x03\x0EW`\0\x80\xFD[\x80cB.\0(\x14a\x02\x96W\x80cM\xDEo\xBC\x14a\x02\xB8W\x80cN<\x10\xB7\x14a\x02\xCEW`\0\x80\xFD[\x80c$!\xE1U\x11a\x01tW\x80c$!\xE1U\x14a\x02\x0BW\x80c,\xD2mE\x14a\x02ZW\x80c?\xE34z\x14a\x02zW`\0\x80\xFD[\x80c\x0B\xA9Y\t\x14a\x01\x9BW\x80c\x1B\xA4l\xFD\x14a\x01\xC3W\x80c!\xE3-U\x14a\x01\xEBW[`\0\x80\xFD[4\x80\x15a\x01\xA7W`\0\x80\xFD[Pa\x01\xB0a\x04\x84V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xCFW`\0\x80\xFD[P`\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xBAV[4\x80\x15a\x01\xF7W`\0\x80\xFD[P`\x02Ta\x01\xD3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\x17W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7Fceler\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x01\xBA\x91\x90a\x1DNV[4\x80\x15a\x02fW`\0\x80\xFD[P`\x08Ta\x01\xD3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\x86W`\0\x80\xFD[P`\x01`@Qa\x01\xBA\x91\x90a\x1DhV[4\x80\x15a\x02\xA2W`\0\x80\xFD[Pa\x02\xB6a\x02\xB16`\x04a\x1E\x9DV[a\x05\rV[\0[4\x80\x15a\x02\xC4W`\0\x80\xFD[Pa\x01\xB0`\x05T\x81V[4\x80\x15a\x02\xDAW`\0\x80\xFD[Pa\x02Ma\x05\xFAV[4\x80\x15a\x02\xEFW`\0\x80\xFD[Pa\x01\xB0`\x06T\x81V[4\x80\x15a\x03\x05W`\0\x80\xFD[Pa\x01\xB0a\x06\x88V[a\x02\xB6a\x03\x1C6`\x04a \"V[a\x07\x0FV[a\x034a\x03/6`\x04a _V[a\x07(V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBAV[a\x02\xB6a\x03R6`\x04a!\x0CV[a\x07\xB2V[4\x80\x15a\x03cW`\0\x80\xFD[Pa\x01\xB0`\x07T\x81V[4\x80\x15a\x03yW`\0\x80\xFD[P`\0Ta\x03\x95\x90`\x01`\xA8\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xBAV[4\x80\x15a\x03\xBAW`\0\x80\xFD[Pa\x01\xB0a\x0B\xDDV[4\x80\x15a\x03\xCFW`\0\x80\xFD[Pa\x01\xB0`\x04T\x81V[4\x80\x15a\x03\xE5W`\0\x80\xFD[Pa\x01\xD3a\x0CdV[4\x80\x15a\x03\xFAW`\0\x80\xFD[P`\tTa\x01\xD3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\x1AW`\0\x80\xFD[Pa\x034a\r\x03V[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x01\xB0a\ruV[4\x80\x15a\x04DW`\0\x80\xFD[Pa\x01\xB0a\r\xFCV[4\x80\x15a\x04YW`\0\x80\xFD[P`\0Ta\x034\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x04zW`\0\x80\xFD[Pa\x01\xB0`\x03T\x81V[`\tT`@QcG:\x061`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG:\x061\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF5\x91\x90a!~V[\x90P\x80\x15a\x05\x03W\x80a\x05\x07V[`\x04T[\x91PP\x90V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x058W`@Qcm\x8F\x11Q`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA8\x1B\x02\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U` \x81\x01Q`\x01\x90a\x05\x8C\x90\x82a\" V[P`@\x01Q`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90U`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x17\x90UV[`\x01\x80Ta\x06\x07\x90a!\x97V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x063\x90a!\x97V[\x80\x15a\x06\x80W\x80`\x1F\x10a\x06UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06cW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\tT`@Qca\x86<\x03`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC3\x0Cx\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xF9\x91\x90a!~V[\x90P\x80\x15a\x07\x07W\x80a\x05\x07V[PP`\x07T\x90V[`@Qc\xE7\xA2O\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07VW`@Qc=\xCA\x01\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\x97\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0E\x83\x92PPPV[\x90Pa\x07\xA5\x86\x88\x85\x84a\x0FKV[P`\x01\x96\x95PPPPPPV[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1E\x91\x90a\"\xDFV[\x15a\x08<W`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xBCXw\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xB3\x91\x90a\"\xDFV[\x15a\t\x80W`\0`@Q\x80`\x80\x01`@R\x802`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x08\xDC`\0\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x86\x90R`@\x91\x82\x01\x85\x90R`\tT\x91Qc\x84\x9E\x8B\x9F`\xE0\x1B\x81R\x92\x93P\x16\x90c\x84\x9E\x8B\x9F\x90a\t \x90\x84\x90`\x04\x01a#\x01V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ta\x91\x90a\"\xDFV[a\t~W`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[a\t\x88a\x0B\xDDV[\x83Q\x10\x15a\t\xA9W`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xB1a\x04\x84V[\x83Q\x11\x15a\t\xD2W`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xDAa\ruV[\x83`\xA0\x01Q\x10\x15a\t\xFEW`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x06a\r\xFCV[\x83`\xC0\x01Q\x10\x15a\n*W`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n2a\x06\x88V[\x83`\xE0\x01Q\x10\x15a\nVW`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\no\x84`@\x01Q\x85`\0\x01Q\x86``\x01Qa\x10>V[\x90P\x80\x84` \x01Q\x14a\n\x95W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT`@Qc\xDFY/}`\xE0\x1B\x81R2`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x01\x91\x90a\"\xDFV[\x15a\x0B\x1FW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x90\x82\x01R`\xC0\x86\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x85\x01Q``\x82\x01R`\x80\x80\x86\x01Q\x90\x82\x01R`\0a\x0Bb\x82a\x11UV[\x90Pa\x0Br\x86`\xA0\x01Q\x82a\x11\xC4V[a\x0B\xA7a\x0B}a\x0CdV[`\xE0\x88\x01Q`\xC0\x89\x01Q\x89Qa\x0B\x93\x91\x90a#LV[a\x0B\x9D\x91\x90a#LV[\x88`\xA0\x01Qa\x12JV[` \x86\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPPPV[`\tT`@QcU%\x98I`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cU%\x98I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CN\x91\x90a!~V[\x90P\x80\x15a\x0C\\W\x80a\x05\x07V[PP`\x03T\x90V[`\tT`@QcA\xFBiy`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cA\xFBiy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xD5\x91\x90a#mV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xFEW`@Qc\x06\xF3\xD63`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rp\x91\x90a\"\xDFV[\x90P\x90V[`\tT`@Qc{R\xB0\r`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c{R\xB0\r\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xE6\x91\x90a!~V[\x90P\x80\x15a\r\xF4W\x80a\x05\x07V[PP`\x05T\x90V[`\tT`@QcO1O\x9D`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9Eb\x9F:\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EIW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Em\x91\x90a!~V[\x90P\x80\x15a\x0E{W\x80a\x05\x07V[PP`\x06T\x90V[a\x0E\xB5`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\x0E\xE7`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\x0E\xF3\x84\x82a\x13PV[\x90\x83R\x90Pa\x0F\x02\x84\x82a\x13PV[` \x84\x01\x91\x90\x91R\x90Pa\x0F\x16\x84\x82a\x13PV[`@\x84\x01\x91\x90\x91R\x90Pa\x0F*\x84\x82a\x13PV[``\x84\x01\x91\x90\x91R\x90Pa\x0F>\x84\x82a\x14\x81V[P`\x80\x83\x01RP\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x0FyW`@Qc\x1BI^\xCF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xA8\x1B\x90\x92\x04\x16\x14a\x0F\xAFW`@Qc7;\xC1!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x03a\x0F\xD1W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xD9a\x0CdV[`\x01`\x01`\xA0\x1B\x03\x16cx\xD6\x0C\xD7\x82\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x06\x92\x91\x90a#\x8AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10 W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x104W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x10\x81W`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x10\xB3W`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x11\x0B\x91`\x04\x01a#\xE7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11L\x91\x90a!~V[\x95\x94PPPPPV[``\x80a\x11e\x83`\0\x01Qa\x15\x8EV[a\x11r\x84` \x01Qa\x15\x8EV[a\x11\x7F\x85`@\x01Qa\x15\x8EV[a\x11\x8C\x86``\x01Qa\x15\x8EV[a\x11\x99\x87`\x80\x01Qa\x16&V[`@Q` \x01a\x11\xAD\x95\x94\x93\x92\x91\x90a$\x18V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[`\x08T`\x02T`\0T`@QcO\x9Er\xAD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93c\x9F<\xE5Z\x93\x87\x93a\x12\x14\x93\x91\x90\x92\x16\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\xA8\x1B\x90\x91\x04\x16\x90\x87\x90`\x04\x01a$\x83V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x12-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12AW=`\0\x80>=`\0\xFD[PPPPPPPV[a\x12T\x81\x83a#LV[4\x14a\x12\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Finsufficient token\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x12\xF4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x12\xF9V[``\x91P[PP\x90P\x80a\x13JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Famount transfer failed\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x12\x9EV[PPPPV[`\0\x80\x83Q\x83` a\x13b\x91\x90a#LV[\x11\x15\x80\x15a\x13yWPa\x13v\x83` a#LV[\x83\x10[a\x13\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\x12\x9EV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x14\x06W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x13\xE6V[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x14hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x12\x9EV[\x80a\x14t\x85` a#LV[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a\x14\x90\x85\x85a\x16]V[\x86Q\x90\x95P\x90\x91Pa\x14\xA2\x82\x86a#LV[\x11\x15\x80\x15a\x14\xB8WPa\x14\xB5\x81\x85a#LV[\x84\x10[a\x15\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\x12\x9EV[``\x81\x15\x80\x15a\x15+W`@Q\x91P` \x82\x01`@Ra\x15uV[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a\x15dW\x80Q\x83R` \x92\x83\x01\x92\x01a\x15LV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a\x15\x81\x83\x87a#LV[\x93P\x93PPP\x92P\x92\x90PV[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x15\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\x12\x9EV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\x16\x16W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x15\xF4V[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\x164\x81a\x18aV[\x83`@Q` \x01a\x16F\x92\x91\x90a$\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`\0\x80`\0a\x16l\x85\x85a\x19+V[\x94P\x90P`\0`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\xFD`\xF8\x1B\x03a\x17\x04Wa\x16\x91\x86\x86a\x19\xB3V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a\x16\xACWPa\xFF\xFF\x81\x11\x15[a\x16\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\x12\x9EV[\x92P\x83\x91Pa\x14z\x90PV[`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\x7F`\xF9\x1B\x03a\x17\x8EWa\x17#\x86\x86a\x1AlV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a\x17BWPc\xFF\xFF\xFF\xFF\x81\x11\x15[a\x16\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x12\x9EV[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x90\x03a\x18\x0BWa\x17\xAA\x86\x86a\x1B=V[\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a\x16\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x12\x9EV[P`\xF8\x81\x90\x1C`\xFD\x81\x10a\x16\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x12\x9EV[```\xFD\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x18\x96W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91R[\x92\x91PPV[a\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x18\xE6Wa\x18\xB6`\xFD`\xF8\x1Ba\x1C\x0EV[a\x18\xBF\x83a\x1C5V[`@Q` \x01a\x18\xD0\x92\x91\x90a$\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x19\x11Wa\x19\x08`\x7F`\xF9\x1Ba\x1C\x0EV[a\x18\xBF\x83a\x1CxV[a\x19\"`\x01`\x01`\xF8\x1B\x03\x19a\x1C\x0EV[a\x18\xBF\x83a\x1C\xBBV[`\0\x80\x83Q\x83`\x01a\x19=\x91\x90a#LV[\x11\x15\x80\x15a\x19TWPa\x19Q\x83`\x01a#LV[\x83\x10[a\x19\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\x12\x9EV[\x83\x83\x01` \x01Q\x80a\x14t\x85`\x01a#LV[`\0\x80\x83Q\x83`\x02a\x19\xC5\x91\x90a#LV[\x11\x15\x80\x15a\x19\xDCWPa\x19\xD9\x83`\x02a#LV[\x83\x10[a\x1A3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12\x9EV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a\x14t\x91\x90a#LV[`\0\x80\x83Q\x83`\x04a\x1A~\x91\x90a#LV[\x11\x15\x80\x15a\x1A\x95WPa\x1A\x92\x83`\x04a#LV[\x83\x10[a\x1A\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12\x9EV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1B!W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1B\x01V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x14t\x85`\x04a#LV[`\0\x80\x83Q\x83`\x08a\x1BO\x91\x90a#LV[\x11\x15\x80\x15a\x1BfWPa\x1Bc\x83`\x08a#LV[\x83\x10[a\x1B\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12\x9EV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1B\xF2W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1B\xD2V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x14t\x85`\x08a#LV[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a\x18\x90V[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1ChW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1CFV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1C\xABW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1C\x89V[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1C\xEEW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1C\xCCV[PPP`(\x81\x01`@R\x92\x91PPV[`\0[\x83\x81\x10\x15a\x1D\x19W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1D\x01V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1D:\x81` \x86\x01` \x86\x01a\x1C\xFEV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1Da` \x83\x01\x84a\x1D\"V[\x93\x92PPPV[` \x81\x01`\x02\x83\x10a\x1D\x8AWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xC9Wa\x1D\xC9a\x1D\x90V[`@R\x90V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xC9Wa\x1D\xC9a\x1D\x90V[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\xFEW`\0\x80\xFD[`\0\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a\x1E&Wa\x1E&a\x1D\x90V[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1EUWa\x1EUa\x1D\x90V[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x1EmW`\0\x80\xFD[\x83\x83` \x83\x017`\0` \x85\x83\x01\x01RP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E\x9AW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x1E\xAFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xC6W`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a\x1E\xD8W`\0\x80\xFD[a\x1E\xE0a\x1D\xA6V[a\x1E\xE9\x82a\x1D\xF3V[\x81R` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\x05W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x86\x13a\x1F\x16W`\0\x80\xFD[a\x1F%\x86\x825` \x84\x01a\x1E\x0BV[` \x83\x01RP`@\x82\x015\x91Pa\x1F;\x82a\x1E\x85V[`@\x81\x01\x91\x90\x91R\x93\x92PPPV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\xFEW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x1F{W`\0\x80\xFD[a\x1Da\x83\x835` \x85\x01a\x1E\x0BV[`\0a\x01\0\x82\x84\x03\x12\x15a\x1F\x9DW`\0\x80\xFD[a\x1F\xA5a\x1D\xCFV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R\x90Pa\x1F\xCB``\x83\x01a\x1FJV[``\x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xEAW`\0\x80\xFD[a\x1F\xF6\x84\x82\x85\x01a\x1FjV[`\x80\x83\x01RP`\xA0\x82\x81\x015\x90\x82\x01R`\xC0\x80\x83\x015\x90\x82\x01R`\xE0\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a 4W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a KW`\0\x80\xFD[a W\x84\x82\x85\x01a\x1F\x8AV[\x94\x93PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a wW`\0\x80\xFD[\x855a \x82\x81a\x1E\x85V[\x94Pa \x90` \x87\x01a\x1D\xF3V[\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xACW`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a \xBDW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xD4W`\0\x80\xFD[\x88` \x82\x84\x01\x01\x11\x15a \xE6W`\0\x80\xFD[` \x91\x90\x91\x01\x93P\x91P``\x86\x015a \xFE\x81a\x1E\x85V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!!W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!8W`\0\x80\xFD[a!D\x86\x82\x87\x01a\x1F\x8AV[\x93PP` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!hW`\0\x80\xFD[a!t\x86\x82\x87\x01a\x1FjV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a!\x90W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a!\xABW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a!\xCBWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\"\x1BW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a!\xF8WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\"\x18W`\0\x81U`\x01\x01a\"\x04V[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\":Wa\":a\x1D\x90V[a\"N\x81a\"H\x84Ta!\x97V[\x84a!\xD1V[` `\x1F\x82\x11`\x01\x81\x14a\"\x82W`\0\x83\x15a\"jWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\"\x18V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\"\xB2W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\"\x92V[P\x84\x82\x10\x15a\"\xD0W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15a\"\xF1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1DaW`\0\x80\xFD[` \x81R`\x01`\x01`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q``\x82\x01R`\0``\x83\x01Q`\x80\x80\x84\x01Ra W`\xA0\x84\x01\x82a\x1D\"V[\x80\x82\x01\x80\x82\x11\x15a\x18\x90WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a#\x7FW`\0\x80\xFD[\x81Qa\x1Da\x81a\x1E\x85V[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra#\xCE`\xE0\x84\x01\x82a\x1D\"V[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a$\x0FW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a#\xF0V[PPP\x92\x91PPV[`\0\x86Qa$*\x81\x84` \x8B\x01a\x1C\xFEV[\x86Q\x90\x83\x01\x90a$>\x81\x83` \x8B\x01a\x1C\xFEV[\x86Q\x91\x01\x90a$Q\x81\x83` \x8A\x01a\x1C\xFEV[\x85Q\x91\x01\x90a$d\x81\x83` \x89\x01a\x1C\xFEV[\x84Q\x91\x01\x90a$w\x81\x83` \x88\x01a\x1C\xFEV[\x01\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R```@\x82\x01R`\0a\x11L``\x83\x01\x84a\x1D\"V[`\0\x83Qa$\xC7\x81\x84` \x88\x01a\x1C\xFEV[\x83Q\x90\x83\x01\x90a$\xDB\x81\x83` \x88\x01a\x1C\xFEV[\x01\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 ]1\xF1/\x90\xAE\xCF\xC8\xBA\x8C\x1B\x9C\xBB5\xB8\x93\xE2S\x88dq\x19EM\xA1\xE3\xDE:\xC9(z\xFAdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MYSTIKOV2CELERMAIN_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes = ::ethers_core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MystikoV2CelerMain<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoV2CelerMain<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoV2CelerMain<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoV2CelerMain<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoV2CelerMain<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MystikoV2CelerMain))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> MystikoV2CelerMain<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    MYSTIKOV2CELERMAIN_ABI.clone(),
                    client,
                ),
            )
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
        pub fn deploy<T: ::ethers_core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers_contract::builders::ContractDeployer<M, Self>,
            ::ethers_contract::ContractError<M>,
        > {
            let factory = ::ethers_contract::ContractFactory::new(
                MYSTIKOV2CELERMAIN_ABI.clone(),
                MYSTIKOV2CELERMAIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `assetAddress` (0x1ba46cfd) function
        pub fn asset_address(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([27, 164, 108, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetType` (0x3fe3347a) function
        pub fn asset_type(&self) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([63, 227, 52, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeProxyAddress` (0x2cd26d45) function
        pub fn bridge_proxy_address(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([44, 210, 109, 69], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeType` (0x2421e155) function
        pub fn bridge_type(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([36, 33, 225, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `certDeposit` (0xcb5c029a) function
        pub fn cert_deposit(
            &self,
            request: DepositRequest,
            certificate_deadline: ::ethers_core::types::U256,
            certificate_signature: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [203, 92, 2, 154],
                    (request, certificate_deadline, certificate_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultMaxAmount` (0xd0b436bd) function
        pub fn default_max_amount(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([208, 180, 54, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultMinAmount` (0xfb3e3d73) function
        pub fn default_min_amount(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([251, 62, 61, 115], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultMinBridgeFee` (0x4dde6fbc) function
        pub fn default_min_bridge_fee(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([77, 222, 111, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultPeerMinExecutorFee` (0x640c0b36) function
        pub fn default_peer_min_executor_fee(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([100, 12, 11, 54], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultPeerMinRollupFee` (0xcbe34285) function
        pub fn default_peer_min_rollup_fee(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([203, 227, 66, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x9a03636c) function
        pub fn deposit(
            &self,
            request: DepositRequest,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 3, 99, 108], (request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeMessage` (0x9c649fdf) function
        pub fn execute_message(
            &self,
            sender: ::ethers_core::types::Address,
            src_chain_id: u64,
            message: ::ethers_core::types::Bytes,
            executor: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [156, 100, 159, 223],
                    (sender, src_chain_id, message, executor),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAssociatedCommitmentPool` (0xddac5dc1) function
        pub fn get_associated_commitment_pool(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([221, 172, 93, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxAmount` (0x0ba95909) function
        pub fn get_max_amount(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([11, 169, 89, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinAmount` (0xcfc7e2da) function
        pub fn get_min_amount(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([207, 199, 226, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinBridgeFee` (0xefbfb2ae) function
        pub fn get_min_bridge_fee(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([239, 191, 178, 174], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinExecutorFee` (0xf4ad17c6) function
        pub fn get_min_executor_fee(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([244, 173, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPeerMinRollupFee` (0x825b5f8d) function
        pub fn get_peer_min_rollup_fee(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([130, 91, 95, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDepositsDisabled` (0xed6ea33a) function
        pub fn is_deposits_disabled(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([237, 110, 163, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPeerContractSet` (0xfa750f56) function
        pub fn is_peer_contract_set(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 117, 15, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerChainId` (0xcdfceeba) function
        pub fn peer_chain_id(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([205, 252, 238, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerChainName` (0x4e3c10b7) function
        pub fn peer_chain_name(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([78, 60, 16, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerContract` (0x21e32d55) function
        pub fn peer_contract(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([33, 227, 45, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPeerContract` (0x422e0028) function
        pub fn set_peer_contract(
            &self,
            peer_contract: PeerContract,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 46, 0, 40], (peer_contract,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settings` (0xe06174e4) function
        pub fn settings(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([224, 97, 116, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CommitmentCrossChain` event
        pub fn commitment_cross_chain_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CommitmentCrossChainFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CommitmentCrossChainFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for MystikoV2CelerMain<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AmountLessThanZero` with signature `AmountLessThanZero()` and selector `0x820bf1e5`
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
    #[etherror(name = "AmountLessThanZero", abi = "AmountLessThanZero()")]
    pub struct AmountLessThanZero;
    ///Custom Error type `AmountTooLarge` with signature `AmountTooLarge()` and selector `0x06250401`
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
    #[etherror(name = "AmountTooLarge", abi = "AmountTooLarge()")]
    pub struct AmountTooLarge;
    ///Custom Error type `AmountTooSmall` with signature `AmountTooSmall()` and selector `0xc2f5625a`
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
    #[etherror(name = "AmountTooSmall", abi = "AmountTooSmall()")]
    pub struct AmountTooSmall;
    ///Custom Error type `AssociatedPoolNotSet` with signature `AssociatedPoolNotSet()` and selector `0xde7ac660`
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
    #[etherror(name = "AssociatedPoolNotSet", abi = "AssociatedPoolNotSet()")]
    pub struct AssociatedPoolNotSet;
    ///Custom Error type `BridgeFeeTooFew` with signature `BridgeFeeTooFew()` and selector `0xc4d8d00d`
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
    #[etherror(name = "BridgeFeeTooFew", abi = "BridgeFeeTooFew()")]
    pub struct BridgeFeeTooFew;
    ///Custom Error type `CertificateInvalid` with signature `CertificateInvalid()` and selector `0xc108107c`
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
    #[etherror(name = "CertificateInvalid", abi = "CertificateInvalid()")]
    pub struct CertificateInvalid;
    ///Custom Error type `CommitmentHashIncorrect` with signature `CommitmentHashIncorrect()` and selector `0x37f544a0`
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
    #[etherror(name = "CommitmentHashIncorrect", abi = "CommitmentHashIncorrect()")]
    pub struct CommitmentHashIncorrect;
    ///Custom Error type `DepositsDisabled` with signature `DepositsDisabled()` and selector `0x717a1648`
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
    #[etherror(name = "DepositsDisabled", abi = "DepositsDisabled()")]
    pub struct DepositsDisabled;
    ///Custom Error type `ExecutorFeeTooFew` with signature `ExecutorFeeTooFew()` and selector `0xab4dad42`
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
    #[etherror(name = "ExecutorFeeTooFew", abi = "ExecutorFeeTooFew()")]
    pub struct ExecutorFeeTooFew;
    ///Custom Error type `HashKGreaterThanFieldSize` with signature `HashKGreaterThanFieldSize()` and selector `0x805f2a49`
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
    #[etherror(name = "HashKGreaterThanFieldSize", abi = "HashKGreaterThanFieldSize()")]
    pub struct HashKGreaterThanFieldSize;
    ///Custom Error type `NotSupport` with signature `NotSupport()` and selector `0xe7a24ff9`
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
    #[etherror(name = "NotSupport", abi = "NotSupport()")]
    pub struct NotSupport;
    ///Custom Error type `PeerChainIdNotMatched` with signature `PeerChainIdNotMatched()` and selector `0x6e778242`
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
    #[etherror(name = "PeerChainIdNotMatched", abi = "PeerChainIdNotMatched()")]
    pub struct PeerChainIdNotMatched;
    ///Custom Error type `PeerContractAlreadySet` with signature `PeerContractAlreadySet()` and selector `0xdb1e22a2`
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
    #[etherror(name = "PeerContractAlreadySet", abi = "PeerContractAlreadySet()")]
    pub struct PeerContractAlreadySet;
    ///Custom Error type `PeerContractNotMatched` with signature `PeerContractNotMatched()` and selector `0xda4af678`
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
    #[etherror(name = "PeerContractNotMatched", abi = "PeerContractNotMatched()")]
    pub struct PeerContractNotMatched;
    ///Custom Error type `RandomSGreaterThanFieldSize` with signature `RandomSGreaterThanFieldSize()` and selector `0xeef782fc`
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
    #[etherror(
        name = "RandomSGreaterThanFieldSize",
        abi = "RandomSGreaterThanFieldSize()"
    )]
    pub struct RandomSGreaterThanFieldSize;
    ///Custom Error type `RollupFeeToFew` with signature `RollupFeeToFew()` and selector `0xf09e057a`
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
    #[etherror(name = "RollupFeeToFew", abi = "RollupFeeToFew()")]
    pub struct RollupFeeToFew;
    ///Custom Error type `SanctionedAddress` with signature `SanctionedAddress()` and selector `0x2e70c0b1`
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
    #[etherror(name = "SanctionedAddress", abi = "SanctionedAddress()")]
    pub struct SanctionedAddress;
    ///Custom Error type `SenderIsNotBridgeProxy` with signature `SenderIsNotBridgeProxy()` and selector `0x7b94039e`
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
    #[etherror(name = "SenderIsNotBridgeProxy", abi = "SenderIsNotBridgeProxy()")]
    pub struct SenderIsNotBridgeProxy;
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
    pub enum MystikoV2CelerMainErrors {
        AmountLessThanZero(AmountLessThanZero),
        AmountTooLarge(AmountTooLarge),
        AmountTooSmall(AmountTooSmall),
        AssociatedPoolNotSet(AssociatedPoolNotSet),
        BridgeFeeTooFew(BridgeFeeTooFew),
        CertificateInvalid(CertificateInvalid),
        CommitmentHashIncorrect(CommitmentHashIncorrect),
        DepositsDisabled(DepositsDisabled),
        ExecutorFeeTooFew(ExecutorFeeTooFew),
        HashKGreaterThanFieldSize(HashKGreaterThanFieldSize),
        NotSupport(NotSupport),
        PeerChainIdNotMatched(PeerChainIdNotMatched),
        PeerContractAlreadySet(PeerContractAlreadySet),
        PeerContractNotMatched(PeerContractNotMatched),
        RandomSGreaterThanFieldSize(RandomSGreaterThanFieldSize),
        RollupFeeToFew(RollupFeeToFew),
        SanctionedAddress(SanctionedAddress),
        SenderIsNotBridgeProxy(SenderIsNotBridgeProxy),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoV2CelerMainErrors {
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
                = <AmountLessThanZero as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountLessThanZero(decoded));
            }
            if let Ok(decoded)
                = <AmountTooLarge as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountTooLarge(decoded));
            }
            if let Ok(decoded)
                = <AmountTooSmall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountTooSmall(decoded));
            }
            if let Ok(decoded)
                = <AssociatedPoolNotSet as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AssociatedPoolNotSet(decoded));
            }
            if let Ok(decoded)
                = <BridgeFeeTooFew as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BridgeFeeTooFew(decoded));
            }
            if let Ok(decoded)
                = <CertificateInvalid as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CertificateInvalid(decoded));
            }
            if let Ok(decoded)
                = <CommitmentHashIncorrect as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CommitmentHashIncorrect(decoded));
            }
            if let Ok(decoded)
                = <DepositsDisabled as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositsDisabled(decoded));
            }
            if let Ok(decoded)
                = <ExecutorFeeTooFew as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecutorFeeTooFew(decoded));
            }
            if let Ok(decoded)
                = <HashKGreaterThanFieldSize as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HashKGreaterThanFieldSize(decoded));
            }
            if let Ok(decoded)
                = <NotSupport as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotSupport(decoded));
            }
            if let Ok(decoded)
                = <PeerChainIdNotMatched as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PeerChainIdNotMatched(decoded));
            }
            if let Ok(decoded)
                = <PeerContractAlreadySet as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PeerContractAlreadySet(decoded));
            }
            if let Ok(decoded)
                = <PeerContractNotMatched as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PeerContractNotMatched(decoded));
            }
            if let Ok(decoded)
                = <RandomSGreaterThanFieldSize as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RandomSGreaterThanFieldSize(decoded));
            }
            if let Ok(decoded)
                = <RollupFeeToFew as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RollupFeeToFew(decoded));
            }
            if let Ok(decoded)
                = <SanctionedAddress as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionedAddress(decoded));
            }
            if let Ok(decoded)
                = <SenderIsNotBridgeProxy as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SenderIsNotBridgeProxy(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for MystikoV2CelerMainErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AmountLessThanZero(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AmountTooLarge(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AmountTooSmall(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AssociatedPoolNotSet(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::BridgeFeeTooFew(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CertificateInvalid(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CommitmentHashIncorrect(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::DepositsDisabled(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ExecutorFeeTooFew(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::HashKGreaterThanFieldSize(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::NotSupport(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PeerChainIdNotMatched(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PeerContractAlreadySet(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PeerContractNotMatched(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RandomSGreaterThanFieldSize(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RollupFeeToFew(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SanctionedAddress(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SenderIsNotBridgeProxy(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for MystikoV2CelerMainErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AmountLessThanZero as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AmountTooLarge as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AmountTooSmall as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AssociatedPoolNotSet as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BridgeFeeTooFew as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CertificateInvalid as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CommitmentHashIncorrect as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DepositsDisabled as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExecutorFeeTooFew as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <HashKGreaterThanFieldSize as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotSupport as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <PeerChainIdNotMatched as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PeerContractAlreadySet as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PeerContractNotMatched as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RandomSGreaterThanFieldSize as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RollupFeeToFew as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SanctionedAddress as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SenderIsNotBridgeProxy as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2CelerMainErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AmountLessThanZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AmountTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssociatedPoolNotSet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BridgeFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::CertificateInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CommitmentHashIncorrect(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutorFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashKGreaterThanFieldSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotSupport(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerChainIdNotMatched(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PeerContractAlreadySet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PeerContractNotMatched(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RandomSGreaterThanFieldSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RollupFeeToFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionedAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SenderIsNotBridgeProxy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MystikoV2CelerMainErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AmountLessThanZero> for MystikoV2CelerMainErrors {
        fn from(value: AmountLessThanZero) -> Self {
            Self::AmountLessThanZero(value)
        }
    }
    impl ::core::convert::From<AmountTooLarge> for MystikoV2CelerMainErrors {
        fn from(value: AmountTooLarge) -> Self {
            Self::AmountTooLarge(value)
        }
    }
    impl ::core::convert::From<AmountTooSmall> for MystikoV2CelerMainErrors {
        fn from(value: AmountTooSmall) -> Self {
            Self::AmountTooSmall(value)
        }
    }
    impl ::core::convert::From<AssociatedPoolNotSet> for MystikoV2CelerMainErrors {
        fn from(value: AssociatedPoolNotSet) -> Self {
            Self::AssociatedPoolNotSet(value)
        }
    }
    impl ::core::convert::From<BridgeFeeTooFew> for MystikoV2CelerMainErrors {
        fn from(value: BridgeFeeTooFew) -> Self {
            Self::BridgeFeeTooFew(value)
        }
    }
    impl ::core::convert::From<CertificateInvalid> for MystikoV2CelerMainErrors {
        fn from(value: CertificateInvalid) -> Self {
            Self::CertificateInvalid(value)
        }
    }
    impl ::core::convert::From<CommitmentHashIncorrect> for MystikoV2CelerMainErrors {
        fn from(value: CommitmentHashIncorrect) -> Self {
            Self::CommitmentHashIncorrect(value)
        }
    }
    impl ::core::convert::From<DepositsDisabled> for MystikoV2CelerMainErrors {
        fn from(value: DepositsDisabled) -> Self {
            Self::DepositsDisabled(value)
        }
    }
    impl ::core::convert::From<ExecutorFeeTooFew> for MystikoV2CelerMainErrors {
        fn from(value: ExecutorFeeTooFew) -> Self {
            Self::ExecutorFeeTooFew(value)
        }
    }
    impl ::core::convert::From<HashKGreaterThanFieldSize> for MystikoV2CelerMainErrors {
        fn from(value: HashKGreaterThanFieldSize) -> Self {
            Self::HashKGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<NotSupport> for MystikoV2CelerMainErrors {
        fn from(value: NotSupport) -> Self {
            Self::NotSupport(value)
        }
    }
    impl ::core::convert::From<PeerChainIdNotMatched> for MystikoV2CelerMainErrors {
        fn from(value: PeerChainIdNotMatched) -> Self {
            Self::PeerChainIdNotMatched(value)
        }
    }
    impl ::core::convert::From<PeerContractAlreadySet> for MystikoV2CelerMainErrors {
        fn from(value: PeerContractAlreadySet) -> Self {
            Self::PeerContractAlreadySet(value)
        }
    }
    impl ::core::convert::From<PeerContractNotMatched> for MystikoV2CelerMainErrors {
        fn from(value: PeerContractNotMatched) -> Self {
            Self::PeerContractNotMatched(value)
        }
    }
    impl ::core::convert::From<RandomSGreaterThanFieldSize>
    for MystikoV2CelerMainErrors {
        fn from(value: RandomSGreaterThanFieldSize) -> Self {
            Self::RandomSGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<RollupFeeToFew> for MystikoV2CelerMainErrors {
        fn from(value: RollupFeeToFew) -> Self {
            Self::RollupFeeToFew(value)
        }
    }
    impl ::core::convert::From<SanctionedAddress> for MystikoV2CelerMainErrors {
        fn from(value: SanctionedAddress) -> Self {
            Self::SanctionedAddress(value)
        }
    }
    impl ::core::convert::From<SenderIsNotBridgeProxy> for MystikoV2CelerMainErrors {
        fn from(value: SenderIsNotBridgeProxy) -> Self {
            Self::SenderIsNotBridgeProxy(value)
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
    #[ethevent(name = "CommitmentCrossChain", abi = "CommitmentCrossChain(uint256)")]
    pub struct CommitmentCrossChainFilter {
        #[ethevent(indexed)]
        pub commitment: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `assetAddress` function with signature `assetAddress()` and selector `0x1ba46cfd`
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
    #[ethcall(name = "assetAddress", abi = "assetAddress()")]
    pub struct AssetAddressCall;
    ///Container type for all input parameters for the `assetType` function with signature `assetType()` and selector `0x3fe3347a`
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
    #[ethcall(name = "assetType", abi = "assetType()")]
    pub struct AssetTypeCall;
    ///Container type for all input parameters for the `bridgeProxyAddress` function with signature `bridgeProxyAddress()` and selector `0x2cd26d45`
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
    #[ethcall(name = "bridgeProxyAddress", abi = "bridgeProxyAddress()")]
    pub struct BridgeProxyAddressCall;
    ///Container type for all input parameters for the `bridgeType` function with signature `bridgeType()` and selector `0x2421e155`
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
    #[ethcall(name = "bridgeType", abi = "bridgeType()")]
    pub struct BridgeTypeCall;
    ///Container type for all input parameters for the `certDeposit` function with signature `certDeposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256),uint256,bytes)` and selector `0xcb5c029a`
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
        name = "certDeposit",
        abi = "certDeposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256),uint256,bytes)"
    )]
    pub struct CertDepositCall {
        pub request: DepositRequest,
        pub certificate_deadline: ::ethers_core::types::U256,
        pub certificate_signature: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `defaultMaxAmount` function with signature `defaultMaxAmount()` and selector `0xd0b436bd`
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
    #[ethcall(name = "defaultMaxAmount", abi = "defaultMaxAmount()")]
    pub struct DefaultMaxAmountCall;
    ///Container type for all input parameters for the `defaultMinAmount` function with signature `defaultMinAmount()` and selector `0xfb3e3d73`
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
    #[ethcall(name = "defaultMinAmount", abi = "defaultMinAmount()")]
    pub struct DefaultMinAmountCall;
    ///Container type for all input parameters for the `defaultMinBridgeFee` function with signature `defaultMinBridgeFee()` and selector `0x4dde6fbc`
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
    #[ethcall(name = "defaultMinBridgeFee", abi = "defaultMinBridgeFee()")]
    pub struct DefaultMinBridgeFeeCall;
    ///Container type for all input parameters for the `defaultPeerMinExecutorFee` function with signature `defaultPeerMinExecutorFee()` and selector `0x640c0b36`
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
    #[ethcall(name = "defaultPeerMinExecutorFee", abi = "defaultPeerMinExecutorFee()")]
    pub struct DefaultPeerMinExecutorFeeCall;
    ///Container type for all input parameters for the `defaultPeerMinRollupFee` function with signature `defaultPeerMinRollupFee()` and selector `0xcbe34285`
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
    #[ethcall(name = "defaultPeerMinRollupFee", abi = "defaultPeerMinRollupFee()")]
    pub struct DefaultPeerMinRollupFeeCall;
    ///Container type for all input parameters for the `deposit` function with signature `deposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256))` and selector `0x9a03636c`
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
        name = "deposit",
        abi = "deposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256))"
    )]
    pub struct DepositCall {
        pub request: DepositRequest,
    }
    ///Container type for all input parameters for the `executeMessage` function with signature `executeMessage(address,uint64,bytes,address)` and selector `0x9c649fdf`
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
        name = "executeMessage",
        abi = "executeMessage(address,uint64,bytes,address)"
    )]
    pub struct ExecuteMessageCall {
        pub sender: ::ethers_core::types::Address,
        pub src_chain_id: u64,
        pub message: ::ethers_core::types::Bytes,
        pub executor: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `getAssociatedCommitmentPool` function with signature `getAssociatedCommitmentPool()` and selector `0xddac5dc1`
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
        name = "getAssociatedCommitmentPool",
        abi = "getAssociatedCommitmentPool()"
    )]
    pub struct GetAssociatedCommitmentPoolCall;
    ///Container type for all input parameters for the `getMaxAmount` function with signature `getMaxAmount()` and selector `0x0ba95909`
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
    #[ethcall(name = "getMaxAmount", abi = "getMaxAmount()")]
    pub struct GetMaxAmountCall;
    ///Container type for all input parameters for the `getMinAmount` function with signature `getMinAmount()` and selector `0xcfc7e2da`
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
    #[ethcall(name = "getMinAmount", abi = "getMinAmount()")]
    pub struct GetMinAmountCall;
    ///Container type for all input parameters for the `getMinBridgeFee` function with signature `getMinBridgeFee()` and selector `0xefbfb2ae`
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
    #[ethcall(name = "getMinBridgeFee", abi = "getMinBridgeFee()")]
    pub struct GetMinBridgeFeeCall;
    ///Container type for all input parameters for the `getMinExecutorFee` function with signature `getMinExecutorFee()` and selector `0xf4ad17c6`
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
    #[ethcall(name = "getMinExecutorFee", abi = "getMinExecutorFee()")]
    pub struct GetMinExecutorFeeCall;
    ///Container type for all input parameters for the `getPeerMinRollupFee` function with signature `getPeerMinRollupFee()` and selector `0x825b5f8d`
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
    #[ethcall(name = "getPeerMinRollupFee", abi = "getPeerMinRollupFee()")]
    pub struct GetPeerMinRollupFeeCall;
    ///Container type for all input parameters for the `isDepositsDisabled` function with signature `isDepositsDisabled()` and selector `0xed6ea33a`
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
    #[ethcall(name = "isDepositsDisabled", abi = "isDepositsDisabled()")]
    pub struct IsDepositsDisabledCall;
    ///Container type for all input parameters for the `isPeerContractSet` function with signature `isPeerContractSet()` and selector `0xfa750f56`
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
    #[ethcall(name = "isPeerContractSet", abi = "isPeerContractSet()")]
    pub struct IsPeerContractSetCall;
    ///Container type for all input parameters for the `peerChainId` function with signature `peerChainId()` and selector `0xcdfceeba`
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
    #[ethcall(name = "peerChainId", abi = "peerChainId()")]
    pub struct PeerChainIdCall;
    ///Container type for all input parameters for the `peerChainName` function with signature `peerChainName()` and selector `0x4e3c10b7`
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
    #[ethcall(name = "peerChainName", abi = "peerChainName()")]
    pub struct PeerChainNameCall;
    ///Container type for all input parameters for the `peerContract` function with signature `peerContract()` and selector `0x21e32d55`
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
    #[ethcall(name = "peerContract", abi = "peerContract()")]
    pub struct PeerContractCall;
    ///Container type for all input parameters for the `setPeerContract` function with signature `setPeerContract((uint64,string,address))` and selector `0x422e0028`
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
        name = "setPeerContract",
        abi = "setPeerContract((uint64,string,address))"
    )]
    pub struct SetPeerContractCall {
        pub peer_contract: PeerContract,
    }
    ///Container type for all input parameters for the `settings` function with signature `settings()` and selector `0xe06174e4`
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
    #[ethcall(name = "settings", abi = "settings()")]
    pub struct SettingsCall;
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
    pub enum MystikoV2CelerMainCalls {
        AssetAddress(AssetAddressCall),
        AssetType(AssetTypeCall),
        BridgeProxyAddress(BridgeProxyAddressCall),
        BridgeType(BridgeTypeCall),
        CertDeposit(CertDepositCall),
        DefaultMaxAmount(DefaultMaxAmountCall),
        DefaultMinAmount(DefaultMinAmountCall),
        DefaultMinBridgeFee(DefaultMinBridgeFeeCall),
        DefaultPeerMinExecutorFee(DefaultPeerMinExecutorFeeCall),
        DefaultPeerMinRollupFee(DefaultPeerMinRollupFeeCall),
        Deposit(DepositCall),
        ExecuteMessage(ExecuteMessageCall),
        GetAssociatedCommitmentPool(GetAssociatedCommitmentPoolCall),
        GetMaxAmount(GetMaxAmountCall),
        GetMinAmount(GetMinAmountCall),
        GetMinBridgeFee(GetMinBridgeFeeCall),
        GetMinExecutorFee(GetMinExecutorFeeCall),
        GetPeerMinRollupFee(GetPeerMinRollupFeeCall),
        IsDepositsDisabled(IsDepositsDisabledCall),
        IsPeerContractSet(IsPeerContractSetCall),
        PeerChainId(PeerChainIdCall),
        PeerChainName(PeerChainNameCall),
        PeerContract(PeerContractCall),
        SetPeerContract(SetPeerContractCall),
        Settings(SettingsCall),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoV2CelerMainCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AssetAddressCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetAddress(decoded));
            }
            if let Ok(decoded)
                = <AssetTypeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetType(decoded));
            }
            if let Ok(decoded)
                = <BridgeProxyAddressCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BridgeProxyAddress(decoded));
            }
            if let Ok(decoded)
                = <BridgeTypeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BridgeType(decoded));
            }
            if let Ok(decoded)
                = <CertDepositCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CertDeposit(decoded));
            }
            if let Ok(decoded)
                = <DefaultMaxAmountCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultMaxAmount(decoded));
            }
            if let Ok(decoded)
                = <DefaultMinAmountCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultMinAmount(decoded));
            }
            if let Ok(decoded)
                = <DefaultMinBridgeFeeCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultMinBridgeFee(decoded));
            }
            if let Ok(decoded)
                = <DefaultPeerMinExecutorFeeCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultPeerMinExecutorFee(decoded));
            }
            if let Ok(decoded)
                = <DefaultPeerMinRollupFeeCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultPeerMinRollupFee(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <ExecuteMessageCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteMessage(decoded));
            }
            if let Ok(decoded)
                = <GetAssociatedCommitmentPoolCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAssociatedCommitmentPool(decoded));
            }
            if let Ok(decoded)
                = <GetMaxAmountCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMaxAmount(decoded));
            }
            if let Ok(decoded)
                = <GetMinAmountCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinAmount(decoded));
            }
            if let Ok(decoded)
                = <GetMinBridgeFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinBridgeFee(decoded));
            }
            if let Ok(decoded)
                = <GetMinExecutorFeeCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetMinExecutorFee(decoded));
            }
            if let Ok(decoded)
                = <GetPeerMinRollupFeeCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPeerMinRollupFee(decoded));
            }
            if let Ok(decoded)
                = <IsDepositsDisabledCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsDepositsDisabled(decoded));
            }
            if let Ok(decoded)
                = <IsPeerContractSetCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsPeerContractSet(decoded));
            }
            if let Ok(decoded)
                = <PeerChainIdCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainId(decoded));
            }
            if let Ok(decoded)
                = <PeerChainNameCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainName(decoded));
            }
            if let Ok(decoded)
                = <PeerContractCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerContract(decoded));
            }
            if let Ok(decoded)
                = <SetPeerContractCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPeerContract(decoded));
            }
            if let Ok(decoded)
                = <SettingsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Settings(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for MystikoV2CelerMainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetAddress(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AssetType(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::BridgeProxyAddress(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::BridgeType(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CertDeposit(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::DefaultMaxAmount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::DefaultMinAmount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::DefaultMinBridgeFee(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::DefaultPeerMinExecutorFee(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::DefaultPeerMinRollupFee(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ExecuteMessage(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetAssociatedCommitmentPool(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxAmount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetMinAmount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetMinBridgeFee(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetMinExecutorFee(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetPeerMinRollupFee(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsDepositsDisabled(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsPeerContractSet(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PeerChainId(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PeerChainName(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PeerContract(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetPeerContract(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Settings(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2CelerMainCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetType(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeProxyAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BridgeType(element) => ::core::fmt::Display::fmt(element, f),
                Self::CertDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultMinAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultMinBridgeFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultPeerMinExecutorFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultPeerMinRollupFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssociatedCommitmentPool(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinBridgeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPeerMinRollupFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsDepositsDisabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsPeerContractSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerChainName(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPeerContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::Settings(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssetAddressCall> for MystikoV2CelerMainCalls {
        fn from(value: AssetAddressCall) -> Self {
            Self::AssetAddress(value)
        }
    }
    impl ::core::convert::From<AssetTypeCall> for MystikoV2CelerMainCalls {
        fn from(value: AssetTypeCall) -> Self {
            Self::AssetType(value)
        }
    }
    impl ::core::convert::From<BridgeProxyAddressCall> for MystikoV2CelerMainCalls {
        fn from(value: BridgeProxyAddressCall) -> Self {
            Self::BridgeProxyAddress(value)
        }
    }
    impl ::core::convert::From<BridgeTypeCall> for MystikoV2CelerMainCalls {
        fn from(value: BridgeTypeCall) -> Self {
            Self::BridgeType(value)
        }
    }
    impl ::core::convert::From<CertDepositCall> for MystikoV2CelerMainCalls {
        fn from(value: CertDepositCall) -> Self {
            Self::CertDeposit(value)
        }
    }
    impl ::core::convert::From<DefaultMaxAmountCall> for MystikoV2CelerMainCalls {
        fn from(value: DefaultMaxAmountCall) -> Self {
            Self::DefaultMaxAmount(value)
        }
    }
    impl ::core::convert::From<DefaultMinAmountCall> for MystikoV2CelerMainCalls {
        fn from(value: DefaultMinAmountCall) -> Self {
            Self::DefaultMinAmount(value)
        }
    }
    impl ::core::convert::From<DefaultMinBridgeFeeCall> for MystikoV2CelerMainCalls {
        fn from(value: DefaultMinBridgeFeeCall) -> Self {
            Self::DefaultMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<DefaultPeerMinExecutorFeeCall>
    for MystikoV2CelerMainCalls {
        fn from(value: DefaultPeerMinExecutorFeeCall) -> Self {
            Self::DefaultPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<DefaultPeerMinRollupFeeCall> for MystikoV2CelerMainCalls {
        fn from(value: DefaultPeerMinRollupFeeCall) -> Self {
            Self::DefaultPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MystikoV2CelerMainCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<ExecuteMessageCall> for MystikoV2CelerMainCalls {
        fn from(value: ExecuteMessageCall) -> Self {
            Self::ExecuteMessage(value)
        }
    }
    impl ::core::convert::From<GetAssociatedCommitmentPoolCall>
    for MystikoV2CelerMainCalls {
        fn from(value: GetAssociatedCommitmentPoolCall) -> Self {
            Self::GetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<GetMaxAmountCall> for MystikoV2CelerMainCalls {
        fn from(value: GetMaxAmountCall) -> Self {
            Self::GetMaxAmount(value)
        }
    }
    impl ::core::convert::From<GetMinAmountCall> for MystikoV2CelerMainCalls {
        fn from(value: GetMinAmountCall) -> Self {
            Self::GetMinAmount(value)
        }
    }
    impl ::core::convert::From<GetMinBridgeFeeCall> for MystikoV2CelerMainCalls {
        fn from(value: GetMinBridgeFeeCall) -> Self {
            Self::GetMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<GetMinExecutorFeeCall> for MystikoV2CelerMainCalls {
        fn from(value: GetMinExecutorFeeCall) -> Self {
            Self::GetMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<GetPeerMinRollupFeeCall> for MystikoV2CelerMainCalls {
        fn from(value: GetPeerMinRollupFeeCall) -> Self {
            Self::GetPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<IsDepositsDisabledCall> for MystikoV2CelerMainCalls {
        fn from(value: IsDepositsDisabledCall) -> Self {
            Self::IsDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<IsPeerContractSetCall> for MystikoV2CelerMainCalls {
        fn from(value: IsPeerContractSetCall) -> Self {
            Self::IsPeerContractSet(value)
        }
    }
    impl ::core::convert::From<PeerChainIdCall> for MystikoV2CelerMainCalls {
        fn from(value: PeerChainIdCall) -> Self {
            Self::PeerChainId(value)
        }
    }
    impl ::core::convert::From<PeerChainNameCall> for MystikoV2CelerMainCalls {
        fn from(value: PeerChainNameCall) -> Self {
            Self::PeerChainName(value)
        }
    }
    impl ::core::convert::From<PeerContractCall> for MystikoV2CelerMainCalls {
        fn from(value: PeerContractCall) -> Self {
            Self::PeerContract(value)
        }
    }
    impl ::core::convert::From<SetPeerContractCall> for MystikoV2CelerMainCalls {
        fn from(value: SetPeerContractCall) -> Self {
            Self::SetPeerContract(value)
        }
    }
    impl ::core::convert::From<SettingsCall> for MystikoV2CelerMainCalls {
        fn from(value: SettingsCall) -> Self {
            Self::Settings(value)
        }
    }
    ///Container type for all return fields from the `assetAddress` function with signature `assetAddress()` and selector `0x1ba46cfd`
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
    pub struct AssetAddressReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `assetType` function with signature `assetType()` and selector `0x3fe3347a`
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
    pub struct AssetTypeReturn(pub u8);
    ///Container type for all return fields from the `bridgeProxyAddress` function with signature `bridgeProxyAddress()` and selector `0x2cd26d45`
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
    pub struct BridgeProxyAddressReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `bridgeType` function with signature `bridgeType()` and selector `0x2421e155`
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
    pub struct BridgeTypeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `defaultMaxAmount` function with signature `defaultMaxAmount()` and selector `0xd0b436bd`
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
    pub struct DefaultMaxAmountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `defaultMinAmount` function with signature `defaultMinAmount()` and selector `0xfb3e3d73`
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
    pub struct DefaultMinAmountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `defaultMinBridgeFee` function with signature `defaultMinBridgeFee()` and selector `0x4dde6fbc`
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
    pub struct DefaultMinBridgeFeeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `defaultPeerMinExecutorFee` function with signature `defaultPeerMinExecutorFee()` and selector `0x640c0b36`
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
    pub struct DefaultPeerMinExecutorFeeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `defaultPeerMinRollupFee` function with signature `defaultPeerMinRollupFee()` and selector `0xcbe34285`
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
    pub struct DefaultPeerMinRollupFeeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `executeMessage` function with signature `executeMessage(address,uint64,bytes,address)` and selector `0x9c649fdf`
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
    pub struct ExecuteMessageReturn(pub bool);
    ///Container type for all return fields from the `getAssociatedCommitmentPool` function with signature `getAssociatedCommitmentPool()` and selector `0xddac5dc1`
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
    pub struct GetAssociatedCommitmentPoolReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `getMaxAmount` function with signature `getMaxAmount()` and selector `0x0ba95909`
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
    pub struct GetMaxAmountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getMinAmount` function with signature `getMinAmount()` and selector `0xcfc7e2da`
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
    pub struct GetMinAmountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getMinBridgeFee` function with signature `getMinBridgeFee()` and selector `0xefbfb2ae`
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
    pub struct GetMinBridgeFeeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getMinExecutorFee` function with signature `getMinExecutorFee()` and selector `0xf4ad17c6`
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
    pub struct GetMinExecutorFeeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getPeerMinRollupFee` function with signature `getPeerMinRollupFee()` and selector `0x825b5f8d`
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
    pub struct GetPeerMinRollupFeeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `isDepositsDisabled` function with signature `isDepositsDisabled()` and selector `0xed6ea33a`
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
    pub struct IsDepositsDisabledReturn(pub bool);
    ///Container type for all return fields from the `isPeerContractSet` function with signature `isPeerContractSet()` and selector `0xfa750f56`
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
    pub struct IsPeerContractSetReturn(pub bool);
    ///Container type for all return fields from the `peerChainId` function with signature `peerChainId()` and selector `0xcdfceeba`
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
    pub struct PeerChainIdReturn(pub u64);
    ///Container type for all return fields from the `peerChainName` function with signature `peerChainName()` and selector `0x4e3c10b7`
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
    pub struct PeerChainNameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `peerContract` function with signature `peerContract()` and selector `0x21e32d55`
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
    pub struct PeerContractReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `settings` function with signature `settings()` and selector `0xe06174e4`
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
    pub struct SettingsReturn(pub ::ethers_core::types::Address);
    ///`DepositRequest(uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256)`
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
    pub struct DepositRequest {
        pub amount: ::ethers_core::types::U256,
        pub commitment: ::ethers_core::types::U256,
        pub hash_k: ::ethers_core::types::U256,
        pub random_s: u128,
        pub encrypted_note: ::ethers_core::types::Bytes,
        pub bridge_fee: ::ethers_core::types::U256,
        pub executor_fee: ::ethers_core::types::U256,
        pub rollup_fee: ::ethers_core::types::U256,
    }
    ///`PeerContract(uint64,string,address)`
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
    pub struct PeerContract {
        pub peer_chain_id: u64,
        pub peer_chain_name: ::std::string::String,
        pub peer_contract: ::ethers_core::types::Address,
    }
}
