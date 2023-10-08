pub use commitment_pool_main::*;
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
pub mod commitment_pool_main {
    const _: () = {
        ::core::include_bytes!("../json/CommitmentPoolMain.json",);
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers_core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_treeHeight"),
                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint8"),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_pathIndices"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_pathIndices"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_fullPath"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_rollupSize"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                        ],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addEnqueueWhitelist"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addEnqueueWhitelist",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_actor"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addRollupWhitelist"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addRollupWhitelist"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_roller"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("auditorCount"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("auditorCount"),
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
                    ::std::borrow::ToOwned::to_owned("disableRollupVerifier"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("disableRollupVerifier",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_rollupSize"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("disableTransactVerifier"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("disableTransactVerifier",),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numInputs"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numOutputs"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enableRollupVerifier"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enableRollupVerifier",),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_rollupSize"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_rollupVerifier"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "contract IVerifier"
                                ),),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("enableTransactVerifier"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enableTransactVerifier",),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numInputs"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numOutputs"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_transactVerifier"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "contract IVerifier"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enqueue"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enqueue"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_request"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers_core::abi::ethabi::ParamType::Bytes,
                                ],),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "struct ICommitmentPool.CommitmentRequest",
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_executor"),
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
                    ::std::borrow::ToOwned::to_owned("getAllAuditorPublicKeys"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllAuditorPublicKeys",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                            ),),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256[]"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAuditorPublicKey"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAuditorPublicKey",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_index"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("getCommitmentCount"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCommitmentCount"),
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
                    ::std::borrow::ToOwned::to_owned("getCommitmentIncludedCount"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCommitmentIncludedCount",),
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
                    ::std::borrow::ToOwned::to_owned("getCommitmentQueuedCount"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCommitmentQueuedCount",),
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
                    ::std::borrow::ToOwned::to_owned("getMinRollupFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMinRollupFee"),
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
                    ::std::borrow::ToOwned::to_owned("getNullifierCount"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNullifierCount"),
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
                    ::std::borrow::ToOwned::to_owned("getQueuedCommitments"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getQueuedCommitments",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                            ),),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256[]"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTreeCapacity"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTreeCapacity"),
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
                    ::std::borrow::ToOwned::to_owned("isHistoricCommitment"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isHistoricCommitment",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_commitment"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("isKnownRoot"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isKnownRoot"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("root"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("isRollupWhitelistDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isRollupWhitelistDisabled",),
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
                    ::std::borrow::ToOwned::to_owned("isSpentSerialNumber"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isSpentSerialNumber",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_serialNumber"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("isVerifierUpdateDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isVerifierUpdateDisabled",),
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
                    ::std::borrow::ToOwned::to_owned("removeEnqueueWhitelist"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeEnqueueWhitelist",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_actor"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeRollupWhitelist"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeRollupWhitelist",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_roller"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rollup"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rollup"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_request"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers_core::abi::ethabi::ParamType::FixedArray(
                                            ::std::boxed::Box::new(::ethers_core::abi::ethabi::ParamType::Uint(
                                                256usize
                                            ),),
                                            2usize,
                                        ),
                                        ::ethers_core::abi::ethabi::ParamType::FixedArray(
                                            ::std::boxed::Box::new(::ethers_core::abi::ethabi::ParamType::Uint(
                                                256usize
                                            ),),
                                            2usize,
                                        ),
                                    ],),
                                    ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ],),
                                ::ethers_core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct ICommitmentPool.RollupRequest",
                            ),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("setMinRollupFee"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMinRollupFee"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_minRollupFee"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setRollupWhitelistDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setRollupWhitelistDisabled",),
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
                    ::std::borrow::ToOwned::to_owned("setVerifierUpdateDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setVerifierUpdateDisabled",),
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
                    ::std::borrow::ToOwned::to_owned("transact"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transact"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_request"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                        ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(::ethers_core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),),
                                                2usize,
                                            ),
                                            ::ethers_core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(::ethers_core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),),
                                                2usize,
                                            ),
                                        ],),
                                        ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                    ],),
                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers_core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                        ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ),),
                                    ::ethers_core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                        ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ),),
                                    ::ethers_core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers_core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                        ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ),),
                                    ::ethers_core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                        ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ),),
                                    ::ethers_core::abi::ethabi::ParamType::Address,
                                    ::ethers_core::abi::ethabi::ParamType::Address,
                                    ::ethers_core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                        ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    ),),
                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers_core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                        ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ),),
                                ],),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "struct ICommitmentPool.TransactRequest",
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_signature"),
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
                    ::std::borrow::ToOwned::to_owned("updateAuditorPublicKey"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateAuditorPublicKey",),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_index"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_publicKey"),
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
                    ::std::borrow::ToOwned::to_owned("AuditorPublicKey"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AuditorPublicKey"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("publicKey"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentIncluded"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CommitmentIncluded"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("commitment"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentQueued"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CommitmentQueued"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("commitment"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("rollupFee"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("leafIndex"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("encryptedNote"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentSpent"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CommitmentSpent"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("rootHash"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("serialNumber"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EncryptedAuditorNote"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EncryptedAuditorNote",),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("auditorPublicKey"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers_core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("encryptedAuditorNote",),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EncryptedAuditorNotes"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EncryptedAuditorNotes",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("notes"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                ::ethers_core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                            ),),
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
                    ::std::borrow::ToOwned::to_owned("RollupWhitelistDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RollupWhitelistDisabled",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bool,
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
                    ::std::borrow::ToOwned::to_owned("VerifierUpdateDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("VerifierUpdateDisabled",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AuditorIndexError"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AuditorIndexError"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AuditorNotesLengthError"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AuditorNotesLengthError",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AuditorPublicKeyNotChanged"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AuditorPublicKeyNotChanged",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CommitmentHasBeenSubmitted"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CommitmentHasBeenSubmitted",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Duplicated"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Duplicated"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("param"),
                            kind: ::ethers_core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IndexOutOfBound"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("IndexOutOfBound"),
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
                    ::std::borrow::ToOwned::to_owned("NewRootIsDuplicated"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NewRootIsDuplicated",),
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
                    ::std::borrow::ToOwned::to_owned("NoteHasBeenSpent"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoteHasBeenSpent"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NumInputsGreaterThanZero"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NumInputsGreaterThanZero",),
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
                    ::std::borrow::ToOwned::to_owned("OnlyWhitelistedRoller"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyWhitelistedRoller",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyWhitelistedSender"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyWhitelistedSender",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutputNotesLessThanThree"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OutputNotesLessThanThree",),
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
                    ::std::borrow::ToOwned::to_owned("RollupSizeNotPowerOfTwo"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RollupSizeNotPowerOfTwo",),
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
                (
                    ::std::borrow::ToOwned::to_owned("TreeHeightLessThanZero"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TreeHeightLessThanZero",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TreeHeightOutOfBounds"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TreeHeightOutOfBounds",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TreeIsFull"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TreeIsFull"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VerifierUpdatesHasBeenDisabled"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("VerifierUpdatesHasBeenDisabled",),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COMMITMENTPOOLMAIN_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R`\x01\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16t\x01@\xC5y#\x92K\\\\TU\xC4\x8D\x931q9\xAD\xDA\xC8\xFB\x17\x90U`\0`\x07\x81\x90U`\x08\x81\x90U`\tU4\x80\x15b\0\0GW`\0\x80\xFD[P`@Qb\0Eu8\x03\x80b\0Eu\x839\x81\x01`@\x81\x90Rb\0\0j\x91b\0\x07\xA9V[`\x01`\0U\x80`\xFF\x81\x16b\0\0\x92W`@Qc,O)\xB1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U`\x01`\xFF\x82\x16\x1B`\x80Rb\0\0\xB9\x81b\0\0\xE0V[`\x0B\x81\x90U`\0\x90\x81R`\n` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UPb\0\x07\xD5\x90PV[`\0`\xFF\x82\x16b\0\x01\x12WP\x7F\t\xF6XEwu\x07O\xF4\xC8B\x03*^\xC2\xF1\x13L2xL\xCAY\xD5\x94\xCA\xAC\x8CP;y#\x91\x90PV[\x81`\xFF\x16`\x01\x14\x15b\0\x01FWP\x7F\x1AwV\x9By\xCB|.\xAF\x93h\xDE\x9E;\x1E\xFC\n`ea\xE5\xAB)\x9C#74\x0F<\xDDWj\x91\x90PV[\x81`\xFF\x16`\x02\x14\x15b\0\x01zWP\x7F\x11\x1B\xD0\0+\x1E;\x8F\x19x\xC92\xB5\xCC\xB2\xFA\xB8\x7F\xDB\n\xCA\xB2\xDD\xB2f\xF9\xA3F\xDC\xB1\x1E\xE1\x91\x90PV[\x81`\xFF\x16`\x03\x14\x15b\0\x01\xAEWP\x7F\x04\x08h\xEA*6>\x05\xD7\xAED(\x0Fi\xB4\x9EB\x9A\xB0\x96\xCA(D\xEF\xE9\x1F\x8Dk=\xADW\xDD\x91\x90PV[\x81`\xFF\x16`\x04\x14\x15b\0\x01\xE1WP~\xF8\xF6\x05\xC9,\x85\x02\xC8\xFE\x83\xBE\x1B\x83\xB2N?g1*8\x8F0\xCB\xAD\xB5\xDE\xE8\x97A7\xC3\x91\x90PV[\x81`\xFF\x16`\x05\x14\x15b\0\x02\x15WP\x7F\x1A\xDC\x04<\x99\xAD\xBC\x0C\x86\xA6\n6\xDB\x0Ff\x1E-\xD9o6\xED\xE3\"\xF9T8m\x895\xA0\xC5\xD9\x91\x90PV[\x81`\xFF\x16`\x06\x14\x15b\0\x02IWP\x7F*\x1F\xED\xFAq\xDAr:\xC3\xE9\xB3\\\xEFu/\xA1\xB6G\xB2\xB77\xA2>\x91$\x1C\xB7\xBD\xC4\x19\xE3\xF4\x91\x90PV[\x81`\xFF\x16`\x07\x14\x15b\0\x02}WP\x7F\x17\xFE\x19tT<LK\"\x8E\x12\x92\xF7\xE3 \r1C_\xC9\x10\xEEZ\x8C\\\xAF\xD3)\xCCK%k\x91\x90PV[\x81`\xFF\x16`\x08\x14\x15b\0\x02\xB1WP\x7F\x0E\x84\xA5\x86\xEBc\xA0\xEE\xC0\xF1\xFEW\x85\x02$A\xF0\xE2\x9EJ\xE8Y\xC7\xCE\x1F_\xC8\x8AB\xAD.k\x91\x90PV[\x81`\xFF\x16`\t\x14\x15b\0\x02\xE5WP\x7F\x19=\xEB\x90\x1Fn\xEB\x03.\x02\xE0\x93(\r\xB1~7=O\xF5+\xAF\xDA\x92\x15\xB4k\xB9\xB0\xA8o>\x91\x90PV[\x81`\xFF\x16`\n\x14\x15b\0\x03\x19WP\x7F\x1D\x10\xCA{\x98V\x97\xCBQ\x95e\xA5\0l?D\xB0 \xB2\xED\xAB\x9Bt\"\xED\x15\xDC4S/\x94\x06\x91\x90PV[\x81`\xFF\x16`\x0B\x14\x15b\0\x03MWP\x7F\x1D\xC2\0v5UFzNX>\0\xBA\xDF\xDC\x1F\xB4\xD3\xD3\xF8\xF1\xCC\x81\xF3\x1F\xD2\xF8\xB3\x87w`\x81\x91\x90PV[\x81`\xFF\x16`\x0C\x14\x15b\0\x03\x81WP\x7F =\xD1\x1F\xDB\xA0\xED\x13\xB2\x0C\xA2\xD6\x95/?\xEBta\x83j\x03Q.\x0C\xCC\xCE\x84g\xB3 \xB2\xF6\x91\x90PV[\x81`\xFF\x16`\r\x14\x15b\0\x03\xB5WP\x7F\x05\x91\xAFd\xE6J>i\xCA\xF2>\xEE+\xDE\xA9\x08\x854:I\xF5G\xEE\x94d\xE9]\x8Dbg\xE4\xF7\x91\x90PV[\x81`\xFF\x16`\x0E\x14\x15b\0\x03\xE9WP\x7F*\xF5r\xF1\x07z2\xF4m\xC8\xE3\x07\xD4<\x0F\xA6u;@\x0B!\x072Yv\xB8\xDFs\x80T_\xF6\x91\x90PV[\x81`\xFF\x16`\x0F\x14\x15b\0\x04\x1DWP\x7F\x042'\xAEKp\xB1\xAA\xCD\x04\xE3^j\xAE\xD7\xB5m\x91\"\x0C1\xE9z\xAFR\xE1*jV\x98NR\x91\x90PV[\x81`\xFF\x16`\x10\x14\x15b\0\x04QWP\x7F)v\xF1\xF6\xA9\x1D\x83\xD4\xC5(\xDA\xD6\x9E\xCEm=\x91\x93K\x0EVW\xE9\x15\xB3`\xC8\xC4\xC2\xFBT\xE6\x91\x90PV[\x81`\xFF\x16`\x11\x14\x15b\0\x04\x84WP~\xE5\xC2Q\xC9\xE0\x93e\x8B\xE0\xCD\x1B\r\xF5[op\xF3\xD0\x91F\xC1\xC8\xB4!*M\xDC\xDEp\r\xBC\x91\x90PV[\x81`\xFF\x16`\x12\x14\x15b\0\x04\xB8WP\x7F\x02g\xCB\xBC\x1B\xC2\xF1\xC3\xE3\x07=.\xE6\r\xF8\xCC\x0B\xFE\xF3\x9F\xE3\xCE\xE75\xC9\xAD\\\x8A\xD3\0d\xD2\x91\x90PV[\x81`\xFF\x16`\x13\x14\x15b\0\x04\xECWP\x7F/5`W\xBCV\xF6}\xBF\x15\x9A\x0E\x895\x02*\xCD^\x98-\xCE\x9F@q\xAD\xC4>MW\xCE'\xE6\x91\x90PV[\x81`\xFF\x16`\x14\x14\x15b\0\x05 WP\x7F'=\xB6\x8FR\xF1*\x9D\x80\"\xAER@P\x06NB\xD4\xD1f\x1C\x9B\xCC\xE9X\xAC\xF8\x9B^\x8B\x12{\x91\x90PV[\x81`\xFF\x16`\x15\x14\x15b\0\x05TWP\x7F\x04\x96\xA1\x8A\xD4\xCC\xA8\x1B|\x98\xCE\xB1\x97C\x9A\xD9%\xE0\xF7\xF6-i\xDF\xA4,\xF9WK\xE7\x7F\xE3\x0F\x91\x90PV[\x81`\xFF\x16`\x16\x14\x15b\0\x05\x88WP\x7F$\xF8\x9A?\x94=B\x1B/:UKeE\x9FB\xB8 \xAC\t\xD6\xFD\x9Di=\xF5\xF8\xBAs*\xB5\x96\x91\x90PV[\x81`\xFF\x16`\x17\x14\x15b\0\x05\xBCWP\x7F\x1BU\xBF\xD7Q\xC6\x80}\xF3hv\xCD\xCEh\x03J\xB42\x10\xBE+\xC8\xAF\xA8\x04<\x7FB\x86\x04\xE7\xA7\x91\x90PV[\x81`\xFF\x16`\x18\x14\x15b\0\x05\xF0WP\x7F\x16\xD6YZ9\x8C\xF2\x0F$\x89\xB9\x0E\x95\x81f\xF1\xE1\x9CS|\x0CF\xE9\xB8\xEATb9\x1E\xE8\xF1C\x91\x90PV[\x81`\xFF\x16`\x19\x14\x15b\0\x06$WP\x7F\x0F\x01D~\xF8\xF6!Y$t\xB6x\xED/\xC4\x04\xEB\xAF\"\xA6\xFC\xE1Sd\xBBAR\xA8\x8C\x116\x13\x91\x90PV[\x81`\xFF\x16`\x1A\x14\x15b\0\x06XWP\x7F\x02l-\xFF\xEEH\xBA\xCB\xC9\xD2\x1C\xF9\n\xA7\xC6\xE5%\xAB\x01\xDBif\xA9\xE7\xE5;=?M\x1FZM\x91\x90PV[\x81`\xFF\x16`\x1B\x14\x15b\0\x06\x8CWP\x7F#O\xE9\x072yWE\xB2\xC5\x04\xC7\x91$*+\xB1\x93\xBA\xA1\xCB\xEA\xB5}\xB92Kk\xB9\x13H\x17\x91\x90PV[\x81`\xFF\x16`\x1C\x14\x15b\0\x06\xC0WP\x7F#\xA8\xE0\xA7\xE6\t\x81\xC5.\xBBI\x8C&\r[\xEFM|e\x14]\x17\x12\x89\x96\xA7|3\xA3&*~\x91\x90PV[\x81`\xFF\x16`\x1D\x14\x15b\0\x06\xF4WP\x7F$\xEEi\xD2VR\x10\xC7\x02\x7F\xF6\xFC&W\xED\x02\x92x\xBCy\xF4\x10w\xFE2\x81\xEA]]\x8E\x80\xF9\x91\x90PV[\x81`\xFF\x16`\x1E\x14\x15b\0\x07(WP\x7F\x1D\xE4\x02\xFA2F;\xB2\x91{s:\xEE\xF0\x19zI\xCA\xCA\xCD\x1F\xE8`\xC3\xAC\xC8\xCD;e\xA3\nh\x91\x90PV[\x81`\xFF\x16`\x1F\x14\x15b\0\x07\\WP\x7F\x1D0\x15\xA0\xF6\xA7\xB3\xF7T\x17\x1D\x05@b\x81\x07\xE5\x0E%\xDE\xBC\xEB\x16\xE0\xE3:\xE4 U\x01\x89m\x91\x90PV[\x81`\xFF\x16` \x14\x15b\0\x07\x90WP\x7F&J&\x0594&G%h\x19\x04k\xAE\x05\xBE\xD9\x03\xA8\xD1\x9B<\x90C\x9Dg1}]\x88\x13\"\x91\x90PV[`@Qc\x97\x80\xF4)`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15b\0\x07\xBCW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14b\0\x07\xCEW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa=vb\0\x07\xFF`\09`\0\x81\x81a\x03\xFF\x01R\x81\x81a\x12\x96\x01Ra\x1B\xB4\x01Ra=v`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x89W`\x005`\xE0\x1C\x80c|\xBF\x0F\xF6\x11a\x01SW\x80c\xB1\xC3\x94\"\x11a\0\xCBW\x80c\xE5\0\xF5\x04\x11a\0\x7FW\x80c\xF2\xDA\x1DA\x11a\0dW\x80c\xF2\xDA\x1DA\x14a\x07\tW\x80c\xF8\xF0S\x88\x14a\x07)W\x80c\xFF\xA8\x9B\x88\x14a\x07IW`\0\x80\xFD[\x80c\xE5\0\xF5\x04\x14a\x06\xBCW\x80c\xECW\x1Cj\x14a\x06\xD1W`\0\x80\xFD[\x80c\xC2Y\xE2\xE6\x11a\0\xB0W\x80c\xC2Y\xE2\xE6\x14a\x06gW\x80c\xDDu|4\x14a\x06\x87W\x80c\xDE\xEF\xF7\xCD\x14a\x06\x9CW`\0\x80\xFD[\x80c\xB1\xC3\x94\"\x14a\x06&W\x80c\xB3\xB7V1\x14a\x06GW`\0\x80\xFD[\x80c\x9B\no\xEA\x11a\x01\"W\x80c\xA6#*\x93\x11a\x01\x07W\x80c\xA6#*\x93\x14a\x05\xC1W\x80c\xA9\xB1\xD2\x96\x14a\x05\xF1W\x80c\xB0\x88\x92\xD0\x14a\x06\x11W`\0\x80\xFD[\x80c\x9B\no\xEA\x14a\x05\x81W\x80c\x9C\xC6\xB3T\x14a\x05\xA1W`\0\x80\xFD[\x80c|\xBF\x0F\xF6\x14a\x05\x0CW\x80c\x7F\xA4\xB0\x9C\x14a\x05,W\x80c\x86j\xC6X\x14a\x05LW\x80c\x87x\r\xF9\x14a\x05aW`\0\x80\xFD[\x80c?\xE34z\x11a\x02\x01W\x80cW\x06\0\x16\x11a\x01\xB5W\x80cr\x08)q\x11a\x01\x9AW\x80cr\x08)q\x14a\x04\xB7W\x80cx\xD6\x0C\xD7\x14a\x04\xD7W\x80czU7D\x14a\x04\xF7W`\0\x80\xFD[\x80cW\x06\0\x16\x14a\x04eW\x80cc\xBC}2\x14a\x04\x95W`\0\x80\xFD[\x80cN\xB0i\xF7\x11a\x01\xE6W\x80cN\xB0i\xF7\x14a\x04#W\x80cU]u\xF0\x14a\x04;W\x80cV\x88\x88\x1F\x14a\x04PW`\0\x80\xFD[\x80c?\xE34z\x14a\x03\xD4W\x80cHN\xB6R\x14a\x03\xF0W`\0\x80\xFD[\x80c\x0C\x88g\xE6\x11a\x02XW\x80c\x14\xA7s}\x11a\x02=W\x80c\x14\xA7s}\x14a\x03TW\x80c0\xF4\x9C\xAC\x14a\x03tW\x80c;\xB8\xD1\xB4\x14a\x03\x94W`\0\x80\xFD[\x80c\x0C\x88g\xE6\x14a\x03\x0CW\x80c\x11_WL\x14a\x03,W`\0\x80\xFD[\x80c\x01\xDB\xF1\x9F\x14a\x02\x95W\x80c\x02\xD4\x98\xF1\x14a\x02\xACW\x80c\x03\xDB\x98t\x14a\x02\xCCW\x80c\x069L\x9B\x14a\x02\xECW`\0\x80\xFD[6a\x02\x90W\0[`\0\x80\xFD[4\x80\x15a\x02\xA1W`\0\x80\xFD[Pa\x02\xAAa\x07fV[\0[4\x80\x15a\x02\xB8W`\0\x80\xFD[Pa\x02\xAAa\x02\xC76`\x04a3\xC4V[a\x07\xEAV[4\x80\x15a\x02\xD8W`\0\x80\xFD[Pa\x02\xAAa\x02\xE76`\x04a3\xC4V[a\x089V[4\x80\x15a\x02\xF8W`\0\x80\xFD[Pa\x02\xAAa\x03\x076`\x04a3\xC4V[a\x08\x85V[4\x80\x15a\x03\x18W`\0\x80\xFD[Pa\x02\xAAa\x03'6`\x04a7\x18V[a\t6V[4\x80\x15a\x038W`\0\x80\xFD[Pa\x03A`\x05\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03`W`\0\x80\xFD[Pa\x02\xAAa\x03o6`\x04a4\xC9V[a\n\x05V[4\x80\x15a\x03\x80W`\0\x80\xFD[Pa\x02\xAAa\x03\x8F6`\x04a3\xC4V[a\x0FhV[4\x80\x15a\x03\xA0W`\0\x80\xFD[Pa\x03\xC4a\x03\xAF6`\x04a6\xFFV[`\0\x90\x81R`\x05` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x03KV[4\x80\x15a\x03\xE0W`\0\x80\xFD[P`\x01`@Qa\x03K\x91\x90a9\xD2V[4\x80\x15a\x03\xFCW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03AV[4\x80\x15a\x04/W`\0\x80\xFD[P`\x10T`\xFF\x16a\x03\xC4V[4\x80\x15a\x04GW`\0\x80\xFD[P`\x07Ta\x03AV[4\x80\x15a\x04\\W`\0\x80\xFD[Pa\x03Aa\x0F\xF5V[4\x80\x15a\x04qW`\0\x80\xFD[Pa\x03\xC4a\x04\x806`\x04a6\xFFV[`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x90V[4\x80\x15a\x04\xA1W`\0\x80\xFD[Pa\x04\xAAa\x10\x0CV[`@Qa\x03K\x91\x90a9\xBFV[4\x80\x15a\x04\xC3W`\0\x80\xFD[Pa\x02\xAAa\x04\xD26`\x04a5=V[a\x10\x89V[4\x80\x15a\x04\xE3W`\0\x80\xFD[Pa\x02\xAAa\x04\xF26`\x04a4\"V[a\x1B[V[4\x80\x15a\x05\x03W`\0\x80\xFD[P`\tTa\x03AV[4\x80\x15a\x05\x18W`\0\x80\xFD[Pa\x02\xAAa\x05'6`\x04a6\xFFV[a\x1C\x8AV[4\x80\x15a\x058W`\0\x80\xFD[Pa\x02\xAAa\x05G6`\x04a7\xD9V[a\x1D\x08V[4\x80\x15a\x05XW`\0\x80\xFD[Pa\x04\xAAa\x1D\xEBV[4\x80\x15a\x05mW`\0\x80\xFD[Pa\x03Aa\x05|6`\x04a6\xFFV[a\x1E\x98V[4\x80\x15a\x05\x8DW`\0\x80\xFD[Pa\x02\xAAa\x05\x9C6`\x04a7]V[a\x1E\xD6V[4\x80\x15a\x05\xADW`\0\x80\xFD[Pa\x02\xAAa\x05\xBC6`\x04a3\xC4V[a\x1F\xCEV[4\x80\x15a\x05\xCDW`\0\x80\xFD[Pa\x03\xC4a\x05\xDC6`\x04a6\xFFV[`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16\x90V[4\x80\x15a\x05\xFDW`\0\x80\xFD[Pa\x02\xAAa\x06\x0C6`\x04a3\xC4V[a \x1AV[4\x80\x15a\x06\x1DW`\0\x80\xFD[P`\rTa\x03AV[4\x80\x15a\x062W`\0\x80\xFD[P`\x01Ta\x03\xC4\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x06SW`\0\x80\xFD[Pa\x02\xAAa\x06b6`\x04a3\xE8V[a iV[4\x80\x15a\x06sW`\0\x80\xFD[Pa\x02\xAAa\x06\x826`\x04a7\xAFV[a \xD5V[4\x80\x15a\x06\x93W`\0\x80\xFD[Pa\x02\xAAa!yV[4\x80\x15a\x06\xA8W`\0\x80\xFD[Pa\x02\xAAa\x06\xB76`\x04a7xV[a!\xF1V[4\x80\x15a\x06\xC8W`\0\x80\xFD[P`\x08Ta\x03AV[4\x80\x15a\x06\xDDW`\0\x80\xFD[P`\x01Ta\x06\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03KV[4\x80\x15a\x07\x15W`\0\x80\xFD[Pa\x03Aa\x07$6`\x04a7:V[a#1V[4\x80\x15a\x075W`\0\x80\xFD[Pa\x02\xAAa\x07D6`\x04a3\xE8V[a#\xBAV[4\x80\x15a\x07UW`\0\x80\xFD[P`\x10Ta\x01\0\x90\x04`\xFF\x16a\x03\xC4V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x91W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x90\x81\x17\x91\x82\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x92a\x07\xE0\x92\x90\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\x15W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0E` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08dW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0F` R`@\x90 \x80T`\xFF\x19\x16\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xB0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x15a\x08\xDFW`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FG!\x12\x9E\x0Egn\xD6\xA9)\t\xBB$\xE8S\xCC\xDDc\xADr(\x0C\xC2\xE9t\xE3\x8EH\x0E\x0EnT\x90`\0\x90\xA2PV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\taW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x82\x10a\t\x82W`@Qc1\x8CCE`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x11\x83`\x05\x81\x10a\t\x96Wa\t\x96a<\xF1V[\x01T\x14\x15a\t\xB6W`@QbJpg`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x11\x83`\x05\x81\x10a\t\xCAWa\t\xCAa<\xF1V[\x01U`@Q\x81\x81R\x82\x90\x7F\x94\xAD\xCA\x83\xCEAD|\xFB\x8E\x07\xD1\xC9\xC4Sj\x1B\x8A\xFB\xFFl|\x83\xC2\xD9\x18!J\xDCh\x10/\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[`\x10Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\n-WP3`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16\x15[\x15a\nKW`@Qc!\xCE\x01\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80\x82\x01Q`\0\x90\x81R`\n` R T`\xFF\x16\x15a\n~W`@Qc\xE2\xE1!\x03`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x11\x80a\n\xBAWP` \x81\x81\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x03\x90\x91R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15[\x15a\n\xFAW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x08T` \x82\x01Qa\x0B\x12\x90c\xFF\xFF\xFF\xFF\x16\x82a<\xA3V[\x15a\x0BMW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01a\n\xF1V[`\0a\x0B]\x82\x84` \x01Qa#1V[\x90P`\0\x83` \x01Qc\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x84Wa\x0B\x84a=\x07V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xADW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x85` \x01Qc\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x0C\xABW`\0a\x0B\xD2\x82\x87a;qV[`\0\x81\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91R\x91\x92P\x90a\x0C\x1FW`@Qc\xD3H/{`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x01Q\x85\x84\x81Q\x81\x10a\x0C6Wa\x0C6a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x84a\x0CR\x91\x90a;qV[`\0\x83\x81R`\x06` R`@\x80\x82 \x82\x81U`\x01\x01\x82\x90U\x83Q\x90Q\x92\x96P\x91\x7F\xFEk\t{F\xA7\x8E\x08Pj1C\xB63|%\x05\xBAw\xDFv\xFE\x05\xC3f:\x98s\x95\xD64\x13\x91\x90\xA2PP\x80\x80a\x0C\xA3\x90a<dV[\x91PPa\x0B\xB4V[P\x84` \x01Qc\xFF\xFF\xFF\xFF\x16`\x07`\0\x82\x82Ta\x0C\xC8\x91\x90a;\xFCV[\x92PP\x81\x90UP`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x83`@Q` \x01a\r\x03\x91\x90a9&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\r&\x91\x90a<\xA3V[\x90P\x80\x86``\x01Q\x14a\r|W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01R\x7FleafHash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R`\0\x91` \x82\x01`\x80\x806\x837\x01\x90PP\x90P`\x0BT\x81`\0\x81Q\x81\x10a\r\xB5Wa\r\xB5a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x86`@\x01Q\x81`\x01\x81Q\x81\x10a\r\xD9Wa\r\xD9a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x81\x81`\x02\x81Q\x81\x10a\r\xF9Wa\r\xF9a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x84\x81`\x03\x81Q\x81\x10a\x0E\x19Wa\x0E\x19a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R\x87\x81\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x03\x90\x91R`@\x80\x82 T\x89Q\x91Qc\xC9AvG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC9AvG\x91a\x0Eq\x91\x90\x86\x90`\x04\x01a9\xFAV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x8BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xC3\x91\x90a4\x05V[\x90P\x80a\x0F\x13W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01R\x7Fproof\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[\x87` \x01Qc\xFF\xFF\xFF\xFF\x16`\x08`\0\x82\x82Ta\x0F/\x91\x90a;qV[\x90\x91UPP`@\x80\x89\x01Q`\x0B\x81\x90U`\0\x90\x81R`\n` R \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0F^\x84a$.V[PPPPPPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\x93W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xBE\x18\x08\x91~\t\x81O\xCA}\x80(\x9A\x97`\xA4\x06_\x10\xD6$\xAC\x1A\xEE\x0B\x9F\x90<O]\x92\xB1\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\0`\x07T`\x08Ta\x10\x07\x91\x90a;qV[\x90P\x90V[`@\x80Q`\x05\x80\x82R`\xC0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01`\xA0\x806\x837\x01\x90PP\x90P`\0[`\x05\x81\x10\x15a\x10\x83W`\x11\x81`\x05\x81\x10a\x10RWa\x10Ra<\xF1V[\x01T\x82\x82\x81Q\x81\x10a\x10fWa\x10fa<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x10{\x81a<dV[\x91PPa\x106V[P\x91\x90PV[`\x02`\0T\x14\x15a\x10\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n\xF1V[`\x02`\0\x90\x81U`@\x83\x01QQa\x10\xF2\x90a$\xC6V[\x90P`\0a\x11\x04\x84`\xE0\x01QQa$\xC6V[c\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R T\x90\x91P`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x11\x81W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7Fi/o length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[\x81c\xFF\xFF\xFF\xFF\x16\x84``\x01QQ\x14a\x11\xDCW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FsigHashes length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[\x80c\xFF\xFF\xFF\xFF\x16\x84a\x01\0\x01QQ\x14a\x128W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FoutRollupFees length\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[\x80c\xFF\xFF\xFF\xFF\x16\x84a\x01`\x01QQ\x14a\x12\x94W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FoutEncryptedNotes length\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81c\xFF\xFF\xFF\xFF\x16`\x07T`\x08Ta\x12\xCC\x91\x90a;qV[a\x12\xD6\x91\x90a;qV[\x11\x15a\x12\xF5W`@Qc;\\\xCBC`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xFE2a%FV[\x15a\x13\x1CW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13*\x84a\x01 \x01Qa%FV[\x15a\x13HW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13Y`\x05c\xFF\xFF\xFF\xFF\x84\x16a;\xB1V[\x84a\x01\xA0\x01QQ\x14a\x13~W`@Qc:\xCFH\xBB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13\x89\x85a%\xE5V[\x90Pa\x13\x95\x81\x85a'qV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x1B\x85`\x80\x01Q\x14a\x13\xF4W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7Fsignature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\0a\x14\x01\x84`\x02a;\xD0V[c\xFF\xFF\xFF\xFF\x16\x90P`\0a\x14\x16\x85`\x02a;\xD0V[a\x14!\x90`\x04a;\x89V[c\xFF\xFF\xFF\xFF\x16\x90P`\0a\x146\x85`\x02a;\xD0V[a\x14F\x90c\xFF\xFF\xFF\xFF\x16\x83a;qV[\x90P`\0`\x05a\x14W\x88`\x02a;\x89V[c\xFF\xFF\xFF\xFF\x16a\x14g\x91\x90a;\xB1V[a\x14r\x83`\x02a;qV[a\x14|\x91\x90a;qV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\x94Wa\x14\x94a=\x07V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xBDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x80\x8B\x01Q`\0\x90\x81R`\n\x90\x91R`@\x90 T\x90\x91P`\xFF\x16a\x15\x0FW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R`\x04\x01a\n\xF1\x90` \x80\x82R`\x04\x90\x82\x01Rc\x1C\x9B\xDB\xDD`\xE2\x1B`@\x82\x01R``\x01\x90V[\x88` \x01Q\x81`\0\x81Q\x81\x10a\x15'Wa\x15'a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0a\x15?\x88`\x01a;\x89V[c\xFF\xFF\xFF\xFF\x16\x90P`\0[\x88c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x16KW`\0\x8B`@\x01Q\x82\x81Q\x81\x10a\x15pWa\x15pa<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\0\x81\x81R`\x05\x90\x92R`@\x90\x91 T\x90\x91P`\xFF\x16\x15a\x15\xC9W`@Q\x7F\xFFUn \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x84a\x15\xD6\x84`\x01a;qV[\x81Q\x81\x10a\x15\xE6Wa\x15\xE6a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x8B``\x01Q\x82\x81Q\x81\x10a\x16\x08Wa\x16\x08a<\xF1V[` \x02` \x01\x01Q\x84\x84\x84a\x16\x1D\x91\x90a;qV[\x81Q\x81\x10a\x16-Wa\x16-a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\x16C\x81a<dV[\x91PPa\x15JV[P`\x80\x8A\x01Q\x82a\x16]\x87`\x01a;qV[\x81Q\x81\x10a\x16mWa\x16ma<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xA0\x8A\x01Q\x82a\x16\x89\x87`\x02a;qV[\x81Q\x81\x10a\x16\x99Wa\x16\x99a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xC0\x8A\x01Q\x82a\x16\xB5\x87`\x03a;qV[\x81Q\x81\x10a\x16\xC5Wa\x16\xC5a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0a\x16\xE2c\xFF\xFF\xFF\xFF\x89\x16\x86a;qV[\x90P`\0[\x88c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x18\\W`\x04`\0\x8D`\xE0\x01Q\x83\x81Q\x81\x10a\x17\x0FWa\x17\x0Fa<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15a\x17{W`@Qc\xBE\xE3a\x11`\xE0\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7Fcommitment\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\rT\x8Ca\x01\0\x01Q\x82\x81Q\x81\x10a\x17\x95Wa\x17\x95a<\xF1V[` \x02` \x01\x01Q\x10\x15a\x17\xBCW`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x8B`\xE0\x01Q\x81\x81Q\x81\x10a\x17\xD2Wa\x17\xD2a<\xF1V[` \x02` \x01\x01Q\x84\x87\x83a\x17\xE7\x91\x90a;qV[\x81Q\x81\x10a\x17\xF7Wa\x17\xF7a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01\0\x01Q\x81\x81Q\x81\x10a\x18\x1AWa\x18\x1Aa<\xF1V[` \x02` \x01\x01Q\x84\x83\x83a\x18/\x91\x90a;qV[\x81Q\x81\x10a\x18?Wa\x18?a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x18T\x81a<dV[\x91PPa\x16\xE7V[Pa\x18h\x8B\x84\x86a'\x95V[c\xFF\xFF\xFF\xFF\x80\x8A\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x8C\x16\x83R\x92\x90R\x81\x81 T\x8DQ\x92Qc\xC9AvG`\xE0\x1B\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC9AvG\x91a\x18\xBD\x91\x88\x90`\x04\x01a9\xFAV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xD7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x0F\x91\x90a4\x05V[\x90P\x80a\x19_W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Ftransact proof\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\0[\x8Ac\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x1A3W`\x01`\x05`\0\x8F`@\x01Q\x84\x81Q\x81\x10a\x19\x8CWa\x19\x8Ca<\xF1V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\t`\0\x82\x82Ta\x19\xCC\x91\x90a;qV[\x90\x91UPP`@\x8D\x01Q\x80Q\x82\x90\x81\x10a\x19\xE8Wa\x19\xE8a<\xF1V[` \x02` \x01\x01Q\x8D` \x01Q\x7F<#r\xABa0\x81{\xD6\xB8\xFCm\xBA\xEC\xAE\x94~\x84 \x1BIS]5\x8D\xEB\xAAl4\xC2>\xCF`@Q`@Q\x80\x91\x03\x90\xA3\x80a\x1A+\x81a<dV[\x91PPa\x19bV[P`\0[\x89c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x1B\x04W`\x01`\x04`\0\x8F`\xE0\x01Q\x84\x81Q\x81\x10a\x1AaWa\x1Aaa<\xF1V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x1A\xF2\x8D`\xE0\x01Q\x82\x81Q\x81\x10a\x1A\xA7Wa\x1A\xA7a<\xF1V[` \x02` \x01\x01Q\x8Ea\x01\0\x01Q\x83\x81Q\x81\x10a\x1A\xC6Wa\x1A\xC6a<\xF1V[` \x02` \x01\x01Q\x8Fa\x01`\x01Q\x84\x81Q\x81\x10a\x1A\xE5Wa\x1A\xE5a<\xF1V[` \x02` \x01\x01Qa)\x8CV[\x80a\x1A\xFC\x81a<dV[\x91PPa\x1A7V[P`\xA0\x8C\x01Q\x15a\x1B\"Wa\x1B\"\x8Ca\x01 \x01Q\x8D`\xA0\x01Qa*+V[`\xC0\x8C\x01Q\x15a\x1B?Wa\x1B?\x8Ca\x01@\x01Q\x8D`\xC0\x01Qa*+V[a\x1BH\x8Ca*\xD3V[PP`\x01`\0UPPPPPPPPPPV[3`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x1B\x8BW`@Qc\xF7^\x9F\xC7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\rT\x82``\x01Q\x10\x15a\x1B\xB2W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x07T`\x08Ta\x1B\xE3\x91\x90a;qV[\x10a\x1C\x01W`@Qc;\\\xCBC`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x83\x01Q`\0\x90\x81R`\x04\x90\x91R`@\x90 T`\xFF\x16\x15a\x1C7W`@Qc\xE3\x8C\xD1M`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x83\x01\x80Q`\0\x90\x81R`\x04\x90\x92R`@\x90\x91 \x80T`\xFF\x19\x16`\x01\x17\x90UQ``\x83\x01Q`\x80\x84\x01Qa\x1Cn\x92\x91\x90a)\x8CV[`@\x82\x01Q\x15a\x1C\x86Wa\x1C\x86\x81\x83`@\x01Qa,\xE8V[PPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\xB5W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x1D\x03W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7F_minRollupFee\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\rUV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1D3W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T`\xFF\x16\x15a\x1DWW`@Qc6\xE3\xE0\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x83\x16a\x1D{W`@Qc\x9F{\xD9K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R`\x01` \x80\x83\x01\x91\x82Rc\xFF\xFF\xFF\xFF\x96\x87\x16`\0\x90\x81R`\x02\x82R\x84\x81 \x96\x90\x97\x16\x87R\x94\x90\x94R\x93 \x92Q\x83T\x92Q\x15\x15`\x01`\xA0\x1B\x02t\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x16\x17\x17\x90UV[```\0`\x07Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\nWa\x1E\na=\x07V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\x07T\x81\x10\x15a\x10\x83W`\0\x81`\x08Ta\x1ES\x91\x90a;qV[`\0\x81\x81R`\x06` R`@\x90 T\x84Q\x91\x92P\x90\x84\x90\x84\x90\x81\x10a\x1EzWa\x1Eza<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\x1E\x90\x81a<dV[\x91PPa\x1E9V[`\0`\x05\x82\x10a\x1E\xBBW`@Qc1\x8CCE`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x11\x82`\x05\x81\x10a\x1E\xCEWa\x1E\xCEa<\xF1V[\x01T\x92\x91PPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1F\x01W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T`\xFF\x16\x15a\x1F%W`@Qc6\xE3\xE0\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x81\x16\x15\x80a\x1F?WPa\x04\0\x81c\xFF\xFF\xFF\xFF\x16\x11[\x15a\x1FzW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01a\n\xF1V[a\x1F\x85`\x01\x82a<\x13V[\x81\x16c\xFF\xFF\xFF\xFF\x16`\0\x14a\x1F\xADW`@Qc\"q\x7F\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\xFF`\xA0\x1B\x19\x16\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1F\xF9W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0E` R`@\x90 \x80T`\xFF\x19\x16\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a EW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0F` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a \x94W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xCFT\xE4\x9C\xBF\x0F\xD7a?\xB6\xB5\xB0\x9F.\x8C\x12U\xA9\xDF\x94\xA8&9\x0Fx=5\xCF+M\xA7\xE4\x90` \x01a\x0F\xEAV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a!\0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T`\xFF\x16\x15a!$W`@Qc6\xE3\xE0\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x82\x16a!HW`@Qc\x9F{\xD9K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x91\x82\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R \x80T`\xFF`\xA0\x1B\x19\x16\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a!\xA4W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\xFF`\xA0\x1B\x19\x16\x90\x81\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x91a\x07\xE0\x91`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\x1CW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T`\xFF\x16\x15a\"@W`@Qc6\xE3\xE0\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x82\x16\x15\x80a\"ZWPa\x04\0\x82c\xFF\xFF\xFF\xFF\x16\x11[\x15a\"\x95W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01a\n\xF1V[a\"\xA0`\x01\x83a<\x13V[\x82\x16c\xFF\xFF\xFF\xFF\x16`\0\x14a\"\xC8W`@Qc\"q\x7F\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R`\x01` \x80\x83\x01\x91\x82Rc\xFF\xFF\xFF\xFF\x90\x95\x16`\0\x90\x81R`\x03\x90\x95R\x91\x90\x93 \x92Q\x83T\x91Q\x15\x15`\x01`\xA0\x1B\x02t\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x92\x16\x91\x90\x91\x17\x17\x90UV[`\0a\x01\0\x82c\xFF\xFF\xFF\xFF\x16\x10a#WW`\x08\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x08\x83\x90\x1C\x92P[`\x10\x82c\xFF\xFF\xFF\xFF\x16\x10a#zW`\x04\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x04\x83\x90\x1C\x92P[`\x04\x82c\xFF\xFF\xFF\xFF\x16\x10a#\x9DW`\x02\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x02\x83\x90\x1C\x92P[`\x02\x82c\xFF\xFF\xFF\xFF\x16\x10a#\xB3W`\x01\x83\x90\x1C\x92P[P\x90\x91\x90PV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a#\xE5W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10\x80T\x82\x15\x15a\x01\0\x02a\xFF\0\x19\x90\x91\x16\x17\x90U`@Q\x7F\x9A$\xEEpHT\xE0\xC51\xDA\x82\x1C\x99`E@i\x0F*\xF5W},\x89,)!\xCE\xA4\t\xB4y\x90a\x0F\xEA\x90\x83\x15\x15\x81R` \x01\x90V[`@Q`\0\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a$pW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a$uV[``\x91P[PP\x90P\x80a\x1C\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Frollup fee transfer failed\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a%BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01R\x7F2 bits\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xF1V[P\x90V[`\x01T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a%bWP`\0\x91\x90PV[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a%\xA7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a%\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xDF\x91\x90a4\x05V[\x92\x91PPV[a\x01`\x81\x01QQ`\0\x90`\x03\x81\x10a&\x10W`@Qc?\xB1\x94]`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81a&dWa\x01 \x84\x01Qa\x01@\x85\x01Q`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x81\x16` \x83\x01R\x91\x90\x92\x1B\x16`4\x82\x01R`H\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa'\x1CV[\x81`\x01\x14\x15a&\xABW\x83a\x01 \x01Q\x84a\x01@\x01Q\x85a\x01`\x01Q`\0\x81Q\x81\x10a&\x91Wa&\x91a<\xF1V[` \x02` \x01\x01Q`@Q` \x01a&N\x93\x92\x91\x90a8\x84V[\x83a\x01 \x01Q\x84a\x01@\x01Q\x85a\x01`\x01Q`\0\x81Q\x81\x10a&\xCFWa&\xCFa<\xF1V[` \x02` \x01\x01Q\x86a\x01`\x01Q`\x01\x81Q\x81\x10a&\xEFWa&\xEFa<\xF1V[` \x02` \x01\x01Q`@Q` \x01a'\n\x94\x93\x92\x91\x90a8\xCAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[\x80Q` \x91\x82\x01 `@\x80Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81\x85\x01R`<\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\\\x01\x90R\x80Q\x91\x01 \x93\x92PPPV[`\0\x80`\0a'\x80\x85\x85a-\x8BV[\x91P\x91Pa'\x8D\x81a-\xFBV[P\x93\x92PPPV[a\x01\x80\x83\x01Q`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\xFF\x83\x90\x1C\x80\x83R`\x01`\x01`\xFF\x1B\x03\x90\x93\x16\x90\x82\x01R\x83Q\x90\x91\x90\x84\x90\x84\x90\x81\x10a'\xE5Wa'\xE5a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x83\x83`\x01a(\x04\x91\x90a;qV[\x81Q\x81\x10a(\x14Wa(\x14a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0a(,\x83`\x02a;qV[\x90P`\0a(;`\x05\x83a;qV[\x90P`\0a(K`\x05`\x02a;\xB1V[a(U\x90\x84a;qV[\x90P`\0[`\x05\x81\x10\x15a)!W`\0a(\xB5`\x11\x83`\x05\x81\x10a({Wa({a<\xF1V[\x01T`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\xFF\x83\x90\x1C\x82R`\x01`\x01`\xFF\x1B\x03\x90\x92\x16\x91\x81\x01\x91\x90\x91R\x90V[\x80Q\x90\x91P\x88a(\xC5\x84\x88a;qV[\x81Q\x81\x10a(\xD5Wa(\xD5a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x88\x83\x86a(\xF3\x91\x90a;qV[\x81Q\x81\x10a)\x03Wa)\x03a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a)\x19\x81a<dV[\x91PPa(ZV[P`\0[\x87a\x01\xA0\x01QQ\x81\x10\x15a\x0F^W\x87a\x01\xA0\x01Q\x81\x81Q\x81\x10a)JWa)Ja<\xF1V[` \x02` \x01\x01Q\x87\x82\x84a)_\x91\x90a;qV[\x81Q\x81\x10a)oWa)oa<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a)\x84\x81a<dV[\x91PPa)%V[`\0`\x08T`\x07Ta)\x9E\x91\x90a;qV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R`\0\x85\x81R`\x06\x90\x92R\x92\x81 \x91Q\x82U\x91Q`\x01\x91\x82\x01U`\x07\x80T\x93\x94P\x90\x92\x90\x91\x90a)\xE2\x90\x84\x90a;qV[\x92PP\x81\x90UP\x83\x7F\xF53\xF9pZ\xACP \xE2\x16\x95\xEA5S\xAC{h\x81\x07\r+i\0\xAB+\x1E0P0K[\xF9\x84\x83\x85`@Qa*\x1D\x93\x92\x91\x90a:kV[`@Q\x80\x91\x03\x90\xA2PPPPV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a*xW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a*}V[``\x91P[PP\x90P\x80a*\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7Fwithdraw failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[PPPV[`\0`\x05\x82`@\x01QQa*\xE7\x91\x90a;\xB1V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x04Wa+\x04a=\x07V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+cW\x81` \x01[a+P`@Q\x80``\x01`@R\x80`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a+\"W\x90P[P\x90P`\0\x80[\x84`@\x01QQ\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a,\xAAW`\0[`\x05\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a,\x97W\x80c\xFF\xFF\xFF\xFF\x16` \x83c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B\x17\x84\x84\x81Q\x81\x10a+\xBEWa+\xBEa<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90R`\x11c\xFF\xFF\xFF\xFF\x82\x16`\x05\x81\x10a+\xF0Wa+\xF0a<\xF1V[\x01T\x84\x84\x81Q\x81\x10a,\x04Wa,\x04a<\xF1V[` \x02` \x01\x01Q` \x01\x81\x81RPP\x85a\x01\xA0\x01Q\x81c\xFF\xFF\xFF\xFF\x16`\x05\x84c\xFF\xFF\xFF\xFF\x16a,4\x91\x90a;\xB1V[a,>\x91\x90a;qV[\x81Q\x81\x10a,NWa,Na<\xF1V[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a,hWa,ha<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@\x01R\x82a,\x81\x81a<dV[\x93PP\x80\x80a,\x8F\x90a<\x7FV[\x91PPa+\x80V[P\x80a,\xA2\x81a<\x7FV[\x91PPa+jV[P\x7Fj\xF0~\xBC\xB391\xAB\xD3H\xE7\x85\xF7\x0Cm%\x93\x90\xCFy\x91\x91\xF8\xC9\x91$$[\x96\xE1\xA6\x13\x82`@Qa,\xDA\x91\x90a9\\V[`@Q\x80\x91\x03\x90\xA1PPPPV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a-5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a-:V[``\x91P[PP\x90P\x80a*\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fexecutor fee transfer failed\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\0\x80\x82Q`A\x14\x15a-\xC2W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa-\xB6\x87\x82\x85\x85a/\xB9V[\x94P\x94PPPPa-\xF4V[\x82Q`@\x14\x15a-\xECW` \x83\x01Q`@\x84\x01Qa-\xE1\x86\x83\x83a0\xA6V[\x93P\x93PPPa-\xF4V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a.\x0FWa.\x0Fa<\xDBV[\x14\x15a.\x18WPV[`\x01\x81`\x04\x81\x11\x15a.,Wa.,a<\xDBV[\x14\x15a.zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\x02\x81`\x04\x81\x11\x15a.\x8EWa.\x8Ea<\xDBV[\x14\x15a.\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\n\xF1V[`\x03\x81`\x04\x81\x11\x15a.\xF0Wa.\xF0a<\xDBV[\x14\x15a/IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xF1V[`\x04\x81`\x04\x81\x11\x15a/]Wa/]a<\xDBV[\x14\x15a/\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xF1V[PV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a/\xF0WP`\0\x90P`\x03a0\x9DV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a0\x08WP\x84`\xFF\x16`\x1C\x14\x15[\x15a0\x19WP`\0\x90P`\x04a0\x9DV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a0mW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a0\x96W`\0`\x01\x92P\x92PPa0\x9DV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a0\xC3`\xFF\x86\x90\x1C`\x1Ba;qV[\x90Pa0\xD1\x87\x82\x88\x85a/\xB9V[\x93P\x93PPP\x93P\x93\x91PPV[\x805a0\xEA\x81a=\x1DV[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a1\0W`\0\x80\xFD[\x815` a1\x15a1\x10\x83a;MV[a;\x1CV[\x80\x83\x82R\x82\x82\x01\x91P\x82\x86\x01\x87\x84\x86`\x05\x1B\x89\x01\x01\x11\x15a15W`\0\x80\xFD[`\0\x80[\x86\x81\x10\x15a1xW\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1WW\x82\x83\xFD[a1e\x8B\x88\x83\x8D\x01\x01a2KV[\x86RP\x93\x85\x01\x93\x91\x85\x01\x91`\x01\x01a19V[P\x91\x98\x97PPPPPPPPV[`\0\x82`\x1F\x83\x01\x12a1\x97W`\0\x80\xFD[a1\x9Fa:\xACV[\x80\x83\x85`@\x86\x01\x11\x15a1\xB1W`\0\x80\xFD[`\0[`\x02\x81\x10\x15a1\xD3W\x815\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a1\xB4V[P\x90\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a1\xEFW`\0\x80\xFD[\x815` a1\xFFa1\x10\x83a;MV[\x80\x83\x82R\x82\x82\x01\x91P\x82\x86\x01\x87\x84\x86`\x05\x1B\x89\x01\x01\x11\x15a2\x1FW`\0\x80\xFD[`\0[\x85\x81\x10\x15a2>W\x815\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a2\"V[P\x90\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12a2\\W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2vWa2va=\x07V[a2\x89`\x1F\x82\x01`\x1F\x19\x16` \x01a;\x1CV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a2\x9EW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a2\xCDW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a2\xF0Wa2\xF0a=\x07V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x81\x83\x03a\x01\0\x81\x12\x15a3\x1EW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a3AWa3Aa=\x07V[`@R\x91P\x81a3Q\x85\x85a2\xBBV[\x81R`\x80`?\x19\x83\x01\x12\x15a3eW`\0\x80\xFD[a3ma:\xACV[\x91Pa3|\x85`@\x86\x01a1\x86V[\x82Ra3\x8B\x85`\x80\x86\x01a1\x86V[` \x83\x01R\x81` \x82\x01Ra3\xA3\x85`\xC0\x86\x01a2\xBBV[`@\x82\x01RPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a0\xEAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a3\xD6W`\0\x80\xFD[\x815a3\xE1\x81a=\x1DV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a3\xFAW`\0\x80\xFD[\x815a3\xE1\x81a=2V[`\0` \x82\x84\x03\x12\x15a4\x17W`\0\x80\xFD[\x81Qa3\xE1\x81a=2V[`\0\x80`@\x83\x85\x03\x12\x15a45W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a4MW`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a4aW`\0\x80\xFD[a4ia:\xD5V[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a4\x9DW`\0\x80\xFD[a4\xA9\x88\x82\x86\x01a2KV[`\x80\x83\x01RP\x93Pa4\xC0\x91PP` \x84\x01a0\xDFV[\x90P\x92P\x92\x90PV[`\0a\x01`\x82\x84\x03\x12\x15a4\xDCW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\xFFWa4\xFFa=\x07V[`@Ra5\x0C\x84\x84a3\nV[\x81Ra5\x1Ba\x01\0\x84\x01a3\xB0V[` \x82\x01Ra\x01 \x83\x015`@\x82\x01Ra\x01@\x90\x92\x015``\x83\x01RP\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a5PW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a5hW`\0\x80\xFD[\x90\x84\x01\x90a\x02\xA0\x82\x87\x03\x12\x15a5}W`\0\x80\xFD[a5\x85a:\xF8V[a5\x8F\x87\x84a3\nV[\x81Ra\x01\0\x80\x84\x015` \x83\x01Ra\x01 \x80\x85\x015\x84\x81\x11\x15a5\xB1W`\0\x80\xFD[a5\xBD\x8A\x82\x88\x01a1\xDEV[`@\x85\x01RPa\x01@\x80\x86\x015\x85\x81\x11\x15a5\xD7W`\0\x80\xFD[a5\xE3\x8B\x82\x89\x01a1\xDEV[``\x86\x01RPa\x01`\x80\x87\x015`\x80\x86\x01Ra\x01\x80\x80\x88\x015`\xA0\x87\x01Ra\x01\xA0\x80\x89\x015`\xC0\x88\x01Ra\x01\xC0\x89\x015\x88\x81\x11\x15a6 W`\0\x80\xFD[a6,\x8E\x82\x8C\x01a1\xDEV[`\xE0\x89\x01RPa\x01\xE0\x89\x015\x88\x81\x11\x15a6EW`\0\x80\xFD[a6Q\x8E\x82\x8C\x01a1\xDEV[\x87\x89\x01RPa6ca\x02\0\x8A\x01a0\xDFV[\x85\x88\x01Ra6ta\x02 \x8A\x01a0\xDFV[\x84\x88\x01Ra\x02@\x89\x015\x95P\x87\x86\x11\x15a6\x8DW`\0\x80\xFD[a6\x99\x8D\x87\x8B\x01a0\xEFV[\x83\x88\x01Ra\x02`\x89\x015\x82\x88\x01Ra\x02\x80\x89\x015\x95P\x87\x86\x11\x15a6\xBCW`\0\x80\xFD[a6\xC8\x8D\x87\x8B\x01a1\xDEV[\x90\x87\x01RP\x93\x97PPPP` \x86\x015\x92PP\x80\x82\x11\x15a6\xE8W`\0\x80\xFD[Pa6\xF5\x85\x82\x86\x01a2KV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a7\x11W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a7+W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`@\x83\x85\x03\x12\x15a7MW`\0\x80\xFD[\x825\x91Pa4\xC0` \x84\x01a3\xB0V[`\0` \x82\x84\x03\x12\x15a7oW`\0\x80\xFD[a3\xE1\x82a3\xB0V[`\0\x80`@\x83\x85\x03\x12\x15a7\x8BW`\0\x80\xFD[a7\x94\x83a3\xB0V[\x91P` \x83\x015a7\xA4\x81a=\x1DV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a7\xC2W`\0\x80\xFD[a7\xCB\x83a3\xB0V[\x91Pa4\xC0` \x84\x01a3\xB0V[`\0\x80`\0``\x84\x86\x03\x12\x15a7\xEEW`\0\x80\xFD[a7\xF7\x84a3\xB0V[\x92Pa8\x05` \x85\x01a3\xB0V[\x91P`@\x84\x015a8\x15\x81a=\x1DV[\x80\x91PP\x92P\x92P\x92V[\x80`\0[`\x02\x81\x10\x15a8CW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a8$V[PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a8yW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a8]V[P\x94\x95\x94PPPPPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x86``\x1B\x16\x83R\x80\x85``\x1B\x16`\x14\x84\x01RP\x82Qa8\xBB\x81`(\x85\x01` \x87\x01a<8V[\x91\x90\x91\x01`(\x01\x94\x93PPPPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x87``\x1B\x16\x83R\x80\x86``\x1B\x16`\x14\x84\x01RP\x83Qa9\x01\x81`(\x85\x01` \x88\x01a<8V[\x83Q\x90\x83\x01\x90a9\x18\x81`(\x84\x01` \x88\x01a<8V[\x01`(\x01\x96\x95PPPPPPV[\x81Q`\0\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a9PW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a94V[P\x92\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a9\xB2W\x81Q\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85R\x86\x81\x01Q\x87\x86\x01R\x85\x01Q\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a9yV[P\x91\x97\x96PPPPPPPV[` \x81R`\0a3\xE1` \x83\x01\x84a8IV[` \x81\x01`\x02\x83\x10a9\xF4WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0a\x01 a:\x14\x83\x86Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[` \x85\x01Qa:'`@\x85\x01\x82Qa8 V[` \x01Qa:8`\x80\x85\x01\x82a8 V[P`@\x85\x01Q\x80Q`\xC0\x85\x01R` \x01Q`\xE0\x84\x01Ra\x01\0\x83\x01\x81\x90Ra:b\x81\x84\x01\x85a8IV[\x95\x94PPPPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\0\x82Q\x80``\x84\x01Ra:\x96\x81`\x80\x85\x01` \x87\x01a<8V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`\x80\x01\x94\x93PPPPV[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a:\xCFWa:\xCFa=\x07V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a:\xCFWa:\xCFa=\x07V[`@Qa\x01\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a:\xCFWa:\xCFa=\x07V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;EWa;Ea=\x07V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a;gWa;ga=\x07V[P`\x05\x1B` \x01\x90V[`\0\x82\x19\x82\x11\x15a;\x84Wa;\x84a<\xC5V[P\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a;\xA8Wa;\xA8a<\xC5V[\x01\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a;\xCBWa;\xCBa<\xC5V[P\x02\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a;\xF3Wa;\xF3a<\xC5V[\x02\x94\x93PPPPV[`\0\x82\x82\x10\x15a<\x0EWa<\x0Ea<\xC5V[P\x03\x90V[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a<0Wa<0a<\xC5V[\x03\x93\x92PPPV[`\0[\x83\x81\x10\x15a<SW\x81\x81\x01Q\x83\x82\x01R` \x01a<;V[\x83\x81\x11\x15a8CWPP`\0\x91\x01RV[`\0`\0\x19\x82\x14\x15a<xWa<xa<\xC5V[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a<\x99Wa<\x99a<\xC5V[`\x01\x01\x93\x92PPPV[`\0\x82a<\xC0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a/\xB6W`\0\x80\xFD[\x80\x15\x15\x81\x14a/\xB6W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x85\xC0v_\xA8l\x177R\xA7o\xC4<\x01R^GY\xAE\xF8H\xDF\x91\x85\xB5\x87FTkQ\xC5\x15dsolcC\0\x08\x07\x003";
    /// The bytecode of the contract.
    pub static COMMITMENTPOOLMAIN_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02\x89W`\x005`\xE0\x1C\x80c|\xBF\x0F\xF6\x11a\x01SW\x80c\xB1\xC3\x94\"\x11a\0\xCBW\x80c\xE5\0\xF5\x04\x11a\0\x7FW\x80c\xF2\xDA\x1DA\x11a\0dW\x80c\xF2\xDA\x1DA\x14a\x07\tW\x80c\xF8\xF0S\x88\x14a\x07)W\x80c\xFF\xA8\x9B\x88\x14a\x07IW`\0\x80\xFD[\x80c\xE5\0\xF5\x04\x14a\x06\xBCW\x80c\xECW\x1Cj\x14a\x06\xD1W`\0\x80\xFD[\x80c\xC2Y\xE2\xE6\x11a\0\xB0W\x80c\xC2Y\xE2\xE6\x14a\x06gW\x80c\xDDu|4\x14a\x06\x87W\x80c\xDE\xEF\xF7\xCD\x14a\x06\x9CW`\0\x80\xFD[\x80c\xB1\xC3\x94\"\x14a\x06&W\x80c\xB3\xB7V1\x14a\x06GW`\0\x80\xFD[\x80c\x9B\no\xEA\x11a\x01\"W\x80c\xA6#*\x93\x11a\x01\x07W\x80c\xA6#*\x93\x14a\x05\xC1W\x80c\xA9\xB1\xD2\x96\x14a\x05\xF1W\x80c\xB0\x88\x92\xD0\x14a\x06\x11W`\0\x80\xFD[\x80c\x9B\no\xEA\x14a\x05\x81W\x80c\x9C\xC6\xB3T\x14a\x05\xA1W`\0\x80\xFD[\x80c|\xBF\x0F\xF6\x14a\x05\x0CW\x80c\x7F\xA4\xB0\x9C\x14a\x05,W\x80c\x86j\xC6X\x14a\x05LW\x80c\x87x\r\xF9\x14a\x05aW`\0\x80\xFD[\x80c?\xE34z\x11a\x02\x01W\x80cW\x06\0\x16\x11a\x01\xB5W\x80cr\x08)q\x11a\x01\x9AW\x80cr\x08)q\x14a\x04\xB7W\x80cx\xD6\x0C\xD7\x14a\x04\xD7W\x80czU7D\x14a\x04\xF7W`\0\x80\xFD[\x80cW\x06\0\x16\x14a\x04eW\x80cc\xBC}2\x14a\x04\x95W`\0\x80\xFD[\x80cN\xB0i\xF7\x11a\x01\xE6W\x80cN\xB0i\xF7\x14a\x04#W\x80cU]u\xF0\x14a\x04;W\x80cV\x88\x88\x1F\x14a\x04PW`\0\x80\xFD[\x80c?\xE34z\x14a\x03\xD4W\x80cHN\xB6R\x14a\x03\xF0W`\0\x80\xFD[\x80c\x0C\x88g\xE6\x11a\x02XW\x80c\x14\xA7s}\x11a\x02=W\x80c\x14\xA7s}\x14a\x03TW\x80c0\xF4\x9C\xAC\x14a\x03tW\x80c;\xB8\xD1\xB4\x14a\x03\x94W`\0\x80\xFD[\x80c\x0C\x88g\xE6\x14a\x03\x0CW\x80c\x11_WL\x14a\x03,W`\0\x80\xFD[\x80c\x01\xDB\xF1\x9F\x14a\x02\x95W\x80c\x02\xD4\x98\xF1\x14a\x02\xACW\x80c\x03\xDB\x98t\x14a\x02\xCCW\x80c\x069L\x9B\x14a\x02\xECW`\0\x80\xFD[6a\x02\x90W\0[`\0\x80\xFD[4\x80\x15a\x02\xA1W`\0\x80\xFD[Pa\x02\xAAa\x07fV[\0[4\x80\x15a\x02\xB8W`\0\x80\xFD[Pa\x02\xAAa\x02\xC76`\x04a3\xC4V[a\x07\xEAV[4\x80\x15a\x02\xD8W`\0\x80\xFD[Pa\x02\xAAa\x02\xE76`\x04a3\xC4V[a\x089V[4\x80\x15a\x02\xF8W`\0\x80\xFD[Pa\x02\xAAa\x03\x076`\x04a3\xC4V[a\x08\x85V[4\x80\x15a\x03\x18W`\0\x80\xFD[Pa\x02\xAAa\x03'6`\x04a7\x18V[a\t6V[4\x80\x15a\x038W`\0\x80\xFD[Pa\x03A`\x05\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03`W`\0\x80\xFD[Pa\x02\xAAa\x03o6`\x04a4\xC9V[a\n\x05V[4\x80\x15a\x03\x80W`\0\x80\xFD[Pa\x02\xAAa\x03\x8F6`\x04a3\xC4V[a\x0FhV[4\x80\x15a\x03\xA0W`\0\x80\xFD[Pa\x03\xC4a\x03\xAF6`\x04a6\xFFV[`\0\x90\x81R`\x05` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x03KV[4\x80\x15a\x03\xE0W`\0\x80\xFD[P`\x01`@Qa\x03K\x91\x90a9\xD2V[4\x80\x15a\x03\xFCW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03AV[4\x80\x15a\x04/W`\0\x80\xFD[P`\x10T`\xFF\x16a\x03\xC4V[4\x80\x15a\x04GW`\0\x80\xFD[P`\x07Ta\x03AV[4\x80\x15a\x04\\W`\0\x80\xFD[Pa\x03Aa\x0F\xF5V[4\x80\x15a\x04qW`\0\x80\xFD[Pa\x03\xC4a\x04\x806`\x04a6\xFFV[`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x90V[4\x80\x15a\x04\xA1W`\0\x80\xFD[Pa\x04\xAAa\x10\x0CV[`@Qa\x03K\x91\x90a9\xBFV[4\x80\x15a\x04\xC3W`\0\x80\xFD[Pa\x02\xAAa\x04\xD26`\x04a5=V[a\x10\x89V[4\x80\x15a\x04\xE3W`\0\x80\xFD[Pa\x02\xAAa\x04\xF26`\x04a4\"V[a\x1B[V[4\x80\x15a\x05\x03W`\0\x80\xFD[P`\tTa\x03AV[4\x80\x15a\x05\x18W`\0\x80\xFD[Pa\x02\xAAa\x05'6`\x04a6\xFFV[a\x1C\x8AV[4\x80\x15a\x058W`\0\x80\xFD[Pa\x02\xAAa\x05G6`\x04a7\xD9V[a\x1D\x08V[4\x80\x15a\x05XW`\0\x80\xFD[Pa\x04\xAAa\x1D\xEBV[4\x80\x15a\x05mW`\0\x80\xFD[Pa\x03Aa\x05|6`\x04a6\xFFV[a\x1E\x98V[4\x80\x15a\x05\x8DW`\0\x80\xFD[Pa\x02\xAAa\x05\x9C6`\x04a7]V[a\x1E\xD6V[4\x80\x15a\x05\xADW`\0\x80\xFD[Pa\x02\xAAa\x05\xBC6`\x04a3\xC4V[a\x1F\xCEV[4\x80\x15a\x05\xCDW`\0\x80\xFD[Pa\x03\xC4a\x05\xDC6`\x04a6\xFFV[`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16\x90V[4\x80\x15a\x05\xFDW`\0\x80\xFD[Pa\x02\xAAa\x06\x0C6`\x04a3\xC4V[a \x1AV[4\x80\x15a\x06\x1DW`\0\x80\xFD[P`\rTa\x03AV[4\x80\x15a\x062W`\0\x80\xFD[P`\x01Ta\x03\xC4\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x06SW`\0\x80\xFD[Pa\x02\xAAa\x06b6`\x04a3\xE8V[a iV[4\x80\x15a\x06sW`\0\x80\xFD[Pa\x02\xAAa\x06\x826`\x04a7\xAFV[a \xD5V[4\x80\x15a\x06\x93W`\0\x80\xFD[Pa\x02\xAAa!yV[4\x80\x15a\x06\xA8W`\0\x80\xFD[Pa\x02\xAAa\x06\xB76`\x04a7xV[a!\xF1V[4\x80\x15a\x06\xC8W`\0\x80\xFD[P`\x08Ta\x03AV[4\x80\x15a\x06\xDDW`\0\x80\xFD[P`\x01Ta\x06\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03KV[4\x80\x15a\x07\x15W`\0\x80\xFD[Pa\x03Aa\x07$6`\x04a7:V[a#1V[4\x80\x15a\x075W`\0\x80\xFD[Pa\x02\xAAa\x07D6`\x04a3\xE8V[a#\xBAV[4\x80\x15a\x07UW`\0\x80\xFD[P`\x10Ta\x01\0\x90\x04`\xFF\x16a\x03\xC4V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x91W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x90\x81\x17\x91\x82\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x92a\x07\xE0\x92\x90\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\x15W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0E` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08dW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0F` R`@\x90 \x80T`\xFF\x19\x16\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xB0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x15a\x08\xDFW`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0C\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FG!\x12\x9E\x0Egn\xD6\xA9)\t\xBB$\xE8S\xCC\xDDc\xADr(\x0C\xC2\xE9t\xE3\x8EH\x0E\x0EnT\x90`\0\x90\xA2PV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\taW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x82\x10a\t\x82W`@Qc1\x8CCE`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x11\x83`\x05\x81\x10a\t\x96Wa\t\x96a<\xF1V[\x01T\x14\x15a\t\xB6W`@QbJpg`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x11\x83`\x05\x81\x10a\t\xCAWa\t\xCAa<\xF1V[\x01U`@Q\x81\x81R\x82\x90\x7F\x94\xAD\xCA\x83\xCEAD|\xFB\x8E\x07\xD1\xC9\xC4Sj\x1B\x8A\xFB\xFFl|\x83\xC2\xD9\x18!J\xDCh\x10/\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[`\x10Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\n-WP3`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16\x15[\x15a\nKW`@Qc!\xCE\x01\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80\x82\x01Q`\0\x90\x81R`\n` R T`\xFF\x16\x15a\n~W`@Qc\xE2\xE1!\x03`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x11\x80a\n\xBAWP` \x81\x81\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x03\x90\x91R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15[\x15a\n\xFAW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x08T` \x82\x01Qa\x0B\x12\x90c\xFF\xFF\xFF\xFF\x16\x82a<\xA3V[\x15a\x0BMW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01a\n\xF1V[`\0a\x0B]\x82\x84` \x01Qa#1V[\x90P`\0\x83` \x01Qc\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x84Wa\x0B\x84a=\x07V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xADW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x85` \x01Qc\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x0C\xABW`\0a\x0B\xD2\x82\x87a;qV[`\0\x81\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91R\x91\x92P\x90a\x0C\x1FW`@Qc\xD3H/{`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x01Q\x85\x84\x81Q\x81\x10a\x0C6Wa\x0C6a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x84a\x0CR\x91\x90a;qV[`\0\x83\x81R`\x06` R`@\x80\x82 \x82\x81U`\x01\x01\x82\x90U\x83Q\x90Q\x92\x96P\x91\x7F\xFEk\t{F\xA7\x8E\x08Pj1C\xB63|%\x05\xBAw\xDFv\xFE\x05\xC3f:\x98s\x95\xD64\x13\x91\x90\xA2PP\x80\x80a\x0C\xA3\x90a<dV[\x91PPa\x0B\xB4V[P\x84` \x01Qc\xFF\xFF\xFF\xFF\x16`\x07`\0\x82\x82Ta\x0C\xC8\x91\x90a;\xFCV[\x92PP\x81\x90UP`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x83`@Q` \x01a\r\x03\x91\x90a9&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\r&\x91\x90a<\xA3V[\x90P\x80\x86``\x01Q\x14a\r|W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01R\x7FleafHash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R`\0\x91` \x82\x01`\x80\x806\x837\x01\x90PP\x90P`\x0BT\x81`\0\x81Q\x81\x10a\r\xB5Wa\r\xB5a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x86`@\x01Q\x81`\x01\x81Q\x81\x10a\r\xD9Wa\r\xD9a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x81\x81`\x02\x81Q\x81\x10a\r\xF9Wa\r\xF9a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x84\x81`\x03\x81Q\x81\x10a\x0E\x19Wa\x0E\x19a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R\x87\x81\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x03\x90\x91R`@\x80\x82 T\x89Q\x91Qc\xC9AvG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC9AvG\x91a\x0Eq\x91\x90\x86\x90`\x04\x01a9\xFAV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x8BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xC3\x91\x90a4\x05V[\x90P\x80a\x0F\x13W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01R\x7Fproof\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[\x87` \x01Qc\xFF\xFF\xFF\xFF\x16`\x08`\0\x82\x82Ta\x0F/\x91\x90a;qV[\x90\x91UPP`@\x80\x89\x01Q`\x0B\x81\x90U`\0\x90\x81R`\n` R \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0F^\x84a$.V[PPPPPPPPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\x93W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xBE\x18\x08\x91~\t\x81O\xCA}\x80(\x9A\x97`\xA4\x06_\x10\xD6$\xAC\x1A\xEE\x0B\x9F\x90<O]\x92\xB1\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\0`\x07T`\x08Ta\x10\x07\x91\x90a;qV[\x90P\x90V[`@\x80Q`\x05\x80\x82R`\xC0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01`\xA0\x806\x837\x01\x90PP\x90P`\0[`\x05\x81\x10\x15a\x10\x83W`\x11\x81`\x05\x81\x10a\x10RWa\x10Ra<\xF1V[\x01T\x82\x82\x81Q\x81\x10a\x10fWa\x10fa<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x10{\x81a<dV[\x91PPa\x106V[P\x91\x90PV[`\x02`\0T\x14\x15a\x10\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n\xF1V[`\x02`\0\x90\x81U`@\x83\x01QQa\x10\xF2\x90a$\xC6V[\x90P`\0a\x11\x04\x84`\xE0\x01QQa$\xC6V[c\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R T\x90\x91P`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x11\x81W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7Fi/o length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[\x81c\xFF\xFF\xFF\xFF\x16\x84``\x01QQ\x14a\x11\xDCW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FsigHashes length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[\x80c\xFF\xFF\xFF\xFF\x16\x84a\x01\0\x01QQ\x14a\x128W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FoutRollupFees length\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[\x80c\xFF\xFF\xFF\xFF\x16\x84a\x01`\x01QQ\x14a\x12\x94W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FoutEncryptedNotes length\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81c\xFF\xFF\xFF\xFF\x16`\x07T`\x08Ta\x12\xCC\x91\x90a;qV[a\x12\xD6\x91\x90a;qV[\x11\x15a\x12\xF5W`@Qc;\\\xCBC`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xFE2a%FV[\x15a\x13\x1CW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13*\x84a\x01 \x01Qa%FV[\x15a\x13HW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13Y`\x05c\xFF\xFF\xFF\xFF\x84\x16a;\xB1V[\x84a\x01\xA0\x01QQ\x14a\x13~W`@Qc:\xCFH\xBB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13\x89\x85a%\xE5V[\x90Pa\x13\x95\x81\x85a'qV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x1B\x85`\x80\x01Q\x14a\x13\xF4W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7Fsignature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\0a\x14\x01\x84`\x02a;\xD0V[c\xFF\xFF\xFF\xFF\x16\x90P`\0a\x14\x16\x85`\x02a;\xD0V[a\x14!\x90`\x04a;\x89V[c\xFF\xFF\xFF\xFF\x16\x90P`\0a\x146\x85`\x02a;\xD0V[a\x14F\x90c\xFF\xFF\xFF\xFF\x16\x83a;qV[\x90P`\0`\x05a\x14W\x88`\x02a;\x89V[c\xFF\xFF\xFF\xFF\x16a\x14g\x91\x90a;\xB1V[a\x14r\x83`\x02a;qV[a\x14|\x91\x90a;qV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\x94Wa\x14\x94a=\x07V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xBDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x80\x8B\x01Q`\0\x90\x81R`\n\x90\x91R`@\x90 T\x90\x91P`\xFF\x16a\x15\x0FW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R`\x04\x01a\n\xF1\x90` \x80\x82R`\x04\x90\x82\x01Rc\x1C\x9B\xDB\xDD`\xE2\x1B`@\x82\x01R``\x01\x90V[\x88` \x01Q\x81`\0\x81Q\x81\x10a\x15'Wa\x15'a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0a\x15?\x88`\x01a;\x89V[c\xFF\xFF\xFF\xFF\x16\x90P`\0[\x88c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x16KW`\0\x8B`@\x01Q\x82\x81Q\x81\x10a\x15pWa\x15pa<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\0\x81\x81R`\x05\x90\x92R`@\x90\x91 T\x90\x91P`\xFF\x16\x15a\x15\xC9W`@Q\x7F\xFFUn \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x84a\x15\xD6\x84`\x01a;qV[\x81Q\x81\x10a\x15\xE6Wa\x15\xE6a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x8B``\x01Q\x82\x81Q\x81\x10a\x16\x08Wa\x16\x08a<\xF1V[` \x02` \x01\x01Q\x84\x84\x84a\x16\x1D\x91\x90a;qV[\x81Q\x81\x10a\x16-Wa\x16-a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\x16C\x81a<dV[\x91PPa\x15JV[P`\x80\x8A\x01Q\x82a\x16]\x87`\x01a;qV[\x81Q\x81\x10a\x16mWa\x16ma<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xA0\x8A\x01Q\x82a\x16\x89\x87`\x02a;qV[\x81Q\x81\x10a\x16\x99Wa\x16\x99a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xC0\x8A\x01Q\x82a\x16\xB5\x87`\x03a;qV[\x81Q\x81\x10a\x16\xC5Wa\x16\xC5a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0a\x16\xE2c\xFF\xFF\xFF\xFF\x89\x16\x86a;qV[\x90P`\0[\x88c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x18\\W`\x04`\0\x8D`\xE0\x01Q\x83\x81Q\x81\x10a\x17\x0FWa\x17\x0Fa<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15a\x17{W`@Qc\xBE\xE3a\x11`\xE0\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7Fcommitment\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\rT\x8Ca\x01\0\x01Q\x82\x81Q\x81\x10a\x17\x95Wa\x17\x95a<\xF1V[` \x02` \x01\x01Q\x10\x15a\x17\xBCW`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x8B`\xE0\x01Q\x81\x81Q\x81\x10a\x17\xD2Wa\x17\xD2a<\xF1V[` \x02` \x01\x01Q\x84\x87\x83a\x17\xE7\x91\x90a;qV[\x81Q\x81\x10a\x17\xF7Wa\x17\xF7a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01\0\x01Q\x81\x81Q\x81\x10a\x18\x1AWa\x18\x1Aa<\xF1V[` \x02` \x01\x01Q\x84\x83\x83a\x18/\x91\x90a;qV[\x81Q\x81\x10a\x18?Wa\x18?a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x18T\x81a<dV[\x91PPa\x16\xE7V[Pa\x18h\x8B\x84\x86a'\x95V[c\xFF\xFF\xFF\xFF\x80\x8A\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x8C\x16\x83R\x92\x90R\x81\x81 T\x8DQ\x92Qc\xC9AvG`\xE0\x1B\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC9AvG\x91a\x18\xBD\x91\x88\x90`\x04\x01a9\xFAV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xD7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x0F\x91\x90a4\x05V[\x90P\x80a\x19_W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Ftransact proof\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\0[\x8Ac\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x1A3W`\x01`\x05`\0\x8F`@\x01Q\x84\x81Q\x81\x10a\x19\x8CWa\x19\x8Ca<\xF1V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\t`\0\x82\x82Ta\x19\xCC\x91\x90a;qV[\x90\x91UPP`@\x8D\x01Q\x80Q\x82\x90\x81\x10a\x19\xE8Wa\x19\xE8a<\xF1V[` \x02` \x01\x01Q\x8D` \x01Q\x7F<#r\xABa0\x81{\xD6\xB8\xFCm\xBA\xEC\xAE\x94~\x84 \x1BIS]5\x8D\xEB\xAAl4\xC2>\xCF`@Q`@Q\x80\x91\x03\x90\xA3\x80a\x1A+\x81a<dV[\x91PPa\x19bV[P`\0[\x89c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x1B\x04W`\x01`\x04`\0\x8F`\xE0\x01Q\x84\x81Q\x81\x10a\x1AaWa\x1Aaa<\xF1V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x1A\xF2\x8D`\xE0\x01Q\x82\x81Q\x81\x10a\x1A\xA7Wa\x1A\xA7a<\xF1V[` \x02` \x01\x01Q\x8Ea\x01\0\x01Q\x83\x81Q\x81\x10a\x1A\xC6Wa\x1A\xC6a<\xF1V[` \x02` \x01\x01Q\x8Fa\x01`\x01Q\x84\x81Q\x81\x10a\x1A\xE5Wa\x1A\xE5a<\xF1V[` \x02` \x01\x01Qa)\x8CV[\x80a\x1A\xFC\x81a<dV[\x91PPa\x1A7V[P`\xA0\x8C\x01Q\x15a\x1B\"Wa\x1B\"\x8Ca\x01 \x01Q\x8D`\xA0\x01Qa*+V[`\xC0\x8C\x01Q\x15a\x1B?Wa\x1B?\x8Ca\x01@\x01Q\x8D`\xC0\x01Qa*+V[a\x1BH\x8Ca*\xD3V[PP`\x01`\0UPPPPPPPPPPV[3`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x1B\x8BW`@Qc\xF7^\x9F\xC7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\rT\x82``\x01Q\x10\x15a\x1B\xB2W`@QcxO\x02\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x07T`\x08Ta\x1B\xE3\x91\x90a;qV[\x10a\x1C\x01W`@Qc;\\\xCBC`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x83\x01Q`\0\x90\x81R`\x04\x90\x91R`@\x90 T`\xFF\x16\x15a\x1C7W`@Qc\xE3\x8C\xD1M`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x83\x01\x80Q`\0\x90\x81R`\x04\x90\x92R`@\x90\x91 \x80T`\xFF\x19\x16`\x01\x17\x90UQ``\x83\x01Q`\x80\x84\x01Qa\x1Cn\x92\x91\x90a)\x8CV[`@\x82\x01Q\x15a\x1C\x86Wa\x1C\x86\x81\x83`@\x01Qa,\xE8V[PPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\xB5W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x1D\x03W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7F_minRollupFee\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\rUV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1D3W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T`\xFF\x16\x15a\x1DWW`@Qc6\xE3\xE0\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x83\x16a\x1D{W`@Qc\x9F{\xD9K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R`\x01` \x80\x83\x01\x91\x82Rc\xFF\xFF\xFF\xFF\x96\x87\x16`\0\x90\x81R`\x02\x82R\x84\x81 \x96\x90\x97\x16\x87R\x94\x90\x94R\x93 \x92Q\x83T\x92Q\x15\x15`\x01`\xA0\x1B\x02t\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x16\x17\x17\x90UV[```\0`\x07Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\nWa\x1E\na=\x07V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\x07T\x81\x10\x15a\x10\x83W`\0\x81`\x08Ta\x1ES\x91\x90a;qV[`\0\x81\x81R`\x06` R`@\x90 T\x84Q\x91\x92P\x90\x84\x90\x84\x90\x81\x10a\x1EzWa\x1Eza<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\x1E\x90\x81a<dV[\x91PPa\x1E9V[`\0`\x05\x82\x10a\x1E\xBBW`@Qc1\x8CCE`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x11\x82`\x05\x81\x10a\x1E\xCEWa\x1E\xCEa<\xF1V[\x01T\x92\x91PPV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1F\x01W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T`\xFF\x16\x15a\x1F%W`@Qc6\xE3\xE0\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x81\x16\x15\x80a\x1F?WPa\x04\0\x81c\xFF\xFF\xFF\xFF\x16\x11[\x15a\x1FzW`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01a\n\xF1V[a\x1F\x85`\x01\x82a<\x13V[\x81\x16c\xFF\xFF\xFF\xFF\x16`\0\x14a\x1F\xADW`@Qc\"q\x7F\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\xFF`\xA0\x1B\x19\x16\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1F\xF9W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0E` R`@\x90 \x80T`\xFF\x19\x16\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a EW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0F` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a \x94W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xCFT\xE4\x9C\xBF\x0F\xD7a?\xB6\xB5\xB0\x9F.\x8C\x12U\xA9\xDF\x94\xA8&9\x0Fx=5\xCF+M\xA7\xE4\x90` \x01a\x0F\xEAV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a!\0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T`\xFF\x16\x15a!$W`@Qc6\xE3\xE0\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x82\x16a!HW`@Qc\x9F{\xD9K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x91\x82\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R \x80T`\xFF`\xA0\x1B\x19\x16\x90UV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a!\xA4W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\xFF`\xA0\x1B\x19\x16\x90\x81\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x91a\x07\xE0\x91`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\x1CW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T`\xFF\x16\x15a\"@W`@Qc6\xE3\xE0\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x82\x16\x15\x80a\"ZWPa\x04\0\x82c\xFF\xFF\xFF\xFF\x16\x11[\x15a\"\x95W`@Qc\x14\xE8\x95[`\xE2\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RirollupSize`\xB0\x1B`D\x82\x01R`d\x01a\n\xF1V[a\"\xA0`\x01\x83a<\x13V[\x82\x16c\xFF\xFF\xFF\xFF\x16`\0\x14a\"\xC8W`@Qc\"q\x7F\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R`\x01` \x80\x83\x01\x91\x82Rc\xFF\xFF\xFF\xFF\x90\x95\x16`\0\x90\x81R`\x03\x90\x95R\x91\x90\x93 \x92Q\x83T\x91Q\x15\x15`\x01`\xA0\x1B\x02t\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x92\x16\x91\x90\x91\x17\x17\x90UV[`\0a\x01\0\x82c\xFF\xFF\xFF\xFF\x16\x10a#WW`\x08\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x08\x83\x90\x1C\x92P[`\x10\x82c\xFF\xFF\xFF\xFF\x16\x10a#zW`\x04\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x04\x83\x90\x1C\x92P[`\x04\x82c\xFF\xFF\xFF\xFF\x16\x10a#\x9DW`\x02\x82c\xFF\xFF\xFF\xFF\x16\x90\x1C\x91P`\x02\x83\x90\x1C\x92P[`\x02\x82c\xFF\xFF\xFF\xFF\x16\x10a#\xB3W`\x01\x83\x90\x1C\x92P[P\x90\x91\x90PV[`\x0CT`\x01`\x01`\xA0\x1B\x03\x163\x14a#\xE5W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10\x80T\x82\x15\x15a\x01\0\x02a\xFF\0\x19\x90\x91\x16\x17\x90U`@Q\x7F\x9A$\xEEpHT\xE0\xC51\xDA\x82\x1C\x99`E@i\x0F*\xF5W},\x89,)!\xCE\xA4\t\xB4y\x90a\x0F\xEA\x90\x83\x15\x15\x81R` \x01\x90V[`@Q`\0\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a$pW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a$uV[``\x91P[PP\x90P\x80a\x1C\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Frollup fee transfer failed\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a%BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01R\x7F2 bits\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xF1V[P\x90V[`\x01T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a%bWP`\0\x91\x90PV[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a%\xA7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a%\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xDF\x91\x90a4\x05V[\x92\x91PPV[a\x01`\x81\x01QQ`\0\x90`\x03\x81\x10a&\x10W`@Qc?\xB1\x94]`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81a&dWa\x01 \x84\x01Qa\x01@\x85\x01Q`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x81\x16` \x83\x01R\x91\x90\x92\x1B\x16`4\x82\x01R`H\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa'\x1CV[\x81`\x01\x14\x15a&\xABW\x83a\x01 \x01Q\x84a\x01@\x01Q\x85a\x01`\x01Q`\0\x81Q\x81\x10a&\x91Wa&\x91a<\xF1V[` \x02` \x01\x01Q`@Q` \x01a&N\x93\x92\x91\x90a8\x84V[\x83a\x01 \x01Q\x84a\x01@\x01Q\x85a\x01`\x01Q`\0\x81Q\x81\x10a&\xCFWa&\xCFa<\xF1V[` \x02` \x01\x01Q\x86a\x01`\x01Q`\x01\x81Q\x81\x10a&\xEFWa&\xEFa<\xF1V[` \x02` \x01\x01Q`@Q` \x01a'\n\x94\x93\x92\x91\x90a8\xCAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[\x80Q` \x91\x82\x01 `@\x80Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81\x85\x01R`<\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\\\x01\x90R\x80Q\x91\x01 \x93\x92PPPV[`\0\x80`\0a'\x80\x85\x85a-\x8BV[\x91P\x91Pa'\x8D\x81a-\xFBV[P\x93\x92PPPV[a\x01\x80\x83\x01Q`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\xFF\x83\x90\x1C\x80\x83R`\x01`\x01`\xFF\x1B\x03\x90\x93\x16\x90\x82\x01R\x83Q\x90\x91\x90\x84\x90\x84\x90\x81\x10a'\xE5Wa'\xE5a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x83\x83`\x01a(\x04\x91\x90a;qV[\x81Q\x81\x10a(\x14Wa(\x14a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0a(,\x83`\x02a;qV[\x90P`\0a(;`\x05\x83a;qV[\x90P`\0a(K`\x05`\x02a;\xB1V[a(U\x90\x84a;qV[\x90P`\0[`\x05\x81\x10\x15a)!W`\0a(\xB5`\x11\x83`\x05\x81\x10a({Wa({a<\xF1V[\x01T`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\xFF\x83\x90\x1C\x82R`\x01`\x01`\xFF\x1B\x03\x90\x92\x16\x91\x81\x01\x91\x90\x91R\x90V[\x80Q\x90\x91P\x88a(\xC5\x84\x88a;qV[\x81Q\x81\x10a(\xD5Wa(\xD5a<\xF1V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q\x88\x83\x86a(\xF3\x91\x90a;qV[\x81Q\x81\x10a)\x03Wa)\x03a<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a)\x19\x81a<dV[\x91PPa(ZV[P`\0[\x87a\x01\xA0\x01QQ\x81\x10\x15a\x0F^W\x87a\x01\xA0\x01Q\x81\x81Q\x81\x10a)JWa)Ja<\xF1V[` \x02` \x01\x01Q\x87\x82\x84a)_\x91\x90a;qV[\x81Q\x81\x10a)oWa)oa<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a)\x84\x81a<dV[\x91PPa)%V[`\0`\x08T`\x07Ta)\x9E\x91\x90a;qV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R`\0\x85\x81R`\x06\x90\x92R\x92\x81 \x91Q\x82U\x91Q`\x01\x91\x82\x01U`\x07\x80T\x93\x94P\x90\x92\x90\x91\x90a)\xE2\x90\x84\x90a;qV[\x92PP\x81\x90UP\x83\x7F\xF53\xF9pZ\xACP \xE2\x16\x95\xEA5S\xAC{h\x81\x07\r+i\0\xAB+\x1E0P0K[\xF9\x84\x83\x85`@Qa*\x1D\x93\x92\x91\x90a:kV[`@Q\x80\x91\x03\x90\xA2PPPPV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a*xW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a*}V[``\x91P[PP\x90P\x80a*\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7Fwithdraw failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[PPPV[`\0`\x05\x82`@\x01QQa*\xE7\x91\x90a;\xB1V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x04Wa+\x04a=\x07V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+cW\x81` \x01[a+P`@Q\x80``\x01`@R\x80`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a+\"W\x90P[P\x90P`\0\x80[\x84`@\x01QQ\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a,\xAAW`\0[`\x05\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a,\x97W\x80c\xFF\xFF\xFF\xFF\x16` \x83c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B\x17\x84\x84\x81Q\x81\x10a+\xBEWa+\xBEa<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90R`\x11c\xFF\xFF\xFF\xFF\x82\x16`\x05\x81\x10a+\xF0Wa+\xF0a<\xF1V[\x01T\x84\x84\x81Q\x81\x10a,\x04Wa,\x04a<\xF1V[` \x02` \x01\x01Q` \x01\x81\x81RPP\x85a\x01\xA0\x01Q\x81c\xFF\xFF\xFF\xFF\x16`\x05\x84c\xFF\xFF\xFF\xFF\x16a,4\x91\x90a;\xB1V[a,>\x91\x90a;qV[\x81Q\x81\x10a,NWa,Na<\xF1V[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a,hWa,ha<\xF1V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@\x01R\x82a,\x81\x81a<dV[\x93PP\x80\x80a,\x8F\x90a<\x7FV[\x91PPa+\x80V[P\x80a,\xA2\x81a<\x7FV[\x91PPa+jV[P\x7Fj\xF0~\xBC\xB391\xAB\xD3H\xE7\x85\xF7\x0Cm%\x93\x90\xCFy\x91\x91\xF8\xC9\x91$$[\x96\xE1\xA6\x13\x82`@Qa,\xDA\x91\x90a9\\V[`@Q\x80\x91\x03\x90\xA1PPPPV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a-5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a-:V[``\x91P[PP\x90P\x80a*\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fexecutor fee transfer failed\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\0\x80\x82Q`A\x14\x15a-\xC2W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa-\xB6\x87\x82\x85\x85a/\xB9V[\x94P\x94PPPPa-\xF4V[\x82Q`@\x14\x15a-\xECW` \x83\x01Q`@\x84\x01Qa-\xE1\x86\x83\x83a0\xA6V[\x93P\x93PPPa-\xF4V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a.\x0FWa.\x0Fa<\xDBV[\x14\x15a.\x18WPV[`\x01\x81`\x04\x81\x11\x15a.,Wa.,a<\xDBV[\x14\x15a.zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xF1V[`\x02\x81`\x04\x81\x11\x15a.\x8EWa.\x8Ea<\xDBV[\x14\x15a.\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\n\xF1V[`\x03\x81`\x04\x81\x11\x15a.\xF0Wa.\xF0a<\xDBV[\x14\x15a/IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xF1V[`\x04\x81`\x04\x81\x11\x15a/]Wa/]a<\xDBV[\x14\x15a/\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xF1V[PV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a/\xF0WP`\0\x90P`\x03a0\x9DV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a0\x08WP\x84`\xFF\x16`\x1C\x14\x15[\x15a0\x19WP`\0\x90P`\x04a0\x9DV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a0mW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a0\x96W`\0`\x01\x92P\x92PPa0\x9DV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a0\xC3`\xFF\x86\x90\x1C`\x1Ba;qV[\x90Pa0\xD1\x87\x82\x88\x85a/\xB9V[\x93P\x93PPP\x93P\x93\x91PPV[\x805a0\xEA\x81a=\x1DV[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a1\0W`\0\x80\xFD[\x815` a1\x15a1\x10\x83a;MV[a;\x1CV[\x80\x83\x82R\x82\x82\x01\x91P\x82\x86\x01\x87\x84\x86`\x05\x1B\x89\x01\x01\x11\x15a15W`\0\x80\xFD[`\0\x80[\x86\x81\x10\x15a1xW\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1WW\x82\x83\xFD[a1e\x8B\x88\x83\x8D\x01\x01a2KV[\x86RP\x93\x85\x01\x93\x91\x85\x01\x91`\x01\x01a19V[P\x91\x98\x97PPPPPPPPV[`\0\x82`\x1F\x83\x01\x12a1\x97W`\0\x80\xFD[a1\x9Fa:\xACV[\x80\x83\x85`@\x86\x01\x11\x15a1\xB1W`\0\x80\xFD[`\0[`\x02\x81\x10\x15a1\xD3W\x815\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a1\xB4V[P\x90\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a1\xEFW`\0\x80\xFD[\x815` a1\xFFa1\x10\x83a;MV[\x80\x83\x82R\x82\x82\x01\x91P\x82\x86\x01\x87\x84\x86`\x05\x1B\x89\x01\x01\x11\x15a2\x1FW`\0\x80\xFD[`\0[\x85\x81\x10\x15a2>W\x815\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a2\"V[P\x90\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12a2\\W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2vWa2va=\x07V[a2\x89`\x1F\x82\x01`\x1F\x19\x16` \x01a;\x1CV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a2\x9EW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a2\xCDW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a2\xF0Wa2\xF0a=\x07V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x81\x83\x03a\x01\0\x81\x12\x15a3\x1EW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a3AWa3Aa=\x07V[`@R\x91P\x81a3Q\x85\x85a2\xBBV[\x81R`\x80`?\x19\x83\x01\x12\x15a3eW`\0\x80\xFD[a3ma:\xACV[\x91Pa3|\x85`@\x86\x01a1\x86V[\x82Ra3\x8B\x85`\x80\x86\x01a1\x86V[` \x83\x01R\x81` \x82\x01Ra3\xA3\x85`\xC0\x86\x01a2\xBBV[`@\x82\x01RPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a0\xEAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a3\xD6W`\0\x80\xFD[\x815a3\xE1\x81a=\x1DV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a3\xFAW`\0\x80\xFD[\x815a3\xE1\x81a=2V[`\0` \x82\x84\x03\x12\x15a4\x17W`\0\x80\xFD[\x81Qa3\xE1\x81a=2V[`\0\x80`@\x83\x85\x03\x12\x15a45W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a4MW`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a4aW`\0\x80\xFD[a4ia:\xD5V[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a4\x9DW`\0\x80\xFD[a4\xA9\x88\x82\x86\x01a2KV[`\x80\x83\x01RP\x93Pa4\xC0\x91PP` \x84\x01a0\xDFV[\x90P\x92P\x92\x90PV[`\0a\x01`\x82\x84\x03\x12\x15a4\xDCW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\xFFWa4\xFFa=\x07V[`@Ra5\x0C\x84\x84a3\nV[\x81Ra5\x1Ba\x01\0\x84\x01a3\xB0V[` \x82\x01Ra\x01 \x83\x015`@\x82\x01Ra\x01@\x90\x92\x015``\x83\x01RP\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a5PW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a5hW`\0\x80\xFD[\x90\x84\x01\x90a\x02\xA0\x82\x87\x03\x12\x15a5}W`\0\x80\xFD[a5\x85a:\xF8V[a5\x8F\x87\x84a3\nV[\x81Ra\x01\0\x80\x84\x015` \x83\x01Ra\x01 \x80\x85\x015\x84\x81\x11\x15a5\xB1W`\0\x80\xFD[a5\xBD\x8A\x82\x88\x01a1\xDEV[`@\x85\x01RPa\x01@\x80\x86\x015\x85\x81\x11\x15a5\xD7W`\0\x80\xFD[a5\xE3\x8B\x82\x89\x01a1\xDEV[``\x86\x01RPa\x01`\x80\x87\x015`\x80\x86\x01Ra\x01\x80\x80\x88\x015`\xA0\x87\x01Ra\x01\xA0\x80\x89\x015`\xC0\x88\x01Ra\x01\xC0\x89\x015\x88\x81\x11\x15a6 W`\0\x80\xFD[a6,\x8E\x82\x8C\x01a1\xDEV[`\xE0\x89\x01RPa\x01\xE0\x89\x015\x88\x81\x11\x15a6EW`\0\x80\xFD[a6Q\x8E\x82\x8C\x01a1\xDEV[\x87\x89\x01RPa6ca\x02\0\x8A\x01a0\xDFV[\x85\x88\x01Ra6ta\x02 \x8A\x01a0\xDFV[\x84\x88\x01Ra\x02@\x89\x015\x95P\x87\x86\x11\x15a6\x8DW`\0\x80\xFD[a6\x99\x8D\x87\x8B\x01a0\xEFV[\x83\x88\x01Ra\x02`\x89\x015\x82\x88\x01Ra\x02\x80\x89\x015\x95P\x87\x86\x11\x15a6\xBCW`\0\x80\xFD[a6\xC8\x8D\x87\x8B\x01a1\xDEV[\x90\x87\x01RP\x93\x97PPPP` \x86\x015\x92PP\x80\x82\x11\x15a6\xE8W`\0\x80\xFD[Pa6\xF5\x85\x82\x86\x01a2KV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a7\x11W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a7+W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`@\x83\x85\x03\x12\x15a7MW`\0\x80\xFD[\x825\x91Pa4\xC0` \x84\x01a3\xB0V[`\0` \x82\x84\x03\x12\x15a7oW`\0\x80\xFD[a3\xE1\x82a3\xB0V[`\0\x80`@\x83\x85\x03\x12\x15a7\x8BW`\0\x80\xFD[a7\x94\x83a3\xB0V[\x91P` \x83\x015a7\xA4\x81a=\x1DV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a7\xC2W`\0\x80\xFD[a7\xCB\x83a3\xB0V[\x91Pa4\xC0` \x84\x01a3\xB0V[`\0\x80`\0``\x84\x86\x03\x12\x15a7\xEEW`\0\x80\xFD[a7\xF7\x84a3\xB0V[\x92Pa8\x05` \x85\x01a3\xB0V[\x91P`@\x84\x015a8\x15\x81a=\x1DV[\x80\x91PP\x92P\x92P\x92V[\x80`\0[`\x02\x81\x10\x15a8CW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a8$V[PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a8yW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a8]V[P\x94\x95\x94PPPPPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x86``\x1B\x16\x83R\x80\x85``\x1B\x16`\x14\x84\x01RP\x82Qa8\xBB\x81`(\x85\x01` \x87\x01a<8V[\x91\x90\x91\x01`(\x01\x94\x93PPPPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x87``\x1B\x16\x83R\x80\x86``\x1B\x16`\x14\x84\x01RP\x83Qa9\x01\x81`(\x85\x01` \x88\x01a<8V[\x83Q\x90\x83\x01\x90a9\x18\x81`(\x84\x01` \x88\x01a<8V[\x01`(\x01\x96\x95PPPPPPV[\x81Q`\0\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a9PW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a94V[P\x92\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a9\xB2W\x81Q\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85R\x86\x81\x01Q\x87\x86\x01R\x85\x01Q\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a9yV[P\x91\x97\x96PPPPPPPV[` \x81R`\0a3\xE1` \x83\x01\x84a8IV[` \x81\x01`\x02\x83\x10a9\xF4WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0a\x01 a:\x14\x83\x86Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[` \x85\x01Qa:'`@\x85\x01\x82Qa8 V[` \x01Qa:8`\x80\x85\x01\x82a8 V[P`@\x85\x01Q\x80Q`\xC0\x85\x01R` \x01Q`\xE0\x84\x01Ra\x01\0\x83\x01\x81\x90Ra:b\x81\x84\x01\x85a8IV[\x95\x94PPPPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\0\x82Q\x80``\x84\x01Ra:\x96\x81`\x80\x85\x01` \x87\x01a<8V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`\x80\x01\x94\x93PPPPV[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a:\xCFWa:\xCFa=\x07V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a:\xCFWa:\xCFa=\x07V[`@Qa\x01\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a:\xCFWa:\xCFa=\x07V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;EWa;Ea=\x07V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a;gWa;ga=\x07V[P`\x05\x1B` \x01\x90V[`\0\x82\x19\x82\x11\x15a;\x84Wa;\x84a<\xC5V[P\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a;\xA8Wa;\xA8a<\xC5V[\x01\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a;\xCBWa;\xCBa<\xC5V[P\x02\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a;\xF3Wa;\xF3a<\xC5V[\x02\x94\x93PPPPV[`\0\x82\x82\x10\x15a<\x0EWa<\x0Ea<\xC5V[P\x03\x90V[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a<0Wa<0a<\xC5V[\x03\x93\x92PPPV[`\0[\x83\x81\x10\x15a<SW\x81\x81\x01Q\x83\x82\x01R` \x01a<;V[\x83\x81\x11\x15a8CWPP`\0\x91\x01RV[`\0`\0\x19\x82\x14\x15a<xWa<xa<\xC5V[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a<\x99Wa<\x99a<\xC5V[`\x01\x01\x93\x92PPPV[`\0\x82a<\xC0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a/\xB6W`\0\x80\xFD[\x80\x15\x15\x81\x14a/\xB6W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x85\xC0v_\xA8l\x177R\xA7o\xC4<\x01R^GY\xAE\xF8H\xDF\x91\x85\xB5\x87FTkQ\xC5\x15dsolcC\0\x08\x07\x003";
    /// The deployed bytecode of the contract.
    pub static COMMITMENTPOOLMAIN_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct CommitmentPoolMain<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for CommitmentPoolMain<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CommitmentPoolMain<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CommitmentPoolMain<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CommitmentPoolMain<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CommitmentPoolMain))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> CommitmentPoolMain<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                COMMITMENTPOOLMAIN_ABI.clone(),
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
                COMMITMENTPOOLMAIN_ABI.clone(),
                COMMITMENTPOOLMAIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `addEnqueueWhitelist` (0xa9b1d296) function
        pub fn add_enqueue_whitelist(
            &self,
            actor: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 177, 210, 150], actor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addRollupWhitelist` (0x02d498f1) function
        pub fn add_rollup_whitelist(
            &self,
            roller: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 212, 152, 241], roller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetType` (0x3fe3347a) function
        pub fn asset_type(&self) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([63, 227, 52, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `auditorCount` (0x115f574c) function
        pub fn auditor_count(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([17, 95, 87, 76], ())
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
        ///Calls the contract's `disableRollupVerifier` (0x9b0a6fea) function
        pub fn disable_rollup_verifier(&self, rollup_size: u32) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 10, 111, 234], rollup_size)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableSanctionsCheck` (0xdd757c34) function
        pub fn disable_sanctions_check(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 117, 124, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableTransactVerifier` (0xc259e2e6) function
        pub fn disable_transact_verifier(
            &self,
            num_inputs: u32,
            num_outputs: u32,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 89, 226, 230], (num_inputs, num_outputs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableRollupVerifier` (0xdeeff7cd) function
        pub fn enable_rollup_verifier(
            &self,
            rollup_size: u32,
            rollup_verifier: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 239, 247, 205], (rollup_size, rollup_verifier))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableSanctionsCheck` (0x01dbf19f) function
        pub fn enable_sanctions_check(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 219, 241, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableTransactVerifier` (0x7fa4b09c) function
        pub fn enable_transact_verifier(
            &self,
            num_inputs: u32,
            num_outputs: u32,
            transact_verifier: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 164, 176, 156], (num_inputs, num_outputs, transact_verifier))
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
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::vec::Vec<::ethers_core::types::U256>> {
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
        pub fn get_commitment_count(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
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
        pub fn get_min_rollup_fee(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([176, 136, 146, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNullifierCount` (0x7a553744) function
        pub fn get_nullifier_count(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([122, 85, 55, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQueuedCommitments` (0x866ac658) function
        pub fn get_queued_commitments(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::vec::Vec<::ethers_core::types::U256>> {
            self.0
                .method_hash([134, 106, 198, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTreeCapacity` (0x484eb652) function
        pub fn get_tree_capacity(&self) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
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
        ///Calls the contract's `isRollupWhitelistDisabled` (0xffa89b88) function
        pub fn is_rollup_whitelist_disabled(&self) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([255, 168, 155, 136], ())
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
        ///Calls the contract's `isVerifierUpdateDisabled` (0x4eb069f7) function
        pub fn is_verifier_update_disabled(&self) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([78, 176, 105, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeEnqueueWhitelist` (0x03db9874) function
        pub fn remove_enqueue_whitelist(
            &self,
            actor: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 219, 152, 116], actor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeRollupWhitelist` (0x9cc6b354) function
        pub fn remove_rollup_whitelist(
            &self,
            roller: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 198, 179, 84], roller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollup` (0x14a7737d) function
        pub fn rollup(&self, request: RollupRequest) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 167, 115, 125], (request,))
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
        ///Calls the contract's `setMinRollupFee` (0x7cbf0ff6) function
        pub fn set_min_rollup_fee(
            &self,
            min_rollup_fee: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 191, 15, 246], min_rollup_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRollupWhitelistDisabled` (0xf8f05388) function
        pub fn set_rollup_whitelist_disabled(&self, state: bool) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 240, 83, 136], state)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setVerifierUpdateDisabled` (0xb3b75631) function
        pub fn set_verifier_update_disabled(&self, state: bool) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 183, 86, 49], state)
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
        ///Calls the contract's `updateAuditorPublicKey` (0x0c8867e6) function
        pub fn update_auditor_public_key(
            &self,
            index: ::ethers_core::types::U256,
            public_key: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 136, 103, 230], (index, public_key))
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
        ///Gets the contract's `AuditorPublicKey` event
        pub fn auditor_public_key_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, AuditorPublicKeyFilter> {
            self.0.event()
        }
        ///Gets the contract's `CommitmentIncluded` event
        pub fn commitment_included_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, CommitmentIncludedFilter> {
            self.0.event()
        }
        ///Gets the contract's `CommitmentQueued` event
        pub fn commitment_queued_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, CommitmentQueuedFilter> {
            self.0.event()
        }
        ///Gets the contract's `CommitmentSpent` event
        pub fn commitment_spent_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, CommitmentSpentFilter> {
            self.0.event()
        }
        ///Gets the contract's `EncryptedAuditorNote` event
        pub fn encrypted_auditor_note_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, EncryptedAuditorNoteFilter> {
            self.0.event()
        }
        ///Gets the contract's `EncryptedAuditorNotes` event
        pub fn encrypted_auditor_notes_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, EncryptedAuditorNotesFilter> {
            self.0.event()
        }
        ///Gets the contract's `OperatorChanged` event
        pub fn operator_changed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, OperatorChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RollupWhitelistDisabled` event
        pub fn rollup_whitelist_disabled_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, RollupWhitelistDisabledFilter> {
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
        ///Gets the contract's `VerifierUpdateDisabled` event
        pub fn verifier_update_disabled_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, VerifierUpdateDisabledFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(&self) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, CommitmentPoolMainEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for CommitmentPoolMain<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AuditorIndexError` with signature `AuditorIndexError()` and selector `0xc6310d14`
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
    #[etherror(name = "AuditorIndexError", abi = "AuditorIndexError()")]
    pub struct AuditorIndexError;
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
        Hash,
    )]
    #[etherror(name = "AuditorNotesLengthError", abi = "AuditorNotesLengthError()")]
    pub struct AuditorNotesLengthError;
    ///Custom Error type `AuditorPublicKeyNotChanged` with signature `AuditorPublicKeyNotChanged()` and selector `0x02538338`
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
    #[etherror(name = "AuditorPublicKeyNotChanged", abi = "AuditorPublicKeyNotChanged()")]
    pub struct AuditorPublicKeyNotChanged;
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
        Hash,
    )]
    #[etherror(name = "CommitmentHasBeenSubmitted", abi = "CommitmentHasBeenSubmitted()")]
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
        Hash,
    )]
    #[etherror(name = "Duplicated", abi = "Duplicated(string)")]
    pub struct Duplicated {
        pub param: ::std::string::String,
    }
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[etherror(name = "NewRootIsDuplicated", abi = "NewRootIsDuplicated()")]
    pub struct NewRootIsDuplicated;
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
        Hash,
    )]
    #[etherror(name = "NoteHasBeenSpent", abi = "NoteHasBeenSpent()")]
    pub struct NoteHasBeenSpent;
    ///Custom Error type `NumInputsGreaterThanZero` with signature `NumInputsGreaterThanZero()` and selector `0x9f7bd94b`
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
    #[etherror(name = "NumInputsGreaterThanZero", abi = "NumInputsGreaterThanZero()")]
    pub struct NumInputsGreaterThanZero;
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
    ///Custom Error type `OnlyWhitelistedRoller` with signature `OnlyWhitelistedRoller()` and selector `0x21ce01f3`
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
    #[etherror(name = "OnlyWhitelistedRoller", abi = "OnlyWhitelistedRoller()")]
    pub struct OnlyWhitelistedRoller;
    ///Custom Error type `OnlyWhitelistedSender` with signature `OnlyWhitelistedSender()` and selector `0xf75e9fc7`
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
    #[etherror(name = "OnlyWhitelistedSender", abi = "OnlyWhitelistedSender()")]
    pub struct OnlyWhitelistedSender;
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
        Hash,
    )]
    #[etherror(name = "OutputNotesLessThanThree", abi = "OutputNotesLessThanThree()")]
    pub struct OutputNotesLessThanThree;
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
    ///Custom Error type `RollupSizeNotPowerOfTwo` with signature `RollupSizeNotPowerOfTwo()` and selector `0x22717ff9`
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
    #[etherror(name = "RollupSizeNotPowerOfTwo", abi = "RollupSizeNotPowerOfTwo()")]
    pub struct RollupSizeNotPowerOfTwo;
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[etherror(name = "TreeIsFull", abi = "TreeIsFull()")]
    pub struct TreeIsFull;
    ///Custom Error type `VerifierUpdatesHasBeenDisabled` with signature `VerifierUpdatesHasBeenDisabled()` and selector `0x36e3e095`
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
    #[etherror(name = "VerifierUpdatesHasBeenDisabled", abi = "VerifierUpdatesHasBeenDisabled()")]
    pub struct VerifierUpdatesHasBeenDisabled;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers_contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum CommitmentPoolMainErrors {
        AuditorIndexError(AuditorIndexError),
        AuditorNotesLengthError(AuditorNotesLengthError),
        AuditorPublicKeyNotChanged(AuditorPublicKeyNotChanged),
        CommitmentHasBeenSubmitted(CommitmentHasBeenSubmitted),
        Duplicated(Duplicated),
        IndexOutOfBound(IndexOutOfBound),
        Invalid(Invalid),
        NewRootIsDuplicated(NewRootIsDuplicated),
        NotChanged(NotChanged),
        NoteHasBeenSpent(NoteHasBeenSpent),
        NumInputsGreaterThanZero(NumInputsGreaterThanZero),
        OnlyOperator(OnlyOperator),
        OnlyWhitelistedRoller(OnlyWhitelistedRoller),
        OnlyWhitelistedSender(OnlyWhitelistedSender),
        OutputNotesLessThanThree(OutputNotesLessThanThree),
        RollupFeeToFew(RollupFeeToFew),
        RollupSizeNotPowerOfTwo(RollupSizeNotPowerOfTwo),
        SanctionedAddress(SanctionedAddress),
        TreeHeightLessThanZero(TreeHeightLessThanZero),
        TreeHeightOutOfBounds(TreeHeightOutOfBounds),
        TreeIsFull(TreeIsFull),
        VerifierUpdatesHasBeenDisabled(VerifierUpdatesHasBeenDisabled),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for CommitmentPoolMainErrors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AuditorIndexError as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AuditorIndexError(decoded));
            }
            if let Ok(decoded) = <AuditorNotesLengthError as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AuditorNotesLengthError(decoded));
            }
            if let Ok(decoded) = <AuditorPublicKeyNotChanged as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AuditorPublicKeyNotChanged(decoded));
            }
            if let Ok(decoded) = <CommitmentHasBeenSubmitted as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CommitmentHasBeenSubmitted(decoded));
            }
            if let Ok(decoded) = <Duplicated as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Duplicated(decoded));
            }
            if let Ok(decoded) = <IndexOutOfBound as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IndexOutOfBound(decoded));
            }
            if let Ok(decoded) = <Invalid as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Invalid(decoded));
            }
            if let Ok(decoded) = <NewRootIsDuplicated as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NewRootIsDuplicated(decoded));
            }
            if let Ok(decoded) = <NotChanged as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotChanged(decoded));
            }
            if let Ok(decoded) = <NoteHasBeenSpent as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoteHasBeenSpent(decoded));
            }
            if let Ok(decoded) = <NumInputsGreaterThanZero as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NumInputsGreaterThanZero(decoded));
            }
            if let Ok(decoded) = <OnlyOperator as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyOperator(decoded));
            }
            if let Ok(decoded) = <OnlyWhitelistedRoller as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyWhitelistedRoller(decoded));
            }
            if let Ok(decoded) = <OnlyWhitelistedSender as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyWhitelistedSender(decoded));
            }
            if let Ok(decoded) = <OutputNotesLessThanThree as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutputNotesLessThanThree(decoded));
            }
            if let Ok(decoded) = <RollupFeeToFew as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RollupFeeToFew(decoded));
            }
            if let Ok(decoded) = <RollupSizeNotPowerOfTwo as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RollupSizeNotPowerOfTwo(decoded));
            }
            if let Ok(decoded) = <SanctionedAddress as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionedAddress(decoded));
            }
            if let Ok(decoded) = <TreeHeightLessThanZero as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TreeHeightLessThanZero(decoded));
            }
            if let Ok(decoded) = <TreeHeightOutOfBounds as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TreeHeightOutOfBounds(decoded));
            }
            if let Ok(decoded) = <TreeIsFull as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TreeIsFull(decoded));
            }
            if let Ok(decoded) = <VerifierUpdatesHasBeenDisabled as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VerifierUpdatesHasBeenDisabled(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for CommitmentPoolMainErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AuditorIndexError(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AuditorNotesLengthError(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AuditorPublicKeyNotChanged(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CommitmentHasBeenSubmitted(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Duplicated(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::IndexOutOfBound(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Invalid(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NewRootIsDuplicated(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NotChanged(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NoteHasBeenSpent(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NumInputsGreaterThanZero(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnlyOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnlyWhitelistedRoller(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnlyWhitelistedSender(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OutputNotesLessThanThree(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RollupFeeToFew(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RollupSizeNotPowerOfTwo(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionedAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::TreeHeightLessThanZero(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::TreeHeightOutOfBounds(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::TreeIsFull(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::VerifierUpdatesHasBeenDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for CommitmentPoolMainErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AuditorIndexError as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <AuditorNotesLengthError as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <AuditorPublicKeyNotChanged as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <CommitmentHasBeenSubmitted as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <Duplicated as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <IndexOutOfBound as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <Invalid as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <NewRootIsDuplicated as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <NotChanged as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <NoteHasBeenSpent as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <NumInputsGreaterThanZero as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OnlyOperator as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OnlyWhitelistedRoller as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OnlyWhitelistedSender as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OutputNotesLessThanThree as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <RollupFeeToFew as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <RollupSizeNotPowerOfTwo as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <SanctionedAddress as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <TreeHeightLessThanZero as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <TreeHeightOutOfBounds as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <TreeIsFull as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <VerifierUpdatesHasBeenDisabled as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CommitmentPoolMainErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AuditorIndexError(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuditorNotesLengthError(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuditorPublicKeyNotChanged(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentHasBeenSubmitted(element) => ::core::fmt::Display::fmt(element, f),
                Self::Duplicated(element) => ::core::fmt::Display::fmt(element, f),
                Self::IndexOutOfBound(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewRootIsDuplicated(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotChanged(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoteHasBeenSpent(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumInputsGreaterThanZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyWhitelistedRoller(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyWhitelistedSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutputNotesLessThanThree(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupFeeToFew(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupSizeNotPowerOfTwo(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionedAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::TreeHeightLessThanZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::TreeHeightOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::TreeIsFull(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifierUpdatesHasBeenDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CommitmentPoolMainErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AuditorIndexError> for CommitmentPoolMainErrors {
        fn from(value: AuditorIndexError) -> Self {
            Self::AuditorIndexError(value)
        }
    }
    impl ::core::convert::From<AuditorNotesLengthError> for CommitmentPoolMainErrors {
        fn from(value: AuditorNotesLengthError) -> Self {
            Self::AuditorNotesLengthError(value)
        }
    }
    impl ::core::convert::From<AuditorPublicKeyNotChanged> for CommitmentPoolMainErrors {
        fn from(value: AuditorPublicKeyNotChanged) -> Self {
            Self::AuditorPublicKeyNotChanged(value)
        }
    }
    impl ::core::convert::From<CommitmentHasBeenSubmitted> for CommitmentPoolMainErrors {
        fn from(value: CommitmentHasBeenSubmitted) -> Self {
            Self::CommitmentHasBeenSubmitted(value)
        }
    }
    impl ::core::convert::From<Duplicated> for CommitmentPoolMainErrors {
        fn from(value: Duplicated) -> Self {
            Self::Duplicated(value)
        }
    }
    impl ::core::convert::From<IndexOutOfBound> for CommitmentPoolMainErrors {
        fn from(value: IndexOutOfBound) -> Self {
            Self::IndexOutOfBound(value)
        }
    }
    impl ::core::convert::From<Invalid> for CommitmentPoolMainErrors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<NewRootIsDuplicated> for CommitmentPoolMainErrors {
        fn from(value: NewRootIsDuplicated) -> Self {
            Self::NewRootIsDuplicated(value)
        }
    }
    impl ::core::convert::From<NotChanged> for CommitmentPoolMainErrors {
        fn from(value: NotChanged) -> Self {
            Self::NotChanged(value)
        }
    }
    impl ::core::convert::From<NoteHasBeenSpent> for CommitmentPoolMainErrors {
        fn from(value: NoteHasBeenSpent) -> Self {
            Self::NoteHasBeenSpent(value)
        }
    }
    impl ::core::convert::From<NumInputsGreaterThanZero> for CommitmentPoolMainErrors {
        fn from(value: NumInputsGreaterThanZero) -> Self {
            Self::NumInputsGreaterThanZero(value)
        }
    }
    impl ::core::convert::From<OnlyOperator> for CommitmentPoolMainErrors {
        fn from(value: OnlyOperator) -> Self {
            Self::OnlyOperator(value)
        }
    }
    impl ::core::convert::From<OnlyWhitelistedRoller> for CommitmentPoolMainErrors {
        fn from(value: OnlyWhitelistedRoller) -> Self {
            Self::OnlyWhitelistedRoller(value)
        }
    }
    impl ::core::convert::From<OnlyWhitelistedSender> for CommitmentPoolMainErrors {
        fn from(value: OnlyWhitelistedSender) -> Self {
            Self::OnlyWhitelistedSender(value)
        }
    }
    impl ::core::convert::From<OutputNotesLessThanThree> for CommitmentPoolMainErrors {
        fn from(value: OutputNotesLessThanThree) -> Self {
            Self::OutputNotesLessThanThree(value)
        }
    }
    impl ::core::convert::From<RollupFeeToFew> for CommitmentPoolMainErrors {
        fn from(value: RollupFeeToFew) -> Self {
            Self::RollupFeeToFew(value)
        }
    }
    impl ::core::convert::From<RollupSizeNotPowerOfTwo> for CommitmentPoolMainErrors {
        fn from(value: RollupSizeNotPowerOfTwo) -> Self {
            Self::RollupSizeNotPowerOfTwo(value)
        }
    }
    impl ::core::convert::From<SanctionedAddress> for CommitmentPoolMainErrors {
        fn from(value: SanctionedAddress) -> Self {
            Self::SanctionedAddress(value)
        }
    }
    impl ::core::convert::From<TreeHeightLessThanZero> for CommitmentPoolMainErrors {
        fn from(value: TreeHeightLessThanZero) -> Self {
            Self::TreeHeightLessThanZero(value)
        }
    }
    impl ::core::convert::From<TreeHeightOutOfBounds> for CommitmentPoolMainErrors {
        fn from(value: TreeHeightOutOfBounds) -> Self {
            Self::TreeHeightOutOfBounds(value)
        }
    }
    impl ::core::convert::From<TreeIsFull> for CommitmentPoolMainErrors {
        fn from(value: TreeIsFull) -> Self {
            Self::TreeIsFull(value)
        }
    }
    impl ::core::convert::From<VerifierUpdatesHasBeenDisabled> for CommitmentPoolMainErrors {
        fn from(value: VerifierUpdatesHasBeenDisabled) -> Self {
            Self::VerifierUpdatesHasBeenDisabled(value)
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
    #[ethevent(name = "AuditorPublicKey", abi = "AuditorPublicKey(uint256,uint256)")]
    pub struct AuditorPublicKeyFilter {
        #[ethevent(indexed)]
        pub index: ::ethers_core::types::U256,
        pub public_key: ::ethers_core::types::U256,
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
        Hash,
    )]
    #[ethevent(name = "CommitmentQueued", abi = "CommitmentQueued(uint256,uint256,uint256,bytes)")]
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
        Hash,
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
        Hash,
    )]
    #[ethevent(name = "EncryptedAuditorNote", abi = "EncryptedAuditorNote(uint64,uint256,uint256)")]
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
        Hash,
    )]
    #[ethevent(
        name = "EncryptedAuditorNotes",
        abi = "EncryptedAuditorNotes((uint64,uint256,uint256)[])"
    )]
    pub struct EncryptedAuditorNotesFilter {
        pub notes: ::std::vec::Vec<AuditorNote>,
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
    #[ethevent(name = "RollupWhitelistDisabled", abi = "RollupWhitelistDisabled(bool)")]
    pub struct RollupWhitelistDisabledFilter {
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
    #[ethevent(name = "VerifierUpdateDisabled", abi = "VerifierUpdateDisabled(bool)")]
    pub struct VerifierUpdateDisabledFilter {
        pub state: bool,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers_contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum CommitmentPoolMainEvents {
        AuditorPublicKeyFilter(AuditorPublicKeyFilter),
        CommitmentIncludedFilter(CommitmentIncludedFilter),
        CommitmentQueuedFilter(CommitmentQueuedFilter),
        CommitmentSpentFilter(CommitmentSpentFilter),
        EncryptedAuditorNoteFilter(EncryptedAuditorNoteFilter),
        EncryptedAuditorNotesFilter(EncryptedAuditorNotesFilter),
        OperatorChangedFilter(OperatorChangedFilter),
        RollupWhitelistDisabledFilter(RollupWhitelistDisabledFilter),
        SanctionsCheckFilter(SanctionsCheckFilter),
        SanctionsListFilter(SanctionsListFilter),
        VerifierUpdateDisabledFilter(VerifierUpdateDisabledFilter),
    }
    impl ::ethers_contract::EthLogDecode for CommitmentPoolMainEvents {
        fn decode_log(log: &::ethers_core::abi::RawLog) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = AuditorPublicKeyFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::AuditorPublicKeyFilter(decoded));
            }
            if let Ok(decoded) = CommitmentIncludedFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::CommitmentIncludedFilter(decoded));
            }
            if let Ok(decoded) = CommitmentQueuedFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::CommitmentQueuedFilter(decoded));
            }
            if let Ok(decoded) = CommitmentSpentFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::CommitmentSpentFilter(decoded));
            }
            if let Ok(decoded) = EncryptedAuditorNoteFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::EncryptedAuditorNoteFilter(decoded));
            }
            if let Ok(decoded) = EncryptedAuditorNotesFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::EncryptedAuditorNotesFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = RollupWhitelistDisabledFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::RollupWhitelistDisabledFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::SanctionsListFilter(decoded));
            }
            if let Ok(decoded) = VerifierUpdateDisabledFilter::decode_log(log) {
                return Ok(CommitmentPoolMainEvents::VerifierUpdateDisabledFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CommitmentPoolMainEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AuditorPublicKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentIncludedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentQueuedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentSpentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncryptedAuditorNoteFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncryptedAuditorNotesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupWhitelistDisabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsCheckFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsListFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifierUpdateDisabledFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AuditorPublicKeyFilter> for CommitmentPoolMainEvents {
        fn from(value: AuditorPublicKeyFilter) -> Self {
            Self::AuditorPublicKeyFilter(value)
        }
    }
    impl ::core::convert::From<CommitmentIncludedFilter> for CommitmentPoolMainEvents {
        fn from(value: CommitmentIncludedFilter) -> Self {
            Self::CommitmentIncludedFilter(value)
        }
    }
    impl ::core::convert::From<CommitmentQueuedFilter> for CommitmentPoolMainEvents {
        fn from(value: CommitmentQueuedFilter) -> Self {
            Self::CommitmentQueuedFilter(value)
        }
    }
    impl ::core::convert::From<CommitmentSpentFilter> for CommitmentPoolMainEvents {
        fn from(value: CommitmentSpentFilter) -> Self {
            Self::CommitmentSpentFilter(value)
        }
    }
    impl ::core::convert::From<EncryptedAuditorNoteFilter> for CommitmentPoolMainEvents {
        fn from(value: EncryptedAuditorNoteFilter) -> Self {
            Self::EncryptedAuditorNoteFilter(value)
        }
    }
    impl ::core::convert::From<EncryptedAuditorNotesFilter> for CommitmentPoolMainEvents {
        fn from(value: EncryptedAuditorNotesFilter) -> Self {
            Self::EncryptedAuditorNotesFilter(value)
        }
    }
    impl ::core::convert::From<OperatorChangedFilter> for CommitmentPoolMainEvents {
        fn from(value: OperatorChangedFilter) -> Self {
            Self::OperatorChangedFilter(value)
        }
    }
    impl ::core::convert::From<RollupWhitelistDisabledFilter> for CommitmentPoolMainEvents {
        fn from(value: RollupWhitelistDisabledFilter) -> Self {
            Self::RollupWhitelistDisabledFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckFilter> for CommitmentPoolMainEvents {
        fn from(value: SanctionsCheckFilter) -> Self {
            Self::SanctionsCheckFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsListFilter> for CommitmentPoolMainEvents {
        fn from(value: SanctionsListFilter) -> Self {
            Self::SanctionsListFilter(value)
        }
    }
    impl ::core::convert::From<VerifierUpdateDisabledFilter> for CommitmentPoolMainEvents {
        fn from(value: VerifierUpdateDisabledFilter) -> Self {
            Self::VerifierUpdateDisabledFilter(value)
        }
    }
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
        Hash,
    )]
    #[ethcall(name = "_pathIndices", abi = "_pathIndices(uint256,uint32)")]
    pub struct PathIndicesCall {
        pub full_path: ::ethers_core::types::U256,
        pub rollup_size: u32,
    }
    ///Container type for all input parameters for the `addEnqueueWhitelist` function with signature `addEnqueueWhitelist(address)` and selector `0xa9b1d296`
    #[derive(
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
    #[ethcall(name = "addEnqueueWhitelist", abi = "addEnqueueWhitelist(address)")]
    pub struct AddEnqueueWhitelistCall {
        pub actor: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `addRollupWhitelist` function with signature `addRollupWhitelist(address)` and selector `0x02d498f1`
    #[derive(
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
    #[ethcall(name = "addRollupWhitelist", abi = "addRollupWhitelist(address)")]
    pub struct AddRollupWhitelistCall {
        pub roller: ::ethers_core::types::Address,
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
    ///Container type for all input parameters for the `auditorCount` function with signature `auditorCount()` and selector `0x115f574c`
    #[derive(
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
    #[ethcall(name = "auditorCount", abi = "auditorCount()")]
    pub struct AuditorCountCall;
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
    ///Container type for all input parameters for the `disableRollupVerifier` function with signature `disableRollupVerifier(uint32)` and selector `0x9b0a6fea`
    #[derive(
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
    #[ethcall(name = "disableRollupVerifier", abi = "disableRollupVerifier(uint32)")]
    pub struct DisableRollupVerifierCall {
        pub rollup_size: u32,
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
    ///Container type for all input parameters for the `disableTransactVerifier` function with signature `disableTransactVerifier(uint32,uint32)` and selector `0xc259e2e6`
    #[derive(
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
    #[ethcall(name = "disableTransactVerifier", abi = "disableTransactVerifier(uint32,uint32)")]
    pub struct DisableTransactVerifierCall {
        pub num_inputs: u32,
        pub num_outputs: u32,
    }
    ///Container type for all input parameters for the `enableRollupVerifier` function with signature `enableRollupVerifier(uint32,address)` and selector `0xdeeff7cd`
    #[derive(
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
    #[ethcall(name = "enableRollupVerifier", abi = "enableRollupVerifier(uint32,address)")]
    pub struct EnableRollupVerifierCall {
        pub rollup_size: u32,
        pub rollup_verifier: ::ethers_core::types::Address,
    }
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
    ///Container type for all input parameters for the `enableTransactVerifier` function with signature `enableTransactVerifier(uint32,uint32,address)` and selector `0x7fa4b09c`
    #[derive(
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
        name = "enableTransactVerifier",
        abi = "enableTransactVerifier(uint32,uint32,address)"
    )]
    pub struct EnableTransactVerifierCall {
        pub num_inputs: u32,
        pub num_outputs: u32,
        pub transact_verifier: ::ethers_core::types::Address,
    }
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
        Hash,
    )]
    #[ethcall(name = "enqueue", abi = "enqueue((uint256,uint256,uint256,uint256,bytes),address)")]
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "isKnownRoot", abi = "isKnownRoot(uint256)")]
    pub struct IsKnownRootCall {
        pub root: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `isRollupWhitelistDisabled` function with signature `isRollupWhitelistDisabled()` and selector `0xffa89b88`
    #[derive(
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
    #[ethcall(name = "isRollupWhitelistDisabled", abi = "isRollupWhitelistDisabled()")]
    pub struct IsRollupWhitelistDisabledCall;
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
        Hash,
    )]
    #[ethcall(name = "isSpentSerialNumber", abi = "isSpentSerialNumber(uint256)")]
    pub struct IsSpentSerialNumberCall {
        pub serial_number: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `isVerifierUpdateDisabled` function with signature `isVerifierUpdateDisabled()` and selector `0x4eb069f7`
    #[derive(
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
    #[ethcall(name = "isVerifierUpdateDisabled", abi = "isVerifierUpdateDisabled()")]
    pub struct IsVerifierUpdateDisabledCall;
    ///Container type for all input parameters for the `removeEnqueueWhitelist` function with signature `removeEnqueueWhitelist(address)` and selector `0x03db9874`
    #[derive(
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
    #[ethcall(name = "removeEnqueueWhitelist", abi = "removeEnqueueWhitelist(address)")]
    pub struct RemoveEnqueueWhitelistCall {
        pub actor: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `removeRollupWhitelist` function with signature `removeRollupWhitelist(address)` and selector `0x9cc6b354`
    #[derive(
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
    #[ethcall(name = "removeRollupWhitelist", abi = "removeRollupWhitelist(address)")]
    pub struct RemoveRollupWhitelistCall {
        pub roller: ::ethers_core::types::Address,
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
        Hash,
    )]
    #[ethcall(
        name = "rollup",
        abi = "rollup((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256))"
    )]
    pub struct RollupCall {
        pub request: RollupRequest,
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
    ///Container type for all input parameters for the `setMinRollupFee` function with signature `setMinRollupFee(uint256)` and selector `0x7cbf0ff6`
    #[derive(
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
    #[ethcall(name = "setMinRollupFee", abi = "setMinRollupFee(uint256)")]
    pub struct SetMinRollupFeeCall {
        pub min_rollup_fee: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `setRollupWhitelistDisabled` function with signature `setRollupWhitelistDisabled(bool)` and selector `0xf8f05388`
    #[derive(
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
    #[ethcall(name = "setRollupWhitelistDisabled", abi = "setRollupWhitelistDisabled(bool)")]
    pub struct SetRollupWhitelistDisabledCall {
        pub state: bool,
    }
    ///Container type for all input parameters for the `setVerifierUpdateDisabled` function with signature `setVerifierUpdateDisabled(bool)` and selector `0xb3b75631`
    #[derive(
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
    #[ethcall(name = "setVerifierUpdateDisabled", abi = "setVerifierUpdateDisabled(bool)")]
    pub struct SetVerifierUpdateDisabledCall {
        pub state: bool,
    }
    ///Container type for all input parameters for the `transact` function with signature `transact((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[]),bytes)` and selector `0x72082971`
    #[derive(
        Clone, ::ethers_contract::EthCall, ::ethers_contract::EthDisplay, serde::Serialize, serde::Deserialize,
    )]
    #[ethcall(
        name = "transact",
        abi = "transact((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[]),bytes)"
    )]
    pub struct TransactCall {
        pub request: TransactRequest,
        pub signature: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `updateAuditorPublicKey` function with signature `updateAuditorPublicKey(uint256,uint256)` and selector `0x0c8867e6`
    #[derive(
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
    #[ethcall(name = "updateAuditorPublicKey", abi = "updateAuditorPublicKey(uint256,uint256)")]
    pub struct UpdateAuditorPublicKeyCall {
        pub index: ::ethers_core::types::U256,
        pub public_key: ::ethers_core::types::U256,
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
    #[derive(Clone, ::ethers_contract::EthAbiType, serde::Serialize, serde::Deserialize)]
    pub enum CommitmentPoolMainCalls {
        PathIndices(PathIndicesCall),
        AddEnqueueWhitelist(AddEnqueueWhitelistCall),
        AddRollupWhitelist(AddRollupWhitelistCall),
        AssetType(AssetTypeCall),
        AuditorCount(AuditorCountCall),
        ChangeOperator(ChangeOperatorCall),
        DisableRollupVerifier(DisableRollupVerifierCall),
        DisableSanctionsCheck(DisableSanctionsCheckCall),
        DisableTransactVerifier(DisableTransactVerifierCall),
        EnableRollupVerifier(EnableRollupVerifierCall),
        EnableSanctionsCheck(EnableSanctionsCheckCall),
        EnableTransactVerifier(EnableTransactVerifierCall),
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
        IsRollupWhitelistDisabled(IsRollupWhitelistDisabledCall),
        IsSpentSerialNumber(IsSpentSerialNumberCall),
        IsVerifierUpdateDisabled(IsVerifierUpdateDisabledCall),
        RemoveEnqueueWhitelist(RemoveEnqueueWhitelistCall),
        RemoveRollupWhitelist(RemoveRollupWhitelistCall),
        Rollup(RollupCall),
        SanctionsCheck(SanctionsCheckCall),
        SanctionsList(SanctionsListCall),
        SetMinRollupFee(SetMinRollupFeeCall),
        SetRollupWhitelistDisabled(SetRollupWhitelistDisabledCall),
        SetVerifierUpdateDisabled(SetVerifierUpdateDisabledCall),
        Transact(TransactCall),
        UpdateAuditorPublicKey(UpdateAuditorPublicKeyCall),
        UpdateSanctionsListAddress(UpdateSanctionsListAddressCall),
    }
    impl ::ethers_core::abi::AbiDecode for CommitmentPoolMainCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <PathIndicesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PathIndices(decoded));
            }
            if let Ok(decoded) = <AddEnqueueWhitelistCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddEnqueueWhitelist(decoded));
            }
            if let Ok(decoded) = <AddRollupWhitelistCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddRollupWhitelist(decoded));
            }
            if let Ok(decoded) = <AssetTypeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetType(decoded));
            }
            if let Ok(decoded) = <AuditorCountCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AuditorCount(decoded));
            }
            if let Ok(decoded) = <ChangeOperatorCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChangeOperator(decoded));
            }
            if let Ok(decoded) = <DisableRollupVerifierCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableRollupVerifier(decoded));
            }
            if let Ok(decoded) = <DisableSanctionsCheckCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableSanctionsCheck(decoded));
            }
            if let Ok(decoded) = <DisableTransactVerifierCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableTransactVerifier(decoded));
            }
            if let Ok(decoded) = <EnableRollupVerifierCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableRollupVerifier(decoded));
            }
            if let Ok(decoded) = <EnableSanctionsCheckCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableSanctionsCheck(decoded));
            }
            if let Ok(decoded) = <EnableTransactVerifierCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableTransactVerifier(decoded));
            }
            if let Ok(decoded) = <EnqueueCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Enqueue(decoded));
            }
            if let Ok(decoded) = <GetAllAuditorPublicKeysCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAllAuditorPublicKeys(decoded));
            }
            if let Ok(decoded) = <GetAuditorPublicKeyCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAuditorPublicKey(decoded));
            }
            if let Ok(decoded) = <GetCommitmentCountCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCommitmentCount(decoded));
            }
            if let Ok(decoded) = <GetCommitmentIncludedCountCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCommitmentIncludedCount(decoded));
            }
            if let Ok(decoded) = <GetCommitmentQueuedCountCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCommitmentQueuedCount(decoded));
            }
            if let Ok(decoded) = <GetMinRollupFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinRollupFee(decoded));
            }
            if let Ok(decoded) = <GetNullifierCountCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNullifierCount(decoded));
            }
            if let Ok(decoded) = <GetQueuedCommitmentsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetQueuedCommitments(decoded));
            }
            if let Ok(decoded) = <GetTreeCapacityCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTreeCapacity(decoded));
            }
            if let Ok(decoded) = <IsHistoricCommitmentCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsHistoricCommitment(decoded));
            }
            if let Ok(decoded) = <IsKnownRootCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsKnownRoot(decoded));
            }
            if let Ok(decoded) = <IsRollupWhitelistDisabledCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsRollupWhitelistDisabled(decoded));
            }
            if let Ok(decoded) = <IsSpentSerialNumberCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsSpentSerialNumber(decoded));
            }
            if let Ok(decoded) = <IsVerifierUpdateDisabledCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsVerifierUpdateDisabled(decoded));
            }
            if let Ok(decoded) = <RemoveEnqueueWhitelistCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveEnqueueWhitelist(decoded));
            }
            if let Ok(decoded) = <RemoveRollupWhitelistCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveRollupWhitelist(decoded));
            }
            if let Ok(decoded) = <RollupCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rollup(decoded));
            }
            if let Ok(decoded) = <SanctionsCheckCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionsCheck(decoded));
            }
            if let Ok(decoded) = <SanctionsListCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionsList(decoded));
            }
            if let Ok(decoded) = <SetMinRollupFeeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMinRollupFee(decoded));
            }
            if let Ok(decoded) = <SetRollupWhitelistDisabledCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRollupWhitelistDisabled(decoded));
            }
            if let Ok(decoded) = <SetVerifierUpdateDisabledCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetVerifierUpdateDisabled(decoded));
            }
            if let Ok(decoded) = <TransactCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transact(decoded));
            }
            if let Ok(decoded) = <UpdateAuditorPublicKeyCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateAuditorPublicKey(decoded));
            }
            if let Ok(decoded) = <UpdateSanctionsListAddressCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateSanctionsListAddress(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for CommitmentPoolMainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::PathIndices(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AddEnqueueWhitelist(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AddRollupWhitelist(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AssetType(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AuditorCount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ChangeOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DisableRollupVerifier(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DisableSanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DisableTransactVerifier(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::EnableRollupVerifier(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::EnableSanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::EnableTransactVerifier(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Enqueue(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetAllAuditorPublicKeys(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetAuditorPublicKey(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetCommitmentCount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetCommitmentIncludedCount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetCommitmentQueuedCount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMinRollupFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetNullifierCount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetQueuedCommitments(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetTreeCapacity(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::IsHistoricCommitment(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::IsKnownRoot(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::IsRollupWhitelistDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::IsSpentSerialNumber(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::IsVerifierUpdateDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RemoveEnqueueWhitelist(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RemoveRollupWhitelist(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Rollup(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionsList(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetMinRollupFee(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetRollupWhitelistDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetVerifierUpdateDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Transact(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UpdateAuditorPublicKey(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UpdateSanctionsListAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CommitmentPoolMainCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PathIndices(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddEnqueueWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddRollupWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetType(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuditorCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableRollupVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableSanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableTransactVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableRollupVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableSanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableTransactVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::Enqueue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllAuditorPublicKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAuditorPublicKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCommitmentCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCommitmentIncludedCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCommitmentQueuedCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNullifierCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetQueuedCommitments(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTreeCapacity(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsHistoricCommitment(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsKnownRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsRollupWhitelistDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSpentSerialNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsVerifierUpdateDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveEnqueueWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveRollupWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rollup(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRollupWhitelistDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetVerifierUpdateDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transact(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAuditorPublicKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateSanctionsListAddress(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PathIndicesCall> for CommitmentPoolMainCalls {
        fn from(value: PathIndicesCall) -> Self {
            Self::PathIndices(value)
        }
    }
    impl ::core::convert::From<AddEnqueueWhitelistCall> for CommitmentPoolMainCalls {
        fn from(value: AddEnqueueWhitelistCall) -> Self {
            Self::AddEnqueueWhitelist(value)
        }
    }
    impl ::core::convert::From<AddRollupWhitelistCall> for CommitmentPoolMainCalls {
        fn from(value: AddRollupWhitelistCall) -> Self {
            Self::AddRollupWhitelist(value)
        }
    }
    impl ::core::convert::From<AssetTypeCall> for CommitmentPoolMainCalls {
        fn from(value: AssetTypeCall) -> Self {
            Self::AssetType(value)
        }
    }
    impl ::core::convert::From<AuditorCountCall> for CommitmentPoolMainCalls {
        fn from(value: AuditorCountCall) -> Self {
            Self::AuditorCount(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorCall> for CommitmentPoolMainCalls {
        fn from(value: ChangeOperatorCall) -> Self {
            Self::ChangeOperator(value)
        }
    }
    impl ::core::convert::From<DisableRollupVerifierCall> for CommitmentPoolMainCalls {
        fn from(value: DisableRollupVerifierCall) -> Self {
            Self::DisableRollupVerifier(value)
        }
    }
    impl ::core::convert::From<DisableSanctionsCheckCall> for CommitmentPoolMainCalls {
        fn from(value: DisableSanctionsCheckCall) -> Self {
            Self::DisableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<DisableTransactVerifierCall> for CommitmentPoolMainCalls {
        fn from(value: DisableTransactVerifierCall) -> Self {
            Self::DisableTransactVerifier(value)
        }
    }
    impl ::core::convert::From<EnableRollupVerifierCall> for CommitmentPoolMainCalls {
        fn from(value: EnableRollupVerifierCall) -> Self {
            Self::EnableRollupVerifier(value)
        }
    }
    impl ::core::convert::From<EnableSanctionsCheckCall> for CommitmentPoolMainCalls {
        fn from(value: EnableSanctionsCheckCall) -> Self {
            Self::EnableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<EnableTransactVerifierCall> for CommitmentPoolMainCalls {
        fn from(value: EnableTransactVerifierCall) -> Self {
            Self::EnableTransactVerifier(value)
        }
    }
    impl ::core::convert::From<EnqueueCall> for CommitmentPoolMainCalls {
        fn from(value: EnqueueCall) -> Self {
            Self::Enqueue(value)
        }
    }
    impl ::core::convert::From<GetAllAuditorPublicKeysCall> for CommitmentPoolMainCalls {
        fn from(value: GetAllAuditorPublicKeysCall) -> Self {
            Self::GetAllAuditorPublicKeys(value)
        }
    }
    impl ::core::convert::From<GetAuditorPublicKeyCall> for CommitmentPoolMainCalls {
        fn from(value: GetAuditorPublicKeyCall) -> Self {
            Self::GetAuditorPublicKey(value)
        }
    }
    impl ::core::convert::From<GetCommitmentCountCall> for CommitmentPoolMainCalls {
        fn from(value: GetCommitmentCountCall) -> Self {
            Self::GetCommitmentCount(value)
        }
    }
    impl ::core::convert::From<GetCommitmentIncludedCountCall> for CommitmentPoolMainCalls {
        fn from(value: GetCommitmentIncludedCountCall) -> Self {
            Self::GetCommitmentIncludedCount(value)
        }
    }
    impl ::core::convert::From<GetCommitmentQueuedCountCall> for CommitmentPoolMainCalls {
        fn from(value: GetCommitmentQueuedCountCall) -> Self {
            Self::GetCommitmentQueuedCount(value)
        }
    }
    impl ::core::convert::From<GetMinRollupFeeCall> for CommitmentPoolMainCalls {
        fn from(value: GetMinRollupFeeCall) -> Self {
            Self::GetMinRollupFee(value)
        }
    }
    impl ::core::convert::From<GetNullifierCountCall> for CommitmentPoolMainCalls {
        fn from(value: GetNullifierCountCall) -> Self {
            Self::GetNullifierCount(value)
        }
    }
    impl ::core::convert::From<GetQueuedCommitmentsCall> for CommitmentPoolMainCalls {
        fn from(value: GetQueuedCommitmentsCall) -> Self {
            Self::GetQueuedCommitments(value)
        }
    }
    impl ::core::convert::From<GetTreeCapacityCall> for CommitmentPoolMainCalls {
        fn from(value: GetTreeCapacityCall) -> Self {
            Self::GetTreeCapacity(value)
        }
    }
    impl ::core::convert::From<IsHistoricCommitmentCall> for CommitmentPoolMainCalls {
        fn from(value: IsHistoricCommitmentCall) -> Self {
            Self::IsHistoricCommitment(value)
        }
    }
    impl ::core::convert::From<IsKnownRootCall> for CommitmentPoolMainCalls {
        fn from(value: IsKnownRootCall) -> Self {
            Self::IsKnownRoot(value)
        }
    }
    impl ::core::convert::From<IsRollupWhitelistDisabledCall> for CommitmentPoolMainCalls {
        fn from(value: IsRollupWhitelistDisabledCall) -> Self {
            Self::IsRollupWhitelistDisabled(value)
        }
    }
    impl ::core::convert::From<IsSpentSerialNumberCall> for CommitmentPoolMainCalls {
        fn from(value: IsSpentSerialNumberCall) -> Self {
            Self::IsSpentSerialNumber(value)
        }
    }
    impl ::core::convert::From<IsVerifierUpdateDisabledCall> for CommitmentPoolMainCalls {
        fn from(value: IsVerifierUpdateDisabledCall) -> Self {
            Self::IsVerifierUpdateDisabled(value)
        }
    }
    impl ::core::convert::From<RemoveEnqueueWhitelistCall> for CommitmentPoolMainCalls {
        fn from(value: RemoveEnqueueWhitelistCall) -> Self {
            Self::RemoveEnqueueWhitelist(value)
        }
    }
    impl ::core::convert::From<RemoveRollupWhitelistCall> for CommitmentPoolMainCalls {
        fn from(value: RemoveRollupWhitelistCall) -> Self {
            Self::RemoveRollupWhitelist(value)
        }
    }
    impl ::core::convert::From<RollupCall> for CommitmentPoolMainCalls {
        fn from(value: RollupCall) -> Self {
            Self::Rollup(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckCall> for CommitmentPoolMainCalls {
        fn from(value: SanctionsCheckCall) -> Self {
            Self::SanctionsCheck(value)
        }
    }
    impl ::core::convert::From<SanctionsListCall> for CommitmentPoolMainCalls {
        fn from(value: SanctionsListCall) -> Self {
            Self::SanctionsList(value)
        }
    }
    impl ::core::convert::From<SetMinRollupFeeCall> for CommitmentPoolMainCalls {
        fn from(value: SetMinRollupFeeCall) -> Self {
            Self::SetMinRollupFee(value)
        }
    }
    impl ::core::convert::From<SetRollupWhitelistDisabledCall> for CommitmentPoolMainCalls {
        fn from(value: SetRollupWhitelistDisabledCall) -> Self {
            Self::SetRollupWhitelistDisabled(value)
        }
    }
    impl ::core::convert::From<SetVerifierUpdateDisabledCall> for CommitmentPoolMainCalls {
        fn from(value: SetVerifierUpdateDisabledCall) -> Self {
            Self::SetVerifierUpdateDisabled(value)
        }
    }
    impl ::core::convert::From<TransactCall> for CommitmentPoolMainCalls {
        fn from(value: TransactCall) -> Self {
            Self::Transact(value)
        }
    }
    impl ::core::convert::From<UpdateAuditorPublicKeyCall> for CommitmentPoolMainCalls {
        fn from(value: UpdateAuditorPublicKeyCall) -> Self {
            Self::UpdateAuditorPublicKey(value)
        }
    }
    impl ::core::convert::From<UpdateSanctionsListAddressCall> for CommitmentPoolMainCalls {
        fn from(value: UpdateSanctionsListAddressCall) -> Self {
            Self::UpdateSanctionsListAddress(value)
        }
    }
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
        Hash,
    )]
    pub struct PathIndicesReturn(pub ::ethers_core::types::U256);
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
    ///Container type for all return fields from the `auditorCount` function with signature `auditorCount()` and selector `0x115f574c`
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
    pub struct AuditorCountReturn(pub ::ethers_core::types::U256);
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
        Hash,
    )]
    pub struct GetAllAuditorPublicKeysReturn(pub ::std::vec::Vec<::ethers_core::types::U256>);
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct GetQueuedCommitmentsReturn(pub ::std::vec::Vec<::ethers_core::types::U256>);
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct IsKnownRootReturn(pub bool);
    ///Container type for all return fields from the `isRollupWhitelistDisabled` function with signature `isRollupWhitelistDisabled()` and selector `0xffa89b88`
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
    pub struct IsRollupWhitelistDisabledReturn(pub bool);
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
        Hash,
    )]
    pub struct IsSpentSerialNumberReturn(pub bool);
    ///Container type for all return fields from the `isVerifierUpdateDisabled` function with signature `isVerifierUpdateDisabled()` and selector `0x4eb069f7`
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
    pub struct IsVerifierUpdateDisabledReturn(pub bool);
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct Proof {
        pub a: G1Point,
        pub b: G2Point,
        pub c: G1Point,
    }
}
