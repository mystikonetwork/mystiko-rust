pub use commitment_pool::*;
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
pub mod commitment_pool {
    const _: () = {
        ::core::include_bytes!("../json/CommitmentPool.json",);
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COMMITMENTPOOL_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    pub struct CommitmentPool<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for CommitmentPool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CommitmentPool<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CommitmentPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CommitmentPool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CommitmentPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> CommitmentPool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                COMMITMENTPOOL_ABI.clone(),
                client,
            ))
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
        pub fn events(&self) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, CommitmentPoolEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for CommitmentPool<M> {
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
    pub enum CommitmentPoolErrors {
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
    impl ::ethers_core::abi::AbiDecode for CommitmentPoolErrors {
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
    impl ::ethers_core::abi::AbiEncode for CommitmentPoolErrors {
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
    impl ::ethers_contract::ContractRevert for CommitmentPoolErrors {
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
    impl ::core::fmt::Display for CommitmentPoolErrors {
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
    impl ::core::convert::From<::std::string::String> for CommitmentPoolErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AuditorIndexError> for CommitmentPoolErrors {
        fn from(value: AuditorIndexError) -> Self {
            Self::AuditorIndexError(value)
        }
    }
    impl ::core::convert::From<AuditorNotesLengthError> for CommitmentPoolErrors {
        fn from(value: AuditorNotesLengthError) -> Self {
            Self::AuditorNotesLengthError(value)
        }
    }
    impl ::core::convert::From<AuditorPublicKeyNotChanged> for CommitmentPoolErrors {
        fn from(value: AuditorPublicKeyNotChanged) -> Self {
            Self::AuditorPublicKeyNotChanged(value)
        }
    }
    impl ::core::convert::From<CommitmentHasBeenSubmitted> for CommitmentPoolErrors {
        fn from(value: CommitmentHasBeenSubmitted) -> Self {
            Self::CommitmentHasBeenSubmitted(value)
        }
    }
    impl ::core::convert::From<Duplicated> for CommitmentPoolErrors {
        fn from(value: Duplicated) -> Self {
            Self::Duplicated(value)
        }
    }
    impl ::core::convert::From<IndexOutOfBound> for CommitmentPoolErrors {
        fn from(value: IndexOutOfBound) -> Self {
            Self::IndexOutOfBound(value)
        }
    }
    impl ::core::convert::From<Invalid> for CommitmentPoolErrors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<NewRootIsDuplicated> for CommitmentPoolErrors {
        fn from(value: NewRootIsDuplicated) -> Self {
            Self::NewRootIsDuplicated(value)
        }
    }
    impl ::core::convert::From<NotChanged> for CommitmentPoolErrors {
        fn from(value: NotChanged) -> Self {
            Self::NotChanged(value)
        }
    }
    impl ::core::convert::From<NoteHasBeenSpent> for CommitmentPoolErrors {
        fn from(value: NoteHasBeenSpent) -> Self {
            Self::NoteHasBeenSpent(value)
        }
    }
    impl ::core::convert::From<NumInputsGreaterThanZero> for CommitmentPoolErrors {
        fn from(value: NumInputsGreaterThanZero) -> Self {
            Self::NumInputsGreaterThanZero(value)
        }
    }
    impl ::core::convert::From<OnlyOperator> for CommitmentPoolErrors {
        fn from(value: OnlyOperator) -> Self {
            Self::OnlyOperator(value)
        }
    }
    impl ::core::convert::From<OnlyWhitelistedRoller> for CommitmentPoolErrors {
        fn from(value: OnlyWhitelistedRoller) -> Self {
            Self::OnlyWhitelistedRoller(value)
        }
    }
    impl ::core::convert::From<OnlyWhitelistedSender> for CommitmentPoolErrors {
        fn from(value: OnlyWhitelistedSender) -> Self {
            Self::OnlyWhitelistedSender(value)
        }
    }
    impl ::core::convert::From<OutputNotesLessThanThree> for CommitmentPoolErrors {
        fn from(value: OutputNotesLessThanThree) -> Self {
            Self::OutputNotesLessThanThree(value)
        }
    }
    impl ::core::convert::From<RollupFeeToFew> for CommitmentPoolErrors {
        fn from(value: RollupFeeToFew) -> Self {
            Self::RollupFeeToFew(value)
        }
    }
    impl ::core::convert::From<RollupSizeNotPowerOfTwo> for CommitmentPoolErrors {
        fn from(value: RollupSizeNotPowerOfTwo) -> Self {
            Self::RollupSizeNotPowerOfTwo(value)
        }
    }
    impl ::core::convert::From<SanctionedAddress> for CommitmentPoolErrors {
        fn from(value: SanctionedAddress) -> Self {
            Self::SanctionedAddress(value)
        }
    }
    impl ::core::convert::From<TreeHeightLessThanZero> for CommitmentPoolErrors {
        fn from(value: TreeHeightLessThanZero) -> Self {
            Self::TreeHeightLessThanZero(value)
        }
    }
    impl ::core::convert::From<TreeHeightOutOfBounds> for CommitmentPoolErrors {
        fn from(value: TreeHeightOutOfBounds) -> Self {
            Self::TreeHeightOutOfBounds(value)
        }
    }
    impl ::core::convert::From<TreeIsFull> for CommitmentPoolErrors {
        fn from(value: TreeIsFull) -> Self {
            Self::TreeIsFull(value)
        }
    }
    impl ::core::convert::From<VerifierUpdatesHasBeenDisabled> for CommitmentPoolErrors {
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
    pub enum CommitmentPoolEvents {
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
    impl ::ethers_contract::EthLogDecode for CommitmentPoolEvents {
        fn decode_log(log: &::ethers_core::abi::RawLog) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = AuditorPublicKeyFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::AuditorPublicKeyFilter(decoded));
            }
            if let Ok(decoded) = CommitmentIncludedFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::CommitmentIncludedFilter(decoded));
            }
            if let Ok(decoded) = CommitmentQueuedFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::CommitmentQueuedFilter(decoded));
            }
            if let Ok(decoded) = CommitmentSpentFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::CommitmentSpentFilter(decoded));
            }
            if let Ok(decoded) = EncryptedAuditorNoteFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::EncryptedAuditorNoteFilter(decoded));
            }
            if let Ok(decoded) = EncryptedAuditorNotesFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::EncryptedAuditorNotesFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = RollupWhitelistDisabledFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::RollupWhitelistDisabledFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::SanctionsListFilter(decoded));
            }
            if let Ok(decoded) = VerifierUpdateDisabledFilter::decode_log(log) {
                return Ok(CommitmentPoolEvents::VerifierUpdateDisabledFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CommitmentPoolEvents {
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
    impl ::core::convert::From<AuditorPublicKeyFilter> for CommitmentPoolEvents {
        fn from(value: AuditorPublicKeyFilter) -> Self {
            Self::AuditorPublicKeyFilter(value)
        }
    }
    impl ::core::convert::From<CommitmentIncludedFilter> for CommitmentPoolEvents {
        fn from(value: CommitmentIncludedFilter) -> Self {
            Self::CommitmentIncludedFilter(value)
        }
    }
    impl ::core::convert::From<CommitmentQueuedFilter> for CommitmentPoolEvents {
        fn from(value: CommitmentQueuedFilter) -> Self {
            Self::CommitmentQueuedFilter(value)
        }
    }
    impl ::core::convert::From<CommitmentSpentFilter> for CommitmentPoolEvents {
        fn from(value: CommitmentSpentFilter) -> Self {
            Self::CommitmentSpentFilter(value)
        }
    }
    impl ::core::convert::From<EncryptedAuditorNoteFilter> for CommitmentPoolEvents {
        fn from(value: EncryptedAuditorNoteFilter) -> Self {
            Self::EncryptedAuditorNoteFilter(value)
        }
    }
    impl ::core::convert::From<EncryptedAuditorNotesFilter> for CommitmentPoolEvents {
        fn from(value: EncryptedAuditorNotesFilter) -> Self {
            Self::EncryptedAuditorNotesFilter(value)
        }
    }
    impl ::core::convert::From<OperatorChangedFilter> for CommitmentPoolEvents {
        fn from(value: OperatorChangedFilter) -> Self {
            Self::OperatorChangedFilter(value)
        }
    }
    impl ::core::convert::From<RollupWhitelistDisabledFilter> for CommitmentPoolEvents {
        fn from(value: RollupWhitelistDisabledFilter) -> Self {
            Self::RollupWhitelistDisabledFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckFilter> for CommitmentPoolEvents {
        fn from(value: SanctionsCheckFilter) -> Self {
            Self::SanctionsCheckFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsListFilter> for CommitmentPoolEvents {
        fn from(value: SanctionsListFilter) -> Self {
            Self::SanctionsListFilter(value)
        }
    }
    impl ::core::convert::From<VerifierUpdateDisabledFilter> for CommitmentPoolEvents {
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
    pub enum CommitmentPoolCalls {
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
    impl ::ethers_core::abi::AbiDecode for CommitmentPoolCalls {
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
    impl ::ethers_core::abi::AbiEncode for CommitmentPoolCalls {
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
    impl ::core::fmt::Display for CommitmentPoolCalls {
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
    impl ::core::convert::From<PathIndicesCall> for CommitmentPoolCalls {
        fn from(value: PathIndicesCall) -> Self {
            Self::PathIndices(value)
        }
    }
    impl ::core::convert::From<AddEnqueueWhitelistCall> for CommitmentPoolCalls {
        fn from(value: AddEnqueueWhitelistCall) -> Self {
            Self::AddEnqueueWhitelist(value)
        }
    }
    impl ::core::convert::From<AddRollupWhitelistCall> for CommitmentPoolCalls {
        fn from(value: AddRollupWhitelistCall) -> Self {
            Self::AddRollupWhitelist(value)
        }
    }
    impl ::core::convert::From<AssetTypeCall> for CommitmentPoolCalls {
        fn from(value: AssetTypeCall) -> Self {
            Self::AssetType(value)
        }
    }
    impl ::core::convert::From<AuditorCountCall> for CommitmentPoolCalls {
        fn from(value: AuditorCountCall) -> Self {
            Self::AuditorCount(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorCall> for CommitmentPoolCalls {
        fn from(value: ChangeOperatorCall) -> Self {
            Self::ChangeOperator(value)
        }
    }
    impl ::core::convert::From<DisableRollupVerifierCall> for CommitmentPoolCalls {
        fn from(value: DisableRollupVerifierCall) -> Self {
            Self::DisableRollupVerifier(value)
        }
    }
    impl ::core::convert::From<DisableSanctionsCheckCall> for CommitmentPoolCalls {
        fn from(value: DisableSanctionsCheckCall) -> Self {
            Self::DisableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<DisableTransactVerifierCall> for CommitmentPoolCalls {
        fn from(value: DisableTransactVerifierCall) -> Self {
            Self::DisableTransactVerifier(value)
        }
    }
    impl ::core::convert::From<EnableRollupVerifierCall> for CommitmentPoolCalls {
        fn from(value: EnableRollupVerifierCall) -> Self {
            Self::EnableRollupVerifier(value)
        }
    }
    impl ::core::convert::From<EnableSanctionsCheckCall> for CommitmentPoolCalls {
        fn from(value: EnableSanctionsCheckCall) -> Self {
            Self::EnableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<EnableTransactVerifierCall> for CommitmentPoolCalls {
        fn from(value: EnableTransactVerifierCall) -> Self {
            Self::EnableTransactVerifier(value)
        }
    }
    impl ::core::convert::From<EnqueueCall> for CommitmentPoolCalls {
        fn from(value: EnqueueCall) -> Self {
            Self::Enqueue(value)
        }
    }
    impl ::core::convert::From<GetAllAuditorPublicKeysCall> for CommitmentPoolCalls {
        fn from(value: GetAllAuditorPublicKeysCall) -> Self {
            Self::GetAllAuditorPublicKeys(value)
        }
    }
    impl ::core::convert::From<GetAuditorPublicKeyCall> for CommitmentPoolCalls {
        fn from(value: GetAuditorPublicKeyCall) -> Self {
            Self::GetAuditorPublicKey(value)
        }
    }
    impl ::core::convert::From<GetCommitmentCountCall> for CommitmentPoolCalls {
        fn from(value: GetCommitmentCountCall) -> Self {
            Self::GetCommitmentCount(value)
        }
    }
    impl ::core::convert::From<GetCommitmentIncludedCountCall> for CommitmentPoolCalls {
        fn from(value: GetCommitmentIncludedCountCall) -> Self {
            Self::GetCommitmentIncludedCount(value)
        }
    }
    impl ::core::convert::From<GetCommitmentQueuedCountCall> for CommitmentPoolCalls {
        fn from(value: GetCommitmentQueuedCountCall) -> Self {
            Self::GetCommitmentQueuedCount(value)
        }
    }
    impl ::core::convert::From<GetMinRollupFeeCall> for CommitmentPoolCalls {
        fn from(value: GetMinRollupFeeCall) -> Self {
            Self::GetMinRollupFee(value)
        }
    }
    impl ::core::convert::From<GetNullifierCountCall> for CommitmentPoolCalls {
        fn from(value: GetNullifierCountCall) -> Self {
            Self::GetNullifierCount(value)
        }
    }
    impl ::core::convert::From<GetQueuedCommitmentsCall> for CommitmentPoolCalls {
        fn from(value: GetQueuedCommitmentsCall) -> Self {
            Self::GetQueuedCommitments(value)
        }
    }
    impl ::core::convert::From<GetTreeCapacityCall> for CommitmentPoolCalls {
        fn from(value: GetTreeCapacityCall) -> Self {
            Self::GetTreeCapacity(value)
        }
    }
    impl ::core::convert::From<IsHistoricCommitmentCall> for CommitmentPoolCalls {
        fn from(value: IsHistoricCommitmentCall) -> Self {
            Self::IsHistoricCommitment(value)
        }
    }
    impl ::core::convert::From<IsKnownRootCall> for CommitmentPoolCalls {
        fn from(value: IsKnownRootCall) -> Self {
            Self::IsKnownRoot(value)
        }
    }
    impl ::core::convert::From<IsRollupWhitelistDisabledCall> for CommitmentPoolCalls {
        fn from(value: IsRollupWhitelistDisabledCall) -> Self {
            Self::IsRollupWhitelistDisabled(value)
        }
    }
    impl ::core::convert::From<IsSpentSerialNumberCall> for CommitmentPoolCalls {
        fn from(value: IsSpentSerialNumberCall) -> Self {
            Self::IsSpentSerialNumber(value)
        }
    }
    impl ::core::convert::From<IsVerifierUpdateDisabledCall> for CommitmentPoolCalls {
        fn from(value: IsVerifierUpdateDisabledCall) -> Self {
            Self::IsVerifierUpdateDisabled(value)
        }
    }
    impl ::core::convert::From<RemoveEnqueueWhitelistCall> for CommitmentPoolCalls {
        fn from(value: RemoveEnqueueWhitelistCall) -> Self {
            Self::RemoveEnqueueWhitelist(value)
        }
    }
    impl ::core::convert::From<RemoveRollupWhitelistCall> for CommitmentPoolCalls {
        fn from(value: RemoveRollupWhitelistCall) -> Self {
            Self::RemoveRollupWhitelist(value)
        }
    }
    impl ::core::convert::From<RollupCall> for CommitmentPoolCalls {
        fn from(value: RollupCall) -> Self {
            Self::Rollup(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckCall> for CommitmentPoolCalls {
        fn from(value: SanctionsCheckCall) -> Self {
            Self::SanctionsCheck(value)
        }
    }
    impl ::core::convert::From<SanctionsListCall> for CommitmentPoolCalls {
        fn from(value: SanctionsListCall) -> Self {
            Self::SanctionsList(value)
        }
    }
    impl ::core::convert::From<SetMinRollupFeeCall> for CommitmentPoolCalls {
        fn from(value: SetMinRollupFeeCall) -> Self {
            Self::SetMinRollupFee(value)
        }
    }
    impl ::core::convert::From<SetRollupWhitelistDisabledCall> for CommitmentPoolCalls {
        fn from(value: SetRollupWhitelistDisabledCall) -> Self {
            Self::SetRollupWhitelistDisabled(value)
        }
    }
    impl ::core::convert::From<SetVerifierUpdateDisabledCall> for CommitmentPoolCalls {
        fn from(value: SetVerifierUpdateDisabledCall) -> Self {
            Self::SetVerifierUpdateDisabled(value)
        }
    }
    impl ::core::convert::From<TransactCall> for CommitmentPoolCalls {
        fn from(value: TransactCall) -> Self {
            Self::Transact(value)
        }
    }
    impl ::core::convert::From<UpdateAuditorPublicKeyCall> for CommitmentPoolCalls {
        fn from(value: UpdateAuditorPublicKeyCall) -> Self {
            Self::UpdateAuditorPublicKey(value)
        }
    }
    impl ::core::convert::From<UpdateSanctionsListAddressCall> for CommitmentPoolCalls {
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
