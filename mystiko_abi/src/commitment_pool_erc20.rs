pub use commitment_pool_erc20::*;
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
pub mod commitment_pool_erc20 {
    const _: () = {
        ::core::include_bytes!(
"../json/CommitmentPoolERC20.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers_core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_treeHeight"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_minRollupFee"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
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
                        name: ::std::borrow::ToOwned::to_owned("_settingsCenter"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AUDITOR_COUNT"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("AUDITOR_COUNT"),
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
                    ::std::borrow::ToOwned::to_owned("_pathIndices"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_pathIndices"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fullPath"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rollupSize"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("defaultMinRollupFee"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultMinRollupFee",
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
                    ::std::borrow::ToOwned::to_owned("enqueue"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("enqueue"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_request"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ICommitmentPool.CommitmentRequest",
                                        ),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllAuditorPublicKeys"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAllAuditorPublicKeys",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAuditorPublicKey"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAuditorPublicKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_index"),
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
                    ::std::borrow::ToOwned::to_owned("getCommitmentCount"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCommitmentCount"),
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
                    ::std::borrow::ToOwned::to_owned("getCommitmentIncludedCount"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCommitmentIncludedCount",
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
                    ::std::borrow::ToOwned::to_owned("getCommitmentQueuedCount"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCommitmentQueuedCount",
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
                    ::std::borrow::ToOwned::to_owned("getMinRollupFee"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMinRollupFee"),
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
                    ::std::borrow::ToOwned::to_owned("getNullifierCount"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNullifierCount"),
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
                    ::std::borrow::ToOwned::to_owned("getQueuedCommitments"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQueuedCommitments",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTreeCapacity"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTreeCapacity"),
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
                    ::std::borrow::ToOwned::to_owned("isHistoricCommitment"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isHistoricCommitment",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_commitment"),
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
                    ::std::borrow::ToOwned::to_owned("isKnownRoot"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isKnownRoot"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("root"),
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
                    ::std::borrow::ToOwned::to_owned("isSpentSerialNumber"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isSpentSerialNumber",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_serialNumber"),
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
                    ::std::borrow::ToOwned::to_owned("rollup"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rollup"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_request"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers_core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                2usize,
                                                            ),
                                                            ::ethers_core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                2usize,
                                                            ),
                                                        ],
                                                    ),
                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ICommitmentPool.RollupRequest",
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
                                        ::std::borrow::ToOwned::to_owned("contract MystikoSettings"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transact"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transact"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_request"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers_core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                2usize,
                                                            ),
                                                            ::ethers_core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                2usize,
                                                            ),
                                                        ],
                                                    ),
                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers_core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers_core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ICommitmentPool.TransactRequest",
                                        ),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_signature"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentIncluded"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CommitmentIncluded"),
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
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentQueued"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CommitmentQueued"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("commitment"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rollupFee"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("leafIndex"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("encryptedNote"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentSpent"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CommitmentSpent"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rootHash"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("serialNumber"),
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
                (
                    ::std::borrow::ToOwned::to_owned("EncryptedAuditorNote"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EncryptedAuditorNote",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("auditorPublicKey"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "encryptedAuditorNote",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("EncryptedAuditorNotes"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EncryptedAuditorNotes",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("notes"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("AssociatedPoolNotMatched"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AssociatedPoolNotMatched",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AuditorNotesLengthError"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AuditorNotesLengthError",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentHasBeenSubmitted"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CommitmentHasBeenSubmitted",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Duplicated"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Duplicated"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("param"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignature"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignature",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureLength"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignatureLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureS"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignatureS",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("IndexOutOfBound"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("IndexOutOfBound"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Invalid"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("param"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewRootIsDuplicated"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewRootIsDuplicated",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoteHasBeenSpent"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoteHasBeenSpent"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutputNotesLessThanThree"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OutputNotesLessThanThree",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReentrancyGuardReentrantCall"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReentrancyGuardReentrantCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RejectRelay"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RejectRelay"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RejectRollup"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RejectRollup"),
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
                    ::std::borrow::ToOwned::to_owned("RollupVerifierDisabled"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RollupVerifierDisabled",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rollupSize"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeCastOverflowedUintDowncast"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeCastOverflowedUintDowncast",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bits"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("TransactVerifierDisabled"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransactVerifierDisabled",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inputNumber"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("outputNumber"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TreeHeightLessThanZero"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TreeHeightLessThanZero",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TreeHeightOutOfBounds"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TreeHeightOutOfBounds",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TreeIsFull"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TreeIsFull"),
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
    pub static COMMITMENTPOOLERC20_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R`\0`\x04U`\0`\x05U`\0`\x06U4\x80\x15a\0\x1FW`\0\x80\xFD[P`@Qa<\xB18\x03\x80a<\xB1\x839\x81\x01`@\x81\x90Ra\0>\x91a\x07\x83V[\x81\x84\x84\x83`\x01`\0\x81\x90UP\x82`\xFF\x16`\0\x03a\0nW`@Qc,O)\xB1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\xFF\x84\x16\x1B`\x80Ra\0\x81\x83a\0\xE0V[`\x08\x81\x90U`\0\x90\x81R`\x07` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\t\x91\x90\x91U`\n\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x0B\x80T\x94\x90\x92\x16\x93\x16\x92\x90\x92\x17\x90\x91UPa\x07\xDE\x93PPPPV[`\0\x81`\xFF\x16`\0\x03a\x01\x14WP\x7F\t\xF6XEwu\x07O\xF4\xC8B\x03*^\xC2\xF1\x13L2xL\xCAY\xD5\x94\xCA\xAC\x8CP;y#\x91\x90PV[\x81`\xFF\x16`\x01\x03a\x01FWP\x7F\x1AwV\x9By\xCB|.\xAF\x93h\xDE\x9E;\x1E\xFC\n`ea\xE5\xAB)\x9C#74\x0F<\xDDWj\x91\x90PV[\x81`\xFF\x16`\x02\x03a\x01xWP\x7F\x11\x1B\xD0\0+\x1E;\x8F\x19x\xC92\xB5\xCC\xB2\xFA\xB8\x7F\xDB\n\xCA\xB2\xDD\xB2f\xF9\xA3F\xDC\xB1\x1E\xE1\x91\x90PV[\x81`\xFF\x16`\x03\x03a\x01\xAAWP\x7F\x04\x08h\xEA*6>\x05\xD7\xAED(\x0Fi\xB4\x9EB\x9A\xB0\x96\xCA(D\xEF\xE9\x1F\x8Dk=\xADW\xDD\x91\x90PV[\x81`\xFF\x16`\x04\x03a\x01\xDBWP~\xF8\xF6\x05\xC9,\x85\x02\xC8\xFE\x83\xBE\x1B\x83\xB2N?g1*8\x8F0\xCB\xAD\xB5\xDE\xE8\x97A7\xC3\x91\x90PV[\x81`\xFF\x16`\x05\x03a\x02\rWP\x7F\x1A\xDC\x04<\x99\xAD\xBC\x0C\x86\xA6\n6\xDB\x0Ff\x1E-\xD9o6\xED\xE3\"\xF9T8m\x895\xA0\xC5\xD9\x91\x90PV[\x81`\xFF\x16`\x06\x03a\x02?WP\x7F*\x1F\xED\xFAq\xDAr:\xC3\xE9\xB3\\\xEFu/\xA1\xB6G\xB2\xB77\xA2>\x91$\x1C\xB7\xBD\xC4\x19\xE3\xF4\x91\x90PV[\x81`\xFF\x16`\x07\x03a\x02qWP\x7F\x17\xFE\x19tT<LK\"\x8E\x12\x92\xF7\xE3 \r1C_\xC9\x10\xEEZ\x8C\\\xAF\xD3)\xCCK%k\x91\x90PV[\x81`\xFF\x16`\x08\x03a\x02\xA3WP\x7F\x0E\x84\xA5\x86\xEBc\xA0\xEE\xC0\xF1\xFEW\x85\x02$A\xF0\xE2\x9EJ\xE8Y\xC7\xCE\x1F_\xC8\x8AB\xAD.k\x91\x90PV[\x81`\xFF\x16`\t\x03a\x02\xD5WP\x7F\x19=\xEB\x90\x1Fn\xEB\x03.\x02\xE0\x93(\r\xB1~7=O\xF5+\xAF\xDA\x92\x15\xB4k\xB9\xB0\xA8o>\x91\x90PV[\x81`\xFF\x16`\n\x03a\x03\x07WP\x7F\x1D\x10\xCA{\x98V\x97\xCBQ\x95e\xA5\0l?D\xB0 \xB2\xED\xAB\x9Bt\"\xED\x15\xDC4S/\x94\x06\x91\x90PV[\x81`\xFF\x16`\x0B\x03a\x039WP\x7F\x1D\xC2\0v5UFzNX>\0\xBA\xDF\xDC\x1F\xB4\xD3\xD3\xF8\xF1\xCC\x81\xF3\x1F\xD2\xF8\xB3\x87w`\x81\x91\x90PV[\x81`\xFF\x16`\x0C\x03a\x03kWP\x7F =\xD1\x1F\xDB\xA0\xED\x13\xB2\x0C\xA2\xD6\x95/?\xEBta\x83j\x03Q.\x0C\xCC\xCE\x84g\xB3 \xB2\xF6\x91\x90PV[\x81`\xFF\x16`\r\x03a\x03\x9DWP\x7F\x05\x91\xAFd\xE6J>i\xCA\xF2>\xEE+\xDE\xA9\x08\x854:I\xF5G\xEE\x94d\xE9]\x8Dbg\xE4\xF7\x91\x90PV[\x81`\xFF\x16`\x0E\x03a\x03\xCFWP\x7F*\xF5r\xF1\x07z2\xF4m\xC8\xE3\x07\xD4<\x0F\xA6u;@\x0B!\x072Yv\xB8\xDFs\x80T_\xF6\x91\x90PV[\x81`\xFF\x16`\x0F\x03a\x04\x01WP\x7F\x042'\xAEKp\xB1\xAA\xCD\x04\xE3^j\xAE\xD7\xB5m\x91\"\x0C1\xE9z\xAFR\xE1*jV\x98NR\x91\x90PV[\x81`\xFF\x16`\x10\x03a\x043WP\x7F)v\xF1\xF6\xA9\x1D\x83\xD4\xC5(\xDA\xD6\x9E\xCEm=\x91\x93K\x0EVW\xE9\x15\xB3`\xC8\xC4\xC2\xFBT\xE6\x91\x90PV[\x81`\xFF\x16`\x11\x03a\x04dWP~\xE5\xC2Q\xC9\xE0\x93e\x8B\xE0\xCD\x1B\r\xF5[op\xF3\xD0\x91F\xC1\xC8\xB4!*M\xDC\xDEp\r\xBC\x91\x90PV[\x81`\xFF\x16`\x12\x03a\x04\x96WP\x7F\x02g\xCB\xBC\x1B\xC2\xF1\xC3\xE3\x07=.\xE6\r\xF8\xCC\x0B\xFE\xF3\x9F\xE3\xCE\xE75\xC9\xAD\\\x8A\xD3\0d\xD2\x91\x90PV[\x81`\xFF\x16`\x13\x03a\x04\xC8WP\x7F/5`W\xBCV\xF6}\xBF\x15\x9A\x0E\x895\x02*\xCD^\x98-\xCE\x9F@q\xAD\xC4>MW\xCE'\xE6\x91\x90PV[\x81`\xFF\x16`\x14\x03a\x04\xFAWP\x7F'=\xB6\x8FR\xF1*\x9D\x80\"\xAER@P\x06NB\xD4\xD1f\x1C\x9B\xCC\xE9X\xAC\xF8\x9B^\x8B\x12{\x91\x90PV[\x81`\xFF\x16`\x15\x03a\x05,WP\x7F\x04\x96\xA1\x8A\xD4\xCC\xA8\x1B|\x98\xCE\xB1\x97C\x9A\xD9%\xE0\xF7\xF6-i\xDF\xA4,\xF9WK\xE7\x7F\xE3\x0F\x91\x90PV[\x81`\xFF\x16`\x16\x03a\x05^WP\x7F$\xF8\x9A?\x94=B\x1B/:UKeE\x9FB\xB8 \xAC\t\xD6\xFD\x9Di=\xF5\xF8\xBAs*\xB5\x96\x91\x90PV[\x81`\xFF\x16`\x17\x03a\x05\x90WP\x7F\x1BU\xBF\xD7Q\xC6\x80}\xF3hv\xCD\xCEh\x03J\xB42\x10\xBE+\xC8\xAF\xA8\x04<\x7FB\x86\x04\xE7\xA7\x91\x90PV[\x81`\xFF\x16`\x18\x03a\x05\xC2WP\x7F\x16\xD6YZ9\x8C\xF2\x0F$\x89\xB9\x0E\x95\x81f\xF1\xE1\x9CS|\x0CF\xE9\xB8\xEATb9\x1E\xE8\xF1C\x91\x90PV[\x81`\xFF\x16`\x19\x03a\x05\xF4WP\x7F\x0F\x01D~\xF8\xF6!Y$t\xB6x\xED/\xC4\x04\xEB\xAF\"\xA6\xFC\xE1Sd\xBBAR\xA8\x8C\x116\x13\x91\x90PV[\x81`\xFF\x16`\x1A\x03a\x06&WP\x7F\x02l-\xFF\xEEH\xBA\xCB\xC9\xD2\x1C\xF9\n\xA7\xC6\xE5%\xAB\x01\xDBif\xA9\xE7\xE5;=?M\x1FZM\x91\x90PV[\x81`\xFF\x16`\x1B\x03a\x06XWP\x7F#O\xE9\x072yWE\xB2\xC5\x04\xC7\x91$*+\xB1\x93\xBA\xA1\xCB\xEA\xB5}\xB92Kk\xB9\x13H\x17\x91\x90PV[\x81`\xFF\x16`\x1C\x03a\x06\x8AWP\x7F#\xA8\xE0\xA7\xE6\t\x81\xC5.\xBBI\x8C&\r[\xEFM|e\x14]\x17\x12\x89\x96\xA7|3\xA3&*~\x91\x90PV[\x81`\xFF\x16`\x1D\x03a\x06\xBCWP\x7F$\xEEi\xD2VR\x10\xC7\x02\x7F\xF6\xFC&W\xED\x02\x92x\xBCy\xF4\x10w\xFE2\x81\xEA]]\x8E\x80\xF9\x91\x90PV[\x81`\xFF\x16`\x1E\x03a\x06\xEEWP\x7F\x1D\xE4\x02\xFA2F;\xB2\x91{s:\xEE\xF0\x19zI\xCA\xCA\xCD\x1F\xE8`\xC3\xAC\xC8\xCD;e\xA3\nh\x91\x90PV[\x81`\xFF\x16`\x1F\x03a\x07 WP\x7F\x1D0\x15\xA0\xF6\xA7\xB3\xF7T\x17\x1D\x05@b\x81\x07\xE5\x0E%\xDE\xBC\xEB\x16\xE0\xE3:\xE4 U\x01\x89m\x91\x90PV[\x81`\xFF\x16` \x03a\x07RWP\x7F&J&\x0594&G%h\x19\x04k\xAE\x05\xBE\xD9\x03\xA8\xD1\x9B<\x90C\x9Dg1}]\x88\x13\"\x91\x90PV[`@Qc\x97\x80\xF4)`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x80W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x07\x99W`\0\x80\xFD[\x84Q`\xFF\x81\x16\x81\x14a\x07\xAAW`\0\x80\xFD[` \x86\x01Q`@\x87\x01Q\x91\x95P\x93Pa\x07\xC2\x81a\x07kV[``\x86\x01Q\x90\x92Pa\x07\xD3\x81a\x07kV[\x93\x96\x92\x95P\x90\x93PPV[`\x80Qa4\xAAa\x08\x07`\09`\0\x81\x81a\x029\x01R\x81\x81a\x0E6\x01Ra\x17\xD2\x01Ra4\xAA`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x98W`\x005`\xE0\x1C\x80czU7D\x11a\0\xE3W\x80c\xB21l3\x11a\0\x8CW\x80c\xE0at\xE4\x11a\0fW\x80c\xE0at\xE4\x14a\x03VW\x80c\xE5\0\xF5\x04\x14a\x03iW\x80c\xF2\xDA\x1DA\x14a\x03qW`\0\x80\xFD[\x80c\xB21l3\x14a\x03+W\x80c\xC2\xD4\x16\x01\x14a\x034W\x80c\xC9#\x0C]\x14a\x03NW`\0\x80\xFD[\x80c\xA5\x92\xBDi\x11a\0\xBDW\x80c\xA5\x92\xBDi\x14a\x02\xF8W\x80c\xA6#*\x93\x14a\x03\0W\x80c\xB0\x88\x92\xD0\x14a\x03#W`\0\x80\xFD[\x80czU7D\x14a\x02\xD5W\x80c\x86j\xC6X\x14a\x02\xDDW\x80c\x87x\r\xF9\x14a\x02\xE5W`\0\x80\xFD[\x80cU]u\xF0\x11a\x01EW\x80cc\xBC}2\x11a\x01\x1FW\x80cc\xBC}2\x14a\x02\x9AW\x80cr\x08)q\x14a\x02\xAFW\x80cx\xD6\x0C\xD7\x14a\x02\xC2W`\0\x80\xFD[\x80cU]u\xF0\x14a\x02gW\x80cV\x88\x88\x1F\x14a\x02oW\x80cW\x06\0\x16\x14a\x02wW`\0\x80\xFD[\x80c;\xB8\xD1\xB4\x11a\x01vW\x80c;\xB8\xD1\xB4\x14a\x01\xF5W\x80c?\xE34z\x14a\x02(W\x80cHN\xB6R\x14a\x027W`\0\x80\xFD[\x80c\x14\xA7s}\x14a\x01\x9DW\x80c\x17m\xE7\xA8\x14a\x01\xB2W\x80c\x1B\xA4l\xFD\x14a\x01\xD0W[`\0\x80\xFD[a\x01\xB0a\x01\xAB6`\x04a)\x98V[a\x03\x84V[\0[a\x01\xBAa\t\xEDV[`@Qa\x01\xC7\x91\x90a*^V[`@Q\x80\x91\x03\x90\xF3[`\x0BT`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xC7V[a\x02\x18a\x02\x036`\x04a*qV[`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xC7V[`\0`@Qa\x01\xC7\x91\x90a*\xA0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q\x90\x81R` \x01a\x01\xC7V[`\x04Ta\x02YV[a\x02Ya\ndV[a\x02\x18a\x02\x856`\x04a*qV[`\0\x90\x81R`\x01` R`@\x90 T`\xFF\x16\x90V[a\x02\xA2a\nvV[`@Qa\x01\xC7\x91\x90a+\x04V[a\x01\xB0a\x02\xBD6`\x04a,\xC4V[a\n\xE8V[a\x01\xB0a\x02\xD06`\x04a.\xB7V[a\x17\x07V[`\x06Ta\x02YV[a\x02\xA2a\x18\xABV[a\x02Ya\x02\xF36`\x04a*qV[a\x19TV[a\x02Y`\x05\x81V[a\x02\x18a\x03\x0E6`\x04a*qV[`\0\x90\x81R`\x07` R`@\x90 T`\xFF\x16\x90V[a\x02Ya\x19\xC8V[a\x02Y`\tT\x81V[a\x03<a\x1AQV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xC7V[a\x01\xBAa\x1A\xBFV[`\nTa\x01\xDD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x05Ta\x02YV[a\x02Ya\x03\x7F6`\x04a/bV[a\x1B\tV[` \x81\x81\x01Q`@\x80Q`\xA0\x81\x01\x82R0\x81R3\x93\x81\x01\x93\x84Rc\xFF\xFF\xFF\xFF\x90\x92\x16\x82\x82\x01\x81\x81R`\x04\x80T``\x86\x01\x90\x81R`\x05T`\x80\x87\x01\x90\x81R`\nT\x95Qb\x03\x9B\x13`\xE1\x1B\x81R\x87Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x82\x01\x94\x90\x94R\x97Q\x83\x16`$\x89\x01R\x92Q`D\x88\x01RQ`d\x87\x01R\x90Q`\x84\x86\x01R\x90\x93\x91\x16\x90b\x076&\x90`\xA4\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04N\x91\x90a/\x95V[a\x04\x84W`@Q\x7F\xFFN4#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@\x80\x85\x01Q`\0\x90\x81R`\x07` R T`\xFF\x16\x15a\x04\xBAW`@Qc\xE2\xE1!\x03`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04T\x84` \x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x05\tW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[` \x84\x01Qa\x05\x1E\x90c\xFF\xFF\xFF\xFF\x16\x82a/\xB0V[\x15a\x05YW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01a\x05\0V[`\nT` \x85\x01Q`@Qc\x05\xAF\xD53`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c-~\xA9\x98\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD1\x91\x90a/\xD2V[\x90P\x80` \x01Qa\x06\x03W` \x85\x01Q`@Qc\xF5sZ_`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x05\0V[`\0a\x06\x13\x83\x87` \x01Qa\x1B\tV[\x90P`\0\x86` \x01Qc\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06:Wa\x06:a'\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06cW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x88` \x01Qc\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x07VW`\0a\x06\x88\x82\x88a0)V[`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x80\x83R`\x01\x90\x91\x01T\x92\x82\x01\x92\x90\x92R\x92\x93P\x90\x03a\x06\xD5W`@Qc\xD3H/{`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x01Q\x85\x84\x81Q\x81\x10a\x06\xECWa\x06\xECa0<V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x84a\x07\x08\x91\x90a0)V[`\0\x83\x81R`\x03` R`@\x80\x82 \x82\x81U`\x01\x01\x82\x90U\x83Q\x90Q\x92\x96P\x91\x7F\xFEk\t{F\xA7\x8E\x08Pj1C\xB63|%\x05\xBAw\xDFv\xFE\x05\xC3f:\x98s\x95\xD64\x13\x91\x90\xA2PP`\x01\x01a\x06jV[P\x87` \x01Qc\xFF\xFF\xFF\xFF\x16`\x04`\0\x82\x82Ta\x07s\x91\x90a0RV[\x92PP\x81\x90UP`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x83`@Q` \x01a\x07\xAE\x91\x90a0eV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x07\xD1\x91\x90a/\xB0V[\x90P\x80\x89``\x01Q\x14a\x08'W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01R\x7FleafHash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R`\0\x91` \x82\x01`\x80\x806\x837\x01\x90PP\x90P`\x08T\x81`\0\x81Q\x81\x10a\x08`Wa\x08`a0<V[` \x02` \x01\x01\x81\x81RPP\x89`@\x01Q\x81`\x01\x81Q\x81\x10a\x08\x84Wa\x08\x84a0<V[` \x02` \x01\x01\x81\x81RPP\x81\x81`\x02\x81Q\x81\x10a\x08\xA4Wa\x08\xA4a0<V[` \x02` \x01\x01\x81\x81RPP\x84\x81`\x03\x81Q\x81\x10a\x08\xC4Wa\x08\xC4a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x85Q\x8AQ`@Qc\xC9AvG`\xE0\x1B\x81R`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xC9AvG\x91a\t\x02\x91\x90\x86\x90`\x04\x01a0\xBEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tE\x91\x90a/\x95V[\x90P\x80a\t\x95W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01R\x7Fproof\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[\x8A` \x01Qc\xFF\xFF\xFF\xFF\x16`\x05`\0\x82\x82Ta\t\xB1\x91\x90a0)V[\x90\x91UPP`@\x80\x8C\x01Q`\x08\x81\x90U`\0\x90\x81R`\x07` R \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\xE0\x84a\x1B\x92V[PPPPPPPPPPPV[`\x0BT`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n_\x91\x90\x81\x01\x90a10V[\x90P\x90V[`\0`\x04T`\x05Ta\n_\x91\x90a0)V[`\nT`@\x80Qc0D\xB7\x9B`\xE2\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xC1\x12\xDEl\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n_\x91\x90\x81\x01\x90a1\xA7V[a\n\xF0a\x1B\xACV[`\0a\x0B\0\x83`@\x01QQa\x1B\xD6V[\x90P`\0a\x0B\x12\x84`\xE0\x01QQa\x1B\xD6V[`\nT`@Qc\x1D\xCD\xF71`\xE3\x1B\x81R0`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xEEo\xB9\x88\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x7F\x91\x90a/\x95V[\x80\x15a\x0B\x90WPc\xFF\xFF\xFF\xFF\x81\x16\x15\x15[\x15a\x0B\xBEW`@Qch\x97ZG`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x05\0V[`\xC0\x84\x01Q\x15a\x0CsW`@\x80Q\x80\x82\x01\x82R0\x81R3` \x82\x01\x90\x81R`\nT\x92Qc^\xE3l\xE9`\xE0\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R\x91Q\x82\x16`$\x82\x01R\x91\x92\x16\x90c^\xE3l\xE9\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CT\x91\x90a/\x95V[a\x0CqW`@Qc2J\xF8\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[`\nT`@Qc\x85\xE8a\xEB`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x85\x16`\x04\x83\x01R\x83\x16`$\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x85\xE8a\xEB\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xED\x91\x90a/\xD2V[\x90P\x80` \x01Qa\r!W`@Qch\x97ZG`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x85\x16`\x04\x83\x01R\x83\x16`$\x82\x01R`D\x01a\x05\0V[\x82c\xFF\xFF\xFF\xFF\x16\x85``\x01QQ\x14a\r|W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FsigHashes length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[\x81c\xFF\xFF\xFF\xFF\x16\x85a\x01\0\x01QQ\x14a\r\xD8W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FoutRollupFees length\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[\x81c\xFF\xFF\xFF\xFF\x16\x85a\x01`\x01QQ\x14a\x0E4W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FoutEncryptedNotes length\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82c\xFF\xFF\xFF\xFF\x16`\x04T`\x05Ta\x0El\x91\x90a0)V[a\x0Ev\x91\x90a0)V[\x11\x15a\x0E\x95W`@Qc;\\\xCBC`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT`@Qc\xDFY/}`\xE0\x1B\x81R2`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x01\x91\x90a/\x95V[\x15a\x0F\x1FW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nTa\x01 \x86\x01Q`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FnW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x92\x91\x90a/\x95V[\x15a\x0F\xB0W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xC1`\x05c\xFF\xFF\xFF\xFF\x85\x16a23V[\x85a\x01\xA0\x01QQ\x14a\x0F\xE6W`@Qc:\xCFH\xBB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xF0\x85\x85a\x1C\x0BV[`\0a\x0F\xFD\x84`\x02a2JV[c\xFF\xFF\xFF\xFF\x16\x90P`\0a\x10\x12\x85`\x02a2JV[a\x10\x1D\x90`\x04a2pV[c\xFF\xFF\xFF\xFF\x16\x90P`\0a\x102\x85`\x02a2JV[a\x10B\x90c\xFF\xFF\xFF\xFF\x16\x83a0)V[\x90P`\0`\x05a\x10S\x88`\x02a2pV[c\xFF\xFF\xFF\xFF\x16a\x10c\x91\x90a23V[a\x10n\x83`\x02a0)V[a\x10x\x91\x90a0)V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x90Wa\x10\x90a'\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xB9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x80\x8B\x01Q`\0\x90\x81R`\x07\x90\x91R`@\x90 T\x90\x91P`\xFF\x16a\x11\x0BW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R`\x04\x01a\x05\0\x90` \x80\x82R`\x04\x90\x82\x01Rc\x1C\x9B\xDB\xDD`\xE2\x1B`@\x82\x01R``\x01\x90V[\x88` \x01Q\x81`\0\x81Q\x81\x10a\x11#Wa\x11#a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0a\x11;\x88`\x01a2pV[c\xFF\xFF\xFF\xFF\x16\x90P`\0[\x88c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x12=W`\0\x8B`@\x01Q\x82\x81Q\x81\x10a\x11lWa\x11la0<V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\0\x81\x81R`\x02\x90\x92R`@\x90\x91 T\x90\x91P`\xFF\x16\x15a\x11\xC5W`@Q\x7F\xFFUn \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x84a\x11\xD2\x84`\x01a0)V[\x81Q\x81\x10a\x11\xE2Wa\x11\xE2a0<V[` \x02` \x01\x01\x81\x81RPP\x8B``\x01Q\x82\x81Q\x81\x10a\x12\x04Wa\x12\x04a0<V[` \x02` \x01\x01Q\x84\x84\x84a\x12\x19\x91\x90a0)V[\x81Q\x81\x10a\x12)Wa\x12)a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x11FV[P`\x80\x8A\x01Q\x82a\x12O\x87`\x01a0)V[\x81Q\x81\x10a\x12_Wa\x12_a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xA0\x8A\x01Q\x82a\x12{\x87`\x02a0)V[\x81Q\x81\x10a\x12\x8BWa\x12\x8Ba0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xC0\x8A\x01Q\x82a\x12\xA7\x87`\x03a0)V[\x81Q\x81\x10a\x12\xB7Wa\x12\xB7a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0a\x12\xD4c\xFF\xFF\xFF\xFF\x89\x16\x86a0)V[\x90P`\0a\x12\xE0a\x19\xC8V[\x90P`\0[\x89c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x14NW`\x01`\0\x8E`\xE0\x01Q\x83\x81Q\x81\x10a\x13\rWa\x13\ra0<V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15a\x13yW`@Qc\xBE\xE3a\x11`\xE0\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7Fcommitment\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[\x81\x8Da\x01\0\x01Q\x82\x81Q\x81\x10a\x13\x91Wa\x13\x91a0<V[` \x02` \x01\x01Q\x10\x15a\x13\xB8W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x8C`\xE0\x01Q\x81\x81Q\x81\x10a\x13\xCEWa\x13\xCEa0<V[` \x02` \x01\x01Q\x85\x88\x83a\x13\xE3\x91\x90a0)V[\x81Q\x81\x10a\x13\xF3Wa\x13\xF3a0<V[` \x02` \x01\x01\x81\x81RPP\x8Ca\x01\0\x01Q\x81\x81Q\x81\x10a\x14\x16Wa\x14\x16a0<V[` \x02` \x01\x01Q\x85\x84\x83a\x14+\x91\x90a0)V[\x81Q\x81\x10a\x14;Wa\x14;a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x12\xE5V[Pa\x14Z\x8C\x85\x87a\x1D\xE8V[\x87Q\x8CQ`@Qc\xC9AvG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xC9AvG\x91a\x14\x8B\x91\x88\x90`\x04\x01a0\xBEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xCE\x91\x90a/\x95V[a\x15\x1BW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Ftransact proof\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[`\0[\x8Ac\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x15\xE5W`\x01`\x02`\0\x8F`@\x01Q\x84\x81Q\x81\x10a\x15HWa\x15Ha0<V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\x06`\0\x82\x82Ta\x15\x88\x91\x90a0)V[\x90\x91UPP`@\x8D\x01Q\x80Q\x82\x90\x81\x10a\x15\xA4Wa\x15\xA4a0<V[` \x02` \x01\x01Q\x8D` \x01Q\x7F<#r\xABa0\x81{\xD6\xB8\xFCm\xBA\xEC\xAE\x94~\x84 \x1BIS]5\x8D\xEB\xAAl4\xC2>\xCF`@Q`@Q\x80\x91\x03\x90\xA3`\x01\x01a\x15\x1EV[P`\0[\x89c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x16\xABW`\x01\x80`\0\x8F`\xE0\x01Q\x84\x81Q\x81\x10a\x16\x12Wa\x16\x12a0<V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x16\xA3\x8D`\xE0\x01Q\x82\x81Q\x81\x10a\x16XWa\x16Xa0<V[` \x02` \x01\x01Q\x8Ea\x01\0\x01Q\x83\x81Q\x81\x10a\x16wWa\x16wa0<V[` \x02` \x01\x01Q\x8Fa\x01`\x01Q\x84\x81Q\x81\x10a\x16\x96Wa\x16\x96a0<V[` \x02` \x01\x01Qa RV[`\x01\x01a\x15\xE9V[P`\xA0\x8C\x01Q\x15a\x16\xC9Wa\x16\xC9\x8Ca\x01 \x01Q\x8D`\xA0\x01Qa \xF1V[`\xC0\x8C\x01Q\x15a\x16\xE6Wa\x16\xE6\x8Ca\x01@\x01Q\x8D`\xC0\x01Qa \xF1V[a\x16\xEF\x8Ca!\x08V[PPPPPPPPPPa\x17\x03`\x01`\0UV[PPV[`\nT`@QcA\xFBiy`\xE0\x1B\x81R3`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cA\xFBiy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17t\x91\x90a2\x8CV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x160\x14a\x17\x9FW`@QcS5\xA0E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x17\xA9a\x19\xC8V[\x90P\x80\x84``\x01Q\x10\x15a\x17\xD0W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04T`\x05Ta\x18\x01\x91\x90a0)V[\x10a\x18\x1FW`@Qc;\\\xCBC`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x85\x01Q`\0\x90\x81R`\x01\x90\x91R`@\x90 T`\xFF\x16\x15a\x18UW`@Qc\xE3\x8C\xD1M`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x85\x01\x80Q`\0\x90\x81R`\x01\x92\x83\x90R`@\x90 \x80T`\xFF\x19\x16\x90\x92\x17\x90\x91UQ``\x85\x01Q`\x80\x86\x01Qa\x18\x8D\x92\x91\x90a RV[`@\x84\x01Q\x15a\x18\xA5Wa\x18\xA5\x83\x85`@\x01Qa \xF1V[PPPPV[```\0`\x04Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\xCAWa\x18\xCAa'\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xF3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\x04T\x81\x10\x15a\x19NW`\0\x81`\x05Ta\x19\x13\x91\x90a0)V[`\0\x81\x81R`\x03` R`@\x90 T\x84Q\x91\x92P\x90\x84\x90\x84\x90\x81\x10a\x19:Wa\x19:a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x18\xF9V[P\x91\x90PV[`\nT`@Qc\xDB\xDA\x08)`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDB\xDA\x08)\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xC2\x91\x90a2\xA9V[\x92\x91PPV[`\nT`@Qc\xC3\xC4\xBD\x0B`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC3\xC4\xBD\x0B\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A9\x91\x90a2\xA9V[\x90P\x80\x15a\x1AGW\x80a\x1AKV[`\tT[\x91PP\x90V[`\x0BT`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1<\xE5g\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1A\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n_\x91\x90a2\xC2V[`\x0BT`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n7W=`\0\x80>=`\0\xFD[`\0a\x01\0\x82c\xFF\xFF\xFF\xFF\x16\x10a\x1B/W`\x08\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x08\x83\x90\x1C\x92P[`\x10\x82c\xFF\xFF\xFF\xFF\x16\x10a\x1BRW`\x04\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x04\x83\x90\x1C\x92P[`\x04\x82c\xFF\xFF\xFF\xFF\x16\x10a\x1BuW`\x02\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x02\x83\x90\x1C\x92P[`\x02\x82c\xFF\xFF\xFF\xFF\x16\x10a\x1B\x8BW`\x01\x83\x90\x1C\x92P[P\x90\x91\x90PV[`\x0BTa\x1B\xA9\x90`\x01`\x01`\xA0\x1B\x03\x163\x83a#\x9CV[PV[`\x02`\0T\x03a\x1B\xCFW`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x1C\x07W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05\0V[P\x90V[a\x01`\x82\x01QQ`\x03\x81\x10a\x1C3W`@Qc?\xB1\x94]`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81`\0\x03a\x1C\x8AWa\x01 \x84\x01Qa\x01@\x85\x01Q`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x81\x16` \x83\x01R\x91\x90\x92\x1B\x16`4\x82\x01R`H\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x1DAV[\x81`\x01\x03a\x1C\xD0W\x83a\x01 \x01Q\x84a\x01@\x01Q\x85a\x01`\x01Q`\0\x81Q\x81\x10a\x1C\xB6Wa\x1C\xB6a0<V[` \x02` \x01\x01Q`@Q` \x01a\x1Ct\x93\x92\x91\x90a2\xE5V[\x83a\x01 \x01Q\x84a\x01@\x01Q\x85a\x01`\x01Q`\0\x81Q\x81\x10a\x1C\xF4Wa\x1C\xF4a0<V[` \x02` \x01\x01Q\x86a\x01`\x01Q`\x01\x81Q\x81\x10a\x1D\x14Wa\x1D\x14a0<V[` \x02` \x01\x01Q`@Q` \x01a\x1D/\x94\x93\x92\x91\x90a36V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[\x80Q` \x82\x01 \x7F\x19Ethereum Signed Message:\n32\0\0\0\0`\0\x90\x81R`\x1C\x91\x90\x91R`<\x90 a\x1D\x82\x81\x85a$\x08V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x1B\x85`\x80\x01Q\x14a\x1D\xE1W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7Fsignature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[PPPPPV[a\x01\x80\x83\x01Q`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\xFF\x83\x90\x1C\x80\x83R`\x01`\x01`\xFF\x1B\x03\x90\x93\x16\x90\x82\x01R\x83Q\x90\x91\x90\x84\x90\x84\x90\x81\x10a\x1E8Wa\x1E8a0<V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x83\x83`\x01a\x1EW\x91\x90a0)V[\x81Q\x81\x10a\x1EgWa\x1Ega0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\nT`@\x80Qc0D\xB7\x9B`\xE2\x1B\x81R\x90Q`\x05\x92`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC1\x12\xDEl\x91`\x04\x80\x82\x01\x92\x86\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\xE9\x91\x90\x81\x01\x90a1\xA7V[\x90P`\0a\x1E\xF8\x85`\x02a0)V[\x90P`\0a\x1F\x06\x84\x83a0)V[\x90P`\0a\x1F\x15\x85`\x02a23V[a\x1F\x1F\x90\x84a0)V[\x90P`\0[\x85\x81\x10\x15a\x1F\xE5W`\0a\x1F\x83\x86\x83\x81Q\x81\x10a\x1FCWa\x1FCa0<V[` \x02` \x01\x01Q`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\xFF\x83\x90\x1C\x82R`\x01`\x01`\xFF\x1B\x03\x90\x92\x16\x91\x81\x01\x91\x90\x91R\x90V[\x80Q\x90\x91P\x8Aa\x1F\x93\x84\x88a0)V[\x81Q\x81\x10a\x1F\xA3Wa\x1F\xA3a0<V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x8A\x83\x86a\x1F\xC1\x91\x90a0)V[\x81Q\x81\x10a\x1F\xD1Wa\x1F\xD1a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x1F$V[P`\0[\x89a\x01\xA0\x01QQ\x81\x10\x15a FW\x89a\x01\xA0\x01Q\x81\x81Q\x81\x10a \x0EWa \x0Ea0<V[` \x02` \x01\x01Q\x89\x82\x84a #\x91\x90a0)V[\x81Q\x81\x10a 3Wa 3a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1F\xE9V[PPPPPPPPPPV[`\0`\x05T`\x04Ta d\x91\x90a0)V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R`\0\x85\x81R`\x03\x90\x92R\x92\x81 \x91Q\x82U\x91Q`\x01\x91\x82\x01U`\x04\x80T\x93\x94P\x90\x92\x90\x91\x90a \xA8\x90\x84\x90a0)V[\x92PP\x81\x90UP\x83\x7F\xF53\xF9pZ\xACP \xE2\x16\x95\xEA5S\xAC{h\x81\x07\r+i\0\xAB+\x1E0P0K[\xF9\x84\x83\x85`@Qa \xE3\x93\x92\x91\x90a3\x9DV[`@Q\x80\x91\x03\x90\xA2PPPPV[`\x0BTa\x17\x03\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83a#\x9CV[`\nT`@\x80Qc0D\xB7\x9B`\xE2\x1B\x81R\x90Q`\x05\x92`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC1\x12\xDEl\x91`\x04\x80\x82\x01\x92\x86\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\x7F\x91\x90\x81\x01\x90a1\xA7V[\x90P`\0\x82\x84`@\x01QQa!\x94\x91\x90a23V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xB1Wa!\xB1a'\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x10W\x81` \x01[a!\xFD`@Q\x80``\x01`@R\x80`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a!\xCFW\x90P[P\x90P`\0\x80[\x86`@\x01QQ\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a#\\W`\0[\x86\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a#IW\x80c\xFF\xFF\xFF\xFF\x16` \x83c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B\x17\x84\x84\x81Q\x81\x10a\"jWa\"ja0<V[` \x90\x81\x02\x91\x90\x91\x01\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90R\x85Q\x86\x90c\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x10a\"\x9DWa\"\x9Da0<V[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a\"\xB7Wa\"\xB7a0<V[` \x02` \x01\x01Q` \x01\x81\x81RPP\x87a\x01\xA0\x01Q\x81c\xFF\xFF\xFF\xFF\x16\x88\x84c\xFF\xFF\xFF\xFF\x16a\"\xE6\x91\x90a23V[a\"\xF0\x91\x90a0)V[\x81Q\x81\x10a#\0Wa#\0a0<V[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a#\x1AWa#\x1Aa0<V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@\x01R\x82a#3\x81a3\xBCV[\x93PP\x80\x80a#A\x90a3\xD5V[\x91PPa\"-V[P\x80a#T\x81a3\xD5V[\x91PPa\"\x17V[P\x7Fj\xF0~\xBC\xB391\xAB\xD3H\xE7\x85\xF7\x0Cm%\x93\x90\xCFy\x91\x91\xF8\xC9\x91$$[\x96\xE1\xA6\x13\x82`@Qa#\x8C\x91\x90a3\xFAV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra$\x03\x90\x84\x90a$2V[PPPV[`\0\x80`\0\x80a$\x18\x86\x86a$\x95V[\x92P\x92P\x92Pa$(\x82\x82a$\xE2V[P\x90\x94\x93PPPPV[`\0a$G`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a%\x9BV[\x90P\x80Q`\0\x14\x15\x80\x15a$lWP\x80\x80` \x01\x90Q\x81\x01\x90a$j\x91\x90a/\x95V[\x15[\x15a$\x03W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x05\0V[`\0\x80`\0\x83Q`A\x03a$\xCFW` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa$\xC1\x88\x82\x85\x85a%\xB0V[\x95P\x95P\x95PPPPa$\xDBV[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a$\xF6Wa$\xF6a*\x8AV[\x03a$\xFFWPPV[`\x01\x82`\x03\x81\x11\x15a%\x13Wa%\x13a*\x8AV[\x03a%1W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a%EWa%Ea*\x8AV[\x03a%fW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05\0V[`\x03\x82`\x03\x81\x11\x15a%zWa%za*\x8AV[\x03a\x17\x03W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05\0V[``a%\xA9\x83\x83`\0a&\x7FV[\x93\x92PPPV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a%\xEBWP`\0\x91P`\x03\x90P\x82a&uV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a&?W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a&kWP`\0\x92P`\x01\x91P\x82\x90Pa&uV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x81G\x10\x15a&\xA4W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x05\0V[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa&\xC0\x91\x90a4XV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a&\xFDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'\x02V[``\x91P[P\x91P\x91Pa'\x12\x86\x83\x83a'\x1CV[\x96\x95PPPPPPV[``\x82a'1Wa',\x82a'xV[a%\xA9V[\x81Q\x15\x80\x15a'HWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a'qW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x05\0V[P\x80a%\xA9V[\x80Q\x15a'\x88W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\xDAWa'\xDAa'\xA1V[`@R\x90V[`@Qa\x01\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\xDAWa'\xDAa'\xA1V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\xDAWa'\xDAa'\xA1V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(PWa(Pa'\xA1V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a(jW`\0\x80\xFD[a(ra'\xB7V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a(\x9AW`\0\x80\xFD[a(\xA2a'\xB7V[\x80`@\x84\x01\x85\x81\x11\x15a(\xB4W`\0\x80\xFD[\x84[\x81\x81\x10\x15a(\xCEW\x805\x84R` \x93\x84\x01\x93\x01a(\xB6V[P\x90\x95\x94PPPPPV[`\0\x81\x83\x03a\x01\0\x81\x12\x15a(\xEDW`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\x10Wa)\x10a'\xA1V[`@R\x91P\x81a) \x85\x85a(XV[\x81R`\x80`?\x19\x83\x01\x12\x15a)4W`\0\x80\xFD[a)<a'\xB7V[\x91Pa)K\x85`@\x86\x01a(\x89V[\x82Ra)Z\x85`\x80\x86\x01a(\x89V[` \x83\x01R\x81` \x82\x01Ra)r\x85`\xC0\x86\x01a(XV[`@\x82\x01RPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a)\x93W`\0\x80\xFD[\x91\x90PV[`\0a\x01`\x82\x84\x03\x12\x80\x15a)\xACW`\0\x80\xFD[P`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\xD0Wa)\xD0a'\xA1V[`@Ra)\xDD\x84\x84a(\xD9V[\x81Ra)\xECa\x01\0\x84\x01a)\x7FV[` \x82\x01Ra\x01 \x83\x015`@\x82\x01Ra\x01@\x90\x92\x015``\x83\x01RP\x91\x90PV[`\0[\x83\x81\x10\x15a*)W\x81\x81\x01Q\x83\x82\x01R` \x01a*\x11V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra*J\x81` \x86\x01` \x86\x01a*\x0EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a%\xA9` \x83\x01\x84a*2V[`\0` \x82\x84\x03\x12\x15a*\x83W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a*\xC2WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15a*\xFAW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a*\xDCV[P\x93\x94\x93PPPPV[` \x81R`\0a%\xA9` \x83\x01\x84a*\xC8V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a+1Wa+1a'\xA1V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a+LW`\0\x80\xFD[\x815a+_a+Z\x82a+\x17V[a('V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a+\x81W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a+\x9EW\x805\x83R` \x92\x83\x01\x92\x01a+\x86V[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xA9W`\0\x80\xFD[\x805a)\x93\x81a+\xA8V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a+\xE2Wa+\xE2a'\xA1V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a,\x01W`\0\x80\xFD[\x815a,\x0Fa+Z\x82a+\xC8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a,$W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a,RW`\0\x80\xFD[\x815a,`a+Z\x82a+\x17V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a,\x82W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a+\x9EW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xA6W`\0\x80\xFD[a,\xB5\x88` \x83\x8A\x01\x01a+\xF0V[\x84RP` \x92\x83\x01\x92\x01a,\x87V[`\0\x80`@\x83\x85\x03\x12\x15a,\xD7W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xEEW`\0\x80\xFD[\x83\x01a\x02\xA0\x81\x86\x03\x12\x15a-\x01W`\0\x80\xFD[a-\ta'\xE0V[a-\x13\x86\x83a(\xD9V[\x81Ra\x01\0\x82\x015` \x82\x01Ra\x01 \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-;W`\0\x80\xFD[a-G\x87\x82\x85\x01a+;V[`@\x83\x01RPa\x01@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-hW`\0\x80\xFD[a-t\x87\x82\x85\x01a+;V[``\x83\x01RPa\x01`\x82\x015`\x80\x82\x01Ra\x01\x80\x82\x015`\xA0\x82\x01Ra\x01\xA0\x82\x015`\xC0\x82\x01Ra\x01\xC0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xB6W`\0\x80\xFD[a-\xC2\x87\x82\x85\x01a+;V[`\xE0\x83\x01RPa\x01\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xE3W`\0\x80\xFD[a-\xEF\x87\x82\x85\x01a+;V[a\x01\0\x83\x01RPa.\x03a\x02\0\x83\x01a+\xBDV[a\x01 \x82\x01Ra.\x16a\x02 \x83\x01a+\xBDV[a\x01@\x82\x01Ra\x02@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.7W`\0\x80\xFD[a.C\x87\x82\x85\x01a,AV[a\x01`\x83\x01RPa\x02`\x82\x015a\x01\x80\x82\x01Ra\x02\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.qW`\0\x80\xFD[a.}\x87\x82\x85\x01a+;V[a\x01\xA0\x83\x01RP\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xA1W`\0\x80\xFD[a.\xAD\x85\x82\x86\x01a+\xF0V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a.\xCAW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xE1W`\0\x80\xFD[\x83\x01`\xA0\x81\x86\x03\x12\x15a.\xF3W`\0\x80\xFD[a.\xFBa(\x04V[\x815\x81R` \x80\x83\x015\x90\x82\x01R`@\x80\x83\x015\x90\x82\x01R``\x80\x83\x015\x90\x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/7W`\0\x80\xFD[a/C\x87\x82\x85\x01a+\xF0V[`\x80\x83\x01RP\x92Pa/Y\x90P` \x84\x01a+\xBDV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a/uW`\0\x80\xFD[\x825\x91Pa/Y` \x84\x01a)\x7FV[\x80Q\x80\x15\x15\x81\x14a)\x93W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a/\xA7W`\0\x80\xFD[a%\xA9\x82a/\x85V[`\0\x82a/\xCDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0`@\x82\x84\x03\x12\x80\x15a/\xE5W`\0\x80\xFD[Pa/\xEEa'\xB7V[\x82Qa/\xF9\x81a+\xA8V[\x81Ra0\x07` \x84\x01a/\x85V[` \x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x19\xC2Wa\x19\xC2a0\x13V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x19\xC2Wa\x19\xC2a0\x13V[\x81Q`\0\x90\x82\x90` \x85\x01\x83[\x82\x81\x10\x15a0\x90W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a0rV[P\x91\x95\x94PPPPPV[\x80`\0[`\x02\x81\x10\x15a\x18\xA5W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a0\x9FV[a0\xD3\x81\x84Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\0` \x84\x01Qa0\xE8`@\x84\x01\x82Qa0\x9BV[` \x01Qa0\xF9`\x80\x84\x01\x82a0\x9BV[P`@\x84\x01Q\x80Q`\xC0\x84\x01R` \x81\x01Q`\xE0\x84\x01RPa\x01 a\x01\0\x83\x01Ra1(a\x01 \x83\x01\x84a*\xC8V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a1BW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1YW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a1jW`\0\x80\xFD[\x80Qa1xa+Z\x82a+\xC8V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a1\x8DW`\0\x80\xFD[a1\x9E\x82` \x83\x01` \x86\x01a*\x0EV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a1\xB9W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xD0W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a1\xE1W`\0\x80\xFD[\x80Qa1\xEFa+Z\x82a+\x17V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a2\x11W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a'\x12W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a2\x18V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x19\xC2Wa\x19\xC2a0\x13V[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a2iWa2ia0\x13V[P\x92\x91PPV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x19\xC2Wa\x19\xC2a0\x13V[`\0` \x82\x84\x03\x12\x15a2\x9EW`\0\x80\xFD[\x81Qa%\xA9\x81a+\xA8V[`\0` \x82\x84\x03\x12\x15a2\xBBW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a2\xD4W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a%\xA9W`\0\x80\xFD[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x84``\x1B\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83``\x1B\x16`\x14\x82\x01R`\0\x82Qa3'\x81`(\x85\x01` \x87\x01a*\x0EV[\x91\x90\x91\x01`(\x01\x94\x93PPPPV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x85``\x1B\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x84``\x1B\x16`\x14\x82\x01R`\0\x83Qa3x\x81`(\x85\x01` \x88\x01a*\x0EV[\x83Q\x90\x83\x01\x90a3\x8F\x81`(\x84\x01` \x88\x01a*\x0EV[\x01`(\x01\x96\x95PPPPPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\0a1\x9E``\x83\x01\x84a*2V[`\0`\x01\x82\x01a3\xCEWa3\xCEa0\x13V[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a3\xF1Wa3\xF1a0\x13V[`\x01\x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\xCEW\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01RP``\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa4\x14V[`\0\x82Qa4j\x81\x84` \x87\x01a*\x0EV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x950D\x0F,V\xB6\xAFU\x16\xF4\x81\xDA\x89\xDB\xB6@\x15\xD2[\n\x9A\xB8\x8E\xFD\xDF\xD4?\xF6OB\xD3dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static COMMITMENTPOOLERC20_BYTECODE: ::ethers_core::types::Bytes = ::ethers_core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x98W`\x005`\xE0\x1C\x80czU7D\x11a\0\xE3W\x80c\xB21l3\x11a\0\x8CW\x80c\xE0at\xE4\x11a\0fW\x80c\xE0at\xE4\x14a\x03VW\x80c\xE5\0\xF5\x04\x14a\x03iW\x80c\xF2\xDA\x1DA\x14a\x03qW`\0\x80\xFD[\x80c\xB21l3\x14a\x03+W\x80c\xC2\xD4\x16\x01\x14a\x034W\x80c\xC9#\x0C]\x14a\x03NW`\0\x80\xFD[\x80c\xA5\x92\xBDi\x11a\0\xBDW\x80c\xA5\x92\xBDi\x14a\x02\xF8W\x80c\xA6#*\x93\x14a\x03\0W\x80c\xB0\x88\x92\xD0\x14a\x03#W`\0\x80\xFD[\x80czU7D\x14a\x02\xD5W\x80c\x86j\xC6X\x14a\x02\xDDW\x80c\x87x\r\xF9\x14a\x02\xE5W`\0\x80\xFD[\x80cU]u\xF0\x11a\x01EW\x80cc\xBC}2\x11a\x01\x1FW\x80cc\xBC}2\x14a\x02\x9AW\x80cr\x08)q\x14a\x02\xAFW\x80cx\xD6\x0C\xD7\x14a\x02\xC2W`\0\x80\xFD[\x80cU]u\xF0\x14a\x02gW\x80cV\x88\x88\x1F\x14a\x02oW\x80cW\x06\0\x16\x14a\x02wW`\0\x80\xFD[\x80c;\xB8\xD1\xB4\x11a\x01vW\x80c;\xB8\xD1\xB4\x14a\x01\xF5W\x80c?\xE34z\x14a\x02(W\x80cHN\xB6R\x14a\x027W`\0\x80\xFD[\x80c\x14\xA7s}\x14a\x01\x9DW\x80c\x17m\xE7\xA8\x14a\x01\xB2W\x80c\x1B\xA4l\xFD\x14a\x01\xD0W[`\0\x80\xFD[a\x01\xB0a\x01\xAB6`\x04a)\x98V[a\x03\x84V[\0[a\x01\xBAa\t\xEDV[`@Qa\x01\xC7\x91\x90a*^V[`@Q\x80\x91\x03\x90\xF3[`\x0BT`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xC7V[a\x02\x18a\x02\x036`\x04a*qV[`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xC7V[`\0`@Qa\x01\xC7\x91\x90a*\xA0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q\x90\x81R` \x01a\x01\xC7V[`\x04Ta\x02YV[a\x02Ya\ndV[a\x02\x18a\x02\x856`\x04a*qV[`\0\x90\x81R`\x01` R`@\x90 T`\xFF\x16\x90V[a\x02\xA2a\nvV[`@Qa\x01\xC7\x91\x90a+\x04V[a\x01\xB0a\x02\xBD6`\x04a,\xC4V[a\n\xE8V[a\x01\xB0a\x02\xD06`\x04a.\xB7V[a\x17\x07V[`\x06Ta\x02YV[a\x02\xA2a\x18\xABV[a\x02Ya\x02\xF36`\x04a*qV[a\x19TV[a\x02Y`\x05\x81V[a\x02\x18a\x03\x0E6`\x04a*qV[`\0\x90\x81R`\x07` R`@\x90 T`\xFF\x16\x90V[a\x02Ya\x19\xC8V[a\x02Y`\tT\x81V[a\x03<a\x1AQV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xC7V[a\x01\xBAa\x1A\xBFV[`\nTa\x01\xDD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x05Ta\x02YV[a\x02Ya\x03\x7F6`\x04a/bV[a\x1B\tV[` \x81\x81\x01Q`@\x80Q`\xA0\x81\x01\x82R0\x81R3\x93\x81\x01\x93\x84Rc\xFF\xFF\xFF\xFF\x90\x92\x16\x82\x82\x01\x81\x81R`\x04\x80T``\x86\x01\x90\x81R`\x05T`\x80\x87\x01\x90\x81R`\nT\x95Qb\x03\x9B\x13`\xE1\x1B\x81R\x87Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x82\x01\x94\x90\x94R\x97Q\x83\x16`$\x89\x01R\x92Q`D\x88\x01RQ`d\x87\x01R\x90Q`\x84\x86\x01R\x90\x93\x91\x16\x90b\x076&\x90`\xA4\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04N\x91\x90a/\x95V[a\x04\x84W`@Q\x7F\xFFN4#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@\x80\x85\x01Q`\0\x90\x81R`\x07` R T`\xFF\x16\x15a\x04\xBAW`@Qc\xE2\xE1!\x03`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04T\x84` \x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x05\tW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[` \x84\x01Qa\x05\x1E\x90c\xFF\xFF\xFF\xFF\x16\x82a/\xB0V[\x15a\x05YW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01a\x05\0V[`\nT` \x85\x01Q`@Qc\x05\xAF\xD53`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c-~\xA9\x98\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD1\x91\x90a/\xD2V[\x90P\x80` \x01Qa\x06\x03W` \x85\x01Q`@Qc\xF5sZ_`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x05\0V[`\0a\x06\x13\x83\x87` \x01Qa\x1B\tV[\x90P`\0\x86` \x01Qc\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06:Wa\x06:a'\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06cW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x88` \x01Qc\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x07VW`\0a\x06\x88\x82\x88a0)V[`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x80\x83R`\x01\x90\x91\x01T\x92\x82\x01\x92\x90\x92R\x92\x93P\x90\x03a\x06\xD5W`@Qc\xD3H/{`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x01Q\x85\x84\x81Q\x81\x10a\x06\xECWa\x06\xECa0<V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x84a\x07\x08\x91\x90a0)V[`\0\x83\x81R`\x03` R`@\x80\x82 \x82\x81U`\x01\x01\x82\x90U\x83Q\x90Q\x92\x96P\x91\x7F\xFEk\t{F\xA7\x8E\x08Pj1C\xB63|%\x05\xBAw\xDFv\xFE\x05\xC3f:\x98s\x95\xD64\x13\x91\x90\xA2PP`\x01\x01a\x06jV[P\x87` \x01Qc\xFF\xFF\xFF\xFF\x16`\x04`\0\x82\x82Ta\x07s\x91\x90a0RV[\x92PP\x81\x90UP`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x83`@Q` \x01a\x07\xAE\x91\x90a0eV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x07\xD1\x91\x90a/\xB0V[\x90P\x80\x89``\x01Q\x14a\x08'W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01R\x7FleafHash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R`\0\x91` \x82\x01`\x80\x806\x837\x01\x90PP\x90P`\x08T\x81`\0\x81Q\x81\x10a\x08`Wa\x08`a0<V[` \x02` \x01\x01\x81\x81RPP\x89`@\x01Q\x81`\x01\x81Q\x81\x10a\x08\x84Wa\x08\x84a0<V[` \x02` \x01\x01\x81\x81RPP\x81\x81`\x02\x81Q\x81\x10a\x08\xA4Wa\x08\xA4a0<V[` \x02` \x01\x01\x81\x81RPP\x84\x81`\x03\x81Q\x81\x10a\x08\xC4Wa\x08\xC4a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x85Q\x8AQ`@Qc\xC9AvG`\xE0\x1B\x81R`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xC9AvG\x91a\t\x02\x91\x90\x86\x90`\x04\x01a0\xBEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tE\x91\x90a/\x95V[\x90P\x80a\t\x95W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01R\x7Fproof\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[\x8A` \x01Qc\xFF\xFF\xFF\xFF\x16`\x05`\0\x82\x82Ta\t\xB1\x91\x90a0)V[\x90\x91UPP`@\x80\x8C\x01Q`\x08\x81\x90U`\0\x90\x81R`\x07` R \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\xE0\x84a\x1B\x92V[PPPPPPPPPPPV[`\x0BT`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n_\x91\x90\x81\x01\x90a10V[\x90P\x90V[`\0`\x04T`\x05Ta\n_\x91\x90a0)V[`\nT`@\x80Qc0D\xB7\x9B`\xE2\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xC1\x12\xDEl\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n_\x91\x90\x81\x01\x90a1\xA7V[a\n\xF0a\x1B\xACV[`\0a\x0B\0\x83`@\x01QQa\x1B\xD6V[\x90P`\0a\x0B\x12\x84`\xE0\x01QQa\x1B\xD6V[`\nT`@Qc\x1D\xCD\xF71`\xE3\x1B\x81R0`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xEEo\xB9\x88\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x7F\x91\x90a/\x95V[\x80\x15a\x0B\x90WPc\xFF\xFF\xFF\xFF\x81\x16\x15\x15[\x15a\x0B\xBEW`@Qch\x97ZG`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x05\0V[`\xC0\x84\x01Q\x15a\x0CsW`@\x80Q\x80\x82\x01\x82R0\x81R3` \x82\x01\x90\x81R`\nT\x92Qc^\xE3l\xE9`\xE0\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R\x91Q\x82\x16`$\x82\x01R\x91\x92\x16\x90c^\xE3l\xE9\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CT\x91\x90a/\x95V[a\x0CqW`@Qc2J\xF8\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[`\nT`@Qc\x85\xE8a\xEB`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x85\x16`\x04\x83\x01R\x83\x16`$\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x85\xE8a\xEB\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xED\x91\x90a/\xD2V[\x90P\x80` \x01Qa\r!W`@Qch\x97ZG`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x85\x16`\x04\x83\x01R\x83\x16`$\x82\x01R`D\x01a\x05\0V[\x82c\xFF\xFF\xFF\xFF\x16\x85``\x01QQ\x14a\r|W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FsigHashes length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[\x81c\xFF\xFF\xFF\xFF\x16\x85a\x01\0\x01QQ\x14a\r\xD8W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FoutRollupFees length\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[\x81c\xFF\xFF\xFF\xFF\x16\x85a\x01`\x01QQ\x14a\x0E4W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FoutEncryptedNotes length\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82c\xFF\xFF\xFF\xFF\x16`\x04T`\x05Ta\x0El\x91\x90a0)V[a\x0Ev\x91\x90a0)V[\x11\x15a\x0E\x95W`@Qc;\\\xCBC`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT`@Qc\xDFY/}`\xE0\x1B\x81R2`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x01\x91\x90a/\x95V[\x15a\x0F\x1FW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nTa\x01 \x86\x01Q`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FnW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x92\x91\x90a/\x95V[\x15a\x0F\xB0W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xC1`\x05c\xFF\xFF\xFF\xFF\x85\x16a23V[\x85a\x01\xA0\x01QQ\x14a\x0F\xE6W`@Qc:\xCFH\xBB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xF0\x85\x85a\x1C\x0BV[`\0a\x0F\xFD\x84`\x02a2JV[c\xFF\xFF\xFF\xFF\x16\x90P`\0a\x10\x12\x85`\x02a2JV[a\x10\x1D\x90`\x04a2pV[c\xFF\xFF\xFF\xFF\x16\x90P`\0a\x102\x85`\x02a2JV[a\x10B\x90c\xFF\xFF\xFF\xFF\x16\x83a0)V[\x90P`\0`\x05a\x10S\x88`\x02a2pV[c\xFF\xFF\xFF\xFF\x16a\x10c\x91\x90a23V[a\x10n\x83`\x02a0)V[a\x10x\x91\x90a0)V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x90Wa\x10\x90a'\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xB9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x80\x8B\x01Q`\0\x90\x81R`\x07\x90\x91R`@\x90 T\x90\x91P`\xFF\x16a\x11\x0BW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R`\x04\x01a\x05\0\x90` \x80\x82R`\x04\x90\x82\x01Rc\x1C\x9B\xDB\xDD`\xE2\x1B`@\x82\x01R``\x01\x90V[\x88` \x01Q\x81`\0\x81Q\x81\x10a\x11#Wa\x11#a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0a\x11;\x88`\x01a2pV[c\xFF\xFF\xFF\xFF\x16\x90P`\0[\x88c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x12=W`\0\x8B`@\x01Q\x82\x81Q\x81\x10a\x11lWa\x11la0<V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\0\x81\x81R`\x02\x90\x92R`@\x90\x91 T\x90\x91P`\xFF\x16\x15a\x11\xC5W`@Q\x7F\xFFUn \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x84a\x11\xD2\x84`\x01a0)V[\x81Q\x81\x10a\x11\xE2Wa\x11\xE2a0<V[` \x02` \x01\x01\x81\x81RPP\x8B``\x01Q\x82\x81Q\x81\x10a\x12\x04Wa\x12\x04a0<V[` \x02` \x01\x01Q\x84\x84\x84a\x12\x19\x91\x90a0)V[\x81Q\x81\x10a\x12)Wa\x12)a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x11FV[P`\x80\x8A\x01Q\x82a\x12O\x87`\x01a0)V[\x81Q\x81\x10a\x12_Wa\x12_a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xA0\x8A\x01Q\x82a\x12{\x87`\x02a0)V[\x81Q\x81\x10a\x12\x8BWa\x12\x8Ba0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xC0\x8A\x01Q\x82a\x12\xA7\x87`\x03a0)V[\x81Q\x81\x10a\x12\xB7Wa\x12\xB7a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0a\x12\xD4c\xFF\xFF\xFF\xFF\x89\x16\x86a0)V[\x90P`\0a\x12\xE0a\x19\xC8V[\x90P`\0[\x89c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x14NW`\x01`\0\x8E`\xE0\x01Q\x83\x81Q\x81\x10a\x13\rWa\x13\ra0<V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15a\x13yW`@Qc\xBE\xE3a\x11`\xE0\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7Fcommitment\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[\x81\x8Da\x01\0\x01Q\x82\x81Q\x81\x10a\x13\x91Wa\x13\x91a0<V[` \x02` \x01\x01Q\x10\x15a\x13\xB8W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x8C`\xE0\x01Q\x81\x81Q\x81\x10a\x13\xCEWa\x13\xCEa0<V[` \x02` \x01\x01Q\x85\x88\x83a\x13\xE3\x91\x90a0)V[\x81Q\x81\x10a\x13\xF3Wa\x13\xF3a0<V[` \x02` \x01\x01\x81\x81RPP\x8Ca\x01\0\x01Q\x81\x81Q\x81\x10a\x14\x16Wa\x14\x16a0<V[` \x02` \x01\x01Q\x85\x84\x83a\x14+\x91\x90a0)V[\x81Q\x81\x10a\x14;Wa\x14;a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x12\xE5V[Pa\x14Z\x8C\x85\x87a\x1D\xE8V[\x87Q\x8CQ`@Qc\xC9AvG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xC9AvG\x91a\x14\x8B\x91\x88\x90`\x04\x01a0\xBEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xCE\x91\x90a/\x95V[a\x15\x1BW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Ftransact proof\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[`\0[\x8Ac\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x15\xE5W`\x01`\x02`\0\x8F`@\x01Q\x84\x81Q\x81\x10a\x15HWa\x15Ha0<V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\x06`\0\x82\x82Ta\x15\x88\x91\x90a0)V[\x90\x91UPP`@\x8D\x01Q\x80Q\x82\x90\x81\x10a\x15\xA4Wa\x15\xA4a0<V[` \x02` \x01\x01Q\x8D` \x01Q\x7F<#r\xABa0\x81{\xD6\xB8\xFCm\xBA\xEC\xAE\x94~\x84 \x1BIS]5\x8D\xEB\xAAl4\xC2>\xCF`@Q`@Q\x80\x91\x03\x90\xA3`\x01\x01a\x15\x1EV[P`\0[\x89c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x16\xABW`\x01\x80`\0\x8F`\xE0\x01Q\x84\x81Q\x81\x10a\x16\x12Wa\x16\x12a0<V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x16\xA3\x8D`\xE0\x01Q\x82\x81Q\x81\x10a\x16XWa\x16Xa0<V[` \x02` \x01\x01Q\x8Ea\x01\0\x01Q\x83\x81Q\x81\x10a\x16wWa\x16wa0<V[` \x02` \x01\x01Q\x8Fa\x01`\x01Q\x84\x81Q\x81\x10a\x16\x96Wa\x16\x96a0<V[` \x02` \x01\x01Qa RV[`\x01\x01a\x15\xE9V[P`\xA0\x8C\x01Q\x15a\x16\xC9Wa\x16\xC9\x8Ca\x01 \x01Q\x8D`\xA0\x01Qa \xF1V[`\xC0\x8C\x01Q\x15a\x16\xE6Wa\x16\xE6\x8Ca\x01@\x01Q\x8D`\xC0\x01Qa \xF1V[a\x16\xEF\x8Ca!\x08V[PPPPPPPPPPa\x17\x03`\x01`\0UV[PPV[`\nT`@QcA\xFBiy`\xE0\x1B\x81R3`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cA\xFBiy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17t\x91\x90a2\x8CV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x160\x14a\x17\x9FW`@QcS5\xA0E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x17\xA9a\x19\xC8V[\x90P\x80\x84``\x01Q\x10\x15a\x17\xD0W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04T`\x05Ta\x18\x01\x91\x90a0)V[\x10a\x18\x1FW`@Qc;\\\xCBC`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x85\x01Q`\0\x90\x81R`\x01\x90\x91R`@\x90 T`\xFF\x16\x15a\x18UW`@Qc\xE3\x8C\xD1M`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x85\x01\x80Q`\0\x90\x81R`\x01\x92\x83\x90R`@\x90 \x80T`\xFF\x19\x16\x90\x92\x17\x90\x91UQ``\x85\x01Q`\x80\x86\x01Qa\x18\x8D\x92\x91\x90a RV[`@\x84\x01Q\x15a\x18\xA5Wa\x18\xA5\x83\x85`@\x01Qa \xF1V[PPPPV[```\0`\x04Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\xCAWa\x18\xCAa'\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xF3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\x04T\x81\x10\x15a\x19NW`\0\x81`\x05Ta\x19\x13\x91\x90a0)V[`\0\x81\x81R`\x03` R`@\x90 T\x84Q\x91\x92P\x90\x84\x90\x84\x90\x81\x10a\x19:Wa\x19:a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x18\xF9V[P\x91\x90PV[`\nT`@Qc\xDB\xDA\x08)`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDB\xDA\x08)\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xC2\x91\x90a2\xA9V[\x92\x91PPV[`\nT`@Qc\xC3\xC4\xBD\x0B`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC3\xC4\xBD\x0B\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A9\x91\x90a2\xA9V[\x90P\x80\x15a\x1AGW\x80a\x1AKV[`\tT[\x91PP\x90V[`\x0BT`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1<\xE5g\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1A\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n_\x91\x90a2\xC2V[`\x0BT`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n7W=`\0\x80>=`\0\xFD[`\0a\x01\0\x82c\xFF\xFF\xFF\xFF\x16\x10a\x1B/W`\x08\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x08\x83\x90\x1C\x92P[`\x10\x82c\xFF\xFF\xFF\xFF\x16\x10a\x1BRW`\x04\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x04\x83\x90\x1C\x92P[`\x04\x82c\xFF\xFF\xFF\xFF\x16\x10a\x1BuW`\x02\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x02\x83\x90\x1C\x92P[`\x02\x82c\xFF\xFF\xFF\xFF\x16\x10a\x1B\x8BW`\x01\x83\x90\x1C\x92P[P\x90\x91\x90PV[`\x0BTa\x1B\xA9\x90`\x01`\x01`\xA0\x1B\x03\x163\x83a#\x9CV[PV[`\x02`\0T\x03a\x1B\xCFW`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x1C\x07W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05\0V[P\x90V[a\x01`\x82\x01QQ`\x03\x81\x10a\x1C3W`@Qc?\xB1\x94]`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81`\0\x03a\x1C\x8AWa\x01 \x84\x01Qa\x01@\x85\x01Q`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x81\x16` \x83\x01R\x91\x90\x92\x1B\x16`4\x82\x01R`H\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x1DAV[\x81`\x01\x03a\x1C\xD0W\x83a\x01 \x01Q\x84a\x01@\x01Q\x85a\x01`\x01Q`\0\x81Q\x81\x10a\x1C\xB6Wa\x1C\xB6a0<V[` \x02` \x01\x01Q`@Q` \x01a\x1Ct\x93\x92\x91\x90a2\xE5V[\x83a\x01 \x01Q\x84a\x01@\x01Q\x85a\x01`\x01Q`\0\x81Q\x81\x10a\x1C\xF4Wa\x1C\xF4a0<V[` \x02` \x01\x01Q\x86a\x01`\x01Q`\x01\x81Q\x81\x10a\x1D\x14Wa\x1D\x14a0<V[` \x02` \x01\x01Q`@Q` \x01a\x1D/\x94\x93\x92\x91\x90a36V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[\x80Q` \x82\x01 \x7F\x19Ethereum Signed Message:\n32\0\0\0\0`\0\x90\x81R`\x1C\x91\x90\x91R`<\x90 a\x1D\x82\x81\x85a$\x08V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x1B\x85`\x80\x01Q\x14a\x1D\xE1W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7Fsignature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\0V[PPPPPV[a\x01\x80\x83\x01Q`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\xFF\x83\x90\x1C\x80\x83R`\x01`\x01`\xFF\x1B\x03\x90\x93\x16\x90\x82\x01R\x83Q\x90\x91\x90\x84\x90\x84\x90\x81\x10a\x1E8Wa\x1E8a0<V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x83\x83`\x01a\x1EW\x91\x90a0)V[\x81Q\x81\x10a\x1EgWa\x1Ega0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\nT`@\x80Qc0D\xB7\x9B`\xE2\x1B\x81R\x90Q`\x05\x92`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC1\x12\xDEl\x91`\x04\x80\x82\x01\x92\x86\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\xE9\x91\x90\x81\x01\x90a1\xA7V[\x90P`\0a\x1E\xF8\x85`\x02a0)V[\x90P`\0a\x1F\x06\x84\x83a0)V[\x90P`\0a\x1F\x15\x85`\x02a23V[a\x1F\x1F\x90\x84a0)V[\x90P`\0[\x85\x81\x10\x15a\x1F\xE5W`\0a\x1F\x83\x86\x83\x81Q\x81\x10a\x1FCWa\x1FCa0<V[` \x02` \x01\x01Q`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\xFF\x83\x90\x1C\x82R`\x01`\x01`\xFF\x1B\x03\x90\x92\x16\x91\x81\x01\x91\x90\x91R\x90V[\x80Q\x90\x91P\x8Aa\x1F\x93\x84\x88a0)V[\x81Q\x81\x10a\x1F\xA3Wa\x1F\xA3a0<V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x8A\x83\x86a\x1F\xC1\x91\x90a0)V[\x81Q\x81\x10a\x1F\xD1Wa\x1F\xD1a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x1F$V[P`\0[\x89a\x01\xA0\x01QQ\x81\x10\x15a FW\x89a\x01\xA0\x01Q\x81\x81Q\x81\x10a \x0EWa \x0Ea0<V[` \x02` \x01\x01Q\x89\x82\x84a #\x91\x90a0)V[\x81Q\x81\x10a 3Wa 3a0<V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1F\xE9V[PPPPPPPPPPV[`\0`\x05T`\x04Ta d\x91\x90a0)V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R`\0\x85\x81R`\x03\x90\x92R\x92\x81 \x91Q\x82U\x91Q`\x01\x91\x82\x01U`\x04\x80T\x93\x94P\x90\x92\x90\x91\x90a \xA8\x90\x84\x90a0)V[\x92PP\x81\x90UP\x83\x7F\xF53\xF9pZ\xACP \xE2\x16\x95\xEA5S\xAC{h\x81\x07\r+i\0\xAB+\x1E0P0K[\xF9\x84\x83\x85`@Qa \xE3\x93\x92\x91\x90a3\x9DV[`@Q\x80\x91\x03\x90\xA2PPPPV[`\x0BTa\x17\x03\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83a#\x9CV[`\nT`@\x80Qc0D\xB7\x9B`\xE2\x1B\x81R\x90Q`\x05\x92`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC1\x12\xDEl\x91`\x04\x80\x82\x01\x92\x86\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\x7F\x91\x90\x81\x01\x90a1\xA7V[\x90P`\0\x82\x84`@\x01QQa!\x94\x91\x90a23V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xB1Wa!\xB1a'\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x10W\x81` \x01[a!\xFD`@Q\x80``\x01`@R\x80`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a!\xCFW\x90P[P\x90P`\0\x80[\x86`@\x01QQ\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a#\\W`\0[\x86\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a#IW\x80c\xFF\xFF\xFF\xFF\x16` \x83c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B\x17\x84\x84\x81Q\x81\x10a\"jWa\"ja0<V[` \x90\x81\x02\x91\x90\x91\x01\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90R\x85Q\x86\x90c\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x10a\"\x9DWa\"\x9Da0<V[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a\"\xB7Wa\"\xB7a0<V[` \x02` \x01\x01Q` \x01\x81\x81RPP\x87a\x01\xA0\x01Q\x81c\xFF\xFF\xFF\xFF\x16\x88\x84c\xFF\xFF\xFF\xFF\x16a\"\xE6\x91\x90a23V[a\"\xF0\x91\x90a0)V[\x81Q\x81\x10a#\0Wa#\0a0<V[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a#\x1AWa#\x1Aa0<V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@\x01R\x82a#3\x81a3\xBCV[\x93PP\x80\x80a#A\x90a3\xD5V[\x91PPa\"-V[P\x80a#T\x81a3\xD5V[\x91PPa\"\x17V[P\x7Fj\xF0~\xBC\xB391\xAB\xD3H\xE7\x85\xF7\x0Cm%\x93\x90\xCFy\x91\x91\xF8\xC9\x91$$[\x96\xE1\xA6\x13\x82`@Qa#\x8C\x91\x90a3\xFAV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra$\x03\x90\x84\x90a$2V[PPPV[`\0\x80`\0\x80a$\x18\x86\x86a$\x95V[\x92P\x92P\x92Pa$(\x82\x82a$\xE2V[P\x90\x94\x93PPPPV[`\0a$G`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a%\x9BV[\x90P\x80Q`\0\x14\x15\x80\x15a$lWP\x80\x80` \x01\x90Q\x81\x01\x90a$j\x91\x90a/\x95V[\x15[\x15a$\x03W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x05\0V[`\0\x80`\0\x83Q`A\x03a$\xCFW` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa$\xC1\x88\x82\x85\x85a%\xB0V[\x95P\x95P\x95PPPPa$\xDBV[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a$\xF6Wa$\xF6a*\x8AV[\x03a$\xFFWPPV[`\x01\x82`\x03\x81\x11\x15a%\x13Wa%\x13a*\x8AV[\x03a%1W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a%EWa%Ea*\x8AV[\x03a%fW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05\0V[`\x03\x82`\x03\x81\x11\x15a%zWa%za*\x8AV[\x03a\x17\x03W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05\0V[``a%\xA9\x83\x83`\0a&\x7FV[\x93\x92PPPV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a%\xEBWP`\0\x91P`\x03\x90P\x82a&uV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a&?W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a&kWP`\0\x92P`\x01\x91P\x82\x90Pa&uV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x81G\x10\x15a&\xA4W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x05\0V[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa&\xC0\x91\x90a4XV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a&\xFDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'\x02V[``\x91P[P\x91P\x91Pa'\x12\x86\x83\x83a'\x1CV[\x96\x95PPPPPPV[``\x82a'1Wa',\x82a'xV[a%\xA9V[\x81Q\x15\x80\x15a'HWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a'qW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x05\0V[P\x80a%\xA9V[\x80Q\x15a'\x88W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\xDAWa'\xDAa'\xA1V[`@R\x90V[`@Qa\x01\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\xDAWa'\xDAa'\xA1V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\xDAWa'\xDAa'\xA1V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(PWa(Pa'\xA1V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a(jW`\0\x80\xFD[a(ra'\xB7V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a(\x9AW`\0\x80\xFD[a(\xA2a'\xB7V[\x80`@\x84\x01\x85\x81\x11\x15a(\xB4W`\0\x80\xFD[\x84[\x81\x81\x10\x15a(\xCEW\x805\x84R` \x93\x84\x01\x93\x01a(\xB6V[P\x90\x95\x94PPPPPV[`\0\x81\x83\x03a\x01\0\x81\x12\x15a(\xEDW`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\x10Wa)\x10a'\xA1V[`@R\x91P\x81a) \x85\x85a(XV[\x81R`\x80`?\x19\x83\x01\x12\x15a)4W`\0\x80\xFD[a)<a'\xB7V[\x91Pa)K\x85`@\x86\x01a(\x89V[\x82Ra)Z\x85`\x80\x86\x01a(\x89V[` \x83\x01R\x81` \x82\x01Ra)r\x85`\xC0\x86\x01a(XV[`@\x82\x01RPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a)\x93W`\0\x80\xFD[\x91\x90PV[`\0a\x01`\x82\x84\x03\x12\x80\x15a)\xACW`\0\x80\xFD[P`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\xD0Wa)\xD0a'\xA1V[`@Ra)\xDD\x84\x84a(\xD9V[\x81Ra)\xECa\x01\0\x84\x01a)\x7FV[` \x82\x01Ra\x01 \x83\x015`@\x82\x01Ra\x01@\x90\x92\x015``\x83\x01RP\x91\x90PV[`\0[\x83\x81\x10\x15a*)W\x81\x81\x01Q\x83\x82\x01R` \x01a*\x11V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra*J\x81` \x86\x01` \x86\x01a*\x0EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a%\xA9` \x83\x01\x84a*2V[`\0` \x82\x84\x03\x12\x15a*\x83W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a*\xC2WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15a*\xFAW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a*\xDCV[P\x93\x94\x93PPPPV[` \x81R`\0a%\xA9` \x83\x01\x84a*\xC8V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a+1Wa+1a'\xA1V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a+LW`\0\x80\xFD[\x815a+_a+Z\x82a+\x17V[a('V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a+\x81W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a+\x9EW\x805\x83R` \x92\x83\x01\x92\x01a+\x86V[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xA9W`\0\x80\xFD[\x805a)\x93\x81a+\xA8V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a+\xE2Wa+\xE2a'\xA1V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a,\x01W`\0\x80\xFD[\x815a,\x0Fa+Z\x82a+\xC8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a,$W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a,RW`\0\x80\xFD[\x815a,`a+Z\x82a+\x17V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a,\x82W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a+\x9EW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xA6W`\0\x80\xFD[a,\xB5\x88` \x83\x8A\x01\x01a+\xF0V[\x84RP` \x92\x83\x01\x92\x01a,\x87V[`\0\x80`@\x83\x85\x03\x12\x15a,\xD7W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xEEW`\0\x80\xFD[\x83\x01a\x02\xA0\x81\x86\x03\x12\x15a-\x01W`\0\x80\xFD[a-\ta'\xE0V[a-\x13\x86\x83a(\xD9V[\x81Ra\x01\0\x82\x015` \x82\x01Ra\x01 \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-;W`\0\x80\xFD[a-G\x87\x82\x85\x01a+;V[`@\x83\x01RPa\x01@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-hW`\0\x80\xFD[a-t\x87\x82\x85\x01a+;V[``\x83\x01RPa\x01`\x82\x015`\x80\x82\x01Ra\x01\x80\x82\x015`\xA0\x82\x01Ra\x01\xA0\x82\x015`\xC0\x82\x01Ra\x01\xC0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xB6W`\0\x80\xFD[a-\xC2\x87\x82\x85\x01a+;V[`\xE0\x83\x01RPa\x01\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xE3W`\0\x80\xFD[a-\xEF\x87\x82\x85\x01a+;V[a\x01\0\x83\x01RPa.\x03a\x02\0\x83\x01a+\xBDV[a\x01 \x82\x01Ra.\x16a\x02 \x83\x01a+\xBDV[a\x01@\x82\x01Ra\x02@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.7W`\0\x80\xFD[a.C\x87\x82\x85\x01a,AV[a\x01`\x83\x01RPa\x02`\x82\x015a\x01\x80\x82\x01Ra\x02\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.qW`\0\x80\xFD[a.}\x87\x82\x85\x01a+;V[a\x01\xA0\x83\x01RP\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xA1W`\0\x80\xFD[a.\xAD\x85\x82\x86\x01a+\xF0V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a.\xCAW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xE1W`\0\x80\xFD[\x83\x01`\xA0\x81\x86\x03\x12\x15a.\xF3W`\0\x80\xFD[a.\xFBa(\x04V[\x815\x81R` \x80\x83\x015\x90\x82\x01R`@\x80\x83\x015\x90\x82\x01R``\x80\x83\x015\x90\x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/7W`\0\x80\xFD[a/C\x87\x82\x85\x01a+\xF0V[`\x80\x83\x01RP\x92Pa/Y\x90P` \x84\x01a+\xBDV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a/uW`\0\x80\xFD[\x825\x91Pa/Y` \x84\x01a)\x7FV[\x80Q\x80\x15\x15\x81\x14a)\x93W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a/\xA7W`\0\x80\xFD[a%\xA9\x82a/\x85V[`\0\x82a/\xCDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0`@\x82\x84\x03\x12\x80\x15a/\xE5W`\0\x80\xFD[Pa/\xEEa'\xB7V[\x82Qa/\xF9\x81a+\xA8V[\x81Ra0\x07` \x84\x01a/\x85V[` \x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x19\xC2Wa\x19\xC2a0\x13V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x19\xC2Wa\x19\xC2a0\x13V[\x81Q`\0\x90\x82\x90` \x85\x01\x83[\x82\x81\x10\x15a0\x90W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a0rV[P\x91\x95\x94PPPPPV[\x80`\0[`\x02\x81\x10\x15a\x18\xA5W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a0\x9FV[a0\xD3\x81\x84Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\0` \x84\x01Qa0\xE8`@\x84\x01\x82Qa0\x9BV[` \x01Qa0\xF9`\x80\x84\x01\x82a0\x9BV[P`@\x84\x01Q\x80Q`\xC0\x84\x01R` \x81\x01Q`\xE0\x84\x01RPa\x01 a\x01\0\x83\x01Ra1(a\x01 \x83\x01\x84a*\xC8V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a1BW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1YW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a1jW`\0\x80\xFD[\x80Qa1xa+Z\x82a+\xC8V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a1\x8DW`\0\x80\xFD[a1\x9E\x82` \x83\x01` \x86\x01a*\x0EV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a1\xB9W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xD0W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a1\xE1W`\0\x80\xFD[\x80Qa1\xEFa+Z\x82a+\x17V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a2\x11W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a'\x12W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a2\x18V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x19\xC2Wa\x19\xC2a0\x13V[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a2iWa2ia0\x13V[P\x92\x91PPV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x19\xC2Wa\x19\xC2a0\x13V[`\0` \x82\x84\x03\x12\x15a2\x9EW`\0\x80\xFD[\x81Qa%\xA9\x81a+\xA8V[`\0` \x82\x84\x03\x12\x15a2\xBBW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a2\xD4W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a%\xA9W`\0\x80\xFD[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x84``\x1B\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83``\x1B\x16`\x14\x82\x01R`\0\x82Qa3'\x81`(\x85\x01` \x87\x01a*\x0EV[\x91\x90\x91\x01`(\x01\x94\x93PPPPV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x85``\x1B\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x84``\x1B\x16`\x14\x82\x01R`\0\x83Qa3x\x81`(\x85\x01` \x88\x01a*\x0EV[\x83Q\x90\x83\x01\x90a3\x8F\x81`(\x84\x01` \x88\x01a*\x0EV[\x01`(\x01\x96\x95PPPPPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\0a1\x9E``\x83\x01\x84a*2V[`\0`\x01\x82\x01a3\xCEWa3\xCEa0\x13V[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a3\xF1Wa3\xF1a0\x13V[`\x01\x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\xCEW\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01RP``\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa4\x14V[`\0\x82Qa4j\x81\x84` \x87\x01a*\x0EV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x950D\x0F,V\xB6\xAFU\x16\xF4\x81\xDA\x89\xDB\xB6@\x15\xD2[\n\x9A\xB8\x8E\xFD\xDF\xD4?\xF6OB\xD3dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static COMMITMENTPOOLERC20_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes = ::ethers_core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CommitmentPoolERC20<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for CommitmentPoolERC20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CommitmentPoolERC20<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CommitmentPoolERC20<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CommitmentPoolERC20<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CommitmentPoolERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> CommitmentPoolERC20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    COMMITMENTPOOLERC20_ABI.clone(),
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
                COMMITMENTPOOLERC20_ABI.clone(),
                COMMITMENTPOOLERC20_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `AUDITOR_COUNT` (0xa592bd69) function
        pub fn auditor_count(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([165, 146, 189, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_pathIndices` (0xf2da1d41) function
        pub fn path_indices(
            &self,
            full_path: ::ethers_core::types::U256,
            rollup_size: u32,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([242, 218, 29, 65], (full_path, rollup_size))
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `defaultMinRollupFee` (0xb2316c33) function
        pub fn default_min_rollup_fee(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([178, 49, 108, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enqueue` (0x78d60cd7) function
        pub fn enqueue(
            &self,
            request: CommitmentRequest,
            executor: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 214, 12, 215], (request, executor))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllAuditorPublicKeys` (0x63bc7d32) function
        pub fn get_all_auditor_public_keys(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers_core::types::U256>,
        > {
            self.0
                .method_hash([99, 188, 125, 50], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAuditorPublicKey` (0x87780df9) function
        pub fn get_auditor_public_key(
            &self,
            index: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([135, 120, 13, 249], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCommitmentCount` (0x5688881f) function
        pub fn get_commitment_count(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([86, 136, 136, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCommitmentIncludedCount` (0xe500f504) function
        pub fn get_commitment_included_count(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([229, 0, 245, 4], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCommitmentQueuedCount` (0x555d75f0) function
        pub fn get_commitment_queued_count(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([85, 93, 117, 240], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinRollupFee` (0xb08892d0) function
        pub fn get_min_rollup_fee(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([176, 136, 146, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNullifierCount` (0x7a553744) function
        pub fn get_nullifier_count(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([122, 85, 55, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQueuedCommitments` (0x866ac658) function
        pub fn get_queued_commitments(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers_core::types::U256>,
        > {
            self.0
                .method_hash([134, 106, 198, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTreeCapacity` (0x484eb652) function
        pub fn get_tree_capacity(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([72, 78, 182, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isHistoricCommitment` (0x57060016) function
        pub fn is_historic_commitment(
            &self,
            commitment: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([87, 6, 0, 22], commitment)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isKnownRoot` (0xa6232a93) function
        pub fn is_known_root(
            &self,
            root: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 35, 42, 147], root)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSpentSerialNumber` (0x3bb8d1b4) function
        pub fn is_spent_serial_number(
            &self,
            serial_number: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 184, 209, 180], serial_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollup` (0x14a7737d) function
        pub fn rollup(
            &self,
            request: RollupRequest,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 167, 115, 125], (request,))
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
        ///Calls the contract's `transact` (0x72082971) function
        pub fn transact(
            &self,
            request: TransactRequest,
            signature: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 8, 41, 113], (request, signature))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CommitmentIncluded` event
        pub fn commitment_included_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CommitmentIncludedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CommitmentQueued` event
        pub fn commitment_queued_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CommitmentQueuedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CommitmentSpent` event
        pub fn commitment_spent_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CommitmentSpentFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EncryptedAuditorNote` event
        pub fn encrypted_auditor_note_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EncryptedAuditorNoteFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EncryptedAuditorNotes` event
        pub fn encrypted_auditor_notes_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EncryptedAuditorNotesFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CommitmentPoolERC20Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for CommitmentPoolERC20<M> {
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
    ///Custom Error type `AssociatedPoolNotMatched` with signature `AssociatedPoolNotMatched()` and selector `0x5335a045`
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
    #[etherror(name = "AssociatedPoolNotMatched", abi = "AssociatedPoolNotMatched()")]
    pub struct AssociatedPoolNotMatched;
    ///Custom Error type `AuditorNotesLengthError` with signature `AuditorNotesLengthError()` and selector `0xeb3d22ec`
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
    #[etherror(name = "AuditorNotesLengthError", abi = "AuditorNotesLengthError()")]
    pub struct AuditorNotesLengthError;
    ///Custom Error type `CommitmentHasBeenSubmitted` with signature `CommitmentHasBeenSubmitted()` and selector `0xe38cd14d`
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
        name = "CommitmentHasBeenSubmitted",
        abi = "CommitmentHasBeenSubmitted()"
    )]
    pub struct CommitmentHasBeenSubmitted;
    ///Custom Error type `Duplicated` with signature `Duplicated(string)` and selector `0xbee36111`
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
    #[etherror(name = "Duplicated", abi = "Duplicated(string)")]
    pub struct Duplicated {
        pub param: ::std::string::String,
    }
    ///Custom Error type `ECDSAInvalidSignature` with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`
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
    #[etherror(name = "ECDSAInvalidSignature", abi = "ECDSAInvalidSignature()")]
    pub struct ECDSAInvalidSignature;
    ///Custom Error type `ECDSAInvalidSignatureLength` with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`
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
        name = "ECDSAInvalidSignatureLength",
        abi = "ECDSAInvalidSignatureLength(uint256)"
    )]
    pub struct ECDSAInvalidSignatureLength {
        pub length: ::ethers_core::types::U256,
    }
    ///Custom Error type `ECDSAInvalidSignatureS` with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`
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
    #[etherror(name = "ECDSAInvalidSignatureS", abi = "ECDSAInvalidSignatureS(bytes32)")]
    pub struct ECDSAInvalidSignatureS {
        pub s: [u8; 32],
    }
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
    ///Custom Error type `IndexOutOfBound` with signature `IndexOutOfBound()` and selector `0xd3482f7b`
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
    #[etherror(name = "IndexOutOfBound", abi = "IndexOutOfBound()")]
    pub struct IndexOutOfBound;
    ///Custom Error type `Invalid` with signature `Invalid(string)` and selector `0x53a2556c`
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
    #[etherror(name = "Invalid", abi = "Invalid(string)")]
    pub struct Invalid {
        pub param: ::std::string::String,
    }
    ///Custom Error type `NewRootIsDuplicated` with signature `NewRootIsDuplicated()` and selector `0xe2e12103`
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
    #[etherror(name = "NewRootIsDuplicated", abi = "NewRootIsDuplicated()")]
    pub struct NewRootIsDuplicated;
    ///Custom Error type `NoteHasBeenSpent` with signature `NoteHasBeenSpent()` and selector `0xff556e20`
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
    #[etherror(name = "NoteHasBeenSpent", abi = "NoteHasBeenSpent()")]
    pub struct NoteHasBeenSpent;
    ///Custom Error type `OutputNotesLessThanThree` with signature `OutputNotesLessThanThree()` and selector `0x7f6328ba`
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
    #[etherror(name = "OutputNotesLessThanThree", abi = "OutputNotesLessThanThree()")]
    pub struct OutputNotesLessThanThree;
    ///Custom Error type `ReentrancyGuardReentrantCall` with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`
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
        name = "ReentrancyGuardReentrantCall",
        abi = "ReentrancyGuardReentrantCall()"
    )]
    pub struct ReentrancyGuardReentrantCall;
    ///Custom Error type `RejectRelay` with signature `RejectRelay()` and selector `0x6495f1b2`
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
    #[etherror(name = "RejectRelay", abi = "RejectRelay()")]
    pub struct RejectRelay;
    ///Custom Error type `RejectRollup` with signature `RejectRollup()` and selector `0xff4e3423`
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
    #[etherror(name = "RejectRollup", abi = "RejectRollup()")]
    pub struct RejectRollup;
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
    ///Custom Error type `RollupVerifierDisabled` with signature `RollupVerifierDisabled(uint256)` and selector `0xf5735a5f`
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
    #[etherror(name = "RollupVerifierDisabled", abi = "RollupVerifierDisabled(uint256)")]
    pub struct RollupVerifierDisabled {
        pub rollup_size: ::ethers_core::types::U256,
    }
    ///Custom Error type `SafeCastOverflowedUintDowncast` with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`
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
        name = "SafeCastOverflowedUintDowncast",
        abi = "SafeCastOverflowedUintDowncast(uint8,uint256)"
    )]
    pub struct SafeCastOverflowedUintDowncast {
        pub bits: u8,
        pub value: ::ethers_core::types::U256,
    }
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
    ///Custom Error type `TransactVerifierDisabled` with signature `TransactVerifierDisabled(uint32,uint32)` and selector `0x68975a47`
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
        name = "TransactVerifierDisabled",
        abi = "TransactVerifierDisabled(uint32,uint32)"
    )]
    pub struct TransactVerifierDisabled {
        pub input_number: u32,
        pub output_number: u32,
    }
    ///Custom Error type `TreeHeightLessThanZero` with signature `TreeHeightLessThanZero()` and selector `0xb13ca6c4`
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
    #[etherror(name = "TreeHeightLessThanZero", abi = "TreeHeightLessThanZero()")]
    pub struct TreeHeightLessThanZero;
    ///Custom Error type `TreeHeightOutOfBounds` with signature `TreeHeightOutOfBounds()` and selector `0x9780f429`
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
    #[etherror(name = "TreeHeightOutOfBounds", abi = "TreeHeightOutOfBounds()")]
    pub struct TreeHeightOutOfBounds;
    ///Custom Error type `TreeIsFull` with signature `TreeIsFull()` and selector `0xed732d0c`
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
    #[etherror(name = "TreeIsFull", abi = "TreeIsFull()")]
    pub struct TreeIsFull;
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
    pub enum CommitmentPoolERC20Errors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        AssociatedPoolNotMatched(AssociatedPoolNotMatched),
        AuditorNotesLengthError(AuditorNotesLengthError),
        CommitmentHasBeenSubmitted(CommitmentHasBeenSubmitted),
        Duplicated(Duplicated),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        FailedInnerCall(FailedInnerCall),
        IndexOutOfBound(IndexOutOfBound),
        Invalid(Invalid),
        NewRootIsDuplicated(NewRootIsDuplicated),
        NoteHasBeenSpent(NoteHasBeenSpent),
        OutputNotesLessThanThree(OutputNotesLessThanThree),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        RejectRelay(RejectRelay),
        RejectRollup(RejectRollup),
        RollupFeeToFew(RollupFeeToFew),
        RollupVerifierDisabled(RollupVerifierDisabled),
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        SanctionedAddress(SanctionedAddress),
        TransactVerifierDisabled(TransactVerifierDisabled),
        TreeHeightLessThanZero(TreeHeightLessThanZero),
        TreeHeightOutOfBounds(TreeHeightOutOfBounds),
        TreeIsFull(TreeIsFull),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for CommitmentPoolERC20Errors {
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
                = <AssociatedPoolNotMatched as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AssociatedPoolNotMatched(decoded));
            }
            if let Ok(decoded)
                = <AuditorNotesLengthError as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AuditorNotesLengthError(decoded));
            }
            if let Ok(decoded)
                = <CommitmentHasBeenSubmitted as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CommitmentHasBeenSubmitted(decoded));
            }
            if let Ok(decoded)
                = <Duplicated as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Duplicated(decoded));
            }
            if let Ok(decoded)
                = <ECDSAInvalidSignature as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ECDSAInvalidSignature(decoded));
            }
            if let Ok(decoded)
                = <ECDSAInvalidSignatureLength as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ECDSAInvalidSignatureLength(decoded));
            }
            if let Ok(decoded)
                = <ECDSAInvalidSignatureS as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ECDSAInvalidSignatureS(decoded));
            }
            if let Ok(decoded)
                = <FailedInnerCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded)
                = <IndexOutOfBound as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IndexOutOfBound(decoded));
            }
            if let Ok(decoded)
                = <Invalid as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Invalid(decoded));
            }
            if let Ok(decoded)
                = <NewRootIsDuplicated as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NewRootIsDuplicated(decoded));
            }
            if let Ok(decoded)
                = <NoteHasBeenSpent as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoteHasBeenSpent(decoded));
            }
            if let Ok(decoded)
                = <OutputNotesLessThanThree as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OutputNotesLessThanThree(decoded));
            }
            if let Ok(decoded)
                = <ReentrancyGuardReentrantCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ReentrancyGuardReentrantCall(decoded));
            }
            if let Ok(decoded)
                = <RejectRelay as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RejectRelay(decoded));
            }
            if let Ok(decoded)
                = <RejectRollup as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RejectRollup(decoded));
            }
            if let Ok(decoded)
                = <RollupFeeToFew as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RollupFeeToFew(decoded));
            }
            if let Ok(decoded)
                = <RollupVerifierDisabled as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RollupVerifierDisabled(decoded));
            }
            if let Ok(decoded)
                = <SafeCastOverflowedUintDowncast as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeCastOverflowedUintDowncast(decoded));
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
                = <TransactVerifierDisabled as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransactVerifierDisabled(decoded));
            }
            if let Ok(decoded)
                = <TreeHeightLessThanZero as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TreeHeightLessThanZero(decoded));
            }
            if let Ok(decoded)
                = <TreeHeightOutOfBounds as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TreeHeightOutOfBounds(decoded));
            }
            if let Ok(decoded)
                = <TreeIsFull as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TreeIsFull(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for CommitmentPoolERC20Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AssociatedPoolNotMatched(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AuditorNotesLengthError(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CommitmentHasBeenSubmitted(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Duplicated(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignature(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IndexOutOfBound(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Invalid(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NewRootIsDuplicated(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::NoteHasBeenSpent(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::OutputNotesLessThanThree(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RejectRelay(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RejectRollup(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RollupFeeToFew(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RollupVerifierDisabled(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SafeCastOverflowedUintDowncast(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SanctionedAddress(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::TransactVerifierDisabled(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::TreeHeightLessThanZero(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::TreeHeightOutOfBounds(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::TreeIsFull(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for CommitmentPoolERC20Errors {
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
                    == <AssociatedPoolNotMatched as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AuditorNotesLengthError as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CommitmentHasBeenSubmitted as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Duplicated as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <ECDSAInvalidSignature as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureLength as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureS as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <IndexOutOfBound as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <NewRootIsDuplicated as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoteHasBeenSpent as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutputNotesLessThanThree as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReentrancyGuardReentrantCall as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RejectRelay as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <RejectRollup as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <RollupFeeToFew as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RollupVerifierDisabled as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeCastOverflowedUintDowncast as ::ethers_contract::EthError>::selector() => {
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
                    == <TransactVerifierDisabled as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TreeHeightLessThanZero as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TreeHeightOutOfBounds as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TreeIsFull as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CommitmentPoolERC20Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssociatedPoolNotMatched(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AuditorNotesLengthError(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CommitmentHasBeenSubmitted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Duplicated(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::IndexOutOfBound(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewRootIsDuplicated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoteHasBeenSpent(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutputNotesLessThanThree(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RejectRelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::RejectRollup(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupFeeToFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupVerifierDisabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeCastOverflowedUintDowncast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SanctionedAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransactVerifierDisabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TreeHeightLessThanZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TreeHeightOutOfBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TreeIsFull(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CommitmentPoolERC20Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for CommitmentPoolERC20Errors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance>
    for CommitmentPoolERC20Errors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<AssociatedPoolNotMatched> for CommitmentPoolERC20Errors {
        fn from(value: AssociatedPoolNotMatched) -> Self {
            Self::AssociatedPoolNotMatched(value)
        }
    }
    impl ::core::convert::From<AuditorNotesLengthError> for CommitmentPoolERC20Errors {
        fn from(value: AuditorNotesLengthError) -> Self {
            Self::AuditorNotesLengthError(value)
        }
    }
    impl ::core::convert::From<CommitmentHasBeenSubmitted>
    for CommitmentPoolERC20Errors {
        fn from(value: CommitmentHasBeenSubmitted) -> Self {
            Self::CommitmentHasBeenSubmitted(value)
        }
    }
    impl ::core::convert::From<Duplicated> for CommitmentPoolERC20Errors {
        fn from(value: Duplicated) -> Self {
            Self::Duplicated(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for CommitmentPoolERC20Errors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength>
    for CommitmentPoolERC20Errors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for CommitmentPoolERC20Errors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for CommitmentPoolERC20Errors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<IndexOutOfBound> for CommitmentPoolERC20Errors {
        fn from(value: IndexOutOfBound) -> Self {
            Self::IndexOutOfBound(value)
        }
    }
    impl ::core::convert::From<Invalid> for CommitmentPoolERC20Errors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<NewRootIsDuplicated> for CommitmentPoolERC20Errors {
        fn from(value: NewRootIsDuplicated) -> Self {
            Self::NewRootIsDuplicated(value)
        }
    }
    impl ::core::convert::From<NoteHasBeenSpent> for CommitmentPoolERC20Errors {
        fn from(value: NoteHasBeenSpent) -> Self {
            Self::NoteHasBeenSpent(value)
        }
    }
    impl ::core::convert::From<OutputNotesLessThanThree> for CommitmentPoolERC20Errors {
        fn from(value: OutputNotesLessThanThree) -> Self {
            Self::OutputNotesLessThanThree(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall>
    for CommitmentPoolERC20Errors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<RejectRelay> for CommitmentPoolERC20Errors {
        fn from(value: RejectRelay) -> Self {
            Self::RejectRelay(value)
        }
    }
    impl ::core::convert::From<RejectRollup> for CommitmentPoolERC20Errors {
        fn from(value: RejectRollup) -> Self {
            Self::RejectRollup(value)
        }
    }
    impl ::core::convert::From<RollupFeeToFew> for CommitmentPoolERC20Errors {
        fn from(value: RollupFeeToFew) -> Self {
            Self::RollupFeeToFew(value)
        }
    }
    impl ::core::convert::From<RollupVerifierDisabled> for CommitmentPoolERC20Errors {
        fn from(value: RollupVerifierDisabled) -> Self {
            Self::RollupVerifierDisabled(value)
        }
    }
    impl ::core::convert::From<SafeCastOverflowedUintDowncast>
    for CommitmentPoolERC20Errors {
        fn from(value: SafeCastOverflowedUintDowncast) -> Self {
            Self::SafeCastOverflowedUintDowncast(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for CommitmentPoolERC20Errors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<SanctionedAddress> for CommitmentPoolERC20Errors {
        fn from(value: SanctionedAddress) -> Self {
            Self::SanctionedAddress(value)
        }
    }
    impl ::core::convert::From<TransactVerifierDisabled> for CommitmentPoolERC20Errors {
        fn from(value: TransactVerifierDisabled) -> Self {
            Self::TransactVerifierDisabled(value)
        }
    }
    impl ::core::convert::From<TreeHeightLessThanZero> for CommitmentPoolERC20Errors {
        fn from(value: TreeHeightLessThanZero) -> Self {
            Self::TreeHeightLessThanZero(value)
        }
    }
    impl ::core::convert::From<TreeHeightOutOfBounds> for CommitmentPoolERC20Errors {
        fn from(value: TreeHeightOutOfBounds) -> Self {
            Self::TreeHeightOutOfBounds(value)
        }
    }
    impl ::core::convert::From<TreeIsFull> for CommitmentPoolERC20Errors {
        fn from(value: TreeIsFull) -> Self {
            Self::TreeIsFull(value)
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
    #[ethevent(name = "CommitmentIncluded", abi = "CommitmentIncluded(uint256)")]
    pub struct CommitmentIncludedFilter {
        #[ethevent(indexed)]
        pub commitment: ::ethers_core::types::U256,
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
        name = "CommitmentQueued",
        abi = "CommitmentQueued(uint256,uint256,uint256,bytes)"
    )]
    pub struct CommitmentQueuedFilter {
        #[ethevent(indexed)]
        pub commitment: ::ethers_core::types::U256,
        pub rollup_fee: ::ethers_core::types::U256,
        pub leaf_index: ::ethers_core::types::U256,
        pub encrypted_note: ::ethers_core::types::Bytes,
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
    #[ethevent(name = "CommitmentSpent", abi = "CommitmentSpent(uint256,uint256)")]
    pub struct CommitmentSpentFilter {
        #[ethevent(indexed)]
        pub root_hash: ::ethers_core::types::U256,
        #[ethevent(indexed)]
        pub serial_number: ::ethers_core::types::U256,
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
        name = "EncryptedAuditorNote",
        abi = "EncryptedAuditorNote(uint64,uint256,uint256)"
    )]
    pub struct EncryptedAuditorNoteFilter {
        pub id: u64,
        pub auditor_public_key: ::ethers_core::types::U256,
        pub encrypted_auditor_note: ::ethers_core::types::U256,
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
        name = "EncryptedAuditorNotes",
        abi = "EncryptedAuditorNotes((uint64,uint256,uint256)[])"
    )]
    pub struct EncryptedAuditorNotesFilter {
        pub notes: ::std::vec::Vec<AuditorNote>,
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
    pub enum CommitmentPoolERC20Events {
        CommitmentIncludedFilter(CommitmentIncludedFilter),
        CommitmentQueuedFilter(CommitmentQueuedFilter),
        CommitmentSpentFilter(CommitmentSpentFilter),
        EncryptedAuditorNoteFilter(EncryptedAuditorNoteFilter),
        EncryptedAuditorNotesFilter(EncryptedAuditorNotesFilter),
    }
    impl ::ethers_contract::EthLogDecode for CommitmentPoolERC20Events {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = CommitmentIncludedFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::CommitmentIncludedFilter(decoded));
            }
            if let Ok(decoded) = CommitmentQueuedFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::CommitmentQueuedFilter(decoded));
            }
            if let Ok(decoded) = CommitmentSpentFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::CommitmentSpentFilter(decoded));
            }
            if let Ok(decoded) = EncryptedAuditorNoteFilter::decode_log(log) {
                return Ok(
                    CommitmentPoolERC20Events::EncryptedAuditorNoteFilter(decoded),
                );
            }
            if let Ok(decoded) = EncryptedAuditorNotesFilter::decode_log(log) {
                return Ok(
                    CommitmentPoolERC20Events::EncryptedAuditorNotesFilter(decoded),
                );
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CommitmentPoolERC20Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CommitmentIncludedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CommitmentQueuedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CommitmentSpentFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncryptedAuditorNoteFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncryptedAuditorNotesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CommitmentIncludedFilter> for CommitmentPoolERC20Events {
        fn from(value: CommitmentIncludedFilter) -> Self {
            Self::CommitmentIncludedFilter(value)
        }
    }
    impl ::core::convert::From<CommitmentQueuedFilter> for CommitmentPoolERC20Events {
        fn from(value: CommitmentQueuedFilter) -> Self {
            Self::CommitmentQueuedFilter(value)
        }
    }
    impl ::core::convert::From<CommitmentSpentFilter> for CommitmentPoolERC20Events {
        fn from(value: CommitmentSpentFilter) -> Self {
            Self::CommitmentSpentFilter(value)
        }
    }
    impl ::core::convert::From<EncryptedAuditorNoteFilter>
    for CommitmentPoolERC20Events {
        fn from(value: EncryptedAuditorNoteFilter) -> Self {
            Self::EncryptedAuditorNoteFilter(value)
        }
    }
    impl ::core::convert::From<EncryptedAuditorNotesFilter>
    for CommitmentPoolERC20Events {
        fn from(value: EncryptedAuditorNotesFilter) -> Self {
            Self::EncryptedAuditorNotesFilter(value)
        }
    }
    ///Container type for all input parameters for the `AUDITOR_COUNT` function with signature `AUDITOR_COUNT()` and selector `0xa592bd69`
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
    #[ethcall(name = "AUDITOR_COUNT", abi = "AUDITOR_COUNT()")]
    pub struct AuditorCountCall;
    ///Container type for all input parameters for the `_pathIndices` function with signature `_pathIndices(uint256,uint32)` and selector `0xf2da1d41`
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
    #[ethcall(name = "_pathIndices", abi = "_pathIndices(uint256,uint32)")]
    pub struct PathIndicesCall {
        pub full_path: ::ethers_core::types::U256,
        pub rollup_size: u32,
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
    ///Container type for all input parameters for the `defaultMinRollupFee` function with signature `defaultMinRollupFee()` and selector `0xb2316c33`
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
    #[ethcall(name = "defaultMinRollupFee", abi = "defaultMinRollupFee()")]
    pub struct DefaultMinRollupFeeCall;
    ///Container type for all input parameters for the `enqueue` function with signature `enqueue((uint256,uint256,uint256,uint256,bytes),address)` and selector `0x78d60cd7`
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
        name = "enqueue",
        abi = "enqueue((uint256,uint256,uint256,uint256,bytes),address)"
    )]
    pub struct EnqueueCall {
        pub request: CommitmentRequest,
        pub executor: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `getAllAuditorPublicKeys` function with signature `getAllAuditorPublicKeys()` and selector `0x63bc7d32`
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
    #[ethcall(name = "getAllAuditorPublicKeys", abi = "getAllAuditorPublicKeys()")]
    pub struct GetAllAuditorPublicKeysCall;
    ///Container type for all input parameters for the `getAuditorPublicKey` function with signature `getAuditorPublicKey(uint256)` and selector `0x87780df9`
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
    #[ethcall(name = "getAuditorPublicKey", abi = "getAuditorPublicKey(uint256)")]
    pub struct GetAuditorPublicKeyCall {
        pub index: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `getCommitmentCount` function with signature `getCommitmentCount()` and selector `0x5688881f`
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
    #[ethcall(name = "getCommitmentCount", abi = "getCommitmentCount()")]
    pub struct GetCommitmentCountCall;
    ///Container type for all input parameters for the `getCommitmentIncludedCount` function with signature `getCommitmentIncludedCount()` and selector `0xe500f504`
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
    #[ethcall(name = "getCommitmentIncludedCount", abi = "getCommitmentIncludedCount()")]
    pub struct GetCommitmentIncludedCountCall;
    ///Container type for all input parameters for the `getCommitmentQueuedCount` function with signature `getCommitmentQueuedCount()` and selector `0x555d75f0`
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
    #[ethcall(name = "getCommitmentQueuedCount", abi = "getCommitmentQueuedCount()")]
    pub struct GetCommitmentQueuedCountCall;
    ///Container type for all input parameters for the `getMinRollupFee` function with signature `getMinRollupFee()` and selector `0xb08892d0`
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
    #[ethcall(name = "getMinRollupFee", abi = "getMinRollupFee()")]
    pub struct GetMinRollupFeeCall;
    ///Container type for all input parameters for the `getNullifierCount` function with signature `getNullifierCount()` and selector `0x7a553744`
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
    #[ethcall(name = "getNullifierCount", abi = "getNullifierCount()")]
    pub struct GetNullifierCountCall;
    ///Container type for all input parameters for the `getQueuedCommitments` function with signature `getQueuedCommitments()` and selector `0x866ac658`
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
    #[ethcall(name = "getQueuedCommitments", abi = "getQueuedCommitments()")]
    pub struct GetQueuedCommitmentsCall;
    ///Container type for all input parameters for the `getTreeCapacity` function with signature `getTreeCapacity()` and selector `0x484eb652`
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
    #[ethcall(name = "getTreeCapacity", abi = "getTreeCapacity()")]
    pub struct GetTreeCapacityCall;
    ///Container type for all input parameters for the `isHistoricCommitment` function with signature `isHistoricCommitment(uint256)` and selector `0x57060016`
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
    #[ethcall(name = "isHistoricCommitment", abi = "isHistoricCommitment(uint256)")]
    pub struct IsHistoricCommitmentCall {
        pub commitment: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `0xa6232a93`
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
    #[ethcall(name = "isKnownRoot", abi = "isKnownRoot(uint256)")]
    pub struct IsKnownRootCall {
        pub root: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `isSpentSerialNumber` function with signature `isSpentSerialNumber(uint256)` and selector `0x3bb8d1b4`
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
    #[ethcall(name = "isSpentSerialNumber", abi = "isSpentSerialNumber(uint256)")]
    pub struct IsSpentSerialNumberCall {
        pub serial_number: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `rollup` function with signature `rollup((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256))` and selector `0x14a7737d`
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
        name = "rollup",
        abi = "rollup((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256))"
    )]
    pub struct RollupCall {
        pub request: RollupRequest,
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
    ///Container type for all input parameters for the `transact` function with signature `transact((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[]),bytes)` and selector `0x72082971`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[ethcall(
        name = "transact",
        abi = "transact((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[]),bytes)"
    )]
    pub struct TransactCall {
        pub request: TransactRequest,
        pub signature: ::ethers_core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub enum CommitmentPoolERC20Calls {
        AuditorCount(AuditorCountCall),
        PathIndices(PathIndicesCall),
        AssetAddress(AssetAddressCall),
        AssetDecimals(AssetDecimalsCall),
        AssetName(AssetNameCall),
        AssetSymbol(AssetSymbolCall),
        AssetType(AssetTypeCall),
        DefaultMinRollupFee(DefaultMinRollupFeeCall),
        Enqueue(EnqueueCall),
        GetAllAuditorPublicKeys(GetAllAuditorPublicKeysCall),
        GetAuditorPublicKey(GetAuditorPublicKeyCall),
        GetCommitmentCount(GetCommitmentCountCall),
        GetCommitmentIncludedCount(GetCommitmentIncludedCountCall),
        GetCommitmentQueuedCount(GetCommitmentQueuedCountCall),
        GetMinRollupFee(GetMinRollupFeeCall),
        GetNullifierCount(GetNullifierCountCall),
        GetQueuedCommitments(GetQueuedCommitmentsCall),
        GetTreeCapacity(GetTreeCapacityCall),
        IsHistoricCommitment(IsHistoricCommitmentCall),
        IsKnownRoot(IsKnownRootCall),
        IsSpentSerialNumber(IsSpentSerialNumberCall),
        Rollup(RollupCall),
        Settings(SettingsCall),
        Transact(TransactCall),
    }
    impl ::ethers_core::abi::AbiDecode for CommitmentPoolERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AuditorCountCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AuditorCount(decoded));
            }
            if let Ok(decoded)
                = <PathIndicesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PathIndices(decoded));
            }
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
                = <DefaultMinRollupFeeCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultMinRollupFee(decoded));
            }
            if let Ok(decoded)
                = <EnqueueCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Enqueue(decoded));
            }
            if let Ok(decoded)
                = <GetAllAuditorPublicKeysCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAllAuditorPublicKeys(decoded));
            }
            if let Ok(decoded)
                = <GetAuditorPublicKeyCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAuditorPublicKey(decoded));
            }
            if let Ok(decoded)
                = <GetCommitmentCountCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetCommitmentCount(decoded));
            }
            if let Ok(decoded)
                = <GetCommitmentIncludedCountCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetCommitmentIncludedCount(decoded));
            }
            if let Ok(decoded)
                = <GetCommitmentQueuedCountCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetCommitmentQueuedCount(decoded));
            }
            if let Ok(decoded)
                = <GetMinRollupFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinRollupFee(decoded));
            }
            if let Ok(decoded)
                = <GetNullifierCountCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetNullifierCount(decoded));
            }
            if let Ok(decoded)
                = <GetQueuedCommitmentsCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetQueuedCommitments(decoded));
            }
            if let Ok(decoded)
                = <GetTreeCapacityCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTreeCapacity(decoded));
            }
            if let Ok(decoded)
                = <IsHistoricCommitmentCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsHistoricCommitment(decoded));
            }
            if let Ok(decoded)
                = <IsKnownRootCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsKnownRoot(decoded));
            }
            if let Ok(decoded)
                = <IsSpentSerialNumberCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsSpentSerialNumber(decoded));
            }
            if let Ok(decoded)
                = <RollupCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rollup(decoded));
            }
            if let Ok(decoded)
                = <SettingsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Settings(decoded));
            }
            if let Ok(decoded)
                = <TransactCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transact(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for CommitmentPoolERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AuditorCount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PathIndices(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
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
                Self::DefaultMinRollupFee(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Enqueue(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetAllAuditorPublicKeys(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetAuditorPublicKey(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetCommitmentCount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetCommitmentIncludedCount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetCommitmentQueuedCount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetMinRollupFee(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetNullifierCount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetQueuedCommitments(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetTreeCapacity(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsHistoricCommitment(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsKnownRoot(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsSpentSerialNumber(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Rollup(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Settings(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Transact(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CommitmentPoolERC20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AuditorCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::PathIndices(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetName(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetSymbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetType(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultMinRollupFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Enqueue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllAuditorPublicKeys(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAuditorPublicKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCommitmentCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCommitmentIncludedCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCommitmentQueuedCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNullifierCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetQueuedCommitments(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTreeCapacity(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsHistoricCommitment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsKnownRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSpentSerialNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Rollup(element) => ::core::fmt::Display::fmt(element, f),
                Self::Settings(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transact(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AuditorCountCall> for CommitmentPoolERC20Calls {
        fn from(value: AuditorCountCall) -> Self {
            Self::AuditorCount(value)
        }
    }
    impl ::core::convert::From<PathIndicesCall> for CommitmentPoolERC20Calls {
        fn from(value: PathIndicesCall) -> Self {
            Self::PathIndices(value)
        }
    }
    impl ::core::convert::From<AssetAddressCall> for CommitmentPoolERC20Calls {
        fn from(value: AssetAddressCall) -> Self {
            Self::AssetAddress(value)
        }
    }
    impl ::core::convert::From<AssetDecimalsCall> for CommitmentPoolERC20Calls {
        fn from(value: AssetDecimalsCall) -> Self {
            Self::AssetDecimals(value)
        }
    }
    impl ::core::convert::From<AssetNameCall> for CommitmentPoolERC20Calls {
        fn from(value: AssetNameCall) -> Self {
            Self::AssetName(value)
        }
    }
    impl ::core::convert::From<AssetSymbolCall> for CommitmentPoolERC20Calls {
        fn from(value: AssetSymbolCall) -> Self {
            Self::AssetSymbol(value)
        }
    }
    impl ::core::convert::From<AssetTypeCall> for CommitmentPoolERC20Calls {
        fn from(value: AssetTypeCall) -> Self {
            Self::AssetType(value)
        }
    }
    impl ::core::convert::From<DefaultMinRollupFeeCall> for CommitmentPoolERC20Calls {
        fn from(value: DefaultMinRollupFeeCall) -> Self {
            Self::DefaultMinRollupFee(value)
        }
    }
    impl ::core::convert::From<EnqueueCall> for CommitmentPoolERC20Calls {
        fn from(value: EnqueueCall) -> Self {
            Self::Enqueue(value)
        }
    }
    impl ::core::convert::From<GetAllAuditorPublicKeysCall>
    for CommitmentPoolERC20Calls {
        fn from(value: GetAllAuditorPublicKeysCall) -> Self {
            Self::GetAllAuditorPublicKeys(value)
        }
    }
    impl ::core::convert::From<GetAuditorPublicKeyCall> for CommitmentPoolERC20Calls {
        fn from(value: GetAuditorPublicKeyCall) -> Self {
            Self::GetAuditorPublicKey(value)
        }
    }
    impl ::core::convert::From<GetCommitmentCountCall> for CommitmentPoolERC20Calls {
        fn from(value: GetCommitmentCountCall) -> Self {
            Self::GetCommitmentCount(value)
        }
    }
    impl ::core::convert::From<GetCommitmentIncludedCountCall>
    for CommitmentPoolERC20Calls {
        fn from(value: GetCommitmentIncludedCountCall) -> Self {
            Self::GetCommitmentIncludedCount(value)
        }
    }
    impl ::core::convert::From<GetCommitmentQueuedCountCall>
    for CommitmentPoolERC20Calls {
        fn from(value: GetCommitmentQueuedCountCall) -> Self {
            Self::GetCommitmentQueuedCount(value)
        }
    }
    impl ::core::convert::From<GetMinRollupFeeCall> for CommitmentPoolERC20Calls {
        fn from(value: GetMinRollupFeeCall) -> Self {
            Self::GetMinRollupFee(value)
        }
    }
    impl ::core::convert::From<GetNullifierCountCall> for CommitmentPoolERC20Calls {
        fn from(value: GetNullifierCountCall) -> Self {
            Self::GetNullifierCount(value)
        }
    }
    impl ::core::convert::From<GetQueuedCommitmentsCall> for CommitmentPoolERC20Calls {
        fn from(value: GetQueuedCommitmentsCall) -> Self {
            Self::GetQueuedCommitments(value)
        }
    }
    impl ::core::convert::From<GetTreeCapacityCall> for CommitmentPoolERC20Calls {
        fn from(value: GetTreeCapacityCall) -> Self {
            Self::GetTreeCapacity(value)
        }
    }
    impl ::core::convert::From<IsHistoricCommitmentCall> for CommitmentPoolERC20Calls {
        fn from(value: IsHistoricCommitmentCall) -> Self {
            Self::IsHistoricCommitment(value)
        }
    }
    impl ::core::convert::From<IsKnownRootCall> for CommitmentPoolERC20Calls {
        fn from(value: IsKnownRootCall) -> Self {
            Self::IsKnownRoot(value)
        }
    }
    impl ::core::convert::From<IsSpentSerialNumberCall> for CommitmentPoolERC20Calls {
        fn from(value: IsSpentSerialNumberCall) -> Self {
            Self::IsSpentSerialNumber(value)
        }
    }
    impl ::core::convert::From<RollupCall> for CommitmentPoolERC20Calls {
        fn from(value: RollupCall) -> Self {
            Self::Rollup(value)
        }
    }
    impl ::core::convert::From<SettingsCall> for CommitmentPoolERC20Calls {
        fn from(value: SettingsCall) -> Self {
            Self::Settings(value)
        }
    }
    impl ::core::convert::From<TransactCall> for CommitmentPoolERC20Calls {
        fn from(value: TransactCall) -> Self {
            Self::Transact(value)
        }
    }
    ///Container type for all return fields from the `AUDITOR_COUNT` function with signature `AUDITOR_COUNT()` and selector `0xa592bd69`
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
    pub struct AuditorCountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `_pathIndices` function with signature `_pathIndices(uint256,uint32)` and selector `0xf2da1d41`
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
    pub struct PathIndicesReturn(pub ::ethers_core::types::U256);
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
    ///Container type for all return fields from the `defaultMinRollupFee` function with signature `defaultMinRollupFee()` and selector `0xb2316c33`
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
    pub struct DefaultMinRollupFeeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getAllAuditorPublicKeys` function with signature `getAllAuditorPublicKeys()` and selector `0x63bc7d32`
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
    pub struct GetAllAuditorPublicKeysReturn(
        pub ::std::vec::Vec<::ethers_core::types::U256>,
    );
    ///Container type for all return fields from the `getAuditorPublicKey` function with signature `getAuditorPublicKey(uint256)` and selector `0x87780df9`
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
    pub struct GetAuditorPublicKeyReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getCommitmentCount` function with signature `getCommitmentCount()` and selector `0x5688881f`
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
    pub struct GetCommitmentCountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getCommitmentIncludedCount` function with signature `getCommitmentIncludedCount()` and selector `0xe500f504`
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
    pub struct GetCommitmentIncludedCountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getCommitmentQueuedCount` function with signature `getCommitmentQueuedCount()` and selector `0x555d75f0`
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
    pub struct GetCommitmentQueuedCountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getMinRollupFee` function with signature `getMinRollupFee()` and selector `0xb08892d0`
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
    pub struct GetMinRollupFeeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getNullifierCount` function with signature `getNullifierCount()` and selector `0x7a553744`
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
    pub struct GetNullifierCountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getQueuedCommitments` function with signature `getQueuedCommitments()` and selector `0x866ac658`
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
    pub struct GetQueuedCommitmentsReturn(
        pub ::std::vec::Vec<::ethers_core::types::U256>,
    );
    ///Container type for all return fields from the `getTreeCapacity` function with signature `getTreeCapacity()` and selector `0x484eb652`
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
    pub struct GetTreeCapacityReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `isHistoricCommitment` function with signature `isHistoricCommitment(uint256)` and selector `0x57060016`
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
    pub struct IsHistoricCommitmentReturn(pub bool);
    ///Container type for all return fields from the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `0xa6232a93`
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
    pub struct IsKnownRootReturn(pub bool);
    ///Container type for all return fields from the `isSpentSerialNumber` function with signature `isSpentSerialNumber(uint256)` and selector `0x3bb8d1b4`
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
    pub struct IsSpentSerialNumberReturn(pub bool);
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
    ///`AuditorNote(uint64,uint256,uint256)`
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
    pub struct AuditorNote {
        pub id: u64,
        pub public_key: ::ethers_core::types::U256,
        pub note: ::ethers_core::types::U256,
    }
    ///`CommitmentRequest(uint256,uint256,uint256,uint256,bytes)`
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
    pub struct CommitmentRequest {
        pub amount: ::ethers_core::types::U256,
        pub commitment: ::ethers_core::types::U256,
        pub executor_fee: ::ethers_core::types::U256,
        pub rollup_fee: ::ethers_core::types::U256,
        pub encrypted_note: ::ethers_core::types::Bytes,
    }
    ///`RollupRequest(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256)`
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
    pub struct RollupRequest {
        pub proof: Proof,
        pub rollup_size: u32,
        pub new_root: ::ethers_core::types::U256,
        pub leaf_hash: ::ethers_core::types::U256,
    }
    ///`TransactRequest(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[])`
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
    pub struct TransactRequest {
        pub proof: Proof,
        pub root_hash: ::ethers_core::types::U256,
        pub serial_numbers: ::std::vec::Vec<::ethers_core::types::U256>,
        pub sig_hashes: ::std::vec::Vec<::ethers_core::types::U256>,
        pub sig_pk: [u8; 32],
        pub public_amount: ::ethers_core::types::U256,
        pub relayer_fee_amount: ::ethers_core::types::U256,
        pub out_commitments: ::std::vec::Vec<::ethers_core::types::U256>,
        pub out_rollup_fees: ::std::vec::Vec<::ethers_core::types::U256>,
        pub public_recipient: ::ethers_core::types::Address,
        pub relayer_address: ::ethers_core::types::Address,
        pub out_encrypted_notes: ::std::vec::Vec<::ethers_core::types::Bytes>,
        pub random_auditing_public_key: ::ethers_core::types::U256,
        pub encrypted_auditor_notes: ::std::vec::Vec<::ethers_core::types::U256>,
    }
    ///`G1Point(uint256,uint256)`
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
    pub struct G1Point {
        pub x: ::ethers_core::types::U256,
        pub y: ::ethers_core::types::U256,
    }
    ///`G2Point(uint256[2],uint256[2])`
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
    pub struct G2Point {
        pub x: [::ethers_core::types::U256; 2],
        pub y: [::ethers_core::types::U256; 2],
    }
    ///`Proof((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`
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
    pub struct Proof {
        pub a: G1Point,
        pub b: G2Point,
        pub c: G1Point,
    }
}
