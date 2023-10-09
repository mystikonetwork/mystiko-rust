pub use mystiko_v2_axelar_erc20::*;
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
pub mod mystiko_v2_axelar_erc20 {
    const _: () = {
        ::core::include_bytes!("../json/MystikoV2AxelarERC20.json",);
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
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("execute"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("commandId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                kind: ::ethers_core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers_core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
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
                    ::std::borrow::ToOwned::to_owned("executeWithToken"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("executeWithToken"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("commandId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                kind: ::ethers_core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                kind: ::ethers_core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenSymbol"),
                                kind: ::ethers_core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("gateway"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("gateway"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract IAxelarGateway"
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("setAxelarGasReceiver"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAxelarGasReceiver",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_gasReceiver"),
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
                    ::std::borrow::ToOwned::to_owned("CallContractMessage"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CallContractMessage",),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("peerChainName"),
                                kind: ::ethers_core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("destinationAddress",),
                                kind: ::ethers_core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
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
                    ::std::borrow::ToOwned::to_owned("NotApprovedByGateway"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotApprovedByGateway",),
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
    pub static MYSTIKOV2AXELARERC20_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16t\x01@\xC5y#\x92K\\\\TU\xC4\x8D\x931q9\xAD\xDA\xC8\xFB\x17\x90U4\x80\x15b\0\08W`\0\x80\xFD[P`@Qb\x003\xAD8\x03\x80b\x003\xAD\x839\x81\x01`@\x81\x90Rb\0\0[\x91b\0\0\x98V[`\x0C\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x82\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U`\x0F\x80T\x90\x91\x16\x91\x90\x92\x16\x17\x90Ub\0\0\xF0V[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\xACW`\0\x80\xFD[\x82Qb\0\0\xB9\x81b\0\0\xD7V[` \x84\x01Q\x90\x92Pb\0\0\xCC\x81b\0\0\xD7V[\x80\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xEDW`\0\x80\xFD[PV[a2\xAD\x80b\0\x01\0`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x02jW`\x005`\xE0\x1C\x80c},\x85 \x11a\x01SW\x80c\xDDu|4\x11a\0\xCBW\x80c\xEA\x0C\xDE\x85\x11a\0\x7FW\x80c\xEDn\xA3:\x11a\0dW\x80c\xEDn\xA3:\x14a\x06\xCEW\x80c\xEF\xBF\xB2\xAE\x14a\x06\xEDW\x80c\xF4\xAD\x17\xC6\x14a\x07\x02W`\0\x80\xFD[\x80c\xEA\x0C\xDE\x85\x14a\x06\x8EW\x80c\xECW\x1Cj\x14a\x06\xAEW`\0\x80\xFD[\x80c\xDFB\x077\x11a\0\xB0W\x80c\xDFB\x077\x14a\x06.W\x80c\xE1\x9A\xBE\xF8\x14a\x06NW\x80c\xE8\x18<D\x14a\x06nW`\0\x80\xFD[\x80c\xDDu|4\x14a\x05\xFBW\x80c\xDD\xAC]\xC1\x14a\x06\x10W`\0\x80\xFD[\x80c\xB1\xC3\x94\"\x11a\x01\"W\x80c\xC9#\x0C]\x11a\x01\x07W\x80c\xC9#\x0C]\x14a\x05\x90W\x80c\xCD\xFC\xEE\xBA\x14a\x05\xA5W\x80c\xCF\xC7\xE2\xDA\x14a\x05\xE6W`\0\x80\xFD[\x80c\xB1\xC3\x94\"\x14a\x058W\x80c\xC2\xD4\x16\x01\x14a\x05iW`\0\x80\xFD[\x80c},\x85 \x14a\x04\xD0W\x80c\x82[_\x8D\x14a\x04\xF0W\x80c\x9A\x03cl\x14a\x05\x05W\x80c\xA3\xBCd\xF2\x14a\x05\x18W`\0\x80\xFD[\x80c$!\xE1U\x11a\x01\xE6W\x80cI\x16\x06X\x11a\x01\xB5W\x80cR\x1F\xF0W\x11a\x01\x9AW\x80cR\x1F\xF0W\x14a\x04{W\x80cX\x98\xA0\xA8\x14a\x04\x9BW\x80c^\x10\xB2\xB7\x14a\x04\xB0W`\0\x80\xFD[\x80cI\x16\x06X\x14a\x04FW\x80cN<\x10\xB7\x14a\x04fW`\0\x80\xFD[\x80c$!\xE1U\x14a\x03\xA4W\x80c,\xD2mE\x14a\x03\xEAW\x80c0\xF4\x9C\xAC\x14a\x04\nW\x80c?\xE34z\x14a\x04*W`\0\x80\xFD[\x80c\x15=\xC4P\x11a\x02=W\x80c\x19\xE7]n\x11a\x02\"W\x80c\x19\xE7]n\x14a\x03DW\x80c\x1A\x98\xB2\xE0\x14a\x03dW\x80c!\xE3-U\x14a\x03\x84W`\0\x80\xFD[\x80c\x15=\xC4P\x14a\x03\x02W\x80c\x17m\xE7\xA8\x14a\x03\"W`\0\x80\xFD[\x80c\x01\xDB\xF1\x9F\x14a\x02oW\x80c\x069L\x9B\x14a\x02\x86W\x80c\x0B\xA9Y\t\x14a\x02\xA6W\x80c\x11a\x91\xB6\x14a\x02\xCAW[`\0\x80\xFD[4\x80\x15a\x02{W`\0\x80\xFD[Pa\x02\x84a\x07\x17V[\0[4\x80\x15a\x02\x92W`\0\x80\xFD[Pa\x02\x84a\x02\xA16`\x04a)+V[a\x07\x9BV[4\x80\x15a\x02\xB2W`\0\x80\xFD[P`\x07T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xD6W`\0\x80\xFD[P`\rTa\x02\xEA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC1V[4\x80\x15a\x03\x0EW`\0\x80\xFD[Pa\x02\x84a\x03\x1D6`\x04a,5V[a\x08?V[4\x80\x15a\x03.W`\0\x80\xFD[Pa\x037a\x08\xA6V[`@Qa\x02\xC1\x91\x90a/\xFFV[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x02\x84a\x03_6`\x04a,5V[a\t,V[4\x80\x15a\x03pW`\0\x80\xFD[Pa\x02\x84a\x03\x7F6`\x04a*&V[a\t\x8BV[4\x80\x15a\x03\x90W`\0\x80\xFD[P`\x04Ta\x02\xEA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03\xB0W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81R\x7Faxelar\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x037V[4\x80\x15a\x03\xF6W`\0\x80\xFD[P`\x05Ta\x02\xEA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\x16W`\0\x80\xFD[Pa\x02\x84a\x04%6`\x04a)+V[a\x0B\rV[4\x80\x15a\x046W`\0\x80\xFD[P`\0`@Qa\x02\xC1\x91\x90a/\xD7V[4\x80\x15a\x04RW`\0\x80\xFD[Pa\x02\x84a\x04a6`\x04a)\x82V[a\x0B\x86V[4\x80\x15a\x04rW`\0\x80\xFD[Pa\x037a\x0C\x06V[4\x80\x15a\x04\x87W`\0\x80\xFD[Pa\x02\x84a\x04\x966`\x04a,5V[a\x0C\x94V[4\x80\x15a\x04\xA7W`\0\x80\xFD[P`\nTa\x02\xB7V[4\x80\x15a\x04\xBCW`\0\x80\xFD[Pa\x02\x84a\x04\xCB6`\x04a,5V[a\rGV[4\x80\x15a\x04\xDCW`\0\x80\xFD[Pa\x02\x84a\x04\xEB6`\x04a,\x89V[a\r\xA7V[4\x80\x15a\x04\xFCW`\0\x80\xFD[P`\x0BTa\x02\xB7V[a\x02\x84a\x05\x136`\x04a+wV[a\x0EDV[4\x80\x15a\x05$W`\0\x80\xFD[Pa\x02\x84a\x0536`\x04a)+V[a\x10TV[4\x80\x15a\x05DW`\0\x80\xFD[P`\0Ta\x05Y\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\xC1V[4\x80\x15a\x05uW`\0\x80\xFD[Pa\x05~a\x10\xA1V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\xC1V[4\x80\x15a\x05\x9CW`\0\x80\xFD[Pa\x037a\x11\x1EV[4\x80\x15a\x05\xB1W`\0\x80\xFD[P`\x02Ta\x05\xCD\x90`\x01`\xA0\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xC1V[4\x80\x15a\x05\xF2W`\0\x80\xFD[P`\x06Ta\x02\xB7V[4\x80\x15a\x06\x07W`\0\x80\xFD[Pa\x02\x84a\x11cV[4\x80\x15a\x06\x1CW`\0\x80\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xEAV[4\x80\x15a\x06:W`\0\x80\xFD[Pa\x02\x84a\x06I6`\x04a)+V[a\x11\xDBV[4\x80\x15a\x06ZW`\0\x80\xFD[Pa\x02\x84a\x06i6`\x04a)+V[a\x12(V[4\x80\x15a\x06zW`\0\x80\xFD[Pa\x02\x84a\x06\x896`\x04a,gV[a\x12uV[4\x80\x15a\x06\x9AW`\0\x80\xFD[Pa\x02\x84a\x06\xA96`\x04a)HV[a\x13\x08V[4\x80\x15a\x06\xBAW`\0\x80\xFD[P`\0Ta\x02\xEA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xDAW`\0\x80\xFD[P`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x05YV[4\x80\x15a\x06\xF9W`\0\x80\xFD[P`\x08Ta\x02\xB7V[4\x80\x15a\x07\x0EW`\0\x80\xFD[P`\tTa\x02\xB7V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07BW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x90\x81\x17\x91\x82\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x92a\x07\x91\x92\x90\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xC6W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x15a\x07\xF5W`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FG!\x12\x9E\x0Egn\xD6\xA9)\t\xBB$\xE8S\xCC\xDDc\xADr(\x0C\xC2\xE9t\xE3\x8EH\x0E\x0EnT\x90`\0\x90\xA2PV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08jW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n\x81\x90U`@Q\x81\x81R\x7F\x14\x98\x824\xD3\xE5\n\x12\xAE\xEC-n\xE5\x95\xB7\x0C/\xAE\x16<\xAF\xF2\x8B\xAA\xB5\x80+\x12+\xF2\xA7S\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x0FT`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\xEBW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t'\x91\x90\x81\x01\x90a+\0V[\x90P\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\tWW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x81\x90U`@Q\x81\x81R~\x91\xF5\xF5\xDB0\x92\xE3\x9E\xCBp\x12\x18\xD4\xAF \xB7W\x1E\x04)YY7\xC34\xF3\xAC\xD1O\xE2\xFE\x90` \x01a\x08\x9BV[`\0\x85\x85`@Qa\t\x9D\x92\x91\x90a.\"V[`@Q\x90\x81\x90\x03\x81 `\rTc\x18v\xEE\xD9`\xE0\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\x18v\xEE\xD9\x90a\t\xE4\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x89\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a/xV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xFEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n6\x91\x90a)eV[a\nSW`@Qc\x14\x03\x11-`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\0\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8E\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8C\x81R\x92P\x8C\x91P\x8B\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8B\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x89\x81R\x8C\x93P\x8B\x92P\x90\x8A\x90\x8A\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92Pa\x13\x80\x91PPV[PPPPPPPPPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B8W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xBE\x18\x08\x91~\t\x81O\xCA}\x80(\x9A\x97`\xA4\x06_\x10\xD6$\xAC\x1A\xEE\x0B\x9F\x90<O]\x92\xB1\x90` \x01a\x08\x9BV[a\x0B\xFD\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x87\x92P\x86\x91Pa\x13\x88\x90PV[PPPPPPPV[`\x03\x80Ta\x0C\x13\x90a1\xD4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C?\x90a1\xD4V[\x80\x15a\x0C\x8CW\x80`\x1F\x10a\x0CaWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\x8CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0CoW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xBFW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\r\x12W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fpeer minimal rollup fee\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x0B\x81\x90U`@Q\x81\x81R\x7F\x87\x80u\xA8\xE8\xAA\x1C|\x15\xE6\x93'RR\x0Fx\x12\xBFWDx^m\xF6\x0577)\xDAAUE\x90` \x01a\x08\x9BV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\rrW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t\x81\x90U`@Q\x81\x81R\x7F\x88\xD2\x10\xDF\xA1\x98\xF7Q\x95y)G!\xF9\x0Cw\x11S\xA7\xB4\x91\x01\xEE\xFB\x95\x14p7\xCC\x8C\xE4\x81\x90` \x01a\x08\x9BV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xD2W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x02\x17\x90U\x81Qa\x0E\x1F\x90`\x03\x90` \x85\x01\x90a'\xD2V[P`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPV[`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0EoW`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T\x81Q\x10\x15a\x0E\x93W`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T\x81Q\x11\x15a\x0E\xB7W`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T\x81`\xA0\x01Q\x10\x15a\x0E\xDEW`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT\x81`\xC0\x01Q\x10\x15a\x0F\x05W`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT\x81`\xE0\x01Q\x10\x15a\x0F,W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0FE\x82`@\x01Q\x83`\0\x01Q\x84``\x01Qa\x14\0V[\x90P\x80\x82` \x01Q\x14a\x0FkW`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Ft3a\x15(V[\x15a\x0F\x92W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`\xC0\x84\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x83\x01Q``\x82\x01R`\x80\x80\x84\x01Q\x90\x82\x01R`\0a\x0F\xD5\x82a\x15\xC5V[\x90Pa\x0F\xE5\x84`\xA0\x01Q\x82a\x164V[`\x02T`\xE0\x85\x01Q`\xC0\x86\x01Q\x86Qa\x10 \x93`\x01`\x01`\xA0\x1B\x03\x16\x92\x91a\x10\x0C\x91a1ZV[a\x10\x16\x91\x90a1ZV[\x86`\xA0\x01Qa\x17aV[` \x84\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\x7FW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x0FT`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1<\xE5g\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x10\xE6W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x10\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t'\x91\x90a-\nV[`\x0FT`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\xEBW`\0\x80\xFD[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\x8EW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90\x81\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x91a\x07\x91\x91`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\x06W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12SW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\xA0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x12\xC1W`@Qc`\x03\xE8!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x82\x90U`\x06\x81\x90U`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x7Fv3\0Lz\"\x98i\xAE\xEA\x10\xDBO\xF3\xE5~;\x154\xAE\xB2\xC9\xE7,]\xB2_\x96X\x95\xC30\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x133W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T\x82\x15\x15`\x01`\xA0\x1B\x02`\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x90U`@Q\x7F\xCD\x16,o\xC2B\x85\xBF\xBE9\x9E\xC0\xCC,\xE2\xC3\x80\xAD'\xD2\xEB\x1F\xCAA\x8Ce-\xF1%~~\t\x90a\x08\x9B\x90\x83\x15\x15\x81R` \x01\x90V[PPPPPPV[`\0a\x13\xC9\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x17\xCD\x92PPPV[`\x02T`\x04T\x91\x92Pa\x13\xF9\x91`\x01`\xA0\x1B\x90\x91\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90`\x01`\x01`\xA0\x1B\x03\x162\x84a\x18\x95V[PPPPPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x14CW`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x14uW`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x14\xCD\x91`\x04\x01a/GV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\xE5W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x1D\x91\x90a,NV[\x91PP[\x93\x92PPPV[`\0\x80T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x15BWP`\0\x91\x90PV[`\0T`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\x87W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x15\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xBF\x91\x90a)eV[\x92\x91PPV[``\x80a\x15\xD5\x83`\0\x01Qa\x19\x82V[a\x15\xE2\x84` \x01Qa\x19\x82V[a\x15\xEF\x85`@\x01Qa\x19\x82V[a\x15\xFC\x86``\x01Qa\x19\x82V[a\x16\t\x87`\x80\x01Qa\x1A\x1AV[`@Q` \x01a\x16\x1D\x95\x94\x93\x92\x91\x90a.}V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[`\x04T`\0\x90a\x16N\x90`\x01`\x01`\xA0\x1B\x03\x16`\x14a\x1AQV[\x90P\x82\x15a\x16\xC4W`\x0ET`@Qc\x0C\x93\xE3\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x0C\x93\xE3\xBB\x90\x85\x90a\x16\x91\x900\x90`\x03\x90\x87\x90\x89\x903\x90`\x04\x01a.\xE8V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x16\xAAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xBEW=`\0\x80>=`\0\xFD[PPPPP[\x7F\xE6\x8D\x82V\x94yX\x1B\xF9y;\x86r\xC8\xF4\x0Bm:\xD0TW\x19\xDC\x05y\xFB>\xE3\t\x19\xD3\xEF`\x03\x82`@Qa\x16\xF6\x92\x91\x90a0\x12V[`@Q\x80\x91\x03\x90\xA1`\x05T`@Qc\x1C\x92\x11_`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x1C\x92\x11_\x90a\x173\x90`\x03\x90\x85\x90\x87\x90`\x04\x01a07V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17MW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xFDW=`\0\x80>=`\0\xFD[\x804\x14a\x17\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Fbridge fee mismatch\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\tV[`\x0FTa\x17\xC8\x90`\x01`\x01`\xA0\x1B\x03\x163\x85\x85a\x1B\xFAV[PPPV[a\x17\xFF`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\x181`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\x18=\x84\x82a\x1CoV[\x90\x83R\x90Pa\x18L\x84\x82a\x1CoV[` \x84\x01\x91\x90\x91R\x90Pa\x18`\x84\x82a\x1CoV[`@\x84\x01\x91\x90\x91R\x90Pa\x18t\x84\x82a\x1CoV[``\x84\x01\x91\x90\x91R\x90Pa\x18\x88\x84\x82a\x1D\xA0V[P`\x80\x83\x01RP\x92\x91PPV[`\x04T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x18\xC3W`@Qc\x14@\xE0y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xA0\x1B\x90\x92\x04\x16\x14a\x18\xF9W`@Qc8v0O`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qa\x19\x18W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`@Qcx\xD6\x0C\xD7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx\xD6\x0C\xD7\x90a\x19J\x90\x84\x90\x86\x90`\x04\x01a0zV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19dW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19xW=`\0\x80>=`\0\xFD[PPPPPPPPV[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x19\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\r\tV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\x1A\nW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x19\xE8V[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\x1A(\x81a\x1E\xADV[\x83`@Q` \x01a\x1A:\x92\x91\x90a.NV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[```\0a\x1A`\x83`\x02a1rV[a\x1Ak\x90`\x02a1ZV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\x83Wa\x1A\x83a2;V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1A\xADW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x1A\xC8Wa\x1A\xC8a2%V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x1A\xF7Wa\x1A\xF7a2%V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x1B\x1B\x84`\x02a1rV[a\x1B&\x90`\x01a1ZV[\x90P[`\x01\x81\x11\x15a\x1B\xABW\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x0F\x16`\x10\x81\x10a\x1BgWa\x1Bga2%V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1B}Wa\x1B}a2%V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x1B\xA4\x81a1\xBDV[\x90Pa\x1B)V[P\x83\x15a\x15!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\r\tV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x1Ci\x90\x85\x90a\x1FzV[PPPPV[`\0\x80\x83Q\x83` a\x1C\x81\x91\x90a1ZV[\x11\x15\x80\x15a\x1C\x98WPa\x1C\x95\x83` a1ZV[\x83\x10[a\x1C\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\r\tV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1D%W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1D\x05V[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x1D\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\tV[\x80a\x1D\x93\x85` a1ZV[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a\x1D\xAF\x85\x85a _V[\x86Q\x90\x95P\x90\x91Pa\x1D\xC1\x82\x86a1ZV[\x11\x15\x80\x15a\x1D\xD7WPa\x1D\xD4\x81\x85a1ZV[\x84\x10[a\x1E/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\r\tV[``\x81\x15\x80\x15a\x1EJW`@Q\x91P` \x82\x01`@Ra\x1E\x94V[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a\x1E\x83W\x80Q\x83R` \x92\x83\x01\x92\x01a\x1EkV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a\x1E\xA0\x83\x87a1ZV[\x93P\x93PPP\x92P\x92\x90PV[```\xFD\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E\xE0W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91Ra\x15\xBFV[a\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1F0Wa\x1F\0`\xFD`\xF8\x1Ba\"eV[a\x1F\t\x83a\"\x8CV[`@Q` \x01a\x1F\x1A\x92\x91\x90a.NV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1F[Wa\x1FR`\x7F`\xF9\x1Ba\"eV[a\x1F\t\x83a\"\xCFV[a\x1Fl`\x01`\x01`\xF8\x1B\x03\x19a\"eV[a\x1F\t\x83a#\x12V[\x91\x90PV[`\0a\x1F\xCF\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a#U\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x17\xC8W\x80\x80` \x01\x90Q\x81\x01\x90a\x1F\xED\x91\x90a)eV[a\x17\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\tV[`\0\x80`\0a n\x85\x85a#lV[\x94P\x90P`\0`\xFD`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a!\x07Wa \x94\x86\x86a#\xF4V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a \xAFWPa\xFF\xFF\x81\x11\x15[a \xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\r\tV[\x92P\x83\x91Pa\x1D\x99\x90PV[`\x7F`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a!\x92Wa!'\x86\x86a$\xADV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a!FWPc\xFF\xFF\xFF\xFF\x81\x11\x15[a \xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\r\tV[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x14\x15a\"\x0FWa!\xAE\x86\x86a%\x7FV[\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a \xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\r\tV[P`\xF8\x81\x90\x1C`\xFD\x81\x10a \xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\r\tV[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a\x15\xBFV[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\"\xBFW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\"\x9DV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a#\x02W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\"\xE0V[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a#EW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a##V[PPP`(\x81\x01`@R\x92\x91PPV[``a#d\x84\x84`\0\x85a&QV[\x94\x93PPPPV[`\0\x80\x83Q\x83`\x01a#~\x91\x90a1ZV[\x11\x15\x80\x15a#\x95WPa#\x92\x83`\x01a1ZV[\x83\x10[a#\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\r\tV[\x83\x83\x01` \x01Q\x80a\x1D\x93\x85`\x01a1ZV[`\0\x80\x83Q\x83`\x02a$\x06\x91\x90a1ZV[\x11\x15\x80\x15a$\x1DWPa$\x1A\x83`\x02a1ZV[\x83\x10[a$tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\r\tV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a\x1D\x93\x91\x90a1ZV[`\0\x80\x83Q\x83`\x04a$\xBF\x91\x90a1ZV[\x11\x15\x80\x15a$\xD6WPa$\xD3\x83`\x04a1ZV[\x83\x10[a%-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\r\tV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a%bW\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa%BV[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a\x1D\x93\x85`\x04a1ZV[`\0\x80\x83Q\x83`\x08a%\x91\x91\x90a1ZV[\x11\x15\x80\x15a%\xA8WPa%\xA5\x83`\x08a1ZV[\x83\x10[a%\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\r\tV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a&4W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa&\x14V[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a\x1D\x93\x85`\x08a1ZV[``\x82G\x10\x15a&\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\tV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a' W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\r\tV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa'<\x91\x90a.2V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a'yW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'~V[``\x91P[P\x91P\x91Pa'\x8E\x82\x82\x86a'\x99V[\x97\x96PPPPPPPV[``\x83\x15a'\xA8WP\x81a\x15!V[\x82Q\x15a'\xB8W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\t\x91\x90a/\xFFV[\x82\x80Ta'\xDE\x90a1\xD4V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a(\0W`\0\x85Ua(FV[\x82`\x1F\x10a(\x19W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua(FV[\x82\x80\x01`\x01\x01\x85U\x82\x15a(FW\x91\x82\x01[\x82\x81\x11\x15a(FW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a(+V[Pa(R\x92\x91Pa(VV[P\x90V[[\x80\x82\x11\x15a(RW`\0\x81U`\x01\x01a(WV[`\0a(~a(y\x84a12V[a1\x01V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a(\x92W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a(\xBBW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xD3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1D\x99W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a(\xFCW`\0\x80\xFD[a\x15!\x83\x835` \x85\x01a(kV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1FuW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a)=W`\0\x80\xFD[\x815a\x15!\x81a2QV[`\0` \x82\x84\x03\x12\x15a)ZW`\0\x80\xFD[\x815a\x15!\x81a2iV[`\0` \x82\x84\x03\x12\x15a)wW`\0\x80\xFD[\x81Qa\x15!\x81a2iV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a)\x9DW`\0\x80\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a)\xBCW`\0\x80\xFD[a)\xC8\x8B\x83\x8C\x01a(\xA9V[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15a)\xE1W`\0\x80\xFD[a)\xED\x8B\x83\x8C\x01a(\xA9V[\x90\x96P\x94P``\x8A\x015\x91P\x80\x82\x11\x15a*\x06W`\0\x80\xFD[Pa*\x13\x8A\x82\x8B\x01a(\xA9V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x8B\x8D\x03\x12\x15a*EW`\0\x80\xFD[\x8A5\x99P` \x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*dW`\0\x80\xFD[a*p\x8E\x83\x8F\x01a(\xA9V[\x90\x9BP\x99P`@\x8D\x015\x91P\x80\x82\x11\x15a*\x89W`\0\x80\xFD[a*\x95\x8E\x83\x8F\x01a(\xA9V[\x90\x99P\x97P``\x8D\x015\x91P\x80\x82\x11\x15a*\xAEW`\0\x80\xFD[a*\xBA\x8E\x83\x8F\x01a(\xA9V[\x90\x97P\x95P`\x80\x8D\x015\x91P\x80\x82\x11\x15a*\xD3W`\0\x80\xFD[Pa*\xE0\x8D\x82\x8E\x01a(\xA9V[\x91P\x80\x94PP\x80\x92PP`\xA0\x8B\x015\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15a+\x12W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+)W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a+:W`\0\x80\xFD[\x80Qa+Ha(y\x82a12V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a+]W`\0\x80\xFD[a+n\x82` \x83\x01` \x86\x01a1\x91V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a+\x89W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a+\xA1W`\0\x80\xFD[\x90\x83\x01\x90a\x01\0\x82\x86\x03\x12\x15a+\xB6W`\0\x80\xFD[a+\xBEa0\xD7V[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01Ra+\xE2``\x84\x01a)\x0BV[``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a+\xF9W`\0\x80\xFD[a,\x05\x87\x82\x86\x01a(\xEBV[`\x80\x83\x01RP`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015`\xE0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a,GW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a,`W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a,zW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a,\x9EW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x82\x14a,\xB7W`\0\x80\xFD[\x90\x93P` \x85\x015\x90\x80\x82\x11\x15a,\xCDW`\0\x80\xFD[P\x84\x01`\x1F\x81\x01\x86\x13a,\xDFW`\0\x80\xFD[a,\xEE\x86\x825` \x84\x01a(kV[\x92PP`@\x84\x015a,\xFF\x81a2QV[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a-\x1CW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x15!W`\0\x80\xFD[`\0\x81Q\x80\x84Ra-E\x81` \x86\x01` \x86\x01a1\x91V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x80T`\0\x90`\x01\x81\x81\x1C\x90\x80\x83\x16\x80a-\x9CW`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x14\x15a-\xBEWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x83\x88R` \x88\x01\x82\x80\x15a-\xD9W`\x01\x81\x14a-\xEAWa.\x15V[`\xFF\x19\x87\x16\x82R\x82\x82\x01\x97Pa.\x15V[`\0\x89\x81R` \x90 `\0[\x87\x81\x10\x15a.\x0FW\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a-\xF6V[\x83\x01\x98PP[PPPPPPP\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82Qa.D\x81\x84` \x87\x01a1\x91V[\x91\x90\x91\x01\x92\x91PPV[`\0\x83Qa.`\x81\x84` \x88\x01a1\x91V[\x83Q\x90\x83\x01\x90a.t\x81\x83` \x88\x01a1\x91V[\x01\x94\x93PPPPV[`\0\x86Qa.\x8F\x81\x84` \x8B\x01a1\x91V[\x86Q\x90\x83\x01\x90a.\xA3\x81\x83` \x8B\x01a1\x91V[\x86Q\x91\x01\x90a.\xB6\x81\x83` \x8A\x01a1\x91V[\x85Q\x91\x01\x90a.\xC9\x81\x83` \x89\x01a1\x91V[\x84Q\x91\x01\x90a.\xDC\x81\x83` \x88\x01a1\x91V[\x01\x97\x96PPPPPPPV[`\0`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x83R`\xA0` \x84\x01Ra/\x0B`\xA0\x84\x01\x88a-\x82V[\x83\x81\x03`@\x85\x01Ra/\x1D\x81\x88a--V[\x90P\x83\x81\x03``\x85\x01Ra/1\x81\x87a--V[\x92PP\x80\x84\x16`\x80\x84\x01RP\x96\x95PPPPPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a/oW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a/PV[PPP\x92\x91PPV[\x89\x81R`\xC0` \x82\x01R`\0a/\x92`\xC0\x83\x01\x8A\x8Ca-YV[\x82\x81\x03`@\x84\x01Ra/\xA5\x81\x89\x8Ba-YV[\x90P\x86``\x84\x01R\x82\x81\x03`\x80\x84\x01Ra/\xC0\x81\x86\x88a-YV[\x91PP\x82`\xA0\x83\x01R\x9A\x99PPPPPPPPPPV[` \x81\x01`\x02\x83\x10a/\xF9WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x81R`\0a\x15!` \x83\x01\x84a--V[`@\x81R`\0a0%`@\x83\x01\x85a-\x82V[\x82\x81\x03` \x84\x01Ra\x15\x1D\x81\x85a--V[``\x81R`\0a0J``\x83\x01\x86a-\x82V[\x82\x81\x03` \x84\x01Ra0\\\x81\x86a--V[\x90P\x82\x81\x03`@\x84\x01Ra0p\x81\x85a--V[\x96\x95PPPPPPV[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra0\xBE`\xE0\x84\x01\x82a--V[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a0\xFBWa0\xFBa2;V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a1*Wa1*a2;V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a1LWa1La2;V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82\x19\x82\x11\x15a1mWa1ma2\x0FV[P\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a1\x8CWa1\x8Ca2\x0FV[P\x02\x90V[`\0[\x83\x81\x10\x15a1\xACW\x81\x81\x01Q\x83\x82\x01R` \x01a1\x94V[\x83\x81\x11\x15a\x1CiWPP`\0\x91\x01RV[`\0\x81a1\xCCWa1\xCCa2\x0FV[P`\0\x19\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a1\xE8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a2\tWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a2fW`\0\x80\xFD[PV[\x80\x15\x15\x81\x14a2fW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xB0\x98)\x01\xF4\xD4\x10\xDD\xD5>\xEF\xB5\x8C\x82\x12\xBD\xF4\xFE\x92\xC7cW%\x0C\xE9B\xBDCo\xAF\x89\xD5dsolcC\0\x08\x07\x003";
    /// The bytecode of the contract.
    pub static MYSTIKOV2AXELARERC20_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02jW`\x005`\xE0\x1C\x80c},\x85 \x11a\x01SW\x80c\xDDu|4\x11a\0\xCBW\x80c\xEA\x0C\xDE\x85\x11a\0\x7FW\x80c\xEDn\xA3:\x11a\0dW\x80c\xEDn\xA3:\x14a\x06\xCEW\x80c\xEF\xBF\xB2\xAE\x14a\x06\xEDW\x80c\xF4\xAD\x17\xC6\x14a\x07\x02W`\0\x80\xFD[\x80c\xEA\x0C\xDE\x85\x14a\x06\x8EW\x80c\xECW\x1Cj\x14a\x06\xAEW`\0\x80\xFD[\x80c\xDFB\x077\x11a\0\xB0W\x80c\xDFB\x077\x14a\x06.W\x80c\xE1\x9A\xBE\xF8\x14a\x06NW\x80c\xE8\x18<D\x14a\x06nW`\0\x80\xFD[\x80c\xDDu|4\x14a\x05\xFBW\x80c\xDD\xAC]\xC1\x14a\x06\x10W`\0\x80\xFD[\x80c\xB1\xC3\x94\"\x11a\x01\"W\x80c\xC9#\x0C]\x11a\x01\x07W\x80c\xC9#\x0C]\x14a\x05\x90W\x80c\xCD\xFC\xEE\xBA\x14a\x05\xA5W\x80c\xCF\xC7\xE2\xDA\x14a\x05\xE6W`\0\x80\xFD[\x80c\xB1\xC3\x94\"\x14a\x058W\x80c\xC2\xD4\x16\x01\x14a\x05iW`\0\x80\xFD[\x80c},\x85 \x14a\x04\xD0W\x80c\x82[_\x8D\x14a\x04\xF0W\x80c\x9A\x03cl\x14a\x05\x05W\x80c\xA3\xBCd\xF2\x14a\x05\x18W`\0\x80\xFD[\x80c$!\xE1U\x11a\x01\xE6W\x80cI\x16\x06X\x11a\x01\xB5W\x80cR\x1F\xF0W\x11a\x01\x9AW\x80cR\x1F\xF0W\x14a\x04{W\x80cX\x98\xA0\xA8\x14a\x04\x9BW\x80c^\x10\xB2\xB7\x14a\x04\xB0W`\0\x80\xFD[\x80cI\x16\x06X\x14a\x04FW\x80cN<\x10\xB7\x14a\x04fW`\0\x80\xFD[\x80c$!\xE1U\x14a\x03\xA4W\x80c,\xD2mE\x14a\x03\xEAW\x80c0\xF4\x9C\xAC\x14a\x04\nW\x80c?\xE34z\x14a\x04*W`\0\x80\xFD[\x80c\x15=\xC4P\x11a\x02=W\x80c\x19\xE7]n\x11a\x02\"W\x80c\x19\xE7]n\x14a\x03DW\x80c\x1A\x98\xB2\xE0\x14a\x03dW\x80c!\xE3-U\x14a\x03\x84W`\0\x80\xFD[\x80c\x15=\xC4P\x14a\x03\x02W\x80c\x17m\xE7\xA8\x14a\x03\"W`\0\x80\xFD[\x80c\x01\xDB\xF1\x9F\x14a\x02oW\x80c\x069L\x9B\x14a\x02\x86W\x80c\x0B\xA9Y\t\x14a\x02\xA6W\x80c\x11a\x91\xB6\x14a\x02\xCAW[`\0\x80\xFD[4\x80\x15a\x02{W`\0\x80\xFD[Pa\x02\x84a\x07\x17V[\0[4\x80\x15a\x02\x92W`\0\x80\xFD[Pa\x02\x84a\x02\xA16`\x04a)+V[a\x07\x9BV[4\x80\x15a\x02\xB2W`\0\x80\xFD[P`\x07T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xD6W`\0\x80\xFD[P`\rTa\x02\xEA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC1V[4\x80\x15a\x03\x0EW`\0\x80\xFD[Pa\x02\x84a\x03\x1D6`\x04a,5V[a\x08?V[4\x80\x15a\x03.W`\0\x80\xFD[Pa\x037a\x08\xA6V[`@Qa\x02\xC1\x91\x90a/\xFFV[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x02\x84a\x03_6`\x04a,5V[a\t,V[4\x80\x15a\x03pW`\0\x80\xFD[Pa\x02\x84a\x03\x7F6`\x04a*&V[a\t\x8BV[4\x80\x15a\x03\x90W`\0\x80\xFD[P`\x04Ta\x02\xEA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03\xB0W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81R\x7Faxelar\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x037V[4\x80\x15a\x03\xF6W`\0\x80\xFD[P`\x05Ta\x02\xEA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\x16W`\0\x80\xFD[Pa\x02\x84a\x04%6`\x04a)+V[a\x0B\rV[4\x80\x15a\x046W`\0\x80\xFD[P`\0`@Qa\x02\xC1\x91\x90a/\xD7V[4\x80\x15a\x04RW`\0\x80\xFD[Pa\x02\x84a\x04a6`\x04a)\x82V[a\x0B\x86V[4\x80\x15a\x04rW`\0\x80\xFD[Pa\x037a\x0C\x06V[4\x80\x15a\x04\x87W`\0\x80\xFD[Pa\x02\x84a\x04\x966`\x04a,5V[a\x0C\x94V[4\x80\x15a\x04\xA7W`\0\x80\xFD[P`\nTa\x02\xB7V[4\x80\x15a\x04\xBCW`\0\x80\xFD[Pa\x02\x84a\x04\xCB6`\x04a,5V[a\rGV[4\x80\x15a\x04\xDCW`\0\x80\xFD[Pa\x02\x84a\x04\xEB6`\x04a,\x89V[a\r\xA7V[4\x80\x15a\x04\xFCW`\0\x80\xFD[P`\x0BTa\x02\xB7V[a\x02\x84a\x05\x136`\x04a+wV[a\x0EDV[4\x80\x15a\x05$W`\0\x80\xFD[Pa\x02\x84a\x0536`\x04a)+V[a\x10TV[4\x80\x15a\x05DW`\0\x80\xFD[P`\0Ta\x05Y\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\xC1V[4\x80\x15a\x05uW`\0\x80\xFD[Pa\x05~a\x10\xA1V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\xC1V[4\x80\x15a\x05\x9CW`\0\x80\xFD[Pa\x037a\x11\x1EV[4\x80\x15a\x05\xB1W`\0\x80\xFD[P`\x02Ta\x05\xCD\x90`\x01`\xA0\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xC1V[4\x80\x15a\x05\xF2W`\0\x80\xFD[P`\x06Ta\x02\xB7V[4\x80\x15a\x06\x07W`\0\x80\xFD[Pa\x02\x84a\x11cV[4\x80\x15a\x06\x1CW`\0\x80\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xEAV[4\x80\x15a\x06:W`\0\x80\xFD[Pa\x02\x84a\x06I6`\x04a)+V[a\x11\xDBV[4\x80\x15a\x06ZW`\0\x80\xFD[Pa\x02\x84a\x06i6`\x04a)+V[a\x12(V[4\x80\x15a\x06zW`\0\x80\xFD[Pa\x02\x84a\x06\x896`\x04a,gV[a\x12uV[4\x80\x15a\x06\x9AW`\0\x80\xFD[Pa\x02\x84a\x06\xA96`\x04a)HV[a\x13\x08V[4\x80\x15a\x06\xBAW`\0\x80\xFD[P`\0Ta\x02\xEA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xDAW`\0\x80\xFD[P`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x05YV[4\x80\x15a\x06\xF9W`\0\x80\xFD[P`\x08Ta\x02\xB7V[4\x80\x15a\x07\x0EW`\0\x80\xFD[P`\tTa\x02\xB7V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07BW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x90\x81\x17\x91\x82\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x92a\x07\x91\x92\x90\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xC6W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x15a\x07\xF5W`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FG!\x12\x9E\x0Egn\xD6\xA9)\t\xBB$\xE8S\xCC\xDDc\xADr(\x0C\xC2\xE9t\xE3\x8EH\x0E\x0EnT\x90`\0\x90\xA2PV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08jW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n\x81\x90U`@Q\x81\x81R\x7F\x14\x98\x824\xD3\xE5\n\x12\xAE\xEC-n\xE5\x95\xB7\x0C/\xAE\x16<\xAF\xF2\x8B\xAA\xB5\x80+\x12+\xF2\xA7S\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x0FT`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\xEBW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t'\x91\x90\x81\x01\x90a+\0V[\x90P\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\tWW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x81\x90U`@Q\x81\x81R~\x91\xF5\xF5\xDB0\x92\xE3\x9E\xCBp\x12\x18\xD4\xAF \xB7W\x1E\x04)YY7\xC34\xF3\xAC\xD1O\xE2\xFE\x90` \x01a\x08\x9BV[`\0\x85\x85`@Qa\t\x9D\x92\x91\x90a.\"V[`@Q\x90\x81\x90\x03\x81 `\rTc\x18v\xEE\xD9`\xE0\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\x18v\xEE\xD9\x90a\t\xE4\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x89\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a/xV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xFEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n6\x91\x90a)eV[a\nSW`@Qc\x14\x03\x11-`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\0\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8E\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8C\x81R\x92P\x8C\x91P\x8B\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8B\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x89\x81R\x8C\x93P\x8B\x92P\x90\x8A\x90\x8A\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92Pa\x13\x80\x91PPV[PPPPPPPPPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B8W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xBE\x18\x08\x91~\t\x81O\xCA}\x80(\x9A\x97`\xA4\x06_\x10\xD6$\xAC\x1A\xEE\x0B\x9F\x90<O]\x92\xB1\x90` \x01a\x08\x9BV[a\x0B\xFD\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x87\x92P\x86\x91Pa\x13\x88\x90PV[PPPPPPPV[`\x03\x80Ta\x0C\x13\x90a1\xD4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C?\x90a1\xD4V[\x80\x15a\x0C\x8CW\x80`\x1F\x10a\x0CaWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\x8CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0CoW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xBFW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\r\x12W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fpeer minimal rollup fee\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x0B\x81\x90U`@Q\x81\x81R\x7F\x87\x80u\xA8\xE8\xAA\x1C|\x15\xE6\x93'RR\x0Fx\x12\xBFWDx^m\xF6\x0577)\xDAAUE\x90` \x01a\x08\x9BV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\rrW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\t\x81\x90U`@Q\x81\x81R\x7F\x88\xD2\x10\xDF\xA1\x98\xF7Q\x95y)G!\xF9\x0Cw\x11S\xA7\xB4\x91\x01\xEE\xFB\x95\x14p7\xCC\x8C\xE4\x81\x90` \x01a\x08\x9BV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xD2W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x02\x17\x90U\x81Qa\x0E\x1F\x90`\x03\x90` \x85\x01\x90a'\xD2V[P`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPV[`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0EoW`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T\x81Q\x10\x15a\x0E\x93W`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T\x81Q\x11\x15a\x0E\xB7W`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T\x81`\xA0\x01Q\x10\x15a\x0E\xDEW`@Qc\xC4\xD8\xD0\r`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT\x81`\xC0\x01Q\x10\x15a\x0F\x05W`@QcU\xA6\xD6\xA1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT\x81`\xE0\x01Q\x10\x15a\x0F,W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0FE\x82`@\x01Q\x83`\0\x01Q\x84``\x01Qa\x14\0V[\x90P\x80\x82` \x01Q\x14a\x0FkW`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Ft3a\x15(V[\x15a\x0F\x92W`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`\xC0\x84\x01Q\x91\x81\x01\x91\x90\x91R`\xE0\x83\x01Q``\x82\x01R`\x80\x80\x84\x01Q\x90\x82\x01R`\0a\x0F\xD5\x82a\x15\xC5V[\x90Pa\x0F\xE5\x84`\xA0\x01Q\x82a\x164V[`\x02T`\xE0\x85\x01Q`\xC0\x86\x01Q\x86Qa\x10 \x93`\x01`\x01`\xA0\x1B\x03\x16\x92\x91a\x10\x0C\x91a1ZV[a\x10\x16\x91\x90a1ZV[\x86`\xA0\x01Qa\x17aV[` \x84\x01Q`@Q\x7F\xD1\x06\xEB8\xB36\x8B|)N6\xFA\xE5Q?\xDE\xFE\x88\x0B\xE5\xAB\xFA\xD5)\xB3{\x04O/\xDD-\xBE\x90`\0\x90\xA2PPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\x7FW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x0FT`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1<\xE5g\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x10\xE6W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x10\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t'\x91\x90a-\nV[`\x0FT`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\xEBW`\0\x80\xFD[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\x8EW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90\x81\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x91a\x07\x91\x91`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\x06W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12SW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\xA0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x12\xC1W`@Qc`\x03\xE8!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x82\x90U`\x06\x81\x90U`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x7Fv3\0Lz\"\x98i\xAE\xEA\x10\xDBO\xF3\xE5~;\x154\xAE\xB2\xC9\xE7,]\xB2_\x96X\x95\xC30\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x133W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T\x82\x15\x15`\x01`\xA0\x1B\x02`\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x90U`@Q\x7F\xCD\x16,o\xC2B\x85\xBF\xBE9\x9E\xC0\xCC,\xE2\xC3\x80\xAD'\xD2\xEB\x1F\xCAA\x8Ce-\xF1%~~\t\x90a\x08\x9B\x90\x83\x15\x15\x81R` \x01\x90V[PPPPPPV[`\0a\x13\xC9\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x17\xCD\x92PPPV[`\x02T`\x04T\x91\x92Pa\x13\xF9\x91`\x01`\xA0\x1B\x90\x91\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90`\x01`\x01`\xA0\x1B\x03\x162\x84a\x18\x95V[PPPPPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x14CW`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x14uW`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x14\xCD\x91`\x04\x01a/GV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\xE5W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x1D\x91\x90a,NV[\x91PP[\x93\x92PPPV[`\0\x80T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x15BWP`\0\x91\x90PV[`\0T`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\x87W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x15\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xBF\x91\x90a)eV[\x92\x91PPV[``\x80a\x15\xD5\x83`\0\x01Qa\x19\x82V[a\x15\xE2\x84` \x01Qa\x19\x82V[a\x15\xEF\x85`@\x01Qa\x19\x82V[a\x15\xFC\x86``\x01Qa\x19\x82V[a\x16\t\x87`\x80\x01Qa\x1A\x1AV[`@Q` \x01a\x16\x1D\x95\x94\x93\x92\x91\x90a.}V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[`\x04T`\0\x90a\x16N\x90`\x01`\x01`\xA0\x1B\x03\x16`\x14a\x1AQV[\x90P\x82\x15a\x16\xC4W`\x0ET`@Qc\x0C\x93\xE3\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x0C\x93\xE3\xBB\x90\x85\x90a\x16\x91\x900\x90`\x03\x90\x87\x90\x89\x903\x90`\x04\x01a.\xE8V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x16\xAAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xBEW=`\0\x80>=`\0\xFD[PPPPP[\x7F\xE6\x8D\x82V\x94yX\x1B\xF9y;\x86r\xC8\xF4\x0Bm:\xD0TW\x19\xDC\x05y\xFB>\xE3\t\x19\xD3\xEF`\x03\x82`@Qa\x16\xF6\x92\x91\x90a0\x12V[`@Q\x80\x91\x03\x90\xA1`\x05T`@Qc\x1C\x92\x11_`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x1C\x92\x11_\x90a\x173\x90`\x03\x90\x85\x90\x87\x90`\x04\x01a07V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17MW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xFDW=`\0\x80>=`\0\xFD[\x804\x14a\x17\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Fbridge fee mismatch\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\tV[`\x0FTa\x17\xC8\x90`\x01`\x01`\xA0\x1B\x03\x163\x85\x85a\x1B\xFAV[PPPV[a\x17\xFF`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[a\x181`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[`\0a\x18=\x84\x82a\x1CoV[\x90\x83R\x90Pa\x18L\x84\x82a\x1CoV[` \x84\x01\x91\x90\x91R\x90Pa\x18`\x84\x82a\x1CoV[`@\x84\x01\x91\x90\x91R\x90Pa\x18t\x84\x82a\x1CoV[``\x84\x01\x91\x90\x91R\x90Pa\x18\x88\x84\x82a\x1D\xA0V[P`\x80\x83\x01RP\x92\x91PPV[`\x04T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x18\xC3W`@Qc\x14@\xE0y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xA0\x1B\x90\x92\x04\x16\x14a\x18\xF9W`@Qc8v0O`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qa\x19\x18W`@Qc\x82\x0B\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`@Qcx\xD6\x0C\xD7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx\xD6\x0C\xD7\x90a\x19J\x90\x84\x90\x86\x90`\x04\x01a0zV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19dW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19xW=`\0\x80>=`\0\xFD[PPPPPPPPV[```\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x19\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FValue exceeds uint255 range\0\0\0\0\0`D\x82\x01R`d\x01a\r\tV[`@Q` \x80\x82R`\0`\x1F[\x82\x82\x10\x15a\x1A\nW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\x19\xE8V[PPP`@\x81\x81\x01\x90R\x92\x91PPV[\x80Q``\x90a\x1A(\x81a\x1E\xADV[\x83`@Q` \x01a\x1A:\x92\x91\x90a.NV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[```\0a\x1A`\x83`\x02a1rV[a\x1Ak\x90`\x02a1ZV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\x83Wa\x1A\x83a2;V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1A\xADW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x1A\xC8Wa\x1A\xC8a2%V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x1A\xF7Wa\x1A\xF7a2%V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x1B\x1B\x84`\x02a1rV[a\x1B&\x90`\x01a1ZV[\x90P[`\x01\x81\x11\x15a\x1B\xABW\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x0F\x16`\x10\x81\x10a\x1BgWa\x1Bga2%V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1B}Wa\x1B}a2%V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x1B\xA4\x81a1\xBDV[\x90Pa\x1B)V[P\x83\x15a\x15!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\r\tV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x1Ci\x90\x85\x90a\x1FzV[PPPPV[`\0\x80\x83Q\x83` a\x1C\x81\x91\x90a1ZV[\x11\x15\x80\x15a\x1C\x98WPa\x1C\x95\x83` a1ZV[\x83\x10[a\x1C\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FNextUint255, offset exceeds maxi`D\x82\x01Rbmum`\xE8\x1B`d\x82\x01R`\x84\x01a\r\tV[`\0`@Q` `\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a\x1D%W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa\x1D\x05V[PPP\x81\x01`@RQ\x90P`\x01`\x01`\xFF\x1B\x03\x81\x11\x15a\x1D\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FValue exceeds the range\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\tV[\x80a\x1D\x93\x85` a1ZV[\x92P\x92PP[\x92P\x92\x90PV[```\0\x80a\x1D\xAF\x85\x85a _V[\x86Q\x90\x95P\x90\x91Pa\x1D\xC1\x82\x86a1ZV[\x11\x15\x80\x15a\x1D\xD7WPa\x1D\xD4\x81\x85a1ZV[\x84\x10[a\x1E/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FNextVarBytes, offset exceeds max`D\x82\x01Rcimum`\xE0\x1B`d\x82\x01R`\x84\x01a\r\tV[``\x81\x15\x80\x15a\x1EJW`@Q\x91P` \x82\x01`@Ra\x1E\x94V[`@Q\x91P`\x1F\x83\x16\x80\x15` \x02\x81\x84\x01\x01\x84\x81\x01\x88\x83\x15` \x02\x84\x8C\x01\x01\x01[\x81\x83\x10\x15a\x1E\x83W\x80Q\x83R` \x92\x83\x01\x92\x01a\x1EkV[PP\x84\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x80a\x1E\xA0\x83\x87a1ZV[\x93P\x93PPP\x92P\x92\x90PV[```\xFD\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E\xE0W`@\x80Q`\x01\x81R`\xF8\x84\x90\x1B` \x82\x01R`!\x81\x01\x90\x91Ra\x15\xBFV[a\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1F0Wa\x1F\0`\xFD`\xF8\x1Ba\"eV[a\x1F\t\x83a\"\x8CV[`@Q` \x01a\x1F\x1A\x92\x91\x90a.NV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[c\xFF\xFF\xFF\xFF\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1F[Wa\x1FR`\x7F`\xF9\x1Ba\"eV[a\x1F\t\x83a\"\xCFV[a\x1Fl`\x01`\x01`\xF8\x1B\x03\x19a\"eV[a\x1F\t\x83a#\x12V[\x91\x90PV[`\0a\x1F\xCF\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a#U\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x17\xC8W\x80\x80` \x01\x90Q\x81\x01\x90a\x1F\xED\x91\x90a)eV[a\x17\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\tV[`\0\x80`\0a n\x85\x85a#lV[\x94P\x90P`\0`\xFD`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a!\x07Wa \x94\x86\x86a#\xF4V[\x95Pa\xFF\xFF\x16\x90P`\xFD\x81\x10\x80\x15\x90a \xAFWPa\xFF\xFF\x81\x11\x15[a \xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNextUint16, value outside range\0`D\x82\x01R`d\x01a\r\tV[\x92P\x83\x91Pa\x1D\x99\x90PV[`\x7F`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x83\x16\x14\x15a!\x92Wa!'\x86\x86a$\xADV[\x95Pc\xFF\xFF\xFF\xFF\x16\x90Pa\xFF\xFF\x81\x11\x80\x15a!FWPc\xFF\xFF\xFF\xFF\x81\x11\x15[a \xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\r\tV[`\x01`\x01`\xF8\x1B\x03\x19\x80\x83\x16\x14\x15a\"\x0FWa!\xAE\x86\x86a%\x7FV[\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pc\xFF\xFF\xFF\xFF\x81\x11a \xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\r\tV[P`\xF8\x81\x90\x1C`\xFD\x81\x10a \xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextVarUint, value outside range`D\x82\x01R`d\x01a\r\tV[`@\x80Q`\x01\x81R`\x01`\x01`\xF8\x1B\x03\x19\x83\x16` \x82\x01R`!\x81\x01\x90\x91R``\x90a\x15\xBFV[`@Q`\x02\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a\"\xBFW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\"\x9DV[PPP`\"\x81\x01`@R\x92\x91PPV[`@Q`\x04\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a#\x02W\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a\"\xE0V[PPP`$\x81\x01`@R\x92\x91PPV[`@Q`\x08\x80\x82R``\x91\x90`\0`\x1F[\x82\x82\x10\x15a#EW\x85\x81\x1A\x82` \x86\x01\x01S`\x01\x91\x90\x91\x01\x90`\0\x19\x01a##V[PPP`(\x81\x01`@R\x92\x91PPV[``a#d\x84\x84`\0\x85a&QV[\x94\x93PPPPV[`\0\x80\x83Q\x83`\x01a#~\x91\x90a1ZV[\x11\x15\x80\x15a#\x95WPa#\x92\x83`\x01a1ZV[\x83\x10[a#\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNextByte, Offset exceeds maximum`D\x82\x01R`d\x01a\r\tV[\x83\x83\x01` \x01Q\x80a\x1D\x93\x85`\x01a1ZV[`\0\x80\x83Q\x83`\x02a$\x06\x91\x90a1ZV[\x11\x15\x80\x15a$\x1DWPa$\x1A\x83`\x02a1ZV[\x83\x10[a$tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint16, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\r\tV[`\0`@Q\x84` \x87\x01\x01Q\x80`\x01\x1A\x82S\x80`\0\x1A`\x01\x83\x01SP`\x02\x81\x01`@R`\x1E\x81\x03Q\x91PP\x80\x84`\x02a\x1D\x93\x91\x90a1ZV[`\0\x80\x83Q\x83`\x04a$\xBF\x91\x90a1ZV[\x11\x15\x80\x15a$\xD6WPa$\xD3\x83`\x04a1ZV[\x83\x10[a%-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint32, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\r\tV[`\0`@Q`\x04`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a%bW\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa%BV[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a\x1D\x93\x85`\x04a1ZV[`\0\x80\x83Q\x83`\x08a%\x91\x91\x90a1ZV[\x11\x15\x80\x15a%\xA8WPa%\xA5\x83`\x08a1ZV[\x83\x10[a%\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNextUint64, offset exceeds maxim`D\x82\x01Raum`\xF0\x1B`d\x82\x01R`\x84\x01a\r\tV[`\0`@Q`\x08`\0`\x01\x82\x03\x87` \x8A\x01\x01Q[\x83\x83\x10\x15a&4W\x80\x82\x1A\x83\x86\x01S`\x01\x83\x01\x92P`\x01\x82\x03\x91Pa&\x14V[PPP\x81\x81\x01`@R` \x03\x90\x03Q\x90P\x80a\x1D\x93\x85`\x08a1ZV[``\x82G\x10\x15a&\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\tV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a' W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\r\tV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa'<\x91\x90a.2V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a'yW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'~V[``\x91P[P\x91P\x91Pa'\x8E\x82\x82\x86a'\x99V[\x97\x96PPPPPPPV[``\x83\x15a'\xA8WP\x81a\x15!V[\x82Q\x15a'\xB8W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\t\x91\x90a/\xFFV[\x82\x80Ta'\xDE\x90a1\xD4V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a(\0W`\0\x85Ua(FV[\x82`\x1F\x10a(\x19W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua(FV[\x82\x80\x01`\x01\x01\x85U\x82\x15a(FW\x91\x82\x01[\x82\x81\x11\x15a(FW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a(+V[Pa(R\x92\x91Pa(VV[P\x90V[[\x80\x82\x11\x15a(RW`\0\x81U`\x01\x01a(WV[`\0a(~a(y\x84a12V[a1\x01V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a(\x92W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a(\xBBW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xD3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1D\x99W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a(\xFCW`\0\x80\xFD[a\x15!\x83\x835` \x85\x01a(kV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1FuW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a)=W`\0\x80\xFD[\x815a\x15!\x81a2QV[`\0` \x82\x84\x03\x12\x15a)ZW`\0\x80\xFD[\x815a\x15!\x81a2iV[`\0` \x82\x84\x03\x12\x15a)wW`\0\x80\xFD[\x81Qa\x15!\x81a2iV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a)\x9DW`\0\x80\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a)\xBCW`\0\x80\xFD[a)\xC8\x8B\x83\x8C\x01a(\xA9V[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15a)\xE1W`\0\x80\xFD[a)\xED\x8B\x83\x8C\x01a(\xA9V[\x90\x96P\x94P``\x8A\x015\x91P\x80\x82\x11\x15a*\x06W`\0\x80\xFD[Pa*\x13\x8A\x82\x8B\x01a(\xA9V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x8B\x8D\x03\x12\x15a*EW`\0\x80\xFD[\x8A5\x99P` \x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*dW`\0\x80\xFD[a*p\x8E\x83\x8F\x01a(\xA9V[\x90\x9BP\x99P`@\x8D\x015\x91P\x80\x82\x11\x15a*\x89W`\0\x80\xFD[a*\x95\x8E\x83\x8F\x01a(\xA9V[\x90\x99P\x97P``\x8D\x015\x91P\x80\x82\x11\x15a*\xAEW`\0\x80\xFD[a*\xBA\x8E\x83\x8F\x01a(\xA9V[\x90\x97P\x95P`\x80\x8D\x015\x91P\x80\x82\x11\x15a*\xD3W`\0\x80\xFD[Pa*\xE0\x8D\x82\x8E\x01a(\xA9V[\x91P\x80\x94PP\x80\x92PP`\xA0\x8B\x015\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15a+\x12W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+)W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a+:W`\0\x80\xFD[\x80Qa+Ha(y\x82a12V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a+]W`\0\x80\xFD[a+n\x82` \x83\x01` \x86\x01a1\x91V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a+\x89W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a+\xA1W`\0\x80\xFD[\x90\x83\x01\x90a\x01\0\x82\x86\x03\x12\x15a+\xB6W`\0\x80\xFD[a+\xBEa0\xD7V[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01Ra+\xE2``\x84\x01a)\x0BV[``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a+\xF9W`\0\x80\xFD[a,\x05\x87\x82\x86\x01a(\xEBV[`\x80\x83\x01RP`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015`\xE0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a,GW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a,`W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a,zW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a,\x9EW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x82\x14a,\xB7W`\0\x80\xFD[\x90\x93P` \x85\x015\x90\x80\x82\x11\x15a,\xCDW`\0\x80\xFD[P\x84\x01`\x1F\x81\x01\x86\x13a,\xDFW`\0\x80\xFD[a,\xEE\x86\x825` \x84\x01a(kV[\x92PP`@\x84\x015a,\xFF\x81a2QV[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a-\x1CW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x15!W`\0\x80\xFD[`\0\x81Q\x80\x84Ra-E\x81` \x86\x01` \x86\x01a1\x91V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x80T`\0\x90`\x01\x81\x81\x1C\x90\x80\x83\x16\x80a-\x9CW`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x14\x15a-\xBEWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x83\x88R` \x88\x01\x82\x80\x15a-\xD9W`\x01\x81\x14a-\xEAWa.\x15V[`\xFF\x19\x87\x16\x82R\x82\x82\x01\x97Pa.\x15V[`\0\x89\x81R` \x90 `\0[\x87\x81\x10\x15a.\x0FW\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a-\xF6V[\x83\x01\x98PP[PPPPPPP\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82Qa.D\x81\x84` \x87\x01a1\x91V[\x91\x90\x91\x01\x92\x91PPV[`\0\x83Qa.`\x81\x84` \x88\x01a1\x91V[\x83Q\x90\x83\x01\x90a.t\x81\x83` \x88\x01a1\x91V[\x01\x94\x93PPPPV[`\0\x86Qa.\x8F\x81\x84` \x8B\x01a1\x91V[\x86Q\x90\x83\x01\x90a.\xA3\x81\x83` \x8B\x01a1\x91V[\x86Q\x91\x01\x90a.\xB6\x81\x83` \x8A\x01a1\x91V[\x85Q\x91\x01\x90a.\xC9\x81\x83` \x89\x01a1\x91V[\x84Q\x91\x01\x90a.\xDC\x81\x83` \x88\x01a1\x91V[\x01\x97\x96PPPPPPPV[`\0`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x83R`\xA0` \x84\x01Ra/\x0B`\xA0\x84\x01\x88a-\x82V[\x83\x81\x03`@\x85\x01Ra/\x1D\x81\x88a--V[\x90P\x83\x81\x03``\x85\x01Ra/1\x81\x87a--V[\x92PP\x80\x84\x16`\x80\x84\x01RP\x96\x95PPPPPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a/oW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a/PV[PPP\x92\x91PPV[\x89\x81R`\xC0` \x82\x01R`\0a/\x92`\xC0\x83\x01\x8A\x8Ca-YV[\x82\x81\x03`@\x84\x01Ra/\xA5\x81\x89\x8Ba-YV[\x90P\x86``\x84\x01R\x82\x81\x03`\x80\x84\x01Ra/\xC0\x81\x86\x88a-YV[\x91PP\x82`\xA0\x83\x01R\x9A\x99PPPPPPPPPPV[` \x81\x01`\x02\x83\x10a/\xF9WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x81R`\0a\x15!` \x83\x01\x84a--V[`@\x81R`\0a0%`@\x83\x01\x85a-\x82V[\x82\x81\x03` \x84\x01Ra\x15\x1D\x81\x85a--V[``\x81R`\0a0J``\x83\x01\x86a-\x82V[\x82\x81\x03` \x84\x01Ra0\\\x81\x86a--V[\x90P\x82\x81\x03`@\x84\x01Ra0p\x81\x85a--V[\x96\x95PPPPPPV[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra0\xBE`\xE0\x84\x01\x82a--V[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a0\xFBWa0\xFBa2;V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a1*Wa1*a2;V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a1LWa1La2;V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82\x19\x82\x11\x15a1mWa1ma2\x0FV[P\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a1\x8CWa1\x8Ca2\x0FV[P\x02\x90V[`\0[\x83\x81\x10\x15a1\xACW\x81\x81\x01Q\x83\x82\x01R` \x01a1\x94V[\x83\x81\x11\x15a\x1CiWPP`\0\x91\x01RV[`\0\x81a1\xCCWa1\xCCa2\x0FV[P`\0\x19\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a1\xE8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a2\tWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a2fW`\0\x80\xFD[PV[\x80\x15\x15\x81\x14a2fW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xB0\x98)\x01\xF4\xD4\x10\xDD\xD5>\xEF\xB5\x8C\x82\x12\xBD\xF4\xFE\x92\xC7cW%\x0C\xE9B\xBDCo\xAF\x89\xD5dsolcC\0\x08\x07\x003";
    /// The deployed bytecode of the contract.
    pub static MYSTIKOV2AXELARERC20_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MystikoV2AxelarERC20<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoV2AxelarERC20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoV2AxelarERC20<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoV2AxelarERC20<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoV2AxelarERC20<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MystikoV2AxelarERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> MystikoV2AxelarERC20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                MYSTIKOV2AXELARERC20_ABI.clone(),
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
                MYSTIKOV2AXELARERC20_ABI.clone(),
                MYSTIKOV2AXELARERC20_BYTECODE.clone().into(),
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
        ///Calls the contract's `execute` (0x49160658) function
        pub fn execute(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            payload: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 22, 6, 88], (command_id, source_chain, source_address, payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeWithToken` (0x1a98b2e0) function
        pub fn execute_with_token(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            payload: ::ethers_core::types::Bytes,
            token_symbol: ::std::string::String,
            amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [26, 152, 178, 224],
                    (command_id, source_chain, source_address, payload, token_symbol, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gateway` (0x116191b6) function
        pub fn gateway(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([17, 97, 145, 182], ())
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
        ///Calls the contract's `setAxelarGasReceiver` (0xdf420737) function
        pub fn set_axelar_gas_receiver(
            &self,
            gas_receiver: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 66, 7, 55], gas_receiver)
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
        ///Gets the contract's `CallContractMessage` event
        pub fn call_contract_message_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, CallContractMessageFilter> {
            self.0.event()
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
        pub fn events(&self) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, MystikoV2AxelarERC20Events> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for MystikoV2AxelarERC20<M> {
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
    ///Custom Error type `NotApprovedByGateway` with signature `NotApprovedByGateway()` and selector `0x500c44b4`
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
    #[etherror(name = "NotApprovedByGateway", abi = "NotApprovedByGateway()")]
    pub struct NotApprovedByGateway;
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
    pub enum MystikoV2AxelarERC20Errors {
        AmountLessThanZero(AmountLessThanZero),
        AmountTooLarge(AmountTooLarge),
        AmountTooSmall(AmountTooSmall),
        BridgeFeeTooFew(BridgeFeeTooFew),
        CommitmentHashIncorrect(CommitmentHashIncorrect),
        DepositsDisabled(DepositsDisabled),
        ExecutorFeeTooFew(ExecutorFeeTooFew),
        FromChainIdNotMatched(FromChainIdNotMatched),
        FromProxyAddressNotMatched(FromProxyAddressNotMatched),
        HashKGreaterThanFieldSize(HashKGreaterThanFieldSize),
        Invalid(Invalid),
        MinAmountGreaterThanMaxAmount(MinAmountGreaterThanMaxAmount),
        NotApprovedByGateway(NotApprovedByGateway),
        NotChanged(NotChanged),
        OnlyOperator(OnlyOperator),
        RandomSGreaterThanFieldSize(RandomSGreaterThanFieldSize),
        RollupFeeToFew(RollupFeeToFew),
        SanctionedAddress(SanctionedAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoV2AxelarERC20Errors {
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
            if let Ok(decoded) = <CommitmentHashIncorrect as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CommitmentHashIncorrect(decoded));
            }
            if let Ok(decoded) = <DepositsDisabled as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositsDisabled(decoded));
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
            if let Ok(decoded) = <NotApprovedByGateway as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotApprovedByGateway(decoded));
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
    impl ::ethers_core::abi::AbiEncode for MystikoV2AxelarERC20Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AmountLessThanZero(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AmountTooLarge(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AmountTooSmall(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::BridgeFeeTooFew(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CommitmentHashIncorrect(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DepositsDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ExecutorFeeTooFew(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::FromChainIdNotMatched(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::FromProxyAddressNotMatched(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::HashKGreaterThanFieldSize(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Invalid(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::MinAmountGreaterThanMaxAmount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NotApprovedByGateway(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NotChanged(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnlyOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RandomSGreaterThanFieldSize(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RollupFeeToFew(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionedAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for MystikoV2AxelarERC20Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AmountLessThanZero as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <AmountTooLarge as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <AmountTooSmall as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <BridgeFeeTooFew as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <CommitmentHashIncorrect as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <DepositsDisabled as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <ExecutorFeeTooFew as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <FromChainIdNotMatched as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <FromProxyAddressNotMatched as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <HashKGreaterThanFieldSize as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <Invalid as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <MinAmountGreaterThanMaxAmount as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <NotApprovedByGateway as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <NotChanged as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OnlyOperator as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <RandomSGreaterThanFieldSize as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <RollupFeeToFew as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <SanctionedAddress as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2AxelarERC20Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AmountLessThanZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentHashIncorrect(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutorFeeTooFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::FromChainIdNotMatched(element) => ::core::fmt::Display::fmt(element, f),
                Self::FromProxyAddressNotMatched(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashKGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinAmountGreaterThanMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotApprovedByGateway(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotChanged(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RandomSGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupFeeToFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionedAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MystikoV2AxelarERC20Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AmountLessThanZero> for MystikoV2AxelarERC20Errors {
        fn from(value: AmountLessThanZero) -> Self {
            Self::AmountLessThanZero(value)
        }
    }
    impl ::core::convert::From<AmountTooLarge> for MystikoV2AxelarERC20Errors {
        fn from(value: AmountTooLarge) -> Self {
            Self::AmountTooLarge(value)
        }
    }
    impl ::core::convert::From<AmountTooSmall> for MystikoV2AxelarERC20Errors {
        fn from(value: AmountTooSmall) -> Self {
            Self::AmountTooSmall(value)
        }
    }
    impl ::core::convert::From<BridgeFeeTooFew> for MystikoV2AxelarERC20Errors {
        fn from(value: BridgeFeeTooFew) -> Self {
            Self::BridgeFeeTooFew(value)
        }
    }
    impl ::core::convert::From<CommitmentHashIncorrect> for MystikoV2AxelarERC20Errors {
        fn from(value: CommitmentHashIncorrect) -> Self {
            Self::CommitmentHashIncorrect(value)
        }
    }
    impl ::core::convert::From<DepositsDisabled> for MystikoV2AxelarERC20Errors {
        fn from(value: DepositsDisabled) -> Self {
            Self::DepositsDisabled(value)
        }
    }
    impl ::core::convert::From<ExecutorFeeTooFew> for MystikoV2AxelarERC20Errors {
        fn from(value: ExecutorFeeTooFew) -> Self {
            Self::ExecutorFeeTooFew(value)
        }
    }
    impl ::core::convert::From<FromChainIdNotMatched> for MystikoV2AxelarERC20Errors {
        fn from(value: FromChainIdNotMatched) -> Self {
            Self::FromChainIdNotMatched(value)
        }
    }
    impl ::core::convert::From<FromProxyAddressNotMatched> for MystikoV2AxelarERC20Errors {
        fn from(value: FromProxyAddressNotMatched) -> Self {
            Self::FromProxyAddressNotMatched(value)
        }
    }
    impl ::core::convert::From<HashKGreaterThanFieldSize> for MystikoV2AxelarERC20Errors {
        fn from(value: HashKGreaterThanFieldSize) -> Self {
            Self::HashKGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<Invalid> for MystikoV2AxelarERC20Errors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<MinAmountGreaterThanMaxAmount> for MystikoV2AxelarERC20Errors {
        fn from(value: MinAmountGreaterThanMaxAmount) -> Self {
            Self::MinAmountGreaterThanMaxAmount(value)
        }
    }
    impl ::core::convert::From<NotApprovedByGateway> for MystikoV2AxelarERC20Errors {
        fn from(value: NotApprovedByGateway) -> Self {
            Self::NotApprovedByGateway(value)
        }
    }
    impl ::core::convert::From<NotChanged> for MystikoV2AxelarERC20Errors {
        fn from(value: NotChanged) -> Self {
            Self::NotChanged(value)
        }
    }
    impl ::core::convert::From<OnlyOperator> for MystikoV2AxelarERC20Errors {
        fn from(value: OnlyOperator) -> Self {
            Self::OnlyOperator(value)
        }
    }
    impl ::core::convert::From<RandomSGreaterThanFieldSize> for MystikoV2AxelarERC20Errors {
        fn from(value: RandomSGreaterThanFieldSize) -> Self {
            Self::RandomSGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<RollupFeeToFew> for MystikoV2AxelarERC20Errors {
        fn from(value: RollupFeeToFew) -> Self {
            Self::RollupFeeToFew(value)
        }
    }
    impl ::core::convert::From<SanctionedAddress> for MystikoV2AxelarERC20Errors {
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
    #[ethevent(name = "CallContractMessage", abi = "CallContractMessage(string,string)")]
    pub struct CallContractMessageFilter {
        pub peer_chain_name: ::std::string::String,
        pub destination_address: ::std::string::String,
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
    pub enum MystikoV2AxelarERC20Events {
        CallContractMessageFilter(CallContractMessageFilter),
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
    impl ::ethers_contract::EthLogDecode for MystikoV2AxelarERC20Events {
        fn decode_log(log: &::ethers_core::abi::RawLog) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = CallContractMessageFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::CallContractMessageFilter(decoded));
            }
            if let Ok(decoded) = CommitmentCrossChainFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::CommitmentCrossChainFilter(decoded));
            }
            if let Ok(decoded) = DepositAmountLimitsFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::DepositAmountLimitsFilter(decoded));
            }
            if let Ok(decoded) = DepositsDisabledFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::DepositsDisabledFilter(decoded));
            }
            if let Ok(decoded) = MinBridgeFeeFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::MinBridgeFeeFilter(decoded));
            }
            if let Ok(decoded) = MinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::MinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = PeerMinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::PeerMinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = PeerMinRollupFeeFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::PeerMinRollupFeeFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(MystikoV2AxelarERC20Events::SanctionsListFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MystikoV2AxelarERC20Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallContractMessageFilter(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<CallContractMessageFilter> for MystikoV2AxelarERC20Events {
        fn from(value: CallContractMessageFilter) -> Self {
            Self::CallContractMessageFilter(value)
        }
    }
    impl ::core::convert::From<CommitmentCrossChainFilter> for MystikoV2AxelarERC20Events {
        fn from(value: CommitmentCrossChainFilter) -> Self {
            Self::CommitmentCrossChainFilter(value)
        }
    }
    impl ::core::convert::From<DepositAmountLimitsFilter> for MystikoV2AxelarERC20Events {
        fn from(value: DepositAmountLimitsFilter) -> Self {
            Self::DepositAmountLimitsFilter(value)
        }
    }
    impl ::core::convert::From<DepositsDisabledFilter> for MystikoV2AxelarERC20Events {
        fn from(value: DepositsDisabledFilter) -> Self {
            Self::DepositsDisabledFilter(value)
        }
    }
    impl ::core::convert::From<MinBridgeFeeFilter> for MystikoV2AxelarERC20Events {
        fn from(value: MinBridgeFeeFilter) -> Self {
            Self::MinBridgeFeeFilter(value)
        }
    }
    impl ::core::convert::From<MinExecutorFeeFilter> for MystikoV2AxelarERC20Events {
        fn from(value: MinExecutorFeeFilter) -> Self {
            Self::MinExecutorFeeFilter(value)
        }
    }
    impl ::core::convert::From<OperatorChangedFilter> for MystikoV2AxelarERC20Events {
        fn from(value: OperatorChangedFilter) -> Self {
            Self::OperatorChangedFilter(value)
        }
    }
    impl ::core::convert::From<PeerMinExecutorFeeFilter> for MystikoV2AxelarERC20Events {
        fn from(value: PeerMinExecutorFeeFilter) -> Self {
            Self::PeerMinExecutorFeeFilter(value)
        }
    }
    impl ::core::convert::From<PeerMinRollupFeeFilter> for MystikoV2AxelarERC20Events {
        fn from(value: PeerMinRollupFeeFilter) -> Self {
            Self::PeerMinRollupFeeFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckFilter> for MystikoV2AxelarERC20Events {
        fn from(value: SanctionsCheckFilter) -> Self {
            Self::SanctionsCheckFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsListFilter> for MystikoV2AxelarERC20Events {
        fn from(value: SanctionsListFilter) -> Self {
            Self::SanctionsListFilter(value)
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
    ///Container type for all input parameters for the `execute` function with signature `execute(bytes32,string,string,bytes)` and selector `0x49160658`
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
    #[ethcall(name = "execute", abi = "execute(bytes32,string,string,bytes)")]
    pub struct ExecuteCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub payload: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeWithToken` function with signature `executeWithToken(bytes32,string,string,bytes,string,uint256)` and selector `0x1a98b2e0`
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
        name = "executeWithToken",
        abi = "executeWithToken(bytes32,string,string,bytes,string,uint256)"
    )]
    pub struct ExecuteWithTokenCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub payload: ::ethers_core::types::Bytes,
        pub token_symbol: ::std::string::String,
        pub amount: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `gateway` function with signature `gateway()` and selector `0x116191b6`
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
    #[ethcall(name = "gateway", abi = "gateway()")]
    pub struct GatewayCall;
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
    ///Container type for all input parameters for the `setAxelarGasReceiver` function with signature `setAxelarGasReceiver(address)` and selector `0xdf420737`
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
    #[ethcall(name = "setAxelarGasReceiver", abi = "setAxelarGasReceiver(address)")]
    pub struct SetAxelarGasReceiverCall {
        pub gas_receiver: ::ethers_core::types::Address,
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
    pub enum MystikoV2AxelarERC20Calls {
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
        Execute(ExecuteCall),
        ExecuteWithToken(ExecuteWithTokenCall),
        Gateway(GatewayCall),
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
        SetAxelarGasReceiver(SetAxelarGasReceiverCall),
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
    impl ::ethers_core::abi::AbiDecode for MystikoV2AxelarERC20Calls {
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
            if let Ok(decoded) = <ExecuteCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ExecuteWithTokenCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteWithToken(decoded));
            }
            if let Ok(decoded) = <GatewayCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Gateway(decoded));
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
            if let Ok(decoded) = <SetAxelarGasReceiverCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAxelarGasReceiver(decoded));
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
    impl ::ethers_core::abi::AbiEncode for MystikoV2AxelarERC20Calls {
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
                Self::Execute(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ExecuteWithToken(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Gateway(element) => ::ethers_core::abi::AbiEncode::encode(element),
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
                Self::SetAxelarGasReceiver(element) => ::ethers_core::abi::AbiEncode::encode(element),
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
    impl ::core::fmt::Display for MystikoV2AxelarERC20Calls {
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
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gateway(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::SetAxelarGasReceiver(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AssetDecimalsCall> for MystikoV2AxelarERC20Calls {
        fn from(value: AssetDecimalsCall) -> Self {
            Self::AssetDecimals(value)
        }
    }
    impl ::core::convert::From<AssetNameCall> for MystikoV2AxelarERC20Calls {
        fn from(value: AssetNameCall) -> Self {
            Self::AssetName(value)
        }
    }
    impl ::core::convert::From<AssetSymbolCall> for MystikoV2AxelarERC20Calls {
        fn from(value: AssetSymbolCall) -> Self {
            Self::AssetSymbol(value)
        }
    }
    impl ::core::convert::From<AssetTypeCall> for MystikoV2AxelarERC20Calls {
        fn from(value: AssetTypeCall) -> Self {
            Self::AssetType(value)
        }
    }
    impl ::core::convert::From<BridgeProxyAddressCall> for MystikoV2AxelarERC20Calls {
        fn from(value: BridgeProxyAddressCall) -> Self {
            Self::BridgeProxyAddress(value)
        }
    }
    impl ::core::convert::From<BridgeTypeCall> for MystikoV2AxelarERC20Calls {
        fn from(value: BridgeTypeCall) -> Self {
            Self::BridgeType(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorCall> for MystikoV2AxelarERC20Calls {
        fn from(value: ChangeOperatorCall) -> Self {
            Self::ChangeOperator(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MystikoV2AxelarERC20Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DisableSanctionsCheckCall> for MystikoV2AxelarERC20Calls {
        fn from(value: DisableSanctionsCheckCall) -> Self {
            Self::DisableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<EnableSanctionsCheckCall> for MystikoV2AxelarERC20Calls {
        fn from(value: EnableSanctionsCheckCall) -> Self {
            Self::EnableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for MystikoV2AxelarERC20Calls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteWithTokenCall> for MystikoV2AxelarERC20Calls {
        fn from(value: ExecuteWithTokenCall) -> Self {
            Self::ExecuteWithToken(value)
        }
    }
    impl ::core::convert::From<GatewayCall> for MystikoV2AxelarERC20Calls {
        fn from(value: GatewayCall) -> Self {
            Self::Gateway(value)
        }
    }
    impl ::core::convert::From<GetAssociatedCommitmentPoolCall> for MystikoV2AxelarERC20Calls {
        fn from(value: GetAssociatedCommitmentPoolCall) -> Self {
            Self::GetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<GetMaxAmountCall> for MystikoV2AxelarERC20Calls {
        fn from(value: GetMaxAmountCall) -> Self {
            Self::GetMaxAmount(value)
        }
    }
    impl ::core::convert::From<GetMinAmountCall> for MystikoV2AxelarERC20Calls {
        fn from(value: GetMinAmountCall) -> Self {
            Self::GetMinAmount(value)
        }
    }
    impl ::core::convert::From<GetMinBridgeFeeCall> for MystikoV2AxelarERC20Calls {
        fn from(value: GetMinBridgeFeeCall) -> Self {
            Self::GetMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<GetMinExecutorFeeCall> for MystikoV2AxelarERC20Calls {
        fn from(value: GetMinExecutorFeeCall) -> Self {
            Self::GetMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<GetPeerMinExecutorFeeCall> for MystikoV2AxelarERC20Calls {
        fn from(value: GetPeerMinExecutorFeeCall) -> Self {
            Self::GetPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<GetPeerMinRollupFeeCall> for MystikoV2AxelarERC20Calls {
        fn from(value: GetPeerMinRollupFeeCall) -> Self {
            Self::GetPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<IsDepositsDisabledCall> for MystikoV2AxelarERC20Calls {
        fn from(value: IsDepositsDisabledCall) -> Self {
            Self::IsDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<PeerChainIdCall> for MystikoV2AxelarERC20Calls {
        fn from(value: PeerChainIdCall) -> Self {
            Self::PeerChainId(value)
        }
    }
    impl ::core::convert::From<PeerChainNameCall> for MystikoV2AxelarERC20Calls {
        fn from(value: PeerChainNameCall) -> Self {
            Self::PeerChainName(value)
        }
    }
    impl ::core::convert::From<PeerContractCall> for MystikoV2AxelarERC20Calls {
        fn from(value: PeerContractCall) -> Self {
            Self::PeerContract(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SanctionsCheckCall) -> Self {
            Self::SanctionsCheck(value)
        }
    }
    impl ::core::convert::From<SanctionsListCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SanctionsListCall) -> Self {
            Self::SanctionsList(value)
        }
    }
    impl ::core::convert::From<SetAssociatedCommitmentPoolCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SetAssociatedCommitmentPoolCall) -> Self {
            Self::SetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<SetAxelarGasReceiverCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SetAxelarGasReceiverCall) -> Self {
            Self::SetAxelarGasReceiver(value)
        }
    }
    impl ::core::convert::From<SetBridgeProxyAddressCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SetBridgeProxyAddressCall) -> Self {
            Self::SetBridgeProxyAddress(value)
        }
    }
    impl ::core::convert::From<SetDepositsDisabledCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SetDepositsDisabledCall) -> Self {
            Self::SetDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<SetMinBridgeFeeCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SetMinBridgeFeeCall) -> Self {
            Self::SetMinBridgeFee(value)
        }
    }
    impl ::core::convert::From<SetMinExecutorFeeCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SetMinExecutorFeeCall) -> Self {
            Self::SetMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<SetPeerContractCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SetPeerContractCall) -> Self {
            Self::SetPeerContract(value)
        }
    }
    impl ::core::convert::From<SetPeerMinExecutorFeeCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SetPeerMinExecutorFeeCall) -> Self {
            Self::SetPeerMinExecutorFee(value)
        }
    }
    impl ::core::convert::From<SetPeerMinRollupFeeCall> for MystikoV2AxelarERC20Calls {
        fn from(value: SetPeerMinRollupFeeCall) -> Self {
            Self::SetPeerMinRollupFee(value)
        }
    }
    impl ::core::convert::From<UpdateDepositAmountLimitsCall> for MystikoV2AxelarERC20Calls {
        fn from(value: UpdateDepositAmountLimitsCall) -> Self {
            Self::UpdateDepositAmountLimits(value)
        }
    }
    impl ::core::convert::From<UpdateSanctionsListAddressCall> for MystikoV2AxelarERC20Calls {
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
    ///Container type for all return fields from the `gateway` function with signature `gateway()` and selector `0x116191b6`
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
    pub struct GatewayReturn(pub ::ethers_core::types::Address);
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
