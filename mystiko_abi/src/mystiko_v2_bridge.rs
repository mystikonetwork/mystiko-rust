pub use mystiko_v2_bridge::*;
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
pub mod mystiko_v2_bridge {
    const _: () = {
        ::core::include_bytes!("../json/MystikoV2Bridge.json",);
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
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
            ]),
            errors: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("ExecutorFeeTooFew"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ExecutorFeeTooFew"),
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
    pub static MYSTIKOV2BRIDGE_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    pub struct MystikoV2Bridge<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoV2Bridge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoV2Bridge<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoV2Bridge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoV2Bridge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MystikoV2Bridge))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> MystikoV2Bridge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                MYSTIKOV2BRIDGE_ABI.clone(),
                client,
            ))
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
        ///Calls the contract's `getAssociatedCommitmentPool` (0xddac5dc1) function
        pub fn get_associated_commitment_pool(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([221, 172, 93, 193], ())
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
        ///Calls the contract's `setDepositsDisabled` (0xea0cde85) function
        pub fn set_deposits_disabled(&self, state: bool) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 12, 222, 133], state)
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(&self) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, MystikoV2BridgeEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for MystikoV2Bridge<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    pub enum MystikoV2BridgeErrors {
        AmountTooLarge(AmountTooLarge),
        AmountTooSmall(AmountTooSmall),
        BridgeFeeTooFew(BridgeFeeTooFew),
        CommitmentHashIncorrect(CommitmentHashIncorrect),
        DepositsDisabled(DepositsDisabled),
        ExecutorFeeTooFew(ExecutorFeeTooFew),
        HashKGreaterThanFieldSize(HashKGreaterThanFieldSize),
        Invalid(Invalid),
        MinAmountGreaterThanMaxAmount(MinAmountGreaterThanMaxAmount),
        NotChanged(NotChanged),
        OnlyOperator(OnlyOperator),
        RandomSGreaterThanFieldSize(RandomSGreaterThanFieldSize),
        RollupFeeToFew(RollupFeeToFew),
        SanctionedAddress(SanctionedAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoV2BridgeErrors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
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
            if let Ok(decoded) = <CommitmentHashIncorrect as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CommitmentHashIncorrect(decoded));
            }
            if let Ok(decoded) = <DepositsDisabled as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositsDisabled(decoded));
            }
            if let Ok(decoded) = <ExecutorFeeTooFew as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecutorFeeTooFew(decoded));
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
    impl ::ethers_core::abi::AbiEncode for MystikoV2BridgeErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AmountTooLarge(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AmountTooSmall(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::BridgeFeeTooFew(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CommitmentHashIncorrect(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DepositsDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ExecutorFeeTooFew(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::HashKGreaterThanFieldSize(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Invalid(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::MinAmountGreaterThanMaxAmount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NotChanged(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnlyOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RandomSGreaterThanFieldSize(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RollupFeeToFew(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionedAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for MystikoV2BridgeErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AmountTooLarge as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <AmountTooSmall as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <BridgeFeeTooFew as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <CommitmentHashIncorrect as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <DepositsDisabled as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <ExecutorFeeTooFew as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <HashKGreaterThanFieldSize as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <Invalid as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <MinAmountGreaterThanMaxAmount as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <NotChanged as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OnlyOperator as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <RandomSGreaterThanFieldSize as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <RollupFeeToFew as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <SanctionedAddress as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2BridgeErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AmountTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentHashIncorrect(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutorFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashKGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinAmountGreaterThanMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotChanged(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RandomSGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupFeeToFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionedAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MystikoV2BridgeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AmountTooLarge> for MystikoV2BridgeErrors {
        fn from(value: AmountTooLarge) -> Self {
            Self::AmountTooLarge(value)
        }
    }
    impl ::core::convert::From<AmountTooSmall> for MystikoV2BridgeErrors {
        fn from(value: AmountTooSmall) -> Self {
            Self::AmountTooSmall(value)
        }
    }
    impl ::core::convert::From<BridgeFeeTooFew> for MystikoV2BridgeErrors {
        fn from(value: BridgeFeeTooFew) -> Self {
            Self::BridgeFeeTooFew(value)
        }
    }
    impl ::core::convert::From<CommitmentHashIncorrect> for MystikoV2BridgeErrors {
        fn from(value: CommitmentHashIncorrect) -> Self {
            Self::CommitmentHashIncorrect(value)
        }
    }
    impl ::core::convert::From<DepositsDisabled> for MystikoV2BridgeErrors {
        fn from(value: DepositsDisabled) -> Self {
            Self::DepositsDisabled(value)
        }
    }
    impl ::core::convert::From<ExecutorFeeTooFew> for MystikoV2BridgeErrors {
        fn from(value: ExecutorFeeTooFew) -> Self {
            Self::ExecutorFeeTooFew(value)
        }
    }
    impl ::core::convert::From<HashKGreaterThanFieldSize> for MystikoV2BridgeErrors {
        fn from(value: HashKGreaterThanFieldSize) -> Self {
            Self::HashKGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<Invalid> for MystikoV2BridgeErrors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<MinAmountGreaterThanMaxAmount> for MystikoV2BridgeErrors {
        fn from(value: MinAmountGreaterThanMaxAmount) -> Self {
            Self::MinAmountGreaterThanMaxAmount(value)
        }
    }
    impl ::core::convert::From<NotChanged> for MystikoV2BridgeErrors {
        fn from(value: NotChanged) -> Self {
            Self::NotChanged(value)
        }
    }
    impl ::core::convert::From<OnlyOperator> for MystikoV2BridgeErrors {
        fn from(value: OnlyOperator) -> Self {
            Self::OnlyOperator(value)
        }
    }
    impl ::core::convert::From<RandomSGreaterThanFieldSize> for MystikoV2BridgeErrors {
        fn from(value: RandomSGreaterThanFieldSize) -> Self {
            Self::RandomSGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<RollupFeeToFew> for MystikoV2BridgeErrors {
        fn from(value: RollupFeeToFew) -> Self {
            Self::RollupFeeToFew(value)
        }
    }
    impl ::core::convert::From<SanctionedAddress> for MystikoV2BridgeErrors {
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
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers_contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoV2BridgeEvents {
        CommitmentCrossChainFilter(CommitmentCrossChainFilter),
        DepositAmountLimitsFilter(DepositAmountLimitsFilter),
        DepositsDisabledFilter(DepositsDisabledFilter),
        MinBridgeFeeFilter(MinBridgeFeeFilter),
        MinExecutorFeeFilter(MinExecutorFeeFilter),
        OperatorChangedFilter(OperatorChangedFilter),
        PeerMinExecutorFeeFilter(PeerMinExecutorFeeFilter),
        PeerMinRollupFeeFilter(PeerMinRollupFeeFilter),
        SanctionsCheckFilter(SanctionsCheckFilter),
        SanctionsListFilter(SanctionsListFilter),
    }
    impl ::ethers_contract::EthLogDecode for MystikoV2BridgeEvents {
        fn decode_log(log: &::ethers_core::abi::RawLog) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = CommitmentCrossChainFilter::decode_log(log) {
                return Ok(MystikoV2BridgeEvents::CommitmentCrossChainFilter(decoded));
            }
            if let Ok(decoded) = DepositAmountLimitsFilter::decode_log(log) {
                return Ok(MystikoV2BridgeEvents::DepositAmountLimitsFilter(decoded));
            }
            if let Ok(decoded) = DepositsDisabledFilter::decode_log(log) {
                return Ok(MystikoV2BridgeEvents::DepositsDisabledFilter(decoded));
            }
            if let Ok(decoded) = MinBridgeFeeFilter::decode_log(log) {
                return Ok(MystikoV2BridgeEvents::MinBridgeFeeFilter(decoded));
            }
            if let Ok(decoded) = MinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2BridgeEvents::MinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(MystikoV2BridgeEvents::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = PeerMinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2BridgeEvents::PeerMinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = PeerMinRollupFeeFilter::decode_log(log) {
                return Ok(MystikoV2BridgeEvents::PeerMinRollupFeeFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(MystikoV2BridgeEvents::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(MystikoV2BridgeEvents::SanctionsListFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MystikoV2BridgeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CommitmentCrossChainFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositAmountLimitsFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsDisabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinBridgeFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinExecutorFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerMinExecutorFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerMinRollupFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsCheckFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsListFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CommitmentCrossChainFilter> for MystikoV2BridgeEvents {
        fn from(value: CommitmentCrossChainFilter) -> Self {
            Self::CommitmentCrossChainFilter(value)
        }
    }
    impl ::core::convert::From<DepositAmountLimitsFilter> for MystikoV2BridgeEvents {
        fn from(value: DepositAmountLimitsFilter) -> Self {
            Self::DepositAmountLimitsFilter(value)
        }
    }
    impl ::core::convert::From<DepositsDisabledFilter> for MystikoV2BridgeEvents {
        fn from(value: DepositsDisabledFilter) -> Self {
            Self::DepositsDisabledFilter(value)
        }
    }
    impl ::core::convert::From<MinBridgeFeeFilter> for MystikoV2BridgeEvents {
        fn from(value: MinBridgeFeeFilter) -> Self {
            Self::MinBridgeFeeFilter(value)
        }
    }
    impl ::core::convert::From<MinExecutorFeeFilter> for MystikoV2BridgeEvents {
        fn from(value: MinExecutorFeeFilter) -> Self {
            Self::MinExecutorFeeFilter(value)
        }
    }
    impl ::core::convert::From<OperatorChangedFilter> for MystikoV2BridgeEvents {
        fn from(value: OperatorChangedFilter) -> Self {
            Self::OperatorChangedFilter(value)
        }
    }
    impl ::core::convert::From<PeerMinExecutorFeeFilter> for MystikoV2BridgeEvents {
        fn from(value: PeerMinExecutorFeeFilter) -> Self {
            Self::PeerMinExecutorFeeFilter(value)
        }
    }
    impl ::core::convert::From<PeerMinRollupFeeFilter> for MystikoV2BridgeEvents {
        fn from(value: PeerMinRollupFeeFilter) -> Self {
            Self::PeerMinRollupFeeFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckFilter> for MystikoV2BridgeEvents {
        fn from(value: SanctionsCheckFilter) -> Self {
            Self::SanctionsCheckFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsListFilter> for MystikoV2BridgeEvents {
        fn from(value: SanctionsListFilter) -> Self {
            Self::SanctionsListFilter(value)
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
    pub enum MystikoV2BridgeCalls {
        AssetType(AssetTypeCall),
        BridgeProxyAddress(BridgeProxyAddressCall),
        BridgeType(BridgeTypeCall),
        ChangeOperator(ChangeOperatorCall),
        Deposit(DepositCall),
        DisableSanctionsCheck(DisableSanctionsCheckCall),
        EnableSanctionsCheck(EnableSanctionsCheckCall),
        GetAssociatedCommitmentPool(GetAssociatedCommitmentPoolCall),
        GetMaxAmount(GetMaxAmountCall),
        GetMinAmount(GetMinAmountCall),
        GetMinBridgeFee(GetMinBridgeFeeCall),
        GetMinExecutorFee(GetMinExecutorFeeCall),
        GetPeerMinExecutorFee(GetPeerMinExecutorFeeCall),
        GetPeerMinRollupFee(GetPeerMinRollupFeeCall),
        IsDepositsDisabled(IsDepositsDisabledCall),
        PeerChainId(PeerChainIdCall),
        PeerChainName(PeerChainNameCall),
        PeerContract(PeerContractCall),
        SanctionsCheck(SanctionsCheckCall),
        SanctionsList(SanctionsListCall),
        SetAssociatedCommitmentPool(SetAssociatedCommitmentPoolCall),
        SetBridgeProxyAddress(SetBridgeProxyAddressCall),
        SetDepositsDisabled(SetDepositsDisabledCall),
        SetMinBridgeFee(SetMinBridgeFeeCall),
        SetMinExecutorFee(SetMinExecutorFeeCall),
        SetPeerContract(SetPeerContractCall),
        SetPeerMinExecutorFee(SetPeerMinExecutorFeeCall),
        SetPeerMinRollupFee(SetPeerMinRollupFeeCall),
        UpdateDepositAmountLimits(UpdateDepositAmountLimitsCall),
        UpdateSanctionsListAddress(UpdateSanctionsListAddressCall),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoV2BridgeCalls {
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
            if let Ok(decoded) = <GetAssociatedCommitmentPoolCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAssociatedCommitmentPool(decoded));
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
            if let Ok(decoded) = <PeerChainIdCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainId(decoded));
            }
            if let Ok(decoded) = <PeerChainNameCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerChainName(decoded));
            }
            if let Ok(decoded) = <PeerContractCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeerContract(decoded));
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
            if let Ok(decoded) = <SetDepositsDisabledCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDepositsDisabled(decoded));
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
            if let Ok(decoded) = <UpdateDepositAmountLimitsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateDepositAmountLimits(decoded));
            }
            if let Ok(decoded) = <UpdateSanctionsListAddressCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateSanctionsListAddress(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for MystikoV2BridgeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetType(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::BridgeProxyAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::BridgeType(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ChangeOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DisableSanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::EnableSanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetAssociatedCommitmentPool(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMaxAmount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMinAmount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMinBridgeFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMinExecutorFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetPeerMinExecutorFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetPeerMinRollupFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::IsDepositsDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::PeerChainId(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::PeerChainName(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::PeerContract(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionsList(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetAssociatedCommitmentPool(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetBridgeProxyAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetDepositsDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetMinBridgeFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetMinExecutorFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetPeerContract(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetPeerMinExecutorFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetPeerMinRollupFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UpdateDepositAmountLimits(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UpdateSanctionsListAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2BridgeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetType(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeProxyAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeType(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableSanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableSanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssociatedCommitmentPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinBridgeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPeerMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPeerMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerChainName(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAssociatedCommitmentPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBridgeProxyAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinBridgeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPeerContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPeerMinExecutorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPeerMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDepositAmountLimits(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateSanctionsListAddress(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssetTypeCall> for MystikoV2BridgeCalls {
        fn from(value: AssetTypeCall) -> Self {
            Self::AssetType(value)
        }
    }
    impl ::core::convert::From<BridgeProxyAddressCall> for MystikoV2BridgeCalls {
        fn from(value: BridgeProxyAddressCall) -> Self {
            Self::BridgeProxyAddress(value)
        }
    }
    impl ::core::convert::From<BridgeTypeCall> for MystikoV2BridgeCalls {
        fn from(value: BridgeTypeCall) -> Self {
            Self::BridgeType(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorCall> for MystikoV2BridgeCalls {
        fn from(value: ChangeOperatorCall) -> Self {
            Self::ChangeOperator(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MystikoV2BridgeCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DisableSanctionsCheckCall> for MystikoV2BridgeCalls {
        fn from(value: DisableSanctionsCheckCall) -> Self {
            Self::DisableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<EnableSanctionsCheckCall> for MystikoV2BridgeCalls {
        fn from(value: EnableSanctionsCheckCall) -> Self {
            Self::EnableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<GetAssociatedCommitmentPoolCall> for MystikoV2BridgeCalls {
        fn from(value: GetAssociatedCommitmentPoolCall) -> Self {
            Self::GetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<GetMaxAmountCall> for MystikoV2BridgeCalls {
        fn from(value: GetMaxAmountCall) -> Self {
            Self::GetMaxAmount(value)
        }
    }
    impl ::core::convert::From<GetMinAmountCall> for MystikoV2BridgeCalls {
        fn from(value: GetMinAmountCall) -> Self {
            Self::GetMinAmount(value)
        }
    }
    impl ::core::convert::From<GetMinBridgeFeeCall> for MystikoV2BridgeCalls {
        fn from(value: GetMinBridgeFeeCall) -> Self {
            Self::GetMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<GetMinExecutorFeeCall> for MystikoV2BridgeCalls {
        fn from(value: GetMinExecutorFeeCall) -> Self {
            Self::GetMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<GetPeerMinExecutorFeeCall> for MystikoV2BridgeCalls {
        fn from(value: GetPeerMinExecutorFeeCall) -> Self {
            Self::GetPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<GetPeerMinRollupFeeCall> for MystikoV2BridgeCalls {
        fn from(value: GetPeerMinRollupFeeCall) -> Self {
            Self::GetPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<IsDepositsDisabledCall> for MystikoV2BridgeCalls {
        fn from(value: IsDepositsDisabledCall) -> Self {
            Self::IsDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<PeerChainIdCall> for MystikoV2BridgeCalls {
        fn from(value: PeerChainIdCall) -> Self {
            Self::PeerChainId(value)
        }
    }
    impl ::core::convert::From<PeerChainNameCall> for MystikoV2BridgeCalls {
        fn from(value: PeerChainNameCall) -> Self {
            Self::PeerChainName(value)
        }
    }
    impl ::core::convert::From<PeerContractCall> for MystikoV2BridgeCalls {
        fn from(value: PeerContractCall) -> Self {
            Self::PeerContract(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckCall> for MystikoV2BridgeCalls {
        fn from(value: SanctionsCheckCall) -> Self {
            Self::SanctionsCheck(value)
        }
    }
    impl ::core::convert::From<SanctionsListCall> for MystikoV2BridgeCalls {
        fn from(value: SanctionsListCall) -> Self {
            Self::SanctionsList(value)
        }
    }
    impl ::core::convert::From<SetAssociatedCommitmentPoolCall> for MystikoV2BridgeCalls {
        fn from(value: SetAssociatedCommitmentPoolCall) -> Self {
            Self::SetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<SetBridgeProxyAddressCall> for MystikoV2BridgeCalls {
        fn from(value: SetBridgeProxyAddressCall) -> Self {
            Self::SetBridgeProxyAddress(value)
        }
    }
    impl ::core::convert::From<SetDepositsDisabledCall> for MystikoV2BridgeCalls {
        fn from(value: SetDepositsDisabledCall) -> Self {
            Self::SetDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<SetMinBridgeFeeCall> for MystikoV2BridgeCalls {
        fn from(value: SetMinBridgeFeeCall) -> Self {
            Self::SetMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<SetMinExecutorFeeCall> for MystikoV2BridgeCalls {
        fn from(value: SetMinExecutorFeeCall) -> Self {
            Self::SetMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<SetPeerContractCall> for MystikoV2BridgeCalls {
        fn from(value: SetPeerContractCall) -> Self {
            Self::SetPeerContract(value)
        }
    }
    impl ::core::convert::From<SetPeerMinExecutorFeeCall> for MystikoV2BridgeCalls {
        fn from(value: SetPeerMinExecutorFeeCall) -> Self {
            Self::SetPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<SetPeerMinRollupFeeCall> for MystikoV2BridgeCalls {
        fn from(value: SetPeerMinRollupFeeCall) -> Self {
            Self::SetPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<UpdateDepositAmountLimitsCall> for MystikoV2BridgeCalls {
        fn from(value: UpdateDepositAmountLimitsCall) -> Self {
            Self::UpdateDepositAmountLimits(value)
        }
    }
    impl ::core::convert::From<UpdateSanctionsListAddressCall> for MystikoV2BridgeCalls {
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
