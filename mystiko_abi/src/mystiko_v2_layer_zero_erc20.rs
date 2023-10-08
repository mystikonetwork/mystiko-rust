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
    const _: () = {
        ::core::include_bytes!("../json/MystikoV2LayerZeroERC20.json",);
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers_core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_hasher3"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                            "contract IHasher3"
                        ),),
                    },
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                            "contract IERC20Metadata"
                        ),),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("assetDecimals"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assetDecimals"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint8"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assetName"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assetName"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assetSymbol"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assetSymbol"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assetType"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assetType"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "enum AssetPool.AssetType"
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bridgeProxyAddress"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("bridgeProxyAddress"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bridgeType"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("bridgeType"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeOperator"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("changeOperator"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_newOperator"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deposit"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_request"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers_core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers_core::abi::ethabi::ParamType::Bytes,
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct IMystikoBridge.DepositRequest",
                            ),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("disableSanctionsCheck"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("disableSanctionsCheck",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enableSanctionsCheck"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enableSanctionsCheck",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("failedMessages"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("failedMessages"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                        ],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("forceResumeReceive"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("forceResumeReceive"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAssociatedCommitmentPool"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAssociatedCommitmentPool",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getConfig"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getConfig"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_version"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_configType"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxAmount"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMaxAmount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMinAmount"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMinAmount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMinBridgeFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMinBridgeFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMinExecutorFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMinExecutorFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPeerMinExecutorFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPeerMinExecutorFee",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPeerMinRollupFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPeerMinRollupFee",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isDepositsDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isDepositsDisabled"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isTrustedRemote"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isTrustedRemote"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("localLayerZeroChainId"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("localLayerZeroChainId",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lzEndpoint"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lzEndpoint"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract ILayerZeroEndpoint",
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lzReceive"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lzReceive"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonblockingLzReceive"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nonblockingLzReceive",),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peerChainId"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("peerChainId"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peerChainName"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("peerChainName"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peerContract"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("peerContract"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peerLayerZeroChainId"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("peerLayerZeroChainId",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("retryMessage"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("retryMessage"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sanctionsCheck"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sanctionsCheck"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sanctionsList"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sanctionsList"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract ISanctionsList"
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAssociatedCommitmentPool"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAssociatedCommitmentPool",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_commitmentPoolAddress",),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setBridgeProxyAddress"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setBridgeProxyAddress",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_bridgeProxyAddress",),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setConfig"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setConfig"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_version"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_configType"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_config"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDepositsDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setDepositsDisabled",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_state"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setEndpoint"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setEndpoint"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_lzChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_lzEndpoint"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMinBridgeFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMinBridgeFee"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_minBridgeFee"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMinExecutorFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMinExecutorFee"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_minExecutorFee"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPeerContract"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPeerContract"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_peerChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_peerChainName"),
                                kind: ::ethers_core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_peerContract"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPeerMinExecutorFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPeerMinExecutorFee",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_peerMinExecutorFee",),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPeerMinRollupFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPeerMinRollupFee",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_peerMinRollupFee"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setReceiveVersion"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setReceiveVersion"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_version"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSendVersion"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSendVersion"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_version"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTrustedRemote"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setTrustedRemote"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_peerLayerZeroChainId",),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_peerAddress"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("trustedRemoteLookup"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("trustedRemoteLookup",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateDepositAmountLimits"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateDepositAmountLimits",),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_maxAmount"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_minAmount"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateSanctionsListAddress"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateSanctionsListAddress",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_sanction"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract ISanctionsList"
                            ),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentCrossChain"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CommitmentCrossChain",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("commitment"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositAmountLimits"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DepositAmountLimits",),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("maxAmount"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("minAmount"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositsDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DepositsDisabled"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MessageFailed"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MessageFailed"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinBridgeFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MinBridgeFee"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("minBridgeFee"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinExecutorFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MinExecutorFee"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("minExecutorFee"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorChanged"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorChanged"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PeerMinExecutorFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PeerMinExecutorFee"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("peerMinExecutorFee",),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PeerMinRollupFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PeerMinRollupFee"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("peerMinRollupFee"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SanctionsCheck"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SanctionsCheck"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SanctionsList"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SanctionsList"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("sanctions"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetTrustedRemote"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetTrustedRemote"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AmountLessThanZero"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AmountLessThanZero"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AmountTooLarge"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AmountTooLarge"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AmountTooSmall"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AmountTooSmall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BridgeFeeTooFew"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BridgeFeeTooFew"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CallIsNotLzApp"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CallIsNotLzApp"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentHashIncorrect"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CommitmentHashIncorrect",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositsDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DepositsDisabled"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DestinationChainIsNotTrusted"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DestinationChainIsNotTrusted",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutorFeeTooFew"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ExecutorFeeTooFew"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FromChainIdNotMatched"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FromChainIdNotMatched",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FromProxyAddressNotMatched"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FromProxyAddressNotMatched",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HashKGreaterThanFieldSize"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("HashKGreaterThanFieldSize",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Invalid"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("param"),
                            kind: ::ethers_core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinAmountGreaterThanMaxAmount"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MinAmountGreaterThanMaxAmount",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoStoredMessage"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoStoredMessage"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotChanged"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotChanged"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyOperator"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyOperator"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RandomSGreaterThanFieldSize"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RandomSGreaterThanFieldSize",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RollupFeeToFew"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RollupFeeToFew"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SanctionedAddress"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
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
    pub static MYSTIKOV2LAYERZEROERC20_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16t\x01@\xC5y#\x92K\\\\TU\xC4\x8D\x931q9\xAD\xDA\xC8\xFB\x17\x90U4\x80\x15b\0\08W`\0\x80\xFD[P`@Qb\0=\xA88\x03\x80b\0=\xA8\x839\x81\x01`@\x81\x90Rb\0\0[\x91b\0\x01\x14V[`\x0C\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x82b\0\0\x98b\0\0\x923\x90V[b\0\0\xC2V[P`\x11\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPb\0\x01l\x90PV[`\r\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01(W`\0\x80\xFD[\x82Qb\0\x015\x81b\0\x01SV[` \x84\x01Q\x90\x92Pb\0\x01H\x81b\0\x01SV[\x80\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01iW`\0\x80\xFD[PV[a<,\x80b\0\x01|`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x03=W`\x005`\xE0\x1C\x80cu3\xD7\x88\x11a\x01\xB0W\x80c\xD1\xDE\xBA\x1F\x11a\0\xECW\x80c\xEB\x8Dr\xB7\x11a\0\x95W\x80c\xEF\xBF\xB2\xAE\x11a\0oW\x80c\xEF\xBF\xB2\xAE\x14a\t\x8CW\x80c\xF2\xFD\xE3\x8B\x14a\t\xA1W\x80c\xF4\xAD\x17\xC6\x14a\t\xC1W\x80c\xF5\xEC\xBD\xBC\x14a\t\xD6W`\0\x80\xFD[\x80c\xEB\x8Dr\xB7\x14a\t-W\x80c\xECW\x1Cj\x14a\tMW\x80c\xEDn\xA3:\x14a\tmW`\0\x80\xFD[\x80c\xE1\x9A\xBE\xF8\x11a\0\xC6W\x80c\xE1\x9A\xBE\xF8\x14a\x08\xCDW\x80c\xE8\x18<D\x14a\x08\xEDW\x80c\xEA\x0C\xDE\x85\x14a\t\rW`\0\x80\xFD[\x80c\xD1\xDE\xBA\x1F\x14a\x08\x87W\x80c\xDDu|4\x14a\x08\x9AW\x80c\xDD\xAC]\xC1\x14a\x08\xAFW`\0\x80\xFD[\x80c\xB1\xC3\x94\"\x11a\x01YW\x80c\xC9#\x0C]\x11a\x013W\x80c\xC9#\x0C]\x14a\x07\xFCW\x80c\xCB\xED\x8B\x9C\x14a\x08\x11W\x80c\xCD\xFC\xEE\xBA\x14a\x081W\x80c\xCF\xC7\xE2\xDA\x14a\x08rW`\0\x80\xFD[\x80c\xB1\xC3\x94\"\x14a\x07\x94W\x80c\xB3S\xAA\xA7\x14a\x07\xB5W\x80c\xC2\xD4\x16\x01\x14a\x07\xD5W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\x01\x8AW\x80c\x8D\xA5\xCB[\x14a\x07CW\x80c\x9A\x03cl\x14a\x07aW\x80c\xA3\xBCd\xF2\x14a\x07tW`\0\x80\xFD[\x80cu3\xD7\x88\x14a\x06\xEEW\x80c},\x85 \x14a\x07\x0EW\x80c\x82[_\x8D\x14a\x07.W`\0\x80\xFD[\x80c0-_K\x11a\x02\x7FW\x80cN\xE7\xDE\xD6\x11a\x02(W\x80c[\x8CA\xE6\x11a\x02\x02W\x80c[\x8CA\xE6\x14a\x06JW\x80c^\x10\xB2\xB7\x14a\x06\x99W\x80cf\xAD\\\x8A\x14a\x06\xB9W\x80cqP\x18\xA6\x14a\x06\xD9W`\0\x80\xFD[\x80cN\xE7\xDE\xD6\x14a\x05\xF5W\x80cR\x1F\xF0W\x14a\x06\x15W\x80cX\x98\xA0\xA8\x14a\x065W`\0\x80\xFD[\x80c?\xE34z\x11a\x02YW\x80c?\xE34z\x14a\x05\xA4W\x80cB\xD6Z\x8D\x14a\x05\xC0W\x80cN<\x10\xB7\x14a\x05\xE0W`\0\x80\xFD[\x80c0-_K\x14a\x052W\x80c0\xF4\x9C\xAC\x14a\x05TW\x80c=\x8B8\xF6\x14a\x05tW`\0\x80\xFD[\x80c\x10\xDD\xB17\x11a\x02\xECW\x80c\x19\xE7]n\x11a\x02\xC6W\x80c\x19\xE7]n\x14a\x04tW\x80c!\xE3-U\x14a\x04\x94W\x80c$!\xE1U\x14a\x04\xCCW\x80c,\xD2mE\x14a\x05\x12W`\0\x80\xFD[\x80c\x10\xDD\xB17\x14a\x04\x12W\x80c\x15=\xC4P\x14a\x042W\x80c\x17m\xE7\xA8\x14a\x04RW`\0\x80\xFD[\x80c\x069L\x9B\x11a\x03\x1DW\x80c\x069L\x9B\x14a\x03\xB3W\x80c\x07\xE0\xDB\x17\x14a\x03\xD3W\x80c\x0B\xA9Y\t\x14a\x03\xF3W`\0\x80\xFD[\x80b\x1D5g\x14a\x03BW\x80b\x97\xA0c\x14a\x03dW\x80c\x01\xDB\xF1\x9F\x14a\x03\x9EW[`\0\x80\xFD[4\x80\x15a\x03NW`\0\x80\xFD[Pa\x03ba\x03]6`\x04a5\xA9V[a\t\xF6V[\0[4\x80\x15a\x03pW`\0\x80\xFD[P`\x0ETa\x03\x86\x90`\x01`\xB0\x1B\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xAAW`\0\x80\xFD[Pa\x03ba\x0B}V[4\x80\x15a\x03\xBFW`\0\x80\xFD[Pa\x03ba\x03\xCE6`\x04a3HV[a\x0C\x01V[4\x80\x15a\x03\xDFW`\0\x80\xFD[Pa\x03ba\x03\xEE6`\x04a4\xA6V[a\x0C\xA5V[4\x80\x15a\x03\xFFW`\0\x80\xFD[P`\x07T[`@Q\x90\x81R` \x01a\x03\x95V[4\x80\x15a\x04\x1EW`\0\x80\xFD[Pa\x03ba\x04-6`\x04a4\xA6V[a\r]V[4\x80\x15a\x04>W`\0\x80\xFD[Pa\x03ba\x04M6`\x04a6\xEAV[a\r\xEBV[4\x80\x15a\x04^W`\0\x80\xFD[Pa\x04ga\x0ERV[`@Qa\x03\x95\x91\x90a9#V[4\x80\x15a\x04\x80W`\0\x80\xFD[Pa\x03ba\x04\x8F6`\x04a6\xEAV[a\x0E\xD8V[4\x80\x15a\x04\xA0W`\0\x80\xFD[P`\x04Ta\x04\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x95V[4\x80\x15a\x04\xD8W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\t\x81R\x7FlayerZero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x04gV[4\x80\x15a\x05\x1EW`\0\x80\xFD[P`\x05Ta\x04\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05>W`\0\x80\xFD[P`\x0ETa\x03\x86\x90`\x01`\xA0\x1B\x90\x04a\xFF\xFF\x16\x81V[4\x80\x15a\x05`W`\0\x80\xFD[Pa\x03ba\x05o6`\x04a3HV[a\x0F7V[4\x80\x15a\x05\x80W`\0\x80\xFD[Pa\x05\x94a\x05\x8F6`\x04a4\xF8V[a\x0F\xB0V[`@Q\x90\x15\x15\x81R` \x01a\x03\x95V[4\x80\x15a\x05\xB0W`\0\x80\xFD[P`\0`@Qa\x03\x95\x91\x90a96V[4\x80\x15a\x05\xCCW`\0\x80\xFD[Pa\x03ba\x05\xDB6`\x04a4\xF8V[a\x10}V[4\x80\x15a\x05\xECW`\0\x80\xFD[Pa\x04ga\x11BV[4\x80\x15a\x06\x01W`\0\x80\xFD[Pa\x03ba\x06\x106`\x04a4\xC1V[a\x11\xD0V[4\x80\x15a\x06!W`\0\x80\xFD[Pa\x03ba\x0606`\x04a6\xEAV[a\x12\x7FV[4\x80\x15a\x06AW`\0\x80\xFD[P`\nTa\x04\x04V[4\x80\x15a\x06VW`\0\x80\xFD[Pa\x04\x04a\x06e6`\x04a5KV[`\x10` \x90\x81R`\0\x93\x84R`@\x80\x85 \x84Q\x80\x86\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x95\x84\x01\x95\x90\x95 \x94R\x92\x90R\x82R\x90 T\x81V[4\x80\x15a\x06\xA5W`\0\x80\xFD[Pa\x03ba\x06\xB46`\x04a6\xEAV[a\x13-V[4\x80\x15a\x06\xC5W`\0\x80\xFD[Pa\x03ba\x06\xD46`\x04a5\xA9V[a\x13\x8DV[4\x80\x15a\x06\xE5W`\0\x80\xFD[Pa\x03ba\x13\xBFV[4\x80\x15a\x06\xFAW`\0\x80\xFD[Pa\x04ga\x07\t6`\x04a4\xA6V[a\x14%V[4\x80\x15a\x07\x1AW`\0\x80\xFD[Pa\x03ba\x07)6`\x04a7>V[a\x14>V[4\x80\x15a\x07:W`\0\x80\xFD[P`\x0BTa\x04\x04V[4\x80\x15a\x07OW`\0\x80\xFD[P`\rT`\x01`\x01`\xA0\x1B\x03\x16a\x04\xB4V[a\x03ba\x07o6`\x04a3\xE8V[a\x14\xDBV[4\x80\x15a\x07\x80W`\0\x80\xFD[Pa\x03ba\x07\x8F6`\x04a3HV[a\x16\xEBV[4\x80\x15a\x07\xA0W`\0\x80\xFD[P`\0Ta\x05\x94\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x07\xC1W`\0\x80\xFD[P`\x0ETa\x04\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\xE1W`\0\x80\xFD[Pa\x07\xEAa\x178V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03\x95V[4\x80\x15a\x08\x08W`\0\x80\xFD[Pa\x04ga\x17\xB5V[4\x80\x15a\x08\x1DW`\0\x80\xFD[Pa\x03ba\x08,6`\x04a6{V[a\x17\xFAV[4\x80\x15a\x08=W`\0\x80\xFD[P`\x02Ta\x08Y\x90`\x01`\xA0\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\x95V[4\x80\x15a\x08~W`\0\x80\xFD[P`\x06Ta\x04\x04V[a\x03ba\x08\x956`\x04a5\xA9V[a\x18\xC5V[4\x80\x15a\x08\xA6W`\0\x80\xFD[Pa\x03ba\x19\xD5V[4\x80\x15a\x08\xBBW`\0\x80\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xB4V[4\x80\x15a\x08\xD9W`\0\x80\xFD[Pa\x03ba\x08\xE86`\x04a3HV[a\x1AMV[4\x80\x15a\x08\xF9W`\0\x80\xFD[Pa\x03ba\t\x086`\x04a7\x1CV[a\x1A\x9AV[4\x80\x15a\t\x19W`\0\x80\xFD[Pa\x03ba\t(6`\x04a3eV[a\x1B-V[4\x80\x15a\t9W`\0\x80\xFD[Pa\x03ba\tH6`\x04a4\xF8V[a\x1B\xA5V[4\x80\x15a\tYW`\0\x80\xFD[P`\0Ta\x04\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\tyW`\0\x80\xFD[P`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x05\x94V[4\x80\x15a\t\x98W`\0\x80\xFD[P`\x08Ta\x04\x04V[4\x80\x15a\t\xADW`\0\x80\xFD[Pa\x03ba\t\xBC6`\x04a3HV[a\x1C\x93V[4\x80\x15a\t\xCDW`\0\x80\xFD[P`\tTa\x04\x04V[4\x80\x15a\t\xE2W`\0\x80\xFD[Pa\x04ga\t\xF16`\x04a6.V[a\x1DuV[`\x0ET`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\n_W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7Fendpoint caller\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF\x84\x16`\0\x90\x81R`\x0F` R`@\x81 \x80Ta\n}\x90a;\x82V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xA9\x90a;\x82V[\x80\x15a\n\xF6W\x80`\x1F\x10a\n\xCBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xF6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xD9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q\x84Q\x14\x15\x80a\x0B\x1CWP\x80\x80Q\x90` \x01 \x84\x80Q\x90` \x01 \x14\x15[\x15a\x0BjW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fsource sending contract\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[a\x0Bv\x85\x85\x85\x85a\x1E\x17V[PPPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xA8W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x90\x81\x17\x91\x82\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x92a\x0B\xF7\x92\x90\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C,W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x15a\x0C[W`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FG!\x12\x9E\x0Egn\xD6\xA9)\t\xBB$\xE8S\xCC\xDDc\xADr(\x0C\xC2\xE9t\xE3\x8EH\x0E\x0EnT\x90`\0\x90\xA2PV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0ET`@Qc\x07\xE0\xDB\x17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07\xE0\xDB\x17\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rIW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0BvW=`\0\x80>=`\0\xFD[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0ET`@Qc\x10\xDD\xB17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x10\xDD\xB17\x90`$\x01a\r/V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x16W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n\x81\x90U`@Q\x81\x81R\x7F\x14\x98\x824\xD3\xE5\n\x12\xAE\xEC-n\xE5\x95\xB7\x0C/\xAE\x16<\xAF\xF2\x8B\xAA\xB5\x80+\x12+\xF2\xA7S\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x11T`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0E\x97W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0E\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\xD3\x91\x90\x81\x01\x90a3\x9FV[\x90P\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\x03W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x81\x90U`@Q\x81\x81R~\x91\xF5\xF5\xDB0\x92\xE3\x9E\xCBp\x12\x18\xD4\xAF \xB7W\x1E\x04)YY7\xC34\xF3\xAC\xD1O\xE2\xFE\x90` \x01a\x0EGV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0FbW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xBE\x18\x08\x91~\t\x81O\xCA}\x80(\x9A\x97`\xA4\x06_\x10\xD6$\xAC\x1A\xEE\x0B\x9F\x90<O]\x92\xB1\x90` \x01a\x0EGV[a\xFF\xFF\x83\x16`\0\x90\x81R`\x0F` R`@\x81 \x80T\x82\x91\x90a\x0F\xD1\x90a;\x82V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xFD\x90a;\x82V[\x80\x15a\x10JW\x80`\x1F\x10a\x10\x1FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10JV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10-W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83\x83`@Qa\x10a\x92\x91\x90a8,V[`@Q\x80\x91\x03\x90 \x81\x80Q\x90` \x01 \x14\x91PP[\x93\x92PPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0ET`@QcB\xD6Z\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cB\xD6Z\x8D\x90a\x11\x0B\x90\x86\x90\x86\x90\x86\x90`\x04\x01a9\xBBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11%W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x119W=`\0\x80>=`\0\xFD[PPPPPPPV[`\x03\x80Ta\x11O\x90a;\x82V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11{\x90a;\x82V[\x80\x15a\x11\xC8W\x80`\x1F\x10a\x11\x9DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\xC8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x11\xABW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0E\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\xA0\x1Ba\xFF\xFF\x94\x90\x94\x16\x93\x90\x93\x02`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x92\x17`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\xAAW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x12\xF8W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fpeer minimal rollup fee\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[`\x0B\x81\x90U`@Q\x81\x81R\x7F\x87\x80u\xA8\xE8\xAA\x1C|\x15\xE6\x93'RR\x0Fx\x12\xBFWDx^m\xF6\x0577)\xDAAUE\x90` \x01a\x0EGV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13XW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t\x81\x90U`@Q\x81\x81R\x7F\x88\xD2\x10\xDF\xA1\x98\xF7Q\x95y)G!\xF9\x0Cw\x11S\xA7\xB4\x91\x01\xEE\xFB\x95\x14p7\xCC\x8C\xE4\x81\x90` \x01a\x0EGV[30\x14a\x13\xADW`@Qcq\xF5\x0E\xC1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\xB9\x84\x84\x84\x84a\x1ExV[PPPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[a\x14#`\0a\x1E\xB3V[V[`\x0F` R`\0\x90\x81R`@\x90 \x80Ta\x11O\x90a;\x82V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14iW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x02\x17\x90U\x81Qa\x14\xB6\x90`\x03\x90` \x85\x01\x90a1!V[P`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPV[`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x15\x06W`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T\x81Q\x10\x15a\x15*W`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T\x81Q\x11\x15a\x15NW`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T\x81`\xA0\x01Q\x10\x15a\x15uW`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT\x81`\xC0\x01Q\x10\x15a\x15\x9CW`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT\x81`\xE0\x01Q\x10\x15a\x15\xC3W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x15\xDC\x82`@\x01Q\x83`\0\x01Q\x84``\x01Qa\x1F\x05V[\x90P\x80\x82` \x01Q\x14a\x16\x02W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\x0B3a \"V[\x15a\x16)W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`\xC0\x84\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x83\x01Q``\x82\x01R`\x80\x80\x84\x01Q\x90\x82\x01R`\0a\x16l\x82a \xBFV[\x90Pa\x16|\x84`\xA0\x01Q\x82a!.V[`\x02T`\xE0\x85\x01Q`\xC0\x86\x01Q\x86Qa\x16\xB7\x93`\x01`\x01`\xA0\x1B\x03\x16\x92\x91a\x16\xA3\x91a;0V[a\x16\xAD\x91\x90a;0V[\x86`\xA0\x01Qa!`V[` \x84\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\x16W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x11T`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1<\xE5g\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x17}W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x17\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD3\x91\x90a7\xB4V[`\x11T`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0E\x97W`\0\x80\xFD[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0ET`@Qc2\xFBb\xE7`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCB\xED\x8B\x9C\x90a\x18\x8C\x90\x88\x90\x88\x90\x88\x90\x88\x90\x88\x90`\x04\x01a:\x7FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\xBAW=`\0\x80>=`\0\xFD[PPPPPPPPPV[a\xFF\xFF\x84\x16`\0\x90\x81R`\x10` R`@\x80\x82 \x90Qa\x18\xE6\x90\x86\x90a8<V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R\x92R\x90 T\x90P\x80a\x19,W`@Qc+\x96\xC9\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q` \x83\x01 \x81\x14a\x19\x82W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x07`$\x82\x01R\x7Fpayload\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[a\xFF\xFF\x85\x16`\0\x90\x81R`\x10` R`@\x80\x82 \x90Qa\x19\xA3\x90\x87\x90a8<V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\0\x90\x81R\x92R\x90 Ua\x0Bv\x85\x85\x85\x85a\x1ExV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90\x81\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x91a\x0B\xF7\x91`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1AxW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xC5W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x1A\xE6W`@Qc`\x03\xE8!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x82\x90U`\x06\x81\x90U`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x7Fv3\0Lz\"\x98i\xAE\xEA\x10\xDBO\xF3\xE5~;\x154\xAE\xB2\xC9\xE7,]\xB2_\x96X\x95\xC30\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BXW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T\x82\x15\x15`\x01`\xA0\x1B\x02`\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x90U`@Q\x7F\xCD\x16,o\xC2B\x85\xBF\xBE9\x9E\xC0\xCC,\xE2\xC3\x80\xAD'\xD2\xEB\x1F\xCAA\x8Ce-\xF1%~~\t\x90a\x0EG\x90\x83\x15\x15\x81R` \x01\x90V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0E\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xB0\x1Ba\xFF\xFF\x86\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`\0\x90\x81R`\x0F` R`@\x90 a\x1CR\x90\x83\x83a1\xA5V[P\x7F\xFAAHz\xD5\xD6r\x8F\x0B\x19'o\xA1\xED\xDC\x16U\x85x\xF5\x10\x9F\xC3\x9D-\xC3<20G\r\xAB\x83\x83\x83`@Qa\x1C\x86\x93\x92\x91\x90a9\xBBV[`@Q\x80\x91\x03\x90\xA1PPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1DiW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nVV[a\x1Dr\x81a\x1E\xB3V[PV[`\x0ET`@Qc={/o`\xE2\x1B\x81Ra\xFF\xFF\x80\x87\x16`\x04\x83\x01R\x85\x16`$\x82\x01R0`D\x82\x01R`d\x81\x01\x83\x90R``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF5\xEC\xBD\xBC\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1D\xD2W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1D\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\x0E\x91\x90\x81\x01\x90a3\x9FV[\x95\x94PPPPPV[`@Qc3V\xAEE`\xE1\x1B\x81R0\x90cf\xAD\\\x8A\x90a\x1E@\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a:@V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1EZW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1EnW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0a\x1E\x83\x82a!\xCCV[`\x02T`\x04T\x91\x92Pa\x0Bv\x91`\x01`\xA0\x1B\x90\x91\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90`\x01`\x01`\xA0\x1B\x03\x162\x84a\"\x94V[`\r\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x1FHW`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x1FzW`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x1F\xD2\x91`\x04\x01a8\xF2V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1F\xEAW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1F\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x0E\x91\x90a7\x03V[`\0\x80T`\x01`\xA0\x1B\x90\x04`\xFF\x16a <WP`\0\x91\x90PV[`\0T`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a \x81W`\0\x80\xFD[PZ\xFA\x15\x80\x15a \x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xB9\x91\x90a3\x82V[\x92\x91PPV[``\x80a \xCF\x83`\0\x01Qa#IV[a \xDC\x84` \x01Qa#IV[a \xE9\x85`@\x01Qa#IV[a \xF6\x86``\x01Qa#IV[a!\x03\x87`\x80\x01Qa#\xE1V[`@Q` \x01a!\x17\x95\x94\x93\x92\x91\x90a8\x87V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[a!\\`\x0E`\x16\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16\x823`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x87a$\x18V[PPV[\x804\x14a!\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Fbridge fee mismatch\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[`\x11Ta!\xC7\x90`\x01`\x01`\xA0\x1B\x03\x163\x85\x85a%OV[PPPV[a!\xFE`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\"0`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\"<\x84\x82a%\xBEV[\x90\x83R\x90Pa\"K\x84\x82a%\xBEV[` \x84\x01\x91\x90\x91R\x90Pa\"_\x84\x82a%\xBEV[`@\x84\x01\x91\x90\x91R\x90Pa\"s\x84\x82a%\xBEV[``\x84\x01\x91\x90\x91R\x90Pa\"\x87\x84\x82a&\xEFV[P`\x80\x83\x01RP\x92\x91PPV[`\x04T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\"\xC2W`@Qc\x14@\xE0y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xA0\x1B\x90\x92\x04\x16\x14a\"\xF8W`@Qc8v0O`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qa#\x17W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`@Qcx\xD6\x0C\xD7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx\xD6\x0C\xD7\x90a\x1E@\x90\x84\x90\x86\x90`\x04\x01a9^V[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a#\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a#\xD1W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a#\xAFV[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a#\xEF\x81a'\xFCV[\x83`@Q` \x01a$\x01\x92\x91\x90a8XV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[a\xFF\xFF\x86\x16`\0\x90\x81R`\x0F` R`@\x81 \x80Ta$6\x90a;\x82V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$b\x90a;\x82V[\x80\x15a$\xAFW\x80`\x1F\x10a$\x84Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$\xAFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\x92W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q`\0\x14\x15a$\xD9W`@Qc\x02\x0B5\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0ET`@Qb\xC5\x801`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5\x801\0\x90\x84\x90a%\x14\x90\x8B\x90\x86\x90\x8C\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a9\xD9V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a%-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a%AW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x13\xB9\x90\x85\x90a(\xC9V[`\0\x80\x83Q\x83` a%\xD0\x91\x90a;0V[\x11\x15\x80\x15a%\xE7WPa%\xE4\x83` a;0V[\x83\x10[a&?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\nVV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a&tW\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa&TV[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a&\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[\x80a&\xE2\x85` a;0V[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a&\xFE\x85\x85a)\xAEV[\x86Q\x90\x95P\x90\x91Pa'\x10\x82\x86a;0V[\x11\x15\x80\x15a'&WPa'#\x81\x85a;0V[\x84\x10[a'~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\nVV[``\x81\x15\x80\x15a'\x99W`@Q\x91P` \x82\x01`@Ra'\xE3V[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a'\xD2W\x80Q\x83R` \x92\x83\x01\x92\x01a'\xBAV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a'\xEF\x83\x87a;0V[\x93P\x93PPP\x92P\x92\x90PV[```\xFD\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a(/W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91Ra \xB9V[a\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a(\x7FWa(O`\xFD`\xF8\x1Ba+\xB4V[a(X\x83a+\xDBV[`@Q` \x01a(i\x92\x91\x90a8XV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a(\xAAWa(\xA1`\x7F`\xF9\x1Ba+\xB4V[a(X\x83a,\x1EV[a(\xBB`\x01`\x01`\xF8\x1B\x03\x19a+\xB4V[a(X\x83a,aV[\x91\x90PV[`\0a)\x1E\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a,\xA4\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a!\xC7W\x80\x80` \x01\x90Q\x81\x01\x90a)<\x91\x90a3\x82V[a!\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nVV[`\0\x80`\0a)\xBD\x85\x85a,\xBBV[\x94P\x90P`\0`\xFD`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a*VWa)\xE3\x86\x86a-CV[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a)\xFEWPa\xFF\xFF\x81\x11\x15[a*JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\nVV[\x92P\x83\x91Pa&\xE8\x90PV[`\x7F`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a*\xE1Wa*v\x86\x86a-\xFCV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a*\x95WPc\xFF\xFF\xFF\xFF\x81\x11\x15[a*JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\nVV[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x14\x15a+^Wa*\xFD\x86\x86a.\xCEV[\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a*JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\nVV[P`\xF8\x81\x90\x1C`\xFD\x81\x10a*JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\nVV[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a \xB9V[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a,\x0EW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a+\xECV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a,QW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a,/V[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a,\x94W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a,rV[PPP`(\x81\x01`@R\x92\x91PPV[``a,\xB3\x84\x84`\0\x85a/\xA0V[\x94\x93PPPPV[`\0\x80\x83Q\x83`\x01a,\xCD\x91\x90a;0V[\x11\x15\x80\x15a,\xE4WPa,\xE1\x83`\x01a;0V[\x83\x10[a-0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\nVV[\x83\x83\x01` \x01Q\x80a&\xE2\x85`\x01a;0V[`\0\x80\x83Q\x83`\x02a-U\x91\x90a;0V[\x11\x15\x80\x15a-lWPa-i\x83`\x02a;0V[\x83\x10[a-\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\nVV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a&\xE2\x91\x90a;0V[`\0\x80\x83Q\x83`\x04a.\x0E\x91\x90a;0V[\x11\x15\x80\x15a.%WPa.\"\x83`\x04a;0V[\x83\x10[a.|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\nVV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a.\xB1W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa.\x91V[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a&\xE2\x85`\x04a;0V[`\0\x80\x83Q\x83`\x08a.\xE0\x91\x90a;0V[\x11\x15\x80\x15a.\xF7WPa.\xF4\x83`\x08a;0V[\x83\x10[a/NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\nVV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a/\x83W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa/cV[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a&\xE2\x85`\x08a;0V[``\x82G\x10\x15a0\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nVV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a0oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\nVV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa0\x8B\x91\x90a8<V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a0\xC8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a0\xCDV[``\x91P[P\x91P\x91Pa0\xDD\x82\x82\x86a0\xE8V[\x97\x96PPPPPPPV[``\x83\x15a0\xF7WP\x81a\x10vV[\x82Q\x15a1\x07W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nV\x91\x90a9#V[\x82\x80Ta1-\x90a;\x82V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a1OW`\0\x85Ua1\x95V[\x82`\x1F\x10a1hW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua1\x95V[\x82\x80\x01`\x01\x01\x85U\x82\x15a1\x95W\x91\x82\x01[\x82\x81\x11\x15a1\x95W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a1zV[Pa1\xA1\x92\x91Pa2\x19V[P\x90V[\x82\x80Ta1\xB1\x90a;\x82V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a1\xD3W`\0\x85Ua1\x95V[\x82`\x1F\x10a1\xECW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua1\x95V[\x82\x80\x01`\x01\x01\x85U\x82\x15a1\x95W\x91\x82\x01[\x82\x81\x11\x15a1\x95W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a1\xFEV[[\x80\x82\x11\x15a1\xA1W`\0\x81U`\x01\x01a2\x1AV[`\0a2Aa2<\x84a;\x08V[a:\xD7V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a2UW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0a2za2<\x84a;\x08V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a2\x8EW`\0\x80\xFD[a\x10v\x83` \x83\x01\x84a;VV[`\0\x80\x83`\x1F\x84\x01\x12a2\xAEW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xC6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a&\xE8W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a2\xEFW`\0\x80\xFD[a\x10v\x83\x835` \x85\x01a2.V[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(\xC4W`\0\x80\xFD[\x805a\xFF\xFF\x81\x16\x81\x14a(\xC4W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(\xC4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a3ZW`\0\x80\xFD[\x815a\x10v\x81a;\xD3V[`\0` \x82\x84\x03\x12\x15a3wW`\0\x80\xFD[\x815a\x10v\x81a;\xE8V[`\0` \x82\x84\x03\x12\x15a3\x94W`\0\x80\xFD[\x81Qa\x10v\x81a;\xE8V[`\0` \x82\x84\x03\x12\x15a3\xB1W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\xC8W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a3\xD9W`\0\x80\xFD[a,\xB3\x84\x82Q` \x84\x01a2lV[`\0` \x82\x84\x03\x12\x15a3\xFAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a4\x12W`\0\x80\xFD[\x90\x83\x01\x90a\x01\0\x82\x86\x03\x12\x15a4'W`\0\x80\xFD[a4/a:\xADV[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01Ra4S``\x84\x01a2\xFEV[``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a4jW`\0\x80\xFD[a4v\x87\x82\x86\x01a2\xDEV[`\x80\x83\x01RP`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015`\xE0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a4\xB8W`\0\x80\xFD[a\x10v\x82a3\x1EV[`\0\x80`@\x83\x85\x03\x12\x15a4\xD4W`\0\x80\xFD[a4\xDD\x83a3\x1EV[\x91P` \x83\x015a4\xED\x81a;\xD3V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a5\rW`\0\x80\xFD[a5\x16\x84a3\x1EV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a52W`\0\x80\xFD[a5>\x86\x82\x87\x01a2\x9CV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a5`W`\0\x80\xFD[a5i\x84a3\x1EV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\x85W`\0\x80\xFD[a5\x91\x86\x82\x87\x01a2\xDEV[\x92PPa5\xA0`@\x85\x01a30V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a5\xBFW`\0\x80\xFD[a5\xC8\x85a3\x1EV[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a5\xE5W`\0\x80\xFD[a5\xF1\x88\x83\x89\x01a2\xDEV[\x94Pa5\xFF`@\x88\x01a30V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a6\x15W`\0\x80\xFD[Pa6\"\x87\x82\x88\x01a2\xDEV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a6DW`\0\x80\xFD[a6M\x85a3\x1EV[\x93Pa6[` \x86\x01a3\x1EV[\x92P`@\x85\x015a6k\x81a;\xD3V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a6\x93W`\0\x80\xFD[a6\x9C\x86a3\x1EV[\x94Pa6\xAA` \x87\x01a3\x1EV[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xCDW`\0\x80\xFD[a6\xD9\x88\x82\x89\x01a2\x9CV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a6\xFCW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a7\x15W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a7/W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a7SW`\0\x80\xFD[a7\\\x84a30V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7xW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a7\x89W`\0\x80\xFD[a7\x98\x86\x825` \x84\x01a2.V[\x92PP`@\x84\x015a7\xA9\x81a;\xD3V[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a7\xC6W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x10vW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x81Q\x80\x84Ra8\x18\x81` \x86\x01` \x86\x01a;VV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82Qa8N\x81\x84` \x87\x01a;VV[\x91\x90\x91\x01\x92\x91PPV[`\0\x83Qa8j\x81\x84` \x88\x01a;VV[\x83Q\x90\x83\x01\x90a8~\x81\x83` \x88\x01a;VV[\x01\x94\x93PPPPV[`\0\x86Qa8\x99\x81\x84` \x8B\x01a;VV[\x86Q\x90\x83\x01\x90a8\xAD\x81\x83` \x8B\x01a;VV[\x86Q\x91\x01\x90a8\xC0\x81\x83` \x8A\x01a;VV[\x85Q\x91\x01\x90a8\xD3\x81\x83` \x89\x01a;VV[\x84Q\x91\x01\x90a8\xE6\x81\x83` \x88\x01a;VV[\x01\x97\x96PPPPPPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a9\x1AW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a8\xFBV[PPP\x92\x91PPV[` \x81R`\0a\x10v` \x83\x01\x84a8\0V[` \x81\x01`\x02\x83\x10a9XWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra9\xA2`\xE0\x84\x01\x82a8\0V[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[a\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x1E\x0E`@\x83\x01\x84\x86a7\xD7V[a\xFF\xFF\x87\x16\x81R`\xC0` \x82\x01R`\0a9\xF6`\xC0\x83\x01\x88a8\0V[\x82\x81\x03`@\x84\x01Ra:\x08\x81\x88a8\0V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16``\x86\x01R\x86\x16`\x80\x85\x01R\x83\x81\x03`\xA0\x85\x01R\x90Pa:3\x81\x85a8\0V[\x99\x98PPPPPPPPPV[a\xFF\xFF\x85\x16\x81R`\x80` \x82\x01R`\0a:]`\x80\x83\x01\x86a8\0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra0\xDD\x81\x85a8\0V[`\0a\xFF\xFF\x80\x88\x16\x83R\x80\x87\x16` \x84\x01RP\x84`@\x83\x01R`\x80``\x83\x01Ra0\xDD`\x80\x83\x01\x84\x86a7\xD7V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a:\xD1Wa:\xD1a;\xBDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;\0Wa;\0a;\xBDV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a;\"Wa;\"a;\xBDV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82\x19\x82\x11\x15a;QWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[`\0[\x83\x81\x10\x15a;qW\x81\x81\x01Q\x83\x82\x01R` \x01a;YV[\x83\x81\x11\x15a\x13\xB9WPP`\0\x91\x01RV[`\x01\x81\x81\x1C\x90\x82\x16\x80a;\x96W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a;\xB7WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1DrW`\0\x80\xFD[\x80\x15\x15\x81\x14a\x1DrW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xF1\x8F@\x84\xAD\xC5\xF2\xCB\xE1\xE7#n\x87\xC6\x8D\xE1]\xF2\xB2O\xC0\x19<\xA8\xED\x08\x07\xED\xAAf\xC2\xAFdsolcC\0\x08\x07\x003";
    /// The bytecode of the contract.
    pub static MYSTIKOV2LAYERZEROERC20_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x03=W`\x005`\xE0\x1C\x80cu3\xD7\x88\x11a\x01\xB0W\x80c\xD1\xDE\xBA\x1F\x11a\0\xECW\x80c\xEB\x8Dr\xB7\x11a\0\x95W\x80c\xEF\xBF\xB2\xAE\x11a\0oW\x80c\xEF\xBF\xB2\xAE\x14a\t\x8CW\x80c\xF2\xFD\xE3\x8B\x14a\t\xA1W\x80c\xF4\xAD\x17\xC6\x14a\t\xC1W\x80c\xF5\xEC\xBD\xBC\x14a\t\xD6W`\0\x80\xFD[\x80c\xEB\x8Dr\xB7\x14a\t-W\x80c\xECW\x1Cj\x14a\tMW\x80c\xEDn\xA3:\x14a\tmW`\0\x80\xFD[\x80c\xE1\x9A\xBE\xF8\x11a\0\xC6W\x80c\xE1\x9A\xBE\xF8\x14a\x08\xCDW\x80c\xE8\x18<D\x14a\x08\xEDW\x80c\xEA\x0C\xDE\x85\x14a\t\rW`\0\x80\xFD[\x80c\xD1\xDE\xBA\x1F\x14a\x08\x87W\x80c\xDDu|4\x14a\x08\x9AW\x80c\xDD\xAC]\xC1\x14a\x08\xAFW`\0\x80\xFD[\x80c\xB1\xC3\x94\"\x11a\x01YW\x80c\xC9#\x0C]\x11a\x013W\x80c\xC9#\x0C]\x14a\x07\xFCW\x80c\xCB\xED\x8B\x9C\x14a\x08\x11W\x80c\xCD\xFC\xEE\xBA\x14a\x081W\x80c\xCF\xC7\xE2\xDA\x14a\x08rW`\0\x80\xFD[\x80c\xB1\xC3\x94\"\x14a\x07\x94W\x80c\xB3S\xAA\xA7\x14a\x07\xB5W\x80c\xC2\xD4\x16\x01\x14a\x07\xD5W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\x01\x8AW\x80c\x8D\xA5\xCB[\x14a\x07CW\x80c\x9A\x03cl\x14a\x07aW\x80c\xA3\xBCd\xF2\x14a\x07tW`\0\x80\xFD[\x80cu3\xD7\x88\x14a\x06\xEEW\x80c},\x85 \x14a\x07\x0EW\x80c\x82[_\x8D\x14a\x07.W`\0\x80\xFD[\x80c0-_K\x11a\x02\x7FW\x80cN\xE7\xDE\xD6\x11a\x02(W\x80c[\x8CA\xE6\x11a\x02\x02W\x80c[\x8CA\xE6\x14a\x06JW\x80c^\x10\xB2\xB7\x14a\x06\x99W\x80cf\xAD\\\x8A\x14a\x06\xB9W\x80cqP\x18\xA6\x14a\x06\xD9W`\0\x80\xFD[\x80cN\xE7\xDE\xD6\x14a\x05\xF5W\x80cR\x1F\xF0W\x14a\x06\x15W\x80cX\x98\xA0\xA8\x14a\x065W`\0\x80\xFD[\x80c?\xE34z\x11a\x02YW\x80c?\xE34z\x14a\x05\xA4W\x80cB\xD6Z\x8D\x14a\x05\xC0W\x80cN<\x10\xB7\x14a\x05\xE0W`\0\x80\xFD[\x80c0-_K\x14a\x052W\x80c0\xF4\x9C\xAC\x14a\x05TW\x80c=\x8B8\xF6\x14a\x05tW`\0\x80\xFD[\x80c\x10\xDD\xB17\x11a\x02\xECW\x80c\x19\xE7]n\x11a\x02\xC6W\x80c\x19\xE7]n\x14a\x04tW\x80c!\xE3-U\x14a\x04\x94W\x80c$!\xE1U\x14a\x04\xCCW\x80c,\xD2mE\x14a\x05\x12W`\0\x80\xFD[\x80c\x10\xDD\xB17\x14a\x04\x12W\x80c\x15=\xC4P\x14a\x042W\x80c\x17m\xE7\xA8\x14a\x04RW`\0\x80\xFD[\x80c\x069L\x9B\x11a\x03\x1DW\x80c\x069L\x9B\x14a\x03\xB3W\x80c\x07\xE0\xDB\x17\x14a\x03\xD3W\x80c\x0B\xA9Y\t\x14a\x03\xF3W`\0\x80\xFD[\x80b\x1D5g\x14a\x03BW\x80b\x97\xA0c\x14a\x03dW\x80c\x01\xDB\xF1\x9F\x14a\x03\x9EW[`\0\x80\xFD[4\x80\x15a\x03NW`\0\x80\xFD[Pa\x03ba\x03]6`\x04a5\xA9V[a\t\xF6V[\0[4\x80\x15a\x03pW`\0\x80\xFD[P`\x0ETa\x03\x86\x90`\x01`\xB0\x1B\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xAAW`\0\x80\xFD[Pa\x03ba\x0B}V[4\x80\x15a\x03\xBFW`\0\x80\xFD[Pa\x03ba\x03\xCE6`\x04a3HV[a\x0C\x01V[4\x80\x15a\x03\xDFW`\0\x80\xFD[Pa\x03ba\x03\xEE6`\x04a4\xA6V[a\x0C\xA5V[4\x80\x15a\x03\xFFW`\0\x80\xFD[P`\x07T[`@Q\x90\x81R` \x01a\x03\x95V[4\x80\x15a\x04\x1EW`\0\x80\xFD[Pa\x03ba\x04-6`\x04a4\xA6V[a\r]V[4\x80\x15a\x04>W`\0\x80\xFD[Pa\x03ba\x04M6`\x04a6\xEAV[a\r\xEBV[4\x80\x15a\x04^W`\0\x80\xFD[Pa\x04ga\x0ERV[`@Qa\x03\x95\x91\x90a9#V[4\x80\x15a\x04\x80W`\0\x80\xFD[Pa\x03ba\x04\x8F6`\x04a6\xEAV[a\x0E\xD8V[4\x80\x15a\x04\xA0W`\0\x80\xFD[P`\x04Ta\x04\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x95V[4\x80\x15a\x04\xD8W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\t\x81R\x7FlayerZero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x04gV[4\x80\x15a\x05\x1EW`\0\x80\xFD[P`\x05Ta\x04\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05>W`\0\x80\xFD[P`\x0ETa\x03\x86\x90`\x01`\xA0\x1B\x90\x04a\xFF\xFF\x16\x81V[4\x80\x15a\x05`W`\0\x80\xFD[Pa\x03ba\x05o6`\x04a3HV[a\x0F7V[4\x80\x15a\x05\x80W`\0\x80\xFD[Pa\x05\x94a\x05\x8F6`\x04a4\xF8V[a\x0F\xB0V[`@Q\x90\x15\x15\x81R` \x01a\x03\x95V[4\x80\x15a\x05\xB0W`\0\x80\xFD[P`\0`@Qa\x03\x95\x91\x90a96V[4\x80\x15a\x05\xCCW`\0\x80\xFD[Pa\x03ba\x05\xDB6`\x04a4\xF8V[a\x10}V[4\x80\x15a\x05\xECW`\0\x80\xFD[Pa\x04ga\x11BV[4\x80\x15a\x06\x01W`\0\x80\xFD[Pa\x03ba\x06\x106`\x04a4\xC1V[a\x11\xD0V[4\x80\x15a\x06!W`\0\x80\xFD[Pa\x03ba\x0606`\x04a6\xEAV[a\x12\x7FV[4\x80\x15a\x06AW`\0\x80\xFD[P`\nTa\x04\x04V[4\x80\x15a\x06VW`\0\x80\xFD[Pa\x04\x04a\x06e6`\x04a5KV[`\x10` \x90\x81R`\0\x93\x84R`@\x80\x85 \x84Q\x80\x86\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x95\x84\x01\x95\x90\x95 \x94R\x92\x90R\x82R\x90 T\x81V[4\x80\x15a\x06\xA5W`\0\x80\xFD[Pa\x03ba\x06\xB46`\x04a6\xEAV[a\x13-V[4\x80\x15a\x06\xC5W`\0\x80\xFD[Pa\x03ba\x06\xD46`\x04a5\xA9V[a\x13\x8DV[4\x80\x15a\x06\xE5W`\0\x80\xFD[Pa\x03ba\x13\xBFV[4\x80\x15a\x06\xFAW`\0\x80\xFD[Pa\x04ga\x07\t6`\x04a4\xA6V[a\x14%V[4\x80\x15a\x07\x1AW`\0\x80\xFD[Pa\x03ba\x07)6`\x04a7>V[a\x14>V[4\x80\x15a\x07:W`\0\x80\xFD[P`\x0BTa\x04\x04V[4\x80\x15a\x07OW`\0\x80\xFD[P`\rT`\x01`\x01`\xA0\x1B\x03\x16a\x04\xB4V[a\x03ba\x07o6`\x04a3\xE8V[a\x14\xDBV[4\x80\x15a\x07\x80W`\0\x80\xFD[Pa\x03ba\x07\x8F6`\x04a3HV[a\x16\xEBV[4\x80\x15a\x07\xA0W`\0\x80\xFD[P`\0Ta\x05\x94\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x07\xC1W`\0\x80\xFD[P`\x0ETa\x04\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\xE1W`\0\x80\xFD[Pa\x07\xEAa\x178V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03\x95V[4\x80\x15a\x08\x08W`\0\x80\xFD[Pa\x04ga\x17\xB5V[4\x80\x15a\x08\x1DW`\0\x80\xFD[Pa\x03ba\x08,6`\x04a6{V[a\x17\xFAV[4\x80\x15a\x08=W`\0\x80\xFD[P`\x02Ta\x08Y\x90`\x01`\xA0\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\x95V[4\x80\x15a\x08~W`\0\x80\xFD[P`\x06Ta\x04\x04V[a\x03ba\x08\x956`\x04a5\xA9V[a\x18\xC5V[4\x80\x15a\x08\xA6W`\0\x80\xFD[Pa\x03ba\x19\xD5V[4\x80\x15a\x08\xBBW`\0\x80\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xB4V[4\x80\x15a\x08\xD9W`\0\x80\xFD[Pa\x03ba\x08\xE86`\x04a3HV[a\x1AMV[4\x80\x15a\x08\xF9W`\0\x80\xFD[Pa\x03ba\t\x086`\x04a7\x1CV[a\x1A\x9AV[4\x80\x15a\t\x19W`\0\x80\xFD[Pa\x03ba\t(6`\x04a3eV[a\x1B-V[4\x80\x15a\t9W`\0\x80\xFD[Pa\x03ba\tH6`\x04a4\xF8V[a\x1B\xA5V[4\x80\x15a\tYW`\0\x80\xFD[P`\0Ta\x04\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\tyW`\0\x80\xFD[P`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x05\x94V[4\x80\x15a\t\x98W`\0\x80\xFD[P`\x08Ta\x04\x04V[4\x80\x15a\t\xADW`\0\x80\xFD[Pa\x03ba\t\xBC6`\x04a3HV[a\x1C\x93V[4\x80\x15a\t\xCDW`\0\x80\xFD[P`\tTa\x04\x04V[4\x80\x15a\t\xE2W`\0\x80\xFD[Pa\x04ga\t\xF16`\x04a6.V[a\x1DuV[`\x0ET`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\n_W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7Fendpoint caller\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF\x84\x16`\0\x90\x81R`\x0F` R`@\x81 \x80Ta\n}\x90a;\x82V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xA9\x90a;\x82V[\x80\x15a\n\xF6W\x80`\x1F\x10a\n\xCBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xF6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xD9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q\x84Q\x14\x15\x80a\x0B\x1CWP\x80\x80Q\x90` \x01 \x84\x80Q\x90` \x01 \x14\x15[\x15a\x0BjW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fsource sending contract\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[a\x0Bv\x85\x85\x85\x85a\x1E\x17V[PPPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xA8W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x90\x81\x17\x91\x82\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x92a\x0B\xF7\x92\x90\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C,W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x15a\x0C[W`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FG!\x12\x9E\x0Egn\xD6\xA9)\t\xBB$\xE8S\xCC\xDDc\xADr(\x0C\xC2\xE9t\xE3\x8EH\x0E\x0EnT\x90`\0\x90\xA2PV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0ET`@Qc\x07\xE0\xDB\x17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07\xE0\xDB\x17\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rIW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0BvW=`\0\x80>=`\0\xFD[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0ET`@Qc\x10\xDD\xB17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x10\xDD\xB17\x90`$\x01a\r/V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x16W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n\x81\x90U`@Q\x81\x81R\x7F\x14\x98\x824\xD3\xE5\n\x12\xAE\xEC-n\xE5\x95\xB7\x0C/\xAE\x16<\xAF\xF2\x8B\xAA\xB5\x80+\x12+\xF2\xA7S\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x11T`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0E\x97W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0E\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\xD3\x91\x90\x81\x01\x90a3\x9FV[\x90P\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\x03W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x81\x90U`@Q\x81\x81R~\x91\xF5\xF5\xDB0\x92\xE3\x9E\xCBp\x12\x18\xD4\xAF \xB7W\x1E\x04)YY7\xC34\xF3\xAC\xD1O\xE2\xFE\x90` \x01a\x0EGV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0FbW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xBE\x18\x08\x91~\t\x81O\xCA}\x80(\x9A\x97`\xA4\x06_\x10\xD6$\xAC\x1A\xEE\x0B\x9F\x90<O]\x92\xB1\x90` \x01a\x0EGV[a\xFF\xFF\x83\x16`\0\x90\x81R`\x0F` R`@\x81 \x80T\x82\x91\x90a\x0F\xD1\x90a;\x82V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xFD\x90a;\x82V[\x80\x15a\x10JW\x80`\x1F\x10a\x10\x1FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10JV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10-W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83\x83`@Qa\x10a\x92\x91\x90a8,V[`@Q\x80\x91\x03\x90 \x81\x80Q\x90` \x01 \x14\x91PP[\x93\x92PPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0ET`@QcB\xD6Z\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cB\xD6Z\x8D\x90a\x11\x0B\x90\x86\x90\x86\x90\x86\x90`\x04\x01a9\xBBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11%W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x119W=`\0\x80>=`\0\xFD[PPPPPPPV[`\x03\x80Ta\x11O\x90a;\x82V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11{\x90a;\x82V[\x80\x15a\x11\xC8W\x80`\x1F\x10a\x11\x9DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\xC8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x11\xABW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0E\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\xA0\x1Ba\xFF\xFF\x94\x90\x94\x16\x93\x90\x93\x02`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x92\x17`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\xAAW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x12\xF8W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fpeer minimal rollup fee\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[`\x0B\x81\x90U`@Q\x81\x81R\x7F\x87\x80u\xA8\xE8\xAA\x1C|\x15\xE6\x93'RR\x0Fx\x12\xBFWDx^m\xF6\x0577)\xDAAUE\x90` \x01a\x0EGV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13XW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t\x81\x90U`@Q\x81\x81R\x7F\x88\xD2\x10\xDF\xA1\x98\xF7Q\x95y)G!\xF9\x0Cw\x11S\xA7\xB4\x91\x01\xEE\xFB\x95\x14p7\xCC\x8C\xE4\x81\x90` \x01a\x0EGV[30\x14a\x13\xADW`@Qcq\xF5\x0E\xC1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\xB9\x84\x84\x84\x84a\x1ExV[PPPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[a\x14#`\0a\x1E\xB3V[V[`\x0F` R`\0\x90\x81R`@\x90 \x80Ta\x11O\x90a;\x82V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14iW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x02\x17\x90U\x81Qa\x14\xB6\x90`\x03\x90` \x85\x01\x90a1!V[P`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPV[`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x15\x06W`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T\x81Q\x10\x15a\x15*W`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T\x81Q\x11\x15a\x15NW`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T\x81`\xA0\x01Q\x10\x15a\x15uW`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT\x81`\xC0\x01Q\x10\x15a\x15\x9CW`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT\x81`\xE0\x01Q\x10\x15a\x15\xC3W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x15\xDC\x82`@\x01Q\x83`\0\x01Q\x84``\x01Qa\x1F\x05V[\x90P\x80\x82` \x01Q\x14a\x16\x02W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\x0B3a \"V[\x15a\x16)W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`\xC0\x84\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x83\x01Q``\x82\x01R`\x80\x80\x84\x01Q\x90\x82\x01R`\0a\x16l\x82a \xBFV[\x90Pa\x16|\x84`\xA0\x01Q\x82a!.V[`\x02T`\xE0\x85\x01Q`\xC0\x86\x01Q\x86Qa\x16\xB7\x93`\x01`\x01`\xA0\x1B\x03\x16\x92\x91a\x16\xA3\x91a;0V[a\x16\xAD\x91\x90a;0V[\x86`\xA0\x01Qa!`V[` \x84\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\x16W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x11T`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1<\xE5g\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x17}W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x17\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD3\x91\x90a7\xB4V[`\x11T`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0E\x97W`\0\x80\xFD[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0ET`@Qc2\xFBb\xE7`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCB\xED\x8B\x9C\x90a\x18\x8C\x90\x88\x90\x88\x90\x88\x90\x88\x90\x88\x90`\x04\x01a:\x7FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\xBAW=`\0\x80>=`\0\xFD[PPPPPPPPPV[a\xFF\xFF\x84\x16`\0\x90\x81R`\x10` R`@\x80\x82 \x90Qa\x18\xE6\x90\x86\x90a8<V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R\x92R\x90 T\x90P\x80a\x19,W`@Qc+\x96\xC9\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q` \x83\x01 \x81\x14a\x19\x82W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x07`$\x82\x01R\x7Fpayload\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[a\xFF\xFF\x85\x16`\0\x90\x81R`\x10` R`@\x80\x82 \x90Qa\x19\xA3\x90\x87\x90a8<V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\0\x90\x81R\x92R\x90 Ua\x0Bv\x85\x85\x85\x85a\x1ExV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90\x81\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x91a\x0B\xF7\x91`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1AxW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xC5W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x1A\xE6W`@Qc`\x03\xE8!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x82\x90U`\x06\x81\x90U`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x7Fv3\0Lz\"\x98i\xAE\xEA\x10\xDBO\xF3\xE5~;\x154\xAE\xB2\xC9\xE7,]\xB2_\x96X\x95\xC30\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BXW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T\x82\x15\x15`\x01`\xA0\x1B\x02`\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x90U`@Q\x7F\xCD\x16,o\xC2B\x85\xBF\xBE9\x9E\xC0\xCC,\xE2\xC3\x80\xAD'\xD2\xEB\x1F\xCAA\x8Ce-\xF1%~~\t\x90a\x0EG\x90\x83\x15\x15\x81R` \x01\x90V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x0E\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xB0\x1Ba\xFF\xFF\x86\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`\0\x90\x81R`\x0F` R`@\x90 a\x1CR\x90\x83\x83a1\xA5V[P\x7F\xFAAHz\xD5\xD6r\x8F\x0B\x19'o\xA1\xED\xDC\x16U\x85x\xF5\x10\x9F\xC3\x9D-\xC3<20G\r\xAB\x83\x83\x83`@Qa\x1C\x86\x93\x92\x91\x90a9\xBBV[`@Q\x80\x91\x03\x90\xA1PPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\nVV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1DiW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nVV[a\x1Dr\x81a\x1E\xB3V[PV[`\x0ET`@Qc={/o`\xE2\x1B\x81Ra\xFF\xFF\x80\x87\x16`\x04\x83\x01R\x85\x16`$\x82\x01R0`D\x82\x01R`d\x81\x01\x83\x90R``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF5\xEC\xBD\xBC\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1D\xD2W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1D\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\x0E\x91\x90\x81\x01\x90a3\x9FV[\x95\x94PPPPPV[`@Qc3V\xAEE`\xE1\x1B\x81R0\x90cf\xAD\\\x8A\x90a\x1E@\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a:@V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1EZW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1EnW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0a\x1E\x83\x82a!\xCCV[`\x02T`\x04T\x91\x92Pa\x0Bv\x91`\x01`\xA0\x1B\x90\x91\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90`\x01`\x01`\xA0\x1B\x03\x162\x84a\"\x94V[`\r\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x1FHW`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x1FzW`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x1F\xD2\x91`\x04\x01a8\xF2V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1F\xEAW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1F\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x0E\x91\x90a7\x03V[`\0\x80T`\x01`\xA0\x1B\x90\x04`\xFF\x16a <WP`\0\x91\x90PV[`\0T`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a \x81W`\0\x80\xFD[PZ\xFA\x15\x80\x15a \x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xB9\x91\x90a3\x82V[\x92\x91PPV[``\x80a \xCF\x83`\0\x01Qa#IV[a \xDC\x84` \x01Qa#IV[a \xE9\x85`@\x01Qa#IV[a \xF6\x86``\x01Qa#IV[a!\x03\x87`\x80\x01Qa#\xE1V[`@Q` \x01a!\x17\x95\x94\x93\x92\x91\x90a8\x87V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[a!\\`\x0E`\x16\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16\x823`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x87a$\x18V[PPV[\x804\x14a!\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Fbridge fee mismatch\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[`\x11Ta!\xC7\x90`\x01`\x01`\xA0\x1B\x03\x163\x85\x85a%OV[PPPV[a!\xFE`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\"0`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\"<\x84\x82a%\xBEV[\x90\x83R\x90Pa\"K\x84\x82a%\xBEV[` \x84\x01\x91\x90\x91R\x90Pa\"_\x84\x82a%\xBEV[`@\x84\x01\x91\x90\x91R\x90Pa\"s\x84\x82a%\xBEV[``\x84\x01\x91\x90\x91R\x90Pa\"\x87\x84\x82a&\xEFV[P`\x80\x83\x01RP\x92\x91PPV[`\x04T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\"\xC2W`@Qc\x14@\xE0y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xA0\x1B\x90\x92\x04\x16\x14a\"\xF8W`@Qc8v0O`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qa#\x17W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`@Qcx\xD6\x0C\xD7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx\xD6\x0C\xD7\x90a\x1E@\x90\x84\x90\x86\x90`\x04\x01a9^V[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a#\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a#\xD1W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a#\xAFV[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a#\xEF\x81a'\xFCV[\x83`@Q` \x01a$\x01\x92\x91\x90a8XV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[a\xFF\xFF\x86\x16`\0\x90\x81R`\x0F` R`@\x81 \x80Ta$6\x90a;\x82V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$b\x90a;\x82V[\x80\x15a$\xAFW\x80`\x1F\x10a$\x84Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$\xAFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\x92W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q`\0\x14\x15a$\xD9W`@Qc\x02\x0B5\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0ET`@Qb\xC5\x801`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5\x801\0\x90\x84\x90a%\x14\x90\x8B\x90\x86\x90\x8C\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a9\xD9V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a%-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a%AW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x13\xB9\x90\x85\x90a(\xC9V[`\0\x80\x83Q\x83` a%\xD0\x91\x90a;0V[\x11\x15\x80\x15a%\xE7WPa%\xE4\x83` a;0V[\x83\x10[a&?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\nVV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a&tW\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa&TV[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a&\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nVV[\x80a&\xE2\x85` a;0V[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a&\xFE\x85\x85a)\xAEV[\x86Q\x90\x95P\x90\x91Pa'\x10\x82\x86a;0V[\x11\x15\x80\x15a'&WPa'#\x81\x85a;0V[\x84\x10[a'~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\nVV[``\x81\x15\x80\x15a'\x99W`@Q\x91P` \x82\x01`@Ra'\xE3V[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a'\xD2W\x80Q\x83R` \x92\x83\x01\x92\x01a'\xBAV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a'\xEF\x83\x87a;0V[\x93P\x93PPP\x92P\x92\x90PV[```\xFD\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a(/W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91Ra \xB9V[a\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a(\x7FWa(O`\xFD`\xF8\x1Ba+\xB4V[a(X\x83a+\xDBV[`@Q` \x01a(i\x92\x91\x90a8XV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a(\xAAWa(\xA1`\x7F`\xF9\x1Ba+\xB4V[a(X\x83a,\x1EV[a(\xBB`\x01`\x01`\xF8\x1B\x03\x19a+\xB4V[a(X\x83a,aV[\x91\x90PV[`\0a)\x1E\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a,\xA4\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a!\xC7W\x80\x80` \x01\x90Q\x81\x01\x90a)<\x91\x90a3\x82V[a!\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nVV[`\0\x80`\0a)\xBD\x85\x85a,\xBBV[\x94P\x90P`\0`\xFD`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a*VWa)\xE3\x86\x86a-CV[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a)\xFEWPa\xFF\xFF\x81\x11\x15[a*JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\nVV[\x92P\x83\x91Pa&\xE8\x90PV[`\x7F`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a*\xE1Wa*v\x86\x86a-\xFCV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a*\x95WPc\xFF\xFF\xFF\xFF\x81\x11\x15[a*JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\nVV[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x14\x15a+^Wa*\xFD\x86\x86a.\xCEV[\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a*JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\nVV[P`\xF8\x81\x90\x1C`\xFD\x81\x10a*JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\nVV[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a \xB9V[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a,\x0EW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a+\xECV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a,QW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a,/V[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a,\x94W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a,rV[PPP`(\x81\x01`@R\x92\x91PPV[``a,\xB3\x84\x84`\0\x85a/\xA0V[\x94\x93PPPPV[`\0\x80\x83Q\x83`\x01a,\xCD\x91\x90a;0V[\x11\x15\x80\x15a,\xE4WPa,\xE1\x83`\x01a;0V[\x83\x10[a-0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\nVV[\x83\x83\x01` \x01Q\x80a&\xE2\x85`\x01a;0V[`\0\x80\x83Q\x83`\x02a-U\x91\x90a;0V[\x11\x15\x80\x15a-lWPa-i\x83`\x02a;0V[\x83\x10[a-\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\nVV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a&\xE2\x91\x90a;0V[`\0\x80\x83Q\x83`\x04a.\x0E\x91\x90a;0V[\x11\x15\x80\x15a.%WPa.\"\x83`\x04a;0V[\x83\x10[a.|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\nVV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a.\xB1W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa.\x91V[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a&\xE2\x85`\x04a;0V[`\0\x80\x83Q\x83`\x08a.\xE0\x91\x90a;0V[\x11\x15\x80\x15a.\xF7WPa.\xF4\x83`\x08a;0V[\x83\x10[a/NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\nVV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a/\x83W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa/cV[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a&\xE2\x85`\x08a;0V[``\x82G\x10\x15a0\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nVV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a0oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\nVV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa0\x8B\x91\x90a8<V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a0\xC8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a0\xCDV[``\x91P[P\x91P\x91Pa0\xDD\x82\x82\x86a0\xE8V[\x97\x96PPPPPPPV[``\x83\x15a0\xF7WP\x81a\x10vV[\x82Q\x15a1\x07W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nV\x91\x90a9#V[\x82\x80Ta1-\x90a;\x82V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a1OW`\0\x85Ua1\x95V[\x82`\x1F\x10a1hW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua1\x95V[\x82\x80\x01`\x01\x01\x85U\x82\x15a1\x95W\x91\x82\x01[\x82\x81\x11\x15a1\x95W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a1zV[Pa1\xA1\x92\x91Pa2\x19V[P\x90V[\x82\x80Ta1\xB1\x90a;\x82V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a1\xD3W`\0\x85Ua1\x95V[\x82`\x1F\x10a1\xECW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua1\x95V[\x82\x80\x01`\x01\x01\x85U\x82\x15a1\x95W\x91\x82\x01[\x82\x81\x11\x15a1\x95W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a1\xFEV[[\x80\x82\x11\x15a1\xA1W`\0\x81U`\x01\x01a2\x1AV[`\0a2Aa2<\x84a;\x08V[a:\xD7V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a2UW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0a2za2<\x84a;\x08V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a2\x8EW`\0\x80\xFD[a\x10v\x83` \x83\x01\x84a;VV[`\0\x80\x83`\x1F\x84\x01\x12a2\xAEW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xC6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a&\xE8W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a2\xEFW`\0\x80\xFD[a\x10v\x83\x835` \x85\x01a2.V[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(\xC4W`\0\x80\xFD[\x805a\xFF\xFF\x81\x16\x81\x14a(\xC4W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(\xC4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a3ZW`\0\x80\xFD[\x815a\x10v\x81a;\xD3V[`\0` \x82\x84\x03\x12\x15a3wW`\0\x80\xFD[\x815a\x10v\x81a;\xE8V[`\0` \x82\x84\x03\x12\x15a3\x94W`\0\x80\xFD[\x81Qa\x10v\x81a;\xE8V[`\0` \x82\x84\x03\x12\x15a3\xB1W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\xC8W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a3\xD9W`\0\x80\xFD[a,\xB3\x84\x82Q` \x84\x01a2lV[`\0` \x82\x84\x03\x12\x15a3\xFAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a4\x12W`\0\x80\xFD[\x90\x83\x01\x90a\x01\0\x82\x86\x03\x12\x15a4'W`\0\x80\xFD[a4/a:\xADV[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01Ra4S``\x84\x01a2\xFEV[``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a4jW`\0\x80\xFD[a4v\x87\x82\x86\x01a2\xDEV[`\x80\x83\x01RP`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015`\xE0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a4\xB8W`\0\x80\xFD[a\x10v\x82a3\x1EV[`\0\x80`@\x83\x85\x03\x12\x15a4\xD4W`\0\x80\xFD[a4\xDD\x83a3\x1EV[\x91P` \x83\x015a4\xED\x81a;\xD3V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a5\rW`\0\x80\xFD[a5\x16\x84a3\x1EV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a52W`\0\x80\xFD[a5>\x86\x82\x87\x01a2\x9CV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a5`W`\0\x80\xFD[a5i\x84a3\x1EV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\x85W`\0\x80\xFD[a5\x91\x86\x82\x87\x01a2\xDEV[\x92PPa5\xA0`@\x85\x01a30V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a5\xBFW`\0\x80\xFD[a5\xC8\x85a3\x1EV[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a5\xE5W`\0\x80\xFD[a5\xF1\x88\x83\x89\x01a2\xDEV[\x94Pa5\xFF`@\x88\x01a30V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a6\x15W`\0\x80\xFD[Pa6\"\x87\x82\x88\x01a2\xDEV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a6DW`\0\x80\xFD[a6M\x85a3\x1EV[\x93Pa6[` \x86\x01a3\x1EV[\x92P`@\x85\x015a6k\x81a;\xD3V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a6\x93W`\0\x80\xFD[a6\x9C\x86a3\x1EV[\x94Pa6\xAA` \x87\x01a3\x1EV[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xCDW`\0\x80\xFD[a6\xD9\x88\x82\x89\x01a2\x9CV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a6\xFCW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a7\x15W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a7/W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a7SW`\0\x80\xFD[a7\\\x84a30V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7xW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a7\x89W`\0\x80\xFD[a7\x98\x86\x825` \x84\x01a2.V[\x92PP`@\x84\x015a7\xA9\x81a;\xD3V[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a7\xC6W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x10vW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x81Q\x80\x84Ra8\x18\x81` \x86\x01` \x86\x01a;VV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82Qa8N\x81\x84` \x87\x01a;VV[\x91\x90\x91\x01\x92\x91PPV[`\0\x83Qa8j\x81\x84` \x88\x01a;VV[\x83Q\x90\x83\x01\x90a8~\x81\x83` \x88\x01a;VV[\x01\x94\x93PPPPV[`\0\x86Qa8\x99\x81\x84` \x8B\x01a;VV[\x86Q\x90\x83\x01\x90a8\xAD\x81\x83` \x8B\x01a;VV[\x86Q\x91\x01\x90a8\xC0\x81\x83` \x8A\x01a;VV[\x85Q\x91\x01\x90a8\xD3\x81\x83` \x89\x01a;VV[\x84Q\x91\x01\x90a8\xE6\x81\x83` \x88\x01a;VV[\x01\x97\x96PPPPPPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a9\x1AW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a8\xFBV[PPP\x92\x91PPV[` \x81R`\0a\x10v` \x83\x01\x84a8\0V[` \x81\x01`\x02\x83\x10a9XWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra9\xA2`\xE0\x84\x01\x82a8\0V[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[a\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x1E\x0E`@\x83\x01\x84\x86a7\xD7V[a\xFF\xFF\x87\x16\x81R`\xC0` \x82\x01R`\0a9\xF6`\xC0\x83\x01\x88a8\0V[\x82\x81\x03`@\x84\x01Ra:\x08\x81\x88a8\0V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16``\x86\x01R\x86\x16`\x80\x85\x01R\x83\x81\x03`\xA0\x85\x01R\x90Pa:3\x81\x85a8\0V[\x99\x98PPPPPPPPPV[a\xFF\xFF\x85\x16\x81R`\x80` \x82\x01R`\0a:]`\x80\x83\x01\x86a8\0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra0\xDD\x81\x85a8\0V[`\0a\xFF\xFF\x80\x88\x16\x83R\x80\x87\x16` \x84\x01RP\x84`@\x83\x01R`\x80``\x83\x01Ra0\xDD`\x80\x83\x01\x84\x86a7\xD7V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a:\xD1Wa:\xD1a;\xBDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;\0Wa;\0a;\xBDV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a;\"Wa;\"a;\xBDV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82\x19\x82\x11\x15a;QWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[`\0[\x83\x81\x10\x15a;qW\x81\x81\x01Q\x83\x82\x01R` \x01a;YV[\x83\x81\x11\x15a\x13\xB9WPP`\0\x91\x01RV[`\x01\x81\x81\x1C\x90\x82\x16\x80a;\x96W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a;\xB7WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1DrW`\0\x80\xFD[\x80\x15\x15\x81\x14a\x1DrW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xF1\x8F@\x84\xAD\xC5\xF2\xCB\xE1\xE7#n\x87\xC6\x8D\xE1]\xF2\xB2O\xC0\x19<\xA8\xED\x08\x07\xED\xAAf\xC2\xAFdsolcC\0\x08\x07\x003";
    /// The deployed bytecode of the contract.
    pub static MYSTIKOV2LAYERZEROERC20_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MystikoV2LayerZeroERC20<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoV2LayerZeroERC20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoV2LayerZeroERC20<M> {
        type Target = ::ethers_contract::Contract<M>;
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
    impl<M: ::ethers_providers::Middleware> MystikoV2LayerZeroERC20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers_contract::Contract::new(
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
        pub fn deploy<T: ::ethers_core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers_contract::builders::ContractDeployer<M, Self>,
            ::ethers_contract::ContractError<M>,
        > {
            let factory = ::ethers_contract::ContractFactory::new(
                MYSTIKOV2LAYERZEROERC20_ABI.clone(),
                MYSTIKOV2LAYERZEROERC20_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `assetDecimals` (0xc2d41601) function
        pub fn asset_decimals(&self) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([194, 212, 22, 1], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetName` (0xc9230c5d) function
        pub fn asset_name(&self) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([201, 35, 12, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetSymbol` (0x176de7a8) function
        pub fn asset_symbol(&self) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
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
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([44, 210, 109, 69], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeType` (0x2421e155) function
        pub fn bridge_type(&self) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([36, 33, 225, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeOperator` (0x06394c9b) function
        pub fn change_operator(
            &self,
            new_operator: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 57, 76, 155], new_operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x9a03636c) function
        pub fn deposit(&self, request: DepositRequest) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 3, 99, 108], (request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableSanctionsCheck` (0xdd757c34) function
        pub fn disable_sanctions_check(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 117, 124, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableSanctionsCheck` (0x01dbf19f) function
        pub fn enable_sanctions_check(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 219, 241, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedMessages` (0x5b8c41e6) function
        pub fn failed_messages(
            &self,
            p0: u16,
            p1: ::ethers_core::types::Bytes,
            p2: u64,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([91, 140, 65, 230], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forceResumeReceive` (0x42d65a8d) function
        pub fn force_resume_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 214, 90, 141], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAssociatedCommitmentPool` (0xddac5dc1) function
        pub fn get_associated_commitment_pool(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([221, 172, 93, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfig` (0xf5ecbdbc) function
        pub fn get_config(
            &self,
            version: u16,
            chain_id: u16,
            p2: ::ethers_core::types::Address,
            config_type: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Bytes> {
            self.0
                .method_hash([245, 236, 189, 188], (version, chain_id, p2, config_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxAmount` (0x0ba95909) function
        pub fn get_max_amount(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([11, 169, 89, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinAmount` (0xcfc7e2da) function
        pub fn get_min_amount(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([207, 199, 226, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinBridgeFee` (0xefbfb2ae) function
        pub fn get_min_bridge_fee(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([239, 191, 178, 174], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinExecutorFee` (0xf4ad17c6) function
        pub fn get_min_executor_fee(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([244, 173, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPeerMinExecutorFee` (0x5898a0a8) function
        pub fn get_peer_min_executor_fee(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([88, 152, 160, 168], ())
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
        pub fn is_deposits_disabled(&self) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([237, 110, 163, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isTrustedRemote` (0x3d8b38f6) function
        pub fn is_trusted_remote(
            &self,
            src_chain_id: u16,
            src_address: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 139, 56, 246], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `localLayerZeroChainId` (0x302d5f4b) function
        pub fn local_layer_zero_chain_id(&self) -> ::ethers_contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([48, 45, 95, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lzEndpoint` (0xb353aaa7) function
        pub fn lz_endpoint(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([179, 83, 170, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lzReceive` (0x001d3567) function
        pub fn lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers_core::types::Bytes,
            nonce: u64,
            payload: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 29, 53, 103], (src_chain_id, src_address, nonce, payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonblockingLzReceive` (0x66ad5c8a) function
        pub fn nonblocking_lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers_core::types::Bytes,
            nonce: u64,
            payload: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 173, 92, 138], (src_chain_id, src_address, nonce, payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerChainId` (0xcdfceeba) function
        pub fn peer_chain_id(&self) -> ::ethers_contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([205, 252, 238, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerChainName` (0x4e3c10b7) function
        pub fn peer_chain_name(&self) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([78, 60, 16, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerContract` (0x21e32d55) function
        pub fn peer_contract(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([33, 227, 45, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peerLayerZeroChainId` (0x0097a063) function
        pub fn peer_layer_zero_chain_id(&self) -> ::ethers_contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([0, 151, 160, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `retryMessage` (0xd1deba1f) function
        pub fn retry_message(
            &self,
            src_chain_id: u16,
            src_address: ::ethers_core::types::Bytes,
            nonce: u64,
            payload: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 222, 186, 31], (src_chain_id, src_address, nonce, payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sanctionsCheck` (0xb1c39422) function
        pub fn sanctions_check(&self) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 195, 148, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sanctionsList` (0xec571c6a) function
        pub fn sanctions_list(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([236, 87, 28, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAssociatedCommitmentPool` (0xe19abef8) function
        pub fn set_associated_commitment_pool(
            &self,
            commitment_pool_address: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 154, 190, 248], commitment_pool_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBridgeProxyAddress` (0xa3bc64f2) function
        pub fn set_bridge_proxy_address(
            &self,
            bridge_proxy_address: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 188, 100, 242], bridge_proxy_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0xcbed8b9c) function
        pub fn set_config(
            &self,
            version: u16,
            chain_id: u16,
            config_type: ::ethers_core::types::U256,
            config: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 237, 139, 156], (version, chain_id, config_type, config))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDepositsDisabled` (0xea0cde85) function
        pub fn set_deposits_disabled(&self, state: bool) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 12, 222, 133], state)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEndpoint` (0x4ee7ded6) function
        pub fn set_endpoint(
            &self,
            lz_chain_id: u16,
            lz_endpoint: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 231, 222, 214], (lz_chain_id, lz_endpoint))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinBridgeFee` (0x19e75d6e) function
        pub fn set_min_bridge_fee(
            &self,
            min_bridge_fee: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 231, 93, 110], min_bridge_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinExecutorFee` (0x5e10b2b7) function
        pub fn set_min_executor_fee(
            &self,
            min_executor_fee: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 16, 178, 183], min_executor_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPeerContract` (0x7d2c8520) function
        pub fn set_peer_contract(
            &self,
            peer_chain_id: u64,
            peer_chain_name: ::std::string::String,
            peer_contract: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 44, 133, 32], (peer_chain_id, peer_chain_name, peer_contract))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPeerMinExecutorFee` (0x153dc450) function
        pub fn set_peer_min_executor_fee(
            &self,
            peer_min_executor_fee: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 61, 196, 80], peer_min_executor_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPeerMinRollupFee` (0x521ff057) function
        pub fn set_peer_min_rollup_fee(
            &self,
            peer_min_rollup_fee: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 31, 240, 87], peer_min_rollup_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReceiveVersion` (0x10ddb137) function
        pub fn set_receive_version(&self, version: u16) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 221, 177, 55], version)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSendVersion` (0x07e0db17) function
        pub fn set_send_version(&self, version: u16) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 224, 219, 23], version)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTrustedRemote` (0xeb8d72b7) function
        pub fn set_trusted_remote(
            &self,
            peer_layer_zero_chain_id: u16,
            peer_address: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 141, 114, 183], (peer_layer_zero_chain_id, peer_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trustedRemoteLookup` (0x7533d788) function
        pub fn trusted_remote_lookup(
            &self,
            p0: u16,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Bytes> {
            self.0
                .method_hash([117, 51, 215, 136], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateDepositAmountLimits` (0xe8183c44) function
        pub fn update_deposit_amount_limits(
            &self,
            max_amount: ::ethers_core::types::U256,
            min_amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 24, 60, 68], (max_amount, min_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateSanctionsListAddress` (0x30f49cac) function
        pub fn update_sanctions_list_address(
            &self,
            sanction: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 244, 156, 172], sanction)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CommitmentCrossChain` event
        pub fn commitment_cross_chain_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, CommitmentCrossChainFilter> {
            self.0.event()
        }
        ///Gets the contract's `DepositAmountLimits` event
        pub fn deposit_amount_limits_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, DepositAmountLimitsFilter> {
            self.0.event()
        }
        ///Gets the contract's `DepositsDisabled` event
        pub fn deposits_disabled_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, DepositsDisabledFilter> {
            self.0.event()
        }
        ///Gets the contract's `MessageFailed` event
        pub fn message_failed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, MessageFailedFilter> {
            self.0.event()
        }
        ///Gets the contract's `MinBridgeFee` event
        pub fn min_bridge_fee_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, MinBridgeFeeFilter> {
            self.0.event()
        }
        ///Gets the contract's `MinExecutorFee` event
        pub fn min_executor_fee_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, MinExecutorFeeFilter> {
            self.0.event()
        }
        ///Gets the contract's `OperatorChanged` event
        pub fn operator_changed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, OperatorChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter> {
            self.0.event()
        }
        ///Gets the contract's `PeerMinExecutorFee` event
        pub fn peer_min_executor_fee_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, PeerMinExecutorFeeFilter> {
            self.0.event()
        }
        ///Gets the contract's `PeerMinRollupFee` event
        pub fn peer_min_rollup_fee_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, PeerMinRollupFeeFilter> {
            self.0.event()
        }
        ///Gets the contract's `SanctionsCheck` event
        pub fn sanctions_check_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, SanctionsCheckFilter> {
            self.0.event()
        }
        ///Gets the contract's `SanctionsList` event
        pub fn sanctions_list_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, SanctionsListFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetTrustedRemote` event
        pub fn set_trusted_remote_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, SetTrustedRemoteFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, MystikoV2LayerZeroERC20Events> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for MystikoV2LayerZeroERC20<M> {
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[etherror(name = "AmountTooSmall", abi = "AmountTooSmall()")]
    pub struct AmountTooSmall;
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
        Hash,
    )]
    #[etherror(name = "BridgeFeeTooFew", abi = "BridgeFeeTooFew()")]
    pub struct BridgeFeeTooFew;
    ///Custom Error type `CallIsNotLzApp` with signature `CallIsNotLzApp()` and selector `0xe3ea1d82`
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
        Hash,
    )]
    #[etherror(name = "CallIsNotLzApp", abi = "CallIsNotLzApp()")]
    pub struct CallIsNotLzApp;
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
        Hash,
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
        Hash,
    )]
    #[etherror(name = "DepositsDisabled", abi = "DepositsDisabled()")]
    pub struct DepositsDisabled;
    ///Custom Error type `DestinationChainIsNotTrusted` with signature `DestinationChainIsNotTrusted()` and selector `0x020b35a1`
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
        Hash,
    )]
    #[etherror(name = "DestinationChainIsNotTrusted", abi = "DestinationChainIsNotTrusted()")]
    pub struct DestinationChainIsNotTrusted;
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
        Hash,
    )]
    #[etherror(name = "ExecutorFeeTooFew", abi = "ExecutorFeeTooFew()")]
    pub struct ExecutorFeeTooFew;
    ///Custom Error type `FromChainIdNotMatched` with signature `FromChainIdNotMatched()` and selector `0xe1d8c13c`
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
        Hash,
    )]
    #[etherror(name = "FromChainIdNotMatched", abi = "FromChainIdNotMatched()")]
    pub struct FromChainIdNotMatched;
    ///Custom Error type `FromProxyAddressNotMatched` with signature `FromProxyAddressNotMatched()` and selector `0x2881c0f2`
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
        Hash,
    )]
    #[etherror(name = "FromProxyAddressNotMatched", abi = "FromProxyAddressNotMatched()")]
    pub struct FromProxyAddressNotMatched;
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
        Hash,
    )]
    #[etherror(name = "HashKGreaterThanFieldSize", abi = "HashKGreaterThanFieldSize()")]
    pub struct HashKGreaterThanFieldSize;
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
        Hash,
    )]
    #[etherror(name = "Invalid", abi = "Invalid(string)")]
    pub struct Invalid {
        pub param: ::std::string::String,
    }
    ///Custom Error type `MinAmountGreaterThanMaxAmount` with signature `MinAmountGreaterThanMaxAmount()` and selector `0xc007d042`
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
        Hash,
    )]
    #[etherror(name = "MinAmountGreaterThanMaxAmount", abi = "MinAmountGreaterThanMaxAmount()")]
    pub struct MinAmountGreaterThanMaxAmount;
    ///Custom Error type `NoStoredMessage` with signature `NoStoredMessage()` and selector `0xae5b2614`
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
        Hash,
    )]
    #[etherror(name = "NoStoredMessage", abi = "NoStoredMessage()")]
    pub struct NoStoredMessage;
    ///Custom Error type `NotChanged` with signature `NotChanged()` and selector `0x36a1c33f`
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
        Hash,
    )]
    #[etherror(name = "NotChanged", abi = "NotChanged()")]
    pub struct NotChanged;
    ///Custom Error type `OnlyOperator` with signature `OnlyOperator()` and selector `0x27e1f1e5`
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
        Hash,
    )]
    #[etherror(name = "OnlyOperator", abi = "OnlyOperator()")]
    pub struct OnlyOperator;
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
        Hash,
    )]
    #[etherror(name = "RandomSGreaterThanFieldSize", abi = "RandomSGreaterThanFieldSize()")]
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
        Hash,
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
        Hash,
    )]
    #[etherror(name = "SanctionedAddress", abi = "SanctionedAddress()")]
    pub struct SanctionedAddress;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers_contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoV2LayerZeroERC20Errors {
        AmountLessThanZero(AmountLessThanZero),
        AmountTooLarge(AmountTooLarge),
        AmountTooSmall(AmountTooSmall),
        BridgeFeeTooFew(BridgeFeeTooFew),
        CallIsNotLzApp(CallIsNotLzApp),
        CommitmentHashIncorrect(CommitmentHashIncorrect),
        DepositsDisabled(DepositsDisabled),
        DestinationChainIsNotTrusted(DestinationChainIsNotTrusted),
        ExecutorFeeTooFew(ExecutorFeeTooFew),
        FromChainIdNotMatched(FromChainIdNotMatched),
        FromProxyAddressNotMatched(FromProxyAddressNotMatched),
        HashKGreaterThanFieldSize(HashKGreaterThanFieldSize),
        Invalid(Invalid),
        MinAmountGreaterThanMaxAmount(MinAmountGreaterThanMaxAmount),
        NoStoredMessage(NoStoredMessage),
        NotChanged(NotChanged),
        OnlyOperator(OnlyOperator),
        RandomSGreaterThanFieldSize(RandomSGreaterThanFieldSize),
        RollupFeeToFew(RollupFeeToFew),
        SanctionedAddress(SanctionedAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoV2LayerZeroERC20Errors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AmountLessThanZero as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountLessThanZero(decoded));
            }
            if let Ok(decoded) = <AmountTooLarge as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountTooLarge(decoded));
            }
            if let Ok(decoded) = <AmountTooSmall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountTooSmall(decoded));
            }
            if let Ok(decoded) = <BridgeFeeTooFew as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BridgeFeeTooFew(decoded));
            }
            if let Ok(decoded) = <CallIsNotLzApp as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallIsNotLzApp(decoded));
            }
            if let Ok(decoded) = <CommitmentHashIncorrect as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CommitmentHashIncorrect(decoded));
            }
            if let Ok(decoded) = <DepositsDisabled as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositsDisabled(decoded));
            }
            if let Ok(decoded) = <DestinationChainIsNotTrusted as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DestinationChainIsNotTrusted(decoded));
            }
            if let Ok(decoded) = <ExecutorFeeTooFew as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecutorFeeTooFew(decoded));
            }
            if let Ok(decoded) = <FromChainIdNotMatched as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FromChainIdNotMatched(decoded));
            }
            if let Ok(decoded) = <FromProxyAddressNotMatched as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FromProxyAddressNotMatched(decoded));
            }
            if let Ok(decoded) = <HashKGreaterThanFieldSize as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashKGreaterThanFieldSize(decoded));
            }
            if let Ok(decoded) = <Invalid as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Invalid(decoded));
            }
            if let Ok(decoded) = <MinAmountGreaterThanMaxAmount as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinAmountGreaterThanMaxAmount(decoded));
            }
            if let Ok(decoded) = <NoStoredMessage as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoStoredMessage(decoded));
            }
            if let Ok(decoded) = <NotChanged as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotChanged(decoded));
            }
            if let Ok(decoded) = <OnlyOperator as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyOperator(decoded));
            }
            if let Ok(decoded) = <RandomSGreaterThanFieldSize as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RandomSGreaterThanFieldSize(decoded));
            }
            if let Ok(decoded) = <RollupFeeToFew as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RollupFeeToFew(decoded));
            }
            if let Ok(decoded) = <SanctionedAddress as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionedAddress(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for MystikoV2LayerZeroERC20Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AmountLessThanZero(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AmountTooLarge(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AmountTooSmall(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::BridgeFeeTooFew(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CallIsNotLzApp(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CommitmentHashIncorrect(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DepositsDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DestinationChainIsNotTrusted(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ExecutorFeeTooFew(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::FromChainIdNotMatched(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::FromProxyAddressNotMatched(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::HashKGreaterThanFieldSize(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Invalid(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::MinAmountGreaterThanMaxAmount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NoStoredMessage(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NotChanged(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnlyOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RandomSGreaterThanFieldSize(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RollupFeeToFew(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionedAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for MystikoV2LayerZeroERC20Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AmountLessThanZero as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <AmountTooLarge as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <AmountTooSmall as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <BridgeFeeTooFew as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <CallIsNotLzApp as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <CommitmentHashIncorrect as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <DepositsDisabled as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <DestinationChainIsNotTrusted as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <ExecutorFeeTooFew as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <FromChainIdNotMatched as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <FromProxyAddressNotMatched as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <HashKGreaterThanFieldSize as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <Invalid as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <MinAmountGreaterThanMaxAmount as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <NoStoredMessage as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <NotChanged as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OnlyOperator as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <RandomSGreaterThanFieldSize as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <RollupFeeToFew as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <SanctionedAddress as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2LayerZeroERC20Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AmountLessThanZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallIsNotLzApp(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentHashIncorrect(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::DestinationChainIsNotTrusted(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutorFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::FromChainIdNotMatched(element) => ::core::fmt::Display::fmt(element, f),
                Self::FromProxyAddressNotMatched(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashKGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinAmountGreaterThanMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoStoredMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotChanged(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RandomSGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupFeeToFew(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<FromChainIdNotMatched> for MystikoV2LayerZeroERC20Errors {
        fn from(value: FromChainIdNotMatched) -> Self {
            Self::FromChainIdNotMatched(value)
        }
    }
    impl ::core::convert::From<FromProxyAddressNotMatched> for MystikoV2LayerZeroERC20Errors {
        fn from(value: FromProxyAddressNotMatched) -> Self {
            Self::FromProxyAddressNotMatched(value)
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
    impl ::core::convert::From<MinAmountGreaterThanMaxAmount> for MystikoV2LayerZeroERC20Errors {
        fn from(value: MinAmountGreaterThanMaxAmount) -> Self {
            Self::MinAmountGreaterThanMaxAmount(value)
        }
    }
    impl ::core::convert::From<NoStoredMessage> for MystikoV2LayerZeroERC20Errors {
        fn from(value: NoStoredMessage) -> Self {
            Self::NoStoredMessage(value)
        }
    }
    impl ::core::convert::From<NotChanged> for MystikoV2LayerZeroERC20Errors {
        fn from(value: NotChanged) -> Self {
            Self::NotChanged(value)
        }
    }
    impl ::core::convert::From<OnlyOperator> for MystikoV2LayerZeroERC20Errors {
        fn from(value: OnlyOperator) -> Self {
            Self::OnlyOperator(value)
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
    impl ::core::convert::From<SanctionedAddress> for MystikoV2LayerZeroERC20Errors {
        fn from(value: SanctionedAddress) -> Self {
            Self::SanctionedAddress(value)
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
        Hash,
    )]
    #[ethevent(name = "CommitmentCrossChain", abi = "CommitmentCrossChain(uint256)")]
    pub struct CommitmentCrossChainFilter {
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
        Hash,
    )]
    #[ethevent(name = "DepositAmountLimits", abi = "DepositAmountLimits(uint256,uint256)")]
    pub struct DepositAmountLimitsFilter {
        pub max_amount: ::ethers_core::types::U256,
        pub min_amount: ::ethers_core::types::U256,
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
        Hash,
    )]
    #[ethevent(name = "DepositsDisabled", abi = "DepositsDisabled(bool)")]
    pub struct DepositsDisabledFilter {
        pub state: bool,
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
        Hash,
    )]
    #[ethevent(name = "MessageFailed", abi = "MessageFailed(uint16,bytes,uint64,bytes)")]
    pub struct MessageFailedFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
        pub nonce: u64,
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
        Hash,
    )]
    #[ethevent(name = "MinBridgeFee", abi = "MinBridgeFee(uint256)")]
    pub struct MinBridgeFeeFilter {
        pub min_bridge_fee: ::ethers_core::types::U256,
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
        Hash,
    )]
    #[ethevent(name = "MinExecutorFee", abi = "MinExecutorFee(uint256)")]
    pub struct MinExecutorFeeFilter {
        pub min_executor_fee: ::ethers_core::types::U256,
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
        Hash,
    )]
    #[ethevent(name = "OperatorChanged", abi = "OperatorChanged(address)")]
    pub struct OperatorChangedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers_core::types::Address,
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
        Hash,
    )]
    #[ethevent(name = "OwnershipTransferred", abi = "OwnershipTransferred(address,address)")]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers_core::types::Address,
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
        Hash,
    )]
    #[ethevent(name = "PeerMinExecutorFee", abi = "PeerMinExecutorFee(uint256)")]
    pub struct PeerMinExecutorFeeFilter {
        pub peer_min_executor_fee: ::ethers_core::types::U256,
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
        Hash,
    )]
    #[ethevent(name = "PeerMinRollupFee", abi = "PeerMinRollupFee(uint256)")]
    pub struct PeerMinRollupFeeFilter {
        pub peer_min_rollup_fee: ::ethers_core::types::U256,
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
        Hash,
    )]
    #[ethevent(name = "SanctionsCheck", abi = "SanctionsCheck(bool)")]
    pub struct SanctionsCheckFilter {
        pub state: bool,
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
        Hash,
    )]
    #[ethevent(name = "SanctionsList", abi = "SanctionsList(address)")]
    pub struct SanctionsListFilter {
        pub sanctions: ::ethers_core::types::Address,
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
        Hash,
    )]
    #[ethevent(name = "SetTrustedRemote", abi = "SetTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers_contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoV2LayerZeroERC20Events {
        CommitmentCrossChainFilter(CommitmentCrossChainFilter),
        DepositAmountLimitsFilter(DepositAmountLimitsFilter),
        DepositsDisabledFilter(DepositsDisabledFilter),
        MessageFailedFilter(MessageFailedFilter),
        MinBridgeFeeFilter(MinBridgeFeeFilter),
        MinExecutorFeeFilter(MinExecutorFeeFilter),
        OperatorChangedFilter(OperatorChangedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PeerMinExecutorFeeFilter(PeerMinExecutorFeeFilter),
        PeerMinRollupFeeFilter(PeerMinRollupFeeFilter),
        SanctionsCheckFilter(SanctionsCheckFilter),
        SanctionsListFilter(SanctionsListFilter),
        SetTrustedRemoteFilter(SetTrustedRemoteFilter),
    }
    impl ::ethers_contract::EthLogDecode for MystikoV2LayerZeroERC20Events {
        fn decode_log(log: &::ethers_core::abi::RawLog) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = CommitmentCrossChainFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::CommitmentCrossChainFilter(decoded));
            }
            if let Ok(decoded) = DepositAmountLimitsFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::DepositAmountLimitsFilter(decoded));
            }
            if let Ok(decoded) = DepositsDisabledFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::DepositsDisabledFilter(decoded));
            }
            if let Ok(decoded) = MessageFailedFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::MessageFailedFilter(decoded));
            }
            if let Ok(decoded) = MinBridgeFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::MinBridgeFeeFilter(decoded));
            }
            if let Ok(decoded) = MinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::MinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PeerMinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::PeerMinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = PeerMinRollupFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::PeerMinRollupFeeFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::SanctionsListFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedRemoteFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroERC20Events::SetTrustedRemoteFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MystikoV2LayerZeroERC20Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CommitmentCrossChainFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositAmountLimitsFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsDisabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MessageFailedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinBridgeFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinExecutorFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerMinExecutorFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerMinRollupFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsCheckFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsListFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTrustedRemoteFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CommitmentCrossChainFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: CommitmentCrossChainFilter) -> Self {
            Self::CommitmentCrossChainFilter(value)
        }
    }
    impl ::core::convert::From<DepositAmountLimitsFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: DepositAmountLimitsFilter) -> Self {
            Self::DepositAmountLimitsFilter(value)
        }
    }
    impl ::core::convert::From<DepositsDisabledFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: DepositsDisabledFilter) -> Self {
            Self::DepositsDisabledFilter(value)
        }
    }
    impl ::core::convert::From<MessageFailedFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: MessageFailedFilter) -> Self {
            Self::MessageFailedFilter(value)
        }
    }
    impl ::core::convert::From<MinBridgeFeeFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: MinBridgeFeeFilter) -> Self {
            Self::MinBridgeFeeFilter(value)
        }
    }
    impl ::core::convert::From<MinExecutorFeeFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: MinExecutorFeeFilter) -> Self {
            Self::MinExecutorFeeFilter(value)
        }
    }
    impl ::core::convert::From<OperatorChangedFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: OperatorChangedFilter) -> Self {
            Self::OperatorChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PeerMinExecutorFeeFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: PeerMinExecutorFeeFilter) -> Self {
            Self::PeerMinExecutorFeeFilter(value)
        }
    }
    impl ::core::convert::From<PeerMinRollupFeeFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: PeerMinRollupFeeFilter) -> Self {
            Self::PeerMinRollupFeeFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: SanctionsCheckFilter) -> Self {
            Self::SanctionsCheckFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsListFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: SanctionsListFilter) -> Self {
            Self::SanctionsListFilter(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteFilter> for MystikoV2LayerZeroERC20Events {
        fn from(value: SetTrustedRemoteFilter) -> Self {
            Self::SetTrustedRemoteFilter(value)
        }
    }
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "bridgeType", abi = "bridgeType()")]
    pub struct BridgeTypeCall;
    ///Container type for all input parameters for the `changeOperator` function with signature `changeOperator(address)` and selector `0x06394c9b`
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
        Hash,
    )]
    #[ethcall(name = "changeOperator", abi = "changeOperator(address)")]
    pub struct ChangeOperatorCall {
        pub new_operator: ::ethers_core::types::Address,
    }
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
        Hash,
    )]
    #[ethcall(
        name = "deposit",
        abi = "deposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256))"
    )]
    pub struct DepositCall {
        pub request: DepositRequest,
    }
    ///Container type for all input parameters for the `disableSanctionsCheck` function with signature `disableSanctionsCheck()` and selector `0xdd757c34`
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
        Hash,
    )]
    #[ethcall(name = "disableSanctionsCheck", abi = "disableSanctionsCheck()")]
    pub struct DisableSanctionsCheckCall;
    ///Container type for all input parameters for the `enableSanctionsCheck` function with signature `enableSanctionsCheck()` and selector `0x01dbf19f`
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
        Hash,
    )]
    #[ethcall(name = "enableSanctionsCheck", abi = "enableSanctionsCheck()")]
    pub struct EnableSanctionsCheckCall;
    ///Container type for all input parameters for the `failedMessages` function with signature `failedMessages(uint16,bytes,uint64)` and selector `0x5b8c41e6`
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
        Hash,
    )]
    #[ethcall(name = "failedMessages", abi = "failedMessages(uint16,bytes,uint64)")]
    pub struct FailedMessagesCall(pub u16, pub ::ethers_core::types::Bytes, pub u64);
    ///Container type for all input parameters for the `forceResumeReceive` function with signature `forceResumeReceive(uint16,bytes)` and selector `0x42d65a8d`
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
        Hash,
    )]
    #[ethcall(name = "forceResumeReceive", abi = "forceResumeReceive(uint16,bytes)")]
    pub struct ForceResumeReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
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
        Hash,
    )]
    #[ethcall(name = "getAssociatedCommitmentPool", abi = "getAssociatedCommitmentPool()")]
    pub struct GetAssociatedCommitmentPoolCall;
    ///Container type for all input parameters for the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `0xf5ecbdbc`
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
        Hash,
    )]
    #[ethcall(name = "getConfig", abi = "getConfig(uint16,uint16,address,uint256)")]
    pub struct GetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub p2: ::ethers_core::types::Address,
        pub config_type: ::ethers_core::types::U256,
    }
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "getMinExecutorFee", abi = "getMinExecutorFee()")]
    pub struct GetMinExecutorFeeCall;
    ///Container type for all input parameters for the `getPeerMinExecutorFee` function with signature `getPeerMinExecutorFee()` and selector `0x5898a0a8`
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
        Hash,
    )]
    #[ethcall(name = "getPeerMinExecutorFee", abi = "getPeerMinExecutorFee()")]
    pub struct GetPeerMinExecutorFeeCall;
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "isDepositsDisabled", abi = "isDepositsDisabled()")]
    pub struct IsDepositsDisabledCall;
    ///Container type for all input parameters for the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `0x3d8b38f6`
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
        Hash,
    )]
    #[ethcall(name = "isTrustedRemote", abi = "isTrustedRemote(uint16,bytes)")]
    pub struct IsTrustedRemoteCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `localLayerZeroChainId` function with signature `localLayerZeroChainId()` and selector `0x302d5f4b`
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
        Hash,
    )]
    #[ethcall(name = "localLayerZeroChainId", abi = "localLayerZeroChainId()")]
    pub struct LocalLayerZeroChainIdCall;
    ///Container type for all input parameters for the `lzEndpoint` function with signature `lzEndpoint()` and selector `0xb353aaa7`
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
        Hash,
    )]
    #[ethcall(name = "lzEndpoint", abi = "lzEndpoint()")]
    pub struct LzEndpointCall;
    ///Container type for all input parameters for the `lzReceive` function with signature `lzReceive(uint16,bytes,uint64,bytes)` and selector `0x001d3567`
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
        Hash,
    )]
    #[ethcall(name = "lzReceive", abi = "lzReceive(uint16,bytes,uint64,bytes)")]
    pub struct LzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `nonblockingLzReceive` function with signature `nonblockingLzReceive(uint16,bytes,uint64,bytes)` and selector `0x66ad5c8a`
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
        Hash,
    )]
    #[ethcall(
        name = "nonblockingLzReceive",
        abi = "nonblockingLzReceive(uint16,bytes,uint64,bytes)"
    )]
    pub struct NonblockingLzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "peerContract", abi = "peerContract()")]
    pub struct PeerContractCall;
    ///Container type for all input parameters for the `peerLayerZeroChainId` function with signature `peerLayerZeroChainId()` and selector `0x0097a063`
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
        Hash,
    )]
    #[ethcall(name = "peerLayerZeroChainId", abi = "peerLayerZeroChainId()")]
    pub struct PeerLayerZeroChainIdCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
        Hash,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `retryMessage` function with signature `retryMessage(uint16,bytes,uint64,bytes)` and selector `0xd1deba1f`
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
        Hash,
    )]
    #[ethcall(name = "retryMessage", abi = "retryMessage(uint16,bytes,uint64,bytes)")]
    pub struct RetryMessageCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `sanctionsCheck` function with signature `sanctionsCheck()` and selector `0xb1c39422`
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
        Hash,
    )]
    #[ethcall(name = "sanctionsCheck", abi = "sanctionsCheck()")]
    pub struct SanctionsCheckCall;
    ///Container type for all input parameters for the `sanctionsList` function with signature `sanctionsList()` and selector `0xec571c6a`
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
        Hash,
    )]
    #[ethcall(name = "sanctionsList", abi = "sanctionsList()")]
    pub struct SanctionsListCall;
    ///Container type for all input parameters for the `setAssociatedCommitmentPool` function with signature `setAssociatedCommitmentPool(address)` and selector `0xe19abef8`
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
        Hash,
    )]
    #[ethcall(name = "setAssociatedCommitmentPool", abi = "setAssociatedCommitmentPool(address)")]
    pub struct SetAssociatedCommitmentPoolCall {
        pub commitment_pool_address: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `setBridgeProxyAddress` function with signature `setBridgeProxyAddress(address)` and selector `0xa3bc64f2`
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
        Hash,
    )]
    #[ethcall(name = "setBridgeProxyAddress", abi = "setBridgeProxyAddress(address)")]
    pub struct SetBridgeProxyAddressCall {
        pub bridge_proxy_address: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig(uint16,uint16,uint256,bytes)` and selector `0xcbed8b9c`
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
        Hash,
    )]
    #[ethcall(name = "setConfig", abi = "setConfig(uint16,uint16,uint256,bytes)")]
    pub struct SetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub config_type: ::ethers_core::types::U256,
        pub config: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `setDepositsDisabled` function with signature `setDepositsDisabled(bool)` and selector `0xea0cde85`
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
        Hash,
    )]
    #[ethcall(name = "setDepositsDisabled", abi = "setDepositsDisabled(bool)")]
    pub struct SetDepositsDisabledCall {
        pub state: bool,
    }
    ///Container type for all input parameters for the `setEndpoint` function with signature `setEndpoint(uint16,address)` and selector `0x4ee7ded6`
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
        Hash,
    )]
    #[ethcall(name = "setEndpoint", abi = "setEndpoint(uint16,address)")]
    pub struct SetEndpointCall {
        pub lz_chain_id: u16,
        pub lz_endpoint: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `setMinBridgeFee` function with signature `setMinBridgeFee(uint256)` and selector `0x19e75d6e`
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
        Hash,
    )]
    #[ethcall(name = "setMinBridgeFee", abi = "setMinBridgeFee(uint256)")]
    pub struct SetMinBridgeFeeCall {
        pub min_bridge_fee: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `setMinExecutorFee` function with signature `setMinExecutorFee(uint256)` and selector `0x5e10b2b7`
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
        Hash,
    )]
    #[ethcall(name = "setMinExecutorFee", abi = "setMinExecutorFee(uint256)")]
    pub struct SetMinExecutorFeeCall {
        pub min_executor_fee: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `setPeerContract` function with signature `setPeerContract(uint64,string,address)` and selector `0x7d2c8520`
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
        Hash,
    )]
    #[ethcall(name = "setPeerContract", abi = "setPeerContract(uint64,string,address)")]
    pub struct SetPeerContractCall {
        pub peer_chain_id: u64,
        pub peer_chain_name: ::std::string::String,
        pub peer_contract: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `setPeerMinExecutorFee` function with signature `setPeerMinExecutorFee(uint256)` and selector `0x153dc450`
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
        Hash,
    )]
    #[ethcall(name = "setPeerMinExecutorFee", abi = "setPeerMinExecutorFee(uint256)")]
    pub struct SetPeerMinExecutorFeeCall {
        pub peer_min_executor_fee: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `setPeerMinRollupFee` function with signature `setPeerMinRollupFee(uint256)` and selector `0x521ff057`
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
        Hash,
    )]
    #[ethcall(name = "setPeerMinRollupFee", abi = "setPeerMinRollupFee(uint256)")]
    pub struct SetPeerMinRollupFeeCall {
        pub peer_min_rollup_fee: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `setReceiveVersion` function with signature `setReceiveVersion(uint16)` and selector `0x10ddb137`
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
        Hash,
    )]
    #[ethcall(name = "setReceiveVersion", abi = "setReceiveVersion(uint16)")]
    pub struct SetReceiveVersionCall {
        pub version: u16,
    }
    ///Container type for all input parameters for the `setSendVersion` function with signature `setSendVersion(uint16)` and selector `0x07e0db17`
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
        Hash,
    )]
    #[ethcall(name = "setSendVersion", abi = "setSendVersion(uint16)")]
    pub struct SetSendVersionCall {
        pub version: u16,
    }
    ///Container type for all input parameters for the `setTrustedRemote` function with signature `setTrustedRemote(uint16,bytes)` and selector `0xeb8d72b7`
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
        Hash,
    )]
    #[ethcall(name = "setTrustedRemote", abi = "setTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteCall {
        pub peer_layer_zero_chain_id: u16,
        pub peer_address: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `0x7533d788`
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
        Hash,
    )]
    #[ethcall(name = "trustedRemoteLookup", abi = "trustedRemoteLookup(uint16)")]
    pub struct TrustedRemoteLookupCall(pub u16);
    ///Container type for all input parameters for the `updateDepositAmountLimits` function with signature `updateDepositAmountLimits(uint256,uint256)` and selector `0xe8183c44`
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
        Hash,
    )]
    #[ethcall(
        name = "updateDepositAmountLimits",
        abi = "updateDepositAmountLimits(uint256,uint256)"
    )]
    pub struct UpdateDepositAmountLimitsCall {
        pub max_amount: ::ethers_core::types::U256,
        pub min_amount: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `updateSanctionsListAddress` function with signature `updateSanctionsListAddress(address)` and selector `0x30f49cac`
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
        Hash,
    )]
    #[ethcall(name = "updateSanctionsListAddress", abi = "updateSanctionsListAddress(address)")]
    pub struct UpdateSanctionsListAddressCall {
        pub sanction: ::ethers_core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers_contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoV2LayerZeroERC20Calls {
        AssetDecimals(AssetDecimalsCall),
        AssetName(AssetNameCall),
        AssetSymbol(AssetSymbolCall),
        AssetType(AssetTypeCall),
        BridgeProxyAddress(BridgeProxyAddressCall),
        BridgeType(BridgeTypeCall),
        ChangeOperator(ChangeOperatorCall),
        Deposit(DepositCall),
        DisableSanctionsCheck(DisableSanctionsCheckCall),
        EnableSanctionsCheck(EnableSanctionsCheckCall),
        FailedMessages(FailedMessagesCall),
        ForceResumeReceive(ForceResumeReceiveCall),
        GetAssociatedCommitmentPool(GetAssociatedCommitmentPoolCall),
        GetConfig(GetConfigCall),
        GetMaxAmount(GetMaxAmountCall),
        GetMinAmount(GetMinAmountCall),
        GetMinBridgeFee(GetMinBridgeFeeCall),
        GetMinExecutorFee(GetMinExecutorFeeCall),
        GetPeerMinExecutorFee(GetPeerMinExecutorFeeCall),
        GetPeerMinRollupFee(GetPeerMinRollupFeeCall),
        IsDepositsDisabled(IsDepositsDisabledCall),
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
        SanctionsCheck(SanctionsCheckCall),
        SanctionsList(SanctionsListCall),
        SetAssociatedCommitmentPool(SetAssociatedCommitmentPoolCall),
        SetBridgeProxyAddress(SetBridgeProxyAddressCall),
        SetConfig(SetConfigCall),
        SetDepositsDisabled(SetDepositsDisabledCall),
        SetEndpoint(SetEndpointCall),
        SetMinBridgeFee(SetMinBridgeFeeCall),
        SetMinExecutorFee(SetMinExecutorFeeCall),
        SetPeerContract(SetPeerContractCall),
        SetPeerMinExecutorFee(SetPeerMinExecutorFeeCall),
        SetPeerMinRollupFee(SetPeerMinRollupFeeCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetSendVersion(SetSendVersionCall),
        SetTrustedRemote(SetTrustedRemoteCall),
        TransferOwnership(TransferOwnershipCall),
        TrustedRemoteLookup(TrustedRemoteLookupCall),
        UpdateDepositAmountLimits(UpdateDepositAmountLimitsCall),
        UpdateSanctionsListAddress(UpdateSanctionsListAddressCall),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoV2LayerZeroERC20Calls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AssetDecimalsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetDecimals(decoded));
            }
            if let Ok(decoded) = <AssetNameCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetName(decoded));
            }
            if let Ok(decoded) = <AssetSymbolCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetSymbol(decoded));
            }
            if let Ok(decoded) = <AssetTypeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetType(decoded));
            }
            if let Ok(decoded) = <BridgeProxyAddressCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BridgeProxyAddress(decoded));
            }
            if let Ok(decoded) = <BridgeTypeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BridgeType(decoded));
            }
            if let Ok(decoded) = <ChangeOperatorCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChangeOperator(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <DisableSanctionsCheckCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableSanctionsCheck(decoded));
            }
            if let Ok(decoded) = <EnableSanctionsCheckCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableSanctionsCheck(decoded));
            }
            if let Ok(decoded) = <FailedMessagesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedMessages(decoded));
            }
            if let Ok(decoded) = <ForceResumeReceiveCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ForceResumeReceive(decoded));
            }
            if let Ok(decoded) = <GetAssociatedCommitmentPoolCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAssociatedCommitmentPool(decoded));
            }
            if let Ok(decoded) = <GetConfigCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetConfig(decoded));
            }
            if let Ok(decoded) = <GetMaxAmountCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMaxAmount(decoded));
            }
            if let Ok(decoded) = <GetMinAmountCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinAmount(decoded));
            }
            if let Ok(decoded) = <GetMinBridgeFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinBridgeFee(decoded));
            }
            if let Ok(decoded) = <GetMinExecutorFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinExecutorFee(decoded));
            }
            if let Ok(decoded) = <GetPeerMinExecutorFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPeerMinExecutorFee(decoded));
            }
            if let Ok(decoded) = <GetPeerMinRollupFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPeerMinRollupFee(decoded));
            }
            if let Ok(decoded) = <IsDepositsDisabledCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsDepositsDisabled(decoded));
            }
            if let Ok(decoded) = <IsTrustedRemoteCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsTrustedRemote(decoded));
            }
            if let Ok(decoded) = <LocalLayerZeroChainIdCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LocalLayerZeroChainId(decoded));
            }
            if let Ok(decoded) = <LzEndpointCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LzEndpoint(decoded));
            }
            if let Ok(decoded) = <LzReceiveCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LzReceive(decoded));
            }
            if let Ok(decoded) = <NonblockingLzReceiveCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NonblockingLzReceive(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PeerChainIdCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainId(decoded));
            }
            if let Ok(decoded) = <PeerChainNameCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainName(decoded));
            }
            if let Ok(decoded) = <PeerContractCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerContract(decoded));
            }
            if let Ok(decoded) = <PeerLayerZeroChainIdCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerLayerZeroChainId(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RetryMessageCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RetryMessage(decoded));
            }
            if let Ok(decoded) = <SanctionsCheckCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionsCheck(decoded));
            }
            if let Ok(decoded) = <SanctionsListCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionsList(decoded));
            }
            if let Ok(decoded) = <SetAssociatedCommitmentPoolCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAssociatedCommitmentPool(decoded));
            }
            if let Ok(decoded) = <SetBridgeProxyAddressCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBridgeProxyAddress(decoded));
            }
            if let Ok(decoded) = <SetConfigCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded) = <SetDepositsDisabledCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDepositsDisabled(decoded));
            }
            if let Ok(decoded) = <SetEndpointCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEndpoint(decoded));
            }
            if let Ok(decoded) = <SetMinBridgeFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMinBridgeFee(decoded));
            }
            if let Ok(decoded) = <SetMinExecutorFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMinExecutorFee(decoded));
            }
            if let Ok(decoded) = <SetPeerContractCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPeerContract(decoded));
            }
            if let Ok(decoded) = <SetPeerMinExecutorFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPeerMinExecutorFee(decoded));
            }
            if let Ok(decoded) = <SetPeerMinRollupFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPeerMinRollupFee(decoded));
            }
            if let Ok(decoded) = <SetReceiveVersionCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetReceiveVersion(decoded));
            }
            if let Ok(decoded) = <SetSendVersionCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSendVersion(decoded));
            }
            if let Ok(decoded) = <SetTrustedRemoteCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTrustedRemote(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <TrustedRemoteLookupCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TrustedRemoteLookup(decoded));
            }
            if let Ok(decoded) = <UpdateDepositAmountLimitsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateDepositAmountLimits(decoded));
            }
            if let Ok(decoded) = <UpdateSanctionsListAddressCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateSanctionsListAddress(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for MystikoV2LayerZeroERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetDecimals(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AssetName(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AssetSymbol(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AssetType(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::BridgeProxyAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::BridgeType(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ChangeOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DisableSanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::EnableSanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::FailedMessages(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ForceResumeReceive(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetAssociatedCommitmentPool(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetConfig(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMaxAmount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMinAmount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMinBridgeFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMinExecutorFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetPeerMinExecutorFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetPeerMinRollupFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::IsDepositsDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::IsTrustedRemote(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::LocalLayerZeroChainId(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::LzEndpoint(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::LzReceive(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NonblockingLzReceive(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::PeerChainId(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::PeerChainName(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::PeerContract(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::PeerLayerZeroChainId(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RetryMessage(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionsList(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetAssociatedCommitmentPool(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetBridgeProxyAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetConfig(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetDepositsDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetEndpoint(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetMinBridgeFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetMinExecutorFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetPeerContract(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetPeerMinExecutorFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetPeerMinRollupFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetReceiveVersion(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetSendVersion(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetTrustedRemote(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::TrustedRemoteLookup(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UpdateDepositAmountLimits(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UpdateSanctionsListAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2LayerZeroERC20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetName(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetSymbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetType(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeProxyAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeType(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableSanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableSanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedMessages(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceResumeReceive(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssociatedCommitmentPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinBridgeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPeerMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPeerMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::SanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAssociatedCommitmentPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBridgeProxyAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinBridgeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPeerContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPeerMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPeerMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReceiveVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSendVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTrustedRemote(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustedRemoteLookup(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDepositAmountLimits(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateSanctionsListAddress(element) => ::core::fmt::Display::fmt(element, f),
            }
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
    impl ::core::convert::From<ChangeOperatorCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: ChangeOperatorCall) -> Self {
            Self::ChangeOperator(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DisableSanctionsCheckCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: DisableSanctionsCheckCall) -> Self {
            Self::DisableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<EnableSanctionsCheckCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: EnableSanctionsCheckCall) -> Self {
            Self::EnableSanctionsCheck(value)
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
    impl ::core::convert::From<GetPeerMinExecutorFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: GetPeerMinExecutorFeeCall) -> Self {
            Self::GetPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<GetPeerMinRollupFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: GetPeerMinRollupFeeCall) -> Self {
            Self::GetPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<IsDepositsDisabledCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: IsDepositsDisabledCall) -> Self {
            Self::IsDepositsDisabled(value)
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
    impl ::core::convert::From<SanctionsCheckCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SanctionsCheckCall) -> Self {
            Self::SanctionsCheck(value)
        }
    }
    impl ::core::convert::From<SanctionsListCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SanctionsListCall) -> Self {
            Self::SanctionsList(value)
        }
    }
    impl ::core::convert::From<SetAssociatedCommitmentPoolCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetAssociatedCommitmentPoolCall) -> Self {
            Self::SetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<SetBridgeProxyAddressCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetBridgeProxyAddressCall) -> Self {
            Self::SetBridgeProxyAddress(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetDepositsDisabledCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetDepositsDisabledCall) -> Self {
            Self::SetDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<SetEndpointCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetEndpointCall) -> Self {
            Self::SetEndpoint(value)
        }
    }
    impl ::core::convert::From<SetMinBridgeFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetMinBridgeFeeCall) -> Self {
            Self::SetMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<SetMinExecutorFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetMinExecutorFeeCall) -> Self {
            Self::SetMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<SetPeerContractCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetPeerContractCall) -> Self {
            Self::SetPeerContract(value)
        }
    }
    impl ::core::convert::From<SetPeerMinExecutorFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetPeerMinExecutorFeeCall) -> Self {
            Self::SetPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<SetPeerMinRollupFeeCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: SetPeerMinRollupFeeCall) -> Self {
            Self::SetPeerMinRollupFee(value)
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
    impl ::core::convert::From<UpdateDepositAmountLimitsCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: UpdateDepositAmountLimitsCall) -> Self {
            Self::UpdateDepositAmountLimits(value)
        }
    }
    impl ::core::convert::From<UpdateSanctionsListAddressCall> for MystikoV2LayerZeroERC20Calls {
        fn from(value: UpdateSanctionsListAddressCall) -> Self {
            Self::UpdateSanctionsListAddress(value)
        }
    }
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct BridgeTypeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `failedMessages` function with signature `failedMessages(uint16,bytes,uint64)` and selector `0x5b8c41e6`
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
        Hash,
    )]
    pub struct FailedMessagesReturn(pub [u8; 32]);
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
        Hash,
    )]
    pub struct GetAssociatedCommitmentPoolReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `0xf5ecbdbc`
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
        Hash,
    )]
    pub struct GetConfigReturn(pub ::ethers_core::types::Bytes);
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct GetMinExecutorFeeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getPeerMinExecutorFee` function with signature `getPeerMinExecutorFee()` and selector `0x5898a0a8`
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
        Hash,
    )]
    pub struct GetPeerMinExecutorFeeReturn(pub ::ethers_core::types::U256);
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
        Hash,
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
        Hash,
    )]
    pub struct IsDepositsDisabledReturn(pub bool);
    ///Container type for all return fields from the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `0x3d8b38f6`
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
        Hash,
    )]
    pub struct IsTrustedRemoteReturn(pub bool);
    ///Container type for all return fields from the `localLayerZeroChainId` function with signature `localLayerZeroChainId()` and selector `0x302d5f4b`
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
        Hash,
    )]
    pub struct LocalLayerZeroChainIdReturn(pub u16);
    ///Container type for all return fields from the `lzEndpoint` function with signature `lzEndpoint()` and selector `0xb353aaa7`
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
        Hash,
    )]
    pub struct LzEndpointReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers_core::types::Address);
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct PeerContractReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `peerLayerZeroChainId` function with signature `peerLayerZeroChainId()` and selector `0x0097a063`
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
        Hash,
    )]
    pub struct PeerLayerZeroChainIdReturn(pub u16);
    ///Container type for all return fields from the `sanctionsCheck` function with signature `sanctionsCheck()` and selector `0xb1c39422`
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
        Hash,
    )]
    pub struct SanctionsCheckReturn(pub bool);
    ///Container type for all return fields from the `sanctionsList` function with signature `sanctionsList()` and selector `0xec571c6a`
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
        Hash,
    )]
    pub struct SanctionsListReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `0x7533d788`
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
        Hash,
    )]
    pub struct TrustedRemoteLookupReturn(pub ::ethers_core::types::Bytes);
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
        Hash,
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
}
