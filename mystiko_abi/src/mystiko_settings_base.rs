pub use mystiko_settings_base::*;
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
pub mod mystiko_settings_base {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AUDITOR_COUNT"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("AUDITOR_COUNT"),
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
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("associatedPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("associatedPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("certificate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("certificate"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract IMystikoCertificate",
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("daoRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("daoRegistry"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract MystikoGovernorRegistry",
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositDisableMap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositDisableMap"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("disableRollupVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("disableRollupVerifier",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_rollupSize"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("disableSanctionsCheck"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("disableSanctionsCheck",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("disableTransactVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("disableTransactVerifier",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numInputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numOutputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enableRollupVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enableRollupVerifier",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_rollupSize"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enableSanctionsCheck"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enableSanctionsCheck",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enableTransactVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enableTransactVerifier",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numInputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numOutputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCertificateIssuer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCertificateIssuer",),
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("role"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("grantRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hasRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("isDepositDisable"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isDepositDisable"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_depositAddress"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("isSanctioned"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isSanctioned"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("isTransferDisable"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isTransferDisable"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("maxDepositAmountMap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("maxDepositAmountMap",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("minDepositAmountMap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("minDepositAmountMap",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("minRollupFeeMap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("minRollupFeeMap"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("queryAllAuditorPublicKeys"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queryAllAuditorPublicKeys",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ),),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256[]"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("queryAssociatedPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queryAssociatedPool",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_depositAddress"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("queryAuditorPublicKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queryAuditorPublicKey",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("queryMaxDepositAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queryMaxDepositAmount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("queryMinDepositAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queryMinDepositAmount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("queryMinRollupFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queryMinRollupFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("queryRollupVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queryRollupVerifier",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_rollupSize"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct WrappedVerifier"
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("queryTransactVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queryTransactVerifier",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numInputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numOutputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct WrappedVerifier"
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("relayerPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("relayerPool"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract IMystikoRelayerPool",
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("callerConfirmation",),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("rollerPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rollerPool"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract IMystikoRollerPool",
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sanctionsCheck"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sanctionsCheck"),
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
                    ::std::borrow::ToOwned::to_owned("sanctionsList"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sanctionsList"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract ISanctions"
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAdminRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAdminRole"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAssociatedPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAssociatedPool"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_depositAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_poolAddress"),
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
                    ::std::borrow::ToOwned::to_owned("setAuditorPublicKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAuditorPublicKey",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_publicKey"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCertificateVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setCertificateVerifier",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_newCertificateVerifier",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDepositDisable"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setDepositDisable"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_depositAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_disable"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMaxDepositAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMaxDepositAmount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_maxDepositAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMinDepositAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMinDepositAmount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_minDepositAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMinRollupFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMinRollupFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_minRollupFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setRelayerPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setRelayerPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_newRelayerPool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setRollerPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setRollerPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_newRollerPool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSanctionsListAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSanctionsListAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_sanction"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTransferDisable"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setTransferDisable"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_disable"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes4"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("transferDisableMap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferDisableMap"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("validateRelayer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateRelayer"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct RelayerValidateParams",
                            ),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("validateRoller"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateRoller"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct RollerValidateParams",
                            ),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("verifyCertificate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifyCertificate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct CertificateParams"
                            ),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssociatedPoolChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AssociatedPoolChanged",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deposit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AuditorPublicKeyChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AuditorPublicKeyChanged",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("publicKey"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CertificateVerifierChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CertificateVerifierChanged",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("verifier"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositDisableChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DepositDisableChanged",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deposit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("disable"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MaxDepositAmountChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MaxDepositAmountChanged",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deposit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("maxDepositAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinDepositAmountChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MinDepositAmountChanged",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deposit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("minDepositAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinRollupFeeChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MinRollupFeeChanged",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("minRollupFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayerPoolChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RelayerPoolChanged"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("relayerPool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RollerPoolChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RollerPoolChanged"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("rollerPool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RollupVerifierDisabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RollupVerifierDisabled",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("rollupSize"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RollupVerifierEnabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RollupVerifierEnabled",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("rollupSize"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SanctionsCheck"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SanctionsCheck"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SanctionsListChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SanctionsListChanged",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("list"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransactVerifierDisabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TransactVerifierDisabled",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("inputNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("outputNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransactVerifierEnabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TransactVerifierEnabled",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("inputNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("outputNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferDisableChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TransferDisableChanged",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("disable"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlBadConfirmation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AccessControlBadConfirmation",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlUnauthorizedAccount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AccessControlUnauthorizedAccount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("neededRole"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AuditorIndexError"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AuditorIndexError"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDepositAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidDepositAmount",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMystikoRegistryAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidMystikoRegistryAddress",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidNumInputs"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidNumInputs"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidRollupSize"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidRollupSize"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotChanged"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyMystikoDAO"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyMystikoDAO"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RollupSizeNotPowerOfTwo"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RollupSizeNotPowerOfTwo",),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MYSTIKOSETTINGSBASE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct MystikoSettingsBase<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoSettingsBase<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoSettingsBase<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoSettingsBase<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoSettingsBase<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MystikoSettingsBase))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MystikoSettingsBase<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MYSTIKOSETTINGSBASE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `AUDITOR_COUNT` (0xa592bd69) function
        pub fn auditor_count(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([165, 146, 189, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `associatedPool` (0x0ac0228f) function
        pub fn associated_pool(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([10, 192, 34, 143], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `certificate` (0x2dbf7b98) function
        pub fn certificate(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([45, 191, 123, 152], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `daoRegistry` (0x2a2b6ba0) function
        pub fn dao_registry(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([42, 43, 107, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositDisableMap` (0xc4b56df2) function
        pub fn deposit_disable_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([196, 181, 109, 242], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableRollupVerifier` (0x9b0a6fea) function
        pub fn disable_rollup_verifier(&self, rollup_size: u32) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 10, 111, 234], rollup_size)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableSanctionsCheck` (0xdd757c34) function
        pub fn disable_sanctions_check(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 117, 124, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableTransactVerifier` (0xc259e2e6) function
        pub fn disable_transact_verifier(
            &self,
            num_inputs: u32,
            num_outputs: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 89, 226, 230], (num_inputs, num_outputs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableRollupVerifier` (0x8bd08bf3) function
        pub fn enable_rollup_verifier(&self, rollup_size: u32) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 208, 139, 243], rollup_size)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableSanctionsCheck` (0x01dbf19f) function
        pub fn enable_sanctions_check(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 219, 241, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableTransactVerifier` (0xae034528) function
        pub fn enable_transact_verifier(
            &self,
            num_inputs: u32,
            num_outputs: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 3, 69, 40], (num_inputs, num_outputs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCertificateIssuer` (0x77bc43d6) function
        pub fn get_certificate_issuer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([119, 188, 67, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(&self, role: [u8; 32]) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCertificateCheckEnabled` (0xbc587706) function
        pub fn is_certificate_check_enabled(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([188, 88, 119, 6], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDepositDisable` (0xbb073205) function
        pub fn is_deposit_disable(
            &self,
            deposit_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([187, 7, 50, 5], deposit_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSanctioned` (0xdf592f7d) function
        pub fn is_sanctioned(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([223, 89, 47, 125], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isTransferDisable` (0xee6fb988) function
        pub fn is_transfer_disable(
            &self,
            pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([238, 111, 185, 136], pool)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxDepositAmountMap` (0x62e52338) function
        pub fn max_deposit_amount_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 229, 35, 56], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minDepositAmountMap` (0x4d8404bc) function
        pub fn min_deposit_amount_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 132, 4, 188], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minRollupFeeMap` (0xde200460) function
        pub fn min_rollup_fee_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([222, 32, 4, 96], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryAllAuditorPublicKeys` (0xc112de6c) function
        pub fn query_all_auditor_public_keys(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<::ethers::core::types::U256>> {
            self.0
                .method_hash([193, 18, 222, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryAssociatedPool` (0x41fb6979) function
        pub fn query_associated_pool(
            &self,
            deposit_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([65, 251, 105, 121], deposit_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryAuditorPublicKey` (0xdbda0829) function
        pub fn query_auditor_public_key(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([219, 218, 8, 41], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryMaxDepositAmount` (0x473a0631) function
        pub fn query_max_deposit_amount(
            &self,
            pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([71, 58, 6, 49], pool)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryMinDepositAmount` (0x55259849) function
        pub fn query_min_deposit_amount(
            &self,
            pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 37, 152, 73], pool)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryMinRollupFee` (0xc3c4bd0b) function
        pub fn query_min_rollup_fee(
            &self,
            pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([195, 196, 189, 11], pool)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryRollupVerifier` (0x2d7ea998) function
        pub fn query_rollup_verifier(
            &self,
            rollup_size: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, WrappedVerifier> {
            self.0
                .method_hash([45, 126, 169, 152], rollup_size)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryTransactVerifier` (0x85e861eb) function
        pub fn query_transact_verifier(
            &self,
            num_inputs: u32,
            num_outputs: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, WrappedVerifier> {
            self.0
                .method_hash([133, 232, 97, 235], (num_inputs, num_outputs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayerPool` (0x031e7261) function
        pub fn relayer_pool(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([3, 30, 114, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            caller_confirmation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, caller_confirmation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollerPool` (0xc48bf6bc) function
        pub fn roller_pool(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([196, 139, 246, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sanctionsCheck` (0xb1c39422) function
        pub fn sanctions_check(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 195, 148, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sanctionsList` (0xec571c6a) function
        pub fn sanctions_list(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([236, 87, 28, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAdminRole` (0x374de218) function
        pub fn set_admin_role(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 77, 226, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAssociatedPool` (0x22f910ad) function
        pub fn set_associated_pool(
            &self,
            deposit_address: ::ethers::core::types::Address,
            pool_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 249, 16, 173], (deposit_address, pool_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAuditorPublicKey` (0xffc66bd9) function
        pub fn set_auditor_public_key(
            &self,
            index: ::ethers::core::types::U256,
            public_key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 198, 107, 217], (index, public_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCertificateVerifier` (0x9e25f778) function
        pub fn set_certificate_verifier(
            &self,
            new_certificate_verifier: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 37, 247, 120], new_certificate_verifier)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDepositDisable` (0xe1896de3) function
        pub fn set_deposit_disable(
            &self,
            deposit_address: ::ethers::core::types::Address,
            disable: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 137, 109, 227], (deposit_address, disable))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxDepositAmount` (0x8bcb8e82) function
        pub fn set_max_deposit_amount(
            &self,
            pool: ::ethers::core::types::Address,
            max_deposit_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 203, 142, 130], (pool, max_deposit_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinDepositAmount` (0xdfe15bac) function
        pub fn set_min_deposit_amount(
            &self,
            pool: ::ethers::core::types::Address,
            min_deposit_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 225, 91, 172], (pool, min_deposit_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinRollupFee` (0x2504c1d8) function
        pub fn set_min_rollup_fee(
            &self,
            pool: ::ethers::core::types::Address,
            min_rollup_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 4, 193, 216], (pool, min_rollup_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRelayerPool` (0x15d2c0e8) function
        pub fn set_relayer_pool(
            &self,
            new_relayer_pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 210, 192, 232], new_relayer_pool)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRollerPool` (0x6df094b9) function
        pub fn set_roller_pool(
            &self,
            new_roller_pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 240, 148, 185], new_roller_pool)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSanctionsListAddress` (0x0d703647) function
        pub fn set_sanctions_list_address(
            &self,
            sanction: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 112, 54, 71], sanction)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTransferDisable` (0xfd8d92fa) function
        pub fn set_transfer_disable(
            &self,
            pool: ::ethers::core::types::Address,
            disable: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 141, 146, 250], (pool, disable))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(&self, interface_id: [u8; 4]) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferDisableMap` (0x1458010d) function
        pub fn transfer_disable_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([20, 88, 1, 13], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateRelayer` (0x5ee36ce9) function
        pub fn validate_relayer(
            &self,
            params: RelayerValidateParams,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([94, 227, 108, 233], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateRoller` (0x00073626) function
        pub fn validate_roller(
            &self,
            params: RollerValidateParams,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([0, 7, 54, 38], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyCertificate` (0x849e8b9f) function
        pub fn verify_certificate(
            &self,
            params: CertificateParams,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([132, 158, 139, 159], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AssociatedPoolChanged` event
        pub fn associated_pool_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AssociatedPoolChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `AuditorPublicKeyChanged` event
        pub fn auditor_public_key_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AuditorPublicKeyChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `CertificateVerifierChanged` event
        pub fn certificate_verifier_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CertificateVerifierChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `DepositDisableChanged` event
        pub fn deposit_disable_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositDisableChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `MaxDepositAmountChanged` event
        pub fn max_deposit_amount_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MaxDepositAmountChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `MinDepositAmountChanged` event
        pub fn min_deposit_amount_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MinDepositAmountChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `MinRollupFeeChanged` event
        pub fn min_rollup_fee_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MinRollupFeeChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RelayerPoolChanged` event
        pub fn relayer_pool_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RelayerPoolChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleAdminChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleGrantedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleRevokedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RollerPoolChanged` event
        pub fn roller_pool_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RollerPoolChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RollupVerifierDisabled` event
        pub fn rollup_verifier_disabled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RollupVerifierDisabledFilter> {
            self.0.event()
        }
        ///Gets the contract's `RollupVerifierEnabled` event
        pub fn rollup_verifier_enabled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RollupVerifierEnabledFilter> {
            self.0.event()
        }
        ///Gets the contract's `SanctionsCheck` event
        pub fn sanctions_check_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SanctionsCheckFilter> {
            self.0.event()
        }
        ///Gets the contract's `SanctionsListChanged` event
        pub fn sanctions_list_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SanctionsListChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `TransactVerifierDisabled` event
        pub fn transact_verifier_disabled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransactVerifierDisabledFilter> {
            self.0.event()
        }
        ///Gets the contract's `TransactVerifierEnabled` event
        pub fn transact_verifier_enabled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransactVerifierEnabledFilter> {
            self.0.event()
        }
        ///Gets the contract's `TransferDisableChanged` event
        pub fn transfer_disable_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferDisableChangedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(&self) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MystikoSettingsBaseEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MystikoSettingsBase<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccessControlBadConfirmation` with signature `AccessControlBadConfirmation()` and selector `0x6697b232`
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
    #[etherror(name = "AccessControlBadConfirmation", abi = "AccessControlBadConfirmation()")]
    pub struct AccessControlBadConfirmation;
    ///Custom Error type `AccessControlUnauthorizedAccount` with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`
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
    #[etherror(
        name = "AccessControlUnauthorizedAccount",
        abi = "AccessControlUnauthorizedAccount(address,bytes32)"
    )]
    pub struct AccessControlUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
        pub needed_role: [u8; 32],
    }
    ///Custom Error type `AuditorIndexError` with signature `AuditorIndexError()` and selector `0xc6310d14`
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
    #[etherror(name = "AuditorIndexError", abi = "AuditorIndexError()")]
    pub struct AuditorIndexError;
    ///Custom Error type `InvalidDepositAmount` with signature `InvalidDepositAmount()` and selector `0xfe9ba5cd`
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
    #[etherror(name = "InvalidDepositAmount", abi = "InvalidDepositAmount()")]
    pub struct InvalidDepositAmount;
    ///Custom Error type `InvalidMystikoRegistryAddress` with signature `InvalidMystikoRegistryAddress()` and selector `0xd758cefd`
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
    #[etherror(name = "InvalidMystikoRegistryAddress", abi = "InvalidMystikoRegistryAddress()")]
    pub struct InvalidMystikoRegistryAddress;
    ///Custom Error type `InvalidNumInputs` with signature `InvalidNumInputs()` and selector `0x8ff3959e`
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
    #[etherror(name = "InvalidNumInputs", abi = "InvalidNumInputs()")]
    pub struct InvalidNumInputs;
    ///Custom Error type `InvalidRollupSize` with signature `InvalidRollupSize()` and selector `0x26b5fd2c`
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
    #[etherror(name = "InvalidRollupSize", abi = "InvalidRollupSize()")]
    pub struct InvalidRollupSize;
    ///Custom Error type `NotChanged` with signature `NotChanged()` and selector `0x36a1c33f`
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
    #[etherror(name = "NotChanged", abi = "NotChanged()")]
    pub struct NotChanged;
    ///Custom Error type `OnlyMystikoDAO` with signature `OnlyMystikoDAO()` and selector `0x2ef792a2`
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
    #[etherror(name = "OnlyMystikoDAO", abi = "OnlyMystikoDAO()")]
    pub struct OnlyMystikoDAO;
    ///Custom Error type `RollupSizeNotPowerOfTwo` with signature `RollupSizeNotPowerOfTwo()` and selector `0x22717ff9`
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
    #[etherror(name = "RollupSizeNotPowerOfTwo", abi = "RollupSizeNotPowerOfTwo()")]
    pub struct RollupSizeNotPowerOfTwo;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoSettingsBaseErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        AuditorIndexError(AuditorIndexError),
        InvalidDepositAmount(InvalidDepositAmount),
        InvalidMystikoRegistryAddress(InvalidMystikoRegistryAddress),
        InvalidNumInputs(InvalidNumInputs),
        InvalidRollupSize(InvalidRollupSize),
        NotChanged(NotChanged),
        OnlyMystikoDAO(OnlyMystikoDAO),
        RollupSizeNotPowerOfTwo(RollupSizeNotPowerOfTwo),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MystikoSettingsBaseErrors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AccessControlBadConfirmation as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AccessControlBadConfirmation(decoded));
            }
            if let Ok(decoded) = <AccessControlUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AccessControlUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <AuditorIndexError as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AuditorIndexError(decoded));
            }
            if let Ok(decoded) = <InvalidDepositAmount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidDepositAmount(decoded));
            }
            if let Ok(decoded) = <InvalidMystikoRegistryAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidMystikoRegistryAddress(decoded));
            }
            if let Ok(decoded) = <InvalidNumInputs as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidNumInputs(decoded));
            }
            if let Ok(decoded) = <InvalidRollupSize as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidRollupSize(decoded));
            }
            if let Ok(decoded) = <NotChanged as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotChanged(decoded));
            }
            if let Ok(decoded) = <OnlyMystikoDAO as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyMystikoDAO(decoded));
            }
            if let Ok(decoded) = <RollupSizeNotPowerOfTwo as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RollupSizeNotPowerOfTwo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MystikoSettingsBaseErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessControlBadConfirmation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AccessControlUnauthorizedAccount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AuditorIndexError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidDepositAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidMystikoRegistryAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidNumInputs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidRollupSize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotChanged(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyMystikoDAO(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RollupSizeNotPowerOfTwo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MystikoSettingsBaseErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AccessControlBadConfirmation as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AccessControlUnauthorizedAccount as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AuditorIndexError as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidDepositAmount as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidMystikoRegistryAddress as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidNumInputs as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidRollupSize as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NotChanged as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <OnlyMystikoDAO as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <RollupSizeNotPowerOfTwo as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MystikoSettingsBaseErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlBadConfirmation(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccessControlUnauthorizedAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuditorIndexError(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidDepositAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMystikoRegistryAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNumInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidRollupSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotChanged(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyMystikoDAO(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupSizeNotPowerOfTwo(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MystikoSettingsBaseErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation> for MystikoSettingsBaseErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount> for MystikoSettingsBaseErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<AuditorIndexError> for MystikoSettingsBaseErrors {
        fn from(value: AuditorIndexError) -> Self {
            Self::AuditorIndexError(value)
        }
    }
    impl ::core::convert::From<InvalidDepositAmount> for MystikoSettingsBaseErrors {
        fn from(value: InvalidDepositAmount) -> Self {
            Self::InvalidDepositAmount(value)
        }
    }
    impl ::core::convert::From<InvalidMystikoRegistryAddress> for MystikoSettingsBaseErrors {
        fn from(value: InvalidMystikoRegistryAddress) -> Self {
            Self::InvalidMystikoRegistryAddress(value)
        }
    }
    impl ::core::convert::From<InvalidNumInputs> for MystikoSettingsBaseErrors {
        fn from(value: InvalidNumInputs) -> Self {
            Self::InvalidNumInputs(value)
        }
    }
    impl ::core::convert::From<InvalidRollupSize> for MystikoSettingsBaseErrors {
        fn from(value: InvalidRollupSize) -> Self {
            Self::InvalidRollupSize(value)
        }
    }
    impl ::core::convert::From<NotChanged> for MystikoSettingsBaseErrors {
        fn from(value: NotChanged) -> Self {
            Self::NotChanged(value)
        }
    }
    impl ::core::convert::From<OnlyMystikoDAO> for MystikoSettingsBaseErrors {
        fn from(value: OnlyMystikoDAO) -> Self {
            Self::OnlyMystikoDAO(value)
        }
    }
    impl ::core::convert::From<RollupSizeNotPowerOfTwo> for MystikoSettingsBaseErrors {
        fn from(value: RollupSizeNotPowerOfTwo) -> Self {
            Self::RollupSizeNotPowerOfTwo(value)
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
    #[ethevent(name = "AssociatedPoolChanged", abi = "AssociatedPoolChanged(address,address)")]
    pub struct AssociatedPoolChangedFilter {
        #[ethevent(indexed)]
        pub deposit: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
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
    #[ethevent(name = "AuditorPublicKeyChanged", abi = "AuditorPublicKeyChanged(uint256,uint256)")]
    pub struct AuditorPublicKeyChangedFilter {
        #[ethevent(indexed)]
        pub index: ::ethers::core::types::U256,
        pub public_key: ::ethers::core::types::U256,
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
    #[ethevent(name = "CertificateVerifierChanged", abi = "CertificateVerifierChanged(address)")]
    pub struct CertificateVerifierChangedFilter {
        #[ethevent(indexed)]
        pub verifier: ::ethers::core::types::Address,
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
    #[ethevent(name = "DepositDisableChanged", abi = "DepositDisableChanged(address,bool)")]
    pub struct DepositDisableChangedFilter {
        #[ethevent(indexed)]
        pub deposit: ::ethers::core::types::Address,
        pub disable: bool,
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
    #[ethevent(name = "MaxDepositAmountChanged", abi = "MaxDepositAmountChanged(address,uint256)")]
    pub struct MaxDepositAmountChangedFilter {
        #[ethevent(indexed)]
        pub deposit: ::ethers::core::types::Address,
        pub max_deposit_amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "MinDepositAmountChanged", abi = "MinDepositAmountChanged(address,uint256)")]
    pub struct MinDepositAmountChangedFilter {
        #[ethevent(indexed)]
        pub deposit: ::ethers::core::types::Address,
        pub min_deposit_amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "MinRollupFeeChanged", abi = "MinRollupFeeChanged(address,uint256)")]
    pub struct MinRollupFeeChangedFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        pub min_rollup_fee: ::ethers::core::types::U256,
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
    #[ethevent(name = "RelayerPoolChanged", abi = "RelayerPoolChanged(address)")]
    pub struct RelayerPoolChangedFilter {
        #[ethevent(indexed)]
        pub relayer_pool: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleAdminChanged", abi = "RoleAdminChanged(bytes32,bytes32,bytes32)")]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RollerPoolChanged", abi = "RollerPoolChanged(address)")]
    pub struct RollerPoolChangedFilter {
        #[ethevent(indexed)]
        pub roller_pool: ::ethers::core::types::Address,
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
    #[ethevent(name = "RollupVerifierDisabled", abi = "RollupVerifierDisabled(uint32)")]
    pub struct RollupVerifierDisabledFilter {
        pub rollup_size: u32,
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
    #[ethevent(name = "RollupVerifierEnabled", abi = "RollupVerifierEnabled(uint32)")]
    pub struct RollupVerifierEnabledFilter {
        pub rollup_size: u32,
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
    #[ethevent(name = "SanctionsCheck", abi = "SanctionsCheck(bool)")]
    pub struct SanctionsCheckFilter {
        pub state: bool,
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
    #[ethevent(name = "SanctionsListChanged", abi = "SanctionsListChanged(address)")]
    pub struct SanctionsListChangedFilter {
        pub list: ::ethers::core::types::Address,
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
    #[ethevent(name = "TransactVerifierDisabled", abi = "TransactVerifierDisabled(uint32,uint32)")]
    pub struct TransactVerifierDisabledFilter {
        pub input_number: u32,
        pub output_number: u32,
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
    #[ethevent(name = "TransactVerifierEnabled", abi = "TransactVerifierEnabled(uint32,uint32)")]
    pub struct TransactVerifierEnabledFilter {
        pub input_number: u32,
        pub output_number: u32,
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
    #[ethevent(name = "TransferDisableChanged", abi = "TransferDisableChanged(address,bool)")]
    pub struct TransferDisableChangedFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        pub disable: bool,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoSettingsBaseEvents {
        AssociatedPoolChangedFilter(AssociatedPoolChangedFilter),
        AuditorPublicKeyChangedFilter(AuditorPublicKeyChangedFilter),
        CertificateVerifierChangedFilter(CertificateVerifierChangedFilter),
        DepositDisableChangedFilter(DepositDisableChangedFilter),
        MaxDepositAmountChangedFilter(MaxDepositAmountChangedFilter),
        MinDepositAmountChangedFilter(MinDepositAmountChangedFilter),
        MinRollupFeeChangedFilter(MinRollupFeeChangedFilter),
        RelayerPoolChangedFilter(RelayerPoolChangedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        RollerPoolChangedFilter(RollerPoolChangedFilter),
        RollupVerifierDisabledFilter(RollupVerifierDisabledFilter),
        RollupVerifierEnabledFilter(RollupVerifierEnabledFilter),
        SanctionsCheckFilter(SanctionsCheckFilter),
        SanctionsListChangedFilter(SanctionsListChangedFilter),
        TransactVerifierDisabledFilter(TransactVerifierDisabledFilter),
        TransactVerifierEnabledFilter(TransactVerifierEnabledFilter),
        TransferDisableChangedFilter(TransferDisableChangedFilter),
    }
    impl ::ethers::contract::EthLogDecode for MystikoSettingsBaseEvents {
        fn decode_log(log: &::ethers::core::abi::RawLog) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssociatedPoolChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::AssociatedPoolChangedFilter(decoded));
            }
            if let Ok(decoded) = AuditorPublicKeyChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::AuditorPublicKeyChangedFilter(decoded));
            }
            if let Ok(decoded) = CertificateVerifierChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::CertificateVerifierChangedFilter(decoded));
            }
            if let Ok(decoded) = DepositDisableChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::DepositDisableChangedFilter(decoded));
            }
            if let Ok(decoded) = MaxDepositAmountChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::MaxDepositAmountChangedFilter(decoded));
            }
            if let Ok(decoded) = MinDepositAmountChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::MinDepositAmountChangedFilter(decoded));
            }
            if let Ok(decoded) = MinRollupFeeChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::MinRollupFeeChangedFilter(decoded));
            }
            if let Ok(decoded) = RelayerPoolChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::RelayerPoolChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = RollerPoolChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::RollerPoolChangedFilter(decoded));
            }
            if let Ok(decoded) = RollupVerifierDisabledFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::RollupVerifierDisabledFilter(decoded));
            }
            if let Ok(decoded) = RollupVerifierEnabledFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::RollupVerifierEnabledFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::SanctionsListChangedFilter(decoded));
            }
            if let Ok(decoded) = TransactVerifierDisabledFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::TransactVerifierDisabledFilter(decoded));
            }
            if let Ok(decoded) = TransactVerifierEnabledFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::TransactVerifierEnabledFilter(decoded));
            }
            if let Ok(decoded) = TransferDisableChangedFilter::decode_log(log) {
                return Ok(MystikoSettingsBaseEvents::TransferDisableChangedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MystikoSettingsBaseEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssociatedPoolChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuditorPublicKeyChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CertificateVerifierChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositDisableChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDepositAmountChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinDepositAmountChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinRollupFeeChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerPoolChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollerPoolChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupVerifierDisabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollupVerifierEnabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsCheckFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsListChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransactVerifierDisabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransactVerifierEnabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferDisableChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssociatedPoolChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: AssociatedPoolChangedFilter) -> Self {
            Self::AssociatedPoolChangedFilter(value)
        }
    }
    impl ::core::convert::From<AuditorPublicKeyChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: AuditorPublicKeyChangedFilter) -> Self {
            Self::AuditorPublicKeyChangedFilter(value)
        }
    }
    impl ::core::convert::From<CertificateVerifierChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: CertificateVerifierChangedFilter) -> Self {
            Self::CertificateVerifierChangedFilter(value)
        }
    }
    impl ::core::convert::From<DepositDisableChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: DepositDisableChangedFilter) -> Self {
            Self::DepositDisableChangedFilter(value)
        }
    }
    impl ::core::convert::From<MaxDepositAmountChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: MaxDepositAmountChangedFilter) -> Self {
            Self::MaxDepositAmountChangedFilter(value)
        }
    }
    impl ::core::convert::From<MinDepositAmountChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: MinDepositAmountChangedFilter) -> Self {
            Self::MinDepositAmountChangedFilter(value)
        }
    }
    impl ::core::convert::From<MinRollupFeeChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: MinRollupFeeChangedFilter) -> Self {
            Self::MinRollupFeeChangedFilter(value)
        }
    }
    impl ::core::convert::From<RelayerPoolChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: RelayerPoolChangedFilter) -> Self {
            Self::RelayerPoolChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for MystikoSettingsBaseEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for MystikoSettingsBaseEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<RollerPoolChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: RollerPoolChangedFilter) -> Self {
            Self::RollerPoolChangedFilter(value)
        }
    }
    impl ::core::convert::From<RollupVerifierDisabledFilter> for MystikoSettingsBaseEvents {
        fn from(value: RollupVerifierDisabledFilter) -> Self {
            Self::RollupVerifierDisabledFilter(value)
        }
    }
    impl ::core::convert::From<RollupVerifierEnabledFilter> for MystikoSettingsBaseEvents {
        fn from(value: RollupVerifierEnabledFilter) -> Self {
            Self::RollupVerifierEnabledFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckFilter> for MystikoSettingsBaseEvents {
        fn from(value: SanctionsCheckFilter) -> Self {
            Self::SanctionsCheckFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsListChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: SanctionsListChangedFilter) -> Self {
            Self::SanctionsListChangedFilter(value)
        }
    }
    impl ::core::convert::From<TransactVerifierDisabledFilter> for MystikoSettingsBaseEvents {
        fn from(value: TransactVerifierDisabledFilter) -> Self {
            Self::TransactVerifierDisabledFilter(value)
        }
    }
    impl ::core::convert::From<TransactVerifierEnabledFilter> for MystikoSettingsBaseEvents {
        fn from(value: TransactVerifierEnabledFilter) -> Self {
            Self::TransactVerifierEnabledFilter(value)
        }
    }
    impl ::core::convert::From<TransferDisableChangedFilter> for MystikoSettingsBaseEvents {
        fn from(value: TransferDisableChangedFilter) -> Self {
            Self::TransferDisableChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `AUDITOR_COUNT` function with signature `AUDITOR_COUNT()` and selector `0xa592bd69`
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
    #[ethcall(name = "AUDITOR_COUNT", abi = "AUDITOR_COUNT()")]
    pub struct AuditorCountCall;
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `associatedPool` function with signature `associatedPool(address)` and selector `0x0ac0228f`
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
    #[ethcall(name = "associatedPool", abi = "associatedPool(address)")]
    pub struct AssociatedPoolCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `certificate` function with signature `certificate()` and selector `0x2dbf7b98`
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
    #[ethcall(name = "certificate", abi = "certificate()")]
    pub struct CertificateCall;
    ///Container type for all input parameters for the `daoRegistry` function with signature `daoRegistry()` and selector `0x2a2b6ba0`
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
    #[ethcall(name = "daoRegistry", abi = "daoRegistry()")]
    pub struct DaoRegistryCall;
    ///Container type for all input parameters for the `depositDisableMap` function with signature `depositDisableMap(address)` and selector `0xc4b56df2`
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
    #[ethcall(name = "depositDisableMap", abi = "depositDisableMap(address)")]
    pub struct DepositDisableMapCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `disableRollupVerifier` function with signature `disableRollupVerifier(uint32)` and selector `0x9b0a6fea`
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
    #[ethcall(name = "disableRollupVerifier", abi = "disableRollupVerifier(uint32)")]
    pub struct DisableRollupVerifierCall {
        pub rollup_size: u32,
    }
    ///Container type for all input parameters for the `disableSanctionsCheck` function with signature `disableSanctionsCheck()` and selector `0xdd757c34`
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
    #[ethcall(name = "disableSanctionsCheck", abi = "disableSanctionsCheck()")]
    pub struct DisableSanctionsCheckCall;
    ///Container type for all input parameters for the `disableTransactVerifier` function with signature `disableTransactVerifier(uint32,uint32)` and selector `0xc259e2e6`
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
    #[ethcall(name = "disableTransactVerifier", abi = "disableTransactVerifier(uint32,uint32)")]
    pub struct DisableTransactVerifierCall {
        pub num_inputs: u32,
        pub num_outputs: u32,
    }
    ///Container type for all input parameters for the `enableRollupVerifier` function with signature `enableRollupVerifier(uint32)` and selector `0x8bd08bf3`
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
    #[ethcall(name = "enableRollupVerifier", abi = "enableRollupVerifier(uint32)")]
    pub struct EnableRollupVerifierCall {
        pub rollup_size: u32,
    }
    ///Container type for all input parameters for the `enableSanctionsCheck` function with signature `enableSanctionsCheck()` and selector `0x01dbf19f`
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
    #[ethcall(name = "enableSanctionsCheck", abi = "enableSanctionsCheck()")]
    pub struct EnableSanctionsCheckCall;
    ///Container type for all input parameters for the `enableTransactVerifier` function with signature `enableTransactVerifier(uint32,uint32)` and selector `0xae034528`
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
    #[ethcall(name = "enableTransactVerifier", abi = "enableTransactVerifier(uint32,uint32)")]
    pub struct EnableTransactVerifierCall {
        pub num_inputs: u32,
        pub num_outputs: u32,
    }
    ///Container type for all input parameters for the `getCertificateIssuer` function with signature `getCertificateIssuer()` and selector `0x77bc43d6`
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
    #[ethcall(name = "getCertificateIssuer", abi = "getCertificateIssuer()")]
    pub struct GetCertificateIssuerCall;
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `isDepositDisable` function with signature `isDepositDisable(address)` and selector `0xbb073205`
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
    #[ethcall(name = "isDepositDisable", abi = "isDepositDisable(address)")]
    pub struct IsDepositDisableCall {
        pub deposit_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isSanctioned` function with signature `isSanctioned(address)` and selector `0xdf592f7d`
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
    #[ethcall(name = "isSanctioned", abi = "isSanctioned(address)")]
    pub struct IsSanctionedCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isTransferDisable` function with signature `isTransferDisable(address)` and selector `0xee6fb988`
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
    #[ethcall(name = "isTransferDisable", abi = "isTransferDisable(address)")]
    pub struct IsTransferDisableCall {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maxDepositAmountMap` function with signature `maxDepositAmountMap(address)` and selector `0x62e52338`
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
    #[ethcall(name = "maxDepositAmountMap", abi = "maxDepositAmountMap(address)")]
    pub struct MaxDepositAmountMapCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `minDepositAmountMap` function with signature `minDepositAmountMap(address)` and selector `0x4d8404bc`
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
    #[ethcall(name = "minDepositAmountMap", abi = "minDepositAmountMap(address)")]
    pub struct MinDepositAmountMapCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `minRollupFeeMap` function with signature `minRollupFeeMap(address)` and selector `0xde200460`
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
    #[ethcall(name = "minRollupFeeMap", abi = "minRollupFeeMap(address)")]
    pub struct MinRollupFeeMapCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `queryAllAuditorPublicKeys` function with signature `queryAllAuditorPublicKeys()` and selector `0xc112de6c`
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
    #[ethcall(name = "queryAllAuditorPublicKeys", abi = "queryAllAuditorPublicKeys()")]
    pub struct QueryAllAuditorPublicKeysCall;
    ///Container type for all input parameters for the `queryAssociatedPool` function with signature `queryAssociatedPool(address)` and selector `0x41fb6979`
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
    #[ethcall(name = "queryAssociatedPool", abi = "queryAssociatedPool(address)")]
    pub struct QueryAssociatedPoolCall {
        pub deposit_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `queryAuditorPublicKey` function with signature `queryAuditorPublicKey(uint256)` and selector `0xdbda0829`
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
    #[ethcall(name = "queryAuditorPublicKey", abi = "queryAuditorPublicKey(uint256)")]
    pub struct QueryAuditorPublicKeyCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `queryMaxDepositAmount` function with signature `queryMaxDepositAmount(address)` and selector `0x473a0631`
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
    #[ethcall(name = "queryMaxDepositAmount", abi = "queryMaxDepositAmount(address)")]
    pub struct QueryMaxDepositAmountCall {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `queryMinDepositAmount` function with signature `queryMinDepositAmount(address)` and selector `0x55259849`
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
    #[ethcall(name = "queryMinDepositAmount", abi = "queryMinDepositAmount(address)")]
    pub struct QueryMinDepositAmountCall {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `queryMinRollupFee` function with signature `queryMinRollupFee(address)` and selector `0xc3c4bd0b`
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
    #[ethcall(name = "queryMinRollupFee", abi = "queryMinRollupFee(address)")]
    pub struct QueryMinRollupFeeCall {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `queryRollupVerifier` function with signature `queryRollupVerifier(uint32)` and selector `0x2d7ea998`
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
    #[ethcall(name = "queryRollupVerifier", abi = "queryRollupVerifier(uint32)")]
    pub struct QueryRollupVerifierCall {
        pub rollup_size: u32,
    }
    ///Container type for all input parameters for the `queryTransactVerifier` function with signature `queryTransactVerifier(uint32,uint32)` and selector `0x85e861eb`
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
    #[ethcall(name = "queryTransactVerifier", abi = "queryTransactVerifier(uint32,uint32)")]
    pub struct QueryTransactVerifierCall {
        pub num_inputs: u32,
        pub num_outputs: u32,
    }
    ///Container type for all input parameters for the `relayerPool` function with signature `relayerPool()` and selector `0x031e7261`
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
    #[ethcall(name = "relayerPool", abi = "relayerPool()")]
    pub struct RelayerPoolCall;
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub caller_confirmation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rollerPool` function with signature `rollerPool()` and selector `0xc48bf6bc`
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
    #[ethcall(name = "rollerPool", abi = "rollerPool()")]
    pub struct RollerPoolCall;
    ///Container type for all input parameters for the `sanctionsCheck` function with signature `sanctionsCheck()` and selector `0xb1c39422`
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
    #[ethcall(name = "sanctionsCheck", abi = "sanctionsCheck()")]
    pub struct SanctionsCheckCall;
    ///Container type for all input parameters for the `sanctionsList` function with signature `sanctionsList()` and selector `0xec571c6a`
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
    #[ethcall(name = "sanctionsList", abi = "sanctionsList()")]
    pub struct SanctionsListCall;
    ///Container type for all input parameters for the `setAdminRole` function with signature `setAdminRole()` and selector `0x374de218`
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
    #[ethcall(name = "setAdminRole", abi = "setAdminRole()")]
    pub struct SetAdminRoleCall;
    ///Container type for all input parameters for the `setAssociatedPool` function with signature `setAssociatedPool(address,address)` and selector `0x22f910ad`
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
    #[ethcall(name = "setAssociatedPool", abi = "setAssociatedPool(address,address)")]
    pub struct SetAssociatedPoolCall {
        pub deposit_address: ::ethers::core::types::Address,
        pub pool_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setAuditorPublicKey` function with signature `setAuditorPublicKey(uint256,uint256)` and selector `0xffc66bd9`
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
    #[ethcall(name = "setAuditorPublicKey", abi = "setAuditorPublicKey(uint256,uint256)")]
    pub struct SetAuditorPublicKeyCall {
        pub index: ::ethers::core::types::U256,
        pub public_key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setCertificateVerifier` function with signature `setCertificateVerifier(address)` and selector `0x9e25f778`
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
    #[ethcall(name = "setCertificateVerifier", abi = "setCertificateVerifier(address)")]
    pub struct SetCertificateVerifierCall {
        pub new_certificate_verifier: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setDepositDisable` function with signature `setDepositDisable(address,bool)` and selector `0xe1896de3`
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
    #[ethcall(name = "setDepositDisable", abi = "setDepositDisable(address,bool)")]
    pub struct SetDepositDisableCall {
        pub deposit_address: ::ethers::core::types::Address,
        pub disable: bool,
    }
    ///Container type for all input parameters for the `setMaxDepositAmount` function with signature `setMaxDepositAmount(address,uint256)` and selector `0x8bcb8e82`
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
    #[ethcall(name = "setMaxDepositAmount", abi = "setMaxDepositAmount(address,uint256)")]
    pub struct SetMaxDepositAmountCall {
        pub pool: ::ethers::core::types::Address,
        pub max_deposit_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setMinDepositAmount` function with signature `setMinDepositAmount(address,uint256)` and selector `0xdfe15bac`
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
    #[ethcall(name = "setMinDepositAmount", abi = "setMinDepositAmount(address,uint256)")]
    pub struct SetMinDepositAmountCall {
        pub pool: ::ethers::core::types::Address,
        pub min_deposit_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setMinRollupFee` function with signature `setMinRollupFee(address,uint256)` and selector `0x2504c1d8`
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
    #[ethcall(name = "setMinRollupFee", abi = "setMinRollupFee(address,uint256)")]
    pub struct SetMinRollupFeeCall {
        pub pool: ::ethers::core::types::Address,
        pub min_rollup_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setRelayerPool` function with signature `setRelayerPool(address)` and selector `0x15d2c0e8`
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
    #[ethcall(name = "setRelayerPool", abi = "setRelayerPool(address)")]
    pub struct SetRelayerPoolCall {
        pub new_relayer_pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setRollerPool` function with signature `setRollerPool(address)` and selector `0x6df094b9`
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
    #[ethcall(name = "setRollerPool", abi = "setRollerPool(address)")]
    pub struct SetRollerPoolCall {
        pub new_roller_pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setSanctionsListAddress` function with signature `setSanctionsListAddress(address)` and selector `0x0d703647`
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
    #[ethcall(name = "setSanctionsListAddress", abi = "setSanctionsListAddress(address)")]
    pub struct SetSanctionsListAddressCall {
        pub sanction: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setTransferDisable` function with signature `setTransferDisable(address,bool)` and selector `0xfd8d92fa`
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
    #[ethcall(name = "setTransferDisable", abi = "setTransferDisable(address,bool)")]
    pub struct SetTransferDisableCall {
        pub pool: ::ethers::core::types::Address,
        pub disable: bool,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `transferDisableMap` function with signature `transferDisableMap(address)` and selector `0x1458010d`
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
    #[ethcall(name = "transferDisableMap", abi = "transferDisableMap(address)")]
    pub struct TransferDisableMapCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `validateRelayer` function with signature `validateRelayer((address,address))` and selector `0x5ee36ce9`
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
    #[ethcall(name = "validateRelayer", abi = "validateRelayer((address,address))")]
    pub struct ValidateRelayerCall {
        pub params: RelayerValidateParams,
    }
    ///Container type for all input parameters for the `validateRoller` function with signature `validateRoller((address,address,uint256,uint256,uint256))` and selector `0x00073626`
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
        name = "validateRoller",
        abi = "validateRoller((address,address,uint256,uint256,uint256))"
    )]
    pub struct ValidateRollerCall {
        pub params: RollerValidateParams,
    }
    ///Container type for all input parameters for the `verifyCertificate` function with signature `verifyCertificate((address,address,uint256,bytes))` and selector `0x849e8b9f`
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
        name = "verifyCertificate",
        abi = "verifyCertificate((address,address,uint256,bytes))"
    )]
    pub struct VerifyCertificateCall {
        pub params: CertificateParams,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoSettingsBaseCalls {
        AuditorCount(AuditorCountCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        AssociatedPool(AssociatedPoolCall),
        Certificate(CertificateCall),
        DaoRegistry(DaoRegistryCall),
        DepositDisableMap(DepositDisableMapCall),
        DisableRollupVerifier(DisableRollupVerifierCall),
        DisableSanctionsCheck(DisableSanctionsCheckCall),
        DisableTransactVerifier(DisableTransactVerifierCall),
        EnableRollupVerifier(EnableRollupVerifierCall),
        EnableSanctionsCheck(EnableSanctionsCheckCall),
        EnableTransactVerifier(EnableTransactVerifierCall),
        GetCertificateIssuer(GetCertificateIssuerCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IsCertificateCheckEnabled(IsCertificateCheckEnabledCall),
        IsDepositDisable(IsDepositDisableCall),
        IsSanctioned(IsSanctionedCall),
        IsTransferDisable(IsTransferDisableCall),
        MaxDepositAmountMap(MaxDepositAmountMapCall),
        MinDepositAmountMap(MinDepositAmountMapCall),
        MinRollupFeeMap(MinRollupFeeMapCall),
        QueryAllAuditorPublicKeys(QueryAllAuditorPublicKeysCall),
        QueryAssociatedPool(QueryAssociatedPoolCall),
        QueryAuditorPublicKey(QueryAuditorPublicKeyCall),
        QueryMaxDepositAmount(QueryMaxDepositAmountCall),
        QueryMinDepositAmount(QueryMinDepositAmountCall),
        QueryMinRollupFee(QueryMinRollupFeeCall),
        QueryRollupVerifier(QueryRollupVerifierCall),
        QueryTransactVerifier(QueryTransactVerifierCall),
        RelayerPool(RelayerPoolCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RollerPool(RollerPoolCall),
        SanctionsCheck(SanctionsCheckCall),
        SanctionsList(SanctionsListCall),
        SetAdminRole(SetAdminRoleCall),
        SetAssociatedPool(SetAssociatedPoolCall),
        SetAuditorPublicKey(SetAuditorPublicKeyCall),
        SetCertificateVerifier(SetCertificateVerifierCall),
        SetDepositDisable(SetDepositDisableCall),
        SetMaxDepositAmount(SetMaxDepositAmountCall),
        SetMinDepositAmount(SetMinDepositAmountCall),
        SetMinRollupFee(SetMinRollupFeeCall),
        SetRelayerPool(SetRelayerPoolCall),
        SetRollerPool(SetRollerPoolCall),
        SetSanctionsListAddress(SetSanctionsListAddressCall),
        SetTransferDisable(SetTransferDisableCall),
        SupportsInterface(SupportsInterfaceCall),
        TransferDisableMap(TransferDisableMapCall),
        ValidateRelayer(ValidateRelayerCall),
        ValidateRoller(ValidateRollerCall),
        VerifyCertificate(VerifyCertificateCall),
    }
    impl ::ethers::core::abi::AbiDecode for MystikoSettingsBaseCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AuditorCountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AuditorCount(decoded));
            }
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <AssociatedPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssociatedPool(decoded));
            }
            if let Ok(decoded) = <CertificateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Certificate(decoded));
            }
            if let Ok(decoded) = <DaoRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DaoRegistry(decoded));
            }
            if let Ok(decoded) = <DepositDisableMapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositDisableMap(decoded));
            }
            if let Ok(decoded) = <DisableRollupVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableRollupVerifier(decoded));
            }
            if let Ok(decoded) = <DisableSanctionsCheckCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableSanctionsCheck(decoded));
            }
            if let Ok(decoded) = <DisableTransactVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableTransactVerifier(decoded));
            }
            if let Ok(decoded) = <EnableRollupVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableRollupVerifier(decoded));
            }
            if let Ok(decoded) = <EnableSanctionsCheckCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableSanctionsCheck(decoded));
            }
            if let Ok(decoded) = <EnableTransactVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableTransactVerifier(decoded));
            }
            if let Ok(decoded) = <GetCertificateIssuerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCertificateIssuer(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <IsCertificateCheckEnabledCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsCertificateCheckEnabled(decoded));
            }
            if let Ok(decoded) = <IsDepositDisableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsDepositDisable(decoded));
            }
            if let Ok(decoded) = <IsSanctionedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsSanctioned(decoded));
            }
            if let Ok(decoded) = <IsTransferDisableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsTransferDisable(decoded));
            }
            if let Ok(decoded) = <MaxDepositAmountMapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxDepositAmountMap(decoded));
            }
            if let Ok(decoded) = <MinDepositAmountMapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinDepositAmountMap(decoded));
            }
            if let Ok(decoded) = <MinRollupFeeMapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinRollupFeeMap(decoded));
            }
            if let Ok(decoded) = <QueryAllAuditorPublicKeysCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryAllAuditorPublicKeys(decoded));
            }
            if let Ok(decoded) = <QueryAssociatedPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryAssociatedPool(decoded));
            }
            if let Ok(decoded) = <QueryAuditorPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryAuditorPublicKey(decoded));
            }
            if let Ok(decoded) = <QueryMaxDepositAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryMaxDepositAmount(decoded));
            }
            if let Ok(decoded) = <QueryMinDepositAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryMinDepositAmount(decoded));
            }
            if let Ok(decoded) = <QueryMinRollupFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryMinRollupFee(decoded));
            }
            if let Ok(decoded) = <QueryRollupVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryRollupVerifier(decoded));
            }
            if let Ok(decoded) = <QueryTransactVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryTransactVerifier(decoded));
            }
            if let Ok(decoded) = <RelayerPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RelayerPool(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <RollerPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RollerPool(decoded));
            }
            if let Ok(decoded) = <SanctionsCheckCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionsCheck(decoded));
            }
            if let Ok(decoded) = <SanctionsListCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionsList(decoded));
            }
            if let Ok(decoded) = <SetAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAdminRole(decoded));
            }
            if let Ok(decoded) = <SetAssociatedPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAssociatedPool(decoded));
            }
            if let Ok(decoded) = <SetAuditorPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAuditorPublicKey(decoded));
            }
            if let Ok(decoded) = <SetCertificateVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetCertificateVerifier(decoded));
            }
            if let Ok(decoded) = <SetDepositDisableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDepositDisable(decoded));
            }
            if let Ok(decoded) = <SetMaxDepositAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMaxDepositAmount(decoded));
            }
            if let Ok(decoded) = <SetMinDepositAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMinDepositAmount(decoded));
            }
            if let Ok(decoded) = <SetMinRollupFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMinRollupFee(decoded));
            }
            if let Ok(decoded) = <SetRelayerPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRelayerPool(decoded));
            }
            if let Ok(decoded) = <SetRollerPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRollerPool(decoded));
            }
            if let Ok(decoded) = <SetSanctionsListAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSanctionsListAddress(decoded));
            }
            if let Ok(decoded) = <SetTransferDisableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTransferDisable(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <TransferDisableMapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferDisableMap(decoded));
            }
            if let Ok(decoded) = <ValidateRelayerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ValidateRelayer(decoded));
            }
            if let Ok(decoded) = <ValidateRollerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ValidateRoller(decoded));
            }
            if let Ok(decoded) = <VerifyCertificateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VerifyCertificate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MystikoSettingsBaseCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AuditorCount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssociatedPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Certificate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DaoRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositDisableMap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DisableRollupVerifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DisableSanctionsCheck(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DisableTransactVerifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnableRollupVerifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnableSanctionsCheck(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnableTransactVerifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCertificateIssuer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRoleAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GrantRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsCertificateCheckEnabled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDepositDisable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsSanctioned(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsTransferDisable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxDepositAmountMap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinDepositAmountMap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinRollupFeeMap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryAllAuditorPublicKeys(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryAssociatedPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryAuditorPublicKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryMaxDepositAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryMinDepositAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryMinRollupFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryRollupVerifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryTransactVerifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RelayerPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RollerPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SanctionsCheck(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SanctionsList(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAssociatedPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAuditorPublicKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetCertificateVerifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetDepositDisable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMaxDepositAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMinDepositAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMinRollupFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetRelayerPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetRollerPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSanctionsListAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetTransferDisable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferDisableMap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateRelayer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateRoller(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyCertificate(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MystikoSettingsBaseCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AuditorCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssociatedPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Certificate(element) => ::core::fmt::Display::fmt(element, f),
                Self::DaoRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositDisableMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableRollupVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableSanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableTransactVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableRollupVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableSanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableTransactVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCertificateIssuer(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsCertificateCheckEnabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDepositDisable(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSanctioned(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsTransferDisable(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDepositAmountMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinDepositAmountMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinRollupFeeMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryAllAuditorPublicKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryAssociatedPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryAuditorPublicKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryMaxDepositAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryMinDepositAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryRollupVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryTransactVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollerPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAssociatedPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuditorPublicKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCertificateVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositDisable(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMaxDepositAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinDepositAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinRollupFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRelayerPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRollerPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSanctionsListAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTransferDisable(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferDisableMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateRelayer(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateRoller(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyCertificate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AuditorCountCall> for MystikoSettingsBaseCalls {
        fn from(value: AuditorCountCall) -> Self {
            Self::AuditorCount(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for MystikoSettingsBaseCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<AssociatedPoolCall> for MystikoSettingsBaseCalls {
        fn from(value: AssociatedPoolCall) -> Self {
            Self::AssociatedPool(value)
        }
    }
    impl ::core::convert::From<CertificateCall> for MystikoSettingsBaseCalls {
        fn from(value: CertificateCall) -> Self {
            Self::Certificate(value)
        }
    }
    impl ::core::convert::From<DaoRegistryCall> for MystikoSettingsBaseCalls {
        fn from(value: DaoRegistryCall) -> Self {
            Self::DaoRegistry(value)
        }
    }
    impl ::core::convert::From<DepositDisableMapCall> for MystikoSettingsBaseCalls {
        fn from(value: DepositDisableMapCall) -> Self {
            Self::DepositDisableMap(value)
        }
    }
    impl ::core::convert::From<DisableRollupVerifierCall> for MystikoSettingsBaseCalls {
        fn from(value: DisableRollupVerifierCall) -> Self {
            Self::DisableRollupVerifier(value)
        }
    }
    impl ::core::convert::From<DisableSanctionsCheckCall> for MystikoSettingsBaseCalls {
        fn from(value: DisableSanctionsCheckCall) -> Self {
            Self::DisableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<DisableTransactVerifierCall> for MystikoSettingsBaseCalls {
        fn from(value: DisableTransactVerifierCall) -> Self {
            Self::DisableTransactVerifier(value)
        }
    }
    impl ::core::convert::From<EnableRollupVerifierCall> for MystikoSettingsBaseCalls {
        fn from(value: EnableRollupVerifierCall) -> Self {
            Self::EnableRollupVerifier(value)
        }
    }
    impl ::core::convert::From<EnableSanctionsCheckCall> for MystikoSettingsBaseCalls {
        fn from(value: EnableSanctionsCheckCall) -> Self {
            Self::EnableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<EnableTransactVerifierCall> for MystikoSettingsBaseCalls {
        fn from(value: EnableTransactVerifierCall) -> Self {
            Self::EnableTransactVerifier(value)
        }
    }
    impl ::core::convert::From<GetCertificateIssuerCall> for MystikoSettingsBaseCalls {
        fn from(value: GetCertificateIssuerCall) -> Self {
            Self::GetCertificateIssuer(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for MystikoSettingsBaseCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for MystikoSettingsBaseCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for MystikoSettingsBaseCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IsCertificateCheckEnabledCall> for MystikoSettingsBaseCalls {
        fn from(value: IsCertificateCheckEnabledCall) -> Self {
            Self::IsCertificateCheckEnabled(value)
        }
    }
    impl ::core::convert::From<IsDepositDisableCall> for MystikoSettingsBaseCalls {
        fn from(value: IsDepositDisableCall) -> Self {
            Self::IsDepositDisable(value)
        }
    }
    impl ::core::convert::From<IsSanctionedCall> for MystikoSettingsBaseCalls {
        fn from(value: IsSanctionedCall) -> Self {
            Self::IsSanctioned(value)
        }
    }
    impl ::core::convert::From<IsTransferDisableCall> for MystikoSettingsBaseCalls {
        fn from(value: IsTransferDisableCall) -> Self {
            Self::IsTransferDisable(value)
        }
    }
    impl ::core::convert::From<MaxDepositAmountMapCall> for MystikoSettingsBaseCalls {
        fn from(value: MaxDepositAmountMapCall) -> Self {
            Self::MaxDepositAmountMap(value)
        }
    }
    impl ::core::convert::From<MinDepositAmountMapCall> for MystikoSettingsBaseCalls {
        fn from(value: MinDepositAmountMapCall) -> Self {
            Self::MinDepositAmountMap(value)
        }
    }
    impl ::core::convert::From<MinRollupFeeMapCall> for MystikoSettingsBaseCalls {
        fn from(value: MinRollupFeeMapCall) -> Self {
            Self::MinRollupFeeMap(value)
        }
    }
    impl ::core::convert::From<QueryAllAuditorPublicKeysCall> for MystikoSettingsBaseCalls {
        fn from(value: QueryAllAuditorPublicKeysCall) -> Self {
            Self::QueryAllAuditorPublicKeys(value)
        }
    }
    impl ::core::convert::From<QueryAssociatedPoolCall> for MystikoSettingsBaseCalls {
        fn from(value: QueryAssociatedPoolCall) -> Self {
            Self::QueryAssociatedPool(value)
        }
    }
    impl ::core::convert::From<QueryAuditorPublicKeyCall> for MystikoSettingsBaseCalls {
        fn from(value: QueryAuditorPublicKeyCall) -> Self {
            Self::QueryAuditorPublicKey(value)
        }
    }
    impl ::core::convert::From<QueryMaxDepositAmountCall> for MystikoSettingsBaseCalls {
        fn from(value: QueryMaxDepositAmountCall) -> Self {
            Self::QueryMaxDepositAmount(value)
        }
    }
    impl ::core::convert::From<QueryMinDepositAmountCall> for MystikoSettingsBaseCalls {
        fn from(value: QueryMinDepositAmountCall) -> Self {
            Self::QueryMinDepositAmount(value)
        }
    }
    impl ::core::convert::From<QueryMinRollupFeeCall> for MystikoSettingsBaseCalls {
        fn from(value: QueryMinRollupFeeCall) -> Self {
            Self::QueryMinRollupFee(value)
        }
    }
    impl ::core::convert::From<QueryRollupVerifierCall> for MystikoSettingsBaseCalls {
        fn from(value: QueryRollupVerifierCall) -> Self {
            Self::QueryRollupVerifier(value)
        }
    }
    impl ::core::convert::From<QueryTransactVerifierCall> for MystikoSettingsBaseCalls {
        fn from(value: QueryTransactVerifierCall) -> Self {
            Self::QueryTransactVerifier(value)
        }
    }
    impl ::core::convert::From<RelayerPoolCall> for MystikoSettingsBaseCalls {
        fn from(value: RelayerPoolCall) -> Self {
            Self::RelayerPool(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for MystikoSettingsBaseCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for MystikoSettingsBaseCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RollerPoolCall> for MystikoSettingsBaseCalls {
        fn from(value: RollerPoolCall) -> Self {
            Self::RollerPool(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckCall> for MystikoSettingsBaseCalls {
        fn from(value: SanctionsCheckCall) -> Self {
            Self::SanctionsCheck(value)
        }
    }
    impl ::core::convert::From<SanctionsListCall> for MystikoSettingsBaseCalls {
        fn from(value: SanctionsListCall) -> Self {
            Self::SanctionsList(value)
        }
    }
    impl ::core::convert::From<SetAdminRoleCall> for MystikoSettingsBaseCalls {
        fn from(value: SetAdminRoleCall) -> Self {
            Self::SetAdminRole(value)
        }
    }
    impl ::core::convert::From<SetAssociatedPoolCall> for MystikoSettingsBaseCalls {
        fn from(value: SetAssociatedPoolCall) -> Self {
            Self::SetAssociatedPool(value)
        }
    }
    impl ::core::convert::From<SetAuditorPublicKeyCall> for MystikoSettingsBaseCalls {
        fn from(value: SetAuditorPublicKeyCall) -> Self {
            Self::SetAuditorPublicKey(value)
        }
    }
    impl ::core::convert::From<SetCertificateVerifierCall> for MystikoSettingsBaseCalls {
        fn from(value: SetCertificateVerifierCall) -> Self {
            Self::SetCertificateVerifier(value)
        }
    }
    impl ::core::convert::From<SetDepositDisableCall> for MystikoSettingsBaseCalls {
        fn from(value: SetDepositDisableCall) -> Self {
            Self::SetDepositDisable(value)
        }
    }
    impl ::core::convert::From<SetMaxDepositAmountCall> for MystikoSettingsBaseCalls {
        fn from(value: SetMaxDepositAmountCall) -> Self {
            Self::SetMaxDepositAmount(value)
        }
    }
    impl ::core::convert::From<SetMinDepositAmountCall> for MystikoSettingsBaseCalls {
        fn from(value: SetMinDepositAmountCall) -> Self {
            Self::SetMinDepositAmount(value)
        }
    }
    impl ::core::convert::From<SetMinRollupFeeCall> for MystikoSettingsBaseCalls {
        fn from(value: SetMinRollupFeeCall) -> Self {
            Self::SetMinRollupFee(value)
        }
    }
    impl ::core::convert::From<SetRelayerPoolCall> for MystikoSettingsBaseCalls {
        fn from(value: SetRelayerPoolCall) -> Self {
            Self::SetRelayerPool(value)
        }
    }
    impl ::core::convert::From<SetRollerPoolCall> for MystikoSettingsBaseCalls {
        fn from(value: SetRollerPoolCall) -> Self {
            Self::SetRollerPool(value)
        }
    }
    impl ::core::convert::From<SetSanctionsListAddressCall> for MystikoSettingsBaseCalls {
        fn from(value: SetSanctionsListAddressCall) -> Self {
            Self::SetSanctionsListAddress(value)
        }
    }
    impl ::core::convert::From<SetTransferDisableCall> for MystikoSettingsBaseCalls {
        fn from(value: SetTransferDisableCall) -> Self {
            Self::SetTransferDisable(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for MystikoSettingsBaseCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TransferDisableMapCall> for MystikoSettingsBaseCalls {
        fn from(value: TransferDisableMapCall) -> Self {
            Self::TransferDisableMap(value)
        }
    }
    impl ::core::convert::From<ValidateRelayerCall> for MystikoSettingsBaseCalls {
        fn from(value: ValidateRelayerCall) -> Self {
            Self::ValidateRelayer(value)
        }
    }
    impl ::core::convert::From<ValidateRollerCall> for MystikoSettingsBaseCalls {
        fn from(value: ValidateRollerCall) -> Self {
            Self::ValidateRoller(value)
        }
    }
    impl ::core::convert::From<VerifyCertificateCall> for MystikoSettingsBaseCalls {
        fn from(value: VerifyCertificateCall) -> Self {
            Self::VerifyCertificate(value)
        }
    }
    ///Container type for all return fields from the `AUDITOR_COUNT` function with signature `AUDITOR_COUNT()` and selector `0xa592bd69`
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
    pub struct AuditorCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `associatedPool` function with signature `associatedPool(address)` and selector `0x0ac0228f`
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
    pub struct AssociatedPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `certificate` function with signature `certificate()` and selector `0x2dbf7b98`
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
    pub struct CertificateReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `daoRegistry` function with signature `daoRegistry()` and selector `0x2a2b6ba0`
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
    pub struct DaoRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `depositDisableMap` function with signature `depositDisableMap(address)` and selector `0xc4b56df2`
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
    pub struct DepositDisableMapReturn(pub bool);
    ///Container type for all return fields from the `getCertificateIssuer` function with signature `getCertificateIssuer()` and selector `0x77bc43d6`
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
    pub struct GetCertificateIssuerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
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
    ///Container type for all return fields from the `isDepositDisable` function with signature `isDepositDisable(address)` and selector `0xbb073205`
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
    pub struct IsDepositDisableReturn(pub bool);
    ///Container type for all return fields from the `isSanctioned` function with signature `isSanctioned(address)` and selector `0xdf592f7d`
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
    pub struct IsSanctionedReturn(pub bool);
    ///Container type for all return fields from the `isTransferDisable` function with signature `isTransferDisable(address)` and selector `0xee6fb988`
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
    pub struct IsTransferDisableReturn(pub bool);
    ///Container type for all return fields from the `maxDepositAmountMap` function with signature `maxDepositAmountMap(address)` and selector `0x62e52338`
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
    pub struct MaxDepositAmountMapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minDepositAmountMap` function with signature `minDepositAmountMap(address)` and selector `0x4d8404bc`
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
    pub struct MinDepositAmountMapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minRollupFeeMap` function with signature `minRollupFeeMap(address)` and selector `0xde200460`
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
    pub struct MinRollupFeeMapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `queryAllAuditorPublicKeys` function with signature `queryAllAuditorPublicKeys()` and selector `0xc112de6c`
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
    pub struct QueryAllAuditorPublicKeysReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `queryAssociatedPool` function with signature `queryAssociatedPool(address)` and selector `0x41fb6979`
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
    pub struct QueryAssociatedPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `queryAuditorPublicKey` function with signature `queryAuditorPublicKey(uint256)` and selector `0xdbda0829`
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
    pub struct QueryAuditorPublicKeyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `queryMaxDepositAmount` function with signature `queryMaxDepositAmount(address)` and selector `0x473a0631`
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
    pub struct QueryMaxDepositAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `queryMinDepositAmount` function with signature `queryMinDepositAmount(address)` and selector `0x55259849`
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
    pub struct QueryMinDepositAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `queryMinRollupFee` function with signature `queryMinRollupFee(address)` and selector `0xc3c4bd0b`
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
    pub struct QueryMinRollupFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `queryRollupVerifier` function with signature `queryRollupVerifier(uint32)` and selector `0x2d7ea998`
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
    pub struct QueryRollupVerifierReturn(pub WrappedVerifier);
    ///Container type for all return fields from the `queryTransactVerifier` function with signature `queryTransactVerifier(uint32,uint32)` and selector `0x85e861eb`
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
    pub struct QueryTransactVerifierReturn(pub WrappedVerifier);
    ///Container type for all return fields from the `relayerPool` function with signature `relayerPool()` and selector `0x031e7261`
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
    pub struct RelayerPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rollerPool` function with signature `rollerPool()` and selector `0xc48bf6bc`
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
    pub struct RollerPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `sanctionsCheck` function with signature `sanctionsCheck()` and selector `0xb1c39422`
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
    pub struct SanctionsCheckReturn(pub bool);
    ///Container type for all return fields from the `sanctionsList` function with signature `sanctionsList()` and selector `0xec571c6a`
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
    pub struct SanctionsListReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `transferDisableMap` function with signature `transferDisableMap(address)` and selector `0x1458010d`
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
    pub struct TransferDisableMapReturn(pub bool);
    ///Container type for all return fields from the `validateRelayer` function with signature `validateRelayer((address,address))` and selector `0x5ee36ce9`
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
    pub struct ValidateRelayerReturn(pub bool);
    ///Container type for all return fields from the `validateRoller` function with signature `validateRoller((address,address,uint256,uint256,uint256))` and selector `0x00073626`
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
    pub struct ValidateRollerReturn(pub bool);
    ///Container type for all return fields from the `verifyCertificate` function with signature `verifyCertificate((address,address,uint256,bytes))` and selector `0x849e8b9f`
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
    pub struct VerifyCertificateReturn(pub bool);
}
