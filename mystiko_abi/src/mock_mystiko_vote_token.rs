pub use mock_mystiko_vote_token::*;
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
pub mod mock_mystiko_vote_token {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_xzk"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("contract IERC20"),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CLOCK_MODE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("CLOCK_MODE"),
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
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
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
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allowance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("approve"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("checkpoints"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkpoints"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pos"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(208usize),
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct Checkpoints.Checkpoint208",
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("clock"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint48"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("delegate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("delegatee"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegateBySig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegateBySig"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("delegatee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint8"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("r"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegates"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("depositFor"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositFor"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eip712Domain"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("eip712Domain"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fields"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(1usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes1"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("name"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("version"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("verifyingContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("salt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("extensions"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256[]"
                                ),),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPastTotalSupply"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPastTotalSupply"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("timepoint"),
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
                    ::std::borrow::ToOwned::to_owned("getPastVotes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPastVotes"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("timepoint"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("getVotes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getVotes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("name"),
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
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nonces"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_owner"),
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
                    ::std::borrow::ToOwned::to_owned("numCheckpoints"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("numCheckpoints"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint32"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("permit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deadline"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint8"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("r"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("underlying"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("underlying"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "contract IERC20"
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawTo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawTo"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Approval"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegateChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DelegateChanged"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("delegator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("fromDelegate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("toDelegate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegateVotesChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DelegateVotesChanged",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("delegate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousVotes"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newVotes"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EIP712DomainChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EIP712DomainChanged",),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                    ::std::borrow::ToOwned::to_owned("CheckpointUnorderedInsertion"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CheckpointUnorderedInsertion",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignature",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureLength"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureLength",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("length"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureS"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureS",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("s"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20ExceededSafeSupply"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20ExceededSafeSupply",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("increasedSupply"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cap"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientAllowance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InsufficientAllowance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("allowance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("needed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InsufficientBalance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("needed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidApprover"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InvalidApprover",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("approver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidReceiver"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InvalidReceiver",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("receiver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("sender"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSpender"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InvalidSpender",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("spender"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidUnderlying"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20InvalidUnderlying",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC2612ExpiredSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC2612ExpiredSignature",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("deadline"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC2612InvalidSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC2612InvalidSigner",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC5805FutureLookup"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC5805FutureLookup",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("timepoint"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("clock"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint48"),),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC6372InconsistentClock"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC6372InconsistentClock",),
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
                    ::std::borrow::ToOwned::to_owned("InvalidAccountNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidAccountNonce",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("currentNonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeCastOverflowedUintDowncast"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SafeCastOverflowedUintDowncast",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("bits"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint8"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("StringTooLong"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("StringTooLong"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("str"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VotesExpiredSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("VotesExpiredSignature",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("expiry"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKMYSTIKOVOTETOKEN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\x80`@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@Qa'}8\x03\x80a'}\x839\x81\x01`@\x81\x90Ra\x000\x91a\x02GV[\x80`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q&\xBC\xB9\xBA4\xB5\xB7\x90+7\xBA2\x90*7\xB5\xB2\xB7`q\x1B\x81RP\x80`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q&\xBC\xB9\xBA4\xB5\xB7\x90+7\xBA2\x90*7\xB5\xB2\xB7`q\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cvXZK`\xE0\x1B\x81RP\x81`\x03\x90\x81a\0\xCE\x91\x90a\x03\x16V[P`\x04a\0\xDB\x82\x82a\x03\x16V[Pa\0\xEB\x91P\x83\x90P`\x05a\x01\xD6V[a\x01 Ra\0\xFA\x81`\x06a\x01\xD6V[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Ra\x01\x87`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0\x81\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x03\x90Pa\x01\xC3W`@QcC\x8Do\xE3`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16a\x01`RPa\x04FV[`\0` \x83Q\x10\x15a\x01\xF2Wa\x01\xEB\x83a\x02\tV[\x90Pa\x02\x03V[\x81a\x01\xFD\x84\x82a\x03\x16V[P`\xFF\x90P[\x92\x91PPV[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15a\x024W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\x01\xBA\x91\x90a\x03\xD4V[\x80Qa\x02?\x82a\x04\"V[\x17\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x02YW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02pW`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x02\xA1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x02\xC1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x03\x11W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x02\xEEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x03\x0EW`\0\x81U`\x01\x01a\x02\xFAV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03/Wa\x03/a\x02wV[a\x03C\x81a\x03=\x84Ta\x02\x8DV[\x84a\x02\xC7V[` `\x1F\x82\x11`\x01\x81\x14a\x03wW`\0\x83\x15a\x03_WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x03\x0EV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x03\xA7W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x03\x87V[P\x84\x82\x10\x15a\x03\xC5W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\x04\x02W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x03\xE5V[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x02\xC1W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\"\xBDa\x04\xC0`\09`\0\x81\x81a\x02\xD9\x01R\x81\x81a\x05g\x01R\x81\x81a\x06\x1A\x01Ra\x0B\xFF\x01R`\0a\x0F\x84\x01R`\0a\x0FW\x01R`\0a\r`\x01R`\0a\r8\x01R`\0a\x0C\x93\x01R`\0a\x0C\xBD\x01R`\0a\x0C\xE7\x01Ra\"\xBD`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8EW`\x005`\xE0\x1C\x80co\xCF\xFFE\x11a\0\xDEW\x80c\x95\xD8\x9BA\x11a\0\x97W\x80c\xC3\xCD\xA5 \x11a\0qW\x80c\xC3\xCD\xA5 \x14a\x03\xD3W\x80c\xD5\x05\xAC\xCF\x14a\x03\xE6W\x80c\xDDb\xED>\x14a\x03\xF9W\x80c\xF1\x12~\xD8\x14a\x042W`\0\x80\xFD[\x80c\x95\xD8\x9BA\x14a\x03\xA5W\x80c\x9A\xB2N\xB0\x14a\x03\xADW\x80c\xA9\x05\x9C\xBB\x14a\x03\xC0W`\0\x80\xFD[\x80co\xCF\xFFE\x14a\x02\xFDW\x80cp\xA0\x821\x14a\x03%W\x80c~\xCE\xBE\0\x14a\x03NW\x80c\x84\xB0\x19n\x14a\x03aW\x80c\x8ES\x9E\x8C\x14a\x03|W\x80c\x91\xDD\xAD\xF4\x14a\x03\x8FW`\0\x80\xFD[\x80c1<\xE5g\x11a\x01KW\x80cK\xF5\xD7\xE9\x11a\x01%W\x80cK\xF5\xD7\xE9\x14a\x02TW\x80cX|\xDE\x1E\x14a\x02~W\x80c\\\x19\xA9\\\x14a\x02\xC2W\x80co0}\xC3\x14a\x02\xD7W`\0\x80\xFD[\x80c1<\xE5g\x14a\x02\x1FW\x80c6D\xE5\x15\x14a\x029W\x80c:F\xB1\xA8\x14a\x02AW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\x93W\x80c\t^\xA7\xB3\x14a\x01\xB1W\x80c\x18\x16\r\xDD\x14a\x01\xD4W\x80c \\(x\x14a\x01\xE6W\x80c#\xB8r\xDD\x14a\x01\xF9W\x80c/O!\xE2\x14a\x02\x0CW[`\0\x80\xFD[a\x01\x9Ba\x04qV[`@Qa\x01\xA8\x91\x90a\x1EnV[`@Q\x80\x91\x03\x90\xF3[a\x01\xC4a\x01\xBF6`\x04a\x1E\x98V[a\x05\x03V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA8V[`\x02T[`@Q\x90\x81R` \x01a\x01\xA8V[a\x01\xC4a\x01\xF46`\x04a\x1E\x98V[a\x05\x1DV[a\x01\xC4a\x02\x076`\x04a\x1E\xC2V[a\x05\x96V[a\x01\xC4a\x02\x1A6`\x04a\x1E\x98V[a\x05\xBCV[a\x02'a\x06KV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xA8V[a\x01\xD8a\x06ZV[a\x01\xD8a\x02O6`\x04a\x1E\x98V[a\x06dV[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x01\x9BV[a\x02\xAAa\x02\x8C6`\x04a\x1E\xFFV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x08` R`@\x90 T\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA8V[a\x02\xD5a\x02\xD06`\x04a\x1E\xFFV[a\x06\xDBV[\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\xAAV[a\x03\x10a\x03\x0B6`\x04a\x1E\xFFV[a\x06\xEAV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xA8V[a\x01\xD8a\x0336`\x04a\x1E\xFFV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x01\xD8a\x03\\6`\x04a\x1E\xFFV[a\x06\xF5V[a\x03ia\x07\0V[`@Qa\x01\xA8\x97\x96\x95\x94\x93\x92\x91\x90a\x1F\x1AV[a\x01\xD8a\x03\x8A6`\x04a\x1F\xB2V[a\x07FV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFFB\x16\x81R` \x01a\x01\xA8V[a\x01\x9Ba\x07\xA6V[a\x01\xD8a\x03\xBB6`\x04a\x1E\xFFV[a\x07\xB5V[a\x01\xC4a\x03\xCE6`\x04a\x1E\x98V[a\x07\xE5V[a\x02\xD5a\x03\xE16`\x04a\x1F\xDAV[a\x07\xF3V[a\x02\xD5a\x03\xF46`\x04a 4V[a\x08\xB0V[a\x01\xD8a\x04\x076`\x04a \xA1V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x04Ea\x04@6`\x04a \xD4V[a\t\xEAV[`@\x80Q\x82Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xD0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x01\xA8V[```\x03\x80Ta\x04\x80\x90a!\x14V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xAC\x90a!\x14V[\x80\x15a\x04\xF9W\x80`\x1F\x10a\x04\xCEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xF9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xDCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x05\x11\x81\x85\x85a\n\x08V[`\x01\x91PP[\x92\x91PPV[`\x000`\x01`\x01`\xA0\x1B\x03\x84\x16\x03a\x05XW`@Qc\xECD/\x05`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x05b3\x83a\n\x1AV[a\x05\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a\nPV[P`\x01\x92\x91PPV[`\x003a\x05\xA4\x85\x82\x85a\n\xAFV[a\x05\xAF\x85\x85\x85a\x0B-V[`\x01\x91PP[\x93\x92PPPV[`\x0030\x81\x03a\x05\xE1W`@QcKc~\x8F`\xE1\x1B\x81R0`\x04\x82\x01R`$\x01a\x05OV[0`\x01`\x01`\xA0\x1B\x03\x85\x16\x03a\x06\x15W`@Qc\xECD/\x05`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x05OV[a\x06A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x820\x86a\x0B\x8CV[a\x05\x11\x84\x84a\x0B\xC5V[`\0a\x06Ua\x0B\xFBV[\x90P\x90V[`\0a\x06Ua\x0C\x86V[`\0Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\x06\xA0W`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\x05OV[a\x06\xCAa\x06\xAC\x84a\r\xB1V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\t` R`@\x90 \x90a\r\xE8V[`\x01`\x01`\xD0\x1B\x03\x16\x94\x93PPPPV[3a\x06\xE6\x81\x83a\x0E\x9EV[PPV[`\0a\x05\x17\x82a\x0F\x10V[`\0a\x05\x17\x82a\x0F2V[`\0``\x80`\0\x80`\0``a\x07\x14a\x0FPV[a\x07\x1Ca\x0F}V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\x07\x82W`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\x05OV[a\x07\x96a\x07\x8E\x84a\r\xB1V[`\n\x90a\r\xE8V[`\x01`\x01`\xD0\x1B\x03\x16\x93\x92PPPV[```\x04\x80Ta\x04\x80\x90a!\x14V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\t` R`@\x81 a\x07\xD6\x90a\x0F\xAAV[`\x01`\x01`\xD0\x1B\x03\x16\x92\x91PPV[`\x003a\x05\x11\x81\x85\x85a\x0B-V[\x83B\x11\x15a\x08\x17W`@Qc#A\xD7\x87`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x05OV[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\0\x90a\x08\x91\x90a\x08\x89\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x0F\xE3V[\x85\x85\x85a\x10\x10V[\x90Pa\x08\x9D\x81\x87a\x10>V[a\x08\xA7\x81\x88a\x0E\x9EV[PPPPPPPV[\x83B\x11\x15a\x08\xD4W`@Qc1<\x89\x81`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x05OV[`\0\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\t!\x8C`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x07` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x90V[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\t|\x82a\x0F\xE3V[\x90P`\0a\t\x8C\x82\x87\x87\x87a\x10\x10V[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xD3W`@Qc%\xC0\x07#`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x8B\x16`$\x82\x01R`D\x01a\x05OV[a\t\xDE\x8A\x8A\x8Aa\n\x08V[PPPPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x05\xB5\x83\x83a\x10\x91V[a\n\x15\x83\x83\x83`\x01a\x10\xC7V[PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\nDW`@QcKc~\x8F`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[a\x06\xE6\x82`\0\x83a\x11\x9CV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra\n\x15\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa\x11\xA7V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T`\0\x19\x81\x14a\x0B'W\x81\x81\x10\x15a\x0B\x18W`@Qc}\xC7\xA0\xD9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x05OV[a\x0B'\x84\x84\x84\x84\x03`\0a\x10\xC7V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0BWW`@QcKc~\x8F`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\x81W`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[a\n\x15\x83\x83\x83a\x11\x9CV[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90Ra\x0B'\x91\x86\x91\x82\x16\x90c#\xB8r\xDD\x90`\x84\x01a\n}V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\xEFW`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[a\x06\xE6`\0\x83\x83a\x11\x9CV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x0CwWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x0Ct\x91\x81\x01\x90a!NV[`\x01[a\x0C\x81WP`\x12\x90V[\x91\x90PV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x0C\xDFWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\r\tWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x06U`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\xE4W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05OV[P\x90V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a\x0EGW`\0a\x0E\x03\x84a\x12\nV[a\x0E\r\x90\x85a!\x81V[`\0\x88\x81R` \x90 \x90\x91P\x81\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a\x0E7W\x80\x91Pa\x0EEV[a\x0EB\x81`\x01a!\x94V[\x92P[P[`\0a\x0EU\x87\x87\x85\x85a\x12\xF2V[\x90P\x80\x15a\x0E\x90Wa\x0Ez\x87a\x0El`\x01\x84a!\x81V[`\0\x91\x82R` \x90\x91 \x01\x90V[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x0E\x93V[`\0[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x08` R`@\x80\x82 \x80T\x86\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x91Q\x91\x90\x94\x16\x93\x92\x84\x92\x90\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\n\x15\x81\x83a\x0F\x0B\x86a\x13TV[a\x13rV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\t` R`@\x81 Ta\x05\x17\x90a\x14\xDEV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x07` R`@\x81 Ta\x05\x17V[``a\x06U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x05a\x15\x0FV[``a\x06U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x06a\x15\x0FV[\x80T`\0\x90\x80\x15a\x0F\xDAWa\x0F\xC4\x83a\x0El`\x01\x84a!\x81V[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x05\xB5V[`\0\x93\x92PPPV[`\0a\x05\x17a\x0F\xF0a\x0C\x86V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0\x80a\x10\"\x88\x88\x88\x88a\x15\xBAV[\x92P\x92P\x92Pa\x102\x82\x82a\x16\x89V[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x07` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x81\x81\x14a\n\x15W`@Qc\x01\xD4\xB6#`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x05OV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x90 a\x05\xB5\x90\x83a\x17BV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x10\xF1W`@Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x11\x1BW`@QcJ\x14\x06\xB1`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90U\x80\x15a\x0B'W\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x11\x8E\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPV[a\n\x15\x83\x83\x83a\x17\xB2V[`\0a\x11\xBC`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x18\x19V[\x90P\x80Q`\0\x14\x15\x80\x15a\x11\xE1WP\x80\x80` \x01\x90Q\x81\x01\x90a\x11\xDF\x91\x90a!\xA7V[\x15[\x15a\n\x15W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x05OV[`\0\x81`\0\x03a\x12\x1CWP`\0\x91\x90PV[`\0`\x01a\x12)\x84a\x18'V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x12BWa\x12Ba!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12ZWa\x12Za!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12rWa\x12ra!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12\x8AWa\x12\x8Aa!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12\xA2Wa\x12\xA2a!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12\xBAWa\x12\xBAa!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12\xD2Wa\x12\xD2a!\xC9V[\x04\x82\x01\x90\x1C\x90Pa\x05\xB5\x81\x82\x85\x81a\x12\xECWa\x12\xECa!\xC9V[\x04a\x18\xBBV[`\0[\x81\x83\x10\x15a\x13LW`\0a\x13\t\x84\x84a\x18\xD1V[`\0\x87\x81R` \x90 \x90\x91Pe\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x138W\x80\x92Pa\x13FV[a\x13C\x81`\x01a!\x94V[\x93P[Pa\x12\xF5V[P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 Ta\x05\x17V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x13\x94WP`\0\x81\x11[\x15a\n\x15W`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x14<W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x81 \x81\x90a\x13\xD7\x90a\x18\xECa\x13\xD2\x86a\x18\xF8V[a\x19,V[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x141\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\n\x15W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\t` R`@\x81 \x81\x90a\x14u\x90a\x19^a\x13\xD2\x86a\x18\xF8V[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x14\xCF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\r\xE4W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05OV[```\xFF\x83\x14a\x15)Wa\x15\"\x83a\x19jV[\x90Pa\x05\x17V[\x81\x80Ta\x155\x90a!\x14V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15a\x90a!\x14V[\x80\x15a\x15\xAEW\x80`\x1F\x10a\x15\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x05\x17V[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x15\xF5WP`\0\x91P`\x03\x90P\x82a\x16\x7FV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x16IW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x16uWP`\0\x92P`\x01\x91P\x82\x90Pa\x16\x7FV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0\x82`\x03\x81\x11\x15a\x16\x9DWa\x16\x9Da!\xDFV[\x03a\x16\xA6WPPV[`\x01\x82`\x03\x81\x11\x15a\x16\xBAWa\x16\xBAa!\xDFV[\x03a\x16\xD8W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x16\xECWa\x16\xECa!\xDFV[\x03a\x17\rW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05OV[`\x03\x82`\x03\x81\x11\x15a\x17!Wa\x17!a!\xDFV[\x03a\x06\xE6W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05OV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x82`\0\x01\x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x17qWa\x17qa!\xF5V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82R`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x17\xBD\x83\x83\x83a\x19\xA9V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x18\x0EW`\0a\x17\xD6`\x02T\x90V[\x90P`\x01`\x01`\xD0\x1B\x03\x80\x82\x11\x15a\x18\x0BW`@Qc\x0EX\xAE\x93`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x05OV[PP[a\n\x15\x83\x83\x83a\x1A\xD3V[``a\x05\xB5\x83\x83`\0a\x1BIV[`\0\x80`\x80\x83\x90\x1C\x15a\x18<W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a\x18NW`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a\x18`W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a\x18rW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a\x18\x84W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a\x18\x96W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a\x18\xA8W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x05\x17W`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a\x18\xCAW\x81a\x05\xB5V[P\x90\x91\x90PV[`\0a\x18\xE0`\x02\x84\x84\x18a\"\x0BV[a\x05\xB5\x90\x84\x84\x16a!\x94V[`\0a\x05\xB5\x82\x84a\"-V[`\0`\x01`\x01`\xD0\x1B\x03\x82\x11\x15a\r\xE4W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`\xD0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05OV[`\0\x80a\x19QBa\x19Ia\x19?\x88a\x0F\xAAV[\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x87\x91\x90a\x1B\xE6V[\x91P\x91P[\x93P\x93\x91PPV[`\0a\x05\xB5\x82\x84a\"LV[```\0a\x19w\x83a\x1B\xF4V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x19\xD4W\x80`\x02`\0\x82\x82Ta\x19\xC9\x91\x90a!\x94V[\x90\x91UPa\x1AF\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x1A'W`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x05OV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x81\x90R`@\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1AbW`\x02\x80T\x82\x90\x03\x90Ua\x1A\x81V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T\x82\x01\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa\x1A\xC6\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x1A\xF5Wa\x1A\xF2`\na\x19^a\x13\xD2\x84a\x18\xF8V[PP[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1B\x17Wa\x1B\x14`\na\x18\xECa\x13\xD2\x84a\x18\xF8V[PP[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x08` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\n\x15\x92\x91\x82\x16\x91\x16\x83a\x13rV[``\x81G\x10\x15a\x1BnW`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x05OV[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x1B\x8A\x91\x90a\"kV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1B\xC7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1B\xCCV[``\x91P[P\x91P\x91Pa\x1B\xDC\x86\x83\x83a\x1C\x1CV[\x96\x95PPPPPPV[`\0\x80a\x19Q\x85\x85\x85a\x1CxV[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x05\x17W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x1C1Wa\x1C,\x82a\x1D\xF2V[a\x05\xB5V[\x81Q\x15\x80\x15a\x1CHWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1CqW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x05OV[P\x80a\x05\xB5V[\x82T`\0\x90\x81\x90\x80\x15a\x1D\x97W`\0a\x1C\x96\x87a\x0El`\x01\x85a!\x81V[`@\x80Q\x80\x82\x01\x90\x91R\x90Te\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01`0\x1B\x90\x92\x04`\x01`\x01`\xD0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a\x1C\xEAW`@Qc% `\x1D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a\x1D6W\x84a\x1D\r\x88a\x0El`\x01\x86a!\x81V[\x80T`\x01`\x01`\xD0\x1B\x03\x92\x90\x92\x16`\x01`0\x1B\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1D\x87V[`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01`0\x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa\x19V\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01`0\x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a\x19VV[\x80Q\x15a\x1E\x02W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0[\x83\x81\x10\x15a\x1E9W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1E!V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1EZ\x81` \x86\x01` \x86\x01a\x1E\x1EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x05\xB5` \x83\x01\x84a\x1EBV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\x81W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\xABW`\0\x80\xFD[a\x1E\xB4\x83a\x1E\x81V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1E\xD7W`\0\x80\xFD[a\x1E\xE0\x84a\x1E\x81V[\x92Pa\x1E\xEE` \x85\x01a\x1E\x81V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\x1F\x11W`\0\x80\xFD[a\x05\xB5\x82a\x1E\x81V[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a\x1F9`\xE0\x83\x01\x89a\x1EBV[\x82\x81\x03`@\x84\x01Ra\x1FK\x81\x89a\x1EBV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90`\0[\x81\x81\x10\x15a\x1F\xA1W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1F\x83V[P\x90\x9B\x9APPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1F\xC4W`\0\x80\xFD[P5\x91\x90PV[`\xFF\x81\x16\x81\x14a\x1E\x1BW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1F\xF3W`\0\x80\xFD[a\x1F\xFC\x87a\x1E\x81V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015a \x1A\x81a\x1F\xCBV[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a OW`\0\x80\xFD[a X\x88a\x1E\x81V[\x96Pa f` \x89\x01a\x1E\x81V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015a \x84\x81a\x1F\xCBV[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a \xB4W`\0\x80\xFD[a \xBD\x83a\x1E\x81V[\x91Pa \xCB` \x84\x01a\x1E\x81V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a \xE7W`\0\x80\xFD[a \xF0\x83a\x1E\x81V[\x91P` \x83\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a!\tW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a!(W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a!HWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a!`W`\0\x80\xFD[\x81Qa\x05\xB5\x81a\x1F\xCBV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\x17Wa\x05\x17a!kV[\x80\x82\x01\x80\x82\x11\x15a\x05\x17Wa\x05\x17a!kV[`\0` \x82\x84\x03\x12\x15a!\xB9W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05\xB5W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a\"(WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x01`\x01`\xD0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x05\x17Wa\x05\x17a!kV[`\x01`\x01`\xD0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x05\x17Wa\x05\x17a!kV[`\0\x82Qa\"}\x81\x84` \x87\x01a\x1E\x1EV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \r\x87W\xC1\xA8 [\xAF.\xF2\xAE-\xC1\x08\xB62;\xD9\x0B\xAB\xCA\x8E\xD6h\xB7x\xD5\"TQ\x96$dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MOCKMYSTIKOVOTETOKEN_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8EW`\x005`\xE0\x1C\x80co\xCF\xFFE\x11a\0\xDEW\x80c\x95\xD8\x9BA\x11a\0\x97W\x80c\xC3\xCD\xA5 \x11a\0qW\x80c\xC3\xCD\xA5 \x14a\x03\xD3W\x80c\xD5\x05\xAC\xCF\x14a\x03\xE6W\x80c\xDDb\xED>\x14a\x03\xF9W\x80c\xF1\x12~\xD8\x14a\x042W`\0\x80\xFD[\x80c\x95\xD8\x9BA\x14a\x03\xA5W\x80c\x9A\xB2N\xB0\x14a\x03\xADW\x80c\xA9\x05\x9C\xBB\x14a\x03\xC0W`\0\x80\xFD[\x80co\xCF\xFFE\x14a\x02\xFDW\x80cp\xA0\x821\x14a\x03%W\x80c~\xCE\xBE\0\x14a\x03NW\x80c\x84\xB0\x19n\x14a\x03aW\x80c\x8ES\x9E\x8C\x14a\x03|W\x80c\x91\xDD\xAD\xF4\x14a\x03\x8FW`\0\x80\xFD[\x80c1<\xE5g\x11a\x01KW\x80cK\xF5\xD7\xE9\x11a\x01%W\x80cK\xF5\xD7\xE9\x14a\x02TW\x80cX|\xDE\x1E\x14a\x02~W\x80c\\\x19\xA9\\\x14a\x02\xC2W\x80co0}\xC3\x14a\x02\xD7W`\0\x80\xFD[\x80c1<\xE5g\x14a\x02\x1FW\x80c6D\xE5\x15\x14a\x029W\x80c:F\xB1\xA8\x14a\x02AW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\x93W\x80c\t^\xA7\xB3\x14a\x01\xB1W\x80c\x18\x16\r\xDD\x14a\x01\xD4W\x80c \\(x\x14a\x01\xE6W\x80c#\xB8r\xDD\x14a\x01\xF9W\x80c/O!\xE2\x14a\x02\x0CW[`\0\x80\xFD[a\x01\x9Ba\x04qV[`@Qa\x01\xA8\x91\x90a\x1EnV[`@Q\x80\x91\x03\x90\xF3[a\x01\xC4a\x01\xBF6`\x04a\x1E\x98V[a\x05\x03V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA8V[`\x02T[`@Q\x90\x81R` \x01a\x01\xA8V[a\x01\xC4a\x01\xF46`\x04a\x1E\x98V[a\x05\x1DV[a\x01\xC4a\x02\x076`\x04a\x1E\xC2V[a\x05\x96V[a\x01\xC4a\x02\x1A6`\x04a\x1E\x98V[a\x05\xBCV[a\x02'a\x06KV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xA8V[a\x01\xD8a\x06ZV[a\x01\xD8a\x02O6`\x04a\x1E\x98V[a\x06dV[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x01\x9BV[a\x02\xAAa\x02\x8C6`\x04a\x1E\xFFV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x08` R`@\x90 T\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA8V[a\x02\xD5a\x02\xD06`\x04a\x1E\xFFV[a\x06\xDBV[\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\xAAV[a\x03\x10a\x03\x0B6`\x04a\x1E\xFFV[a\x06\xEAV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xA8V[a\x01\xD8a\x0336`\x04a\x1E\xFFV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x01\xD8a\x03\\6`\x04a\x1E\xFFV[a\x06\xF5V[a\x03ia\x07\0V[`@Qa\x01\xA8\x97\x96\x95\x94\x93\x92\x91\x90a\x1F\x1AV[a\x01\xD8a\x03\x8A6`\x04a\x1F\xB2V[a\x07FV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFFB\x16\x81R` \x01a\x01\xA8V[a\x01\x9Ba\x07\xA6V[a\x01\xD8a\x03\xBB6`\x04a\x1E\xFFV[a\x07\xB5V[a\x01\xC4a\x03\xCE6`\x04a\x1E\x98V[a\x07\xE5V[a\x02\xD5a\x03\xE16`\x04a\x1F\xDAV[a\x07\xF3V[a\x02\xD5a\x03\xF46`\x04a 4V[a\x08\xB0V[a\x01\xD8a\x04\x076`\x04a \xA1V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x04Ea\x04@6`\x04a \xD4V[a\t\xEAV[`@\x80Q\x82Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xD0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x01\xA8V[```\x03\x80Ta\x04\x80\x90a!\x14V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xAC\x90a!\x14V[\x80\x15a\x04\xF9W\x80`\x1F\x10a\x04\xCEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xF9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xDCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x05\x11\x81\x85\x85a\n\x08V[`\x01\x91PP[\x92\x91PPV[`\x000`\x01`\x01`\xA0\x1B\x03\x84\x16\x03a\x05XW`@Qc\xECD/\x05`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x05b3\x83a\n\x1AV[a\x05\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a\nPV[P`\x01\x92\x91PPV[`\x003a\x05\xA4\x85\x82\x85a\n\xAFV[a\x05\xAF\x85\x85\x85a\x0B-V[`\x01\x91PP[\x93\x92PPPV[`\x0030\x81\x03a\x05\xE1W`@QcKc~\x8F`\xE1\x1B\x81R0`\x04\x82\x01R`$\x01a\x05OV[0`\x01`\x01`\xA0\x1B\x03\x85\x16\x03a\x06\x15W`@Qc\xECD/\x05`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x05OV[a\x06A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x820\x86a\x0B\x8CV[a\x05\x11\x84\x84a\x0B\xC5V[`\0a\x06Ua\x0B\xFBV[\x90P\x90V[`\0a\x06Ua\x0C\x86V[`\0Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\x06\xA0W`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\x05OV[a\x06\xCAa\x06\xAC\x84a\r\xB1V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\t` R`@\x90 \x90a\r\xE8V[`\x01`\x01`\xD0\x1B\x03\x16\x94\x93PPPPV[3a\x06\xE6\x81\x83a\x0E\x9EV[PPV[`\0a\x05\x17\x82a\x0F\x10V[`\0a\x05\x17\x82a\x0F2V[`\0``\x80`\0\x80`\0``a\x07\x14a\x0FPV[a\x07\x1Ca\x0F}V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\x07\x82W`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\x05OV[a\x07\x96a\x07\x8E\x84a\r\xB1V[`\n\x90a\r\xE8V[`\x01`\x01`\xD0\x1B\x03\x16\x93\x92PPPV[```\x04\x80Ta\x04\x80\x90a!\x14V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\t` R`@\x81 a\x07\xD6\x90a\x0F\xAAV[`\x01`\x01`\xD0\x1B\x03\x16\x92\x91PPV[`\x003a\x05\x11\x81\x85\x85a\x0B-V[\x83B\x11\x15a\x08\x17W`@Qc#A\xD7\x87`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x05OV[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\0\x90a\x08\x91\x90a\x08\x89\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x0F\xE3V[\x85\x85\x85a\x10\x10V[\x90Pa\x08\x9D\x81\x87a\x10>V[a\x08\xA7\x81\x88a\x0E\x9EV[PPPPPPPV[\x83B\x11\x15a\x08\xD4W`@Qc1<\x89\x81`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x05OV[`\0\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\t!\x8C`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x07` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x90V[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\t|\x82a\x0F\xE3V[\x90P`\0a\t\x8C\x82\x87\x87\x87a\x10\x10V[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xD3W`@Qc%\xC0\x07#`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x8B\x16`$\x82\x01R`D\x01a\x05OV[a\t\xDE\x8A\x8A\x8Aa\n\x08V[PPPPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x05\xB5\x83\x83a\x10\x91V[a\n\x15\x83\x83\x83`\x01a\x10\xC7V[PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\nDW`@QcKc~\x8F`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[a\x06\xE6\x82`\0\x83a\x11\x9CV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra\n\x15\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa\x11\xA7V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T`\0\x19\x81\x14a\x0B'W\x81\x81\x10\x15a\x0B\x18W`@Qc}\xC7\xA0\xD9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x05OV[a\x0B'\x84\x84\x84\x84\x03`\0a\x10\xC7V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0BWW`@QcKc~\x8F`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\x81W`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[a\n\x15\x83\x83\x83a\x11\x9CV[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90Ra\x0B'\x91\x86\x91\x82\x16\x90c#\xB8r\xDD\x90`\x84\x01a\n}V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\xEFW`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[a\x06\xE6`\0\x83\x83a\x11\x9CV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x0CwWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x0Ct\x91\x81\x01\x90a!NV[`\x01[a\x0C\x81WP`\x12\x90V[\x91\x90PV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x0C\xDFWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\r\tWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x06U`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\xE4W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05OV[P\x90V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a\x0EGW`\0a\x0E\x03\x84a\x12\nV[a\x0E\r\x90\x85a!\x81V[`\0\x88\x81R` \x90 \x90\x91P\x81\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a\x0E7W\x80\x91Pa\x0EEV[a\x0EB\x81`\x01a!\x94V[\x92P[P[`\0a\x0EU\x87\x87\x85\x85a\x12\xF2V[\x90P\x80\x15a\x0E\x90Wa\x0Ez\x87a\x0El`\x01\x84a!\x81V[`\0\x91\x82R` \x90\x91 \x01\x90V[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x0E\x93V[`\0[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x08` R`@\x80\x82 \x80T\x86\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x91Q\x91\x90\x94\x16\x93\x92\x84\x92\x90\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\n\x15\x81\x83a\x0F\x0B\x86a\x13TV[a\x13rV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\t` R`@\x81 Ta\x05\x17\x90a\x14\xDEV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x07` R`@\x81 Ta\x05\x17V[``a\x06U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x05a\x15\x0FV[``a\x06U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x06a\x15\x0FV[\x80T`\0\x90\x80\x15a\x0F\xDAWa\x0F\xC4\x83a\x0El`\x01\x84a!\x81V[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x05\xB5V[`\0\x93\x92PPPV[`\0a\x05\x17a\x0F\xF0a\x0C\x86V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0\x80a\x10\"\x88\x88\x88\x88a\x15\xBAV[\x92P\x92P\x92Pa\x102\x82\x82a\x16\x89V[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x07` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x81\x81\x14a\n\x15W`@Qc\x01\xD4\xB6#`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x05OV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x90 a\x05\xB5\x90\x83a\x17BV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x10\xF1W`@Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x11\x1BW`@QcJ\x14\x06\xB1`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x05OV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90U\x80\x15a\x0B'W\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x11\x8E\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPV[a\n\x15\x83\x83\x83a\x17\xB2V[`\0a\x11\xBC`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x18\x19V[\x90P\x80Q`\0\x14\x15\x80\x15a\x11\xE1WP\x80\x80` \x01\x90Q\x81\x01\x90a\x11\xDF\x91\x90a!\xA7V[\x15[\x15a\n\x15W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x05OV[`\0\x81`\0\x03a\x12\x1CWP`\0\x91\x90PV[`\0`\x01a\x12)\x84a\x18'V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x12BWa\x12Ba!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12ZWa\x12Za!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12rWa\x12ra!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12\x8AWa\x12\x8Aa!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12\xA2Wa\x12\xA2a!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12\xBAWa\x12\xBAa!\xC9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x12\xD2Wa\x12\xD2a!\xC9V[\x04\x82\x01\x90\x1C\x90Pa\x05\xB5\x81\x82\x85\x81a\x12\xECWa\x12\xECa!\xC9V[\x04a\x18\xBBV[`\0[\x81\x83\x10\x15a\x13LW`\0a\x13\t\x84\x84a\x18\xD1V[`\0\x87\x81R` \x90 \x90\x91Pe\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x138W\x80\x92Pa\x13FV[a\x13C\x81`\x01a!\x94V[\x93P[Pa\x12\xF5V[P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 Ta\x05\x17V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x13\x94WP`\0\x81\x11[\x15a\n\x15W`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x14<W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\t` R`@\x81 \x81\x90a\x13\xD7\x90a\x18\xECa\x13\xD2\x86a\x18\xF8V[a\x19,V[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x141\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\n\x15W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\t` R`@\x81 \x81\x90a\x14u\x90a\x19^a\x13\xD2\x86a\x18\xF8V[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x14\xCF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\r\xE4W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05OV[```\xFF\x83\x14a\x15)Wa\x15\"\x83a\x19jV[\x90Pa\x05\x17V[\x81\x80Ta\x155\x90a!\x14V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15a\x90a!\x14V[\x80\x15a\x15\xAEW\x80`\x1F\x10a\x15\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x05\x17V[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x15\xF5WP`\0\x91P`\x03\x90P\x82a\x16\x7FV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x16IW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x16uWP`\0\x92P`\x01\x91P\x82\x90Pa\x16\x7FV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0\x82`\x03\x81\x11\x15a\x16\x9DWa\x16\x9Da!\xDFV[\x03a\x16\xA6WPPV[`\x01\x82`\x03\x81\x11\x15a\x16\xBAWa\x16\xBAa!\xDFV[\x03a\x16\xD8W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x16\xECWa\x16\xECa!\xDFV[\x03a\x17\rW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05OV[`\x03\x82`\x03\x81\x11\x15a\x17!Wa\x17!a!\xDFV[\x03a\x06\xE6W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05OV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x82`\0\x01\x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x17qWa\x17qa!\xF5V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82R`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x17\xBD\x83\x83\x83a\x19\xA9V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x18\x0EW`\0a\x17\xD6`\x02T\x90V[\x90P`\x01`\x01`\xD0\x1B\x03\x80\x82\x11\x15a\x18\x0BW`@Qc\x0EX\xAE\x93`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x05OV[PP[a\n\x15\x83\x83\x83a\x1A\xD3V[``a\x05\xB5\x83\x83`\0a\x1BIV[`\0\x80`\x80\x83\x90\x1C\x15a\x18<W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a\x18NW`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a\x18`W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a\x18rW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a\x18\x84W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a\x18\x96W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a\x18\xA8W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x05\x17W`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a\x18\xCAW\x81a\x05\xB5V[P\x90\x91\x90PV[`\0a\x18\xE0`\x02\x84\x84\x18a\"\x0BV[a\x05\xB5\x90\x84\x84\x16a!\x94V[`\0a\x05\xB5\x82\x84a\"-V[`\0`\x01`\x01`\xD0\x1B\x03\x82\x11\x15a\r\xE4W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`\xD0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05OV[`\0\x80a\x19QBa\x19Ia\x19?\x88a\x0F\xAAV[\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x87\x91\x90a\x1B\xE6V[\x91P\x91P[\x93P\x93\x91PPV[`\0a\x05\xB5\x82\x84a\"LV[```\0a\x19w\x83a\x1B\xF4V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x19\xD4W\x80`\x02`\0\x82\x82Ta\x19\xC9\x91\x90a!\x94V[\x90\x91UPa\x1AF\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x1A'W`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x05OV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x81\x90R`@\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1AbW`\x02\x80T\x82\x90\x03\x90Ua\x1A\x81V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T\x82\x01\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa\x1A\xC6\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x1A\xF5Wa\x1A\xF2`\na\x19^a\x13\xD2\x84a\x18\xF8V[PP[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1B\x17Wa\x1B\x14`\na\x18\xECa\x13\xD2\x84a\x18\xF8V[PP[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x08` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\n\x15\x92\x91\x82\x16\x91\x16\x83a\x13rV[``\x81G\x10\x15a\x1BnW`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x05OV[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x1B\x8A\x91\x90a\"kV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1B\xC7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1B\xCCV[``\x91P[P\x91P\x91Pa\x1B\xDC\x86\x83\x83a\x1C\x1CV[\x96\x95PPPPPPV[`\0\x80a\x19Q\x85\x85\x85a\x1CxV[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x05\x17W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x1C1Wa\x1C,\x82a\x1D\xF2V[a\x05\xB5V[\x81Q\x15\x80\x15a\x1CHWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1CqW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x05OV[P\x80a\x05\xB5V[\x82T`\0\x90\x81\x90\x80\x15a\x1D\x97W`\0a\x1C\x96\x87a\x0El`\x01\x85a!\x81V[`@\x80Q\x80\x82\x01\x90\x91R\x90Te\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01`0\x1B\x90\x92\x04`\x01`\x01`\xD0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a\x1C\xEAW`@Qc% `\x1D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a\x1D6W\x84a\x1D\r\x88a\x0El`\x01\x86a!\x81V[\x80T`\x01`\x01`\xD0\x1B\x03\x92\x90\x92\x16`\x01`0\x1B\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1D\x87V[`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01`0\x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa\x19V\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01`0\x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a\x19VV[\x80Q\x15a\x1E\x02W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0[\x83\x81\x10\x15a\x1E9W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1E!V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1EZ\x81` \x86\x01` \x86\x01a\x1E\x1EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x05\xB5` \x83\x01\x84a\x1EBV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\x81W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\xABW`\0\x80\xFD[a\x1E\xB4\x83a\x1E\x81V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1E\xD7W`\0\x80\xFD[a\x1E\xE0\x84a\x1E\x81V[\x92Pa\x1E\xEE` \x85\x01a\x1E\x81V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\x1F\x11W`\0\x80\xFD[a\x05\xB5\x82a\x1E\x81V[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a\x1F9`\xE0\x83\x01\x89a\x1EBV[\x82\x81\x03`@\x84\x01Ra\x1FK\x81\x89a\x1EBV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90`\0[\x81\x81\x10\x15a\x1F\xA1W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1F\x83V[P\x90\x9B\x9APPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1F\xC4W`\0\x80\xFD[P5\x91\x90PV[`\xFF\x81\x16\x81\x14a\x1E\x1BW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1F\xF3W`\0\x80\xFD[a\x1F\xFC\x87a\x1E\x81V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015a \x1A\x81a\x1F\xCBV[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a OW`\0\x80\xFD[a X\x88a\x1E\x81V[\x96Pa f` \x89\x01a\x1E\x81V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015a \x84\x81a\x1F\xCBV[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a \xB4W`\0\x80\xFD[a \xBD\x83a\x1E\x81V[\x91Pa \xCB` \x84\x01a\x1E\x81V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a \xE7W`\0\x80\xFD[a \xF0\x83a\x1E\x81V[\x91P` \x83\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a!\tW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a!(W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a!HWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a!`W`\0\x80\xFD[\x81Qa\x05\xB5\x81a\x1F\xCBV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\x17Wa\x05\x17a!kV[\x80\x82\x01\x80\x82\x11\x15a\x05\x17Wa\x05\x17a!kV[`\0` \x82\x84\x03\x12\x15a!\xB9W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05\xB5W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a\"(WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x01`\x01`\xD0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x05\x17Wa\x05\x17a!kV[`\x01`\x01`\xD0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x05\x17Wa\x05\x17a!kV[`\0\x82Qa\"}\x81\x84` \x87\x01a\x1E\x1EV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \r\x87W\xC1\xA8 [\xAF.\xF2\xAE-\xC1\x08\xB62;\xD9\x0B\xAB\xCA\x8E\xD6h\xB7x\xD5\"TQ\x96$dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKMYSTIKOVOTETOKEN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockMystikoVoteToken<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockMystikoVoteToken<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockMystikoVoteToken<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockMystikoVoteToken<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockMystikoVoteToken<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockMystikoVoteToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockMystikoVoteToken<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKMYSTIKOVOTETOKEN_ABI.clone(),
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
                MOCKMYSTIKOVOTETOKEN_ABI.clone(),
                MOCKMYSTIKOVOTETOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `CLOCK_MODE` (0x4bf5d7e9) function
        pub fn clock_mode(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([75, 245, 215, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkpoints` (0xf1127ed8) function
        pub fn checkpoints(
            &self,
            account: ::ethers::core::types::Address,
            pos: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, Checkpoint208> {
            self.0
                .method_hash([241, 18, 126, 216], (account, pos))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clock` (0x91ddadf4) function
        pub fn clock(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([145, 221, 173, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegate` (0x5c19a95c) function
        pub fn delegate(
            &self,
            delegatee: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 25, 169, 92], delegatee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegateBySig` (0xc3cda520) function
        pub fn delegate_by_sig(
            &self,
            delegatee: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 205, 165, 32], (delegatee, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegates` (0x587cde1e) function
        pub fn delegates(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([88, 124, 222, 30], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositFor` (0x2f4f21e2) function
        pub fn deposit_for(
            &self,
            account: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 79, 33, 226], (account, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eip712Domain` (0x84b0196e) function
        pub fn eip_712_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 1],
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                [u8; 32],
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([132, 176, 25, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPastTotalSupply` (0x8e539e8c) function
        pub fn get_past_total_supply(
            &self,
            timepoint: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 83, 158, 140], timepoint)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPastVotes` (0x3a46b1a8) function
        pub fn get_past_votes(
            &self,
            account: ::ethers::core::types::Address,
            timepoint: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([58, 70, 177, 168], (account, timepoint))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVotes` (0x9ab24eb0) function
        pub fn get_votes(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 178, 78, 176], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numCheckpoints` (0x6fcfff45) function
        pub fn num_checkpoints(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([111, 207, 255, 69], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 5, 172, 207], (owner, spender, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlying` (0x6f307dc3) function
        pub fn underlying(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([111, 48, 125, 195], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawTo` (0x205c2878) function
        pub fn withdraw_to(
            &self,
            account: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([32, 92, 40, 120], (account, value))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(&self) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `DelegateChanged` event
        pub fn delegate_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DelegateChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `DelegateVotesChanged` event
        pub fn delegate_votes_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DelegateVotesChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `EIP712DomainChanged` event
        pub fn eip712_domain_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, Eip712DomainChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(&self) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MockMystikoVoteTokenEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockMystikoVoteToken<M> {
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
    ///Custom Error type `CheckpointUnorderedInsertion` with signature `CheckpointUnorderedInsertion()` and selector `0x2520601d`
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
    #[etherror(name = "CheckpointUnorderedInsertion", abi = "CheckpointUnorderedInsertion()")]
    pub struct CheckpointUnorderedInsertion;
    ///Custom Error type `ECDSAInvalidSignature` with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`
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
    #[etherror(name = "ECDSAInvalidSignature", abi = "ECDSAInvalidSignature()")]
    pub struct ECDSAInvalidSignature;
    ///Custom Error type `ECDSAInvalidSignatureLength` with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`
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
    #[etherror(name = "ECDSAInvalidSignatureLength", abi = "ECDSAInvalidSignatureLength(uint256)")]
    pub struct ECDSAInvalidSignatureLength {
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `ECDSAInvalidSignatureS` with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`
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
    #[etherror(name = "ECDSAInvalidSignatureS", abi = "ECDSAInvalidSignatureS(bytes32)")]
    pub struct ECDSAInvalidSignatureS {
        pub s: [u8; 32],
    }
    ///Custom Error type `ERC20ExceededSafeSupply` with signature `ERC20ExceededSafeSupply(uint256,uint256)` and selector `0x1cb15d26`
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
    #[etherror(name = "ERC20ExceededSafeSupply", abi = "ERC20ExceededSafeSupply(uint256,uint256)")]
    pub struct ERC20ExceededSafeSupply {
        pub increased_supply: ::ethers::core::types::U256,
        pub cap: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InsufficientAllowance` with signature `ERC20InsufficientAllowance(address,uint256,uint256)` and selector `0xfb8f41b2`
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
        name = "ERC20InsufficientAllowance",
        abi = "ERC20InsufficientAllowance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientAllowance {
        pub spender: ::ethers::core::types::Address,
        pub allowance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InsufficientBalance` with signature `ERC20InsufficientBalance(address,uint256,uint256)` and selector `0xe450d38c`
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
        name = "ERC20InsufficientBalance",
        abi = "ERC20InsufficientBalance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientBalance {
        pub sender: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InvalidApprover` with signature `ERC20InvalidApprover(address)` and selector `0xe602df05`
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
    #[etherror(name = "ERC20InvalidApprover", abi = "ERC20InvalidApprover(address)")]
    pub struct ERC20InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidReceiver` with signature `ERC20InvalidReceiver(address)` and selector `0xec442f05`
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
    #[etherror(name = "ERC20InvalidReceiver", abi = "ERC20InvalidReceiver(address)")]
    pub struct ERC20InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSender` with signature `ERC20InvalidSender(address)` and selector `0x96c6fd1e`
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
    #[etherror(name = "ERC20InvalidSender", abi = "ERC20InvalidSender(address)")]
    pub struct ERC20InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSpender` with signature `ERC20InvalidSpender(address)` and selector `0x94280d62`
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
    #[etherror(name = "ERC20InvalidSpender", abi = "ERC20InvalidSpender(address)")]
    pub struct ERC20InvalidSpender {
        pub spender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidUnderlying` with signature `ERC20InvalidUnderlying(address)` and selector `0x438d6fe3`
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
    #[etherror(name = "ERC20InvalidUnderlying", abi = "ERC20InvalidUnderlying(address)")]
    pub struct ERC20InvalidUnderlying {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC2612ExpiredSignature` with signature `ERC2612ExpiredSignature(uint256)` and selector `0x62791302`
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
    #[etherror(name = "ERC2612ExpiredSignature", abi = "ERC2612ExpiredSignature(uint256)")]
    pub struct ERC2612ExpiredSignature {
        pub deadline: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC2612InvalidSigner` with signature `ERC2612InvalidSigner(address,address)` and selector `0x4b800e46`
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
    #[etherror(name = "ERC2612InvalidSigner", abi = "ERC2612InvalidSigner(address,address)")]
    pub struct ERC2612InvalidSigner {
        pub signer: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC5805FutureLookup` with signature `ERC5805FutureLookup(uint256,uint48)` and selector `0xecd3f81e`
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
    #[etherror(name = "ERC5805FutureLookup", abi = "ERC5805FutureLookup(uint256,uint48)")]
    pub struct ERC5805FutureLookup {
        pub timepoint: ::ethers::core::types::U256,
        pub clock: u64,
    }
    ///Custom Error type `ERC6372InconsistentClock` with signature `ERC6372InconsistentClock()` and selector `0x6ff07140`
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
    #[etherror(name = "ERC6372InconsistentClock", abi = "ERC6372InconsistentClock()")]
    pub struct ERC6372InconsistentClock;
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
    ///Custom Error type `InvalidAccountNonce` with signature `InvalidAccountNonce(address,uint256)` and selector `0x752d88c0`
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
    #[etherror(name = "InvalidAccountNonce", abi = "InvalidAccountNonce(address,uint256)")]
    pub struct InvalidAccountNonce {
        pub account: ::ethers::core::types::Address,
        pub current_nonce: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidShortString` with signature `InvalidShortString()` and selector `0xb3512b0c`
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
    #[etherror(name = "InvalidShortString", abi = "InvalidShortString()")]
    pub struct InvalidShortString;
    ///Custom Error type `SafeCastOverflowedUintDowncast` with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`
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
        name = "SafeCastOverflowedUintDowncast",
        abi = "SafeCastOverflowedUintDowncast(uint8,uint256)"
    )]
    pub struct SafeCastOverflowedUintDowncast {
        pub bits: u8,
        pub value: ::ethers::core::types::U256,
    }
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
    ///Custom Error type `StringTooLong` with signature `StringTooLong(string)` and selector `0x305a27a9`
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
    #[etherror(name = "StringTooLong", abi = "StringTooLong(string)")]
    pub struct StringTooLong {
        pub str: ::std::string::String,
    }
    ///Custom Error type `VotesExpiredSignature` with signature `VotesExpiredSignature(uint256)` and selector `0x4683af0e`
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
    #[etherror(name = "VotesExpiredSignature", abi = "VotesExpiredSignature(uint256)")]
    pub struct VotesExpiredSignature {
        pub expiry: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockMystikoVoteTokenErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        CheckpointUnorderedInsertion(CheckpointUnorderedInsertion),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        ERC20ExceededSafeSupply(ERC20ExceededSafeSupply),
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        ERC20InvalidApprover(ERC20InvalidApprover),
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        ERC20InvalidSender(ERC20InvalidSender),
        ERC20InvalidSpender(ERC20InvalidSpender),
        ERC20InvalidUnderlying(ERC20InvalidUnderlying),
        ERC2612ExpiredSignature(ERC2612ExpiredSignature),
        ERC2612InvalidSigner(ERC2612InvalidSigner),
        ERC5805FutureLookup(ERC5805FutureLookup),
        ERC6372InconsistentClock(ERC6372InconsistentClock),
        FailedInnerCall(FailedInnerCall),
        InvalidAccountNonce(InvalidAccountNonce),
        InvalidShortString(InvalidShortString),
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        StringTooLong(StringTooLong),
        VotesExpiredSignature(VotesExpiredSignature),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockMystikoVoteTokenErrors {
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
            if let Ok(decoded) = <CheckpointUnorderedInsertion as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckpointUnorderedInsertion(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ECDSAInvalidSignature(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignatureLength as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ECDSAInvalidSignatureLength(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignatureS as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ECDSAInvalidSignatureS(decoded));
            }
            if let Ok(decoded) = <ERC20ExceededSafeSupply as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC20ExceededSafeSupply(decoded));
            }
            if let Ok(decoded) = <ERC20InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC20InsufficientAllowance(decoded));
            }
            if let Ok(decoded) = <ERC20InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC20InsufficientBalance(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC20InvalidApprover(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC20InvalidReceiver(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC20InvalidSender(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidSpender as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC20InvalidSpender(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidUnderlying as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC20InvalidUnderlying(decoded));
            }
            if let Ok(decoded) = <ERC2612ExpiredSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC2612ExpiredSignature(decoded));
            }
            if let Ok(decoded) = <ERC2612InvalidSigner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC2612InvalidSigner(decoded));
            }
            if let Ok(decoded) = <ERC5805FutureLookup as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC5805FutureLookup(decoded));
            }
            if let Ok(decoded) = <ERC6372InconsistentClock as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC6372InconsistentClock(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <InvalidAccountNonce as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidAccountNonce(decoded));
            }
            if let Ok(decoded) = <InvalidShortString as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidShortString(decoded));
            }
            if let Ok(decoded) = <SafeCastOverflowedUintDowncast as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SafeCastOverflowedUintDowncast(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) = <StringTooLong as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StringTooLong(decoded));
            }
            if let Ok(decoded) = <VotesExpiredSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VotesExpiredSignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockMystikoVoteTokenErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddressInsufficientBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckpointUnorderedInsertion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ECDSAInvalidSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ECDSAInvalidSignatureLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ECDSAInvalidSignatureS(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20ExceededSafeSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InsufficientAllowance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InsufficientBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InvalidApprover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InvalidReceiver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InvalidSender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InvalidSpender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InvalidUnderlying(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC2612ExpiredSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC2612InvalidSigner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC5805FutureLookup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC6372InconsistentClock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedInnerCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidAccountNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidShortString(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeCastOverflowedUintDowncast(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeERC20FailedOperation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StringTooLong(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VotesExpiredSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockMystikoVoteTokenErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <CheckpointUnorderedInsertion as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ECDSAInvalidSignature as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ECDSAInvalidSignatureLength as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ECDSAInvalidSignatureS as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20ExceededSafeSupply as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InsufficientAllowance as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InsufficientBalance as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InvalidApprover as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InvalidReceiver as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InvalidSender as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InvalidSpender as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InvalidUnderlying as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC2612ExpiredSignature as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC2612InvalidSigner as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC5805FutureLookup as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC6372InconsistentClock as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <FailedInnerCall as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidAccountNonce as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidShortString as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <SafeCastOverflowedUintDowncast as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <StringTooLong as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <VotesExpiredSignature as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockMystikoVoteTokenErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckpointUnorderedInsertion(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureS(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20ExceededSafeSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InsufficientAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidSpender(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidUnderlying(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC2612ExpiredSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC2612InvalidSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC5805FutureLookup(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC6372InconsistentClock(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAccountNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidShortString(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeCastOverflowedUintDowncast(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeERC20FailedOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::StringTooLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotesExpiredSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockMystikoVoteTokenErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for MockMystikoVoteTokenErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for MockMystikoVoteTokenErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<CheckpointUnorderedInsertion> for MockMystikoVoteTokenErrors {
        fn from(value: CheckpointUnorderedInsertion) -> Self {
            Self::CheckpointUnorderedInsertion(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for MockMystikoVoteTokenErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength> for MockMystikoVoteTokenErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for MockMystikoVoteTokenErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<ERC20ExceededSafeSupply> for MockMystikoVoteTokenErrors {
        fn from(value: ERC20ExceededSafeSupply) -> Self {
            Self::ERC20ExceededSafeSupply(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientAllowance> for MockMystikoVoteTokenErrors {
        fn from(value: ERC20InsufficientAllowance) -> Self {
            Self::ERC20InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientBalance> for MockMystikoVoteTokenErrors {
        fn from(value: ERC20InsufficientBalance) -> Self {
            Self::ERC20InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidApprover> for MockMystikoVoteTokenErrors {
        fn from(value: ERC20InvalidApprover) -> Self {
            Self::ERC20InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidReceiver> for MockMystikoVoteTokenErrors {
        fn from(value: ERC20InvalidReceiver) -> Self {
            Self::ERC20InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSender> for MockMystikoVoteTokenErrors {
        fn from(value: ERC20InvalidSender) -> Self {
            Self::ERC20InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSpender> for MockMystikoVoteTokenErrors {
        fn from(value: ERC20InvalidSpender) -> Self {
            Self::ERC20InvalidSpender(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidUnderlying> for MockMystikoVoteTokenErrors {
        fn from(value: ERC20InvalidUnderlying) -> Self {
            Self::ERC20InvalidUnderlying(value)
        }
    }
    impl ::core::convert::From<ERC2612ExpiredSignature> for MockMystikoVoteTokenErrors {
        fn from(value: ERC2612ExpiredSignature) -> Self {
            Self::ERC2612ExpiredSignature(value)
        }
    }
    impl ::core::convert::From<ERC2612InvalidSigner> for MockMystikoVoteTokenErrors {
        fn from(value: ERC2612InvalidSigner) -> Self {
            Self::ERC2612InvalidSigner(value)
        }
    }
    impl ::core::convert::From<ERC5805FutureLookup> for MockMystikoVoteTokenErrors {
        fn from(value: ERC5805FutureLookup) -> Self {
            Self::ERC5805FutureLookup(value)
        }
    }
    impl ::core::convert::From<ERC6372InconsistentClock> for MockMystikoVoteTokenErrors {
        fn from(value: ERC6372InconsistentClock) -> Self {
            Self::ERC6372InconsistentClock(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for MockMystikoVoteTokenErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InvalidAccountNonce> for MockMystikoVoteTokenErrors {
        fn from(value: InvalidAccountNonce) -> Self {
            Self::InvalidAccountNonce(value)
        }
    }
    impl ::core::convert::From<InvalidShortString> for MockMystikoVoteTokenErrors {
        fn from(value: InvalidShortString) -> Self {
            Self::InvalidShortString(value)
        }
    }
    impl ::core::convert::From<SafeCastOverflowedUintDowncast> for MockMystikoVoteTokenErrors {
        fn from(value: SafeCastOverflowedUintDowncast) -> Self {
            Self::SafeCastOverflowedUintDowncast(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for MockMystikoVoteTokenErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<StringTooLong> for MockMystikoVoteTokenErrors {
        fn from(value: StringTooLong) -> Self {
            Self::StringTooLong(value)
        }
    }
    impl ::core::convert::From<VotesExpiredSignature> for MockMystikoVoteTokenErrors {
        fn from(value: VotesExpiredSignature) -> Self {
            Self::VotesExpiredSignature(value)
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "DelegateChanged", abi = "DelegateChanged(address,address,address)")]
    pub struct DelegateChangedFilter {
        #[ethevent(indexed)]
        pub delegator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from_delegate: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_delegate: ::ethers::core::types::Address,
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
    #[ethevent(name = "DelegateVotesChanged", abi = "DelegateVotesChanged(address,uint256,uint256)")]
    pub struct DelegateVotesChangedFilter {
        #[ethevent(indexed)]
        pub delegate: ::ethers::core::types::Address,
        pub previous_votes: ::ethers::core::types::U256,
        pub new_votes: ::ethers::core::types::U256,
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
    #[ethevent(name = "EIP712DomainChanged", abi = "EIP712DomainChanged()")]
    pub struct Eip712DomainChangedFilter;
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockMystikoVoteTokenEvents {
        ApprovalFilter(ApprovalFilter),
        DelegateChangedFilter(DelegateChangedFilter),
        DelegateVotesChangedFilter(DelegateVotesChangedFilter),
        Eip712DomainChangedFilter(Eip712DomainChangedFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockMystikoVoteTokenEvents {
        fn decode_log(log: &::ethers::core::abi::RawLog) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockMystikoVoteTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DelegateChangedFilter::decode_log(log) {
                return Ok(MockMystikoVoteTokenEvents::DelegateChangedFilter(decoded));
            }
            if let Ok(decoded) = DelegateVotesChangedFilter::decode_log(log) {
                return Ok(MockMystikoVoteTokenEvents::DelegateVotesChangedFilter(decoded));
            }
            if let Ok(decoded) = Eip712DomainChangedFilter::decode_log(log) {
                return Ok(MockMystikoVoteTokenEvents::Eip712DomainChangedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockMystikoVoteTokenEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockMystikoVoteTokenEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateVotesChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712DomainChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockMystikoVoteTokenEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<DelegateChangedFilter> for MockMystikoVoteTokenEvents {
        fn from(value: DelegateChangedFilter) -> Self {
            Self::DelegateChangedFilter(value)
        }
    }
    impl ::core::convert::From<DelegateVotesChangedFilter> for MockMystikoVoteTokenEvents {
        fn from(value: DelegateVotesChangedFilter) -> Self {
            Self::DelegateVotesChangedFilter(value)
        }
    }
    impl ::core::convert::From<Eip712DomainChangedFilter> for MockMystikoVoteTokenEvents {
        fn from(value: Eip712DomainChangedFilter) -> Self {
            Self::Eip712DomainChangedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockMystikoVoteTokenEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `CLOCK_MODE` function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`
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
    #[ethcall(name = "CLOCK_MODE", abi = "CLOCK_MODE()")]
    pub struct ClockModeCall;
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `checkpoints` function with signature `checkpoints(address,uint32)` and selector `0xf1127ed8`
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
    #[ethcall(name = "checkpoints", abi = "checkpoints(address,uint32)")]
    pub struct CheckpointsCall {
        pub account: ::ethers::core::types::Address,
        pub pos: u32,
    }
    ///Container type for all input parameters for the `clock` function with signature `clock()` and selector `0x91ddadf4`
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
    #[ethcall(name = "clock", abi = "clock()")]
    pub struct ClockCall;
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `delegate` function with signature `delegate(address)` and selector `0x5c19a95c`
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
    #[ethcall(name = "delegate", abi = "delegate(address)")]
    pub struct DelegateCall {
        pub delegatee: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `delegateBySig` function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc3cda520`
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
        name = "delegateBySig",
        abi = "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct DelegateBySigCall {
        pub delegatee: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `delegates` function with signature `delegates(address)` and selector `0x587cde1e`
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
    #[ethcall(name = "delegates", abi = "delegates(address)")]
    pub struct DelegatesCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `depositFor` function with signature `depositFor(address,uint256)` and selector `0x2f4f21e2`
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
    #[ethcall(name = "depositFor", abi = "depositFor(address,uint256)")]
    pub struct DepositForCall {
        pub account: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    #[ethcall(name = "eip712Domain", abi = "eip712Domain()")]
    pub struct Eip712DomainCall;
    ///Container type for all input parameters for the `getPastTotalSupply` function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`
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
    #[ethcall(name = "getPastTotalSupply", abi = "getPastTotalSupply(uint256)")]
    pub struct GetPastTotalSupplyCall {
        pub timepoint: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPastVotes` function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`
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
    #[ethcall(name = "getPastVotes", abi = "getPastVotes(address,uint256)")]
    pub struct GetPastVotesCall {
        pub account: ::ethers::core::types::Address,
        pub timepoint: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getVotes` function with signature `getVotes(address)` and selector `0x9ab24eb0`
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
    #[ethcall(name = "getVotes", abi = "getVotes(address)")]
    pub struct GetVotesCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `numCheckpoints` function with signature `numCheckpoints(address)` and selector `0x6fcfff45`
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
    #[ethcall(name = "numCheckpoints", abi = "numCheckpoints(address)")]
    pub struct NumCheckpointsCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `underlying` function with signature `underlying()` and selector `0x6f307dc3`
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
    #[ethcall(name = "underlying", abi = "underlying()")]
    pub struct UnderlyingCall;
    ///Container type for all input parameters for the `withdrawTo` function with signature `withdrawTo(address,uint256)` and selector `0x205c2878`
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
    #[ethcall(name = "withdrawTo", abi = "withdrawTo(address,uint256)")]
    pub struct WithdrawToCall {
        pub account: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockMystikoVoteTokenCalls {
        ClockMode(ClockModeCall),
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Checkpoints(CheckpointsCall),
        Clock(ClockCall),
        Decimals(DecimalsCall),
        Delegate(DelegateCall),
        DelegateBySig(DelegateBySigCall),
        Delegates(DelegatesCall),
        DepositFor(DepositForCall),
        Eip712Domain(Eip712DomainCall),
        GetPastTotalSupply(GetPastTotalSupplyCall),
        GetPastVotes(GetPastVotesCall),
        GetVotes(GetVotesCall),
        Name(NameCall),
        Nonces(NoncesCall),
        NumCheckpoints(NumCheckpointsCall),
        Permit(PermitCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Underlying(UnderlyingCall),
        WithdrawTo(WithdrawToCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockMystikoVoteTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ClockModeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClockMode(decoded));
            }
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <CheckpointsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Checkpoints(decoded));
            }
            if let Ok(decoded) = <ClockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Clock(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DelegateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegate(decoded));
            }
            if let Ok(decoded) = <DelegateBySigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DelegateBySig(decoded));
            }
            if let Ok(decoded) = <DelegatesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegates(decoded));
            }
            if let Ok(decoded) = <DepositForCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositFor(decoded));
            }
            if let Ok(decoded) = <Eip712DomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Eip712Domain(decoded));
            }
            if let Ok(decoded) = <GetPastTotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPastTotalSupply(decoded));
            }
            if let Ok(decoded) = <GetPastVotesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPastVotes(decoded));
            }
            if let Ok(decoded) = <GetVotesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVotes(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <NumCheckpointsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NumCheckpoints(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <UnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Underlying(decoded));
            }
            if let Ok(decoded) = <WithdrawToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockMystikoVoteTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClockMode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DomainSeparator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Checkpoints(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Clock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Delegate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DelegateBySig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Delegates(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositFor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Eip712Domain(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPastTotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPastVotes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetVotes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumCheckpoints(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Underlying(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockMystikoVoteTokenCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClockMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Checkpoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::Clock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateBySig(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegates(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Domain(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPastTotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPastVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumCheckpoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::Underlying(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClockModeCall> for MockMystikoVoteTokenCalls {
        fn from(value: ClockModeCall) -> Self {
            Self::ClockMode(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for MockMystikoVoteTokenCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for MockMystikoVoteTokenCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for MockMystikoVoteTokenCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockMystikoVoteTokenCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<CheckpointsCall> for MockMystikoVoteTokenCalls {
        fn from(value: CheckpointsCall) -> Self {
            Self::Checkpoints(value)
        }
    }
    impl ::core::convert::From<ClockCall> for MockMystikoVoteTokenCalls {
        fn from(value: ClockCall) -> Self {
            Self::Clock(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for MockMystikoVoteTokenCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DelegateCall> for MockMystikoVoteTokenCalls {
        fn from(value: DelegateCall) -> Self {
            Self::Delegate(value)
        }
    }
    impl ::core::convert::From<DelegateBySigCall> for MockMystikoVoteTokenCalls {
        fn from(value: DelegateBySigCall) -> Self {
            Self::DelegateBySig(value)
        }
    }
    impl ::core::convert::From<DelegatesCall> for MockMystikoVoteTokenCalls {
        fn from(value: DelegatesCall) -> Self {
            Self::Delegates(value)
        }
    }
    impl ::core::convert::From<DepositForCall> for MockMystikoVoteTokenCalls {
        fn from(value: DepositForCall) -> Self {
            Self::DepositFor(value)
        }
    }
    impl ::core::convert::From<Eip712DomainCall> for MockMystikoVoteTokenCalls {
        fn from(value: Eip712DomainCall) -> Self {
            Self::Eip712Domain(value)
        }
    }
    impl ::core::convert::From<GetPastTotalSupplyCall> for MockMystikoVoteTokenCalls {
        fn from(value: GetPastTotalSupplyCall) -> Self {
            Self::GetPastTotalSupply(value)
        }
    }
    impl ::core::convert::From<GetPastVotesCall> for MockMystikoVoteTokenCalls {
        fn from(value: GetPastVotesCall) -> Self {
            Self::GetPastVotes(value)
        }
    }
    impl ::core::convert::From<GetVotesCall> for MockMystikoVoteTokenCalls {
        fn from(value: GetVotesCall) -> Self {
            Self::GetVotes(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockMystikoVoteTokenCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for MockMystikoVoteTokenCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<NumCheckpointsCall> for MockMystikoVoteTokenCalls {
        fn from(value: NumCheckpointsCall) -> Self {
            Self::NumCheckpoints(value)
        }
    }
    impl ::core::convert::From<PermitCall> for MockMystikoVoteTokenCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockMystikoVoteTokenCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockMystikoVoteTokenCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for MockMystikoVoteTokenCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockMystikoVoteTokenCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<UnderlyingCall> for MockMystikoVoteTokenCalls {
        fn from(value: UnderlyingCall) -> Self {
            Self::Underlying(value)
        }
    }
    impl ::core::convert::From<WithdrawToCall> for MockMystikoVoteTokenCalls {
        fn from(value: WithdrawToCall) -> Self {
            Self::WithdrawTo(value)
        }
    }
    ///Container type for all return fields from the `CLOCK_MODE` function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`
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
    pub struct ClockModeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `checkpoints` function with signature `checkpoints(address,uint32)` and selector `0xf1127ed8`
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
    pub struct CheckpointsReturn(pub Checkpoint208);
    ///Container type for all return fields from the `clock` function with signature `clock()` and selector `0x91ddadf4`
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
    pub struct ClockReturn(pub u64);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `delegates` function with signature `delegates(address)` and selector `0x587cde1e`
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
    pub struct DelegatesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `depositFor` function with signature `depositFor(address,uint256)` and selector `0x2f4f21e2`
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
    pub struct DepositForReturn(pub bool);
    ///Container type for all return fields from the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    pub struct Eip712DomainReturn {
        pub fields: [u8; 1],
        pub name: ::std::string::String,
        pub version: ::std::string::String,
        pub chain_id: ::ethers::core::types::U256,
        pub verifying_contract: ::ethers::core::types::Address,
        pub salt: [u8; 32],
        pub extensions: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `getPastTotalSupply` function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`
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
    pub struct GetPastTotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPastVotes` function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`
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
    pub struct GetPastVotesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVotes` function with signature `getVotes(address)` and selector `0x9ab24eb0`
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
    pub struct GetVotesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `numCheckpoints` function with signature `numCheckpoints(address)` and selector `0x6fcfff45`
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
    pub struct NumCheckpointsReturn(pub u32);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
    ///Container type for all return fields from the `underlying` function with signature `underlying()` and selector `0x6f307dc3`
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
    pub struct UnderlyingReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `withdrawTo` function with signature `withdrawTo(address,uint256)` and selector `0x205c2878`
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
    pub struct WithdrawToReturn(pub bool);
}
