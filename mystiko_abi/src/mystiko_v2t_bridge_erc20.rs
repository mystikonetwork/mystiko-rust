pub use mystiko_v2t_bridge_erc20::*;
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
pub mod mystiko_v2t_bridge_erc20 {
    const _: () = {
        ::core::include_bytes!(
"../json/MystikoV2TBridgeERC20.json",
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
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IERC20Metadata"),
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
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assetDecimals"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetDecimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assetName"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetName"),
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
                    ::std::borrow::ToOwned::to_owned("assetSymbol"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetSymbol"),
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
                    ::std::borrow::ToOwned::to_owned("crossChainSyncTx"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("crossChainSyncTx"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fromChainId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fromContract"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressInsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
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
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedOperation",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
    pub static MYSTIKOV2TBRIDGEERC20_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90U4\x80\x15a\0\x1DW`\0\x80\xFD[P`@Qa+\x058\x03\x80a+\x05\x839\x81\x01`@\x81\x90Ra\0<\x91a\x01XV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x98\x89\x16\x17\x90\x91U`\x08\x80T\x82\x16\x95\x88\x16\x95\x90\x95\x17\x90\x94U\x81Q`\x03U` \x80\x83\x01Q`\x04U`@\x90\x92\x01Q`\x05U\x80Q`\x06U\x01Q`\x07U`\t\x80T\x83\x16\x91\x85\x16\x91\x90\x91\x17\x90U`\n\x80T\x90\x91\x16\x91\x90\x92\x16\x17\x90Ua\x02\x04V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xC3W`\0\x80\xFD[PV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xF6WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0`@\x82\x84\x03\x12\x15a\x01\x0EW`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01>WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x01 \x81\x12\x15a\x01sW`\0\x80\xFD[\x87Qa\x01~\x81a\0\xAEV[` \x89\x01Q\x90\x97Pa\x01\x8F\x81a\0\xAEV[`@\x89\x01Q\x90\x96Pa\x01\xA0\x81a\0\xAEV[``\x89\x01Q\x90\x95Pa\x01\xB1\x81a\0\xAEV[\x93P```\x7F\x19\x82\x01\x12\x15a\x01\xC5W`\0\x80\xFD[Pa\x01\xCEa\0\xC6V[`\x80\x88\x01Q\x81R`\xA0\x88\x01Q` \x82\x01R`\xC0\x88\x01Q`@\x82\x01R\x91Pa\x01\xF8\x88`\xE0\x89\x01a\0\xFCV[\x90P\x92\x95P\x92\x95P\x92\x95V[a(\xF2\x80a\x02\x13`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xB7W`\x005`\xE0\x1C\x80c\xC2\xD4\x16\x01\x11a\0\xECW\x80c\xDD\xAC]\xC1\x11a\0\x8AW\x80c\xEF\xBF\xB2\xAE\x11a\0dW\x80c\xEF\xBF\xB2\xAE\x14a\x04\xB0W\x80c\xF4\xAD\x17\xC6\x14a\x04\xC5W\x80c\xFAu\x0FV\x14a\x04\xDAW\x80c\xFB>=s\x14a\x04\xFBW`\0\x80\xFD[\x80c\xDD\xAC]\xC1\x14a\x04fW\x80c\xE0at\xE4\x14a\x04{W\x80c\xEDn\xA3:\x14a\x04\x9BW`\0\x80\xFD[\x80c\xCB\xE3B\x85\x11a\0\xC6W\x80c\xCB\xE3B\x85\x14a\x03\xE4W\x80c\xCD\xFC\xEE\xBA\x14a\x03\xFAW\x80c\xCF\xC7\xE2\xDA\x14a\x04;W\x80c\xD0\xB46\xBD\x14a\x04PW`\0\x80\xFD[\x80c\xC2\xD4\x16\x01\x14a\x03\x95W\x80c\xC9#\x0C]\x14a\x03\xBCW\x80c\xCB\\\x02\x9A\x14a\x03\xD1W`\0\x80\xFD[\x80cB.\0(\x11a\x01YW\x80cd\x0C\x0B6\x11a\x013W\x80cd\x0C\x0B6\x14a\x03'W\x80c\x82[_\x8D\x14a\x03=W\x80c\x82\xD2\x1C\xD8\x14a\x03RW\x80c\x9A\x03cl\x14a\x03\x82W`\0\x80\xFD[\x80cB.\0(\x14a\x02\xDAW\x80cM\xDEo\xBC\x14a\x02\xFCW\x80cN<\x10\xB7\x14a\x03\x12W`\0\x80\xFD[\x80c!\xE3-U\x11a\x01\x95W\x80c!\xE3-U\x14a\x028W\x80c$!\xE1U\x14a\x02XW\x80c,\xD2mE\x14a\x02\x9EW\x80c?\xE34z\x14a\x02\xBEW`\0\x80\xFD[\x80c\x0B\xA9Y\t\x14a\x01\xBCW\x80c\x17m\xE7\xA8\x14a\x01\xE4W\x80c\x1B\xA4l\xFD\x14a\x02\x06W[`\0\x80\xFD[4\x80\x15a\x01\xC8W`\0\x80\xFD[Pa\x01\xD1a\x05\x11V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF0W`\0\x80\xFD[Pa\x01\xF9a\x05\x9AV[`@Qa\x01\xDB\x91\x90a ^V[4\x80\x15a\x02\x12W`\0\x80\xFD[P`\nT`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xDBV[4\x80\x15a\x02DW`\0\x80\xFD[P`\x02Ta\x02 \x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02dW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x07\x81R\x7Ftbridge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x01\xF9V[4\x80\x15a\x02\xAAW`\0\x80\xFD[P`\x08Ta\x02 \x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\xCAW`\0\x80\xFD[P`\0`@Qa\x01\xDB\x91\x90a qV[4\x80\x15a\x02\xE6W`\0\x80\xFD[Pa\x02\xFAa\x02\xF56`\x04a!\xC0V[a\x06\x11V[\0[4\x80\x15a\x03\x08W`\0\x80\xFD[Pa\x01\xD1`\x05T\x81V[4\x80\x15a\x03\x1EW`\0\x80\xFD[Pa\x01\xF9a\x06\xFEV[4\x80\x15a\x033W`\0\x80\xFD[Pa\x01\xD1`\x06T\x81V[4\x80\x15a\x03IW`\0\x80\xFD[Pa\x01\xD1a\x07\x8CV[4\x80\x15a\x03^W`\0\x80\xFD[Pa\x03ra\x03m6`\x04a\"mV[a\x08\x13V[`@Q\x90\x15\x15\x81R` \x01a\x01\xDBV[a\x02\xFAa\x03\x906`\x04a#\xF2V[a\x08\x9DV[4\x80\x15a\x03\xA1W`\0\x80\xFD[Pa\x03\xAAa\x08\xB6V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xDBV[4\x80\x15a\x03\xC8W`\0\x80\xFD[Pa\x01\xF9a\t$V[a\x02\xFAa\x03\xDF6`\x04a$/V[a\tnV[4\x80\x15a\x03\xF0W`\0\x80\xFD[Pa\x01\xD1`\x07T\x81V[4\x80\x15a\x04\x06W`\0\x80\xFD[P`\0Ta\x04\"\x90`\x01`\xA8\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xDBV[4\x80\x15a\x04GW`\0\x80\xFD[Pa\x01\xD1a\r\xA3V[4\x80\x15a\x04\\W`\0\x80\xFD[Pa\x01\xD1`\x04T\x81V[4\x80\x15a\x04rW`\0\x80\xFD[Pa\x02 a\x0E*V[4\x80\x15a\x04\x87W`\0\x80\xFD[P`\tTa\x02 \x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xA7W`\0\x80\xFD[Pa\x03ra\x0E\xC9V[4\x80\x15a\x04\xBCW`\0\x80\xFD[Pa\x01\xD1a\x0F6V[4\x80\x15a\x04\xD1W`\0\x80\xFD[Pa\x01\xD1a\x0F\xBDV[4\x80\x15a\x04\xE6W`\0\x80\xFD[P`\0Ta\x03r\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x05\x07W`\0\x80\xFD[Pa\x01\xD1`\x03T\x81V[`\tT`@QcG:\x061`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG:\x061\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x82\x91\x90a$\xA1V[\x90P\x80\x15a\x05\x90W\x80a\x05\x94V[`\x04T[\x91PP\x90V[`\nT`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06\x0C\x91\x90\x81\x01\x90a$\xBAV[\x90P\x90V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x06<W`@Qcm\x8F\x11Q`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA8\x1B\x02\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U` \x81\x01Q`\x01\x90a\x06\x90\x90\x82a%\xB9V[P`@\x01Q`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90U`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x17\x90UV[`\x01\x80Ta\x07\x0B\x90a%1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x077\x90a%1V[\x80\x15a\x07\x84W\x80`\x1F\x10a\x07YWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x84V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07gW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\tT`@Qca\x86<\x03`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC3\x0Cx\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xFD\x91\x90a$\xA1V[\x90P\x80\x15a\x08\x0BW\x80a\x05\x94V[PP`\x07T\x90V[`\x08T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08AW`@Qc=\xCA\x01\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x08\x82\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x10D\x92PPPV[\x90Pa\x08\x90\x87\x87\x85\x84a\x11\x0CV[P`\x01\x96\x95PPPPPPV[`@Qc\xE7\xA2O\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1<\xE5g\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\t\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x0C\x91\x90a&xV[`\nT`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xE4W=`\0\x80>=`\0\xFD[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xDA\x91\x90a&\x9BV[\x15a\t\xF8W`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xBCXw\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nKW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\no\x91\x90a&\x9BV[\x15a\x0BFW`\0`@Q\x80`\x80\x01`@R\x802`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\n\xA2`\nT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x86\x90R`@\x91\x82\x01\x85\x90R`\tT\x91Qc\x84\x9E\x8B\x9F`\xE0\x1B\x81R\x92\x93P\x16\x90c\x84\x9E\x8B\x9F\x90a\n\xE6\x90\x84\x90`\x04\x01a&\xBDV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B'\x91\x90a&\x9BV[a\x0BDW`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[a\x0BNa\r\xA3V[\x83Q\x10\x15a\x0BoW`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Bwa\x05\x11V[\x83Q\x11\x15a\x0B\x98W`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xA0a\x0F6V[\x83`\xA0\x01Q\x10\x15a\x0B\xC4W`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xCCa\x0F\xBDV[\x83`\xC0\x01Q\x10\x15a\x0B\xF0W`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xF8a\x07\x8CV[\x83`\xE0\x01Q\x10\x15a\x0C\x1CW`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0C5\x84`@\x01Q\x85`\0\x01Q\x86``\x01Qa\x11\xFFV[\x90P\x80\x84` \x01Q\x14a\x0C[W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT`@Qc\xDFY/}`\xE0\x1B\x81R2`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC7\x91\x90a&\x9BV[\x15a\x0C\xE5W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x90\x82\x01R`\xC0\x86\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x85\x01Q``\x82\x01R`\x80\x80\x86\x01Q\x90\x82\x01R`\0a\r(\x82a\x13\x18V[\x90Pa\r8\x86`\xA0\x01Q\x82a\x13\x87V[a\rma\rCa\x0E*V[`\xE0\x88\x01Q`\xC0\x89\x01Q\x89Qa\rY\x91\x90a'\x08V[a\rc\x91\x90a'\x08V[\x88`\xA0\x01Qa\x14\rV[` \x86\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPPPV[`\tT`@QcU%\x98I`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cU%\x98I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x14\x91\x90a$\xA1V[\x90P\x80\x15a\x0E\"W\x80a\x05\x94V[PP`\x03T\x90V[`\tT`@QcA\xFBiy`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cA\xFBiy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EwW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x9B\x91\x90a')V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\xC4W`@Qc\x06\xF3\xD63`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x0C\x91\x90a&\x9BV[`\tT`@Qc{R\xB0\r`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c{R\xB0\r\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA7\x91\x90a$\xA1V[\x90P\x80\x15a\x0F\xB5W\x80a\x05\x94V[PP`\x05T\x90V[`\tT`@QcO1O\x9D`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9Eb\x9F:\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10.\x91\x90a$\xA1V[\x90P\x80\x15a\x10<W\x80a\x05\x94V[PP`\x06T\x90V[a\x10v`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\x10\xA8`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\x10\xB4\x84\x82a\x14~V[\x90\x83R\x90Pa\x10\xC3\x84\x82a\x14~V[` \x84\x01\x91\x90\x91R\x90Pa\x10\xD7\x84\x82a\x14~V[`@\x84\x01\x91\x90\x91R\x90Pa\x10\xEB\x84\x82a\x14~V[``\x84\x01\x91\x90\x91R\x90Pa\x10\xFF\x84\x82a\x15\xAFV[P`\x80\x83\x01RP\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x11:W`@Qc\x1BI^\xCF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xA8\x1B\x90\x92\x04\x16\x14a\x11pW`@Qc7;\xC1!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x03a\x11\x92W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\x9Aa\x0E*V[`\x01`\x01`\xA0\x1B\x03\x16cx\xD6\x0C\xD7\x82\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xC7\x92\x91\x90a'FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xF5W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x12BW`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x12tW`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x12\xCC\x91`\x04\x01a'\xA3V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\r\x91\x90a$\xA1V[\x91PP[\x93\x92PPPV[``\x80a\x13(\x83`\0\x01Qa\x16\xBCV[a\x135\x84` \x01Qa\x16\xBCV[a\x13B\x85`@\x01Qa\x16\xBCV[a\x13O\x86``\x01Qa\x16\xBCV[a\x13\\\x87`\x80\x01Qa\x17TV[`@Q` \x01a\x13p\x95\x94\x93\x92\x91\x90a'\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[`\x08T`\x02T`\0T`@Qc\xC8\x179\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93c\xC8\x179\xCD\x93\x87\x93a\x13\xD7\x93\x91\x90\x92\x16\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\xA8\x1B\x90\x91\x04\x16\x90\x87\x90`\x04\x01a(?V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x13\xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x04W=`\0\x80>=`\0\xFD[PPPPPPPV[\x804\x14a\x14aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Fbridge fee mismatch\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\nTa\x14y\x90`\x01`\x01`\xA0\x1B\x03\x163\x85\x85a\x17\x8BV[PPPV[`\0\x80\x83Q\x83` a\x14\x90\x91\x90a'\x08V[\x11\x15\x80\x15a\x14\xA7WPa\x14\xA4\x83` a'\x08V[\x83\x10[a\x14\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\x14XV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x154W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x15\x14V[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x15\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x14XV[\x80a\x15\xA2\x85` a'\x08V[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a\x15\xBE\x85\x85a\x18\0V[\x86Q\x90\x95P\x90\x91Pa\x15\xD0\x82\x86a'\x08V[\x11\x15\x80\x15a\x15\xE6WPa\x15\xE3\x81\x85a'\x08V[\x84\x10[a\x16>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\x14XV[``\x81\x15\x80\x15a\x16YW`@Q\x91P` \x82\x01`@Ra\x16\xA3V[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a\x16\x92W\x80Q\x83R` \x92\x83\x01\x92\x01a\x16zV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a\x16\xAF\x83\x87a'\x08V[\x93P\x93PPP\x92P\x92\x90PV[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x17\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\x14XV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\x17DW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x17\"V[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\x17b\x81a\x1A\x04V[\x83`@Q` \x01a\x17t\x92\x91\x90a(qV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x17\xFA\x90\x85\x90a\x1A\xCEV[PPPPV[`\0\x80`\0a\x18\x0F\x85\x85a\x1B1V[\x94P\x90P`\0`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\xFD`\xF8\x1B\x03a\x18\xA7Wa\x184\x86\x86a\x1B\xB9V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a\x18OWPa\xFF\xFF\x81\x11\x15[a\x18\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\x14XV[\x92P\x83\x91Pa\x15\xA8\x90PV[`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\x7F`\xF9\x1B\x03a\x191Wa\x18\xC6\x86\x86a\x1CrV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a\x18\xE5WPc\xFF\xFF\xFF\xFF\x81\x11\x15[a\x18\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x14XV[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x90\x03a\x19\xAEWa\x19M\x86\x86a\x1DCV[\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a\x18\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x14XV[P`\xF8\x81\x90\x1C`\xFD\x81\x10a\x18\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x14XV[```\xFD\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x1A9W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91R[\x92\x91PPV[a\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1A\x89Wa\x1AY`\xFD`\xF8\x1Ba\x1E\x14V[a\x1Ab\x83a\x1E;V[`@Q` \x01a\x1As\x92\x91\x90a(qV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1A\xB4Wa\x1A\xAB`\x7F`\xF9\x1Ba\x1E\x14V[a\x1Ab\x83a\x1E~V[a\x1A\xC5`\x01`\x01`\xF8\x1B\x03\x19a\x1E\x14V[a\x1Ab\x83a\x1E\xC1V[`\0a\x1A\xE3`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x1F\x04V[\x90P\x80Q`\0\x14\x15\x80\x15a\x1B\x08WP\x80\x80` \x01\x90Q\x81\x01\x90a\x1B\x06\x91\x90a&\x9BV[\x15[\x15a\x14yW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x14XV[`\0\x80\x83Q\x83`\x01a\x1BC\x91\x90a'\x08V[\x11\x15\x80\x15a\x1BZWPa\x1BW\x83`\x01a'\x08V[\x83\x10[a\x1B\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\x14XV[\x83\x83\x01` \x01Q\x80a\x15\xA2\x85`\x01a'\x08V[`\0\x80\x83Q\x83`\x02a\x1B\xCB\x91\x90a'\x08V[\x11\x15\x80\x15a\x1B\xE2WPa\x1B\xDF\x83`\x02a'\x08V[\x83\x10[a\x1C9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x14XV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a\x15\xA2\x91\x90a'\x08V[`\0\x80\x83Q\x83`\x04a\x1C\x84\x91\x90a'\x08V[\x11\x15\x80\x15a\x1C\x9BWPa\x1C\x98\x83`\x04a'\x08V[\x83\x10[a\x1C\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x14XV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1D'W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1D\x07V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x15\xA2\x85`\x04a'\x08V[`\0\x80\x83Q\x83`\x08a\x1DU\x91\x90a'\x08V[\x11\x15\x80\x15a\x1DlWPa\x1Di\x83`\x08a'\x08V[\x83\x10[a\x1D\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x14XV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1D\xF8W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1D\xD8V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x15\xA2\x85`\x08a'\x08V[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a\x1A3V[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1EnW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1ELV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1E\xB1W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1E\x8FV[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1E\xF4W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1E\xD2V[PPP`(\x81\x01`@R\x92\x91PPV[``a\x13\x11\x83\x83`\0\x84`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x1F*\x91\x90a(\xA0V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1FgW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1FlV[``\x91P[P\x91P\x91Pa\x1F|\x86\x83\x83a\x1F\x86V[\x96\x95PPPPPPV[``\x82a\x1F\x9BWa\x1F\x96\x82a\x1F\xE2V[a\x13\x11V[\x81Q\x15\x80\x15a\x1F\xB2WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1F\xDBW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x14XV[P\x80a\x13\x11V[\x80Q\x15a\x1F\xF2W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0[\x83\x81\x10\x15a )W\x81\x81\x01Q\x83\x82\x01R` \x01a \x11V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra J\x81` \x86\x01` \x86\x01a \x0EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x13\x11` \x83\x01\x84a 2V[` \x81\x01`\x02\x83\x10a \x93WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a \xD2Wa \xD2a \x99V[`@R\x90V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a \xD2Wa \xD2a \x99V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a!%Wa!%a \x99V[`@R\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E\xC4W`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a!_Wa!_a \x99V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0a!\x80a!{\x84a!EV[a \xFCV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a!\x94W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a \x0BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a!\xD2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xE9W`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a!\xFBW`\0\x80\xFD[a\"\x03a \xAFV[a\"\x0C\x82a!-V[\x81R` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"(W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x86\x13a\"9W`\0\x80\xFD[a\"H\x86\x825` \x84\x01a!mV[` \x83\x01RP`@\x82\x015\x91Pa\"^\x82a!\xABV[`@\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\"\x85W`\0\x80\xFD[a\"\x8E\x86a!-V[\x94P` \x86\x015a\"\x9E\x81a!\xABV[\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xBAW`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\"\xCBW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xE2W`\0\x80\xFD[\x88` \x82\x84\x01\x01\x11\x15a\"\xF4W`\0\x80\xFD[` \x91\x90\x91\x01\x93P\x91P``\x86\x015a#\x0C\x81a!\xABV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E\xC4W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a#KW`\0\x80\xFD[a\x13\x11\x83\x835` \x85\x01a!mV[`\0a\x01\0\x82\x84\x03\x12\x15a#mW`\0\x80\xFD[a#ua \xD8V[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R\x90Pa#\x9B``\x83\x01a#\x1AV[``\x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xBAW`\0\x80\xFD[a#\xC6\x84\x82\x85\x01a#:V[`\x80\x83\x01RP`\xA0\x82\x81\x015\x90\x82\x01R`\xC0\x80\x83\x015\x90\x82\x01R`\xE0\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a$\x04W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\x1BW`\0\x80\xFD[a$'\x84\x82\x85\x01a#ZV[\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a$DW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$[W`\0\x80\xFD[a$g\x86\x82\x87\x01a#ZV[\x93PP` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\x8BW`\0\x80\xFD[a$\x97\x86\x82\x87\x01a#:V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a$\xB3W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a$\xCCW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xE3W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a$\xF4W`\0\x80\xFD[\x80Qa%\x02a!{\x82a!EV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a%\x17W`\0\x80\xFD[a%(\x82` \x83\x01` \x86\x01a \x0EV[\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a%EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a%eWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x14yW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a%\x92WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a%\xB2W`\0\x81U`\x01\x01a%\x9EV[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xD3Wa%\xD3a \x99V[a%\xE7\x81a%\xE1\x84Ta%1V[\x84a%kV[` `\x1F\x82\x11`\x01\x81\x14a&\x1BW`\0\x83\x15a&\x03WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua%\xB2V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a&KW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a&+V[P\x84\x82\x10\x15a&iW\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15a&\x8AW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x13\x11W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a&\xADW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\x11W`\0\x80\xFD[` \x81R`\x01`\x01`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q``\x82\x01R`\0``\x83\x01Q`\x80\x80\x84\x01Ra$'`\xA0\x84\x01\x82a 2V[\x80\x82\x01\x80\x82\x11\x15a\x1A3WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a';W`\0\x80\xFD[\x81Qa\x13\x11\x81a!\xABV[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra'\x8A`\xE0\x84\x01\x82a 2V[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a'\xCBW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a'\xACV[PPP\x92\x91PPV[`\0\x86Qa'\xE6\x81\x84` \x8B\x01a \x0EV[\x86Q\x90\x83\x01\x90a'\xFA\x81\x83` \x8B\x01a \x0EV[\x86Q\x91\x01\x90a(\r\x81\x83` \x8A\x01a \x0EV[\x85Q\x91\x01\x90a( \x81\x83` \x89\x01a \x0EV[\x84Q\x91\x01\x90a(3\x81\x83` \x88\x01a \x0EV[\x01\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R```@\x82\x01R`\0a%(``\x83\x01\x84a 2V[`\0\x83Qa(\x83\x81\x84` \x88\x01a \x0EV[\x83Q\x90\x83\x01\x90a(\x97\x81\x83` \x88\x01a \x0EV[\x01\x94\x93PPPPV[`\0\x82Qa(\xB2\x81\x84` \x87\x01a \x0EV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 S\xC1\xC8d\xEA\x8F\xFB\xBF\xF9\x93\x97\x03\xFCH`\xC3\x87<\x11\xA6p\xC2\xDEI\xEA\xA0(\x19\x91i\x92}dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MYSTIKOV2TBRIDGEERC20_BYTECODE: ::ethers_core::types::Bytes = ::ethers_core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xB7W`\x005`\xE0\x1C\x80c\xC2\xD4\x16\x01\x11a\0\xECW\x80c\xDD\xAC]\xC1\x11a\0\x8AW\x80c\xEF\xBF\xB2\xAE\x11a\0dW\x80c\xEF\xBF\xB2\xAE\x14a\x04\xB0W\x80c\xF4\xAD\x17\xC6\x14a\x04\xC5W\x80c\xFAu\x0FV\x14a\x04\xDAW\x80c\xFB>=s\x14a\x04\xFBW`\0\x80\xFD[\x80c\xDD\xAC]\xC1\x14a\x04fW\x80c\xE0at\xE4\x14a\x04{W\x80c\xEDn\xA3:\x14a\x04\x9BW`\0\x80\xFD[\x80c\xCB\xE3B\x85\x11a\0\xC6W\x80c\xCB\xE3B\x85\x14a\x03\xE4W\x80c\xCD\xFC\xEE\xBA\x14a\x03\xFAW\x80c\xCF\xC7\xE2\xDA\x14a\x04;W\x80c\xD0\xB46\xBD\x14a\x04PW`\0\x80\xFD[\x80c\xC2\xD4\x16\x01\x14a\x03\x95W\x80c\xC9#\x0C]\x14a\x03\xBCW\x80c\xCB\\\x02\x9A\x14a\x03\xD1W`\0\x80\xFD[\x80cB.\0(\x11a\x01YW\x80cd\x0C\x0B6\x11a\x013W\x80cd\x0C\x0B6\x14a\x03'W\x80c\x82[_\x8D\x14a\x03=W\x80c\x82\xD2\x1C\xD8\x14a\x03RW\x80c\x9A\x03cl\x14a\x03\x82W`\0\x80\xFD[\x80cB.\0(\x14a\x02\xDAW\x80cM\xDEo\xBC\x14a\x02\xFCW\x80cN<\x10\xB7\x14a\x03\x12W`\0\x80\xFD[\x80c!\xE3-U\x11a\x01\x95W\x80c!\xE3-U\x14a\x028W\x80c$!\xE1U\x14a\x02XW\x80c,\xD2mE\x14a\x02\x9EW\x80c?\xE34z\x14a\x02\xBEW`\0\x80\xFD[\x80c\x0B\xA9Y\t\x14a\x01\xBCW\x80c\x17m\xE7\xA8\x14a\x01\xE4W\x80c\x1B\xA4l\xFD\x14a\x02\x06W[`\0\x80\xFD[4\x80\x15a\x01\xC8W`\0\x80\xFD[Pa\x01\xD1a\x05\x11V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF0W`\0\x80\xFD[Pa\x01\xF9a\x05\x9AV[`@Qa\x01\xDB\x91\x90a ^V[4\x80\x15a\x02\x12W`\0\x80\xFD[P`\nT`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xDBV[4\x80\x15a\x02DW`\0\x80\xFD[P`\x02Ta\x02 \x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02dW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x07\x81R\x7Ftbridge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x01\xF9V[4\x80\x15a\x02\xAAW`\0\x80\xFD[P`\x08Ta\x02 \x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\xCAW`\0\x80\xFD[P`\0`@Qa\x01\xDB\x91\x90a qV[4\x80\x15a\x02\xE6W`\0\x80\xFD[Pa\x02\xFAa\x02\xF56`\x04a!\xC0V[a\x06\x11V[\0[4\x80\x15a\x03\x08W`\0\x80\xFD[Pa\x01\xD1`\x05T\x81V[4\x80\x15a\x03\x1EW`\0\x80\xFD[Pa\x01\xF9a\x06\xFEV[4\x80\x15a\x033W`\0\x80\xFD[Pa\x01\xD1`\x06T\x81V[4\x80\x15a\x03IW`\0\x80\xFD[Pa\x01\xD1a\x07\x8CV[4\x80\x15a\x03^W`\0\x80\xFD[Pa\x03ra\x03m6`\x04a\"mV[a\x08\x13V[`@Q\x90\x15\x15\x81R` \x01a\x01\xDBV[a\x02\xFAa\x03\x906`\x04a#\xF2V[a\x08\x9DV[4\x80\x15a\x03\xA1W`\0\x80\xFD[Pa\x03\xAAa\x08\xB6V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xDBV[4\x80\x15a\x03\xC8W`\0\x80\xFD[Pa\x01\xF9a\t$V[a\x02\xFAa\x03\xDF6`\x04a$/V[a\tnV[4\x80\x15a\x03\xF0W`\0\x80\xFD[Pa\x01\xD1`\x07T\x81V[4\x80\x15a\x04\x06W`\0\x80\xFD[P`\0Ta\x04\"\x90`\x01`\xA8\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xDBV[4\x80\x15a\x04GW`\0\x80\xFD[Pa\x01\xD1a\r\xA3V[4\x80\x15a\x04\\W`\0\x80\xFD[Pa\x01\xD1`\x04T\x81V[4\x80\x15a\x04rW`\0\x80\xFD[Pa\x02 a\x0E*V[4\x80\x15a\x04\x87W`\0\x80\xFD[P`\tTa\x02 \x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xA7W`\0\x80\xFD[Pa\x03ra\x0E\xC9V[4\x80\x15a\x04\xBCW`\0\x80\xFD[Pa\x01\xD1a\x0F6V[4\x80\x15a\x04\xD1W`\0\x80\xFD[Pa\x01\xD1a\x0F\xBDV[4\x80\x15a\x04\xE6W`\0\x80\xFD[P`\0Ta\x03r\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x05\x07W`\0\x80\xFD[Pa\x01\xD1`\x03T\x81V[`\tT`@QcG:\x061`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG:\x061\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x82\x91\x90a$\xA1V[\x90P\x80\x15a\x05\x90W\x80a\x05\x94V[`\x04T[\x91PP\x90V[`\nT`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06\x0C\x91\x90\x81\x01\x90a$\xBAV[\x90P\x90V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x06<W`@Qcm\x8F\x11Q`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA8\x1B\x02\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U` \x81\x01Q`\x01\x90a\x06\x90\x90\x82a%\xB9V[P`@\x01Q`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90U`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x17\x90UV[`\x01\x80Ta\x07\x0B\x90a%1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x077\x90a%1V[\x80\x15a\x07\x84W\x80`\x1F\x10a\x07YWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x84V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07gW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\tT`@Qca\x86<\x03`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC3\x0Cx\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xFD\x91\x90a$\xA1V[\x90P\x80\x15a\x08\x0BW\x80a\x05\x94V[PP`\x07T\x90V[`\x08T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08AW`@Qc=\xCA\x01\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x08\x82\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x10D\x92PPPV[\x90Pa\x08\x90\x87\x87\x85\x84a\x11\x0CV[P`\x01\x96\x95PPPPPPV[`@Qc\xE7\xA2O\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1<\xE5g\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\t\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x0C\x91\x90a&xV[`\nT`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xE4W=`\0\x80>=`\0\xFD[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xDA\x91\x90a&\x9BV[\x15a\t\xF8W`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xBCXw\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nKW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\no\x91\x90a&\x9BV[\x15a\x0BFW`\0`@Q\x80`\x80\x01`@R\x802`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\n\xA2`\nT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x86\x90R`@\x91\x82\x01\x85\x90R`\tT\x91Qc\x84\x9E\x8B\x9F`\xE0\x1B\x81R\x92\x93P\x16\x90c\x84\x9E\x8B\x9F\x90a\n\xE6\x90\x84\x90`\x04\x01a&\xBDV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B'\x91\x90a&\x9BV[a\x0BDW`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[a\x0BNa\r\xA3V[\x83Q\x10\x15a\x0BoW`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Bwa\x05\x11V[\x83Q\x11\x15a\x0B\x98W`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xA0a\x0F6V[\x83`\xA0\x01Q\x10\x15a\x0B\xC4W`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xCCa\x0F\xBDV[\x83`\xC0\x01Q\x10\x15a\x0B\xF0W`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xF8a\x07\x8CV[\x83`\xE0\x01Q\x10\x15a\x0C\x1CW`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0C5\x84`@\x01Q\x85`\0\x01Q\x86``\x01Qa\x11\xFFV[\x90P\x80\x84` \x01Q\x14a\x0C[W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT`@Qc\xDFY/}`\xE0\x1B\x81R2`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC7\x91\x90a&\x9BV[\x15a\x0C\xE5W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x90\x82\x01R`\xC0\x86\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x85\x01Q``\x82\x01R`\x80\x80\x86\x01Q\x90\x82\x01R`\0a\r(\x82a\x13\x18V[\x90Pa\r8\x86`\xA0\x01Q\x82a\x13\x87V[a\rma\rCa\x0E*V[`\xE0\x88\x01Q`\xC0\x89\x01Q\x89Qa\rY\x91\x90a'\x08V[a\rc\x91\x90a'\x08V[\x88`\xA0\x01Qa\x14\rV[` \x86\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPPPV[`\tT`@QcU%\x98I`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cU%\x98I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x14\x91\x90a$\xA1V[\x90P\x80\x15a\x0E\"W\x80a\x05\x94V[PP`\x03T\x90V[`\tT`@QcA\xFBiy`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cA\xFBiy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EwW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x9B\x91\x90a')V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\xC4W`@Qc\x06\xF3\xD63`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x0C\x91\x90a&\x9BV[`\tT`@Qc{R\xB0\r`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c{R\xB0\r\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA7\x91\x90a$\xA1V[\x90P\x80\x15a\x0F\xB5W\x80a\x05\x94V[PP`\x05T\x90V[`\tT`@QcO1O\x9D`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9Eb\x9F:\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10.\x91\x90a$\xA1V[\x90P\x80\x15a\x10<W\x80a\x05\x94V[PP`\x06T\x90V[a\x10v`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\x10\xA8`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\x10\xB4\x84\x82a\x14~V[\x90\x83R\x90Pa\x10\xC3\x84\x82a\x14~V[` \x84\x01\x91\x90\x91R\x90Pa\x10\xD7\x84\x82a\x14~V[`@\x84\x01\x91\x90\x91R\x90Pa\x10\xEB\x84\x82a\x14~V[``\x84\x01\x91\x90\x91R\x90Pa\x10\xFF\x84\x82a\x15\xAFV[P`\x80\x83\x01RP\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x11:W`@Qc\x1BI^\xCF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xA8\x1B\x90\x92\x04\x16\x14a\x11pW`@Qc7;\xC1!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x03a\x11\x92W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\x9Aa\x0E*V[`\x01`\x01`\xA0\x1B\x03\x16cx\xD6\x0C\xD7\x82\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xC7\x92\x91\x90a'FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xF5W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x12BW`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x12tW`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x12\xCC\x91`\x04\x01a'\xA3V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\r\x91\x90a$\xA1V[\x91PP[\x93\x92PPPV[``\x80a\x13(\x83`\0\x01Qa\x16\xBCV[a\x135\x84` \x01Qa\x16\xBCV[a\x13B\x85`@\x01Qa\x16\xBCV[a\x13O\x86``\x01Qa\x16\xBCV[a\x13\\\x87`\x80\x01Qa\x17TV[`@Q` \x01a\x13p\x95\x94\x93\x92\x91\x90a'\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[`\x08T`\x02T`\0T`@Qc\xC8\x179\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93c\xC8\x179\xCD\x93\x87\x93a\x13\xD7\x93\x91\x90\x92\x16\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\xA8\x1B\x90\x91\x04\x16\x90\x87\x90`\x04\x01a(?V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x13\xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x04W=`\0\x80>=`\0\xFD[PPPPPPPV[\x804\x14a\x14aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Fbridge fee mismatch\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\nTa\x14y\x90`\x01`\x01`\xA0\x1B\x03\x163\x85\x85a\x17\x8BV[PPPV[`\0\x80\x83Q\x83` a\x14\x90\x91\x90a'\x08V[\x11\x15\x80\x15a\x14\xA7WPa\x14\xA4\x83` a'\x08V[\x83\x10[a\x14\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\x14XV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x154W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x15\x14V[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x15\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x14XV[\x80a\x15\xA2\x85` a'\x08V[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a\x15\xBE\x85\x85a\x18\0V[\x86Q\x90\x95P\x90\x91Pa\x15\xD0\x82\x86a'\x08V[\x11\x15\x80\x15a\x15\xE6WPa\x15\xE3\x81\x85a'\x08V[\x84\x10[a\x16>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\x14XV[``\x81\x15\x80\x15a\x16YW`@Q\x91P` \x82\x01`@Ra\x16\xA3V[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a\x16\x92W\x80Q\x83R` \x92\x83\x01\x92\x01a\x16zV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a\x16\xAF\x83\x87a'\x08V[\x93P\x93PPP\x92P\x92\x90PV[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x17\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\x14XV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\x17DW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x17\"V[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\x17b\x81a\x1A\x04V[\x83`@Q` \x01a\x17t\x92\x91\x90a(qV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x17\xFA\x90\x85\x90a\x1A\xCEV[PPPPV[`\0\x80`\0a\x18\x0F\x85\x85a\x1B1V[\x94P\x90P`\0`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\xFD`\xF8\x1B\x03a\x18\xA7Wa\x184\x86\x86a\x1B\xB9V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a\x18OWPa\xFF\xFF\x81\x11\x15[a\x18\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\x14XV[\x92P\x83\x91Pa\x15\xA8\x90PV[`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\x7F`\xF9\x1B\x03a\x191Wa\x18\xC6\x86\x86a\x1CrV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a\x18\xE5WPc\xFF\xFF\xFF\xFF\x81\x11\x15[a\x18\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x14XV[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x90\x03a\x19\xAEWa\x19M\x86\x86a\x1DCV[\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a\x18\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x14XV[P`\xF8\x81\x90\x1C`\xFD\x81\x10a\x18\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\x14XV[```\xFD\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x1A9W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91R[\x92\x91PPV[a\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1A\x89Wa\x1AY`\xFD`\xF8\x1Ba\x1E\x14V[a\x1Ab\x83a\x1E;V[`@Q` \x01a\x1As\x92\x91\x90a(qV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1A\xB4Wa\x1A\xAB`\x7F`\xF9\x1Ba\x1E\x14V[a\x1Ab\x83a\x1E~V[a\x1A\xC5`\x01`\x01`\xF8\x1B\x03\x19a\x1E\x14V[a\x1Ab\x83a\x1E\xC1V[`\0a\x1A\xE3`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x1F\x04V[\x90P\x80Q`\0\x14\x15\x80\x15a\x1B\x08WP\x80\x80` \x01\x90Q\x81\x01\x90a\x1B\x06\x91\x90a&\x9BV[\x15[\x15a\x14yW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x14XV[`\0\x80\x83Q\x83`\x01a\x1BC\x91\x90a'\x08V[\x11\x15\x80\x15a\x1BZWPa\x1BW\x83`\x01a'\x08V[\x83\x10[a\x1B\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\x14XV[\x83\x83\x01` \x01Q\x80a\x15\xA2\x85`\x01a'\x08V[`\0\x80\x83Q\x83`\x02a\x1B\xCB\x91\x90a'\x08V[\x11\x15\x80\x15a\x1B\xE2WPa\x1B\xDF\x83`\x02a'\x08V[\x83\x10[a\x1C9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x14XV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a\x15\xA2\x91\x90a'\x08V[`\0\x80\x83Q\x83`\x04a\x1C\x84\x91\x90a'\x08V[\x11\x15\x80\x15a\x1C\x9BWPa\x1C\x98\x83`\x04a'\x08V[\x83\x10[a\x1C\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x14XV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1D'W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1D\x07V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x15\xA2\x85`\x04a'\x08V[`\0\x80\x83Q\x83`\x08a\x1DU\x91\x90a'\x08V[\x11\x15\x80\x15a\x1DlWPa\x1Di\x83`\x08a'\x08V[\x83\x10[a\x1D\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x14XV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1D\xF8W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1D\xD8V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x15\xA2\x85`\x08a'\x08V[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a\x1A3V[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1EnW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1ELV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1E\xB1W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1E\x8FV[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1E\xF4W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1E\xD2V[PPP`(\x81\x01`@R\x92\x91PPV[``a\x13\x11\x83\x83`\0\x84`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x1F*\x91\x90a(\xA0V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1FgW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1FlV[``\x91P[P\x91P\x91Pa\x1F|\x86\x83\x83a\x1F\x86V[\x96\x95PPPPPPV[``\x82a\x1F\x9BWa\x1F\x96\x82a\x1F\xE2V[a\x13\x11V[\x81Q\x15\x80\x15a\x1F\xB2WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1F\xDBW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x14XV[P\x80a\x13\x11V[\x80Q\x15a\x1F\xF2W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0[\x83\x81\x10\x15a )W\x81\x81\x01Q\x83\x82\x01R` \x01a \x11V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra J\x81` \x86\x01` \x86\x01a \x0EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x13\x11` \x83\x01\x84a 2V[` \x81\x01`\x02\x83\x10a \x93WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a \xD2Wa \xD2a \x99V[`@R\x90V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a \xD2Wa \xD2a \x99V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a!%Wa!%a \x99V[`@R\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E\xC4W`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a!_Wa!_a \x99V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0a!\x80a!{\x84a!EV[a \xFCV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a!\x94W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a \x0BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a!\xD2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xE9W`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a!\xFBW`\0\x80\xFD[a\"\x03a \xAFV[a\"\x0C\x82a!-V[\x81R` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"(W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x86\x13a\"9W`\0\x80\xFD[a\"H\x86\x825` \x84\x01a!mV[` \x83\x01RP`@\x82\x015\x91Pa\"^\x82a!\xABV[`@\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\"\x85W`\0\x80\xFD[a\"\x8E\x86a!-V[\x94P` \x86\x015a\"\x9E\x81a!\xABV[\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xBAW`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\"\xCBW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xE2W`\0\x80\xFD[\x88` \x82\x84\x01\x01\x11\x15a\"\xF4W`\0\x80\xFD[` \x91\x90\x91\x01\x93P\x91P``\x86\x015a#\x0C\x81a!\xABV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E\xC4W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a#KW`\0\x80\xFD[a\x13\x11\x83\x835` \x85\x01a!mV[`\0a\x01\0\x82\x84\x03\x12\x15a#mW`\0\x80\xFD[a#ua \xD8V[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R\x90Pa#\x9B``\x83\x01a#\x1AV[``\x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xBAW`\0\x80\xFD[a#\xC6\x84\x82\x85\x01a#:V[`\x80\x83\x01RP`\xA0\x82\x81\x015\x90\x82\x01R`\xC0\x80\x83\x015\x90\x82\x01R`\xE0\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a$\x04W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\x1BW`\0\x80\xFD[a$'\x84\x82\x85\x01a#ZV[\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a$DW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$[W`\0\x80\xFD[a$g\x86\x82\x87\x01a#ZV[\x93PP` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\x8BW`\0\x80\xFD[a$\x97\x86\x82\x87\x01a#:V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a$\xB3W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a$\xCCW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xE3W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a$\xF4W`\0\x80\xFD[\x80Qa%\x02a!{\x82a!EV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a%\x17W`\0\x80\xFD[a%(\x82` \x83\x01` \x86\x01a \x0EV[\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a%EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a%eWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x14yW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a%\x92WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a%\xB2W`\0\x81U`\x01\x01a%\x9EV[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xD3Wa%\xD3a \x99V[a%\xE7\x81a%\xE1\x84Ta%1V[\x84a%kV[` `\x1F\x82\x11`\x01\x81\x14a&\x1BW`\0\x83\x15a&\x03WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua%\xB2V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a&KW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a&+V[P\x84\x82\x10\x15a&iW\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15a&\x8AW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x13\x11W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a&\xADW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\x11W`\0\x80\xFD[` \x81R`\x01`\x01`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q``\x82\x01R`\0``\x83\x01Q`\x80\x80\x84\x01Ra$'`\xA0\x84\x01\x82a 2V[\x80\x82\x01\x80\x82\x11\x15a\x1A3WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a';W`\0\x80\xFD[\x81Qa\x13\x11\x81a!\xABV[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra'\x8A`\xE0\x84\x01\x82a 2V[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a'\xCBW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a'\xACV[PPP\x92\x91PPV[`\0\x86Qa'\xE6\x81\x84` \x8B\x01a \x0EV[\x86Q\x90\x83\x01\x90a'\xFA\x81\x83` \x8B\x01a \x0EV[\x86Q\x91\x01\x90a(\r\x81\x83` \x8A\x01a \x0EV[\x85Q\x91\x01\x90a( \x81\x83` \x89\x01a \x0EV[\x84Q\x91\x01\x90a(3\x81\x83` \x88\x01a \x0EV[\x01\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R```@\x82\x01R`\0a%(``\x83\x01\x84a 2V[`\0\x83Qa(\x83\x81\x84` \x88\x01a \x0EV[\x83Q\x90\x83\x01\x90a(\x97\x81\x83` \x88\x01a \x0EV[\x01\x94\x93PPPPV[`\0\x82Qa(\xB2\x81\x84` \x87\x01a \x0EV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 S\xC1\xC8d\xEA\x8F\xFB\xBF\xF9\x93\x97\x03\xFCH`\xC3\x87<\x11\xA6p\xC2\xDEI\xEA\xA0(\x19\x91i\x92}dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MYSTIKOV2TBRIDGEERC20_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes = ::ethers_core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MystikoV2TBridgeERC20<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoV2TBridgeERC20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoV2TBridgeERC20<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoV2TBridgeERC20<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoV2TBridgeERC20<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MystikoV2TBridgeERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> MystikoV2TBridgeERC20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    MYSTIKOV2TBRIDGEERC20_ABI.clone(),
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
                MYSTIKOV2TBRIDGEERC20_ABI.clone(),
                MYSTIKOV2TBRIDGEERC20_BYTECODE.clone().into(),
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
        ///Calls the contract's `assetDecimals` (0xc2d41601) function
        pub fn asset_decimals(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([194, 212, 22, 1], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetName` (0xc9230c5d) function
        pub fn asset_name(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([201, 35, 12, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetSymbol` (0x176de7a8) function
        pub fn asset_symbol(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([23, 109, 231, 168], ())
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
        ///Calls the contract's `crossChainSyncTx` (0x82d21cd8) function
        pub fn cross_chain_sync_tx(
            &self,
            from_chain_id: u64,
            from_contract: ::ethers_core::types::Address,
            message: ::ethers_core::types::Bytes,
            executor: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [130, 210, 28, 216],
                    (from_chain_id, from_contract, message, executor),
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
    for MystikoV2TBridgeERC20<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
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
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers_core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
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
        name = "AddressInsufficientBalance",
        abi = "AddressInsufficientBalance(address)"
    )]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers_core::types::Address,
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
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
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
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
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
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
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
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers_core::types::Address,
    }
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
    pub enum MystikoV2TBridgeERC20Errors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        AmountLessThanZero(AmountLessThanZero),
        AmountTooLarge(AmountTooLarge),
        AmountTooSmall(AmountTooSmall),
        AssociatedPoolNotSet(AssociatedPoolNotSet),
        BridgeFeeTooFew(BridgeFeeTooFew),
        CertificateInvalid(CertificateInvalid),
        CommitmentHashIncorrect(CommitmentHashIncorrect),
        DepositsDisabled(DepositsDisabled),
        ExecutorFeeTooFew(ExecutorFeeTooFew),
        FailedInnerCall(FailedInnerCall),
        HashKGreaterThanFieldSize(HashKGreaterThanFieldSize),
        NotSupport(NotSupport),
        PeerChainIdNotMatched(PeerChainIdNotMatched),
        PeerContractAlreadySet(PeerContractAlreadySet),
        PeerContractNotMatched(PeerContractNotMatched),
        RandomSGreaterThanFieldSize(RandomSGreaterThanFieldSize),
        RollupFeeToFew(RollupFeeToFew),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        SanctionedAddress(SanctionedAddress),
        SenderIsNotBridgeProxy(SenderIsNotBridgeProxy),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoV2TBridgeERC20Errors {
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
                = <AddressEmptyCode as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded)
                = <AddressInsufficientBalance as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddressInsufficientBalance(decoded));
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
                = <FailedInnerCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
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
                = <SafeERC20FailedOperation as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
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
    impl ::ethers_core::abi::AbiEncode for MystikoV2TBridgeERC20Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
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
                Self::FailedInnerCall(element) => {
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
                Self::SafeERC20FailedOperation(element) => {
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
    impl ::ethers_contract::ContractRevert for MystikoV2TBridgeERC20Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressInsufficientBalance as ::ethers_contract::EthError>::selector() => {
                    true
                }
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
                    == <FailedInnerCall as ::ethers_contract::EthError>::selector() => {
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
                    == <SafeERC20FailedOperation as ::ethers_contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for MystikoV2TBridgeERC20Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SanctionedAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SenderIsNotBridgeProxy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MystikoV2TBridgeERC20Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for MystikoV2TBridgeERC20Errors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance>
    for MystikoV2TBridgeERC20Errors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<AmountLessThanZero> for MystikoV2TBridgeERC20Errors {
        fn from(value: AmountLessThanZero) -> Self {
            Self::AmountLessThanZero(value)
        }
    }
    impl ::core::convert::From<AmountTooLarge> for MystikoV2TBridgeERC20Errors {
        fn from(value: AmountTooLarge) -> Self {
            Self::AmountTooLarge(value)
        }
    }
    impl ::core::convert::From<AmountTooSmall> for MystikoV2TBridgeERC20Errors {
        fn from(value: AmountTooSmall) -> Self {
            Self::AmountTooSmall(value)
        }
    }
    impl ::core::convert::From<AssociatedPoolNotSet> for MystikoV2TBridgeERC20Errors {
        fn from(value: AssociatedPoolNotSet) -> Self {
            Self::AssociatedPoolNotSet(value)
        }
    }
    impl ::core::convert::From<BridgeFeeTooFew> for MystikoV2TBridgeERC20Errors {
        fn from(value: BridgeFeeTooFew) -> Self {
            Self::BridgeFeeTooFew(value)
        }
    }
    impl ::core::convert::From<CertificateInvalid> for MystikoV2TBridgeERC20Errors {
        fn from(value: CertificateInvalid) -> Self {
            Self::CertificateInvalid(value)
        }
    }
    impl ::core::convert::From<CommitmentHashIncorrect> for MystikoV2TBridgeERC20Errors {
        fn from(value: CommitmentHashIncorrect) -> Self {
            Self::CommitmentHashIncorrect(value)
        }
    }
    impl ::core::convert::From<DepositsDisabled> for MystikoV2TBridgeERC20Errors {
        fn from(value: DepositsDisabled) -> Self {
            Self::DepositsDisabled(value)
        }
    }
    impl ::core::convert::From<ExecutorFeeTooFew> for MystikoV2TBridgeERC20Errors {
        fn from(value: ExecutorFeeTooFew) -> Self {
            Self::ExecutorFeeTooFew(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for MystikoV2TBridgeERC20Errors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<HashKGreaterThanFieldSize>
    for MystikoV2TBridgeERC20Errors {
        fn from(value: HashKGreaterThanFieldSize) -> Self {
            Self::HashKGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<NotSupport> for MystikoV2TBridgeERC20Errors {
        fn from(value: NotSupport) -> Self {
            Self::NotSupport(value)
        }
    }
    impl ::core::convert::From<PeerChainIdNotMatched> for MystikoV2TBridgeERC20Errors {
        fn from(value: PeerChainIdNotMatched) -> Self {
            Self::PeerChainIdNotMatched(value)
        }
    }
    impl ::core::convert::From<PeerContractAlreadySet> for MystikoV2TBridgeERC20Errors {
        fn from(value: PeerContractAlreadySet) -> Self {
            Self::PeerContractAlreadySet(value)
        }
    }
    impl ::core::convert::From<PeerContractNotMatched> for MystikoV2TBridgeERC20Errors {
        fn from(value: PeerContractNotMatched) -> Self {
            Self::PeerContractNotMatched(value)
        }
    }
    impl ::core::convert::From<RandomSGreaterThanFieldSize>
    for MystikoV2TBridgeERC20Errors {
        fn from(value: RandomSGreaterThanFieldSize) -> Self {
            Self::RandomSGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<RollupFeeToFew> for MystikoV2TBridgeERC20Errors {
        fn from(value: RollupFeeToFew) -> Self {
            Self::RollupFeeToFew(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation>
    for MystikoV2TBridgeERC20Errors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<SanctionedAddress> for MystikoV2TBridgeERC20Errors {
        fn from(value: SanctionedAddress) -> Self {
            Self::SanctionedAddress(value)
        }
    }
    impl ::core::convert::From<SenderIsNotBridgeProxy> for MystikoV2TBridgeERC20Errors {
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
    ///Container type for all input parameters for the `assetDecimals` function with signature `assetDecimals()` and selector `0xc2d41601`
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
    #[ethcall(name = "assetDecimals", abi = "assetDecimals()")]
    pub struct AssetDecimalsCall;
    ///Container type for all input parameters for the `assetName` function with signature `assetName()` and selector `0xc9230c5d`
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
    #[ethcall(name = "assetName", abi = "assetName()")]
    pub struct AssetNameCall;
    ///Container type for all input parameters for the `assetSymbol` function with signature `assetSymbol()` and selector `0x176de7a8`
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
    #[ethcall(name = "assetSymbol", abi = "assetSymbol()")]
    pub struct AssetSymbolCall;
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
    ///Container type for all input parameters for the `crossChainSyncTx` function with signature `crossChainSyncTx(uint64,address,bytes,address)` and selector `0x82d21cd8`
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
        name = "crossChainSyncTx",
        abi = "crossChainSyncTx(uint64,address,bytes,address)"
    )]
    pub struct CrossChainSyncTxCall {
        pub from_chain_id: u64,
        pub from_contract: ::ethers_core::types::Address,
        pub message: ::ethers_core::types::Bytes,
        pub executor: ::ethers_core::types::Address,
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
    pub enum MystikoV2TBridgeERC20Calls {
        AssetAddress(AssetAddressCall),
        AssetDecimals(AssetDecimalsCall),
        AssetName(AssetNameCall),
        AssetSymbol(AssetSymbolCall),
        AssetType(AssetTypeCall),
        BridgeProxyAddress(BridgeProxyAddressCall),
        BridgeType(BridgeTypeCall),
        CertDeposit(CertDepositCall),
        CrossChainSyncTx(CrossChainSyncTxCall),
        DefaultMaxAmount(DefaultMaxAmountCall),
        DefaultMinAmount(DefaultMinAmountCall),
        DefaultMinBridgeFee(DefaultMinBridgeFeeCall),
        DefaultPeerMinExecutorFee(DefaultPeerMinExecutorFeeCall),
        DefaultPeerMinRollupFee(DefaultPeerMinRollupFeeCall),
        Deposit(DepositCall),
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
    impl ::ethers_core::abi::AbiDecode for MystikoV2TBridgeERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AssetAddressCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetAddress(decoded));
            }
            if let Ok(decoded)
                = <AssetDecimalsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetDecimals(decoded));
            }
            if let Ok(decoded)
                = <AssetNameCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetName(decoded));
            }
            if let Ok(decoded)
                = <AssetSymbolCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetSymbol(decoded));
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
                = <CrossChainSyncTxCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CrossChainSyncTx(decoded));
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
    impl ::ethers_core::abi::AbiEncode for MystikoV2TBridgeERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetAddress(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AssetDecimals(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AssetName(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AssetSymbol(element) => {
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
                Self::CrossChainSyncTx(element) => {
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
    impl ::core::fmt::Display for MystikoV2TBridgeERC20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetName(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetSymbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetType(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeProxyAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BridgeType(element) => ::core::fmt::Display::fmt(element, f),
                Self::CertDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::CrossChainSyncTx(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AssetAddressCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: AssetAddressCall) -> Self {
            Self::AssetAddress(value)
        }
    }
    impl ::core::convert::From<AssetDecimalsCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: AssetDecimalsCall) -> Self {
            Self::AssetDecimals(value)
        }
    }
    impl ::core::convert::From<AssetNameCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: AssetNameCall) -> Self {
            Self::AssetName(value)
        }
    }
    impl ::core::convert::From<AssetSymbolCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: AssetSymbolCall) -> Self {
            Self::AssetSymbol(value)
        }
    }
    impl ::core::convert::From<AssetTypeCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: AssetTypeCall) -> Self {
            Self::AssetType(value)
        }
    }
    impl ::core::convert::From<BridgeProxyAddressCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: BridgeProxyAddressCall) -> Self {
            Self::BridgeProxyAddress(value)
        }
    }
    impl ::core::convert::From<BridgeTypeCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: BridgeTypeCall) -> Self {
            Self::BridgeType(value)
        }
    }
    impl ::core::convert::From<CertDepositCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: CertDepositCall) -> Self {
            Self::CertDeposit(value)
        }
    }
    impl ::core::convert::From<CrossChainSyncTxCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: CrossChainSyncTxCall) -> Self {
            Self::CrossChainSyncTx(value)
        }
    }
    impl ::core::convert::From<DefaultMaxAmountCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: DefaultMaxAmountCall) -> Self {
            Self::DefaultMaxAmount(value)
        }
    }
    impl ::core::convert::From<DefaultMinAmountCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: DefaultMinAmountCall) -> Self {
            Self::DefaultMinAmount(value)
        }
    }
    impl ::core::convert::From<DefaultMinBridgeFeeCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: DefaultMinBridgeFeeCall) -> Self {
            Self::DefaultMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<DefaultPeerMinExecutorFeeCall>
    for MystikoV2TBridgeERC20Calls {
        fn from(value: DefaultPeerMinExecutorFeeCall) -> Self {
            Self::DefaultPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<DefaultPeerMinRollupFeeCall>
    for MystikoV2TBridgeERC20Calls {
        fn from(value: DefaultPeerMinRollupFeeCall) -> Self {
            Self::DefaultPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<GetAssociatedCommitmentPoolCall>
    for MystikoV2TBridgeERC20Calls {
        fn from(value: GetAssociatedCommitmentPoolCall) -> Self {
            Self::GetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<GetMaxAmountCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: GetMaxAmountCall) -> Self {
            Self::GetMaxAmount(value)
        }
    }
    impl ::core::convert::From<GetMinAmountCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: GetMinAmountCall) -> Self {
            Self::GetMinAmount(value)
        }
    }
    impl ::core::convert::From<GetMinBridgeFeeCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: GetMinBridgeFeeCall) -> Self {
            Self::GetMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<GetMinExecutorFeeCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: GetMinExecutorFeeCall) -> Self {
            Self::GetMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<GetPeerMinRollupFeeCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: GetPeerMinRollupFeeCall) -> Self {
            Self::GetPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<IsDepositsDisabledCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: IsDepositsDisabledCall) -> Self {
            Self::IsDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<IsPeerContractSetCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: IsPeerContractSetCall) -> Self {
            Self::IsPeerContractSet(value)
        }
    }
    impl ::core::convert::From<PeerChainIdCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: PeerChainIdCall) -> Self {
            Self::PeerChainId(value)
        }
    }
    impl ::core::convert::From<PeerChainNameCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: PeerChainNameCall) -> Self {
            Self::PeerChainName(value)
        }
    }
    impl ::core::convert::From<PeerContractCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: PeerContractCall) -> Self {
            Self::PeerContract(value)
        }
    }
    impl ::core::convert::From<SetPeerContractCall> for MystikoV2TBridgeERC20Calls {
        fn from(value: SetPeerContractCall) -> Self {
            Self::SetPeerContract(value)
        }
    }
    impl ::core::convert::From<SettingsCall> for MystikoV2TBridgeERC20Calls {
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
    ///Container type for all return fields from the `assetDecimals` function with signature `assetDecimals()` and selector `0xc2d41601`
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
    pub struct AssetDecimalsReturn(pub u8);
    ///Container type for all return fields from the `assetName` function with signature `assetName()` and selector `0xc9230c5d`
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
    pub struct AssetNameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `assetSymbol` function with signature `assetSymbol()` and selector `0x176de7a8`
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
    pub struct AssetSymbolReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `crossChainSyncTx` function with signature `crossChainSyncTx(uint64,address,bytes,address)` and selector `0x82d21cd8`
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
    pub struct CrossChainSyncTxReturn(pub bool);
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
