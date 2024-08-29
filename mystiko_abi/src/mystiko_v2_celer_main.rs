pub use mystiko_v2_celer_main::*;
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
pub mod mystiko_v2_celer_main {
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("executeMessage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("executeMessage"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_message"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_executor"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
            ]),
            events: ::core::convert::From::from([(
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
            )]),
            errors: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("ExecutorFeeTooFew"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ExecutorFeeTooFew"),
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
                    ::std::borrow::ToOwned::to_owned("NotSupport"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotSupport"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("SanctionedAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SanctionedAddress"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SenderIsNotBridgeProxy"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SenderIsNotBridgeProxy",),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MYSTIKOV2CELERMAIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90U4\x80\x15a\0\x1DW`\0\x80\xFD[P`@Qa&J8\x03\x80a&J\x839\x81\x01`@\x81\x90Ra\0<\x91a\x01\x1DV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x08\x80T\x95\x87\x16\x95\x82\x16\x95\x90\x95\x17\x90\x94U\x81Q`\x03U` \x80\x83\x01Q`\x04U`@\x90\x92\x01Q`\x05U\x80Q`\x06U\x01Q`\x07U`\t\x80T\x91\x90\x93\x16\x91\x16\x17\x90Ua\x01\xD7V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xB4W`\0\x80\xFD[PV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xE7WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xE7WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\0\x85\x87\x03a\x01\0\x81\x12\x15a\x017W`\0\x80\xFD[\x86Qa\x01B\x81a\0\x9FV[` \x88\x01Q\x90\x96Pa\x01S\x81a\0\x9FV[`@\x88\x01Q\x90\x95Pa\x01d\x81a\0\x9FV[\x93P```_\x19\x82\x01\x12\x15a\x01xW`\0\x80\xFD[a\x01\x80a\0\xB7V[``\x88\x01Q\x81R`\x80\x88\x01Q` \x82\x01R`\xA0\x88\x01Q`@\x80\x83\x01\x91\x90\x91R\x90\x93P`\xBF\x19\x82\x01\x12\x15a\x01\xB2W`\0\x80\xFD[Pa\x01\xBBa\0\xEDV[`\xC0\x87\x01Q\x81R`\xE0\x90\x96\x01Q` \x87\x01RP\x92\x95\x91\x94P\x92\x90V[a$d\x80a\x01\xE6`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x81W`\x005`\xE0\x1C\x80c\xBCXw\x06\x11a\0\xD1W\x80c\xDD\xAC]\xC1\x11a\0\x8AW\x80c\xEF\xBF\xB2\xAE\x11a\0dW\x80c\xEF\xBF\xB2\xAE\x14a\x04\tW\x80c\xF4\xAD\x17\xC6\x14a\x04\x1EW\x80c\xFAu\x0FV\x14a\x043W\x80c\xFB>=s\x14a\x04TW`\0\x80\xFD[\x80c\xDD\xAC]\xC1\x14a\x03\xBFW\x80c\xE0at\xE4\x14a\x03\xD4W\x80c\xEDn\xA3:\x14a\x03\xF4W`\0\x80\xFD[\x80c\xBCXw\x06\x14a\x03\x17W\x80c\xCB\\\x02\x9A\x14a\x03,W\x80c\xCB\xE3B\x85\x14a\x03?W\x80c\xCD\xFC\xEE\xBA\x14a\x03UW\x80c\xCF\xC7\xE2\xDA\x14a\x03\x94W\x80c\xD0\xB46\xBD\x14a\x03\xA9W`\0\x80\xFD[\x80cB.\0(\x11a\x01>W\x80cd\x0C\x0B6\x11a\x01\x18W\x80cd\x0C\x0B6\x14a\x02\xB6W\x80c\x82[_\x8D\x14a\x02\xCCW\x80c\x9A\x03cl\x14a\x02\xE1W\x80c\x9Cd\x9F\xDF\x14a\x02\xF4W`\0\x80\xFD[\x80cB.\0(\x14a\x02iW\x80cM\xDEo\xBC\x14a\x02\x8BW\x80cN<\x10\xB7\x14a\x02\xA1W`\0\x80\xFD[\x80c\x0B\xA9Y\t\x14a\x01\x86W\x80c\x1B\xA4l\xFD\x14a\x01\xAEW\x80c!\xE3-U\x14a\x01\xD6W\x80c$!\xE1U\x14a\x01\xF6W\x80c,\xD2mE\x14a\x02-W\x80c?\xE34z\x14a\x02MW[`\0\x80\xFD[4\x80\x15a\x01\x92W`\0\x80\xFD[Pa\x01\x9Ba\x04jV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xBAW`\0\x80\xFD[P`\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA5V[4\x80\x15a\x01\xE2W`\0\x80\xFD[P`\x02Ta\x01\xBE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\x02W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rd1\xB2\xB62\xB9`\xD9\x1B` \x82\x01R[`@Qa\x01\xA5\x91\x90a\x1C|V[4\x80\x15a\x029W`\0\x80\xFD[P`\x08Ta\x01\xBE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02YW`\0\x80\xFD[P`\x01`@Qa\x01\xA5\x91\x90a\x1C\x96V[4\x80\x15a\x02uW`\0\x80\xFD[Pa\x02\x89a\x02\x846`\x04a\x1D\xC6V[a\x04\xF3V[\0[4\x80\x15a\x02\x97W`\0\x80\xFD[Pa\x01\x9B`\x05T\x81V[4\x80\x15a\x02\xADW`\0\x80\xFD[Pa\x02 a\x05\x98V[4\x80\x15a\x02\xC2W`\0\x80\xFD[Pa\x01\x9B`\x06T\x81V[4\x80\x15a\x02\xD8W`\0\x80\xFD[Pa\x01\x9Ba\x06&V[a\x02\x89a\x02\xEF6`\x04a\x1F?V[a\x06\xADV[a\x03\x07a\x03\x026`\x04a\x1F{V[a\x06\xC6V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA5V[4\x80\x15a\x03#W`\0\x80\xFD[Pa\x03\x07a\x07PV[a\x02\x89a\x03:6`\x04a &V[a\x07\xC3V[4\x80\x15a\x03KW`\0\x80\xFD[Pa\x01\x9B`\x07T\x81V[4\x80\x15a\x03aW`\0\x80\xFD[P`\0Ta\x03|\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA5V[4\x80\x15a\x03\xA0W`\0\x80\xFD[Pa\x01\x9Ba\x0B\xEEV[4\x80\x15a\x03\xB5W`\0\x80\xFD[Pa\x01\x9B`\x04T\x81V[4\x80\x15a\x03\xCBW`\0\x80\xFD[Pa\x01\xBEa\x0CuV[4\x80\x15a\x03\xE0W`\0\x80\xFD[P`\tTa\x01\xBE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\0W`\0\x80\xFD[Pa\x03\x07a\r\x14V[4\x80\x15a\x04\x15W`\0\x80\xFD[Pa\x01\x9Ba\r]V[4\x80\x15a\x04*W`\0\x80\xFD[Pa\x01\x9Ba\r\xE4V[4\x80\x15a\x04?W`\0\x80\xFD[P`\0Ta\x03\x07\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x04`W`\0\x80\xFD[Pa\x01\x9B`\x03T\x81V[`\tT`@QcG:\x061`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG:\x061\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xDB\x91\x90a \x96V[\x90P\x80\x15a\x04\xE9W\x80a\x04\xEDV[`\x04T[\x91PP\x90V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x05\x1EW`@Qcm\x8F\x11Q`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\xA8\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U` \x81\x01Q`\x01\x90a\x05]\x90\x82a!8V[P`@\x01Q`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UV[`\x01\x80Ta\x05\xA5\x90a \xAFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xD1\x90a \xAFV[\x80\x15a\x06\x1EW\x80`\x1F\x10a\x05\xF3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x1EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x01W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\tT`@Qca\x86<\x03`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC3\x0Cx\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x97\x91\x90a \x96V[\x90P\x80\x15a\x06\xA5W\x80a\x04\xEDV[PP`\x07T\x90V[`@Qc\xE7\xA2O\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xF4W`@Qc=\xCA\x01\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x075\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0Ek\x92PPPV[\x90Pa\x07C\x86\x88\x85\x84a\x0F3V[P`\x01\x96\x95PPPPPPV[`\tT`@\x80Qc^,;\x83`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xBCXw\x06\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x07\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xBE\x91\x90a!\xF6V[\x90P\x90V[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08/\x91\x90a!\xF6V[\x15a\x08MW`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xBCXw\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC4\x91\x90a!\xF6V[\x15a\t\x91W`\0`@Q\x80`\x80\x01`@R\x802`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x08\xED`\0\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x86\x90R`@\x91\x82\x01\x85\x90R`\tT\x91Qc\x84\x9E\x8B\x9F`\xE0\x1B\x81R\x92\x93P\x16\x90c\x84\x9E\x8B\x9F\x90a\t1\x90\x84\x90`\x04\x01a\"\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tNW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tr\x91\x90a!\xF6V[a\t\x8FW`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[a\t\x99a\x0B\xEEV[\x83Q\x10\x15a\t\xBAW`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xC2a\x04jV[\x83Q\x11\x15a\t\xE3W`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xEBa\r]V[\x83`\xA0\x01Q\x10\x15a\n\x0FW`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x17a\r\xE4V[\x83`\xC0\x01Q\x10\x15a\n;W`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\nCa\x06&V[\x83`\xE0\x01Q\x10\x15a\ngW`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\n\x80\x84`@\x01Q\x85`\0\x01Q\x86``\x01Qa\x10%V[\x90P\x80\x84` \x01Q\x14a\n\xA6W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT`@Qc\xDFY/}`\xE0\x1B\x81R2`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x12\x91\x90a!\xF6V[\x15a\x0B0W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x90\x82\x01R`\xC0\x86\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x85\x01Q``\x82\x01R`\x80\x80\x86\x01Q\x90\x82\x01R`\0a\x0Bs\x82a\x11*V[\x90Pa\x0B\x83\x86`\xA0\x01Q\x82a\x11\x99V[a\x0B\xB8a\x0B\x8Ea\x0CuV[`\xE0\x88\x01Q`\xC0\x89\x01Q\x89Qa\x0B\xA4\x91\x90a\"aV[a\x0B\xAE\x91\x90a\"aV[\x88`\xA0\x01Qa\x12\x1EV[` \x86\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPPPV[`\tT`@QcU%\x98I`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cU%\x98I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C_\x91\x90a \x96V[\x90P\x80\x15a\x0CmW\x80a\x04\xEDV[PP`\x03T\x90V[`\tT`@QcA\xFBiy`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cA\xFBiy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE6\x91\x90a\"\x82V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\r\x0FW`@Qc\x06\xF3\xD63`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x9AW=`\0\x80>=`\0\xFD[`\tT`@Qc{R\xB0\r`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c{R\xB0\r\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCE\x91\x90a \x96V[\x90P\x80\x15a\r\xDCW\x80a\x04\xEDV[PP`\x05T\x90V[`\tT`@QcO1O\x9D`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9Eb\x9F:\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EU\x91\x90a \x96V[\x90P\x80\x15a\x0EcW\x80a\x04\xEDV[PP`\x06T\x90V[a\x0E\x9D`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\x0E\xCF`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\x0E\xDB\x84\x82a\x13\x12V[\x90\x83R\x90Pa\x0E\xEA\x84\x82a\x13\x12V[` \x84\x01\x91\x90\x91R\x90Pa\x0E\xFE\x84\x82a\x13\x12V[`@\x84\x01\x91\x90\x91R\x90Pa\x0F\x12\x84\x82a\x13\x12V[``\x84\x01\x91\x90\x91R\x90Pa\x0F&\x84\x82a\x14CV[P`\x80\x83\x01RP\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x0FaW`@Qc\x1BI^\xCF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`@\x1B\x03\x85\x81\x16`\x01`\xA8\x1B\x90\x92\x04\x16\x14a\x0F\x96W`@Qc7;\xC1!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x03a\x0F\xB8W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xC0a\x0CuV[`\x01`\x01`\xA0\x1B\x03\x16cx\xD6\x0C\xD7\x82\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xED\x92\x91\x90a\"\x9FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x07W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x1BW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x10hW`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83`\x01`\x01`\x80\x1B\x03\x16\x10a\x10\x91W`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90R`\x01`\x01`\x80\x1B\x03\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x10\xE0\x91`\x04\x01a\"\xFBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11!\x91\x90a \x96V[\x95\x94PPPPPV[``\x80a\x11:\x83`\0\x01Qa\x15PV[a\x11G\x84` \x01Qa\x15PV[a\x11T\x85`@\x01Qa\x15PV[a\x11a\x86``\x01Qa\x15PV[a\x11n\x87`\x80\x01Qa\x15\xE8V[`@Q` \x01a\x11\x82\x95\x94\x93\x92\x91\x90a#,V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[`\x08T`\x02T`\0T`@QcO\x9Er\xAD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93c\x9F<\xE5Z\x93\x87\x93a\x11\xE8\x93\x91\x90\x92\x16\x91`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x16\x90\x87\x90`\x04\x01a#\x97V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x12\x01W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\x15W=`\0\x80>=`\0\xFD[PPPPPPPV[a\x12(\x81\x83a\"aV[4\x14a\x12pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq4\xB79\xBA\xB334\xB1\xB4\xB2\xB7:\x10:7\xB5\xB2\xB7`q\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x12\xBDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x12\xC2V[``\x91P[PP\x90P\x80a\x13\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x18[[\xDD[\x9D\x08\x1D\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`R\x1B`D\x82\x01R`d\x01a\x12gV[PPPPV[`\0\x80\x83Q\x83` a\x13$\x91\x90a\"aV[\x11\x15\x80\x15a\x13;WPa\x138\x83` a\"aV[\x83\x10[a\x13\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\x12gV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x13\xC8W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x13\xA8V[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x14*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x12gV[\x80a\x146\x85` a\"aV[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a\x14R\x85\x85a\x16\x1FV[\x86Q\x90\x95P\x90\x91Pa\x14d\x82\x86a\"aV[\x11\x15\x80\x15a\x14zWPa\x14w\x81\x85a\"aV[\x84\x10[a\x14\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\x12gV[``\x81\x15\x80\x15a\x14\xEDW`@Q\x91P` \x82\x01`@Ra\x157V[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a\x15&W\x80Q\x83R` \x92\x83\x01\x92\x01a\x15\x0EV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a\x15C\x83\x87a\"aV[\x93P\x93PPP\x92P\x92\x90PV[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x15\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\x12gV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\x15\xD8W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x15\xB6V[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\x15\xF6\x81a\x17\x92V[\x83`@Q` \x01a\x16\x08\x92\x91\x90a#\xCAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`\0\x80`\0a\x16.\x85\x85a\x18YV[\x94P\x90P`\0`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\xFD`\xF8\x1B\x03a\x16\xC6Wa\x16S\x86\x86a\x18\xE1V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a\x16nWPa\xFF\xFF\x81\x11\x15[a\x16\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\x12gV[\x92P\x83\x91Pa\x14<\x90PV[`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\x7F`\xF9\x1B\x03a\x17 Wa\x16\xE5\x86\x86a\x19\x9AV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a\x17\x04WPc\xFF\xFF\xFF\xFF\x81\x11\x15[a\x16\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12g\x90a#\xF9V[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x90\x03a\x17lWa\x17<\x86\x86a\x1AkV[\x95P`\x01`\x01`@\x1B\x03\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a\x16\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12g\x90a#\xF9V[P`\xF8\x81\x90\x1C`\xFD\x81\x10a\x16\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12g\x90a#\xF9V[```\xFD\x82`\x01`\x01`@\x1B\x03\x16\x10\x15a\x17\xC6W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91R[\x92\x91PPV[a\xFF\xFF\x82`\x01`\x01`@\x1B\x03\x16\x11a\x18\x15Wa\x17\xE5`\xFD`\xF8\x1Ba\x1B<V[a\x17\xEE\x83a\x1BcV[`@Q` \x01a\x17\xFF\x92\x91\x90a#\xCAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82`\x01`\x01`@\x1B\x03\x16\x11a\x18?Wa\x186`\x7F`\xF9\x1Ba\x1B<V[a\x17\xEE\x83a\x1B\xA6V[a\x18P`\x01`\x01`\xF8\x1B\x03\x19a\x1B<V[a\x17\xEE\x83a\x1B\xE9V[`\0\x80\x83Q\x83`\x01a\x18k\x91\x90a\"aV[\x11\x15\x80\x15a\x18\x82WPa\x18\x7F\x83`\x01a\"aV[\x83\x10[a\x18\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\x12gV[\x83\x83\x01` \x01Q\x80a\x146\x85`\x01a\"aV[`\0\x80\x83Q\x83`\x02a\x18\xF3\x91\x90a\"aV[\x11\x15\x80\x15a\x19\nWPa\x19\x07\x83`\x02a\"aV[\x83\x10[a\x19aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12gV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a\x146\x91\x90a\"aV[`\0\x80\x83Q\x83`\x04a\x19\xAC\x91\x90a\"aV[\x11\x15\x80\x15a\x19\xC3WPa\x19\xC0\x83`\x04a\"aV[\x83\x10[a\x1A\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12gV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1AOW\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1A/V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x146\x85`\x04a\"aV[`\0\x80\x83Q\x83`\x08a\x1A}\x91\x90a\"aV[\x11\x15\x80\x15a\x1A\x94WPa\x1A\x91\x83`\x08a\"aV[\x83\x10[a\x1A\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12gV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1B W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1B\0V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x146\x85`\x08a\"aV[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a\x17\xC0V[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1B\x96W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1BtV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1B\xD9W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1B\xB7V[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1C\x1CW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1B\xFAV[PPP`(\x81\x01`@R\x92\x91PPV[`\0[\x83\x81\x10\x15a\x1CGW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1C/V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1Ch\x81` \x86\x01` \x86\x01a\x1C,V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1C\x8F` \x83\x01\x84a\x1CPV[\x93\x92PPPV[` \x81\x01`\x02\x83\x10a\x1C\xB8WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xF6Wa\x1C\xF6a\x1C\xBEV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xF6Wa\x1C\xF6a\x1C\xBEV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\r\x0FW`\0\x80\xFD[`\0\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\x1DPWa\x1DPa\x1C\xBEV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1D~Wa\x1D~a\x1C\xBEV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x1D\x96W`\0\x80\xFD[\x83\x83` \x83\x017`\0` \x85\x83\x01\x01RP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1D\xC3W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x1D\xD8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xEEW`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a\x1E\0W`\0\x80\xFD[a\x1E\x08a\x1C\xD4V[a\x1E\x11\x82a\x1D\x1FV[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E,W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x86\x13a\x1E=W`\0\x80\xFD[a\x1EL\x86\x825` \x84\x01a\x1D6V[` \x83\x01RP`@\x82\x015\x91Pa\x1Eb\x82a\x1D\xAEV[`@\x81\x01\x91\x90\x91R\x93\x92PPPV[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\r\x0FW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x1E\x99W`\0\x80\xFD[a\x1C\x8F\x83\x835` \x85\x01a\x1D6V[`\0a\x01\0\x82\x84\x03\x12\x15a\x1E\xBBW`\0\x80\xFD[a\x1E\xC3a\x1C\xFCV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R\x90Pa\x1E\xE9``\x83\x01a\x1EqV[``\x82\x01R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x07W`\0\x80\xFD[a\x1F\x13\x84\x82\x85\x01a\x1E\x88V[`\x80\x83\x01RP`\xA0\x82\x81\x015\x90\x82\x01R`\xC0\x80\x83\x015\x90\x82\x01R`\xE0\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1FQW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FgW`\0\x80\xFD[a\x1Fs\x84\x82\x85\x01a\x1E\xA8V[\x94\x93PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x1F\x93W`\0\x80\xFD[\x855a\x1F\x9E\x81a\x1D\xAEV[\x94Pa\x1F\xAC` \x87\x01a\x1D\x1FV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xC7W`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\x1F\xD8W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xEEW`\0\x80\xFD[\x88` \x82\x84\x01\x01\x11\x15a \0W`\0\x80\xFD[` \x91\x90\x91\x01\x93P\x91P``\x86\x015a \x18\x81a\x1D\xAEV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0``\x84\x86\x03\x12\x15a ;W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a QW`\0\x80\xFD[a ]\x86\x82\x87\x01a\x1E\xA8V[\x93PP` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \x80W`\0\x80\xFD[a \x8C\x86\x82\x87\x01a\x1E\x88V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a \xA8W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a \xC3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a \xE3WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a!3W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a!\x10WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a!0W`\0\x81U`\x01\x01a!\x1CV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a!QWa!Qa\x1C\xBEV[a!e\x81a!_\x84Ta \xAFV[\x84a \xE9V[` `\x1F\x82\x11`\x01\x81\x14a!\x99W`\0\x83\x15a!\x81WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua!0V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a!\xC9W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a!\xA9V[P\x84\x82\x10\x15a!\xE7W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15a\"\x08W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1C\x8FW`\0\x80\xFD[` \x81R`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q``\x82\x01R`\0``\x83\x01Q`\x80\x80\x84\x01Ra\x1Fs`\xA0\x84\x01\x82a\x1CPV[\x80\x82\x01\x80\x82\x11\x15a\x17\xC0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\"\x94W`\0\x80\xFD[\x81Qa\x1C\x8F\x81a\x1D\xAEV[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra\"\xE3`\xE0\x84\x01\x82a\x1CPV[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a##W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a#\x04V[PPP\x92\x91PPV[`\0\x86Qa#>\x81\x84` \x8B\x01a\x1C,V[\x86Q\x90\x83\x01\x90a#R\x81\x83` \x8B\x01a\x1C,V[\x86Q\x91\x01\x90a#e\x81\x83` \x8A\x01a\x1C,V[\x85Q\x91\x01\x90a#x\x81\x83` \x89\x01a\x1C,V[\x84Q\x91\x01\x90a#\x8B\x81\x83` \x88\x01a\x1C,V[\x01\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\x01`\x01`@\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x11!\x90\x83\x01\x84a\x1CPV[`\0\x83Qa#\xDC\x81\x84` \x88\x01a\x1C,V[\x83Q\x90\x83\x01\x90a#\xF0\x81\x83` \x88\x01a\x1C,V[\x01\x94\x93PPPPV[` \x80\x82R\x81\x81\x01R\x7FNextVarUint, value outside range`@\x82\x01R``\x01\x90V\xFE\xA2dipfsX\"\x12 Gr\xD7\x84\x84\x83\xE8\x96{\xAA\xFF\xB2\x98\x15 \r\xAE\x08S/\xC6\xB6\xAA\xE1\x14\xCCNE\x91\0k\xEAdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MYSTIKOV2CELERMAIN_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x81W`\x005`\xE0\x1C\x80c\xBCXw\x06\x11a\0\xD1W\x80c\xDD\xAC]\xC1\x11a\0\x8AW\x80c\xEF\xBF\xB2\xAE\x11a\0dW\x80c\xEF\xBF\xB2\xAE\x14a\x04\tW\x80c\xF4\xAD\x17\xC6\x14a\x04\x1EW\x80c\xFAu\x0FV\x14a\x043W\x80c\xFB>=s\x14a\x04TW`\0\x80\xFD[\x80c\xDD\xAC]\xC1\x14a\x03\xBFW\x80c\xE0at\xE4\x14a\x03\xD4W\x80c\xEDn\xA3:\x14a\x03\xF4W`\0\x80\xFD[\x80c\xBCXw\x06\x14a\x03\x17W\x80c\xCB\\\x02\x9A\x14a\x03,W\x80c\xCB\xE3B\x85\x14a\x03?W\x80c\xCD\xFC\xEE\xBA\x14a\x03UW\x80c\xCF\xC7\xE2\xDA\x14a\x03\x94W\x80c\xD0\xB46\xBD\x14a\x03\xA9W`\0\x80\xFD[\x80cB.\0(\x11a\x01>W\x80cd\x0C\x0B6\x11a\x01\x18W\x80cd\x0C\x0B6\x14a\x02\xB6W\x80c\x82[_\x8D\x14a\x02\xCCW\x80c\x9A\x03cl\x14a\x02\xE1W\x80c\x9Cd\x9F\xDF\x14a\x02\xF4W`\0\x80\xFD[\x80cB.\0(\x14a\x02iW\x80cM\xDEo\xBC\x14a\x02\x8BW\x80cN<\x10\xB7\x14a\x02\xA1W`\0\x80\xFD[\x80c\x0B\xA9Y\t\x14a\x01\x86W\x80c\x1B\xA4l\xFD\x14a\x01\xAEW\x80c!\xE3-U\x14a\x01\xD6W\x80c$!\xE1U\x14a\x01\xF6W\x80c,\xD2mE\x14a\x02-W\x80c?\xE34z\x14a\x02MW[`\0\x80\xFD[4\x80\x15a\x01\x92W`\0\x80\xFD[Pa\x01\x9Ba\x04jV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xBAW`\0\x80\xFD[P`\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA5V[4\x80\x15a\x01\xE2W`\0\x80\xFD[P`\x02Ta\x01\xBE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\x02W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rd1\xB2\xB62\xB9`\xD9\x1B` \x82\x01R[`@Qa\x01\xA5\x91\x90a\x1C|V[4\x80\x15a\x029W`\0\x80\xFD[P`\x08Ta\x01\xBE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02YW`\0\x80\xFD[P`\x01`@Qa\x01\xA5\x91\x90a\x1C\x96V[4\x80\x15a\x02uW`\0\x80\xFD[Pa\x02\x89a\x02\x846`\x04a\x1D\xC6V[a\x04\xF3V[\0[4\x80\x15a\x02\x97W`\0\x80\xFD[Pa\x01\x9B`\x05T\x81V[4\x80\x15a\x02\xADW`\0\x80\xFD[Pa\x02 a\x05\x98V[4\x80\x15a\x02\xC2W`\0\x80\xFD[Pa\x01\x9B`\x06T\x81V[4\x80\x15a\x02\xD8W`\0\x80\xFD[Pa\x01\x9Ba\x06&V[a\x02\x89a\x02\xEF6`\x04a\x1F?V[a\x06\xADV[a\x03\x07a\x03\x026`\x04a\x1F{V[a\x06\xC6V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA5V[4\x80\x15a\x03#W`\0\x80\xFD[Pa\x03\x07a\x07PV[a\x02\x89a\x03:6`\x04a &V[a\x07\xC3V[4\x80\x15a\x03KW`\0\x80\xFD[Pa\x01\x9B`\x07T\x81V[4\x80\x15a\x03aW`\0\x80\xFD[P`\0Ta\x03|\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA5V[4\x80\x15a\x03\xA0W`\0\x80\xFD[Pa\x01\x9Ba\x0B\xEEV[4\x80\x15a\x03\xB5W`\0\x80\xFD[Pa\x01\x9B`\x04T\x81V[4\x80\x15a\x03\xCBW`\0\x80\xFD[Pa\x01\xBEa\x0CuV[4\x80\x15a\x03\xE0W`\0\x80\xFD[P`\tTa\x01\xBE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\0W`\0\x80\xFD[Pa\x03\x07a\r\x14V[4\x80\x15a\x04\x15W`\0\x80\xFD[Pa\x01\x9Ba\r]V[4\x80\x15a\x04*W`\0\x80\xFD[Pa\x01\x9Ba\r\xE4V[4\x80\x15a\x04?W`\0\x80\xFD[P`\0Ta\x03\x07\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x04`W`\0\x80\xFD[Pa\x01\x9B`\x03T\x81V[`\tT`@QcG:\x061`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG:\x061\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xDB\x91\x90a \x96V[\x90P\x80\x15a\x04\xE9W\x80a\x04\xEDV[`\x04T[\x91PP\x90V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x05\x1EW`@Qcm\x8F\x11Q`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\xA8\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U` \x81\x01Q`\x01\x90a\x05]\x90\x82a!8V[P`@\x01Q`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UV[`\x01\x80Ta\x05\xA5\x90a \xAFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xD1\x90a \xAFV[\x80\x15a\x06\x1EW\x80`\x1F\x10a\x05\xF3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x1EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x01W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\tT`@Qca\x86<\x03`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC3\x0Cx\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x97\x91\x90a \x96V[\x90P\x80\x15a\x06\xA5W\x80a\x04\xEDV[PP`\x07T\x90V[`@Qc\xE7\xA2O\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xF4W`@Qc=\xCA\x01\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x075\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0Ek\x92PPPV[\x90Pa\x07C\x86\x88\x85\x84a\x0F3V[P`\x01\x96\x95PPPPPPV[`\tT`@\x80Qc^,;\x83`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xBCXw\x06\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x07\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xBE\x91\x90a!\xF6V[\x90P\x90V[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08/\x91\x90a!\xF6V[\x15a\x08MW`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xBCXw\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC4\x91\x90a!\xF6V[\x15a\t\x91W`\0`@Q\x80`\x80\x01`@R\x802`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x08\xED`\0\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x86\x90R`@\x91\x82\x01\x85\x90R`\tT\x91Qc\x84\x9E\x8B\x9F`\xE0\x1B\x81R\x92\x93P\x16\x90c\x84\x9E\x8B\x9F\x90a\t1\x90\x84\x90`\x04\x01a\"\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tNW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tr\x91\x90a!\xF6V[a\t\x8FW`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[a\t\x99a\x0B\xEEV[\x83Q\x10\x15a\t\xBAW`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xC2a\x04jV[\x83Q\x11\x15a\t\xE3W`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xEBa\r]V[\x83`\xA0\x01Q\x10\x15a\n\x0FW`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x17a\r\xE4V[\x83`\xC0\x01Q\x10\x15a\n;W`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\nCa\x06&V[\x83`\xE0\x01Q\x10\x15a\ngW`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\n\x80\x84`@\x01Q\x85`\0\x01Q\x86``\x01Qa\x10%V[\x90P\x80\x84` \x01Q\x14a\n\xA6W`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT`@Qc\xDFY/}`\xE0\x1B\x81R2`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x12\x91\x90a!\xF6V[\x15a\x0B0W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x90\x82\x01R`\xC0\x86\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x85\x01Q``\x82\x01R`\x80\x80\x86\x01Q\x90\x82\x01R`\0a\x0Bs\x82a\x11*V[\x90Pa\x0B\x83\x86`\xA0\x01Q\x82a\x11\x99V[a\x0B\xB8a\x0B\x8Ea\x0CuV[`\xE0\x88\x01Q`\xC0\x89\x01Q\x89Qa\x0B\xA4\x91\x90a\"aV[a\x0B\xAE\x91\x90a\"aV[\x88`\xA0\x01Qa\x12\x1EV[` \x86\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPPPV[`\tT`@QcU%\x98I`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cU%\x98I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C_\x91\x90a \x96V[\x90P\x80\x15a\x0CmW\x80a\x04\xEDV[PP`\x03T\x90V[`\tT`@QcA\xFBiy`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cA\xFBiy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE6\x91\x90a\"\x82V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\r\x0FW`@Qc\x06\xF3\xD63`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[`\tT`@Qc\xBB\x072\x05`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xBB\x072\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x9AW=`\0\x80>=`\0\xFD[`\tT`@Qc{R\xB0\r`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c{R\xB0\r\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCE\x91\x90a \x96V[\x90P\x80\x15a\r\xDCW\x80a\x04\xEDV[PP`\x05T\x90V[`\tT`@QcO1O\x9D`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9Eb\x9F:\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EU\x91\x90a \x96V[\x90P\x80\x15a\x0EcW\x80a\x04\xEDV[PP`\x06T\x90V[a\x0E\x9D`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\x0E\xCF`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\x0E\xDB\x84\x82a\x13\x12V[\x90\x83R\x90Pa\x0E\xEA\x84\x82a\x13\x12V[` \x84\x01\x91\x90\x91R\x90Pa\x0E\xFE\x84\x82a\x13\x12V[`@\x84\x01\x91\x90\x91R\x90Pa\x0F\x12\x84\x82a\x13\x12V[``\x84\x01\x91\x90\x91R\x90Pa\x0F&\x84\x82a\x14CV[P`\x80\x83\x01RP\x92\x91PPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x0FaW`@Qc\x1BI^\xCF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`@\x1B\x03\x85\x81\x16`\x01`\xA8\x1B\x90\x92\x04\x16\x14a\x0F\x96W`@Qc7;\xC1!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x03a\x0F\xB8W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xC0a\x0CuV[`\x01`\x01`\xA0\x1B\x03\x16cx\xD6\x0C\xD7\x82\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xED\x92\x91\x90a\"\x9FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x07W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x1BW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x10hW`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83`\x01`\x01`\x80\x1B\x03\x16\x10a\x10\x91W`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90R`\x01`\x01`\x80\x1B\x03\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x10\xE0\x91`\x04\x01a\"\xFBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11!\x91\x90a \x96V[\x95\x94PPPPPV[``\x80a\x11:\x83`\0\x01Qa\x15PV[a\x11G\x84` \x01Qa\x15PV[a\x11T\x85`@\x01Qa\x15PV[a\x11a\x86``\x01Qa\x15PV[a\x11n\x87`\x80\x01Qa\x15\xE8V[`@Q` \x01a\x11\x82\x95\x94\x93\x92\x91\x90a#,V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[`\x08T`\x02T`\0T`@QcO\x9Er\xAD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93c\x9F<\xE5Z\x93\x87\x93a\x11\xE8\x93\x91\x90\x92\x16\x91`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x16\x90\x87\x90`\x04\x01a#\x97V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x12\x01W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\x15W=`\0\x80>=`\0\xFD[PPPPPPPV[a\x12(\x81\x83a\"aV[4\x14a\x12pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq4\xB79\xBA\xB334\xB1\xB4\xB2\xB7:\x10:7\xB5\xB2\xB7`q\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x12\xBDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x12\xC2V[``\x91P[PP\x90P\x80a\x13\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x18[[\xDD[\x9D\x08\x1D\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`R\x1B`D\x82\x01R`d\x01a\x12gV[PPPPV[`\0\x80\x83Q\x83` a\x13$\x91\x90a\"aV[\x11\x15\x80\x15a\x13;WPa\x138\x83` a\"aV[\x83\x10[a\x13\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\x12gV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x13\xC8W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x13\xA8V[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x14*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x12gV[\x80a\x146\x85` a\"aV[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a\x14R\x85\x85a\x16\x1FV[\x86Q\x90\x95P\x90\x91Pa\x14d\x82\x86a\"aV[\x11\x15\x80\x15a\x14zWPa\x14w\x81\x85a\"aV[\x84\x10[a\x14\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\x12gV[``\x81\x15\x80\x15a\x14\xEDW`@Q\x91P` \x82\x01`@Ra\x157V[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a\x15&W\x80Q\x83R` \x92\x83\x01\x92\x01a\x15\x0EV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a\x15C\x83\x87a\"aV[\x93P\x93PPP\x92P\x92\x90PV[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x15\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\x12gV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\x15\xD8W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x15\xB6V[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\x15\xF6\x81a\x17\x92V[\x83`@Q` \x01a\x16\x08\x92\x91\x90a#\xCAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`\0\x80`\0a\x16.\x85\x85a\x18YV[\x94P\x90P`\0`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\xFD`\xF8\x1B\x03a\x16\xC6Wa\x16S\x86\x86a\x18\xE1V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a\x16nWPa\xFF\xFF\x81\x11\x15[a\x16\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\x12gV[\x92P\x83\x91Pa\x14<\x90PV[`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\x7F`\xF9\x1B\x03a\x17 Wa\x16\xE5\x86\x86a\x19\x9AV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a\x17\x04WPc\xFF\xFF\xFF\xFF\x81\x11\x15[a\x16\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12g\x90a#\xF9V[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x90\x03a\x17lWa\x17<\x86\x86a\x1AkV[\x95P`\x01`\x01`@\x1B\x03\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a\x16\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12g\x90a#\xF9V[P`\xF8\x81\x90\x1C`\xFD\x81\x10a\x16\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12g\x90a#\xF9V[```\xFD\x82`\x01`\x01`@\x1B\x03\x16\x10\x15a\x17\xC6W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91R[\x92\x91PPV[a\xFF\xFF\x82`\x01`\x01`@\x1B\x03\x16\x11a\x18\x15Wa\x17\xE5`\xFD`\xF8\x1Ba\x1B<V[a\x17\xEE\x83a\x1BcV[`@Q` \x01a\x17\xFF\x92\x91\x90a#\xCAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82`\x01`\x01`@\x1B\x03\x16\x11a\x18?Wa\x186`\x7F`\xF9\x1Ba\x1B<V[a\x17\xEE\x83a\x1B\xA6V[a\x18P`\x01`\x01`\xF8\x1B\x03\x19a\x1B<V[a\x17\xEE\x83a\x1B\xE9V[`\0\x80\x83Q\x83`\x01a\x18k\x91\x90a\"aV[\x11\x15\x80\x15a\x18\x82WPa\x18\x7F\x83`\x01a\"aV[\x83\x10[a\x18\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\x12gV[\x83\x83\x01` \x01Q\x80a\x146\x85`\x01a\"aV[`\0\x80\x83Q\x83`\x02a\x18\xF3\x91\x90a\"aV[\x11\x15\x80\x15a\x19\nWPa\x19\x07\x83`\x02a\"aV[\x83\x10[a\x19aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12gV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a\x146\x91\x90a\"aV[`\0\x80\x83Q\x83`\x04a\x19\xAC\x91\x90a\"aV[\x11\x15\x80\x15a\x19\xC3WPa\x19\xC0\x83`\x04a\"aV[\x83\x10[a\x1A\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12gV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1AOW\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1A/V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x146\x85`\x04a\"aV[`\0\x80\x83Q\x83`\x08a\x1A}\x91\x90a\"aV[\x11\x15\x80\x15a\x1A\x94WPa\x1A\x91\x83`\x08a\"aV[\x83\x10[a\x1A\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\x12gV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1B W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1B\0V[PPP\x01`@\x81\x90R`\x1F\x19\x01Q\x90P\x80a\x146\x85`\x08a\"aV[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a\x17\xC0V[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1B\x96W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1BtV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1B\xD9W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1B\xB7V[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\x1C\x1CW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x1B\xFAV[PPP`(\x81\x01`@R\x92\x91PPV[`\0[\x83\x81\x10\x15a\x1CGW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1C/V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1Ch\x81` \x86\x01` \x86\x01a\x1C,V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1C\x8F` \x83\x01\x84a\x1CPV[\x93\x92PPPV[` \x81\x01`\x02\x83\x10a\x1C\xB8WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xF6Wa\x1C\xF6a\x1C\xBEV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xF6Wa\x1C\xF6a\x1C\xBEV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\r\x0FW`\0\x80\xFD[`\0\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\x1DPWa\x1DPa\x1C\xBEV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1D~Wa\x1D~a\x1C\xBEV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x1D\x96W`\0\x80\xFD[\x83\x83` \x83\x017`\0` \x85\x83\x01\x01RP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1D\xC3W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x1D\xD8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xEEW`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a\x1E\0W`\0\x80\xFD[a\x1E\x08a\x1C\xD4V[a\x1E\x11\x82a\x1D\x1FV[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E,W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x86\x13a\x1E=W`\0\x80\xFD[a\x1EL\x86\x825` \x84\x01a\x1D6V[` \x83\x01RP`@\x82\x015\x91Pa\x1Eb\x82a\x1D\xAEV[`@\x81\x01\x91\x90\x91R\x93\x92PPPV[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\r\x0FW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x1E\x99W`\0\x80\xFD[a\x1C\x8F\x83\x835` \x85\x01a\x1D6V[`\0a\x01\0\x82\x84\x03\x12\x15a\x1E\xBBW`\0\x80\xFD[a\x1E\xC3a\x1C\xFCV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R\x90Pa\x1E\xE9``\x83\x01a\x1EqV[``\x82\x01R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x07W`\0\x80\xFD[a\x1F\x13\x84\x82\x85\x01a\x1E\x88V[`\x80\x83\x01RP`\xA0\x82\x81\x015\x90\x82\x01R`\xC0\x80\x83\x015\x90\x82\x01R`\xE0\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1FQW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FgW`\0\x80\xFD[a\x1Fs\x84\x82\x85\x01a\x1E\xA8V[\x94\x93PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x1F\x93W`\0\x80\xFD[\x855a\x1F\x9E\x81a\x1D\xAEV[\x94Pa\x1F\xAC` \x87\x01a\x1D\x1FV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xC7W`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\x1F\xD8W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xEEW`\0\x80\xFD[\x88` \x82\x84\x01\x01\x11\x15a \0W`\0\x80\xFD[` \x91\x90\x91\x01\x93P\x91P``\x86\x015a \x18\x81a\x1D\xAEV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0``\x84\x86\x03\x12\x15a ;W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a QW`\0\x80\xFD[a ]\x86\x82\x87\x01a\x1E\xA8V[\x93PP` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \x80W`\0\x80\xFD[a \x8C\x86\x82\x87\x01a\x1E\x88V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a \xA8W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a \xC3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a \xE3WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a!3W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a!\x10WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a!0W`\0\x81U`\x01\x01a!\x1CV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a!QWa!Qa\x1C\xBEV[a!e\x81a!_\x84Ta \xAFV[\x84a \xE9V[` `\x1F\x82\x11`\x01\x81\x14a!\x99W`\0\x83\x15a!\x81WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua!0V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a!\xC9W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a!\xA9V[P\x84\x82\x10\x15a!\xE7W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15a\"\x08W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1C\x8FW`\0\x80\xFD[` \x81R`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q``\x82\x01R`\0``\x83\x01Q`\x80\x80\x84\x01Ra\x1Fs`\xA0\x84\x01\x82a\x1CPV[\x80\x82\x01\x80\x82\x11\x15a\x17\xC0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\"\x94W`\0\x80\xFD[\x81Qa\x1C\x8F\x81a\x1D\xAEV[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra\"\xE3`\xE0\x84\x01\x82a\x1CPV[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a##W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a#\x04V[PPP\x92\x91PPV[`\0\x86Qa#>\x81\x84` \x8B\x01a\x1C,V[\x86Q\x90\x83\x01\x90a#R\x81\x83` \x8B\x01a\x1C,V[\x86Q\x91\x01\x90a#e\x81\x83` \x8A\x01a\x1C,V[\x85Q\x91\x01\x90a#x\x81\x83` \x89\x01a\x1C,V[\x84Q\x91\x01\x90a#\x8B\x81\x83` \x88\x01a\x1C,V[\x01\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\x01`\x01`@\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x11!\x90\x83\x01\x84a\x1CPV[`\0\x83Qa#\xDC\x81\x84` \x88\x01a\x1C,V[\x83Q\x90\x83\x01\x90a#\xF0\x81\x83` \x88\x01a\x1C,V[\x01\x94\x93PPPPV[` \x80\x82R\x81\x81\x01R\x7FNextVarUint, value outside range`@\x82\x01R``\x01\x90V\xFE\xA2dipfsX\"\x12 Gr\xD7\x84\x84\x83\xE8\x96{\xAA\xFF\xB2\x98\x15 \r\xAE\x08S/\xC6\xB6\xAA\xE1\x14\xCCNE\x91\0k\xEAdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MYSTIKOV2CELERMAIN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MystikoV2CelerMain<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoV2CelerMain<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoV2CelerMain<M> {
        type Target = ::ethers::contract::Contract<M>;
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
    impl<M: ::ethers::providers::Middleware> MystikoV2CelerMain<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MYSTIKOV2CELERMAIN_ABI.clone(),
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
                MYSTIKOV2CELERMAIN_ABI.clone(),
                MYSTIKOV2CELERMAIN_BYTECODE.clone().into(),
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
        ///Calls the contract's `executeMessage` (0x9c649fdf) function
        pub fn execute_message(
            &self,
            sender: ::ethers::core::types::Address,
            src_chain_id: u64,
            message: ::ethers::core::types::Bytes,
            executor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([156, 100, 159, 223], (sender, src_chain_id, message, executor))
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
        ///Calls the contract's `setPeerContract` (0x422e0028) function
        pub fn set_peer_contract(
            &self,
            peer_contract: BridgePeerContract,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 46, 0, 40], (peer_contract,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settings` (0xe06174e4) function
        pub fn settings(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([224, 97, 116, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CommitmentCrossChain` event
        pub fn commitment_cross_chain_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CommitmentCrossChainFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CommitmentCrossChainFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MystikoV2CelerMain<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    ///Custom Error type `SenderIsNotBridgeProxy` with signature `SenderIsNotBridgeProxy()` and selector `0x7b94039e`
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
    #[etherror(name = "SenderIsNotBridgeProxy", abi = "SenderIsNotBridgeProxy()")]
    pub struct SenderIsNotBridgeProxy;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
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
    impl ::ethers::core::abi::AbiDecode for MystikoV2CelerMainErrors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
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
            if let Ok(decoded) = <CertificateInvalid as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CertificateInvalid(decoded));
            }
            if let Ok(decoded) = <CommitmentHashIncorrect as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CommitmentHashIncorrect(decoded));
            }
            if let Ok(decoded) = <DepositsDisabled as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositsDisabled(decoded));
            }
            if let Ok(decoded) = <ExecutorFeeTooFew as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecutorFeeTooFew(decoded));
            }
            if let Ok(decoded) = <HashKGreaterThanFieldSize as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashKGreaterThanFieldSize(decoded));
            }
            if let Ok(decoded) = <NotSupport as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotSupport(decoded));
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
            if let Ok(decoded) = <SanctionedAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionedAddress(decoded));
            }
            if let Ok(decoded) = <SenderIsNotBridgeProxy as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SenderIsNotBridgeProxy(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MystikoV2CelerMainErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AmountLessThanZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AmountTooLarge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AmountTooSmall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssociatedPoolNotSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BridgeFeeTooFew(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CertificateInvalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CommitmentHashIncorrect(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositsDisabled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecutorFeeTooFew(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HashKGreaterThanFieldSize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotSupport(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerChainIdNotMatched(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerContractAlreadySet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerContractNotMatched(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RandomSGreaterThanFieldSize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RollupFeeToFew(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SanctionedAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SenderIsNotBridgeProxy(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MystikoV2CelerMainErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AmountLessThanZero as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AmountTooLarge as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AmountTooSmall as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AssociatedPoolNotSet as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <BridgeFeeTooFew as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <CertificateInvalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <CommitmentHashIncorrect as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <DepositsDisabled as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ExecutorFeeTooFew as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <HashKGreaterThanFieldSize as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NotSupport as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <PeerChainIdNotMatched as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <PeerContractAlreadySet as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <PeerContractNotMatched as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <RandomSGreaterThanFieldSize as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <RollupFeeToFew as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <SanctionedAddress as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <SenderIsNotBridgeProxy as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2CelerMainErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AmountLessThanZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssociatedPoolNotSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::CertificateInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentHashIncorrect(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutorFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashKGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotSupport(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerChainIdNotMatched(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerContractAlreadySet(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerContractNotMatched(element) => ::core::fmt::Display::fmt(element, f),
                Self::RandomSGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupFeeToFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionedAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SenderIsNotBridgeProxy(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<RandomSGreaterThanFieldSize> for MystikoV2CelerMainErrors {
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
    ///Container type for all input parameters for the `executeMessage` function with signature `executeMessage(address,uint64,bytes,address)` and selector `0x9c649fdf`
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
    #[ethcall(name = "executeMessage", abi = "executeMessage(address,uint64,bytes,address)")]
    pub struct ExecuteMessageCall {
        pub sender: ::ethers::core::types::Address,
        pub src_chain_id: u64,
        pub message: ::ethers::core::types::Bytes,
        pub executor: ::ethers::core::types::Address,
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
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
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
        IsCertificateCheckEnabled(IsCertificateCheckEnabledCall),
        IsDepositsDisabled(IsDepositsDisabledCall),
        IsPeerContractSet(IsPeerContractSetCall),
        PeerChainId(PeerChainIdCall),
        PeerChainName(PeerChainNameCall),
        PeerContract(PeerContractCall),
        SetPeerContract(SetPeerContractCall),
        Settings(SettingsCall),
    }
    impl ::ethers::core::abi::AbiDecode for MystikoV2CelerMainCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AssetAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetAddress(decoded));
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
            if let Ok(decoded) = <ExecuteMessageCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteMessage(decoded));
            }
            if let Ok(decoded) = <GetAssociatedCommitmentPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAssociatedCommitmentPool(decoded));
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
            if let Ok(decoded) = <PeerChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainId(decoded));
            }
            if let Ok(decoded) = <PeerChainNameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainName(decoded));
            }
            if let Ok(decoded) = <PeerContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerContract(decoded));
            }
            if let Ok(decoded) = <SetPeerContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPeerContract(decoded));
            }
            if let Ok(decoded) = <SettingsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Settings(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MystikoV2CelerMainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::ExecuteMessage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAssociatedCommitmentPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMaxAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMinAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMinBridgeFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMinExecutorFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPeerMinRollupFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsCertificateCheckEnabled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDepositsDisabled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsPeerContractSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerChainName(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeerContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPeerContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Settings(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2CelerMainCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetAddress(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::ExecuteMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssociatedCommitmentPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinBridgeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPeerMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsCertificateCheckEnabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<DefaultPeerMinExecutorFeeCall> for MystikoV2CelerMainCalls {
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
    impl ::core::convert::From<GetAssociatedCommitmentPoolCall> for MystikoV2CelerMainCalls {
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
    impl ::core::convert::From<IsCertificateCheckEnabledCall> for MystikoV2CelerMainCalls {
        fn from(value: IsCertificateCheckEnabledCall) -> Self {
            Self::IsCertificateCheckEnabled(value)
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
    ///Container type for all return fields from the `executeMessage` function with signature `executeMessage(address,uint64,bytes,address)` and selector `0x9c649fdf`
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
    pub struct ExecuteMessageReturn(pub bool);
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
}
