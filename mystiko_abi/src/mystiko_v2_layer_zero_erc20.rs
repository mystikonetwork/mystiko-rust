pub use mystiko_v2_layer_zero_erc20::*;
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
pub mod mystiko_v2_layer_zero_erc20 {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_hasher3"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                            "contract IHasher3"
                        ),),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                            "contract IERC20Metadata"
                        ),),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_bridgeProxyAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_settingsCenter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_localConfig"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        ],),
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                            "struct IMystikoBridge.BridgeLocalConfig",
                        ),),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_peerConfig"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        ],),
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                            "struct IMystikoBridge.BridgePeerConfig",
                        ),),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("assetAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assetAddress"),
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
                    ::std::borrow::ToOwned::to_owned("assetDecimals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assetDecimals"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint8"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assetName"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assetName"),
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
                    ::std::borrow::ToOwned::to_owned("assetSymbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assetSymbol"),
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
                    ::std::borrow::ToOwned::to_owned("assetType"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assetType"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "enum AssetPool.AssetType"
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bridgeProxyAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("bridgeProxyAddress"),
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
                    ::std::borrow::ToOwned::to_owned("bridgeType"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("bridgeType"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("certDeposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("certDeposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_request"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "struct IMystikoBridge.BridgeDepositRequest",
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_certificateDeadline",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_certificateSignature",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultMaxAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("defaultMaxAmount"),
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
                    ::std::borrow::ToOwned::to_owned("defaultMinAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("defaultMinAmount"),
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
                    ::std::borrow::ToOwned::to_owned("defaultMinBridgeFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("defaultMinBridgeFee",),
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
                    ::std::borrow::ToOwned::to_owned("defaultPeerMinExecutorFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("defaultPeerMinExecutorFee",),
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
                    ::std::borrow::ToOwned::to_owned("defaultPeerMinRollupFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("defaultPeerMinRollupFee",),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deposit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_request"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct IMystikoBridge.BridgeDepositRequest",
                            ),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("failedMessages"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("failedMessages"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("forceResumeReceive"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("forceResumeReceive"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
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
                    ::std::borrow::ToOwned::to_owned("getAssociatedCommitmentPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAssociatedCommitmentPool",),
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
                    ::std::borrow::ToOwned::to_owned("getConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getConfig"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_version"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_configType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMaxAmount"),
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
                    ::std::borrow::ToOwned::to_owned("getMinAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMinAmount"),
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
                    ::std::borrow::ToOwned::to_owned("getMinBridgeFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMinBridgeFee"),
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
                    ::std::borrow::ToOwned::to_owned("getMinExecutorFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMinExecutorFee"),
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
                    ::std::borrow::ToOwned::to_owned("getPeerMinRollupFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPeerMinRollupFee",),
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
                    ::std::borrow::ToOwned::to_owned("isCertificateCheckEnabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isCertificateCheckEnabled",),
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
                    ::std::borrow::ToOwned::to_owned("isDepositsDisabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isDepositsDisabled"),
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
                    ::std::borrow::ToOwned::to_owned("isPeerContractSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isPeerContractSet"),
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
                    ::std::borrow::ToOwned::to_owned("isTrustedRemote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isTrustedRemote"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
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
                    ::std::borrow::ToOwned::to_owned("localLayerZeroChainId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("localLayerZeroChainId",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lzEndpoint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lzEndpoint"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract ILayerZeroEndpoint",
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lzReceive"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lzReceive"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
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
                    ::std::borrow::ToOwned::to_owned("nonblockingLzReceive"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nonblockingLzReceive",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("peerChainId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("peerChainId"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peerChainName"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("peerChainName"),
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
                    ::std::borrow::ToOwned::to_owned("peerContract"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("peerContract"),
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
                    ::std::borrow::ToOwned::to_owned("peerLayerZeroChainId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("peerLayerZeroChainId",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("retryMessage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("retryMessage"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setConfig"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_version"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_configType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_config"),
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
                    ::std::borrow::ToOwned::to_owned("setEndpoint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setEndpoint"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_lzChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_lzEndpoint"),
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
                    ::std::borrow::ToOwned::to_owned("setPeerContract"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPeerContract"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_peerContract"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::String,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct IMystikoBridge.BridgePeerContract",
                            ),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setReceiveVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setReceiveVersion"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSendVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSendVersion"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTrustedRemote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setTrustedRemote"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_peerLayerZeroChainId",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_peerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("settings"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("settings"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract MystikoBridgeSettings",
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("trustedRemoteLookup"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("trustedRemoteLookup",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentCrossChain"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CommitmentCrossChain",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("commitment"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MessageFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MessageFailed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetTrustedRemote"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetTrustedRemote"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("target"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AmountLessThanZero"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AmountLessThanZero"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AmountTooLarge"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AmountTooLarge"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AmountTooSmall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AmountTooSmall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssociatedPoolNotSet"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AssociatedPoolNotSet",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BridgeFeeTooFew"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BridgeFeeTooFew"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CallIsNotLzApp"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CallIsNotLzApp"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CertificateInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CertificateInvalid"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentHashIncorrect"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CommitmentHashIncorrect",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositsDisabled"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DepositsDisabled"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DestinationChainIsNotTrusted"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DestinationChainIsNotTrusted",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutorFeeTooFew"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ExecutorFeeTooFew"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HashKGreaterThanFieldSize"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("HashKGreaterThanFieldSize",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Invalid"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("param"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoStoredMessage"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoStoredMessage"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotSupport"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotSupport"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PeerChainIdNotMatched"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PeerChainIdNotMatched",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PeerContractAlreadySet"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PeerContractAlreadySet",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PeerContractNotMatched"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PeerContractNotMatched",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RandomSGreaterThanFieldSize"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RandomSGreaterThanFieldSize",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RollupFeeToFew"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RollupFeeToFew"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SanctionedAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SanctionedAddress"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MYSTIKOV2LAYERZEROERC20_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90U4\x80\x15a\0\x1DW`\0\x80\xFD[P`@Qa:\x0E8\x03\x80a:\x0E\x839\x81\x01`@\x81\x90Ra\0<\x91a\x01\xFEV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x08\x80T\x83\x88\x16\x90\x83\x16\x17\x90U\x83Q`\x03U` \x80\x85\x01Q`\x04U`@\x85\x01Q`\x05U\x83Q`\x06U\x83\x01Q`\x07U`\t\x80T\x92\x86\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x84\x86\x85\x85\x85\x852\x80a\0\xC6W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[a\0\xCF\x81a\x01\x02V[PP`\x0E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16\x95\x90\x95\x17\x90\x94UPa\x02\xAA\x98PPPPPPPPPV[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01iW`\0\x80\xFD[PV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\x9CWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0`@\x82\x84\x03\x12\x15a\x01\xB4W`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\xE4WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x01 \x81\x12\x15a\x02\x19W`\0\x80\xFD[\x87Qa\x02$\x81a\x01TV[` \x89\x01Q\x90\x97Pa\x025\x81a\x01TV[`@\x89\x01Q\x90\x96Pa\x02F\x81a\x01TV[``\x89\x01Q\x90\x95Pa\x02W\x81a\x01TV[\x93P```\x7F\x19\x82\x01\x12\x15a\x02kW`\0\x80\xFD[Pa\x02ta\x01lV[`\x80\x88\x01Q\x81R`\xA0\x88\x01Q` \x82\x01R`\xC0\x88\x01Q`@\x82\x01R\x91Pa\x02\x9E\x88`\xE0\x89\x01a\x01\xA2V[\x90P\x92\x95P\x92\x95P\x92\x95V[a7U\x80a\x02\xB9`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x86W`\x005`\xE0\x1C\x80c\x82[_\x8D\x11a\x01ZW\x80c\xD0\xB46\xBD\x11a\0\xC1W\x80c\xEF\xBF\xB2\xAE\x11a\0zW\x80c\xEF\xBF\xB2\xAE\x14a\x07\xAAW\x80c\xF2\xFD\xE3\x8B\x14a\x07\xBFW\x80c\xF4\xAD\x17\xC6\x14a\x07\xDFW\x80c\xF5\xEC\xBD\xBC\x14a\x07\xF4W\x80c\xFAu\x0FV\x14a\x08\x14W\x80c\xFB>=s\x14a\x085W`\0\x80\xFD[\x80c\xD0\xB46\xBD\x14a\x07\x17W\x80c\xD1\xDE\xBA\x1F\x14a\x07-W\x80c\xDD\xAC]\xC1\x14a\x07@W\x80c\xE0at\xE4\x14a\x07UW\x80c\xEB\x8Dr\xB7\x14a\x07uW\x80c\xEDn\xA3:\x14a\x07\x95W`\0\x80\xFD[\x80c\xC9#\x0C]\x11a\x01\x13W\x80c\xC9#\x0C]\x14a\x06eW\x80c\xCB\\\x02\x9A\x14a\x06zW\x80c\xCB\xE3B\x85\x14a\x06\x8DW\x80c\xCB\xED\x8B\x9C\x14a\x06\xA3W\x80c\xCD\xFC\xEE\xBA\x14a\x06\xC3W\x80c\xCF\xC7\xE2\xDA\x14a\x07\x02W`\0\x80\xFD[\x80c\x82[_\x8D\x14a\x05\xC3W\x80c\x8D\xA5\xCB[\x14a\x05\xD8W\x80c\x9A\x03cl\x14a\x05\xF6W\x80c\xB3S\xAA\xA7\x14a\x06\tW\x80c\xBCXw\x06\x14a\x06)W\x80c\xC2\xD4\x16\x01\x14a\x06>W`\0\x80\xFD[\x80c=\x8B8\xF6\x11a\x01\xFEW\x80cN\xE7\xDE\xD6\x11a\x01\xB7W\x80cN\xE7\xDE\xD6\x14a\x04\xE9W\x80c[\x8CA\xE6\x14a\x05\tW\x80cd\x0C\x0B6\x14a\x05XW\x80cf\xAD\\\x8A\x14a\x05nW\x80cqP\x18\xA6\x14a\x05\x8EW\x80cu3\xD7\x88\x14a\x05\xA3W`\0\x80\xFD[\x80c=\x8B8\xF6\x14a\x042W\x80c?\xE34z\x14a\x04bW\x80cB.\0(\x14a\x04~W\x80cB\xD6Z\x8D\x14a\x04\x9EW\x80cM\xDEo\xBC\x14a\x04\xBEW\x80cN<\x10\xB7\x14a\x04\xD4W`\0\x80\xFD[\x80c\x17m\xE7\xA8\x11a\x02PW\x80c\x17m\xE7\xA8\x14a\x03JW\x80c\x1B\xA4l\xFD\x14a\x03lW\x80c!\xE3-U\x14a\x03\x9EW\x80c$!\xE1U\x14a\x03\xBEW\x80c,\xD2mE\x14a\x03\xF0W\x80c0-_K\x14a\x04\x10W`\0\x80\xFD[\x80b\x1D5g\x14a\x02\x8BW\x80b\x97\xA0c\x14a\x02\xADW\x80c\x07\xE0\xDB\x17\x14a\x02\xE7W\x80c\x0B\xA9Y\t\x14a\x03\x07W\x80c\x10\xDD\xB17\x14a\x03*W[`\0\x80\xFD[4\x80\x15a\x02\x97W`\0\x80\xFD[Pa\x02\xABa\x02\xA66`\x04a+\xD0V[a\x08KV[\0[4\x80\x15a\x02\xB9W`\0\x80\xFD[P`\x0BTa\x02\xCF\x90`\x01`\xB0\x1B\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xF3W`\0\x80\xFD[Pa\x02\xABa\x03\x026`\x04a,XV[a\t\xC4V[4\x80\x15a\x03\x13W`\0\x80\xFD[Pa\x03\x1Ca\n*V[`@Q\x90\x81R` \x01a\x02\xDEV[4\x80\x15a\x036W`\0\x80\xFD[Pa\x02\xABa\x03E6`\x04a,XV[a\n\xB3V[4\x80\x15a\x03VW`\0\x80\xFD[Pa\x03_a\n\xEFV[`@Qa\x02\xDE\x91\x90a,\xC3V[4\x80\x15a\x03xW`\0\x80\xFD[P`\x0ET`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xDEV[4\x80\x15a\x03\xAAW`\0\x80\xFD[P`\x02Ta\x03\x86\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03\xCAW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\t\x81RhlayerZero`\xB8\x1B` \x82\x01Ra\x03_V[4\x80\x15a\x03\xFCW`\0\x80\xFD[P`\x08Ta\x03\x86\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\x1CW`\0\x80\xFD[P`\x0BTa\x02\xCF\x90`\x01`\xA0\x1B\x90\x04a\xFF\xFF\x16\x81V[4\x80\x15a\x04>W`\0\x80\xFD[Pa\x04Ra\x04M6`\x04a-\x17V[a\x0BfV[`@Q\x90\x15\x15\x81R` \x01a\x02\xDEV[4\x80\x15a\x04nW`\0\x80\xFD[P`\0`@Qa\x02\xDE\x91\x90a-iV[4\x80\x15a\x04\x8AW`\0\x80\xFD[Pa\x02\xABa\x04\x996`\x04a-\xA6V[a\x0C3V[4\x80\x15a\x04\xAAW`\0\x80\xFD[Pa\x02\xABa\x04\xB96`\x04a-\x17V[a\x0C\xD8V[4\x80\x15a\x04\xCAW`\0\x80\xFD[Pa\x03\x1C`\x05T\x81V[4\x80\x15a\x04\xE0W`\0\x80\xFD[Pa\x03_a\rKV[4\x80\x15a\x04\xF5W`\0\x80\xFD[Pa\x02\xABa\x05\x046`\x04a.QV[a\r\xD9V[4\x80\x15a\x05\x15W`\0\x80\xFD[Pa\x03\x1Ca\x05$6`\x04a.\x88V[`\r` \x90\x81R`\0\x93\x84R`@\x80\x85 \x84Q\x80\x86\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x95\x84\x01\x95\x90\x95 \x94R\x92\x90R\x82R\x90 T\x81V[4\x80\x15a\x05dW`\0\x80\xFD[Pa\x03\x1C`\x06T\x81V[4\x80\x15a\x05zW`\0\x80\xFD[Pa\x02\xABa\x05\x896`\x04a+\xD0V[a\x0E\x1EV[4\x80\x15a\x05\x9AW`\0\x80\xFD[Pa\x02\xABa\x0EPV[4\x80\x15a\x05\xAFW`\0\x80\xFD[Pa\x03_a\x05\xBE6`\x04a,XV[a\x0EdV[4\x80\x15a\x05\xCFW`\0\x80\xFD[Pa\x03\x1Ca\x0E}V[4\x80\x15a\x05\xE4W`\0\x80\xFD[P`\nT`\x01`\x01`\xA0\x1B\x03\x16a\x03\x86V[a\x02\xABa\x06\x046`\x04a/\x93V[a\x0F\x04V[4\x80\x15a\x06\x15W`\0\x80\xFD[P`\x0BTa\x03\x86\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x065W`\0\x80\xFD[Pa\x04Ra\x0F\x1DV[4\x80\x15a\x06JW`\0\x80\xFD[Pa\x06Sa\x0F\x8BV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\xDEV[4\x80\x15a\x06qW`\0\x80\xFD[Pa\x03_a\x0F\xF9V[a\x02\xABa\x06\x886`\x04a/\xCFV[a\x10CV[4\x80\x15a\x06\x99W`\0\x80\xFD[Pa\x03\x1C`\x07T\x81V[4\x80\x15a\x06\xAFW`\0\x80\xFD[Pa\x02\xABa\x06\xBE6`\x04a0?V[a\x14xV[4\x80\x15a\x06\xCFW`\0\x80\xFD[P`\0Ta\x06\xEA\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xDEV[4\x80\x15a\x07\x0EW`\0\x80\xFD[Pa\x03\x1Ca\x14\xF1V[4\x80\x15a\x07#W`\0\x80\xFD[Pa\x03\x1C`\x04T\x81V[a\x02\xABa\x07;6`\x04a+\xD0V[a\x15xV[4\x80\x15a\x07LW`\0\x80\xFD[Pa\x03\x86a\x16pV[4\x80\x15a\x07aW`\0\x80\xFD[P`\tTa\x03\x86\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\x81W`\0\x80\xFD[Pa\x02\xABa\x07\x906`\x04a-\x17V[a\x17\x0FV[4\x80\x15a\x07\xA1W`\0\x80\xFD[Pa\x04Ra\x17\x91V[4\x80\x15a\x07\xB6W`\0\x80\xFD[Pa\x03\x1Ca\x17\xDAV[4\x80\x15a\x07\xCBW`\0\x80\xFD[Pa\x02\xABa\x07\xDA6`\x04a0\xADV[a\x18aV[4\x80\x15a\x07\xEBW`\0\x80\xFD[Pa\x03\x1Ca\x18\x9FV[4\x80\x15a\x08\0W`\0\x80\xFD[Pa\x03_a\x08\x0F6`\x04a0\xCAV[a\x19&V[4\x80\x15a\x08 W`\0\x80\xFD[P`\0Ta\x04R\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x08AW`\0\x80\xFD[Pa\x03\x1C`\x03T\x81V[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\xA6W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn2\xB7287\xB4\xB7:\x101\xB0\xB662\xB9`\x89\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF\x84\x16`\0\x90\x81R`\x0C` R`@\x81 \x80Ta\x08\xC4\x90a1\x17V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xF0\x90a1\x17V[\x80\x15a\t=W\x80`\x1F\x10a\t\x12Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t=V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q\x84Q\x14\x15\x80a\tcWP\x80\x80Q\x90` \x01 \x84\x80Q\x90` \x01 \x14\x15[\x15a\t\xB1W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fsource sending contract\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x9DV[a\t\xBD\x85\x85\x85\x85a\x19\xB9V[PPPPPV[a\t\xCCa\x1A\x1AV[`\x0BT`@Qc\x07\xE0\xDB\x17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07\xE0\xDB\x17\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\x16W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xBDW=`\0\x80>=`\0\xFD[`\tT`@QcG:\x061`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG:\x061\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nwW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9B\x91\x90a1QV[\x90P\x80\x15a\n\xA9W\x80a\n\xADV[`\x04T[\x91PP\x90V[a\n\xBBa\x1A\x1AV[`\x0BT`@Qc\x10\xDD\xB17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x10\xDD\xB17\x90`$\x01a\t\xFCV[`\x0ET`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0B9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Ba\x91\x90\x81\x01\x90a1\x9AV[\x90P\x90V[a\xFF\xFF\x83\x16`\0\x90\x81R`\x0C` R`@\x81 \x80T\x82\x91\x90a\x0B\x87\x90a1\x17V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xB3\x90a1\x17V[\x80\x15a\x0C\0W\x80`\x1F\x10a\x0B\xD5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xE3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83\x83`@Qa\x0C\x17\x92\x91\x90a1\xE2V[`@Q\x80\x91\x03\x90 \x81\x80Q\x90` \x01 \x14\x91PP[\x93\x92PPPV[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0C^W`@Qcm\x8F\x11Q`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\xA8\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U` \x81\x01Q`\x01\x90a\x0C\x9D\x90\x82a29V[P`@\x01Q`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UV[a\x0C\xE0a\x1A\x1AV[`\x0BT`@QcB\xD6Z\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cB\xD6Z\x8D\x90a\r\x14\x90\x86\x90\x86\x90\x86\x90`\x04\x01a3 V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r.W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\rBW=`\0\x80>=`\0\xFD[PPPPPPPV[`\x01\x80Ta\rX\x90a1\x17V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\x84\x90a1\x17V[\x80\x15a\r\xD1W\x80`\x1F\x10a\r\xA6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xD1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xB4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[a\r\xE1a\x1A\x1AV[`\x0B\x80T`\x01`\x01`\xB0\x1B\x03\x19\x16`\x01`\xA0\x1Ba\xFF\xFF\x94\x90\x94\x16\x93\x90\x93\x02`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x92\x17`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x90UV[30\x14a\x0E>W`@Qcq\xF5\x0E\xC1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0EJ\x84\x84\x84\x84a\x1AGV[PPPPV[a\x0EXa\x1A\x1AV[a\x0Eb`\0a\x1A\x81V[V[`\x0C` R`\0\x90\x81R`@\x90 \x80Ta\rX\x90a1\x17V[`\tT`@Qca\x86<\x03`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC3\x0Cx\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xEE\x91\x90a1QV[\x90P\x80\x15a\x0E\xFCW\x80a\n\xADV[PP`\x07T\x90V[`@Qc\xE7\xA2O\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT`@\x80Qc^,;\x83`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xBCXw\x06\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0FgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ba\x91\x90a3>V[`\x0ET`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1<\xE5g\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0F\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ba\x91\x90a3`V[`\x0ET`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0B9W=`\0\x80>=`\0\xFD[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xAF\x91\x90a3>V[\x15a\x10\xCDW`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xBCXw\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11D\x91\x90a3>V[\x15a\x12\x1BW`\0`@Q\x80`\x80\x01`@R\x802`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x11w`\x0ET`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x86\x90R`@\x91\x82\x01\x85\x90R`\tT\x91Qc\x84\x9E\x8B\x9F`\xE0\x1B\x81R\x92\x93P\x16\x90c\x84\x9E\x8B\x9F\x90a\x11\xBB\x90\x84\x90`\x04\x01a3\x83V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xFC\x91\x90a3>V[a\x12\x19W`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[a\x12#a\x14\xF1V[\x83Q\x10\x15a\x12DW`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12La\n*V[\x83Q\x11\x15a\x12mW`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12ua\x17\xDAV[\x83`\xA0\x01Q\x10\x15a\x12\x99W`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xA1a\x18\x9FV[\x83`\xC0\x01Q\x10\x15a\x12\xC5W`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xCDa\x0E}V[\x83`\xE0\x01Q\x10\x15a\x12\xF1W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13\n\x84`@\x01Q\x85`\0\x01Q\x86``\x01Qa\x1A\xD3V[\x90P\x80\x84` \x01Q\x14a\x130W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT`@Qc\xDFY/}`\xE0\x1B\x81R2`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x9C\x91\x90a3>V[\x15a\x13\xBAW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x90\x82\x01R`\xC0\x86\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x85\x01Q``\x82\x01R`\x80\x80\x86\x01Q\x90\x82\x01R`\0a\x13\xFD\x82a\x1B\xCFV[\x90Pa\x14\r\x86`\xA0\x01Q\x82a\x1C>V[a\x14Ba\x14\x18a\x16pV[`\xE0\x88\x01Q`\xC0\x89\x01Q\x89Qa\x14.\x91\x90a3\xCCV[a\x148\x91\x90a3\xCCV[\x88`\xA0\x01Qa\x1CpV[` \x86\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPPPV[a\x14\x80a\x1A\x1AV[`\x0BT`@Qc2\xFBb\xE7`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCB\xED\x8B\x9C\x90a\x14\xB8\x90\x88\x90\x88\x90\x88\x90\x88\x90\x88\x90`\x04\x01a3\xEDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\xD2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xE6W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\tT`@QcU%\x98I`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cU%\x98I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15b\x91\x90a1QV[\x90P\x80\x15a\x15pW\x80a\n\xADV[PP`\x03T\x90V[a\xFF\xFF\x84\x16`\0\x90\x81R`\r` R`@\x80\x82 \x90Qa\x15\x99\x90\x86\x90a4&V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\x01`\x01`@\x1B\x03\x86\x16`\0\x90\x81R\x92R\x90 T\x90P\x80a\x15\xDEW`@Qc+\x96\xC9\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q` \x83\x01 \x81\x14a\x16\x1EW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x07`$\x82\x01Rf\x1C\x18^[\x1B\xD8Y`\xCA\x1B`D\x82\x01R`d\x01a\x08\x9DV[a\xFF\xFF\x85\x16`\0\x90\x81R`\r` R`@\x80\x82 \x90Qa\x16?\x90\x87\x90a4&V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\x01`\x01`@\x1B\x03\x87\x16`\0\x90\x81R\x92R\x90 Ua\t\xBD\x85\x85\x85\x85a\x1AGV[`\tT`@QcA\xFBiy`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cA\xFBiy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE1\x91\x90a4BV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x17\nW`@Qc\x06\xF3\xD63`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a\x17\x17a\x1A\x1AV[`\x0B\x80Ta\xFF\xFF`\xB0\x1B\x19\x16`\x01`\xB0\x1Ba\xFF\xFF\x86\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`\0\x90\x81R`\x0C` R`@\x90 a\x17P\x82\x84\x83a4_V[P\x7F\xFAAHz\xD5\xD6r\x8F\x0B\x19'o\xA1\xED\xDC\x16U\x85x\xF5\x10\x9F\xC3\x9D-\xC3<20G\r\xAB\x83\x83\x83`@Qa\x17\x84\x93\x92\x91\x90a3 V[`@Q\x80\x91\x03\x90\xA1PPPV[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FgW=`\0\x80>=`\0\xFD[`\tT`@Qc{R\xB0\r`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c{R\xB0\r\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18K\x91\x90a1QV[\x90P\x80\x15a\x18YW\x80a\n\xADV[PP`\x05T\x90V[a\x18ia\x1A\x1AV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18\x93W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x08\x9DV[a\x18\x9C\x81a\x1A\x81V[PV[`\tT`@QcO1O\x9D`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9Eb\x9F:\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x10\x91\x90a1QV[\x90P\x80\x15a\x19\x1EW\x80a\n\xADV[PP`\x06T\x90V[`\x0BT`@Qc={/o`\xE2\x1B\x81Ra\xFF\xFF\x80\x87\x16`\x04\x83\x01R\x85\x16`$\x82\x01R0`D\x82\x01R`d\x81\x01\x83\x90R``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF5\xEC\xBD\xBC\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19\xB0\x91\x90\x81\x01\x90a1\x9AV[\x95\x94PPPPPV[`@Qc3V\xAEE`\xE1\x1B\x81R0\x90cf\xAD\\\x8A\x90a\x19\xE2\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a5\x1EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xFCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\x10W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\nT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EbW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x08\x9DV[`\0a\x1AR\x82a\x1C\xD2V[`\0T`\x02T\x91\x92Pa\t\xBD\x91`\x01`\xA8\x1B\x90\x91\x04`\x01`\x01`@\x1B\x03\x16\x90`\x01`\x01`\xA0\x1B\x03\x162\x84a\x1D\x9AV[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x1B\x16W`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83`\x01`\x01`\x80\x1B\x03\x16\x10a\x1B?W`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90R`\x01`\x01`\x80\x1B\x03\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x1B\x8E\x91`\x04\x01a5\\V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xB0\x91\x90a1QV[``\x80a\x1B\xDF\x83`\0\x01Qa\x1ETV[a\x1B\xEC\x84` \x01Qa\x1ETV[a\x1B\xF9\x85`@\x01Qa\x1ETV[a\x1C\x06\x86``\x01Qa\x1ETV[a\x1C\x13\x87`\x80\x01Qa\x1E\xECV[`@Q` \x01a\x1C'\x95\x94\x93\x92\x91\x90a5\x8DV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[a\x1Cl`\x0B`\x16\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16\x823`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x87a\x1F#V[PPV[\x804\x14a\x1C\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x0CNM,\x8C\xEC\xA4\x0C\xCC\xAC\xA4\r\xAD.m\xAC.\x8Cm`k\x1B`D\x82\x01R`d\x01a\x08\x9DV[`\x0ETa\x1C\xCD\x90`\x01`\x01`\xA0\x1B\x03\x163\x85\x85a YV[PPPV[a\x1D\x04`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\x1D6`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\x1DB\x84\x82a \xB3V[\x90\x83R\x90Pa\x1DQ\x84\x82a \xB3V[` \x84\x01\x91\x90\x91R\x90Pa\x1De\x84\x82a \xB3V[`@\x84\x01\x91\x90\x91R\x90Pa\x1Dy\x84\x82a \xB3V[``\x84\x01\x91\x90\x91R\x90Pa\x1D\x8D\x84\x82a!\xE4V[P`\x80\x83\x01RP\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x1D\xC8W`@Qc\x1BI^\xCF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`@\x1B\x03\x85\x81\x16`\x01`\xA8\x1B\x90\x92\x04\x16\x14a\x1D\xFDW`@Qc7;\xC1!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x03a\x1E\x1FW`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E'a\x16pV[`\x01`\x01`\xA0\x1B\x03\x16cx\xD6\x0C\xD7\x82\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xE2\x92\x91\x90a5\xF8V[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x1E\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x9DV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\x1E\xDCW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1E\xBAV[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\x1E\xFA\x81a\"\xF1V[\x83`@Q` \x01a\x1F\x0C\x92\x91\x90a6TV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[a\xFF\xFF\x86\x16`\0\x90\x81R`\x0C` R`@\x81 \x80Ta\x1FA\x90a1\x17V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Fm\x90a1\x17V[\x80\x15a\x1F\xBAW\x80`\x1F\x10a\x1F\x8FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xBAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x9DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q`\0\x03a\x1F\xE3W`@Qc\x02\x0B5\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT`@Qb\xC5\x801`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5\x801\0\x90\x84\x90a \x1E\x90\x8B\x90\x86\x90\x8C\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a6\x83V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a 7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a KW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x0EJ\x90\x85\x90a#\xB8V[`\0\x80\x83Q\x83` a \xC5\x91\x90a3\xCCV[\x11\x15\x80\x15a \xDCWPa \xD9\x83` a3\xCCV[\x83\x10[a!4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\x08\x9DV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a!iW\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa!IV[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a!\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x9DV[\x80a!\xD7\x85` a3\xCCV[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a!\xF3\x85\x85a$\x1BV[\x86Q\x90\x95P\x90\x91Pa\"\x05\x82\x86a3\xCCV[\x11\x15\x80\x15a\"\x1BWPa\"\x18\x81\x85a3\xCCV[\x84\x10[a\"sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\x08\x9DV[``\x81\x15\x80\x15a\"\x8EW`@Q\x91P` \x82\x01`@Ra\"\xD8V[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a\"\xC7W\x80Q\x83R` \x92\x83\x01\x92\x01a\"\xAFV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a\"\xE4\x83\x87a3\xCCV[\x93P\x93PPP\x92P\x92\x90PV[```\xFD\x82`\x01`\x01`@\x1B\x03\x16\x10\x15a#%W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91R[\x92\x91PPV[a\xFF\xFF\x82`\x01`\x01`@\x1B\x03\x16\x11a#tWa#D`\xFD`\xF8\x1Ba%\x8EV[a#M\x83a%\xB5V[`@Q` \x01a#^\x92\x91\x90a6TV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82`\x01`\x01`@\x1B\x03\x16\x11a#\x9EWa#\x95`\x7F`\xF9\x1Ba%\x8EV[a#M\x83a%\xF8V[a#\xAF`\x01`\x01`\xF8\x1B\x03\x19a%\x8EV[a#M\x83a&;V[`\0a#\xCD`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a&~V[\x90P\x80Q`\0\x14\x15\x80\x15a#\xF2WP\x80\x80` \x01\x90Q\x81\x01\x90a#\xF0\x91\x90a3>V[\x15[\x15a\x1C\xCDW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x08\x9DV[`\0\x80`\0a$*\x85\x85a&\x8CV[\x94P\x90P`\0`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\xFD`\xF8\x1B\x03a$\xC2Wa$O\x86\x86a'\x14V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a$jWPa\xFF\xFF\x81\x11\x15[a$\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\x08\x9DV[\x92P\x83\x91Pa!\xDD\x90PV[`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\x7F`\xF9\x1B\x03a%\x1CWa$\xE1\x86\x86a'\xCDV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a%\0WPc\xFF\xFF\xFF\xFF\x81\x11\x15[a$\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x9D\x90a6\xEAV[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x90\x03a%hWa%8\x86\x86a(\x9EV[\x95P`\x01`\x01`@\x1B\x03\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a$\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x9D\x90a6\xEAV[P`\xF8\x81\x90\x1C`\xFD\x81\x10a$\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x9D\x90a6\xEAV[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a#\x1FV[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a%\xE8W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a%\xC6V[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a&+W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a&\tV[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a&nW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a&LV[PPP`(\x81\x01`@R\x92\x91PPV[``a\x0C,\x83\x83`\0a)oV[`\0\x80\x83Q\x83`\x01a&\x9E\x91\x90a3\xCCV[\x11\x15\x80\x15a&\xB5WPa&\xB2\x83`\x01a3\xCCV[\x83\x10[a'\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\x08\x9DV[\x83\x83\x01` \x01Q\x80a!\xD7\x85`\x01a3\xCCV[`\0\x80\x83Q\x83`\x02a'&\x91\x90a3\xCCV[\x11\x15\x80\x15a'=WPa':\x83`\x02a3\xCCV[\x83\x10[a'\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\x9DV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a!\xD7\x91\x90a3\xCCV[`\0\x80\x83Q\x83`\x04a'\xDF\x91\x90a3\xCCV[\x11\x15\x80\x15a'\xF6WPa'\xF3\x83`\x04a3\xCCV[\x83\x10[a(MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\x9DV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a(\x82W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa(bV[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a!\xD7\x85`\x04a3\xCCV[`\0\x80\x83Q\x83`\x08a(\xB0\x91\x90a3\xCCV[\x11\x15\x80\x15a(\xC7WPa(\xC4\x83`\x08a3\xCCV[\x83\x10[a)\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\x9DV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a)SW\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa)3V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a!\xD7\x85`\x08a3\xCCV[``\x81G\x10\x15a)\x94W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x08\x9DV[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa)\xB0\x91\x90a4&V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a)\xEDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a)\xF2V[``\x91P[P\x91P\x91Pa*\x02\x86\x83\x83a*\x0CV[\x96\x95PPPPPPV[``\x82a*!Wa*\x1C\x82a*hV[a\x0C,V[\x81Q\x15\x80\x15a*8WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a*aW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x08\x9DV[P\x80a\x0C,V[\x80Q\x15a*xW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x805a\xFF\xFF\x81\x16\x81\x14a\x17\nW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a*\xDBWa*\xDBa*\xA3V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a*\xDBWa*\xDBa*\xA3V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a+,Wa+,a*\xA3V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a+MWa+Ma*\xA3V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0a+na+i\x84a+4V[a+\x04V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a+\x82W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a+\xAAW`\0\x80\xFD[a\x0C,\x83\x835` \x85\x01a+[V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x17\nW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a+\xE6W`\0\x80\xFD[a+\xEF\x85a*\x91V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,\nW`\0\x80\xFD[a,\x16\x87\x82\x88\x01a+\x99V[\x93PPa,%`@\x86\x01a+\xB9V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,@W`\0\x80\xFD[a,L\x87\x82\x88\x01a+\x99V[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a,jW`\0\x80\xFD[a\x0C,\x82a*\x91V[`\0[\x83\x81\x10\x15a,\x8EW\x81\x81\x01Q\x83\x82\x01R` \x01a,vV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra,\xAF\x81` \x86\x01` \x86\x01a,sV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0C,` \x83\x01\x84a,\x97V[`\0\x80\x83`\x1F\x84\x01\x12a,\xE8W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a,\xFFW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a!\xDDW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a-,W`\0\x80\xFD[a-5\x84a*\x91V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-PW`\0\x80\xFD[a-\\\x86\x82\x87\x01a,\xD6V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x81\x01`\x02\x83\x10a-\x8BWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\x9CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a-\xB8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xCEW`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a-\xE0W`\0\x80\xFD[a-\xE8a*\xB9V[a-\xF1\x82a+\xB9V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\x0CW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x86\x13a.\x1DW`\0\x80\xFD[a.,\x86\x825` \x84\x01a+[V[` \x83\x01RP`@\x82\x015\x91Pa.B\x82a-\x91V[`@\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a.dW`\0\x80\xFD[a.m\x83a*\x91V[\x91P` \x83\x015a.}\x81a-\x91V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a.\x9DW`\0\x80\xFD[a.\xA6\x84a*\x91V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xC1W`\0\x80\xFD[a.\xCD\x86\x82\x87\x01a+\x99V[\x92PPa.\xDC`@\x85\x01a+\xB9V[\x90P\x92P\x92P\x92V[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x17\nW`\0\x80\xFD[`\0a\x01\0\x82\x84\x03\x12\x15a/\x0FW`\0\x80\xFD[a/\x17a*\xE1V[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R\x90Pa/=``\x83\x01a.\xE5V[``\x82\x01R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/[W`\0\x80\xFD[a/g\x84\x82\x85\x01a+\x99V[`\x80\x83\x01RP`\xA0\x82\x81\x015\x90\x82\x01R`\xC0\x80\x83\x015\x90\x82\x01R`\xE0\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a/\xA5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xBBW`\0\x80\xFD[a/\xC7\x84\x82\x85\x01a.\xFCV[\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a/\xE4W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xFAW`\0\x80\xFD[a0\x06\x86\x82\x87\x01a.\xFCV[\x93PP` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0)W`\0\x80\xFD[a05\x86\x82\x87\x01a+\x99V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a0WW`\0\x80\xFD[a0`\x86a*\x91V[\x94Pa0n` \x87\x01a*\x91V[\x93P`@\x86\x015\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x90W`\0\x80\xFD[a0\x9C\x88\x82\x89\x01a,\xD6V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a0\xBFW`\0\x80\xFD[\x815a\x0C,\x81a-\x91V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a0\xE0W`\0\x80\xFD[a0\xE9\x85a*\x91V[\x93Pa0\xF7` \x86\x01a*\x91V[\x92P`@\x85\x015a1\x07\x81a-\x91V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a1+W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a1KWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a1cW`\0\x80\xFD[PQ\x91\x90PV[`\0a1xa+i\x84a+4V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a1\x8CW`\0\x80\xFD[a\x0C,\x83` \x83\x01\x84a,sV[`\0` \x82\x84\x03\x12\x15a1\xACW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xC2W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a1\xD3W`\0\x80\xFD[a/\xC7\x84\x82Q` \x84\x01a1jV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\x1F\x82\x11\x15a\x1C\xCDW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a2\x19WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\t\xBDW`\0\x81U`\x01\x01a2%V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a2RWa2Ra*\xA3V[a2f\x81a2`\x84Ta1\x17V[\x84a1\xF2V[` `\x1F\x82\x11`\x01\x81\x14a2\x9AW`\0\x83\x15a2\x82WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\t\xBDV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a2\xCAW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a2\xAAV[P\x84\x82\x10\x15a2\xE8W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[a\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x19\xB0`@\x83\x01\x84\x86a2\xF7V[`\0` \x82\x84\x03\x12\x15a3PW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0C,W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a3rW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x0C,W`\0\x80\xFD[` \x81R`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q``\x82\x01R`\0``\x83\x01Q`\x80\x80\x84\x01Ra/\xC7`\xA0\x84\x01\x82a,\x97V[\x80\x82\x01\x80\x82\x11\x15a#\x1FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[a\xFF\xFF\x86\x16\x81Ra\xFF\xFF\x85\x16` \x82\x01R\x83`@\x82\x01R`\x80``\x82\x01R`\0a4\x1B`\x80\x83\x01\x84\x86a2\xF7V[\x97\x96PPPPPPPV[`\0\x82Qa48\x81\x84` \x87\x01a,sV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a4TW`\0\x80\xFD[\x81Qa\x0C,\x81a-\x91V[`\x01`\x01`@\x1B\x03\x83\x11\x15a4vWa4va*\xA3V[a4\x8A\x83a4\x84\x83Ta1\x17V[\x83a1\xF2V[`\0`\x1F\x84\x11`\x01\x81\x14a4\xBEW`\0\x85\x15a4\xA6WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\t\xBDV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a4\xEFW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a4\xCFV[P\x86\x82\x10\x15a5\x0CW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[a\xFF\xFF\x85\x16\x81R`\x80` \x82\x01R`\0a5;`\x80\x83\x01\x86a,\x97V[`\x01`\x01`@\x1B\x03\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra4\x1B\x81\x85a,\x97V[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a5\x84W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a5eV[PPP\x92\x91PPV[`\0\x86Qa5\x9F\x81\x84` \x8B\x01a,sV[\x86Q\x90\x83\x01\x90a5\xB3\x81\x83` \x8B\x01a,sV[\x86Q\x91\x01\x90a5\xC6\x81\x83` \x8A\x01a,sV[\x85Q\x91\x01\x90a5\xD9\x81\x83` \x89\x01a,sV[\x84Q\x91\x01\x90a5\xEC\x81\x83` \x88\x01a,sV[\x01\x97\x96PPPPPPPV[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra6<`\xE0\x84\x01\x82a,\x97V[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0\x83Qa6f\x81\x84` \x88\x01a,sV[\x83Q\x90\x83\x01\x90a6z\x81\x83` \x88\x01a,sV[\x01\x94\x93PPPPV[a\xFF\xFF\x87\x16\x81R`\xC0` \x82\x01R`\0a6\xA0`\xC0\x83\x01\x88a,\x97V[\x82\x81\x03`@\x84\x01Ra6\xB2\x81\x88a,\x97V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16``\x86\x01R\x86\x16`\x80\x85\x01R\x83\x81\x03`\xA0\x85\x01R\x90Pa6\xDD\x81\x85a,\x97V[\x99\x98PPPPPPPPPV[` \x80\x82R\x81\x81\x01R\x7FNextVarUint, value outside range`@\x82\x01R``\x01\x90V\xFE\xA2dipfsX\"\x12 \xB1\xC0\xD1b\xE9w\"SL\x19\xAF\xF4\"\xDB[\xF6!\x95h\x19\x10\x05\x94X?\xC6\xF8\xE6\xBA\x1D\xA4(dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MYSTIKOV2LAYERZEROERC20_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02\x86W`\x005`\xE0\x1C\x80c\x82[_\x8D\x11a\x01ZW\x80c\xD0\xB46\xBD\x11a\0\xC1W\x80c\xEF\xBF\xB2\xAE\x11a\0zW\x80c\xEF\xBF\xB2\xAE\x14a\x07\xAAW\x80c\xF2\xFD\xE3\x8B\x14a\x07\xBFW\x80c\xF4\xAD\x17\xC6\x14a\x07\xDFW\x80c\xF5\xEC\xBD\xBC\x14a\x07\xF4W\x80c\xFAu\x0FV\x14a\x08\x14W\x80c\xFB>=s\x14a\x085W`\0\x80\xFD[\x80c\xD0\xB46\xBD\x14a\x07\x17W\x80c\xD1\xDE\xBA\x1F\x14a\x07-W\x80c\xDD\xAC]\xC1\x14a\x07@W\x80c\xE0at\xE4\x14a\x07UW\x80c\xEB\x8Dr\xB7\x14a\x07uW\x80c\xEDn\xA3:\x14a\x07\x95W`\0\x80\xFD[\x80c\xC9#\x0C]\x11a\x01\x13W\x80c\xC9#\x0C]\x14a\x06eW\x80c\xCB\\\x02\x9A\x14a\x06zW\x80c\xCB\xE3B\x85\x14a\x06\x8DW\x80c\xCB\xED\x8B\x9C\x14a\x06\xA3W\x80c\xCD\xFC\xEE\xBA\x14a\x06\xC3W\x80c\xCF\xC7\xE2\xDA\x14a\x07\x02W`\0\x80\xFD[\x80c\x82[_\x8D\x14a\x05\xC3W\x80c\x8D\xA5\xCB[\x14a\x05\xD8W\x80c\x9A\x03cl\x14a\x05\xF6W\x80c\xB3S\xAA\xA7\x14a\x06\tW\x80c\xBCXw\x06\x14a\x06)W\x80c\xC2\xD4\x16\x01\x14a\x06>W`\0\x80\xFD[\x80c=\x8B8\xF6\x11a\x01\xFEW\x80cN\xE7\xDE\xD6\x11a\x01\xB7W\x80cN\xE7\xDE\xD6\x14a\x04\xE9W\x80c[\x8CA\xE6\x14a\x05\tW\x80cd\x0C\x0B6\x14a\x05XW\x80cf\xAD\\\x8A\x14a\x05nW\x80cqP\x18\xA6\x14a\x05\x8EW\x80cu3\xD7\x88\x14a\x05\xA3W`\0\x80\xFD[\x80c=\x8B8\xF6\x14a\x042W\x80c?\xE34z\x14a\x04bW\x80cB.\0(\x14a\x04~W\x80cB\xD6Z\x8D\x14a\x04\x9EW\x80cM\xDEo\xBC\x14a\x04\xBEW\x80cN<\x10\xB7\x14a\x04\xD4W`\0\x80\xFD[\x80c\x17m\xE7\xA8\x11a\x02PW\x80c\x17m\xE7\xA8\x14a\x03JW\x80c\x1B\xA4l\xFD\x14a\x03lW\x80c!\xE3-U\x14a\x03\x9EW\x80c$!\xE1U\x14a\x03\xBEW\x80c,\xD2mE\x14a\x03\xF0W\x80c0-_K\x14a\x04\x10W`\0\x80\xFD[\x80b\x1D5g\x14a\x02\x8BW\x80b\x97\xA0c\x14a\x02\xADW\x80c\x07\xE0\xDB\x17\x14a\x02\xE7W\x80c\x0B\xA9Y\t\x14a\x03\x07W\x80c\x10\xDD\xB17\x14a\x03*W[`\0\x80\xFD[4\x80\x15a\x02\x97W`\0\x80\xFD[Pa\x02\xABa\x02\xA66`\x04a+\xD0V[a\x08KV[\0[4\x80\x15a\x02\xB9W`\0\x80\xFD[P`\x0BTa\x02\xCF\x90`\x01`\xB0\x1B\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xF3W`\0\x80\xFD[Pa\x02\xABa\x03\x026`\x04a,XV[a\t\xC4V[4\x80\x15a\x03\x13W`\0\x80\xFD[Pa\x03\x1Ca\n*V[`@Q\x90\x81R` \x01a\x02\xDEV[4\x80\x15a\x036W`\0\x80\xFD[Pa\x02\xABa\x03E6`\x04a,XV[a\n\xB3V[4\x80\x15a\x03VW`\0\x80\xFD[Pa\x03_a\n\xEFV[`@Qa\x02\xDE\x91\x90a,\xC3V[4\x80\x15a\x03xW`\0\x80\xFD[P`\x0ET`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xDEV[4\x80\x15a\x03\xAAW`\0\x80\xFD[P`\x02Ta\x03\x86\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03\xCAW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\t\x81RhlayerZero`\xB8\x1B` \x82\x01Ra\x03_V[4\x80\x15a\x03\xFCW`\0\x80\xFD[P`\x08Ta\x03\x86\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\x1CW`\0\x80\xFD[P`\x0BTa\x02\xCF\x90`\x01`\xA0\x1B\x90\x04a\xFF\xFF\x16\x81V[4\x80\x15a\x04>W`\0\x80\xFD[Pa\x04Ra\x04M6`\x04a-\x17V[a\x0BfV[`@Q\x90\x15\x15\x81R` \x01a\x02\xDEV[4\x80\x15a\x04nW`\0\x80\xFD[P`\0`@Qa\x02\xDE\x91\x90a-iV[4\x80\x15a\x04\x8AW`\0\x80\xFD[Pa\x02\xABa\x04\x996`\x04a-\xA6V[a\x0C3V[4\x80\x15a\x04\xAAW`\0\x80\xFD[Pa\x02\xABa\x04\xB96`\x04a-\x17V[a\x0C\xD8V[4\x80\x15a\x04\xCAW`\0\x80\xFD[Pa\x03\x1C`\x05T\x81V[4\x80\x15a\x04\xE0W`\0\x80\xFD[Pa\x03_a\rKV[4\x80\x15a\x04\xF5W`\0\x80\xFD[Pa\x02\xABa\x05\x046`\x04a.QV[a\r\xD9V[4\x80\x15a\x05\x15W`\0\x80\xFD[Pa\x03\x1Ca\x05$6`\x04a.\x88V[`\r` \x90\x81R`\0\x93\x84R`@\x80\x85 \x84Q\x80\x86\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x95\x84\x01\x95\x90\x95 \x94R\x92\x90R\x82R\x90 T\x81V[4\x80\x15a\x05dW`\0\x80\xFD[Pa\x03\x1C`\x06T\x81V[4\x80\x15a\x05zW`\0\x80\xFD[Pa\x02\xABa\x05\x896`\x04a+\xD0V[a\x0E\x1EV[4\x80\x15a\x05\x9AW`\0\x80\xFD[Pa\x02\xABa\x0EPV[4\x80\x15a\x05\xAFW`\0\x80\xFD[Pa\x03_a\x05\xBE6`\x04a,XV[a\x0EdV[4\x80\x15a\x05\xCFW`\0\x80\xFD[Pa\x03\x1Ca\x0E}V[4\x80\x15a\x05\xE4W`\0\x80\xFD[P`\nT`\x01`\x01`\xA0\x1B\x03\x16a\x03\x86V[a\x02\xABa\x06\x046`\x04a/\x93V[a\x0F\x04V[4\x80\x15a\x06\x15W`\0\x80\xFD[P`\x0BTa\x03\x86\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x065W`\0\x80\xFD[Pa\x04Ra\x0F\x1DV[4\x80\x15a\x06JW`\0\x80\xFD[Pa\x06Sa\x0F\x8BV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\xDEV[4\x80\x15a\x06qW`\0\x80\xFD[Pa\x03_a\x0F\xF9V[a\x02\xABa\x06\x886`\x04a/\xCFV[a\x10CV[4\x80\x15a\x06\x99W`\0\x80\xFD[Pa\x03\x1C`\x07T\x81V[4\x80\x15a\x06\xAFW`\0\x80\xFD[Pa\x02\xABa\x06\xBE6`\x04a0?V[a\x14xV[4\x80\x15a\x06\xCFW`\0\x80\xFD[P`\0Ta\x06\xEA\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xDEV[4\x80\x15a\x07\x0EW`\0\x80\xFD[Pa\x03\x1Ca\x14\xF1V[4\x80\x15a\x07#W`\0\x80\xFD[Pa\x03\x1C`\x04T\x81V[a\x02\xABa\x07;6`\x04a+\xD0V[a\x15xV[4\x80\x15a\x07LW`\0\x80\xFD[Pa\x03\x86a\x16pV[4\x80\x15a\x07aW`\0\x80\xFD[P`\tTa\x03\x86\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\x81W`\0\x80\xFD[Pa\x02\xABa\x07\x906`\x04a-\x17V[a\x17\x0FV[4\x80\x15a\x07\xA1W`\0\x80\xFD[Pa\x04Ra\x17\x91V[4\x80\x15a\x07\xB6W`\0\x80\xFD[Pa\x03\x1Ca\x17\xDAV[4\x80\x15a\x07\xCBW`\0\x80\xFD[Pa\x02\xABa\x07\xDA6`\x04a0\xADV[a\x18aV[4\x80\x15a\x07\xEBW`\0\x80\xFD[Pa\x03\x1Ca\x18\x9FV[4\x80\x15a\x08\0W`\0\x80\xFD[Pa\x03_a\x08\x0F6`\x04a0\xCAV[a\x19&V[4\x80\x15a\x08 W`\0\x80\xFD[P`\0Ta\x04R\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x08AW`\0\x80\xFD[Pa\x03\x1C`\x03T\x81V[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\xA6W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn2\xB7287\xB4\xB7:\x101\xB0\xB662\xB9`\x89\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF\x84\x16`\0\x90\x81R`\x0C` R`@\x81 \x80Ta\x08\xC4\x90a1\x17V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xF0\x90a1\x17V[\x80\x15a\t=W\x80`\x1F\x10a\t\x12Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t=V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q\x84Q\x14\x15\x80a\tcWP\x80\x80Q\x90` \x01 \x84\x80Q\x90` \x01 \x14\x15[\x15a\t\xB1W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fsource sending contract\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x9DV[a\t\xBD\x85\x85\x85\x85a\x19\xB9V[PPPPPV[a\t\xCCa\x1A\x1AV[`\x0BT`@Qc\x07\xE0\xDB\x17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07\xE0\xDB\x17\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\x16W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xBDW=`\0\x80>=`\0\xFD[`\tT`@QcG:\x061`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG:\x061\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nwW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9B\x91\x90a1QV[\x90P\x80\x15a\n\xA9W\x80a\n\xADV[`\x04T[\x91PP\x90V[a\n\xBBa\x1A\x1AV[`\x0BT`@Qc\x10\xDD\xB17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x10\xDD\xB17\x90`$\x01a\t\xFCV[`\x0ET`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0B9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Ba\x91\x90\x81\x01\x90a1\x9AV[\x90P\x90V[a\xFF\xFF\x83\x16`\0\x90\x81R`\x0C` R`@\x81 \x80T\x82\x91\x90a\x0B\x87\x90a1\x17V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xB3\x90a1\x17V[\x80\x15a\x0C\0W\x80`\x1F\x10a\x0B\xD5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xE3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83\x83`@Qa\x0C\x17\x92\x91\x90a1\xE2V[`@Q\x80\x91\x03\x90 \x81\x80Q\x90` \x01 \x14\x91PP[\x93\x92PPPV[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0C^W`@Qcm\x8F\x11Q`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\xA8\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U` \x81\x01Q`\x01\x90a\x0C\x9D\x90\x82a29V[P`@\x01Q`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UV[a\x0C\xE0a\x1A\x1AV[`\x0BT`@QcB\xD6Z\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cB\xD6Z\x8D\x90a\r\x14\x90\x86\x90\x86\x90\x86\x90`\x04\x01a3 V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r.W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\rBW=`\0\x80>=`\0\xFD[PPPPPPPV[`\x01\x80Ta\rX\x90a1\x17V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\x84\x90a1\x17V[\x80\x15a\r\xD1W\x80`\x1F\x10a\r\xA6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xD1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xB4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[a\r\xE1a\x1A\x1AV[`\x0B\x80T`\x01`\x01`\xB0\x1B\x03\x19\x16`\x01`\xA0\x1Ba\xFF\xFF\x94\x90\x94\x16\x93\x90\x93\x02`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x92\x17`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x90UV[30\x14a\x0E>W`@Qcq\xF5\x0E\xC1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0EJ\x84\x84\x84\x84a\x1AGV[PPPPV[a\x0EXa\x1A\x1AV[a\x0Eb`\0a\x1A\x81V[V[`\x0C` R`\0\x90\x81R`@\x90 \x80Ta\rX\x90a1\x17V[`\tT`@Qca\x86<\x03`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC3\x0Cx\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xEE\x91\x90a1QV[\x90P\x80\x15a\x0E\xFCW\x80a\n\xADV[PP`\x07T\x90V[`@Qc\xE7\xA2O\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT`@\x80Qc^,;\x83`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xBCXw\x06\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0FgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ba\x91\x90a3>V[`\x0ET`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1<\xE5g\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0F\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ba\x91\x90a3`V[`\x0ET`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0B9W=`\0\x80>=`\0\xFD[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xAF\x91\x90a3>V[\x15a\x10\xCDW`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xBCXw\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11D\x91\x90a3>V[\x15a\x12\x1BW`\0`@Q\x80`\x80\x01`@R\x802`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x11w`\x0ET`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x86\x90R`@\x91\x82\x01\x85\x90R`\tT\x91Qc\x84\x9E\x8B\x9F`\xE0\x1B\x81R\x92\x93P\x16\x90c\x84\x9E\x8B\x9F\x90a\x11\xBB\x90\x84\x90`\x04\x01a3\x83V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xFC\x91\x90a3>V[a\x12\x19W`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[a\x12#a\x14\xF1V[\x83Q\x10\x15a\x12DW`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12La\n*V[\x83Q\x11\x15a\x12mW`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12ua\x17\xDAV[\x83`\xA0\x01Q\x10\x15a\x12\x99W`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xA1a\x18\x9FV[\x83`\xC0\x01Q\x10\x15a\x12\xC5W`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xCDa\x0E}V[\x83`\xE0\x01Q\x10\x15a\x12\xF1W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13\n\x84`@\x01Q\x85`\0\x01Q\x86``\x01Qa\x1A\xD3V[\x90P\x80\x84` \x01Q\x14a\x130W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT`@Qc\xDFY/}`\xE0\x1B\x81R2`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x9C\x91\x90a3>V[\x15a\x13\xBAW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x90\x82\x01R`\xC0\x86\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x85\x01Q``\x82\x01R`\x80\x80\x86\x01Q\x90\x82\x01R`\0a\x13\xFD\x82a\x1B\xCFV[\x90Pa\x14\r\x86`\xA0\x01Q\x82a\x1C>V[a\x14Ba\x14\x18a\x16pV[`\xE0\x88\x01Q`\xC0\x89\x01Q\x89Qa\x14.\x91\x90a3\xCCV[a\x148\x91\x90a3\xCCV[\x88`\xA0\x01Qa\x1CpV[` \x86\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPPPV[a\x14\x80a\x1A\x1AV[`\x0BT`@Qc2\xFBb\xE7`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCB\xED\x8B\x9C\x90a\x14\xB8\x90\x88\x90\x88\x90\x88\x90\x88\x90\x88\x90`\x04\x01a3\xEDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\xD2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xE6W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\tT`@QcU%\x98I`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cU%\x98I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15b\x91\x90a1QV[\x90P\x80\x15a\x15pW\x80a\n\xADV[PP`\x03T\x90V[a\xFF\xFF\x84\x16`\0\x90\x81R`\r` R`@\x80\x82 \x90Qa\x15\x99\x90\x86\x90a4&V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\x01`\x01`@\x1B\x03\x86\x16`\0\x90\x81R\x92R\x90 T\x90P\x80a\x15\xDEW`@Qc+\x96\xC9\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q` \x83\x01 \x81\x14a\x16\x1EW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x07`$\x82\x01Rf\x1C\x18^[\x1B\xD8Y`\xCA\x1B`D\x82\x01R`d\x01a\x08\x9DV[a\xFF\xFF\x85\x16`\0\x90\x81R`\r` R`@\x80\x82 \x90Qa\x16?\x90\x87\x90a4&V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\x01`\x01`@\x1B\x03\x87\x16`\0\x90\x81R\x92R\x90 Ua\t\xBD\x85\x85\x85\x85a\x1AGV[`\tT`@QcA\xFBiy`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cA\xFBiy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE1\x91\x90a4BV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x17\nW`@Qc\x06\xF3\xD63`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a\x17\x17a\x1A\x1AV[`\x0B\x80Ta\xFF\xFF`\xB0\x1B\x19\x16`\x01`\xB0\x1Ba\xFF\xFF\x86\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`\0\x90\x81R`\x0C` R`@\x90 a\x17P\x82\x84\x83a4_V[P\x7F\xFAAHz\xD5\xD6r\x8F\x0B\x19'o\xA1\xED\xDC\x16U\x85x\xF5\x10\x9F\xC3\x9D-\xC3<20G\r\xAB\x83\x83\x83`@Qa\x17\x84\x93\x92\x91\x90a3 V[`@Q\x80\x91\x03\x90\xA1PPPV[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FgW=`\0\x80>=`\0\xFD[`\tT`@Qc{R\xB0\r`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c{R\xB0\r\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18K\x91\x90a1QV[\x90P\x80\x15a\x18YW\x80a\n\xADV[PP`\x05T\x90V[a\x18ia\x1A\x1AV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18\x93W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x08\x9DV[a\x18\x9C\x81a\x1A\x81V[PV[`\tT`@QcO1O\x9D`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9Eb\x9F:\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x10\x91\x90a1QV[\x90P\x80\x15a\x19\x1EW\x80a\n\xADV[PP`\x06T\x90V[`\x0BT`@Qc={/o`\xE2\x1B\x81Ra\xFF\xFF\x80\x87\x16`\x04\x83\x01R\x85\x16`$\x82\x01R0`D\x82\x01R`d\x81\x01\x83\x90R``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF5\xEC\xBD\xBC\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19\xB0\x91\x90\x81\x01\x90a1\x9AV[\x95\x94PPPPPV[`@Qc3V\xAEE`\xE1\x1B\x81R0\x90cf\xAD\\\x8A\x90a\x19\xE2\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a5\x1EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xFCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\x10W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\nT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EbW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x08\x9DV[`\0a\x1AR\x82a\x1C\xD2V[`\0T`\x02T\x91\x92Pa\t\xBD\x91`\x01`\xA8\x1B\x90\x91\x04`\x01`\x01`@\x1B\x03\x16\x90`\x01`\x01`\xA0\x1B\x03\x162\x84a\x1D\x9AV[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x1B\x16W`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83`\x01`\x01`\x80\x1B\x03\x16\x10a\x1B?W`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90R`\x01`\x01`\x80\x1B\x03\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x1B\x8E\x91`\x04\x01a5\\V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xB0\x91\x90a1QV[``\x80a\x1B\xDF\x83`\0\x01Qa\x1ETV[a\x1B\xEC\x84` \x01Qa\x1ETV[a\x1B\xF9\x85`@\x01Qa\x1ETV[a\x1C\x06\x86``\x01Qa\x1ETV[a\x1C\x13\x87`\x80\x01Qa\x1E\xECV[`@Q` \x01a\x1C'\x95\x94\x93\x92\x91\x90a5\x8DV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[a\x1Cl`\x0B`\x16\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16\x823`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x87a\x1F#V[PPV[\x804\x14a\x1C\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x0CNM,\x8C\xEC\xA4\x0C\xCC\xAC\xA4\r\xAD.m\xAC.\x8Cm`k\x1B`D\x82\x01R`d\x01a\x08\x9DV[`\x0ETa\x1C\xCD\x90`\x01`\x01`\xA0\x1B\x03\x163\x85\x85a YV[PPPV[a\x1D\x04`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\x1D6`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\x1DB\x84\x82a \xB3V[\x90\x83R\x90Pa\x1DQ\x84\x82a \xB3V[` \x84\x01\x91\x90\x91R\x90Pa\x1De\x84\x82a \xB3V[`@\x84\x01\x91\x90\x91R\x90Pa\x1Dy\x84\x82a \xB3V[``\x84\x01\x91\x90\x91R\x90Pa\x1D\x8D\x84\x82a!\xE4V[P`\x80\x83\x01RP\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x1D\xC8W`@Qc\x1BI^\xCF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`@\x1B\x03\x85\x81\x16`\x01`\xA8\x1B\x90\x92\x04\x16\x14a\x1D\xFDW`@Qc7;\xC1!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x03a\x1E\x1FW`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E'a\x16pV[`\x01`\x01`\xA0\x1B\x03\x16cx\xD6\x0C\xD7\x82\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xE2\x92\x91\x90a5\xF8V[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x1E\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x9DV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\x1E\xDCW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1E\xBAV[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\x1E\xFA\x81a\"\xF1V[\x83`@Q` \x01a\x1F\x0C\x92\x91\x90a6TV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[a\xFF\xFF\x86\x16`\0\x90\x81R`\x0C` R`@\x81 \x80Ta\x1FA\x90a1\x17V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Fm\x90a1\x17V[\x80\x15a\x1F\xBAW\x80`\x1F\x10a\x1F\x8FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xBAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x9DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q`\0\x03a\x1F\xE3W`@Qc\x02\x0B5\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT`@Qb\xC5\x801`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5\x801\0\x90\x84\x90a \x1E\x90\x8B\x90\x86\x90\x8C\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a6\x83V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a 7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a KW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x0EJ\x90\x85\x90a#\xB8V[`\0\x80\x83Q\x83` a \xC5\x91\x90a3\xCCV[\x11\x15\x80\x15a \xDCWPa \xD9\x83` a3\xCCV[\x83\x10[a!4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\x08\x9DV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a!iW\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa!IV[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a!\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x9DV[\x80a!\xD7\x85` a3\xCCV[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a!\xF3\x85\x85a$\x1BV[\x86Q\x90\x95P\x90\x91Pa\"\x05\x82\x86a3\xCCV[\x11\x15\x80\x15a\"\x1BWPa\"\x18\x81\x85a3\xCCV[\x84\x10[a\"sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\x08\x9DV[``\x81\x15\x80\x15a\"\x8EW`@Q\x91P` \x82\x01`@Ra\"\xD8V[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a\"\xC7W\x80Q\x83R` \x92\x83\x01\x92\x01a\"\xAFV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a\"\xE4\x83\x87a3\xCCV[\x93P\x93PPP\x92P\x92\x90PV[```\xFD\x82`\x01`\x01`@\x1B\x03\x16\x10\x15a#%W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91R[\x92\x91PPV[a\xFF\xFF\x82`\x01`\x01`@\x1B\x03\x16\x11a#tWa#D`\xFD`\xF8\x1Ba%\x8EV[a#M\x83a%\xB5V[`@Q` \x01a#^\x92\x91\x90a6TV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82`\x01`\x01`@\x1B\x03\x16\x11a#\x9EWa#\x95`\x7F`\xF9\x1Ba%\x8EV[a#M\x83a%\xF8V[a#\xAF`\x01`\x01`\xF8\x1B\x03\x19a%\x8EV[a#M\x83a&;V[`\0a#\xCD`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a&~V[\x90P\x80Q`\0\x14\x15\x80\x15a#\xF2WP\x80\x80` \x01\x90Q\x81\x01\x90a#\xF0\x91\x90a3>V[\x15[\x15a\x1C\xCDW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x08\x9DV[`\0\x80`\0a$*\x85\x85a&\x8CV[\x94P\x90P`\0`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\xFD`\xF8\x1B\x03a$\xC2Wa$O\x86\x86a'\x14V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a$jWPa\xFF\xFF\x81\x11\x15[a$\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\x08\x9DV[\x92P\x83\x91Pa!\xDD\x90PV[`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\x7F`\xF9\x1B\x03a%\x1CWa$\xE1\x86\x86a'\xCDV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a%\0WPc\xFF\xFF\xFF\xFF\x81\x11\x15[a$\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x9D\x90a6\xEAV[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x90\x03a%hWa%8\x86\x86a(\x9EV[\x95P`\x01`\x01`@\x1B\x03\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a$\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x9D\x90a6\xEAV[P`\xF8\x81\x90\x1C`\xFD\x81\x10a$\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x9D\x90a6\xEAV[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a#\x1FV[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a%\xE8W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a%\xC6V[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a&+W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a&\tV[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a&nW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a&LV[PPP`(\x81\x01`@R\x92\x91PPV[``a\x0C,\x83\x83`\0a)oV[`\0\x80\x83Q\x83`\x01a&\x9E\x91\x90a3\xCCV[\x11\x15\x80\x15a&\xB5WPa&\xB2\x83`\x01a3\xCCV[\x83\x10[a'\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\x08\x9DV[\x83\x83\x01` \x01Q\x80a!\xD7\x85`\x01a3\xCCV[`\0\x80\x83Q\x83`\x02a'&\x91\x90a3\xCCV[\x11\x15\x80\x15a'=WPa':\x83`\x02a3\xCCV[\x83\x10[a'\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\x9DV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a!\xD7\x91\x90a3\xCCV[`\0\x80\x83Q\x83`\x04a'\xDF\x91\x90a3\xCCV[\x11\x15\x80\x15a'\xF6WPa'\xF3\x83`\x04a3\xCCV[\x83\x10[a(MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\x9DV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a(\x82W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa(bV[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a!\xD7\x85`\x04a3\xCCV[`\0\x80\x83Q\x83`\x08a(\xB0\x91\x90a3\xCCV[\x11\x15\x80\x15a(\xC7WPa(\xC4\x83`\x08a3\xCCV[\x83\x10[a)\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\x9DV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a)SW\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa)3V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a!\xD7\x85`\x08a3\xCCV[``\x81G\x10\x15a)\x94W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x08\x9DV[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa)\xB0\x91\x90a4&V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a)\xEDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a)\xF2V[``\x91P[P\x91P\x91Pa*\x02\x86\x83\x83a*\x0CV[\x96\x95PPPPPPV[``\x82a*!Wa*\x1C\x82a*hV[a\x0C,V[\x81Q\x15\x80\x15a*8WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a*aW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x08\x9DV[P\x80a\x0C,V[\x80Q\x15a*xW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x805a\xFF\xFF\x81\x16\x81\x14a\x17\nW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a*\xDBWa*\xDBa*\xA3V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a*\xDBWa*\xDBa*\xA3V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a+,Wa+,a*\xA3V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a+MWa+Ma*\xA3V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0a+na+i\x84a+4V[a+\x04V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a+\x82W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a+\xAAW`\0\x80\xFD[a\x0C,\x83\x835` \x85\x01a+[V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x17\nW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a+\xE6W`\0\x80\xFD[a+\xEF\x85a*\x91V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,\nW`\0\x80\xFD[a,\x16\x87\x82\x88\x01a+\x99V[\x93PPa,%`@\x86\x01a+\xB9V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,@W`\0\x80\xFD[a,L\x87\x82\x88\x01a+\x99V[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a,jW`\0\x80\xFD[a\x0C,\x82a*\x91V[`\0[\x83\x81\x10\x15a,\x8EW\x81\x81\x01Q\x83\x82\x01R` \x01a,vV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra,\xAF\x81` \x86\x01` \x86\x01a,sV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0C,` \x83\x01\x84a,\x97V[`\0\x80\x83`\x1F\x84\x01\x12a,\xE8W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a,\xFFW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a!\xDDW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a-,W`\0\x80\xFD[a-5\x84a*\x91V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-PW`\0\x80\xFD[a-\\\x86\x82\x87\x01a,\xD6V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x81\x01`\x02\x83\x10a-\x8BWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\x9CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a-\xB8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xCEW`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a-\xE0W`\0\x80\xFD[a-\xE8a*\xB9V[a-\xF1\x82a+\xB9V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\x0CW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x86\x13a.\x1DW`\0\x80\xFD[a.,\x86\x825` \x84\x01a+[V[` \x83\x01RP`@\x82\x015\x91Pa.B\x82a-\x91V[`@\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a.dW`\0\x80\xFD[a.m\x83a*\x91V[\x91P` \x83\x015a.}\x81a-\x91V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a.\x9DW`\0\x80\xFD[a.\xA6\x84a*\x91V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xC1W`\0\x80\xFD[a.\xCD\x86\x82\x87\x01a+\x99V[\x92PPa.\xDC`@\x85\x01a+\xB9V[\x90P\x92P\x92P\x92V[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x17\nW`\0\x80\xFD[`\0a\x01\0\x82\x84\x03\x12\x15a/\x0FW`\0\x80\xFD[a/\x17a*\xE1V[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R\x90Pa/=``\x83\x01a.\xE5V[``\x82\x01R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/[W`\0\x80\xFD[a/g\x84\x82\x85\x01a+\x99V[`\x80\x83\x01RP`\xA0\x82\x81\x015\x90\x82\x01R`\xC0\x80\x83\x015\x90\x82\x01R`\xE0\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a/\xA5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xBBW`\0\x80\xFD[a/\xC7\x84\x82\x85\x01a.\xFCV[\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a/\xE4W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xFAW`\0\x80\xFD[a0\x06\x86\x82\x87\x01a.\xFCV[\x93PP` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0)W`\0\x80\xFD[a05\x86\x82\x87\x01a+\x99V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a0WW`\0\x80\xFD[a0`\x86a*\x91V[\x94Pa0n` \x87\x01a*\x91V[\x93P`@\x86\x015\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x90W`\0\x80\xFD[a0\x9C\x88\x82\x89\x01a,\xD6V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a0\xBFW`\0\x80\xFD[\x815a\x0C,\x81a-\x91V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a0\xE0W`\0\x80\xFD[a0\xE9\x85a*\x91V[\x93Pa0\xF7` \x86\x01a*\x91V[\x92P`@\x85\x015a1\x07\x81a-\x91V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a1+W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a1KWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a1cW`\0\x80\xFD[PQ\x91\x90PV[`\0a1xa+i\x84a+4V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a1\x8CW`\0\x80\xFD[a\x0C,\x83` \x83\x01\x84a,sV[`\0` \x82\x84\x03\x12\x15a1\xACW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xC2W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a1\xD3W`\0\x80\xFD[a/\xC7\x84\x82Q` \x84\x01a1jV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\x1F\x82\x11\x15a\x1C\xCDW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a2\x19WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\t\xBDW`\0\x81U`\x01\x01a2%V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a2RWa2Ra*\xA3V[a2f\x81a2`\x84Ta1\x17V[\x84a1\xF2V[` `\x1F\x82\x11`\x01\x81\x14a2\x9AW`\0\x83\x15a2\x82WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\t\xBDV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a2\xCAW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a2\xAAV[P\x84\x82\x10\x15a2\xE8W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[a\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x19\xB0`@\x83\x01\x84\x86a2\xF7V[`\0` \x82\x84\x03\x12\x15a3PW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0C,W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a3rW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x0C,W`\0\x80\xFD[` \x81R`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q``\x82\x01R`\0``\x83\x01Q`\x80\x80\x84\x01Ra/\xC7`\xA0\x84\x01\x82a,\x97V[\x80\x82\x01\x80\x82\x11\x15a#\x1FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[a\xFF\xFF\x86\x16\x81Ra\xFF\xFF\x85\x16` \x82\x01R\x83`@\x82\x01R`\x80``\x82\x01R`\0a4\x1B`\x80\x83\x01\x84\x86a2\xF7V[\x97\x96PPPPPPPV[`\0\x82Qa48\x81\x84` \x87\x01a,sV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a4TW`\0\x80\xFD[\x81Qa\x0C,\x81a-\x91V[`\x01`\x01`@\x1B\x03\x83\x11\x15a4vWa4va*\xA3V[a4\x8A\x83a4\x84\x83Ta1\x17V[\x83a1\xF2V[`\0`\x1F\x84\x11`\x01\x81\x14a4\xBEW`\0\x85\x15a4\xA6WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\t\xBDV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a4\xEFW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a4\xCFV[P\x86\x82\x10\x15a5\x0CW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[a\xFF\xFF\x85\x16\x81R`\x80` \x82\x01R`\0a5;`\x80\x83\x01\x86a,\x97V[`\x01`\x01`@\x1B\x03\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra4\x1B\x81\x85a,\x97V[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a5\x84W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a5eV[PPP\x92\x91PPV[`\0\x86Qa5\x9F\x81\x84` \x8B\x01a,sV[\x86Q\x90\x83\x01\x90a5\xB3\x81\x83` \x8B\x01a,sV[\x86Q\x91\x01\x90a5\xC6\x81\x83` \x8A\x01a,sV[\x85Q\x91\x01\x90a5\xD9\x81\x83` \x89\x01a,sV[\x84Q\x91\x01\x90a5\xEC\x81\x83` \x88\x01a,sV[\x01\x97\x96PPPPPPPV[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra6<`\xE0\x84\x01\x82a,\x97V[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0\x83Qa6f\x81\x84` \x88\x01a,sV[\x83Q\x90\x83\x01\x90a6z\x81\x83` \x88\x01a,sV[\x01\x94\x93PPPPV[a\xFF\xFF\x87\x16\x81R`\xC0` \x82\x01R`\0a6\xA0`\xC0\x83\x01\x88a,\x97V[\x82\x81\x03`@\x84\x01Ra6\xB2\x81\x88a,\x97V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16``\x86\x01R\x86\x16`\x80\x85\x01R\x83\x81\x03`\xA0\x85\x01R\x90Pa6\xDD\x81\x85a,\x97V[\x99\x98PPPPPPPPPV[` \x80\x82R\x81\x81\x01R\x7FNextVarUint, value outside range`@\x82\x01R``\x01\x90V\xFE\xA2dipfsX\"\x12 \xB1\xC0\xD1b\xE9w\"SL\x19\xAF\xF4\"\xDB[\xF6!\x95h\x19\x10\x05\x94X?\xC6\xF8\xE6\xBA\x1D\xA4(dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MYSTIKOV2LAYERZEROERC20_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MystikoV2LayerZeroERC20<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoV2LayerZeroERC20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoV2LayerZeroERC20<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoV2LayerZeroERC20<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoV2LayerZeroERC20<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MystikoV2LayerZeroERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MystikoV2LayerZeroERC20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MYSTIKOV2LAYERZEROERC20_ABI.clone(),
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
                MYSTIKOV2LAYERZEROERC20_ABI.clone(),
                MYSTIKOV2LAYERZEROERC20_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `assetAddress` (0x1ba46cfd) function
        pub fn asset_address(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([27, 164, 108, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetDecimals` (0xc2d41601) function
        pub fn asset_decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([194, 212, 22, 1], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetName` (0xc9230c5d) function
        pub fn asset_name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([201, 35, 12, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetSymbol` (0x176de7a8) function
        pub fn asset_symbol(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([23, 109, 231, 168], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetType` (0x3fe3347a) function
        pub fn asset_type(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([63, 227, 52, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeProxyAddress` (0x2cd26d45) function
        pub fn bridge_proxy_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([44, 210, 109, 69], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeType` (0x2421e155) function
        pub fn bridge_type(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([36, 33, 225, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `certDeposit` (0xcb5c029a) function
        pub fn cert_deposit(
            &self,
            request: BridgeDepositRequest,
            certificate_deadline: ::ethers::core::types::U256,
            certificate_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [203, 92, 2, 154],
                    (request, certificate_deadline, certificate_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultMaxAmount` (0xd0b436bd) function
        pub fn default_max_amount(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([208, 180, 54, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultMinAmount` (0xfb3e3d73) function
        pub fn default_min_amount(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([251, 62, 61, 115], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultMinBridgeFee` (0x4dde6fbc) function
        pub fn default_min_bridge_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 222, 111, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultPeerMinExecutorFee` (0x640c0b36) function
        pub fn default_peer_min_executor_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([100, 12, 11, 54], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultPeerMinRollupFee` (0xcbe34285) function
        pub fn default_peer_min_rollup_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([203, 227, 66, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x9a03636c) function
        pub fn deposit(&self, request: BridgeDepositRequest) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 3, 99, 108], (request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedMessages` (0x5b8c41e6) function
        pub fn failed_messages(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Bytes,
            p2: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([91, 140, 65, 230], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forceResumeReceive` (0x42d65a8d) function
        pub fn force_resume_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 214, 90, 141], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAssociatedCommitmentPool` (0xddac5dc1) function
        pub fn get_associated_commitment_pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([221, 172, 93, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfig` (0xf5ecbdbc) function
        pub fn get_config(
            &self,
            version: u16,
            chain_id: u16,
            p2: ::ethers::core::types::Address,
            config_type: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([245, 236, 189, 188], (version, chain_id, p2, config_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxAmount` (0x0ba95909) function
        pub fn get_max_amount(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([11, 169, 89, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinAmount` (0xcfc7e2da) function
        pub fn get_min_amount(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([207, 199, 226, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinBridgeFee` (0xefbfb2ae) function
        pub fn get_min_bridge_fee(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([239, 191, 178, 174], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinExecutorFee` (0xf4ad17c6) function
        pub fn get_min_executor_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 173, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPeerMinRollupFee` (0x825b5f8d) function
        pub fn get_peer_min_rollup_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([130, 91, 95, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCertificateCheckEnabled` (0xbc587706) function
        pub fn is_certificate_check_enabled(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([188, 88, 119, 6], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDepositsDisabled` (0xed6ea33a) function
        pub fn is_deposits_disabled(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([237, 110, 163, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPeerContractSet` (0xfa750f56) function
        pub fn is_peer_contract_set(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 117, 15, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isTrustedRemote` (0x3d8b38f6) function
        pub fn is_trusted_remote(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 139, 56, 246], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `localLayerZeroChainId` (0x302d5f4b) function
        pub fn local_layer_zero_chain_id(&self) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([48, 45, 95, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lzEndpoint` (0xb353aaa7) function
        pub fn lz_endpoint(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([179, 83, 170, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lzReceive` (0x001d3567) function
        pub fn lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
            nonce: u64,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 29, 53, 103], (src_chain_id, src_address, nonce, payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonblockingLzReceive` (0x66ad5c8a) function
        pub fn nonblocking_lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
            nonce: u64,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 173, 92, 138], (src_chain_id, src_address, nonce, payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerChainId` (0xcdfceeba) function
        pub fn peer_chain_id(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([205, 252, 238, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerChainName` (0x4e3c10b7) function
        pub fn peer_chain_name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([78, 60, 16, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerContract` (0x21e32d55) function
        pub fn peer_contract(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([33, 227, 45, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerLayerZeroChainId` (0x0097a063) function
        pub fn peer_layer_zero_chain_id(&self) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([0, 151, 160, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `retryMessage` (0xd1deba1f) function
        pub fn retry_message(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
            nonce: u64,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 222, 186, 31], (src_chain_id, src_address, nonce, payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0xcbed8b9c) function
        pub fn set_config(
            &self,
            version: u16,
            chain_id: u16,
            config_type: ::ethers::core::types::U256,
            config: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 237, 139, 156], (version, chain_id, config_type, config))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEndpoint` (0x4ee7ded6) function
        pub fn set_endpoint(
            &self,
            lz_chain_id: u16,
            lz_endpoint: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 231, 222, 214], (lz_chain_id, lz_endpoint))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPeerContract` (0x422e0028) function
        pub fn set_peer_contract(
            &self,
            peer_contract: BridgePeerContract,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 46, 0, 40], (peer_contract,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReceiveVersion` (0x10ddb137) function
        pub fn set_receive_version(&self, version: u16) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 221, 177, 55], version)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSendVersion` (0x07e0db17) function
        pub fn set_send_version(&self, version: u16) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 224, 219, 23], version)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTrustedRemote` (0xeb8d72b7) function
        pub fn set_trusted_remote(
            &self,
            peer_layer_zero_chain_id: u16,
            peer_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 141, 114, 183], (peer_layer_zero_chain_id, peer_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settings` (0xe06174e4) function
        pub fn settings(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([224, 97, 116, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trustedRemoteLookup` (0x7533d788) function
        pub fn trusted_remote_lookup(
            &self,
            p0: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([117, 51, 215, 136], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CommitmentCrossChain` event
        pub fn commitment_cross_chain_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CommitmentCrossChainFilter> {
            self.0.event()
        }
        ///Gets the contract's `MessageFailed` event
        pub fn message_failed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MessageFailedFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetTrustedRemote` event
        pub fn set_trusted_remote_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetTrustedRemoteFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MystikoV2LayerZeroERC20Events> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MystikoV2LayerZeroERC20<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
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
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
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
    #[etherror(name = "AddressInsufficientBalance", abi = "AddressInsufficientBalance(address)")]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `AmountLessThanZero` with signature `AmountLessThanZero()` and selector `0x820bf1e5`
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
    #[etherror(name = "AmountLessThanZero", abi = "AmountLessThanZero()")]
    pub struct AmountLessThanZero;
    ///Custom Error type `AmountTooLarge` with signature `AmountTooLarge()` and selector `0x06250401`
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
    #[etherror(name = "AmountTooLarge", abi = "AmountTooLarge()")]
    pub struct AmountTooLarge;
    ///Custom Error type `AmountTooSmall` with signature `AmountTooSmall()` and selector `0xc2f5625a`
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
    #[etherror(name = "AmountTooSmall", abi = "AmountTooSmall()")]
    pub struct AmountTooSmall;
    ///Custom Error type `AssociatedPoolNotSet` with signature `AssociatedPoolNotSet()` and selector `0xde7ac660`
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
    #[etherror(name = "AssociatedPoolNotSet", abi = "AssociatedPoolNotSet()")]
    pub struct AssociatedPoolNotSet;
    ///Custom Error type `BridgeFeeTooFew` with signature `BridgeFeeTooFew()` and selector `0xc4d8d00d`
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
    #[etherror(name = "BridgeFeeTooFew", abi = "BridgeFeeTooFew()")]
    pub struct BridgeFeeTooFew;
    ///Custom Error type `CallIsNotLzApp` with signature `CallIsNotLzApp()` and selector `0xe3ea1d82`
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
    #[etherror(name = "CallIsNotLzApp", abi = "CallIsNotLzApp()")]
    pub struct CallIsNotLzApp;
    ///Custom Error type `CertificateInvalid` with signature `CertificateInvalid()` and selector `0xc108107c`
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
    #[etherror(name = "CertificateInvalid", abi = "CertificateInvalid()")]
    pub struct CertificateInvalid;
    ///Custom Error type `CommitmentHashIncorrect` with signature `CommitmentHashIncorrect()` and selector `0x37f544a0`
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
    #[etherror(name = "CommitmentHashIncorrect", abi = "CommitmentHashIncorrect()")]
    pub struct CommitmentHashIncorrect;
    ///Custom Error type `DepositsDisabled` with signature `DepositsDisabled()` and selector `0x717a1648`
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
    #[etherror(name = "DepositsDisabled", abi = "DepositsDisabled()")]
    pub struct DepositsDisabled;
    ///Custom Error type `DestinationChainIsNotTrusted` with signature `DestinationChainIsNotTrusted()` and selector `0x020b35a1`
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
    #[etherror(name = "DestinationChainIsNotTrusted", abi = "DestinationChainIsNotTrusted()")]
    pub struct DestinationChainIsNotTrusted;
    ///Custom Error type `ExecutorFeeTooFew` with signature `ExecutorFeeTooFew()` and selector `0xab4dad42`
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
    #[etherror(name = "ExecutorFeeTooFew", abi = "ExecutorFeeTooFew()")]
    pub struct ExecutorFeeTooFew;
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
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
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `HashKGreaterThanFieldSize` with signature `HashKGreaterThanFieldSize()` and selector `0x805f2a49`
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
    #[etherror(name = "HashKGreaterThanFieldSize", abi = "HashKGreaterThanFieldSize()")]
    pub struct HashKGreaterThanFieldSize;
    ///Custom Error type `Invalid` with signature `Invalid(string)` and selector `0x53a2556c`
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
    #[etherror(name = "Invalid", abi = "Invalid(string)")]
    pub struct Invalid {
        pub param: ::std::string::String,
    }
    ///Custom Error type `NoStoredMessage` with signature `NoStoredMessage()` and selector `0xae5b2614`
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
    #[etherror(name = "NoStoredMessage", abi = "NoStoredMessage()")]
    pub struct NoStoredMessage;
    ///Custom Error type `NotSupport` with signature `NotSupport()` and selector `0xe7a24ff9`
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
    #[etherror(name = "NotSupport", abi = "NotSupport()")]
    pub struct NotSupport;
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
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
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
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
    #[etherror(name = "OwnableUnauthorizedAccount", abi = "OwnableUnauthorizedAccount(address)")]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `PeerChainIdNotMatched` with signature `PeerChainIdNotMatched()` and selector `0x6e778242`
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
    #[etherror(name = "PeerChainIdNotMatched", abi = "PeerChainIdNotMatched()")]
    pub struct PeerChainIdNotMatched;
    ///Custom Error type `PeerContractAlreadySet` with signature `PeerContractAlreadySet()` and selector `0xdb1e22a2`
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
    #[etherror(name = "PeerContractAlreadySet", abi = "PeerContractAlreadySet()")]
    pub struct PeerContractAlreadySet;
    ///Custom Error type `PeerContractNotMatched` with signature `PeerContractNotMatched()` and selector `0xda4af678`
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
    #[etherror(name = "PeerContractNotMatched", abi = "PeerContractNotMatched()")]
    pub struct PeerContractNotMatched;
    ///Custom Error type `RandomSGreaterThanFieldSize` with signature `RandomSGreaterThanFieldSize()` and selector `0xeef782fc`
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
    #[etherror(name = "RandomSGreaterThanFieldSize", abi = "RandomSGreaterThanFieldSize()")]
    pub struct RandomSGreaterThanFieldSize;
    ///Custom Error type `RollupFeeToFew` with signature `RollupFeeToFew()` and selector `0xf09e057a`
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
    #[etherror(name = "RollupFeeToFew", abi = "RollupFeeToFew()")]
    pub struct RollupFeeToFew;
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
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
    #[etherror(name = "SafeERC20FailedOperation", abi = "SafeERC20FailedOperation(address)")]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `SanctionedAddress` with signature `SanctionedAddress()` and selector `0x2e70c0b1`
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
    #[etherror(name = "SanctionedAddress", abi = "SanctionedAddress()")]
    pub struct SanctionedAddress;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoV2LayerZeroERC20Errors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        AmountLessThanZero(AmountLessThanZero),
        AmountTooLarge(AmountTooLarge),
        AmountTooSmall(AmountTooSmall),
        AssociatedPoolNotSet(AssociatedPoolNotSet),
        BridgeFeeTooFew(BridgeFeeTooFew),
        CallIsNotLzApp(CallIsNotLzApp),
        CertificateInvalid(CertificateInvalid),
        CommitmentHashIncorrect(CommitmentHashIncorrect),
        DepositsDisabled(DepositsDisabled),
        DestinationChainIsNotTrusted(DestinationChainIsNotTrusted),
        ExecutorFeeTooFew(ExecutorFeeTooFew),
        FailedInnerCall(FailedInnerCall),
        HashKGreaterThanFieldSize(HashKGreaterThanFieldSize),
        Invalid(Invalid),
        NoStoredMessage(NoStoredMessage),
        NotSupport(NotSupport),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        PeerChainIdNotMatched(PeerChainIdNotMatched),
        PeerContractAlreadySet(PeerContractAlreadySet),
        PeerContractNotMatched(PeerContractNotMatched),
        RandomSGreaterThanFieldSize(RandomSGreaterThanFieldSize),
        RollupFeeToFew(RollupFeeToFew),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        SanctionedAddress(SanctionedAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MystikoV2LayerZeroERC20Errors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddressInsufficientBalance(decoded));
            }
            if let Ok(decoded) = <AmountLessThanZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountLessThanZero(decoded));
            }
            if let Ok(decoded) = <AmountTooLarge as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountTooLarge(decoded));
            }
            if let Ok(decoded) = <AmountTooSmall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountTooSmall(decoded));
            }
            if let Ok(decoded) = <AssociatedPoolNotSet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssociatedPoolNotSet(decoded));
            }
            if let Ok(decoded) = <BridgeFeeTooFew as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BridgeFeeTooFew(decoded));
            }
            if let Ok(decoded) = <CallIsNotLzApp as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallIsNotLzApp(decoded));
            }
            if let Ok(decoded) = <CertificateInvalid as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CertificateInvalid(decoded));
            }
            if let Ok(decoded) = <CommitmentHashIncorrect as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CommitmentHashIncorrect(decoded));
            }
            if let Ok(decoded) = <DepositsDisabled as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositsDisabled(decoded));
            }
            if let Ok(decoded) = <DestinationChainIsNotTrusted as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DestinationChainIsNotTrusted(decoded));
            }
            if let Ok(decoded) = <ExecutorFeeTooFew as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecutorFeeTooFew(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <HashKGreaterThanFieldSize as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashKGreaterThanFieldSize(decoded));
            }
            if let Ok(decoded) = <Invalid as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Invalid(decoded));
            }
            if let Ok(decoded) = <NoStoredMessage as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoStoredMessage(decoded));
            }
            if let Ok(decoded) = <NotSupport as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotSupport(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <PeerChainIdNotMatched as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainIdNotMatched(decoded));
            }
            if let Ok(decoded) = <PeerContractAlreadySet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerContractAlreadySet(decoded));
            }
            if let Ok(decoded) = <PeerContractNotMatched as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerContractNotMatched(decoded));
            }
            if let Ok(decoded) = <RandomSGreaterThanFieldSize as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RandomSGreaterThanFieldSize(decoded));
            }
            if let Ok(decoded) = <RollupFeeToFew as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RollupFeeToFew(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) = <SanctionedAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionedAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MystikoV2LayerZeroERC20Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddressInsufficientBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AmountLessThanZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AmountTooLarge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AmountTooSmall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssociatedPoolNotSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BridgeFeeTooFew(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CallIsNotLzApp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CertificateInvalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CommitmentHashIncorrect(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositsDisabled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DestinationChainIsNotTrusted(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecutorFeeTooFew(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedInnerCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HashKGreaterThanFieldSize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Invalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NoStoredMessage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotSupport(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnableInvalidOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnableUnauthorizedAccount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerChainIdNotMatched(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerContractAlreadySet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerContractNotMatched(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RandomSGreaterThanFieldSize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RollupFeeToFew(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeERC20FailedOperation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SanctionedAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MystikoV2LayerZeroERC20Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AmountLessThanZero as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AmountTooLarge as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AmountTooSmall as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AssociatedPoolNotSet as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <BridgeFeeTooFew as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <CallIsNotLzApp as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <CertificateInvalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <CommitmentHashIncorrect as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <DepositsDisabled as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <DestinationChainIsNotTrusted as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ExecutorFeeTooFew as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <FailedInnerCall as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <HashKGreaterThanFieldSize as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Invalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NoStoredMessage as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NotSupport as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <PeerChainIdNotMatched as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <PeerContractAlreadySet as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <PeerContractNotMatched as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <RandomSGreaterThanFieldSize as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <RollupFeeToFew as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <SanctionedAddress as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2LayerZeroERC20Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountLessThanZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssociatedPoolNotSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallIsNotLzApp(element) => ::core::fmt::Display::fmt(element, f),
                Self::CertificateInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentHashIncorrect(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::DestinationChainIsNotTrusted(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutorFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashKGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoStoredMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotSupport(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableUnauthorizedAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerChainIdNotMatched(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerContractAlreadySet(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerContractNotMatched(element) => ::core::fmt::Display::fmt(element, f),
                Self::RandomSGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupFeeToFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeERC20FailedOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionedAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MystikoV2LayerZeroERC20Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for MystikoV2LayerZeroERC20Errors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for MystikoV2LayerZeroERC20Errors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<AmountLessThanZero> for MystikoV2LayerZeroERC20Errors {
        fn from(value: AmountLessThanZero) -> Self {
            Self::AmountLessThanZero(value)
        }
    }
    impl ::core::convert::From<AmountTooLarge> for MystikoV2LayerZeroERC20Errors {
        fn from(value: AmountTooLarge) -> Self {
            Self::AmountTooLarge(value)
        }
    }
    impl ::core::convert::From<AmountTooSmall> for MystikoV2LayerZeroERC20Errors {
        fn from(value: AmountTooSmall) -> Self {
            Self::AmountTooSmall(value)
        }
    }
    impl ::core::convert::From<AssociatedPoolNotSet> for MystikoV2LayerZeroERC20Errors {
        fn from(value: AssociatedPoolNotSet) -> Self {
            Self::AssociatedPoolNotSet(value)
        }
    }
    impl ::core::convert::From<BridgeFeeTooFew> for MystikoV2LayerZeroERC20Errors {
        fn from(value: BridgeFeeTooFew) -> Self {
            Self::BridgeFeeTooFew(value)
        }
    }
    impl ::core::convert::From<CallIsNotLzApp> for MystikoV2LayerZeroERC20Errors {
        fn from(value: CallIsNotLzApp) -> Self {
            Self::CallIsNotLzApp(value)
        }
    }
    impl ::core::convert::From<CertificateInvalid> for MystikoV2LayerZeroERC20Errors {
        fn from(value: CertificateInvalid) -> Self {
            Self::CertificateInvalid(value)
        }
    }
    impl ::core::convert::From<CommitmentHashIncorrect> for MystikoV2LayerZeroERC20Errors {
        fn from(value: CommitmentHashIncorrect) -> Self {
            Self::CommitmentHashIncorrect(value)
        }
    }
    impl ::core::convert::From<DepositsDisabled> for MystikoV2LayerZeroERC20Errors {
        fn from(value: DepositsDisabled) -> Self {
            Self::DepositsDisabled(value)
        }
    }
    impl ::core::convert::From<DestinationChainIsNotTrusted> for MystikoV2LayerZeroERC20Errors {
        fn from(value: DestinationChainIsNotTrusted) -> Self {
            Self::DestinationChainIsNotTrusted(value)
        }
    }
    impl ::core::convert::From<ExecutorFeeTooFew> for MystikoV2LayerZeroERC20Errors {
        fn from(value: ExecutorFeeTooFew) -> Self {
            Self::ExecutorFeeTooFew(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for MystikoV2LayerZeroERC20Errors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<HashKGreaterThanFieldSize> for MystikoV2LayerZeroERC20Errors {
        fn from(value: HashKGreaterThanFieldSize) -> Self {
            Self::HashKGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<Invalid> for MystikoV2LayerZeroERC20Errors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<NoStoredMessage> for MystikoV2LayerZeroERC20Errors {
        fn from(value: NoStoredMessage) -> Self {
            Self::NoStoredMessage(value)
        }
    }
    impl ::core::convert::From<NotSupport> for MystikoV2LayerZeroERC20Errors {
        fn from(value: NotSupport) -> Self {
            Self::NotSupport(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for MystikoV2LayerZeroERC20Errors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for MystikoV2LayerZeroERC20Errors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<PeerChainIdNotMatched> for MystikoV2LayerZeroERC20Errors {
        fn from(value: PeerChainIdNotMatched) -> Self {
            Self::PeerChainIdNotMatched(value)
        }
    }
    impl ::core::convert::From<PeerContractAlreadySet> for MystikoV2LayerZeroERC20Errors {
        fn from(value: PeerContractAlreadySet) -> Self {
            Self::PeerContractAlreadySet(value)
        }
    }
    impl ::core::convert::From<PeerContractNotMatched> for MystikoV2LayerZeroERC20Errors {
        fn from(value: PeerContractNotMatched) -> Self {
            Self::PeerContractNotMatched(value)
        }
    }
    impl ::core::convert::From<RandomSGreaterThanFieldSize> for MystikoV2LayerZeroERC20Errors {
        fn from(value: RandomSGreaterThanFieldSize) -> Self {
            Self::RandomSGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<RollupFeeToFew> for MystikoV2LayerZeroERC20Errors {
        fn from(value: RollupFeeToFew) -> Self {
            Self::RollupFeeToFew(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for MystikoV2LayerZeroERC20Errors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<SanctionedAddress> for MystikoV2LayerZeroERC20Errors {
        fn from(value: SanctionedAddress) -> Self {
            Self::SanctionedAddress(value)
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
    #[ethevent(name = "CommitmentCrossChain", abi = "CommitmentCrossChain(uint256)")]
    pub struct CommitmentCrossChainFilter {
        #[ethevent(indexed)]
        pub commitment: ::ethers::core::types::U256,
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
    #[ethevent(name = "MessageFailed", abi = "MessageFailed(uint16,bytes,uint64,bytes)")]
    pub struct MessageFailedFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
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
    #[ethevent(name = "OwnershipTransferred", abi = "OwnershipTransferred(address,address)")]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "SetTrustedRemote", abi = "SetTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoV2LayerZeroERC20Events {
        CommitmentCrossChainFilter(CommitmentCrossChainFilter),
        MessageFailedFilter(MessageFailedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SetTrustedRemoteFilter(SetTrustedRemoteFilter),
    }
    impl ::ethers::contract::EthLogDecode for MystikoV2LayerZeroERC20Events {
        fn decode_log(log: &::ethers::core::abi::RawLog) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CommitmentCrossChainFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::CommitmentCrossChainFilter(decoded));
            }
            if let Ok(decoded) = MessageFailedFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::MessageFailedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedRemoteFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::SetTrustedRemoteFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MystikoV2LayerZeroERC20Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CommitmentCrossChainFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MessageFailedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTrustedRemoteFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CommitmentCrossChainFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: CommitmentCrossChainFilter) -> Self {
            Self::CommitmentCrossChainFilter(value)
        }
    }
    impl ::core::convert::From<MessageFailedFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: MessageFailedFilter) -> Self {
            Self::MessageFailedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: SetTrustedRemoteFilter) -> Self {
            Self::SetTrustedRemoteFilter(value)
        }
    }
    ///Container type for all input parameters for the `assetAddress` function with signature `assetAddress()` and selector `0x1ba46cfd`
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
    #[ethcall(name = "assetAddress", abi = "assetAddress()")]
    pub struct AssetAddressCall;
    ///Container type for all input parameters for the `assetDecimals` function with signature `assetDecimals()` and selector `0xc2d41601`
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
    #[ethcall(name = "assetDecimals", abi = "assetDecimals()")]
    pub struct AssetDecimalsCall;
    ///Container type for all input parameters for the `assetName` function with signature `assetName()` and selector `0xc9230c5d`
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
    #[ethcall(name = "assetName", abi = "assetName()")]
    pub struct AssetNameCall;
    ///Container type for all input parameters for the `assetSymbol` function with signature `assetSymbol()` and selector `0x176de7a8`
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
    #[ethcall(name = "assetSymbol", abi = "assetSymbol()")]
    pub struct AssetSymbolCall;
    ///Container type for all input parameters for the `assetType` function with signature `assetType()` and selector `0x3fe3347a`
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
    #[ethcall(name = "assetType", abi = "assetType()")]
    pub struct AssetTypeCall;
    ///Container type for all input parameters for the `bridgeProxyAddress` function with signature `bridgeProxyAddress()` and selector `0x2cd26d45`
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
    #[ethcall(name = "bridgeProxyAddress", abi = "bridgeProxyAddress()")]
    pub struct BridgeProxyAddressCall;
    ///Container type for all input parameters for the `bridgeType` function with signature `bridgeType()` and selector `0x2421e155`
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
    #[ethcall(name = "bridgeType", abi = "bridgeType()")]
    pub struct BridgeTypeCall;
    ///Container type for all input parameters for the `certDeposit` function with signature `certDeposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256),uint256,bytes)` and selector `0xcb5c029a`
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
        name = "certDeposit",
        abi = "certDeposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256),uint256,bytes)"
    )]
    pub struct CertDepositCall {
        pub request: BridgeDepositRequest,
        pub certificate_deadline: ::ethers::core::types::U256,
        pub certificate_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `defaultMaxAmount` function with signature `defaultMaxAmount()` and selector `0xd0b436bd`
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
    #[ethcall(name = "defaultMaxAmount", abi = "defaultMaxAmount()")]
    pub struct DefaultMaxAmountCall;
    ///Container type for all input parameters for the `defaultMinAmount` function with signature `defaultMinAmount()` and selector `0xfb3e3d73`
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
    #[ethcall(name = "defaultMinAmount", abi = "defaultMinAmount()")]
    pub struct DefaultMinAmountCall;
    ///Container type for all input parameters for the `defaultMinBridgeFee` function with signature `defaultMinBridgeFee()` and selector `0x4dde6fbc`
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
    #[ethcall(name = "defaultMinBridgeFee", abi = "defaultMinBridgeFee()")]
    pub struct DefaultMinBridgeFeeCall;
    ///Container type for all input parameters for the `defaultPeerMinExecutorFee` function with signature `defaultPeerMinExecutorFee()` and selector `0x640c0b36`
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
    #[ethcall(name = "defaultPeerMinExecutorFee", abi = "defaultPeerMinExecutorFee()")]
    pub struct DefaultPeerMinExecutorFeeCall;
    ///Container type for all input parameters for the `defaultPeerMinRollupFee` function with signature `defaultPeerMinRollupFee()` and selector `0xcbe34285`
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
    #[ethcall(name = "defaultPeerMinRollupFee", abi = "defaultPeerMinRollupFee()")]
    pub struct DefaultPeerMinRollupFeeCall;
    ///Container type for all input parameters for the `deposit` function with signature `deposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256))` and selector `0x9a03636c`
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
        name = "deposit",
        abi = "deposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256))"
    )]
    pub struct DepositCall {
        pub request: BridgeDepositRequest,
    }
    ///Container type for all input parameters for the `failedMessages` function with signature `failedMessages(uint16,bytes,uint64)` and selector `0x5b8c41e6`
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
    #[ethcall(name = "failedMessages", abi = "failedMessages(uint16,bytes,uint64)")]
    pub struct FailedMessagesCall(pub u16, pub ::ethers::core::types::Bytes, pub u64);
    ///Container type for all input parameters for the `forceResumeReceive` function with signature `forceResumeReceive(uint16,bytes)` and selector `0x42d65a8d`
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
    #[ethcall(name = "forceResumeReceive", abi = "forceResumeReceive(uint16,bytes)")]
    pub struct ForceResumeReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getAssociatedCommitmentPool` function with signature `getAssociatedCommitmentPool()` and selector `0xddac5dc1`
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
    #[ethcall(name = "getAssociatedCommitmentPool", abi = "getAssociatedCommitmentPool()")]
    pub struct GetAssociatedCommitmentPoolCall;
    ///Container type for all input parameters for the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `0xf5ecbdbc`
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
    #[ethcall(name = "getConfig", abi = "getConfig(uint16,uint16,address,uint256)")]
    pub struct GetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub p2: ::ethers::core::types::Address,
        pub config_type: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getMaxAmount` function with signature `getMaxAmount()` and selector `0x0ba95909`
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
    #[ethcall(name = "getMaxAmount", abi = "getMaxAmount()")]
    pub struct GetMaxAmountCall;
    ///Container type for all input parameters for the `getMinAmount` function with signature `getMinAmount()` and selector `0xcfc7e2da`
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
    #[ethcall(name = "getMinAmount", abi = "getMinAmount()")]
    pub struct GetMinAmountCall;
    ///Container type for all input parameters for the `getMinBridgeFee` function with signature `getMinBridgeFee()` and selector `0xefbfb2ae`
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
    #[ethcall(name = "getMinBridgeFee", abi = "getMinBridgeFee()")]
    pub struct GetMinBridgeFeeCall;
    ///Container type for all input parameters for the `getMinExecutorFee` function with signature `getMinExecutorFee()` and selector `0xf4ad17c6`
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
    #[ethcall(name = "getMinExecutorFee", abi = "getMinExecutorFee()")]
    pub struct GetMinExecutorFeeCall;
    ///Container type for all input parameters for the `getPeerMinRollupFee` function with signature `getPeerMinRollupFee()` and selector `0x825b5f8d`
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
    #[ethcall(name = "getPeerMinRollupFee", abi = "getPeerMinRollupFee()")]
    pub struct GetPeerMinRollupFeeCall;
    ///Container type for all input parameters for the `isCertificateCheckEnabled` function with signature `isCertificateCheckEnabled()` and selector `0xbc587706`
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
    #[ethcall(name = "isCertificateCheckEnabled", abi = "isCertificateCheckEnabled()")]
    pub struct IsCertificateCheckEnabledCall;
    ///Container type for all input parameters for the `isDepositsDisabled` function with signature `isDepositsDisabled()` and selector `0xed6ea33a`
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
    #[ethcall(name = "isDepositsDisabled", abi = "isDepositsDisabled()")]
    pub struct IsDepositsDisabledCall;
    ///Container type for all input parameters for the `isPeerContractSet` function with signature `isPeerContractSet()` and selector `0xfa750f56`
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
    #[ethcall(name = "isPeerContractSet", abi = "isPeerContractSet()")]
    pub struct IsPeerContractSetCall;
    ///Container type for all input parameters for the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `0x3d8b38f6`
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
    #[ethcall(name = "isTrustedRemote", abi = "isTrustedRemote(uint16,bytes)")]
    pub struct IsTrustedRemoteCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `localLayerZeroChainId` function with signature `localLayerZeroChainId()` and selector `0x302d5f4b`
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
    #[ethcall(name = "localLayerZeroChainId", abi = "localLayerZeroChainId()")]
    pub struct LocalLayerZeroChainIdCall;
    ///Container type for all input parameters for the `lzEndpoint` function with signature `lzEndpoint()` and selector `0xb353aaa7`
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
    #[ethcall(name = "lzEndpoint", abi = "lzEndpoint()")]
    pub struct LzEndpointCall;
    ///Container type for all input parameters for the `lzReceive` function with signature `lzReceive(uint16,bytes,uint64,bytes)` and selector `0x001d3567`
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
    #[ethcall(name = "lzReceive", abi = "lzReceive(uint16,bytes,uint64,bytes)")]
    pub struct LzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `nonblockingLzReceive` function with signature `nonblockingLzReceive(uint16,bytes,uint64,bytes)` and selector `0x66ad5c8a`
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
        name = "nonblockingLzReceive",
        abi = "nonblockingLzReceive(uint16,bytes,uint64,bytes)"
    )]
    pub struct NonblockingLzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `peerChainId` function with signature `peerChainId()` and selector `0xcdfceeba`
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
    #[ethcall(name = "peerChainId", abi = "peerChainId()")]
    pub struct PeerChainIdCall;
    ///Container type for all input parameters for the `peerChainName` function with signature `peerChainName()` and selector `0x4e3c10b7`
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
    #[ethcall(name = "peerChainName", abi = "peerChainName()")]
    pub struct PeerChainNameCall;
    ///Container type for all input parameters for the `peerContract` function with signature `peerContract()` and selector `0x21e32d55`
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
    #[ethcall(name = "peerContract", abi = "peerContract()")]
    pub struct PeerContractCall;
    ///Container type for all input parameters for the `peerLayerZeroChainId` function with signature `peerLayerZeroChainId()` and selector `0x0097a063`
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
    #[ethcall(name = "peerLayerZeroChainId", abi = "peerLayerZeroChainId()")]
    pub struct PeerLayerZeroChainIdCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `retryMessage` function with signature `retryMessage(uint16,bytes,uint64,bytes)` and selector `0xd1deba1f`
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
    #[ethcall(name = "retryMessage", abi = "retryMessage(uint16,bytes,uint64,bytes)")]
    pub struct RetryMessageCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig(uint16,uint16,uint256,bytes)` and selector `0xcbed8b9c`
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
    #[ethcall(name = "setConfig", abi = "setConfig(uint16,uint16,uint256,bytes)")]
    pub struct SetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub config_type: ::ethers::core::types::U256,
        pub config: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setEndpoint` function with signature `setEndpoint(uint16,address)` and selector `0x4ee7ded6`
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
    #[ethcall(name = "setEndpoint", abi = "setEndpoint(uint16,address)")]
    pub struct SetEndpointCall {
        pub lz_chain_id: u16,
        pub lz_endpoint: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPeerContract` function with signature `setPeerContract((uint64,string,address))` and selector `0x422e0028`
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
    #[ethcall(name = "setPeerContract", abi = "setPeerContract((uint64,string,address))")]
    pub struct SetPeerContractCall {
        pub peer_contract: BridgePeerContract,
    }
    ///Container type for all input parameters for the `setReceiveVersion` function with signature `setReceiveVersion(uint16)` and selector `0x10ddb137`
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
    #[ethcall(name = "setReceiveVersion", abi = "setReceiveVersion(uint16)")]
    pub struct SetReceiveVersionCall {
        pub version: u16,
    }
    ///Container type for all input parameters for the `setSendVersion` function with signature `setSendVersion(uint16)` and selector `0x07e0db17`
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
    #[ethcall(name = "setSendVersion", abi = "setSendVersion(uint16)")]
    pub struct SetSendVersionCall {
        pub version: u16,
    }
    ///Container type for all input parameters for the `setTrustedRemote` function with signature `setTrustedRemote(uint16,bytes)` and selector `0xeb8d72b7`
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
    #[ethcall(name = "setTrustedRemote", abi = "setTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteCall {
        pub peer_layer_zero_chain_id: u16,
        pub peer_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `settings` function with signature `settings()` and selector `0xe06174e4`
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
    #[ethcall(name = "settings", abi = "settings()")]
    pub struct SettingsCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `0x7533d788`
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
    #[ethcall(name = "trustedRemoteLookup", abi = "trustedRemoteLookup(uint16)")]
    pub struct TrustedRemoteLookupCall(pub u16);
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoV2LayerZeroERC20Calls {
        AssetAddress(AssetAddressCall),
        AssetDecimals(AssetDecimalsCall),
        AssetName(AssetNameCall),
        AssetSymbol(AssetSymbolCall),
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
        FailedMessages(FailedMessagesCall),
        ForceResumeReceive(ForceResumeReceiveCall),
        GetAssociatedCommitmentPool(GetAssociatedCommitmentPoolCall),
        GetConfig(GetConfigCall),
        GetMaxAmount(GetMaxAmountCall),
        GetMinAmount(GetMinAmountCall),
        GetMinBridgeFee(GetMinBridgeFeeCall),
        GetMinExecutorFee(GetMinExecutorFeeCall),
        GetPeerMinRollupFee(GetPeerMinRollupFeeCall),
        IsCertificateCheckEnabled(IsCertificateCheckEnabledCall),
        IsDepositsDisabled(IsDepositsDisabledCall),
        IsPeerContractSet(IsPeerContractSetCall),
        IsTrustedRemote(IsTrustedRemoteCall),
        LocalLayerZeroChainId(LocalLayerZeroChainIdCall),
        LzEndpoint(LzEndpointCall),
        LzReceive(LzReceiveCall),
        NonblockingLzReceive(NonblockingLzReceiveCall),
        Owner(OwnerCall),
        PeerChainId(PeerChainIdCall),
        PeerChainName(PeerChainNameCall),
        PeerContract(PeerContractCall),
        PeerLayerZeroChainId(PeerLayerZeroChainIdCall),
        RenounceOwnership(RenounceOwnershipCall),
        RetryMessage(RetryMessageCall),
        SetConfig(SetConfigCall),
        SetEndpoint(SetEndpointCall),
        SetPeerContract(SetPeerContractCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetSendVersion(SetSendVersionCall),
        SetTrustedRemote(SetTrustedRemoteCall),
        Settings(SettingsCall),
        TransferOwnership(TransferOwnershipCall),
        TrustedRemoteLookup(TrustedRemoteLookupCall),
    }
    impl ::ethers::core::abi::AbiDecode for MystikoV2LayerZeroERC20Calls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AssetAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetAddress(decoded));
            }
            if let Ok(decoded) = <AssetDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetDecimals(decoded));
            }
            if let Ok(decoded) = <AssetNameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetName(decoded));
            }
            if let Ok(decoded) = <AssetSymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetSymbol(decoded));
            }
            if let Ok(decoded) = <AssetTypeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetType(decoded));
            }
            if let Ok(decoded) = <BridgeProxyAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BridgeProxyAddress(decoded));
            }
            if let Ok(decoded) = <BridgeTypeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BridgeType(decoded));
            }
            if let Ok(decoded) = <CertDepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CertDeposit(decoded));
            }
            if let Ok(decoded) = <DefaultMaxAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultMaxAmount(decoded));
            }
            if let Ok(decoded) = <DefaultMinAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultMinAmount(decoded));
            }
            if let Ok(decoded) = <DefaultMinBridgeFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultMinBridgeFee(decoded));
            }
            if let Ok(decoded) = <DefaultPeerMinExecutorFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultPeerMinExecutorFee(decoded));
            }
            if let Ok(decoded) = <DefaultPeerMinRollupFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultPeerMinRollupFee(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <FailedMessagesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedMessages(decoded));
            }
            if let Ok(decoded) = <ForceResumeReceiveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ForceResumeReceive(decoded));
            }
            if let Ok(decoded) = <GetAssociatedCommitmentPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAssociatedCommitmentPool(decoded));
            }
            if let Ok(decoded) = <GetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetConfig(decoded));
            }
            if let Ok(decoded) = <GetMaxAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMaxAmount(decoded));
            }
            if let Ok(decoded) = <GetMinAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinAmount(decoded));
            }
            if let Ok(decoded) = <GetMinBridgeFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinBridgeFee(decoded));
            }
            if let Ok(decoded) = <GetMinExecutorFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinExecutorFee(decoded));
            }
            if let Ok(decoded) = <GetPeerMinRollupFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPeerMinRollupFee(decoded));
            }
            if let Ok(decoded) = <IsCertificateCheckEnabledCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsCertificateCheckEnabled(decoded));
            }
            if let Ok(decoded) = <IsDepositsDisabledCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsDepositsDisabled(decoded));
            }
            if let Ok(decoded) = <IsPeerContractSetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsPeerContractSet(decoded));
            }
            if let Ok(decoded) = <IsTrustedRemoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsTrustedRemote(decoded));
            }
            if let Ok(decoded) = <LocalLayerZeroChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LocalLayerZeroChainId(decoded));
            }
            if let Ok(decoded) = <LzEndpointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LzEndpoint(decoded));
            }
            if let Ok(decoded) = <LzReceiveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LzReceive(decoded));
            }
            if let Ok(decoded) = <NonblockingLzReceiveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NonblockingLzReceive(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PeerChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainId(decoded));
            }
            if let Ok(decoded) = <PeerChainNameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainName(decoded));
            }
            if let Ok(decoded) = <PeerContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerContract(decoded));
            }
            if let Ok(decoded) = <PeerLayerZeroChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerLayerZeroChainId(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RetryMessageCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RetryMessage(decoded));
            }
            if let Ok(decoded) = <SetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded) = <SetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEndpoint(decoded));
            }
            if let Ok(decoded) = <SetPeerContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPeerContract(decoded));
            }
            if let Ok(decoded) = <SetReceiveVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetReceiveVersion(decoded));
            }
            if let Ok(decoded) = <SetSendVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSendVersion(decoded));
            }
            if let Ok(decoded) = <SetTrustedRemoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTrustedRemote(decoded));
            }
            if let Ok(decoded) = <SettingsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Settings(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <TrustedRemoteLookupCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TrustedRemoteLookup(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MystikoV2LayerZeroERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetDecimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetName(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetSymbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BridgeProxyAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BridgeType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CertDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultMaxAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultMinAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultMinBridgeFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultPeerMinExecutorFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultPeerMinRollupFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedMessages(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ForceResumeReceive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAssociatedCommitmentPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMaxAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMinAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMinBridgeFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMinExecutorFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPeerMinRollupFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsCertificateCheckEnabled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDepositsDisabled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsPeerContractSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsTrustedRemote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LocalLayerZeroChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LzEndpoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LzReceive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NonblockingLzReceive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerChainName(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerLayerZeroChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RetryMessage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetEndpoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPeerContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetReceiveVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSendVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetTrustedRemote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Settings(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TrustedRemoteLookup(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2LayerZeroERC20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetName(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetSymbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetType(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeProxyAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeType(element) => ::core::fmt::Display::fmt(element, f),
                Self::CertDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultMinAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultMinBridgeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultPeerMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultPeerMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedMessages(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceResumeReceive(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssociatedCommitmentPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinBridgeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPeerMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsCertificateCheckEnabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPeerContractSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsTrustedRemote(element) => ::core::fmt::Display::fmt(element, f),
                Self::LocalLayerZeroChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::LzEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::LzReceive(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonblockingLzReceive(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerChainName(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerLayerZeroChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RetryMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPeerContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReceiveVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSendVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTrustedRemote(element) => ::core::fmt::Display::fmt(element, f),
                Self::Settings(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustedRemoteLookup(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssetAddressCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: AssetAddressCall) -> Self {
            Self::AssetAddress(value)
        }
    }
    impl ::core::convert::From<AssetDecimalsCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: AssetDecimalsCall) -> Self {
            Self::AssetDecimals(value)
        }
    }
    impl ::core::convert::From<AssetNameCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: AssetNameCall) -> Self {
            Self::AssetName(value)
        }
    }
    impl ::core::convert::From<AssetSymbolCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: AssetSymbolCall) -> Self {
            Self::AssetSymbol(value)
        }
    }
    impl ::core::convert::From<AssetTypeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: AssetTypeCall) -> Self {
            Self::AssetType(value)
        }
    }
    impl ::core::convert::From<BridgeProxyAddressCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: BridgeProxyAddressCall) -> Self {
            Self::BridgeProxyAddress(value)
        }
    }
    impl ::core::convert::From<BridgeTypeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: BridgeTypeCall) -> Self {
            Self::BridgeType(value)
        }
    }
    impl ::core::convert::From<CertDepositCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: CertDepositCall) -> Self {
            Self::CertDeposit(value)
        }
    }
    impl ::core::convert::From<DefaultMaxAmountCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: DefaultMaxAmountCall) -> Self {
            Self::DefaultMaxAmount(value)
        }
    }
    impl ::core::convert::From<DefaultMinAmountCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: DefaultMinAmountCall) -> Self {
            Self::DefaultMinAmount(value)
        }
    }
    impl ::core::convert::From<DefaultMinBridgeFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: DefaultMinBridgeFeeCall) -> Self {
            Self::DefaultMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<DefaultPeerMinExecutorFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: DefaultPeerMinExecutorFeeCall) -> Self {
            Self::DefaultPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<DefaultPeerMinRollupFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: DefaultPeerMinRollupFeeCall) -> Self {
            Self::DefaultPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<FailedMessagesCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: FailedMessagesCall) -> Self {
            Self::FailedMessages(value)
        }
    }
    impl ::core::convert::From<ForceResumeReceiveCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: ForceResumeReceiveCall) -> Self {
            Self::ForceResumeReceive(value)
        }
    }
    impl ::core::convert::From<GetAssociatedCommitmentPoolCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: GetAssociatedCommitmentPoolCall) -> Self {
            Self::GetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<GetConfigCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: GetConfigCall) -> Self {
            Self::GetConfig(value)
        }
    }
    impl ::core::convert::From<GetMaxAmountCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: GetMaxAmountCall) -> Self {
            Self::GetMaxAmount(value)
        }
    }
    impl ::core::convert::From<GetMinAmountCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: GetMinAmountCall) -> Self {
            Self::GetMinAmount(value)
        }
    }
    impl ::core::convert::From<GetMinBridgeFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: GetMinBridgeFeeCall) -> Self {
            Self::GetMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<GetMinExecutorFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: GetMinExecutorFeeCall) -> Self {
            Self::GetMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<GetPeerMinRollupFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: GetPeerMinRollupFeeCall) -> Self {
            Self::GetPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<IsCertificateCheckEnabledCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: IsCertificateCheckEnabledCall) -> Self {
            Self::IsCertificateCheckEnabled(value)
        }
    }
    impl ::core::convert::From<IsDepositsDisabledCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: IsDepositsDisabledCall) -> Self {
            Self::IsDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<IsPeerContractSetCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: IsPeerContractSetCall) -> Self {
            Self::IsPeerContractSet(value)
        }
    }
    impl ::core::convert::From<IsTrustedRemoteCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: IsTrustedRemoteCall) -> Self {
            Self::IsTrustedRemote(value)
        }
    }
    impl ::core::convert::From<LocalLayerZeroChainIdCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: LocalLayerZeroChainIdCall) -> Self {
            Self::LocalLayerZeroChainId(value)
        }
    }
    impl ::core::convert::From<LzEndpointCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: LzEndpointCall) -> Self {
            Self::LzEndpoint(value)
        }
    }
    impl ::core::convert::From<LzReceiveCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: LzReceiveCall) -> Self {
            Self::LzReceive(value)
        }
    }
    impl ::core::convert::From<NonblockingLzReceiveCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: NonblockingLzReceiveCall) -> Self {
            Self::NonblockingLzReceive(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PeerChainIdCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: PeerChainIdCall) -> Self {
            Self::PeerChainId(value)
        }
    }
    impl ::core::convert::From<PeerChainNameCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: PeerChainNameCall) -> Self {
            Self::PeerChainName(value)
        }
    }
    impl ::core::convert::From<PeerContractCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: PeerContractCall) -> Self {
            Self::PeerContract(value)
        }
    }
    impl ::core::convert::From<PeerLayerZeroChainIdCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: PeerLayerZeroChainIdCall) -> Self {
            Self::PeerLayerZeroChainId(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RetryMessageCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: RetryMessageCall) -> Self {
            Self::RetryMessage(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetEndpointCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetEndpointCall) -> Self {
            Self::SetEndpoint(value)
        }
    }
    impl ::core::convert::From<SetPeerContractCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetPeerContractCall) -> Self {
            Self::SetPeerContract(value)
        }
    }
    impl ::core::convert::From<SetReceiveVersionCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetReceiveVersionCall) -> Self {
            Self::SetReceiveVersion(value)
        }
    }
    impl ::core::convert::From<SetSendVersionCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetSendVersionCall) -> Self {
            Self::SetSendVersion(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetTrustedRemoteCall) -> Self {
            Self::SetTrustedRemote(value)
        }
    }
    impl ::core::convert::From<SettingsCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SettingsCall) -> Self {
            Self::Settings(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TrustedRemoteLookupCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: TrustedRemoteLookupCall) -> Self {
            Self::TrustedRemoteLookup(value)
        }
    }
    ///Container type for all return fields from the `assetAddress` function with signature `assetAddress()` and selector `0x1ba46cfd`
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
    pub struct AssetAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `assetDecimals` function with signature `assetDecimals()` and selector `0xc2d41601`
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
    pub struct AssetDecimalsReturn(pub u8);
    ///Container type for all return fields from the `assetName` function with signature `assetName()` and selector `0xc9230c5d`
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
    pub struct AssetNameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `assetSymbol` function with signature `assetSymbol()` and selector `0x176de7a8`
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
    pub struct AssetSymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `assetType` function with signature `assetType()` and selector `0x3fe3347a`
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
    pub struct AssetTypeReturn(pub u8);
    ///Container type for all return fields from the `bridgeProxyAddress` function with signature `bridgeProxyAddress()` and selector `0x2cd26d45`
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
    pub struct BridgeProxyAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `bridgeType` function with signature `bridgeType()` and selector `0x2421e155`
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
    pub struct BridgeTypeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `defaultMaxAmount` function with signature `defaultMaxAmount()` and selector `0xd0b436bd`
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
    pub struct DefaultMaxAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `defaultMinAmount` function with signature `defaultMinAmount()` and selector `0xfb3e3d73`
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
    pub struct DefaultMinAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `defaultMinBridgeFee` function with signature `defaultMinBridgeFee()` and selector `0x4dde6fbc`
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
    pub struct DefaultMinBridgeFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `defaultPeerMinExecutorFee` function with signature `defaultPeerMinExecutorFee()` and selector `0x640c0b36`
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
    pub struct DefaultPeerMinExecutorFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `defaultPeerMinRollupFee` function with signature `defaultPeerMinRollupFee()` and selector `0xcbe34285`
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
    pub struct DefaultPeerMinRollupFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `failedMessages` function with signature `failedMessages(uint16,bytes,uint64)` and selector `0x5b8c41e6`
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
    pub struct FailedMessagesReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getAssociatedCommitmentPool` function with signature `getAssociatedCommitmentPool()` and selector `0xddac5dc1`
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
    pub struct GetAssociatedCommitmentPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `0xf5ecbdbc`
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
    pub struct GetConfigReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getMaxAmount` function with signature `getMaxAmount()` and selector `0x0ba95909`
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
    pub struct GetMaxAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMinAmount` function with signature `getMinAmount()` and selector `0xcfc7e2da`
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
    pub struct GetMinAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMinBridgeFee` function with signature `getMinBridgeFee()` and selector `0xefbfb2ae`
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
    pub struct GetMinBridgeFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMinExecutorFee` function with signature `getMinExecutorFee()` and selector `0xf4ad17c6`
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
    pub struct GetMinExecutorFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPeerMinRollupFee` function with signature `getPeerMinRollupFee()` and selector `0x825b5f8d`
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
    pub struct GetPeerMinRollupFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isCertificateCheckEnabled` function with signature `isCertificateCheckEnabled()` and selector `0xbc587706`
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
    pub struct IsCertificateCheckEnabledReturn(pub bool);
    ///Container type for all return fields from the `isDepositsDisabled` function with signature `isDepositsDisabled()` and selector `0xed6ea33a`
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
    pub struct IsDepositsDisabledReturn(pub bool);
    ///Container type for all return fields from the `isPeerContractSet` function with signature `isPeerContractSet()` and selector `0xfa750f56`
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
    pub struct IsPeerContractSetReturn(pub bool);
    ///Container type for all return fields from the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `0x3d8b38f6`
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
    pub struct IsTrustedRemoteReturn(pub bool);
    ///Container type for all return fields from the `localLayerZeroChainId` function with signature `localLayerZeroChainId()` and selector `0x302d5f4b`
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
    pub struct LocalLayerZeroChainIdReturn(pub u16);
    ///Container type for all return fields from the `lzEndpoint` function with signature `lzEndpoint()` and selector `0xb353aaa7`
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
    pub struct LzEndpointReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `peerChainId` function with signature `peerChainId()` and selector `0xcdfceeba`
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
    pub struct PeerChainIdReturn(pub u64);
    ///Container type for all return fields from the `peerChainName` function with signature `peerChainName()` and selector `0x4e3c10b7`
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
    pub struct PeerChainNameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `peerContract` function with signature `peerContract()` and selector `0x21e32d55`
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
    pub struct PeerContractReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `peerLayerZeroChainId` function with signature `peerLayerZeroChainId()` and selector `0x0097a063`
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
    pub struct PeerLayerZeroChainIdReturn(pub u16);
    ///Container type for all return fields from the `settings` function with signature `settings()` and selector `0xe06174e4`
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
    pub struct SettingsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `0x7533d788`
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
    pub struct TrustedRemoteLookupReturn(pub ::ethers::core::types::Bytes);
}
