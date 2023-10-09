pub use mystiko_v2_layer_zero_main::*;
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
pub mod mystiko_v2_layer_zero_main {
    const _: () = {
        ::core::include_bytes!("../json/MystikoV2LayerZeroMain.json",);
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers_core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_hasher3"),
                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("contract IHasher3"),),
                },],
            }),
            functions: ::core::convert::From::from([
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
    pub static MYSTIKOV2LAYERZEROMAIN_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16t\x01@\xC5y#\x92K\\\\TU\xC4\x8D\x931q9\xAD\xDA\xC8\xFB\x17\x90U4\x80\x15b\0\08W`\0\x80\xFD[P`@Qb\09'8\x03\x80b\09'\x839\x81\x01`@\x81\x90Rb\0\0[\x91b\0\0\xF1V[`\x0C\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x80b\0\0\x97b\0\0\x913\x90V[b\0\0\x9FV[PPb\0\x01#V[`\r\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0` \x82\x84\x03\x12\x15b\0\x01\x04W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1CW`\0\x80\xFD[\x93\x92PPPV[a7\xF4\x80b\0\x013`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x03\x1CW`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\x01\xA5W\x80c\xD1\xDE\xBA\x1F\x11a\0\xECW\x80c\xEB\x8Dr\xB7\x11a\0\x95W\x80c\xEF\xBF\xB2\xAE\x11a\0oW\x80c\xEF\xBF\xB2\xAE\x14a\t\x16W\x80c\xF2\xFD\xE3\x8B\x14a\t+W\x80c\xF4\xAD\x17\xC6\x14a\tKW\x80c\xF5\xEC\xBD\xBC\x14a\t`W`\0\x80\xFD[\x80c\xEB\x8Dr\xB7\x14a\x08\xB7W\x80c\xECW\x1Cj\x14a\x08\xD7W\x80c\xEDn\xA3:\x14a\x08\xF7W`\0\x80\xFD[\x80c\xE1\x9A\xBE\xF8\x11a\0\xC6W\x80c\xE1\x9A\xBE\xF8\x14a\x08WW\x80c\xE8\x18<D\x14a\x08wW\x80c\xEA\x0C\xDE\x85\x14a\x08\x97W`\0\x80\xFD[\x80c\xD1\xDE\xBA\x1F\x14a\x08\x11W\x80c\xDDu|4\x14a\x08$W\x80c\xDD\xAC]\xC1\x14a\x089W`\0\x80\xFD[\x80c\xA3\xBCd\xF2\x11a\x01NW\x80c\xCB\xED\x8B\x9C\x11a\x01(W\x80c\xCB\xED\x8B\x9C\x14a\x07\x9BW\x80c\xCD\xFC\xEE\xBA\x14a\x07\xBBW\x80c\xCF\xC7\xE2\xDA\x14a\x07\xFCW`\0\x80\xFD[\x80c\xA3\xBCd\xF2\x14a\x07:W\x80c\xB1\xC3\x94\"\x14a\x07ZW\x80c\xB3S\xAA\xA7\x14a\x07{W`\0\x80\xFD[\x80c\x82[_\x8D\x11a\x01\x7FW\x80c\x82[_\x8D\x14a\x06\xF4W\x80c\x8D\xA5\xCB[\x14a\x07\tW\x80c\x9A\x03cl\x14a\x07'W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x06\x9FW\x80cu3\xD7\x88\x14a\x06\xB4W\x80c},\x85 \x14a\x06\xD4W`\0\x80\xFD[\x80c0-_K\x11a\x02iW\x80cN\xE7\xDE\xD6\x11a\x02\x12W\x80c[\x8CA\xE6\x11a\x01\xECW\x80c[\x8CA\xE6\x14a\x06\x10W\x80c^\x10\xB2\xB7\x14a\x06_W\x80cf\xAD\\\x8A\x14a\x06\x7FW`\0\x80\xFD[\x80cN\xE7\xDE\xD6\x14a\x05\xBBW\x80cR\x1F\xF0W\x14a\x05\xDBW\x80cX\x98\xA0\xA8\x14a\x05\xFBW`\0\x80\xFD[\x80c?\xE34z\x11a\x02CW\x80c?\xE34z\x14a\x05jW\x80cB\xD6Z\x8D\x14a\x05\x86W\x80cN<\x10\xB7\x14a\x05\xA6W`\0\x80\xFD[\x80c0-_K\x14a\x04\xF8W\x80c0\xF4\x9C\xAC\x14a\x05\x1AW\x80c=\x8B8\xF6\x14a\x05:W`\0\x80\xFD[\x80c\x10\xDD\xB17\x11a\x02\xCBW\x80c!\xE3-U\x11a\x02\xA5W\x80c!\xE3-U\x14a\x04QW\x80c$!\xE1U\x14a\x04\x89W\x80c,\xD2mE\x14a\x04\xD8W`\0\x80\xFD[\x80c\x10\xDD\xB17\x14a\x03\xF1W\x80c\x15=\xC4P\x14a\x04\x11W\x80c\x19\xE7]n\x14a\x041W`\0\x80\xFD[\x80c\x069L\x9B\x11a\x02\xFCW\x80c\x069L\x9B\x14a\x03\x92W\x80c\x07\xE0\xDB\x17\x14a\x03\xB2W\x80c\x0B\xA9Y\t\x14a\x03\xD2W`\0\x80\xFD[\x80b\x1D5g\x14a\x03!W\x80b\x97\xA0c\x14a\x03CW\x80c\x01\xDB\xF1\x9F\x14a\x03}W[`\0\x80\xFD[4\x80\x15a\x03-W`\0\x80\xFD[Pa\x03Aa\x03<6`\x04a1\x89V[a\t\x80V[\0[4\x80\x15a\x03OW`\0\x80\xFD[P`\x0ETa\x03e\x90`\x01`\xB0\x1B\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x89W`\0\x80\xFD[Pa\x03Aa\x0B\x07V[4\x80\x15a\x03\x9EW`\0\x80\xFD[Pa\x03Aa\x03\xAD6`\x04a/\x03V[a\x0B\x8BV[4\x80\x15a\x03\xBEW`\0\x80\xFD[Pa\x03Aa\x03\xCD6`\x04a0\x86V[a\x0C/V[4\x80\x15a\x03\xDEW`\0\x80\xFD[P`\x07T[`@Q\x90\x81R` \x01a\x03tV[4\x80\x15a\x03\xFDW`\0\x80\xFD[Pa\x03Aa\x04\x0C6`\x04a0\x86V[a\x0C\xE7V[4\x80\x15a\x04\x1DW`\0\x80\xFD[Pa\x03Aa\x04,6`\x04a2\xCAV[a\ruV[4\x80\x15a\x04=W`\0\x80\xFD[Pa\x03Aa\x04L6`\x04a2\xCAV[a\r\xDCV[4\x80\x15a\x04]W`\0\x80\xFD[P`\x04Ta\x04q\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03tV[4\x80\x15a\x04\x95W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\t\x81R\x7FlayerZero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x03t\x91\x90a4\xE0V[4\x80\x15a\x04\xE4W`\0\x80\xFD[P`\x05Ta\x04q\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\x04W`\0\x80\xFD[P`\x0ETa\x03e\x90`\x01`\xA0\x1B\x90\x04a\xFF\xFF\x16\x81V[4\x80\x15a\x05&W`\0\x80\xFD[Pa\x03Aa\x0556`\x04a/\x03V[a\x0E;V[4\x80\x15a\x05FW`\0\x80\xFD[Pa\x05Za\x05U6`\x04a0\xD8V[a\x0E\xB4V[`@Q\x90\x15\x15\x81R` \x01a\x03tV[4\x80\x15a\x05vW`\0\x80\xFD[P`\x01`@Qa\x03t\x91\x90a4\xF3V[4\x80\x15a\x05\x92W`\0\x80\xFD[Pa\x03Aa\x05\xA16`\x04a0\xD8V[a\x0F\x80V[4\x80\x15a\x05\xB2W`\0\x80\xFD[Pa\x04\xCBa\x10EV[4\x80\x15a\x05\xC7W`\0\x80\xFD[Pa\x03Aa\x05\xD66`\x04a0\xA1V[a\x10\xD3V[4\x80\x15a\x05\xE7W`\0\x80\xFD[Pa\x03Aa\x05\xF66`\x04a2\xCAV[a\x11\x82V[4\x80\x15a\x06\x07W`\0\x80\xFD[P`\nTa\x03\xE3V[4\x80\x15a\x06\x1CW`\0\x80\xFD[Pa\x03\xE3a\x06+6`\x04a1+V[`\x10` \x90\x81R`\0\x93\x84R`@\x80\x85 \x84Q\x80\x86\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x95\x84\x01\x95\x90\x95 \x94R\x92\x90R\x82R\x90 T\x81V[4\x80\x15a\x06kW`\0\x80\xFD[Pa\x03Aa\x06z6`\x04a2\xCAV[a\x120V[4\x80\x15a\x06\x8BW`\0\x80\xFD[Pa\x03Aa\x06\x9A6`\x04a1\x89V[a\x12\x90V[4\x80\x15a\x06\xABW`\0\x80\xFD[Pa\x03Aa\x12\xC2V[4\x80\x15a\x06\xC0W`\0\x80\xFD[Pa\x04\xCBa\x06\xCF6`\x04a0\x86V[a\x13(V[4\x80\x15a\x06\xE0W`\0\x80\xFD[Pa\x03Aa\x06\xEF6`\x04a3\x1EV[a\x13AV[4\x80\x15a\x07\0W`\0\x80\xFD[P`\x0BTa\x03\xE3V[4\x80\x15a\x07\x15W`\0\x80\xFD[P`\rT`\x01`\x01`\xA0\x1B\x03\x16a\x04qV[a\x03Aa\x0756`\x04a/\xC8V[a\x13\xDEV[4\x80\x15a\x07FW`\0\x80\xFD[Pa\x03Aa\x07U6`\x04a/\x03V[a\x15\xEEV[4\x80\x15a\x07fW`\0\x80\xFD[P`\0Ta\x05Z\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x07\x87W`\0\x80\xFD[P`\x0ETa\x04q\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\xA7W`\0\x80\xFD[Pa\x03Aa\x07\xB66`\x04a2[V[a\x16;V[4\x80\x15a\x07\xC7W`\0\x80\xFD[P`\x02Ta\x07\xE3\x90`\x01`\xA0\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03tV[4\x80\x15a\x08\x08W`\0\x80\xFD[P`\x06Ta\x03\xE3V[a\x03Aa\x08\x1F6`\x04a1\x89V[a\x17\x06V[4\x80\x15a\x080W`\0\x80\xFD[Pa\x03Aa\x18\x16V[4\x80\x15a\x08EW`\0\x80\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x04qV[4\x80\x15a\x08cW`\0\x80\xFD[Pa\x03Aa\x08r6`\x04a/\x03V[a\x18\x8EV[4\x80\x15a\x08\x83W`\0\x80\xFD[Pa\x03Aa\x08\x926`\x04a2\xFCV[a\x18\xDBV[4\x80\x15a\x08\xA3W`\0\x80\xFD[Pa\x03Aa\x08\xB26`\x04a/ V[a\x19nV[4\x80\x15a\x08\xC3W`\0\x80\xFD[Pa\x03Aa\x08\xD26`\x04a0\xD8V[a\x19\xE6V[4\x80\x15a\x08\xE3W`\0\x80\xFD[P`\0Ta\x04q\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\t\x03W`\0\x80\xFD[P`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x05ZV[4\x80\x15a\t\"W`\0\x80\xFD[P`\x08Ta\x03\xE3V[4\x80\x15a\t7W`\0\x80\xFD[Pa\x03Aa\tF6`\x04a/\x03V[a\x1A\xD4V[4\x80\x15a\tWW`\0\x80\xFD[P`\tTa\x03\xE3V[4\x80\x15a\tlW`\0\x80\xFD[Pa\x04\xCBa\t{6`\x04a2\x0EV[a\x1B\xB6V[`\x0ET`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xE9W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7Fendpoint caller\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF\x84\x16`\0\x90\x81R`\x0F` R`@\x81 \x80Ta\n\x07\x90a7JV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n3\x90a7JV[\x80\x15a\n\x80W\x80`\x1F\x10a\nUWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\ncW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q\x84Q\x14\x15\x80a\n\xA6WP\x80\x80Q\x90` \x01 \x84\x80Q\x90` \x01 \x14\x15[\x15a\n\xF4W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fsource sending contract\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[a\x0B\0\x85\x85\x85\x85a\x1CXV[PPPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B2W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x90\x81\x17\x91\x82\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x92a\x0B\x81\x92\x90\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xB6W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x15a\x0B\xE5W`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FG!\x12\x9E\x0Egn\xD6\xA9)\t\xBB$\xE8S\xCC\xDDc\xADr(\x0C\xC2\xE9t\xE3\x8EH\x0E\x0EnT\x90`\0\x90\xA2PV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0ET`@Qc\x07\xE0\xDB\x17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07\xE0\xDB\x17\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xD3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\0W=`\0\x80>=`\0\xFD[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\rAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0ET`@Qc\x10\xDD\xB17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x10\xDD\xB17\x90`$\x01a\x0C\xB9V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xA0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n\x81\x90U`@Q\x81\x81R\x7F\x14\x98\x824\xD3\xE5\n\x12\xAE\xEC-n\xE5\x95\xB7\x0C/\xAE\x16<\xAF\xF2\x8B\xAA\xB5\x80+\x12+\xF2\xA7S\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x07W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x81\x90U`@Q\x81\x81R~\x91\xF5\xF5\xDB0\x92\xE3\x9E\xCBp\x12\x18\xD4\xAF \xB7W\x1E\x04)YY7\xC34\xF3\xAC\xD1O\xE2\xFE\x90` \x01a\r\xD1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EfW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xBE\x18\x08\x91~\t\x81O\xCA}\x80(\x9A\x97`\xA4\x06_\x10\xD6$\xAC\x1A\xEE\x0B\x9F\x90<O]\x92\xB1\x90` \x01a\r\xD1V[a\xFF\xFF\x83\x16`\0\x90\x81R`\x0F` R`@\x81 \x80T\x82\x91\x90a\x0E\xD5\x90a7JV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\x01\x90a7JV[\x80\x15a\x0FNW\x80`\x1F\x10a\x0F#Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FNV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83\x83`@Qa\x0Fe\x92\x91\x90a3\xE9V[`@Q\x80\x91\x03\x90 \x81\x80Q\x90` \x01 \x14\x91PP\x93\x92PPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0ET`@QcB\xD6Z\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cB\xD6Z\x8D\x90a\x10\x0E\x90\x86\x90\x86\x90\x86\x90`\x04\x01a5xV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10(W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10<W=`\0\x80>=`\0\xFD[PPPPPPPV[`\x03\x80Ta\x10R\x90a7JV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10~\x90a7JV[\x80\x15a\x10\xCBW\x80`\x1F\x10a\x10\xA0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\xCBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xAEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0E\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\xA0\x1Ba\xFF\xFF\x94\x90\x94\x16\x93\x90\x93\x02`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x92\x17`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\xADW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x11\xFBW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fpeer minimal rollup fee\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[`\x0B\x81\x90U`@Q\x81\x81R\x7F\x87\x80u\xA8\xE8\xAA\x1C|\x15\xE6\x93'RR\x0Fx\x12\xBFWDx^m\xF6\x0577)\xDAAUE\x90` \x01a\r\xD1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12[W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t\x81\x90U`@Q\x81\x81R\x7F\x88\xD2\x10\xDF\xA1\x98\xF7Q\x95y)G!\xF9\x0Cw\x11S\xA7\xB4\x91\x01\xEE\xFB\x95\x14p7\xCC\x8C\xE4\x81\x90` \x01a\r\xD1V[30\x14a\x12\xB0W`@Qcq\xF5\x0E\xC1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xBC\x84\x84\x84\x84a\x1C\xB9V[PPPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[a\x13&`\0a\x1C\xF4V[V[`\x0F` R`\0\x90\x81R`@\x90 \x80Ta\x10R\x90a7JV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13lW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x02\x17\x90U\x81Qa\x13\xB9\x90`\x03\x90` \x85\x01\x90a-\x05V[P`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPV[`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x14\tW`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T\x81Q\x10\x15a\x14-W`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T\x81Q\x11\x15a\x14QW`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T\x81`\xA0\x01Q\x10\x15a\x14xW`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT\x81`\xC0\x01Q\x10\x15a\x14\x9FW`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT\x81`\xE0\x01Q\x10\x15a\x14\xC6W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xDF\x82`@\x01Q\x83`\0\x01Q\x84``\x01Qa\x1DFV[\x90P\x80\x82` \x01Q\x14a\x15\x05W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15\x0E3a\x1EcV[\x15a\x15,W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`\xC0\x84\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x83\x01Q``\x82\x01R`\x80\x80\x84\x01Q\x90\x82\x01R`\0a\x15o\x82a\x1F\0V[\x90Pa\x15\x7F\x84`\xA0\x01Q\x82a\x1FoV[`\x02T`\xE0\x85\x01Q`\xC0\x86\x01Q\x86Qa\x15\xBA\x93`\x01`\x01`\xA0\x1B\x03\x16\x92\x91a\x15\xA6\x91a6\xF8V[a\x15\xB0\x91\x90a6\xF8V[\x86`\xA0\x01Qa\x1F\xA1V[` \x84\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\x19W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0ET`@Qc2\xFBb\xE7`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCB\xED\x8B\x9C\x90a\x16\xCD\x90\x88\x90\x88\x90\x88\x90\x88\x90\x88\x90`\x04\x01a6GV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xFBW=`\0\x80>=`\0\xFD[PPPPPPPPPV[a\xFF\xFF\x84\x16`\0\x90\x81R`\x10` R`@\x80\x82 \x90Qa\x17'\x90\x86\x90a3\xF9V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R\x92R\x90 T\x90P\x80a\x17mW`@Qc+\x96\xC9\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q` \x83\x01 \x81\x14a\x17\xC3W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x07`$\x82\x01R\x7Fpayload\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[a\xFF\xFF\x85\x16`\0\x90\x81R`\x10` R`@\x80\x82 \x90Qa\x17\xE4\x90\x87\x90a3\xF9V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\0\x90\x81R\x92R\x90 Ua\x0B\0\x85\x85\x85\x85a\x1C\xB9V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18AW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90\x81\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x91a\x0B\x81\x91`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\xB9W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\x06W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x19'W`@Qc`\x03\xE8!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x82\x90U`\x06\x81\x90U`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x7Fv3\0Lz\"\x98i\xAE\xEA\x10\xDBO\xF3\xE5~;\x154\xAE\xB2\xC9\xE7,]\xB2_\x96X\x95\xC30\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\x99W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T\x82\x15\x15`\x01`\xA0\x1B\x02`\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x90U`@Q\x7F\xCD\x16,o\xC2B\x85\xBF\xBE9\x9E\xC0\xCC,\xE2\xC3\x80\xAD'\xD2\xEB\x1F\xCAA\x8Ce-\xF1%~~\t\x90a\r\xD1\x90\x83\x15\x15\x81R` \x01\x90V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0E\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xB0\x1Ba\xFF\xFF\x86\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`\0\x90\x81R`\x0F` R`@\x90 a\x1A\x93\x90\x83\x83a-\x89V[P\x7F\xFAAHz\xD5\xD6r\x8F\x0B\x19'o\xA1\xED\xDC\x16U\x85x\xF5\x10\x9F\xC3\x9D-\xC3<20G\r\xAB\x83\x83\x83`@Qa\x1A\xC7\x93\x92\x91\x90a5xV[`@Q\x80\x91\x03\x90\xA1PPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\xE0V[a\x1B\xB3\x81a\x1C\xF4V[PV[`\x0ET`@Qc={/o`\xE2\x1B\x81Ra\xFF\xFF\x80\x87\x16`\x04\x83\x01R\x85\x16`$\x82\x01R0`D\x82\x01R`d\x81\x01\x83\x90R``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF5\xEC\xBD\xBC\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\x13W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1C'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1CO\x91\x90\x81\x01\x90a/ZV[\x95\x94PPPPPV[`@Qc3V\xAEE`\xE1\x1B\x81R0\x90cf\xAD\\\x8A\x90a\x1C\x81\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a5\xFDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\xAFW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0a\x1C\xC4\x82a \x9CV[`\x02T`\x04T\x91\x92Pa\x0B\0\x91`\x01`\xA0\x1B\x90\x91\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90`\x01`\x01`\xA0\x1B\x03\x162\x84a!dV[`\r\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x1D\x89W`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x1D\xBBW`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x1E\x13\x91`\x04\x01a4\xAFV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E+W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1E?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CO\x91\x90a2\xE3V[`\0\x80T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x1E}WP`\0\x91\x90PV[`\0T`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xC2W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1E\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xFA\x91\x90a/=V[\x92\x91PPV[``\x80a\x1F\x10\x83`\0\x01Qa\"\x19V[a\x1F\x1D\x84` \x01Qa\"\x19V[a\x1F*\x85`@\x01Qa\"\x19V[a\x1F7\x86``\x01Qa\"\x19V[a\x1FD\x87`\x80\x01Qa\"\xB1V[`@Q` \x01a\x1FX\x95\x94\x93\x92\x91\x90a4DV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[a\x1F\x9D`\x0E`\x16\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16\x823`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x87a\"\xE8V[PPV[a\x1F\xAB\x81\x83a6\xF8V[4\x14a\x1F\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Finsufficient token\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a KV[``\x91P[PP\x90P\x80a\x12\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Famount transfer failed\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[a \xCE`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a!\0`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a!\x0C\x84\x82a$\x1FV[\x90\x83R\x90Pa!\x1B\x84\x82a$\x1FV[` \x84\x01\x91\x90\x91R\x90Pa!/\x84\x82a$\x1FV[`@\x84\x01\x91\x90\x91R\x90Pa!C\x84\x82a$\x1FV[``\x84\x01\x91\x90\x91R\x90Pa!W\x84\x82a%PV[P`\x80\x83\x01RP\x92\x91PPV[`\x04T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a!\x92W`@Qc\x14@\xE0y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xA0\x1B\x90\x92\x04\x16\x14a!\xC8W`@Qc8v0O`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qa!\xE7W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`@Qcx\xD6\x0C\xD7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx\xD6\x0C\xD7\x90a\x1C\x81\x90\x84\x90\x86\x90`\x04\x01a5\x1BV[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\"rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\"\xA1W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\"\x7FV[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\"\xBF\x81a&]V[\x83`@Q` \x01a\"\xD1\x92\x91\x90a4\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[a\xFF\xFF\x86\x16`\0\x90\x81R`\x0F` R`@\x81 \x80Ta#\x06\x90a7JV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta#2\x90a7JV[\x80\x15a#\x7FW\x80`\x1F\x10a#TWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a#\x7FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#bW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q`\0\x14\x15a#\xA9W`@Qc\x02\x0B5\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0ET`@Qb\xC5\x801`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5\x801\0\x90\x84\x90a#\xE4\x90\x8B\x90\x86\x90\x8C\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a5\x96V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a#\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a$\x11W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0\x80\x83Q\x83` a$1\x91\x90a6\xF8V[\x11\x15\x80\x15a$HWPa$E\x83` a6\xF8V[\x83\x10[a$\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\t\xE0V[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a$\xD5W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa$\xB5V[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a%7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[\x80a%C\x85` a6\xF8V[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a%_\x85\x85a'*V[\x86Q\x90\x95P\x90\x91Pa%q\x82\x86a6\xF8V[\x11\x15\x80\x15a%\x87WPa%\x84\x81\x85a6\xF8V[\x84\x10[a%\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\t\xE0V[``\x81\x15\x80\x15a%\xFAW`@Q\x91P` \x82\x01`@Ra&DV[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a&3W\x80Q\x83R` \x92\x83\x01\x92\x01a&\x1BV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a&P\x83\x87a6\xF8V[\x93P\x93PPP\x92P\x92\x90PV[```\xFD\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a&\x90W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91Ra\x1E\xFAV[a\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a&\xE0Wa&\xB0`\xFD`\xF8\x1Ba)0V[a&\xB9\x83a)WV[`@Q` \x01a&\xCA\x92\x91\x90a4\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a'\x0BWa'\x02`\x7F`\xF9\x1Ba)0V[a&\xB9\x83a)\x9AV[a'\x1C`\x01`\x01`\xF8\x1B\x03\x19a)0V[a&\xB9\x83a)\xDDV[\x91\x90PV[`\0\x80`\0a'9\x85\x85a* V[\x94P\x90P`\0`\xFD`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a'\xD2Wa'_\x86\x86a*\xA8V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a'zWPa\xFF\xFF\x81\x11\x15[a'\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\t\xE0V[\x92P\x83\x91Pa%I\x90PV[`\x7F`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a(]Wa'\xF2\x86\x86a+aV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a(\x11WPc\xFF\xFF\xFF\xFF\x81\x11\x15[a'\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\t\xE0V[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x14\x15a(\xDAWa(y\x86\x86a,3V[\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a'\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\t\xE0V[P`\xF8\x81\x90\x1C`\xFD\x81\x10a'\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\t\xE0V[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a\x1E\xFAV[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a)\x8AW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a)hV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a)\xCDW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a)\xABV[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a*\x10W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a)\xEEV[PPP`(\x81\x01`@R\x92\x91PPV[`\0\x80\x83Q\x83`\x01a*2\x91\x90a6\xF8V[\x11\x15\x80\x15a*IWPa*F\x83`\x01a6\xF8V[\x83\x10[a*\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\t\xE0V[\x83\x83\x01` \x01Q\x80a%C\x85`\x01a6\xF8V[`\0\x80\x83Q\x83`\x02a*\xBA\x91\x90a6\xF8V[\x11\x15\x80\x15a*\xD1WPa*\xCE\x83`\x02a6\xF8V[\x83\x10[a+(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\t\xE0V[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a%C\x91\x90a6\xF8V[`\0\x80\x83Q\x83`\x04a+s\x91\x90a6\xF8V[\x11\x15\x80\x15a+\x8AWPa+\x87\x83`\x04a6\xF8V[\x83\x10[a+\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\t\xE0V[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a,\x16W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa+\xF6V[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a%C\x85`\x04a6\xF8V[`\0\x80\x83Q\x83`\x08a,E\x91\x90a6\xF8V[\x11\x15\x80\x15a,\\WPa,Y\x83`\x08a6\xF8V[\x83\x10[a,\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\t\xE0V[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a,\xE8W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa,\xC8V[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a%C\x85`\x08a6\xF8V[\x82\x80Ta-\x11\x90a7JV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a-3W`\0\x85Ua-yV[\x82`\x1F\x10a-LW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua-yV[\x82\x80\x01`\x01\x01\x85U\x82\x15a-yW\x91\x82\x01[\x82\x81\x11\x15a-yW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a-^V[Pa-\x85\x92\x91Pa-\xFDV[P\x90V[\x82\x80Ta-\x95\x90a7JV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a-\xB7W`\0\x85Ua-yV[\x82`\x1F\x10a-\xD0W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua-yV[\x82\x80\x01`\x01\x01\x85U\x82\x15a-yW\x91\x82\x01[\x82\x81\x11\x15a-yW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a-\xE2V[[\x80\x82\x11\x15a-\x85W`\0\x81U`\x01\x01a-\xFEV[`\0a.%a. \x84a6\xD0V[a6\x9FV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a.9W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a.bW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.zW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a%IW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a.\xA3W`\0\x80\xFD[a.\xB2\x83\x835` \x85\x01a.\x12V[\x93\x92PPPV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'%W`\0\x80\xFD[\x805a\xFF\xFF\x81\x16\x81\x14a'%W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'%W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a/\x15W`\0\x80\xFD[\x815a.\xB2\x81a7\x9BV[`\0` \x82\x84\x03\x12\x15a/2W`\0\x80\xFD[\x815a.\xB2\x81a7\xB0V[`\0` \x82\x84\x03\x12\x15a/OW`\0\x80\xFD[\x81Qa.\xB2\x81a7\xB0V[`\0` \x82\x84\x03\x12\x15a/lW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\x83W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a/\x94W`\0\x80\xFD[\x80Qa/\xA2a. \x82a6\xD0V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a/\xB7W`\0\x80\xFD[a\x1CO\x82` \x83\x01` \x86\x01a7\x1EV[`\0` \x82\x84\x03\x12\x15a/\xDAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a/\xF2W`\0\x80\xFD[\x90\x83\x01\x90a\x01\0\x82\x86\x03\x12\x15a0\x07W`\0\x80\xFD[a0\x0Fa6uV[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01Ra03``\x84\x01a.\xB9V[``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a0JW`\0\x80\xFD[a0V\x87\x82\x86\x01a.\x92V[`\x80\x83\x01RP`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015`\xE0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a0\x98W`\0\x80\xFD[a.\xB2\x82a.\xD9V[`\0\x80`@\x83\x85\x03\x12\x15a0\xB4W`\0\x80\xFD[a0\xBD\x83a.\xD9V[\x91P` \x83\x015a0\xCD\x81a7\x9BV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a0\xEDW`\0\x80\xFD[a0\xF6\x84a.\xD9V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x12W`\0\x80\xFD[a1\x1E\x86\x82\x87\x01a.PV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a1@W`\0\x80\xFD[a1I\x84a.\xD9V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1eW`\0\x80\xFD[a1q\x86\x82\x87\x01a.\x92V[\x92PPa1\x80`@\x85\x01a.\xEBV[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a1\x9FW`\0\x80\xFD[a1\xA8\x85a.\xD9V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a1\xC5W`\0\x80\xFD[a1\xD1\x88\x83\x89\x01a.\x92V[\x94Pa1\xDF`@\x88\x01a.\xEBV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a1\xF5W`\0\x80\xFD[Pa2\x02\x87\x82\x88\x01a.\x92V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a2$W`\0\x80\xFD[a2-\x85a.\xD9V[\x93Pa2;` \x86\x01a.\xD9V[\x92P`@\x85\x015a2K\x81a7\x9BV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a2sW`\0\x80\xFD[a2|\x86a.\xD9V[\x94Pa2\x8A` \x87\x01a.\xD9V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xADW`\0\x80\xFD[a2\xB9\x88\x82\x89\x01a.PV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a2\xDCW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a2\xF5W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a3\x0FW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a33W`\0\x80\xFD[a3<\x84a.\xEBV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3XW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a3iW`\0\x80\xFD[a3x\x86\x825` \x84\x01a.\x12V[\x92PP`@\x84\x015a3\x89\x81a7\x9BV[\x80\x91PP\x92P\x92P\x92V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x81Q\x80\x84Ra3\xD5\x81` \x86\x01` \x86\x01a7\x1EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82Qa4\x0B\x81\x84` \x87\x01a7\x1EV[\x91\x90\x91\x01\x92\x91PPV[`\0\x83Qa4'\x81\x84` \x88\x01a7\x1EV[\x83Q\x90\x83\x01\x90a4;\x81\x83` \x88\x01a7\x1EV[\x01\x94\x93PPPPV[`\0\x86Qa4V\x81\x84` \x8B\x01a7\x1EV[\x86Q\x90\x83\x01\x90a4j\x81\x83` \x8B\x01a7\x1EV[\x86Q\x91\x01\x90a4}\x81\x83` \x8A\x01a7\x1EV[\x85Q\x91\x01\x90a4\x90\x81\x83` \x89\x01a7\x1EV[\x84Q\x91\x01\x90a4\xA3\x81\x83` \x88\x01a7\x1EV[\x01\x97\x96PPPPPPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a4\xD7W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a4\xB8V[PPP\x92\x91PPV[` \x81R`\0a.\xB2` \x83\x01\x84a3\xBDV[` \x81\x01`\x02\x83\x10a5\x15WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra5_`\xE0\x84\x01\x82a3\xBDV[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[a\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x1CO`@\x83\x01\x84\x86a3\x94V[a\xFF\xFF\x87\x16\x81R`\xC0` \x82\x01R`\0a5\xB3`\xC0\x83\x01\x88a3\xBDV[\x82\x81\x03`@\x84\x01Ra5\xC5\x81\x88a3\xBDV[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16``\x86\x01R\x86\x16`\x80\x85\x01R\x83\x81\x03`\xA0\x85\x01R\x90Pa5\xF0\x81\x85a3\xBDV[\x99\x98PPPPPPPPPV[a\xFF\xFF\x85\x16\x81R`\x80` \x82\x01R`\0a6\x1A`\x80\x83\x01\x86a3\xBDV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra6<\x81\x85a3\xBDV[\x97\x96PPPPPPPV[`\0a\xFF\xFF\x80\x88\x16\x83R\x80\x87\x16` \x84\x01RP\x84`@\x83\x01R`\x80``\x83\x01Ra6<`\x80\x83\x01\x84\x86a3\x94V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a6\x99Wa6\x99a7\x85V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a6\xC8Wa6\xC8a7\x85V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a6\xEAWa6\xEAa7\x85V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82\x19\x82\x11\x15a7\x19WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[`\0[\x83\x81\x10\x15a79W\x81\x81\x01Q\x83\x82\x01R` \x01a7!V[\x83\x81\x11\x15a\x12\xBCWPP`\0\x91\x01RV[`\x01\x81\x81\x1C\x90\x82\x16\x80a7^W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a7\x7FWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xB3W`\0\x80\xFD[\x80\x15\x15\x81\x14a\x1B\xB3W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 g\xF4\xCA\n\xA6\xCB\x86\xBD\xD9\xE9|\x14\xB5\x90\x89=\xA03a\x02\x8E\xF4\x08\x98\x0F\n\xE26H\xE4~\xFCdsolcC\0\x08\x07\x003";
    /// The bytecode of the contract.
    pub static MYSTIKOV2LAYERZEROMAIN_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x03\x1CW`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\x01\xA5W\x80c\xD1\xDE\xBA\x1F\x11a\0\xECW\x80c\xEB\x8Dr\xB7\x11a\0\x95W\x80c\xEF\xBF\xB2\xAE\x11a\0oW\x80c\xEF\xBF\xB2\xAE\x14a\t\x16W\x80c\xF2\xFD\xE3\x8B\x14a\t+W\x80c\xF4\xAD\x17\xC6\x14a\tKW\x80c\xF5\xEC\xBD\xBC\x14a\t`W`\0\x80\xFD[\x80c\xEB\x8Dr\xB7\x14a\x08\xB7W\x80c\xECW\x1Cj\x14a\x08\xD7W\x80c\xEDn\xA3:\x14a\x08\xF7W`\0\x80\xFD[\x80c\xE1\x9A\xBE\xF8\x11a\0\xC6W\x80c\xE1\x9A\xBE\xF8\x14a\x08WW\x80c\xE8\x18<D\x14a\x08wW\x80c\xEA\x0C\xDE\x85\x14a\x08\x97W`\0\x80\xFD[\x80c\xD1\xDE\xBA\x1F\x14a\x08\x11W\x80c\xDDu|4\x14a\x08$W\x80c\xDD\xAC]\xC1\x14a\x089W`\0\x80\xFD[\x80c\xA3\xBCd\xF2\x11a\x01NW\x80c\xCB\xED\x8B\x9C\x11a\x01(W\x80c\xCB\xED\x8B\x9C\x14a\x07\x9BW\x80c\xCD\xFC\xEE\xBA\x14a\x07\xBBW\x80c\xCF\xC7\xE2\xDA\x14a\x07\xFCW`\0\x80\xFD[\x80c\xA3\xBCd\xF2\x14a\x07:W\x80c\xB1\xC3\x94\"\x14a\x07ZW\x80c\xB3S\xAA\xA7\x14a\x07{W`\0\x80\xFD[\x80c\x82[_\x8D\x11a\x01\x7FW\x80c\x82[_\x8D\x14a\x06\xF4W\x80c\x8D\xA5\xCB[\x14a\x07\tW\x80c\x9A\x03cl\x14a\x07'W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x06\x9FW\x80cu3\xD7\x88\x14a\x06\xB4W\x80c},\x85 \x14a\x06\xD4W`\0\x80\xFD[\x80c0-_K\x11a\x02iW\x80cN\xE7\xDE\xD6\x11a\x02\x12W\x80c[\x8CA\xE6\x11a\x01\xECW\x80c[\x8CA\xE6\x14a\x06\x10W\x80c^\x10\xB2\xB7\x14a\x06_W\x80cf\xAD\\\x8A\x14a\x06\x7FW`\0\x80\xFD[\x80cN\xE7\xDE\xD6\x14a\x05\xBBW\x80cR\x1F\xF0W\x14a\x05\xDBW\x80cX\x98\xA0\xA8\x14a\x05\xFBW`\0\x80\xFD[\x80c?\xE34z\x11a\x02CW\x80c?\xE34z\x14a\x05jW\x80cB\xD6Z\x8D\x14a\x05\x86W\x80cN<\x10\xB7\x14a\x05\xA6W`\0\x80\xFD[\x80c0-_K\x14a\x04\xF8W\x80c0\xF4\x9C\xAC\x14a\x05\x1AW\x80c=\x8B8\xF6\x14a\x05:W`\0\x80\xFD[\x80c\x10\xDD\xB17\x11a\x02\xCBW\x80c!\xE3-U\x11a\x02\xA5W\x80c!\xE3-U\x14a\x04QW\x80c$!\xE1U\x14a\x04\x89W\x80c,\xD2mE\x14a\x04\xD8W`\0\x80\xFD[\x80c\x10\xDD\xB17\x14a\x03\xF1W\x80c\x15=\xC4P\x14a\x04\x11W\x80c\x19\xE7]n\x14a\x041W`\0\x80\xFD[\x80c\x069L\x9B\x11a\x02\xFCW\x80c\x069L\x9B\x14a\x03\x92W\x80c\x07\xE0\xDB\x17\x14a\x03\xB2W\x80c\x0B\xA9Y\t\x14a\x03\xD2W`\0\x80\xFD[\x80b\x1D5g\x14a\x03!W\x80b\x97\xA0c\x14a\x03CW\x80c\x01\xDB\xF1\x9F\x14a\x03}W[`\0\x80\xFD[4\x80\x15a\x03-W`\0\x80\xFD[Pa\x03Aa\x03<6`\x04a1\x89V[a\t\x80V[\0[4\x80\x15a\x03OW`\0\x80\xFD[P`\x0ETa\x03e\x90`\x01`\xB0\x1B\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x89W`\0\x80\xFD[Pa\x03Aa\x0B\x07V[4\x80\x15a\x03\x9EW`\0\x80\xFD[Pa\x03Aa\x03\xAD6`\x04a/\x03V[a\x0B\x8BV[4\x80\x15a\x03\xBEW`\0\x80\xFD[Pa\x03Aa\x03\xCD6`\x04a0\x86V[a\x0C/V[4\x80\x15a\x03\xDEW`\0\x80\xFD[P`\x07T[`@Q\x90\x81R` \x01a\x03tV[4\x80\x15a\x03\xFDW`\0\x80\xFD[Pa\x03Aa\x04\x0C6`\x04a0\x86V[a\x0C\xE7V[4\x80\x15a\x04\x1DW`\0\x80\xFD[Pa\x03Aa\x04,6`\x04a2\xCAV[a\ruV[4\x80\x15a\x04=W`\0\x80\xFD[Pa\x03Aa\x04L6`\x04a2\xCAV[a\r\xDCV[4\x80\x15a\x04]W`\0\x80\xFD[P`\x04Ta\x04q\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03tV[4\x80\x15a\x04\x95W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\t\x81R\x7FlayerZero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x03t\x91\x90a4\xE0V[4\x80\x15a\x04\xE4W`\0\x80\xFD[P`\x05Ta\x04q\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\x04W`\0\x80\xFD[P`\x0ETa\x03e\x90`\x01`\xA0\x1B\x90\x04a\xFF\xFF\x16\x81V[4\x80\x15a\x05&W`\0\x80\xFD[Pa\x03Aa\x0556`\x04a/\x03V[a\x0E;V[4\x80\x15a\x05FW`\0\x80\xFD[Pa\x05Za\x05U6`\x04a0\xD8V[a\x0E\xB4V[`@Q\x90\x15\x15\x81R` \x01a\x03tV[4\x80\x15a\x05vW`\0\x80\xFD[P`\x01`@Qa\x03t\x91\x90a4\xF3V[4\x80\x15a\x05\x92W`\0\x80\xFD[Pa\x03Aa\x05\xA16`\x04a0\xD8V[a\x0F\x80V[4\x80\x15a\x05\xB2W`\0\x80\xFD[Pa\x04\xCBa\x10EV[4\x80\x15a\x05\xC7W`\0\x80\xFD[Pa\x03Aa\x05\xD66`\x04a0\xA1V[a\x10\xD3V[4\x80\x15a\x05\xE7W`\0\x80\xFD[Pa\x03Aa\x05\xF66`\x04a2\xCAV[a\x11\x82V[4\x80\x15a\x06\x07W`\0\x80\xFD[P`\nTa\x03\xE3V[4\x80\x15a\x06\x1CW`\0\x80\xFD[Pa\x03\xE3a\x06+6`\x04a1+V[`\x10` \x90\x81R`\0\x93\x84R`@\x80\x85 \x84Q\x80\x86\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x95\x84\x01\x95\x90\x95 \x94R\x92\x90R\x82R\x90 T\x81V[4\x80\x15a\x06kW`\0\x80\xFD[Pa\x03Aa\x06z6`\x04a2\xCAV[a\x120V[4\x80\x15a\x06\x8BW`\0\x80\xFD[Pa\x03Aa\x06\x9A6`\x04a1\x89V[a\x12\x90V[4\x80\x15a\x06\xABW`\0\x80\xFD[Pa\x03Aa\x12\xC2V[4\x80\x15a\x06\xC0W`\0\x80\xFD[Pa\x04\xCBa\x06\xCF6`\x04a0\x86V[a\x13(V[4\x80\x15a\x06\xE0W`\0\x80\xFD[Pa\x03Aa\x06\xEF6`\x04a3\x1EV[a\x13AV[4\x80\x15a\x07\0W`\0\x80\xFD[P`\x0BTa\x03\xE3V[4\x80\x15a\x07\x15W`\0\x80\xFD[P`\rT`\x01`\x01`\xA0\x1B\x03\x16a\x04qV[a\x03Aa\x0756`\x04a/\xC8V[a\x13\xDEV[4\x80\x15a\x07FW`\0\x80\xFD[Pa\x03Aa\x07U6`\x04a/\x03V[a\x15\xEEV[4\x80\x15a\x07fW`\0\x80\xFD[P`\0Ta\x05Z\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x07\x87W`\0\x80\xFD[P`\x0ETa\x04q\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\xA7W`\0\x80\xFD[Pa\x03Aa\x07\xB66`\x04a2[V[a\x16;V[4\x80\x15a\x07\xC7W`\0\x80\xFD[P`\x02Ta\x07\xE3\x90`\x01`\xA0\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03tV[4\x80\x15a\x08\x08W`\0\x80\xFD[P`\x06Ta\x03\xE3V[a\x03Aa\x08\x1F6`\x04a1\x89V[a\x17\x06V[4\x80\x15a\x080W`\0\x80\xFD[Pa\x03Aa\x18\x16V[4\x80\x15a\x08EW`\0\x80\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x04qV[4\x80\x15a\x08cW`\0\x80\xFD[Pa\x03Aa\x08r6`\x04a/\x03V[a\x18\x8EV[4\x80\x15a\x08\x83W`\0\x80\xFD[Pa\x03Aa\x08\x926`\x04a2\xFCV[a\x18\xDBV[4\x80\x15a\x08\xA3W`\0\x80\xFD[Pa\x03Aa\x08\xB26`\x04a/ V[a\x19nV[4\x80\x15a\x08\xC3W`\0\x80\xFD[Pa\x03Aa\x08\xD26`\x04a0\xD8V[a\x19\xE6V[4\x80\x15a\x08\xE3W`\0\x80\xFD[P`\0Ta\x04q\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\t\x03W`\0\x80\xFD[P`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x05ZV[4\x80\x15a\t\"W`\0\x80\xFD[P`\x08Ta\x03\xE3V[4\x80\x15a\t7W`\0\x80\xFD[Pa\x03Aa\tF6`\x04a/\x03V[a\x1A\xD4V[4\x80\x15a\tWW`\0\x80\xFD[P`\tTa\x03\xE3V[4\x80\x15a\tlW`\0\x80\xFD[Pa\x04\xCBa\t{6`\x04a2\x0EV[a\x1B\xB6V[`\x0ET`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xE9W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7Fendpoint caller\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF\x84\x16`\0\x90\x81R`\x0F` R`@\x81 \x80Ta\n\x07\x90a7JV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n3\x90a7JV[\x80\x15a\n\x80W\x80`\x1F\x10a\nUWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\ncW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q\x84Q\x14\x15\x80a\n\xA6WP\x80\x80Q\x90` \x01 \x84\x80Q\x90` \x01 \x14\x15[\x15a\n\xF4W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fsource sending contract\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[a\x0B\0\x85\x85\x85\x85a\x1CXV[PPPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B2W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x90\x81\x17\x91\x82\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x92a\x0B\x81\x92\x90\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xB6W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x15a\x0B\xE5W`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FG!\x12\x9E\x0Egn\xD6\xA9)\t\xBB$\xE8S\xCC\xDDc\xADr(\x0C\xC2\xE9t\xE3\x8EH\x0E\x0EnT\x90`\0\x90\xA2PV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0ET`@Qc\x07\xE0\xDB\x17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07\xE0\xDB\x17\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xD3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\0W=`\0\x80>=`\0\xFD[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\rAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0ET`@Qc\x10\xDD\xB17`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x10\xDD\xB17\x90`$\x01a\x0C\xB9V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xA0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n\x81\x90U`@Q\x81\x81R\x7F\x14\x98\x824\xD3\xE5\n\x12\xAE\xEC-n\xE5\x95\xB7\x0C/\xAE\x16<\xAF\xF2\x8B\xAA\xB5\x80+\x12+\xF2\xA7S\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x07W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x81\x90U`@Q\x81\x81R~\x91\xF5\xF5\xDB0\x92\xE3\x9E\xCBp\x12\x18\xD4\xAF \xB7W\x1E\x04)YY7\xC34\xF3\xAC\xD1O\xE2\xFE\x90` \x01a\r\xD1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EfW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xBE\x18\x08\x91~\t\x81O\xCA}\x80(\x9A\x97`\xA4\x06_\x10\xD6$\xAC\x1A\xEE\x0B\x9F\x90<O]\x92\xB1\x90` \x01a\r\xD1V[a\xFF\xFF\x83\x16`\0\x90\x81R`\x0F` R`@\x81 \x80T\x82\x91\x90a\x0E\xD5\x90a7JV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\x01\x90a7JV[\x80\x15a\x0FNW\x80`\x1F\x10a\x0F#Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FNV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83\x83`@Qa\x0Fe\x92\x91\x90a3\xE9V[`@Q\x80\x91\x03\x90 \x81\x80Q\x90` \x01 \x14\x91PP\x93\x92PPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0ET`@QcB\xD6Z\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cB\xD6Z\x8D\x90a\x10\x0E\x90\x86\x90\x86\x90\x86\x90`\x04\x01a5xV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10(W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10<W=`\0\x80>=`\0\xFD[PPPPPPPV[`\x03\x80Ta\x10R\x90a7JV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10~\x90a7JV[\x80\x15a\x10\xCBW\x80`\x1F\x10a\x10\xA0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\xCBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xAEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0E\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\xA0\x1Ba\xFF\xFF\x94\x90\x94\x16\x93\x90\x93\x02`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x92\x17`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\xADW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x11\xFBW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fpeer minimal rollup fee\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[`\x0B\x81\x90U`@Q\x81\x81R\x7F\x87\x80u\xA8\xE8\xAA\x1C|\x15\xE6\x93'RR\x0Fx\x12\xBFWDx^m\xF6\x0577)\xDAAUE\x90` \x01a\r\xD1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12[W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t\x81\x90U`@Q\x81\x81R\x7F\x88\xD2\x10\xDF\xA1\x98\xF7Q\x95y)G!\xF9\x0Cw\x11S\xA7\xB4\x91\x01\xEE\xFB\x95\x14p7\xCC\x8C\xE4\x81\x90` \x01a\r\xD1V[30\x14a\x12\xB0W`@Qcq\xF5\x0E\xC1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xBC\x84\x84\x84\x84a\x1C\xB9V[PPPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[a\x13&`\0a\x1C\xF4V[V[`\x0F` R`\0\x90\x81R`@\x90 \x80Ta\x10R\x90a7JV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13lW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x02\x17\x90U\x81Qa\x13\xB9\x90`\x03\x90` \x85\x01\x90a-\x05V[P`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPV[`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x14\tW`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T\x81Q\x10\x15a\x14-W`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T\x81Q\x11\x15a\x14QW`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T\x81`\xA0\x01Q\x10\x15a\x14xW`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT\x81`\xC0\x01Q\x10\x15a\x14\x9FW`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT\x81`\xE0\x01Q\x10\x15a\x14\xC6W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xDF\x82`@\x01Q\x83`\0\x01Q\x84``\x01Qa\x1DFV[\x90P\x80\x82` \x01Q\x14a\x15\x05W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15\x0E3a\x1EcV[\x15a\x15,W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`\xC0\x84\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x83\x01Q``\x82\x01R`\x80\x80\x84\x01Q\x90\x82\x01R`\0a\x15o\x82a\x1F\0V[\x90Pa\x15\x7F\x84`\xA0\x01Q\x82a\x1FoV[`\x02T`\xE0\x85\x01Q`\xC0\x86\x01Q\x86Qa\x15\xBA\x93`\x01`\x01`\xA0\x1B\x03\x16\x92\x91a\x15\xA6\x91a6\xF8V[a\x15\xB0\x91\x90a6\xF8V[\x86`\xA0\x01Qa\x1F\xA1V[` \x84\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\x19W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0ET`@Qc2\xFBb\xE7`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCB\xED\x8B\x9C\x90a\x16\xCD\x90\x88\x90\x88\x90\x88\x90\x88\x90\x88\x90`\x04\x01a6GV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xFBW=`\0\x80>=`\0\xFD[PPPPPPPPPV[a\xFF\xFF\x84\x16`\0\x90\x81R`\x10` R`@\x80\x82 \x90Qa\x17'\x90\x86\x90a3\xF9V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R\x92R\x90 T\x90P\x80a\x17mW`@Qc+\x96\xC9\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q` \x83\x01 \x81\x14a\x17\xC3W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x07`$\x82\x01R\x7Fpayload\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[a\xFF\xFF\x85\x16`\0\x90\x81R`\x10` R`@\x80\x82 \x90Qa\x17\xE4\x90\x87\x90a3\xF9V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\0\x90\x81R\x92R\x90 Ua\x0B\0\x85\x85\x85\x85a\x1C\xB9V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18AW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90\x81\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x91a\x0B\x81\x91`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\xB9W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\x06W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x19'W`@Qc`\x03\xE8!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x82\x90U`\x06\x81\x90U`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x7Fv3\0Lz\"\x98i\xAE\xEA\x10\xDBO\xF3\xE5~;\x154\xAE\xB2\xC9\xE7,]\xB2_\x96X\x95\xC30\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\x99W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T\x82\x15\x15`\x01`\xA0\x1B\x02`\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x90U`@Q\x7F\xCD\x16,o\xC2B\x85\xBF\xBE9\x9E\xC0\xCC,\xE2\xC3\x80\xAD'\xD2\xEB\x1F\xCAA\x8Ce-\xF1%~~\t\x90a\r\xD1\x90\x83\x15\x15\x81R` \x01\x90V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x0E\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xB0\x1Ba\xFF\xFF\x86\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`\0\x90\x81R`\x0F` R`@\x90 a\x1A\x93\x90\x83\x83a-\x89V[P\x7F\xFAAHz\xD5\xD6r\x8F\x0B\x19'o\xA1\xED\xDC\x16U\x85x\xF5\x10\x9F\xC3\x9D-\xC3<20G\r\xAB\x83\x83\x83`@Qa\x1A\xC7\x93\x92\x91\x90a5xV[`@Q\x80\x91\x03\x90\xA1PPPV[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xE0V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\xE0V[a\x1B\xB3\x81a\x1C\xF4V[PV[`\x0ET`@Qc={/o`\xE2\x1B\x81Ra\xFF\xFF\x80\x87\x16`\x04\x83\x01R\x85\x16`$\x82\x01R0`D\x82\x01R`d\x81\x01\x83\x90R``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF5\xEC\xBD\xBC\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\x13W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1C'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1CO\x91\x90\x81\x01\x90a/ZV[\x95\x94PPPPPV[`@Qc3V\xAEE`\xE1\x1B\x81R0\x90cf\xAD\\\x8A\x90a\x1C\x81\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a5\xFDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\xAFW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0a\x1C\xC4\x82a \x9CV[`\x02T`\x04T\x91\x92Pa\x0B\0\x91`\x01`\xA0\x1B\x90\x91\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90`\x01`\x01`\xA0\x1B\x03\x162\x84a!dV[`\r\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x1D\x89W`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x1D\xBBW`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x1E\x13\x91`\x04\x01a4\xAFV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E+W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1E?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CO\x91\x90a2\xE3V[`\0\x80T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x1E}WP`\0\x91\x90PV[`\0T`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xC2W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1E\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xFA\x91\x90a/=V[\x92\x91PPV[``\x80a\x1F\x10\x83`\0\x01Qa\"\x19V[a\x1F\x1D\x84` \x01Qa\"\x19V[a\x1F*\x85`@\x01Qa\"\x19V[a\x1F7\x86``\x01Qa\"\x19V[a\x1FD\x87`\x80\x01Qa\"\xB1V[`@Q` \x01a\x1FX\x95\x94\x93\x92\x91\x90a4DV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[a\x1F\x9D`\x0E`\x16\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16\x823`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x87a\"\xE8V[PPV[a\x1F\xAB\x81\x83a6\xF8V[4\x14a\x1F\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Finsufficient token\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a KV[``\x91P[PP\x90P\x80a\x12\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Famount transfer failed\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[a \xCE`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a!\0`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a!\x0C\x84\x82a$\x1FV[\x90\x83R\x90Pa!\x1B\x84\x82a$\x1FV[` \x84\x01\x91\x90\x91R\x90Pa!/\x84\x82a$\x1FV[`@\x84\x01\x91\x90\x91R\x90Pa!C\x84\x82a$\x1FV[``\x84\x01\x91\x90\x91R\x90Pa!W\x84\x82a%PV[P`\x80\x83\x01RP\x92\x91PPV[`\x04T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a!\x92W`@Qc\x14@\xE0y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xA0\x1B\x90\x92\x04\x16\x14a!\xC8W`@Qc8v0O`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qa!\xE7W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`@Qcx\xD6\x0C\xD7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx\xD6\x0C\xD7\x90a\x1C\x81\x90\x84\x90\x86\x90`\x04\x01a5\x1BV[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\"rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\"\xA1W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\"\x7FV[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\"\xBF\x81a&]V[\x83`@Q` \x01a\"\xD1\x92\x91\x90a4\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[a\xFF\xFF\x86\x16`\0\x90\x81R`\x0F` R`@\x81 \x80Ta#\x06\x90a7JV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta#2\x90a7JV[\x80\x15a#\x7FW\x80`\x1F\x10a#TWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a#\x7FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#bW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q`\0\x14\x15a#\xA9W`@Qc\x02\x0B5\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0ET`@Qb\xC5\x801`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5\x801\0\x90\x84\x90a#\xE4\x90\x8B\x90\x86\x90\x8C\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a5\x96V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a#\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a$\x11W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0\x80\x83Q\x83` a$1\x91\x90a6\xF8V[\x11\x15\x80\x15a$HWPa$E\x83` a6\xF8V[\x83\x10[a$\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\t\xE0V[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a$\xD5W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa$\xB5V[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a%7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xE0V[\x80a%C\x85` a6\xF8V[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a%_\x85\x85a'*V[\x86Q\x90\x95P\x90\x91Pa%q\x82\x86a6\xF8V[\x11\x15\x80\x15a%\x87WPa%\x84\x81\x85a6\xF8V[\x84\x10[a%\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\t\xE0V[``\x81\x15\x80\x15a%\xFAW`@Q\x91P` \x82\x01`@Ra&DV[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a&3W\x80Q\x83R` \x92\x83\x01\x92\x01a&\x1BV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a&P\x83\x87a6\xF8V[\x93P\x93PPP\x92P\x92\x90PV[```\xFD\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a&\x90W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91Ra\x1E\xFAV[a\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a&\xE0Wa&\xB0`\xFD`\xF8\x1Ba)0V[a&\xB9\x83a)WV[`@Q` \x01a&\xCA\x92\x91\x90a4\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a'\x0BWa'\x02`\x7F`\xF9\x1Ba)0V[a&\xB9\x83a)\x9AV[a'\x1C`\x01`\x01`\xF8\x1B\x03\x19a)0V[a&\xB9\x83a)\xDDV[\x91\x90PV[`\0\x80`\0a'9\x85\x85a* V[\x94P\x90P`\0`\xFD`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a'\xD2Wa'_\x86\x86a*\xA8V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a'zWPa\xFF\xFF\x81\x11\x15[a'\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\t\xE0V[\x92P\x83\x91Pa%I\x90PV[`\x7F`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a(]Wa'\xF2\x86\x86a+aV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a(\x11WPc\xFF\xFF\xFF\xFF\x81\x11\x15[a'\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\t\xE0V[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x14\x15a(\xDAWa(y\x86\x86a,3V[\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a'\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\t\xE0V[P`\xF8\x81\x90\x1C`\xFD\x81\x10a'\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\t\xE0V[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a\x1E\xFAV[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a)\x8AW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a)hV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a)\xCDW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a)\xABV[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a*\x10W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a)\xEEV[PPP`(\x81\x01`@R\x92\x91PPV[`\0\x80\x83Q\x83`\x01a*2\x91\x90a6\xF8V[\x11\x15\x80\x15a*IWPa*F\x83`\x01a6\xF8V[\x83\x10[a*\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\t\xE0V[\x83\x83\x01` \x01Q\x80a%C\x85`\x01a6\xF8V[`\0\x80\x83Q\x83`\x02a*\xBA\x91\x90a6\xF8V[\x11\x15\x80\x15a*\xD1WPa*\xCE\x83`\x02a6\xF8V[\x83\x10[a+(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\t\xE0V[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a%C\x91\x90a6\xF8V[`\0\x80\x83Q\x83`\x04a+s\x91\x90a6\xF8V[\x11\x15\x80\x15a+\x8AWPa+\x87\x83`\x04a6\xF8V[\x83\x10[a+\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\t\xE0V[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a,\x16W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa+\xF6V[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a%C\x85`\x04a6\xF8V[`\0\x80\x83Q\x83`\x08a,E\x91\x90a6\xF8V[\x11\x15\x80\x15a,\\WPa,Y\x83`\x08a6\xF8V[\x83\x10[a,\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\t\xE0V[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a,\xE8W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa,\xC8V[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a%C\x85`\x08a6\xF8V[\x82\x80Ta-\x11\x90a7JV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a-3W`\0\x85Ua-yV[\x82`\x1F\x10a-LW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua-yV[\x82\x80\x01`\x01\x01\x85U\x82\x15a-yW\x91\x82\x01[\x82\x81\x11\x15a-yW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a-^V[Pa-\x85\x92\x91Pa-\xFDV[P\x90V[\x82\x80Ta-\x95\x90a7JV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a-\xB7W`\0\x85Ua-yV[\x82`\x1F\x10a-\xD0W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua-yV[\x82\x80\x01`\x01\x01\x85U\x82\x15a-yW\x91\x82\x01[\x82\x81\x11\x15a-yW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a-\xE2V[[\x80\x82\x11\x15a-\x85W`\0\x81U`\x01\x01a-\xFEV[`\0a.%a. \x84a6\xD0V[a6\x9FV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a.9W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a.bW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.zW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a%IW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a.\xA3W`\0\x80\xFD[a.\xB2\x83\x835` \x85\x01a.\x12V[\x93\x92PPPV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'%W`\0\x80\xFD[\x805a\xFF\xFF\x81\x16\x81\x14a'%W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'%W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a/\x15W`\0\x80\xFD[\x815a.\xB2\x81a7\x9BV[`\0` \x82\x84\x03\x12\x15a/2W`\0\x80\xFD[\x815a.\xB2\x81a7\xB0V[`\0` \x82\x84\x03\x12\x15a/OW`\0\x80\xFD[\x81Qa.\xB2\x81a7\xB0V[`\0` \x82\x84\x03\x12\x15a/lW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\x83W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a/\x94W`\0\x80\xFD[\x80Qa/\xA2a. \x82a6\xD0V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a/\xB7W`\0\x80\xFD[a\x1CO\x82` \x83\x01` \x86\x01a7\x1EV[`\0` \x82\x84\x03\x12\x15a/\xDAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a/\xF2W`\0\x80\xFD[\x90\x83\x01\x90a\x01\0\x82\x86\x03\x12\x15a0\x07W`\0\x80\xFD[a0\x0Fa6uV[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01Ra03``\x84\x01a.\xB9V[``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a0JW`\0\x80\xFD[a0V\x87\x82\x86\x01a.\x92V[`\x80\x83\x01RP`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015`\xE0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a0\x98W`\0\x80\xFD[a.\xB2\x82a.\xD9V[`\0\x80`@\x83\x85\x03\x12\x15a0\xB4W`\0\x80\xFD[a0\xBD\x83a.\xD9V[\x91P` \x83\x015a0\xCD\x81a7\x9BV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a0\xEDW`\0\x80\xFD[a0\xF6\x84a.\xD9V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x12W`\0\x80\xFD[a1\x1E\x86\x82\x87\x01a.PV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a1@W`\0\x80\xFD[a1I\x84a.\xD9V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1eW`\0\x80\xFD[a1q\x86\x82\x87\x01a.\x92V[\x92PPa1\x80`@\x85\x01a.\xEBV[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a1\x9FW`\0\x80\xFD[a1\xA8\x85a.\xD9V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a1\xC5W`\0\x80\xFD[a1\xD1\x88\x83\x89\x01a.\x92V[\x94Pa1\xDF`@\x88\x01a.\xEBV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a1\xF5W`\0\x80\xFD[Pa2\x02\x87\x82\x88\x01a.\x92V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a2$W`\0\x80\xFD[a2-\x85a.\xD9V[\x93Pa2;` \x86\x01a.\xD9V[\x92P`@\x85\x015a2K\x81a7\x9BV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a2sW`\0\x80\xFD[a2|\x86a.\xD9V[\x94Pa2\x8A` \x87\x01a.\xD9V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xADW`\0\x80\xFD[a2\xB9\x88\x82\x89\x01a.PV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a2\xDCW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a2\xF5W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a3\x0FW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a33W`\0\x80\xFD[a3<\x84a.\xEBV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3XW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a3iW`\0\x80\xFD[a3x\x86\x825` \x84\x01a.\x12V[\x92PP`@\x84\x015a3\x89\x81a7\x9BV[\x80\x91PP\x92P\x92P\x92V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x81Q\x80\x84Ra3\xD5\x81` \x86\x01` \x86\x01a7\x1EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82Qa4\x0B\x81\x84` \x87\x01a7\x1EV[\x91\x90\x91\x01\x92\x91PPV[`\0\x83Qa4'\x81\x84` \x88\x01a7\x1EV[\x83Q\x90\x83\x01\x90a4;\x81\x83` \x88\x01a7\x1EV[\x01\x94\x93PPPPV[`\0\x86Qa4V\x81\x84` \x8B\x01a7\x1EV[\x86Q\x90\x83\x01\x90a4j\x81\x83` \x8B\x01a7\x1EV[\x86Q\x91\x01\x90a4}\x81\x83` \x8A\x01a7\x1EV[\x85Q\x91\x01\x90a4\x90\x81\x83` \x89\x01a7\x1EV[\x84Q\x91\x01\x90a4\xA3\x81\x83` \x88\x01a7\x1EV[\x01\x97\x96PPPPPPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a4\xD7W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a4\xB8V[PPP\x92\x91PPV[` \x81R`\0a.\xB2` \x83\x01\x84a3\xBDV[` \x81\x01`\x02\x83\x10a5\x15WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra5_`\xE0\x84\x01\x82a3\xBDV[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[a\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x1CO`@\x83\x01\x84\x86a3\x94V[a\xFF\xFF\x87\x16\x81R`\xC0` \x82\x01R`\0a5\xB3`\xC0\x83\x01\x88a3\xBDV[\x82\x81\x03`@\x84\x01Ra5\xC5\x81\x88a3\xBDV[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16``\x86\x01R\x86\x16`\x80\x85\x01R\x83\x81\x03`\xA0\x85\x01R\x90Pa5\xF0\x81\x85a3\xBDV[\x99\x98PPPPPPPPPV[a\xFF\xFF\x85\x16\x81R`\x80` \x82\x01R`\0a6\x1A`\x80\x83\x01\x86a3\xBDV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra6<\x81\x85a3\xBDV[\x97\x96PPPPPPPV[`\0a\xFF\xFF\x80\x88\x16\x83R\x80\x87\x16` \x84\x01RP\x84`@\x83\x01R`\x80``\x83\x01Ra6<`\x80\x83\x01\x84\x86a3\x94V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a6\x99Wa6\x99a7\x85V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a6\xC8Wa6\xC8a7\x85V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a6\xEAWa6\xEAa7\x85V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82\x19\x82\x11\x15a7\x19WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[`\0[\x83\x81\x10\x15a79W\x81\x81\x01Q\x83\x82\x01R` \x01a7!V[\x83\x81\x11\x15a\x12\xBCWPP`\0\x91\x01RV[`\x01\x81\x81\x1C\x90\x82\x16\x80a7^W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a7\x7FWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xB3W`\0\x80\xFD[\x80\x15\x15\x81\x14a\x1B\xB3W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 g\xF4\xCA\n\xA6\xCB\x86\xBD\xD9\xE9|\x14\xB5\x90\x89=\xA03a\x02\x8E\xF4\x08\x98\x0F\n\xE26H\xE4~\xFCdsolcC\0\x08\x07\x003";
    /// The deployed bytecode of the contract.
    pub static MYSTIKOV2LAYERZEROMAIN_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MystikoV2LayerZeroMain<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoV2LayerZeroMain<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoV2LayerZeroMain<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoV2LayerZeroMain<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoV2LayerZeroMain<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MystikoV2LayerZeroMain))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> MystikoV2LayerZeroMain<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                MYSTIKOV2LAYERZEROMAIN_ABI.clone(),
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
                MYSTIKOV2LAYERZEROMAIN_ABI.clone(),
                MYSTIKOV2LAYERZEROMAIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, MystikoV2LayerZeroMainEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for MystikoV2LayerZeroMain<M> {
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
    pub enum MystikoV2LayerZeroMainErrors {
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
    impl ::ethers_core::abi::AbiDecode for MystikoV2LayerZeroMainErrors {
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
    impl ::ethers_core::abi::AbiEncode for MystikoV2LayerZeroMainErrors {
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
    impl ::ethers_contract::ContractRevert for MystikoV2LayerZeroMainErrors {
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
    impl ::core::fmt::Display for MystikoV2LayerZeroMainErrors {
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
    impl ::core::convert::From<::std::string::String> for MystikoV2LayerZeroMainErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AmountLessThanZero> for MystikoV2LayerZeroMainErrors {
        fn from(value: AmountLessThanZero) -> Self {
            Self::AmountLessThanZero(value)
        }
    }
    impl ::core::convert::From<AmountTooLarge> for MystikoV2LayerZeroMainErrors {
        fn from(value: AmountTooLarge) -> Self {
            Self::AmountTooLarge(value)
        }
    }
    impl ::core::convert::From<AmountTooSmall> for MystikoV2LayerZeroMainErrors {
        fn from(value: AmountTooSmall) -> Self {
            Self::AmountTooSmall(value)
        }
    }
    impl ::core::convert::From<BridgeFeeTooFew> for MystikoV2LayerZeroMainErrors {
        fn from(value: BridgeFeeTooFew) -> Self {
            Self::BridgeFeeTooFew(value)
        }
    }
    impl ::core::convert::From<CallIsNotLzApp> for MystikoV2LayerZeroMainErrors {
        fn from(value: CallIsNotLzApp) -> Self {
            Self::CallIsNotLzApp(value)
        }
    }
    impl ::core::convert::From<CommitmentHashIncorrect> for MystikoV2LayerZeroMainErrors {
        fn from(value: CommitmentHashIncorrect) -> Self {
            Self::CommitmentHashIncorrect(value)
        }
    }
    impl ::core::convert::From<DepositsDisabled> for MystikoV2LayerZeroMainErrors {
        fn from(value: DepositsDisabled) -> Self {
            Self::DepositsDisabled(value)
        }
    }
    impl ::core::convert::From<DestinationChainIsNotTrusted> for MystikoV2LayerZeroMainErrors {
        fn from(value: DestinationChainIsNotTrusted) -> Self {
            Self::DestinationChainIsNotTrusted(value)
        }
    }
    impl ::core::convert::From<ExecutorFeeTooFew> for MystikoV2LayerZeroMainErrors {
        fn from(value: ExecutorFeeTooFew) -> Self {
            Self::ExecutorFeeTooFew(value)
        }
    }
    impl ::core::convert::From<FromChainIdNotMatched> for MystikoV2LayerZeroMainErrors {
        fn from(value: FromChainIdNotMatched) -> Self {
            Self::FromChainIdNotMatched(value)
        }
    }
    impl ::core::convert::From<FromProxyAddressNotMatched> for MystikoV2LayerZeroMainErrors {
        fn from(value: FromProxyAddressNotMatched) -> Self {
            Self::FromProxyAddressNotMatched(value)
        }
    }
    impl ::core::convert::From<HashKGreaterThanFieldSize> for MystikoV2LayerZeroMainErrors {
        fn from(value: HashKGreaterThanFieldSize) -> Self {
            Self::HashKGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<Invalid> for MystikoV2LayerZeroMainErrors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<MinAmountGreaterThanMaxAmount> for MystikoV2LayerZeroMainErrors {
        fn from(value: MinAmountGreaterThanMaxAmount) -> Self {
            Self::MinAmountGreaterThanMaxAmount(value)
        }
    }
    impl ::core::convert::From<NoStoredMessage> for MystikoV2LayerZeroMainErrors {
        fn from(value: NoStoredMessage) -> Self {
            Self::NoStoredMessage(value)
        }
    }
    impl ::core::convert::From<NotChanged> for MystikoV2LayerZeroMainErrors {
        fn from(value: NotChanged) -> Self {
            Self::NotChanged(value)
        }
    }
    impl ::core::convert::From<OnlyOperator> for MystikoV2LayerZeroMainErrors {
        fn from(value: OnlyOperator) -> Self {
            Self::OnlyOperator(value)
        }
    }
    impl ::core::convert::From<RandomSGreaterThanFieldSize> for MystikoV2LayerZeroMainErrors {
        fn from(value: RandomSGreaterThanFieldSize) -> Self {
            Self::RandomSGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<RollupFeeToFew> for MystikoV2LayerZeroMainErrors {
        fn from(value: RollupFeeToFew) -> Self {
            Self::RollupFeeToFew(value)
        }
    }
    impl ::core::convert::From<SanctionedAddress> for MystikoV2LayerZeroMainErrors {
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
    pub enum MystikoV2LayerZeroMainEvents {
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
    impl ::ethers_contract::EthLogDecode for MystikoV2LayerZeroMainEvents {
        fn decode_log(log: &::ethers_core::abi::RawLog) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = CommitmentCrossChainFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::CommitmentCrossChainFilter(decoded));
            }
            if let Ok(decoded) = DepositAmountLimitsFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::DepositAmountLimitsFilter(decoded));
            }
            if let Ok(decoded) = DepositsDisabledFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::DepositsDisabledFilter(decoded));
            }
            if let Ok(decoded) = MessageFailedFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::MessageFailedFilter(decoded));
            }
            if let Ok(decoded) = MinBridgeFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::MinBridgeFeeFilter(decoded));
            }
            if let Ok(decoded) = MinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::MinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PeerMinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::PeerMinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = PeerMinRollupFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::PeerMinRollupFeeFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::SanctionsListFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedRemoteFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroMainEvents::SetTrustedRemoteFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MystikoV2LayerZeroMainEvents {
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
    impl ::core::convert::From<CommitmentCrossChainFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: CommitmentCrossChainFilter) -> Self {
            Self::CommitmentCrossChainFilter(value)
        }
    }
    impl ::core::convert::From<DepositAmountLimitsFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: DepositAmountLimitsFilter) -> Self {
            Self::DepositAmountLimitsFilter(value)
        }
    }
    impl ::core::convert::From<DepositsDisabledFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: DepositsDisabledFilter) -> Self {
            Self::DepositsDisabledFilter(value)
        }
    }
    impl ::core::convert::From<MessageFailedFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: MessageFailedFilter) -> Self {
            Self::MessageFailedFilter(value)
        }
    }
    impl ::core::convert::From<MinBridgeFeeFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: MinBridgeFeeFilter) -> Self {
            Self::MinBridgeFeeFilter(value)
        }
    }
    impl ::core::convert::From<MinExecutorFeeFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: MinExecutorFeeFilter) -> Self {
            Self::MinExecutorFeeFilter(value)
        }
    }
    impl ::core::convert::From<OperatorChangedFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: OperatorChangedFilter) -> Self {
            Self::OperatorChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PeerMinExecutorFeeFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: PeerMinExecutorFeeFilter) -> Self {
            Self::PeerMinExecutorFeeFilter(value)
        }
    }
    impl ::core::convert::From<PeerMinRollupFeeFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: PeerMinRollupFeeFilter) -> Self {
            Self::PeerMinRollupFeeFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: SanctionsCheckFilter) -> Self {
            Self::SanctionsCheckFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsListFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: SanctionsListFilter) -> Self {
            Self::SanctionsListFilter(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteFilter> for MystikoV2LayerZeroMainEvents {
        fn from(value: SetTrustedRemoteFilter) -> Self {
            Self::SetTrustedRemoteFilter(value)
        }
    }
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
    pub enum MystikoV2LayerZeroMainCalls {
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
    impl ::ethers_core::abi::AbiDecode for MystikoV2LayerZeroMainCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
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
    impl ::ethers_core::abi::AbiEncode for MystikoV2LayerZeroMainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
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
    impl ::core::fmt::Display for MystikoV2LayerZeroMainCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
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
    impl ::core::convert::From<AssetTypeCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: AssetTypeCall) -> Self {
            Self::AssetType(value)
        }
    }
    impl ::core::convert::From<BridgeProxyAddressCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: BridgeProxyAddressCall) -> Self {
            Self::BridgeProxyAddress(value)
        }
    }
    impl ::core::convert::From<BridgeTypeCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: BridgeTypeCall) -> Self {
            Self::BridgeType(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: ChangeOperatorCall) -> Self {
            Self::ChangeOperator(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DisableSanctionsCheckCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: DisableSanctionsCheckCall) -> Self {
            Self::DisableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<EnableSanctionsCheckCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: EnableSanctionsCheckCall) -> Self {
            Self::EnableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<FailedMessagesCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: FailedMessagesCall) -> Self {
            Self::FailedMessages(value)
        }
    }
    impl ::core::convert::From<ForceResumeReceiveCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: ForceResumeReceiveCall) -> Self {
            Self::ForceResumeReceive(value)
        }
    }
    impl ::core::convert::From<GetAssociatedCommitmentPoolCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: GetAssociatedCommitmentPoolCall) -> Self {
            Self::GetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<GetConfigCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: GetConfigCall) -> Self {
            Self::GetConfig(value)
        }
    }
    impl ::core::convert::From<GetMaxAmountCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: GetMaxAmountCall) -> Self {
            Self::GetMaxAmount(value)
        }
    }
    impl ::core::convert::From<GetMinAmountCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: GetMinAmountCall) -> Self {
            Self::GetMinAmount(value)
        }
    }
    impl ::core::convert::From<GetMinBridgeFeeCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: GetMinBridgeFeeCall) -> Self {
            Self::GetMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<GetMinExecutorFeeCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: GetMinExecutorFeeCall) -> Self {
            Self::GetMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<GetPeerMinExecutorFeeCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: GetPeerMinExecutorFeeCall) -> Self {
            Self::GetPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<GetPeerMinRollupFeeCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: GetPeerMinRollupFeeCall) -> Self {
            Self::GetPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<IsDepositsDisabledCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: IsDepositsDisabledCall) -> Self {
            Self::IsDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<IsTrustedRemoteCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: IsTrustedRemoteCall) -> Self {
            Self::IsTrustedRemote(value)
        }
    }
    impl ::core::convert::From<LocalLayerZeroChainIdCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: LocalLayerZeroChainIdCall) -> Self {
            Self::LocalLayerZeroChainId(value)
        }
    }
    impl ::core::convert::From<LzEndpointCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: LzEndpointCall) -> Self {
            Self::LzEndpoint(value)
        }
    }
    impl ::core::convert::From<LzReceiveCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: LzReceiveCall) -> Self {
            Self::LzReceive(value)
        }
    }
    impl ::core::convert::From<NonblockingLzReceiveCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: NonblockingLzReceiveCall) -> Self {
            Self::NonblockingLzReceive(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PeerChainIdCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: PeerChainIdCall) -> Self {
            Self::PeerChainId(value)
        }
    }
    impl ::core::convert::From<PeerChainNameCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: PeerChainNameCall) -> Self {
            Self::PeerChainName(value)
        }
    }
    impl ::core::convert::From<PeerContractCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: PeerContractCall) -> Self {
            Self::PeerContract(value)
        }
    }
    impl ::core::convert::From<PeerLayerZeroChainIdCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: PeerLayerZeroChainIdCall) -> Self {
            Self::PeerLayerZeroChainId(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RetryMessageCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: RetryMessageCall) -> Self {
            Self::RetryMessage(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SanctionsCheckCall) -> Self {
            Self::SanctionsCheck(value)
        }
    }
    impl ::core::convert::From<SanctionsListCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SanctionsListCall) -> Self {
            Self::SanctionsList(value)
        }
    }
    impl ::core::convert::From<SetAssociatedCommitmentPoolCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetAssociatedCommitmentPoolCall) -> Self {
            Self::SetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<SetBridgeProxyAddressCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetBridgeProxyAddressCall) -> Self {
            Self::SetBridgeProxyAddress(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetDepositsDisabledCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetDepositsDisabledCall) -> Self {
            Self::SetDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<SetEndpointCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetEndpointCall) -> Self {
            Self::SetEndpoint(value)
        }
    }
    impl ::core::convert::From<SetMinBridgeFeeCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetMinBridgeFeeCall) -> Self {
            Self::SetMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<SetMinExecutorFeeCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetMinExecutorFeeCall) -> Self {
            Self::SetMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<SetPeerContractCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetPeerContractCall) -> Self {
            Self::SetPeerContract(value)
        }
    }
    impl ::core::convert::From<SetPeerMinExecutorFeeCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetPeerMinExecutorFeeCall) -> Self {
            Self::SetPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<SetPeerMinRollupFeeCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetPeerMinRollupFeeCall) -> Self {
            Self::SetPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<SetReceiveVersionCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetReceiveVersionCall) -> Self {
            Self::SetReceiveVersion(value)
        }
    }
    impl ::core::convert::From<SetSendVersionCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetSendVersionCall) -> Self {
            Self::SetSendVersion(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: SetTrustedRemoteCall) -> Self {
            Self::SetTrustedRemote(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TrustedRemoteLookupCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: TrustedRemoteLookupCall) -> Self {
            Self::TrustedRemoteLookup(value)
        }
    }
    impl ::core::convert::From<UpdateDepositAmountLimitsCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: UpdateDepositAmountLimitsCall) -> Self {
            Self::UpdateDepositAmountLimits(value)
        }
    }
    impl ::core::convert::From<UpdateSanctionsListAddressCall> for MystikoV2LayerZeroMainCalls {
        fn from(value: UpdateSanctionsListAddressCall) -> Self {
            Self::UpdateSanctionsListAddress(value)
        }
    }
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
