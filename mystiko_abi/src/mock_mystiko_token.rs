pub use mock_mystiko_token::*;
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
pub mod mock_mystiko_token {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
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
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKMYSTIKOTOKEN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01``@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l&\xBC\xB9\xBA4\xB5\xB7\x90*7\xB5\xB2\xB7`\x99\x1B\x81RP\x80`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l&\xBC\xB9\xBA4\xB5\xB7\x90*7\xB5\xB2\xB7`\x99\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bXZK`\xE8\x1B\x81RP\x81`\x03\x90\x81a\0\xA4\x91\x90a\x04\x08V[P`\x04a\0\xB1\x82\x82a\x04\x08V[Pa\0\xC1\x91P\x83\x90P`\x05a\x01\x8FV[a\x01 Ra\0\xD0\x81`\x06a\x01\x8FV[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Ra\x01]`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0RPa\x01\x8A3a\x01w`\x12`\na\x05\xC3V[a\x01\x85\x90c;\x9A\xCA\0a\x05\xD9V[a\x01\xC2V[a\x06uV[`\0` \x83Q\x10\x15a\x01\xABWa\x01\xA4\x83a\x02\x01V[\x90Pa\x01\xBCV[\x81a\x01\xB6\x84\x82a\x04\x08V[P`\xFF\x90P[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x01\xF1W`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xFD`\0\x83\x83a\x02?V[PPV[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15a\x02,W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\x01\xE8\x91\x90a\x05\xF0V[\x80Qa\x027\x82a\x06>V[\x17\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x02jW\x80`\x02`\0\x82\x82Ta\x02_\x91\x90a\x06bV[\x90\x91UPa\x02\xDC\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x02\xBDW`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x01\xE8V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x81\x90R`@\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02\xF8W`\x02\x80T\x82\x90\x03\x90Ua\x03\x17V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T\x82\x01\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa\x03\\\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x03\x93W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x03\xB3WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x04\x03W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x03\xE0WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x04\0W`\0\x81U`\x01\x01a\x03\xECV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x04!Wa\x04!a\x03iV[a\x045\x81a\x04/\x84Ta\x03\x7FV[\x84a\x03\xB9V[` `\x1F\x82\x11`\x01\x81\x14a\x04iW`\0\x83\x15a\x04QWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x04\0V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x04\x99W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x04yV[P\x84\x82\x10\x15a\x04\xB7W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x81[`\x01\x84\x11\x15a\x05\x17W\x80\x85\x04\x81\x11\x15a\x04\xFBWa\x04\xFBa\x04\xC6V[`\x01\x84\x16\x15a\x05\tW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x04\xE0V[\x93P\x93\x91PPV[`\0\x82a\x05.WP`\x01a\x01\xBCV[\x81a\x05;WP`\0a\x01\xBCV[\x81`\x01\x81\x14a\x05QW`\x02\x81\x14a\x05[Wa\x05wV[`\x01\x91PPa\x01\xBCV[`\xFF\x84\x11\x15a\x05lWa\x05la\x04\xC6V[PP`\x01\x82\x1Ba\x01\xBCV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x05\x9AWP\x81\x81\na\x01\xBCV[a\x05\xA7`\0\x19\x84\x84a\x04\xDCV[\x80`\0\x19\x04\x82\x11\x15a\x05\xBBWa\x05\xBBa\x04\xC6V[\x02\x93\x92PPPV[`\0a\x05\xD2`\xFF\x84\x16\x83a\x05\x1FV[\x93\x92PPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x01\xBCWa\x01\xBCa\x04\xC6V[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\x06\x1EW` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x06\x01V[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x03\xB3W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x01\xBCWa\x01\xBCa\x04\xC6V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x0E\xFFa\x06\xCF`\09`\0a\x06\xFE\x01R`\0a\x06\xD1\x01R`\0a\x06y\x01R`\0a\x06Q\x01R`\0a\x05\xAC\x01R`\0a\x05\xD6\x01R`\0a\x06\0\x01Ra\x0E\xFF`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\0\x8CW\x80c\x95\xD8\x9BA\x11a\0fW\x80c\x95\xD8\x9BA\x14a\x01\xA8W\x80c\xA9\x05\x9C\xBB\x14a\x01\xB0W\x80c\xD5\x05\xAC\xCF\x14a\x01\xC3W\x80c\xDDb\xED>\x14a\x01\xD8W`\0\x80\xFD[\x80cp\xA0\x821\x14a\x01QW\x80c~\xCE\xBE\0\x14a\x01zW\x80c\x84\xB0\x19n\x14a\x01\x8DW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\0\xD4W\x80c\t^\xA7\xB3\x14a\0\xF2W\x80c\x18\x16\r\xDD\x14a\x01\x15W\x80c#\xB8r\xDD\x14a\x01'W\x80c1<\xE5g\x14a\x01:W\x80c6D\xE5\x15\x14a\x01IW[`\0\x80\xFD[a\0\xDCa\x02\x11V[`@Qa\0\xE9\x91\x90a\x0CbV[`@Q\x80\x91\x03\x90\xF3[a\x01\x05a\x01\x006`\x04a\x0C\x98V[a\x02\xA3V[`@Q\x90\x15\x15\x81R` \x01a\0\xE9V[`\x02T[`@Q\x90\x81R` \x01a\0\xE9V[a\x01\x05a\x0156`\x04a\x0C\xC2V[a\x02\xBDV[`@Q`\x12\x81R` \x01a\0\xE9V[a\x01\x19a\x02\xE1V[a\x01\x19a\x01_6`\x04a\x0C\xFFV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x01\x19a\x01\x886`\x04a\x0C\xFFV[a\x02\xF0V[a\x01\x95a\x03\x0EV[`@Qa\0\xE9\x97\x96\x95\x94\x93\x92\x91\x90a\r\x1AV[a\0\xDCa\x03TV[a\x01\x05a\x01\xBE6`\x04a\x0C\x98V[a\x03cV[a\x01\xD6a\x01\xD16`\x04a\r\xB2V[a\x03qV[\0[a\x01\x19a\x01\xE66`\x04a\x0E%V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[```\x03\x80Ta\x02 \x90a\x0EXV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02L\x90a\x0EXV[\x80\x15a\x02\x99W\x80`\x1F\x10a\x02nWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x99V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02|W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x02\xB1\x81\x85\x85a\x04\xB0V[`\x01\x91PP[\x92\x91PPV[`\x003a\x02\xCB\x85\x82\x85a\x04\xC2V[a\x02\xD6\x85\x85\x85a\x05@V[P`\x01\x94\x93PPPPV[`\0a\x02\xEBa\x05\x9FV[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x07` R`@\x81 Ta\x02\xB7V[`\0``\x80`\0\x80`\0``a\x03\"a\x06\xCAV[a\x03*a\x06\xF7V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[```\x04\x80Ta\x02 \x90a\x0EXV[`\x003a\x02\xB1\x81\x85\x85a\x05@V[\x83B\x11\x15a\x03\x9AW`@Qc1<\x89\x81`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\x03\xE7\x8C`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x07` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x90V[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x04B\x82a\x07$V[\x90P`\0a\x04R\x82\x87\x87\x87a\x07QV[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x99W`@Qc%\xC0\x07#`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x8B\x16`$\x82\x01R`D\x01a\x03\x91V[a\x04\xA4\x8A\x8A\x8Aa\x04\xB0V[PPPPPPPPPPV[a\x04\xBD\x83\x83\x83`\x01a\x07\x7FV[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T`\0\x19\x81\x14a\x05:W\x81\x81\x10\x15a\x05+W`@Qc}\xC7\xA0\xD9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x03\x91V[a\x05:\x84\x84\x84\x84\x03`\0a\x07\x7FV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x05jW`@QcKc~\x8F`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\x91V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05\x94W`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\x91V[a\x04\xBD\x83\x83\x83a\x08TV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x05\xF8WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\x06\"WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x02\xEB`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[``a\x02\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x05a\t~V[``a\x02\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x06a\t~V[`\0a\x02\xB7a\x071a\x05\x9FV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0\x80a\x07c\x88\x88\x88\x88a\n)V[\x92P\x92P\x92Pa\x07s\x82\x82a\n\xF8V[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x07\xA9W`@Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\x91V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x07\xD3W`@QcJ\x14\x06\xB1`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\x91V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90U\x80\x15a\x05:W\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x08F\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x08\x7FW\x80`\x02`\0\x82\x82Ta\x08t\x91\x90a\x0E\x92V[\x90\x91UPa\x08\xF1\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x08\xD2W`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x03\x91V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x81\x90R`@\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\rW`\x02\x80T\x82\x90\x03\x90Ua\t,V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T\x82\x01\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa\tq\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[```\xFF\x83\x14a\t\x98Wa\t\x91\x83a\x0B\xB5V[\x90Pa\x02\xB7V[\x81\x80Ta\t\xA4\x90a\x0EXV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xD0\x90a\x0EXV[\x80\x15a\n\x1DW\x80`\x1F\x10a\t\xF2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x1DV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x02\xB7V[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\ndWP`\0\x91P`\x03\x90P\x82a\n\xEEV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\n\xB8W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\n\xE4WP`\0\x92P`\x01\x91P\x82\x90Pa\n\xEEV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0\x82`\x03\x81\x11\x15a\x0B\x0CWa\x0B\x0Ca\x0E\xB3V[\x03a\x0B\x15WPPV[`\x01\x82`\x03\x81\x11\x15a\x0B)Wa\x0B)a\x0E\xB3V[\x03a\x0BGW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x0B[Wa\x0B[a\x0E\xB3V[\x03a\x0B|W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x91V[`\x03\x82`\x03\x81\x11\x15a\x0B\x90Wa\x0B\x90a\x0E\xB3V[\x03a\x0B\xB1W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x91V[PPV[```\0a\x0B\xC2\x83a\x0B\xF4V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x02\xB7W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0CBW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0C&V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x0Cu` \x83\x01\x84a\x0C\x1CV[\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\x93W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xABW`\0\x80\xFD[a\x0C\xB4\x83a\x0C|V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xD7W`\0\x80\xFD[a\x0C\xE0\x84a\x0C|V[\x92Pa\x0C\xEE` \x85\x01a\x0C|V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\r\x11W`\0\x80\xFD[a\x0Cu\x82a\x0C|V[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a\r9`\xE0\x83\x01\x89a\x0C\x1CV[\x82\x81\x03`@\x84\x01Ra\rK\x81\x89a\x0C\x1CV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90`\0[\x81\x81\x10\x15a\r\xA1W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\r\x83V[P\x90\x9B\x9APPPPPPPPPPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\r\xCDW`\0\x80\xFD[a\r\xD6\x88a\x0C|V[\x96Pa\r\xE4` \x89\x01a\x0C|V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0E\x08W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E8W`\0\x80\xFD[a\x0EA\x83a\x0C|V[\x91Pa\x0EO` \x84\x01a\x0C|V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0ElW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E\x8CWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x02\xB7WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x96\xC1n\xF1M\xA6\x86\x7F\xE310\\]\x9B\x8C\x99k\x9F]\x95\xED\"\xC2\x7F\x01\x9E\xBF\x0C\x0Bl\x13\xCCdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MOCKMYSTIKOTOKEN_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\0\x8CW\x80c\x95\xD8\x9BA\x11a\0fW\x80c\x95\xD8\x9BA\x14a\x01\xA8W\x80c\xA9\x05\x9C\xBB\x14a\x01\xB0W\x80c\xD5\x05\xAC\xCF\x14a\x01\xC3W\x80c\xDDb\xED>\x14a\x01\xD8W`\0\x80\xFD[\x80cp\xA0\x821\x14a\x01QW\x80c~\xCE\xBE\0\x14a\x01zW\x80c\x84\xB0\x19n\x14a\x01\x8DW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\0\xD4W\x80c\t^\xA7\xB3\x14a\0\xF2W\x80c\x18\x16\r\xDD\x14a\x01\x15W\x80c#\xB8r\xDD\x14a\x01'W\x80c1<\xE5g\x14a\x01:W\x80c6D\xE5\x15\x14a\x01IW[`\0\x80\xFD[a\0\xDCa\x02\x11V[`@Qa\0\xE9\x91\x90a\x0CbV[`@Q\x80\x91\x03\x90\xF3[a\x01\x05a\x01\x006`\x04a\x0C\x98V[a\x02\xA3V[`@Q\x90\x15\x15\x81R` \x01a\0\xE9V[`\x02T[`@Q\x90\x81R` \x01a\0\xE9V[a\x01\x05a\x0156`\x04a\x0C\xC2V[a\x02\xBDV[`@Q`\x12\x81R` \x01a\0\xE9V[a\x01\x19a\x02\xE1V[a\x01\x19a\x01_6`\x04a\x0C\xFFV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x01\x19a\x01\x886`\x04a\x0C\xFFV[a\x02\xF0V[a\x01\x95a\x03\x0EV[`@Qa\0\xE9\x97\x96\x95\x94\x93\x92\x91\x90a\r\x1AV[a\0\xDCa\x03TV[a\x01\x05a\x01\xBE6`\x04a\x0C\x98V[a\x03cV[a\x01\xD6a\x01\xD16`\x04a\r\xB2V[a\x03qV[\0[a\x01\x19a\x01\xE66`\x04a\x0E%V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[```\x03\x80Ta\x02 \x90a\x0EXV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02L\x90a\x0EXV[\x80\x15a\x02\x99W\x80`\x1F\x10a\x02nWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x99V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02|W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x02\xB1\x81\x85\x85a\x04\xB0V[`\x01\x91PP[\x92\x91PPV[`\x003a\x02\xCB\x85\x82\x85a\x04\xC2V[a\x02\xD6\x85\x85\x85a\x05@V[P`\x01\x94\x93PPPPV[`\0a\x02\xEBa\x05\x9FV[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x07` R`@\x81 Ta\x02\xB7V[`\0``\x80`\0\x80`\0``a\x03\"a\x06\xCAV[a\x03*a\x06\xF7V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[```\x04\x80Ta\x02 \x90a\x0EXV[`\x003a\x02\xB1\x81\x85\x85a\x05@V[\x83B\x11\x15a\x03\x9AW`@Qc1<\x89\x81`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\x03\xE7\x8C`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x07` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x90V[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x04B\x82a\x07$V[\x90P`\0a\x04R\x82\x87\x87\x87a\x07QV[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x99W`@Qc%\xC0\x07#`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x8B\x16`$\x82\x01R`D\x01a\x03\x91V[a\x04\xA4\x8A\x8A\x8Aa\x04\xB0V[PPPPPPPPPPV[a\x04\xBD\x83\x83\x83`\x01a\x07\x7FV[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T`\0\x19\x81\x14a\x05:W\x81\x81\x10\x15a\x05+W`@Qc}\xC7\xA0\xD9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x03\x91V[a\x05:\x84\x84\x84\x84\x03`\0a\x07\x7FV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x05jW`@QcKc~\x8F`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\x91V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05\x94W`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\x91V[a\x04\xBD\x83\x83\x83a\x08TV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x05\xF8WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\x06\"WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x02\xEB`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[``a\x02\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x05a\t~V[``a\x02\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x06a\t~V[`\0a\x02\xB7a\x071a\x05\x9FV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0\x80a\x07c\x88\x88\x88\x88a\n)V[\x92P\x92P\x92Pa\x07s\x82\x82a\n\xF8V[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x07\xA9W`@Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\x91V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x07\xD3W`@QcJ\x14\x06\xB1`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\x91V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90U\x80\x15a\x05:W\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x08F\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x08\x7FW\x80`\x02`\0\x82\x82Ta\x08t\x91\x90a\x0E\x92V[\x90\x91UPa\x08\xF1\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x08\xD2W`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x03\x91V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x81\x90R`@\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\rW`\x02\x80T\x82\x90\x03\x90Ua\t,V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T\x82\x01\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa\tq\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[```\xFF\x83\x14a\t\x98Wa\t\x91\x83a\x0B\xB5V[\x90Pa\x02\xB7V[\x81\x80Ta\t\xA4\x90a\x0EXV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xD0\x90a\x0EXV[\x80\x15a\n\x1DW\x80`\x1F\x10a\t\xF2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x1DV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x02\xB7V[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\ndWP`\0\x91P`\x03\x90P\x82a\n\xEEV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\n\xB8W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\n\xE4WP`\0\x92P`\x01\x91P\x82\x90Pa\n\xEEV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0\x82`\x03\x81\x11\x15a\x0B\x0CWa\x0B\x0Ca\x0E\xB3V[\x03a\x0B\x15WPPV[`\x01\x82`\x03\x81\x11\x15a\x0B)Wa\x0B)a\x0E\xB3V[\x03a\x0BGW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x0B[Wa\x0B[a\x0E\xB3V[\x03a\x0B|W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x91V[`\x03\x82`\x03\x81\x11\x15a\x0B\x90Wa\x0B\x90a\x0E\xB3V[\x03a\x0B\xB1W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x91V[PPV[```\0a\x0B\xC2\x83a\x0B\xF4V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x02\xB7W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0CBW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0C&V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x0Cu` \x83\x01\x84a\x0C\x1CV[\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\x93W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xABW`\0\x80\xFD[a\x0C\xB4\x83a\x0C|V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xD7W`\0\x80\xFD[a\x0C\xE0\x84a\x0C|V[\x92Pa\x0C\xEE` \x85\x01a\x0C|V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\r\x11W`\0\x80\xFD[a\x0Cu\x82a\x0C|V[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a\r9`\xE0\x83\x01\x89a\x0C\x1CV[\x82\x81\x03`@\x84\x01Ra\rK\x81\x89a\x0C\x1CV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90`\0[\x81\x81\x10\x15a\r\xA1W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\r\x83V[P\x90\x9B\x9APPPPPPPPPPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\r\xCDW`\0\x80\xFD[a\r\xD6\x88a\x0C|V[\x96Pa\r\xE4` \x89\x01a\x0C|V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0E\x08W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E8W`\0\x80\xFD[a\x0EA\x83a\x0C|V[\x91Pa\x0EO` \x84\x01a\x0C|V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0ElW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E\x8CWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x02\xB7WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x96\xC1n\xF1M\xA6\x86\x7F\xE310\\]\x9B\x8C\x99k\x9F]\x95\xED\"\xC2\x7F\x01\x9E\xBF\x0C\x0Bl\x13\xCCdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKMYSTIKOTOKEN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockMystikoToken<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockMystikoToken<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockMystikoToken<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockMystikoToken<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockMystikoToken<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockMystikoToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockMystikoToken<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKMYSTIKOTOKEN_ABI.clone(),
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
                MOCKMYSTIKOTOKEN_ABI.clone(),
                MOCKMYSTIKOTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
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
        ///Gets the contract's `Approval` event
        pub fn approval_filter(&self) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
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
        pub fn events(&self) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MockMystikoTokenEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockMystikoToken<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockMystikoTokenErrors {
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        ERC20InvalidApprover(ERC20InvalidApprover),
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        ERC20InvalidSender(ERC20InvalidSender),
        ERC20InvalidSpender(ERC20InvalidSpender),
        ERC2612ExpiredSignature(ERC2612ExpiredSignature),
        ERC2612InvalidSigner(ERC2612InvalidSigner),
        InvalidAccountNonce(InvalidAccountNonce),
        InvalidShortString(InvalidShortString),
        StringTooLong(StringTooLong),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockMystikoTokenErrors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
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
            if let Ok(decoded) = <ERC2612ExpiredSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC2612ExpiredSignature(decoded));
            }
            if let Ok(decoded) = <ERC2612InvalidSigner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC2612InvalidSigner(decoded));
            }
            if let Ok(decoded) = <InvalidAccountNonce as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidAccountNonce(decoded));
            }
            if let Ok(decoded) = <InvalidShortString as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidShortString(decoded));
            }
            if let Ok(decoded) = <StringTooLong as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StringTooLong(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockMystikoTokenErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ECDSAInvalidSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ECDSAInvalidSignatureLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ECDSAInvalidSignatureS(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InsufficientAllowance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InsufficientBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InvalidApprover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InvalidReceiver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InvalidSender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC20InvalidSpender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC2612ExpiredSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC2612InvalidSigner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidAccountNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidShortString(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StringTooLong(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockMystikoTokenErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <ECDSAInvalidSignature as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ECDSAInvalidSignatureLength as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ECDSAInvalidSignatureS as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InsufficientAllowance as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InsufficientBalance as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InvalidApprover as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InvalidReceiver as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InvalidSender as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC20InvalidSpender as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC2612ExpiredSignature as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ERC2612InvalidSigner as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidAccountNonce as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidShortString as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <StringTooLong as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockMystikoTokenErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ECDSAInvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureS(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InsufficientAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC20InvalidSpender(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC2612ExpiredSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC2612InvalidSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAccountNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidShortString(element) => ::core::fmt::Display::fmt(element, f),
                Self::StringTooLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockMystikoTokenErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for MockMystikoTokenErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength> for MockMystikoTokenErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for MockMystikoTokenErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientAllowance> for MockMystikoTokenErrors {
        fn from(value: ERC20InsufficientAllowance) -> Self {
            Self::ERC20InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientBalance> for MockMystikoTokenErrors {
        fn from(value: ERC20InsufficientBalance) -> Self {
            Self::ERC20InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidApprover> for MockMystikoTokenErrors {
        fn from(value: ERC20InvalidApprover) -> Self {
            Self::ERC20InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidReceiver> for MockMystikoTokenErrors {
        fn from(value: ERC20InvalidReceiver) -> Self {
            Self::ERC20InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSender> for MockMystikoTokenErrors {
        fn from(value: ERC20InvalidSender) -> Self {
            Self::ERC20InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSpender> for MockMystikoTokenErrors {
        fn from(value: ERC20InvalidSpender) -> Self {
            Self::ERC20InvalidSpender(value)
        }
    }
    impl ::core::convert::From<ERC2612ExpiredSignature> for MockMystikoTokenErrors {
        fn from(value: ERC2612ExpiredSignature) -> Self {
            Self::ERC2612ExpiredSignature(value)
        }
    }
    impl ::core::convert::From<ERC2612InvalidSigner> for MockMystikoTokenErrors {
        fn from(value: ERC2612InvalidSigner) -> Self {
            Self::ERC2612InvalidSigner(value)
        }
    }
    impl ::core::convert::From<InvalidAccountNonce> for MockMystikoTokenErrors {
        fn from(value: InvalidAccountNonce) -> Self {
            Self::InvalidAccountNonce(value)
        }
    }
    impl ::core::convert::From<InvalidShortString> for MockMystikoTokenErrors {
        fn from(value: InvalidShortString) -> Self {
            Self::InvalidShortString(value)
        }
    }
    impl ::core::convert::From<StringTooLong> for MockMystikoTokenErrors {
        fn from(value: StringTooLong) -> Self {
            Self::StringTooLong(value)
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
    pub enum MockMystikoTokenEvents {
        ApprovalFilter(ApprovalFilter),
        Eip712DomainChangedFilter(Eip712DomainChangedFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockMystikoTokenEvents {
        fn decode_log(log: &::ethers::core::abi::RawLog) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockMystikoTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = Eip712DomainChangedFilter::decode_log(log) {
                return Ok(MockMystikoTokenEvents::Eip712DomainChangedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockMystikoTokenEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockMystikoTokenEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712DomainChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockMystikoTokenEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<Eip712DomainChangedFilter> for MockMystikoTokenEvents {
        fn from(value: Eip712DomainChangedFilter) -> Self {
            Self::Eip712DomainChangedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockMystikoTokenEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
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
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockMystikoTokenCalls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        Eip712Domain(Eip712DomainCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockMystikoTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <Eip712DomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Eip712Domain(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockMystikoTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Eip712Domain(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockMystikoTokenCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Domain(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for MockMystikoTokenCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for MockMystikoTokenCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for MockMystikoTokenCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockMystikoTokenCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for MockMystikoTokenCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<Eip712DomainCall> for MockMystikoTokenCalls {
        fn from(value: Eip712DomainCall) -> Self {
            Self::Eip712Domain(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockMystikoTokenCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for MockMystikoTokenCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PermitCall> for MockMystikoTokenCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockMystikoTokenCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockMystikoTokenCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for MockMystikoTokenCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockMystikoTokenCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
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
}
