pub use mock_lz_endpoint::*;
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
pub mod mock_lz_endpoint {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_chainId"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addrToPackedBytes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addrToPackedBytes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_a"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("blockNextMsg"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("blockNextMsg"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("estimateFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("estimateFees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_nativeFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_zroFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("forceResumeReceive"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("forceResumeReceive"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getChainId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getChainId"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getConfig"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInboundNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getInboundNonce"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainID"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("getLengthOfQueue"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLengthOfQueue"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
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
                    ::std::borrow::ToOwned::to_owned("getOutboundNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOutboundNonce"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainID"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("getReceiveLibraryAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getReceiveLibraryAddress",),
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
                    ::std::borrow::ToOwned::to_owned("getReceiveVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getReceiveVersion"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSendLibraryAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSendLibraryAddress",),
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
                    ::std::borrow::ToOwned::to_owned("getSendVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSendVersion"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasStoredPayload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hasStoredPayload"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
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
                    ::std::borrow::ToOwned::to_owned("inboundNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("inboundNonce"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("isReceivingPayload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isReceivingPayload"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isSendingPayload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isSendingPayload"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lzEndpointLookup"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lzEndpointLookup"),
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
                    ::std::borrow::ToOwned::to_owned("mockBlockConfirmations"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mockBlockConfirmations",),
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
                    ::std::borrow::ToOwned::to_owned("mockChainId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mockChainId"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mockLayerZeroVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mockLayerZeroVersion",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mockLibraryVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mockLibraryVersion"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mockOracle"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mockOracle"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "address payable"
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mockRelayer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mockRelayer"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "address payable"
                            ),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mockStaticNativeFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mockStaticNativeFee",),
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
                    ::std::borrow::ToOwned::to_owned("msgsToDeliver"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("msgsToDeliver"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("dstAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nativeFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nativeFee"),
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
                    ::std::borrow::ToOwned::to_owned("outboundNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("outboundNonce"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("packedBytesToAddr"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("packedBytesToAddr"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_b"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("receivePayload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("receivePayload"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_dstAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("retryPayload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("retryPayload"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("send"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("send"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_destination"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address payable"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_adapterParams"),
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
                    ::std::borrow::ToOwned::to_owned("setConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setConfig"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDestLzEndpoint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setDestLzEndpoint"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destAddr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lzEndpointAddr"),
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
                    ::std::borrow::ToOwned::to_owned("setEstimatedFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setEstimatedFees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_nativeFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_zroFee"),
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
                    ::std::borrow::ToOwned::to_owned("setReceiveVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setReceiveVersion"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSendVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSendVersion"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("storedPayload"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("storedPayload"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint16"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payloadLength"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("dstAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "bytes32"
                                ),),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("zroFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("zroFee"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PayloadCleared"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PayloadCleared"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("dstAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PayloadStored"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PayloadStored"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("srcChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("dstAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("reason"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UaForceResumeReceive"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("UaForceResumeReceive",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("chainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("srcAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKLZENDPOINT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`@Qa#\xFF8\x03\x80a#\xFF\x839\x81\x01`@\x81\x90R`,\x91`WV[`*`\x05U`\x06\x80T`\x01a\xFF\xFF\x19\x91\x82\x16\x81\x17\x90\x92U\x81T\x16a\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\x80V[`\0` \x82\x84\x03\x12\x15`hW`\0\x80\xFD[\x81Qa\xFF\xFF\x81\x16\x81\x14`yW`\0\x80\xFD[\x93\x92PPPV[a#p\x80a\0\x8F`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x020W`\x005`\xE0\x1C\x80c\x99oy\xC0\x11a\x01.W\x80c\xCA\x06k5\x11a\0\xABW\x80c\xDB\x14\xF3\x05\x11a\0oW\x80c\xDB\x14\xF3\x05\x14a\x07\xAEW\x80c\xE9a\xA9R\x14a\x07\xC9W\x80c\xE9zD\x8A\x14a\x074W\x80c\xF5\xEC\xBD\xBC\x14a\x07\xDFW\x80c\xFD\xC0|p\x14a\x08\x11W`\0\x80\xFD[\x80c\xCA\x06k5\x14a\x074W\x80c\xCB\xED\x8B\x9C\x14a\x07HW\x80c\xD21\x04\xF1\x14a\x07iW\x80c\xDA\x1A|\x9A\x14a\x02\x88W\x80c\xDA\xB3\x12w\x14a\x07\x88W`\0\x80\xFD[\x80c\xB2\x08d\x99\x11a\0\xF2W\x80c\xB2\x08d\x99\x14a\x06AW\x80c\xC0\x8F\x15\xA1\x14a\x06\x82W\x80c\xC2\xFAH\x13\x14a\x06\xCBW\x80c\xC5\x801\0\x14a\x06\xEBW\x80c\xC8\x1B8:\x14a\x06\xFEW`\0\x80\xFD[\x80c\x99oy\xC0\x14a\x05\xDAW\x80c\x9Cr\x9D\xA1\x14a\x04EW\x80c\xAA\xFF_\x16\x14a\x05\xF0W\x80c\xAF@j\xA5\x14a\x06\x10W\x80c\xB1!\x07p\x14a\x06&W`\0\x80\xFD[\x80c@\xA7\xBB\x10\x11a\x01\xBCW\x80cq\xBA/\xD6\x11a\x01\x80W\x80cq\xBA/\xD6\x14a\x04EW\x80cv\xA3\x86\xDC\x14a\x04eW\x80cz\x14WH\x14a\x04\xFEW\x80c\x7Fm\xF8\xE6\x14a\x05hW\x80c\x99$\xD3;\x14a\x05\x88W`\0\x80\xFD[\x80c@\xA7\xBB\x10\x14a\x03\x7FW\x80cB\xD6Z\x8D\x14a\x03\xB4W\x80cO\x05[\x04\x14a\x03\xD4W\x80cYR\xC4\xEC\x14a\x03\xF4W\x80c[0\x115\x14a\x04\x18W`\0\x80\xFD[\x80c\x0E\xAFn\xA6\x11a\x02\x03W\x80c\x0E\xAFn\xA6\x14a\x02\xE1W\x80c\x10\xDD\xB17\x14a\x02hW\x80c\x12\xA9\xEEk\x14a\x03\x11W\x80c&F\xAF\x08\x14a\x03@W\x80c4\x08\xE4p\x14a\x03fW`\0\x80\xFD[\x80c\x05\xBCq\x0F\x14a\x025W\x80c\x07\xE0\xDB\x17\x14a\x02hW\x80c\teh\xF6\x14a\x02\x88W\x80c\t\xED\xDFT\x14a\x02\xA9W[`\0\x80\xFD[4\x80\x15a\x02AW`\0\x80\xFD[P`\x04Ta\x02P\x90a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02tW`\0\x80\xFD[Pa\x02\x86a\x02\x836`\x04a\x17yV[PV[\0[4\x80\x15a\x02\x94W`\0\x80\xFD[Pa\x02Pa\x02\xA36`\x04a\x17\xB0V[P`\x01\x90V[4\x80\x15a\x02\xB5W`\0\x80\xFD[Pa\x02\xC9a\x02\xC46`\x04a\x18\x15V[a\x081V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02_V[4\x80\x15a\x02\xEDW`\0\x80\xFD[Pa\x03\x01a\x02\xFC6`\x04a\x18VV[a\x08PV[`@Q\x90\x15\x15\x81R` \x01a\x02_V[4\x80\x15a\x03\x1DW`\0\x80\xFD[Pa\x031a\x03,6`\x04a\x19KV[a\x08\x95V[`@Qa\x02_\x93\x92\x91\x90a\x19\xE8V[4\x80\x15a\x03LW`\0\x80\xFD[Pa\x02\x86a\x03[6`\x04a\x1A$V[`\x07\x91\x90\x91U`\x08UV[4\x80\x15a\x03rW`\0\x80\xFD[P`\x01Ta\xFF\xFF\x16a\x02PV[4\x80\x15a\x03\x8BW`\0\x80\xFD[Pa\x03\x9Fa\x03\x9A6`\x04a\x1AFV[a\t\x92V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02_V[4\x80\x15a\x03\xC0W`\0\x80\xFD[Pa\x02\x86a\x03\xCF6`\x04a\x18VV[a\t\xB5V[4\x80\x15a\x03\xE0W`\0\x80\xFD[P`\x02Ta\x02\xC9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\0W`\0\x80\xFD[Pa\x04\n`\x03T\x81V[`@Q\x90\x81R` \x01a\x02_V[4\x80\x15a\x04$W`\0\x80\xFD[Pa\x048a\x0436`\x04a\x17\xB0V[a\n\xFCV[`@Qa\x02_\x91\x90a\x1A\xE8V[4\x80\x15a\x04QW`\0\x80\xFD[Pa\x02\xC9a\x04`6`\x04a\x17\xB0V[P0\x90V[4\x80\x15a\x04qW`\0\x80\xFD[Pa\x04\xD1a\x04\x806`\x04a\x1A\xFBV[`\x0C` \x90\x81R`\0\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91R\x80T`\x01\x90\x91\x01T`\x01`\x01`@\x1B\x03\x82\x16\x91`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90\x83V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\x02_V[4\x80\x15a\x05\nW`\0\x80\xFD[Pa\x05Pa\x05\x196`\x04a\x1BHV[a\xFF\xFF\x82\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02_V[4\x80\x15a\x05tW`\0\x80\xFD[Pa\x04\na\x05\x836`\x04a\x18VV[a\x0B.V[4\x80\x15a\x05\x94W`\0\x80\xFD[Pa\x05Pa\x05\xA36`\x04a\x1A\xFBV[`\n` \x90\x81R`\0\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91RT`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x05\xE6W`\0\x80\xFD[Pa\x04\n`\x07T\x81V[4\x80\x15a\x05\xFCW`\0\x80\xFD[Pa\x02\x86a\x06\x0B6`\x04a\x1B\x7FV[a\x0BjV[4\x80\x15a\x06\x1CW`\0\x80\xFD[Pa\x04\n`\x08T\x81V[4\x80\x15a\x062W`\0\x80\xFD[P`\x06Ta\x02P\x90a\xFF\xFF\x16\x81V[4\x80\x15a\x06MW`\0\x80\xFD[Pa\x05Pa\x06\\6`\x04a\x1BHV[`\x0B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x06\x8EW`\0\x80\xFD[Pa\x02\x86a\x06\x9D6`\x04a\x1C\x03V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UV[4\x80\x15a\x06\xD7W`\0\x80\xFD[Pa\x02\x86a\x06\xE66`\x04a\x1C!V[a\r\x80V[a\x02\x86a\x06\xF96`\x04a\x1C\xDFV[a\x12\xDEV[4\x80\x15a\x07\nW`\0\x80\xFD[Pa\x02\xC9a\x07\x196`\x04a\x17\xB0V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07@W`\0\x80\xFD[P`\0a\x03\x01V[4\x80\x15a\x07TW`\0\x80\xFD[Pa\x02\x86a\x07c6`\x04a\x1D\xB1V[PPPPV[4\x80\x15a\x07uW`\0\x80\xFD[Pa\x02\x86`\t\x80T`\xFF\x19\x16`\x01\x17\x90UV[4\x80\x15a\x07\x94W`\0\x80\xFD[P`\x01Ta\x02\xC9\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\xBAW`\0\x80\xFD[P`\x01Ta\x02P\x90a\xFF\xFF\x16\x81V[4\x80\x15a\x07\xD5W`\0\x80\xFD[Pa\x04\n`\x05T\x81V[4\x80\x15a\x07\xEBW`\0\x80\xFD[Pa\x048a\x07\xFA6`\x04a\x1E\x18V[`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x94\x93PPPPV[4\x80\x15a\x08\x1DW`\0\x80\xFD[Pa\x05Pa\x08,6`\x04a\x18VV[a\x14\xD8V[`\0\x80`@Q`\x02\x84\x01`\x02\x86\x03\x827`\t\x19\x01Q\x91PP[\x92\x91PPV[a\xFF\xFF\x83\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x90Q\x82\x91\x90a\x08v\x90\x86\x90\x86\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 `\x01\x01T\x15\x15\x91PP\x93\x92PPPV[`\r` \x90\x81R`\0\x84\x81R`@\x90 \x83Q\x80\x85\x01\x83\x01\x80Q\x92\x81R\x90\x83\x01\x92\x85\x01\x92\x90\x92 \x91R\x80T\x82\x90\x81\x10a\x08\xCCW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x83\x16\x96P`\x01`\xA0\x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x16\x94P\x91\x92Pa\t\x0F\x90a\x1EuV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t;\x90a\x1EuV[\x80\x15a\t\x88W\x80`\x1F\x10a\t]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x88V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tkW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[`\0\x80\x84Q`\x07Ta\t\xA4\x91\x90a\x1E\xC5V[\x91P`\x08T\x90P\x95P\x95\x93PPPPV[a\xFF\xFF\x83\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x90Qa\t\xD8\x90\x85\x90\x85\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 `\x01\x81\x01T\x90\x91Pa\n@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FLayerZero: no stored payload\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FLayerZero: invalid caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n7V[\x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U`\0`\x01\x82\x01U`@Q\x7F#\xD2hO9n\x92\xA6\xE2\xFF-\x16\xF9\x8Eo\xEA\0\xD5\x0C\xB2zd\xB51\xBC\x07H\xF70!\x1F\x98\x90a\n\xE9\x90\x86\x90\x86\x90\x86\x90a\x1F\x05V[`@Q\x80\x91\x03\x90\xA1a\x07c\x84\x84\x84a\x15\x1DV[`@\x80Q``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x83\x01R\x80Q\x80\x83\x03`\x14\x01\x81R`4\x90\x92\x01\x90R\x90V[a\xFF\xFF\x83\x16`\0\x90\x81R`\r` R`@\x80\x82 \x90Qa\x0BQ\x90\x85\x90\x85\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T\x90P\x93\x92PPPV[a\xFF\xFF\x85\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x90Qa\x0B\x8D\x90\x87\x90\x87\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 `\x01\x81\x01T\x90\x91Pa\x0B\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FLayerZero: no stored payload\0\0\0\0`D\x82\x01R`d\x01a\n7V[\x80T`\x01`\x01`@\x1B\x03\x16\x82\x14\x80\x15a\x0C#WP\x80`\x01\x01T\x83\x83`@Qa\x0C\x19\x92\x91\x90a\x1EeV[`@Q\x80\x91\x03\x90 \x14[a\x0CoW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FLayerZero: invalid payload\0\0\0\0\0\0`D\x82\x01R`d\x01a\n7V[\x80T`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x82U`\0`\x01\x83\x01\x81\x90Ua\xFF\xFF\x88\x16\x81R`\n` R`@\x80\x82 \x90Q`\x01`@\x1B\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x16\x92a\x0C\xB9\x90\x89\x90\x89\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 Tb\x1D5g`\xE0\x1B\x82R`\x01`\x01`@\x1B\x03\x16\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90b\x1D5g\x90a\r\x05\x90\x8B\x90\x8B\x90\x8B\x90\x87\x90\x8C\x90\x8C\x90`\x04\x01a\x1F#V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x1FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r3W=`\0\x80>=`\0\xFD[PPPP\x7Fa$4\xF3\x95\x81\xC8\xE7\xD9\x97F\xC9\xC2\x0Cn\xB0\xCE\x8C\x0E\xB9\x9F\0|W\x19\xD6 \x84\x13p\x95}\x88\x88\x88\x84\x86`@Qa\rn\x95\x94\x93\x92\x91\x90a\x1FpV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[a\xFF\xFF\x88\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x90Qa\r\xA3\x90\x8A\x90\x8A\x90a\x1EeV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x90P`\n`\0\x8Aa\xFF\xFF\x16a\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x88\x88`@Qa\r\xDE\x92\x91\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T`\0\x90a\x0E\x04\x90`\x01`\x01`@\x1B\x03\x16a\x1F\xB9V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90U`\x01`\x01`@\x1B\x03\x16\x85`\x01`\x01`@\x1B\x03\x16\x14a\x0E\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuLayerZero: wrong nonce`P\x1B`D\x82\x01R`d\x01a\n7V[`\x01\x81\x01T\x15a\x11FWa\xFF\xFF\x89\x16`\0\x90\x81R`\r` R`@\x80\x82 \x90Qa\x0E\xAD\x90\x8B\x90\x8B\x90a\x1EeV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x90P`\0`@Q\x80``\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x91RP\x82T\x90\x91P\x15a\x10\xD7W\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x90\x81\x90 \x84Q`\x02\x90\x94\x02\x01\x80T\x91\x85\x01Q`\x01`\x01`@\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x93\x90\x93\x17\x17\x82U`@\x83\x01Q\x83\x92\x91\x82\x01\x90a\x0F\x8E\x90\x82a 3V[PPP`\0[\x82Ta\x0F\xA2\x90`\x01\x90a \xF4V[\x81\x10\x15a\x10\\W\x82\x81\x81T\x81\x10a\x0F\xBBWa\x0F\xBBa!\x07V[\x90`\0R` `\0 \x90`\x02\x02\x01\x83\x82`\x01a\x0F\xD7\x91\x90a!\x1DV[\x81T\x81\x10a\x0F\xE7Wa\x0F\xE7a!\x07V[`\0\x91\x82R` \x90\x91 \x82T`\x02\x90\x92\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x82U\x83T`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x17`\x01`\xA0\x1B\x92\x83\x90\x04`\x01`\x01`@\x1B\x03\x16\x90\x92\x02\x91\x90\x91\x17\x81U`\x01\x80\x82\x01\x90a\x10Q\x90\x84\x01\x82a!0V[PPP`\x01\x01a\x0F\x94V[P\x80\x82`\0\x81T\x81\x10a\x10qWa\x10qa!\x07V[`\0\x91\x82R` \x91\x82\x90 \x83Q`\x02\x90\x92\x02\x01\x80T\x92\x84\x01Q`\x01`\x01`@\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x81U`@\x82\x01Q`\x01\x82\x01\x90a\x10\xCE\x90\x82a 3V[P\x90PPa\x11?V[\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x90\x81\x90 \x84Q`\x02\x90\x94\x02\x01\x80T\x91\x85\x01Q`\x01`\x01`@\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x93\x90\x93\x17\x17\x82U`@\x83\x01Q\x83\x92\x91\x82\x01\x90a\x11;\x90\x82a 3V[PPP[PPa\x12\xD3V[`\tT`\xFF\x16\x15a\x12lW`@Q\x80``\x01`@R\x80\x84\x84\x90P`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x84`@Qa\x11\x8D\x92\x91\x90a\x1EeV[`@\x80Q\x91\x82\x90\x03\x90\x91 \x90\x91Ra\xFF\xFF\x8B\x16`\0\x90\x81R`\x0C` R\x81\x90 \x90Qa\x11\xBC\x90\x8B\x90\x8B\x90a\x1EeV[\x90\x81R`@\x80Q\x91\x82\x90\x03` \x90\x81\x01\x83 \x84Q\x81T\x86\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x17\x81U\x93\x82\x01Q`\x01\x90\x94\x01\x93\x90\x93U\x91\x81\x01\x82R`\0\x81R\x90Q\x7F\x0F\x9EM\x95\xB6/\x08\"-a+Z\xB9 9\xCD\x8F\xBB\xBE\xA5P\xA9^\x8D\xF9\xF9'Ck\xBD\xF5\xDB\x91a\x12U\x91\x8C\x91\x8C\x91\x8C\x91\x8C\x91\x8C\x91\x8B\x91\x8B\x91\x90a\"\x04V[`@Q\x80\x91\x03\x90\xA1`\t\x80T`\xFF\x19\x16\x90Ua\x12\xD3V[`@Qb\x1D5g`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90b\x1D5g\x90a\x12\xA0\x90\x8C\x90\x8C\x90\x8C\x90\x8B\x90\x8A\x90\x8A\x90`\x04\x01a\x1F#V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xBAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xCEW=`\0\x80>=`\0\xFD[PPPP[PPPPPPPPPV[`\0a\x12\xEA\x88\x88a\x081V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x91\x92P\x16\x80a\x13{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FLayerZeroMock: destination Layer`D\x82\x01R\x7FZero Endpoint not found\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n7V[`\x07Ta\x13\x89\x90\x87\x90a\x1E\xC5V[4\x10\x15a\x13\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FLayerZeroMock: not enough native`D\x82\x01Rh for fees`\xB8\x1B`d\x82\x01R`\x84\x01a\n7V[a\xFF\xFF\x8A\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x82\x90a\x14\x1E\x90`\x01`\x01`@\x1B\x03\x16a\x1F\xB9V[\x82T`\x01`\x01`@\x1B\x03\x80\x83\x16a\x01\0\x94\x90\x94\n\x93\x84\x02\x93\x02\x19\x16\x91\x90\x91\x17\x90\x91U\x90P`\0a\x14M3a\n\xFCV[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16c\xC2\xFAH\x13`\x01`\0\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16\x83\x87\x86`\0\x8F\x8F`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x98\x97\x96\x95\x94\x93\x92\x91\x90a\"vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\xB2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xC6W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPPV[a\xFF\xFF\x83\x16`\0\x90\x81R`\n` R`@\x80\x82 \x90Qa\x14\xFB\x90\x85\x90\x85\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`@\x1B\x03\x16\x90P\x93\x92PPPV[a\xFF\xFF\x83\x16`\0\x90\x81R`\r` R`@\x80\x82 \x90Qa\x15@\x90\x85\x90\x85\x90a\x1EeV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x90P[\x80T\x15a\x07cW\x80T`\0\x90\x82\x90a\x15k\x90`\x01\x90a \xF4V[\x81T\x81\x10a\x15{Wa\x15{a!\x07V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R`\x02\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x84R`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x90\x91\x04\x16\x93\x83\x01\x93\x90\x93R`\x01\x83\x01\x80T\x92\x93\x92\x91\x84\x01\x91a\x15\xD4\x90a\x1EuV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\0\x90a\x1EuV[\x80\x15a\x16MW\x80`\x1F\x10a\x16\"Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16MV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x160W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16b\x1D5g\x86\x86\x86\x85` \x01Q\x86`@\x01Q`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x97\x95\x94\x93\x92\x91\x90a\"\xD9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xC5W=`\0\x80>=`\0\xFD[PPPP\x81\x80T\x80a\x16\xD9Wa\x16\xD9a#$V[`\0\x82\x81R` \x81 `\x02`\0\x19\x90\x93\x01\x92\x83\x02\x01\x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\x17\n`\x01\x83\x01\x82a\x17\x14V[PP\x90UPa\x15QV[P\x80Ta\x17 \x90a\x1EuV[`\0\x82U\x80`\x1F\x10a\x170WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x02\x83\x91\x90[\x80\x82\x11\x15a\x17^W`\0\x81U`\x01\x01a\x17JV[P\x90V[\x805a\xFF\xFF\x81\x16\x81\x14a\x17tW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x17\x8BW`\0\x80\xFD[a\x17\x94\x82a\x17bV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x83W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x17\xC2W`\0\x80\xFD[\x815a\x17\x94\x81a\x17\x9BV[`\0\x80\x83`\x1F\x84\x01\x12a\x17\xDFW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xF6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x18\x0EW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x18(W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18>W`\0\x80\xFD[a\x18J\x85\x82\x86\x01a\x17\xCDV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x18kW`\0\x80\xFD[a\x18t\x84a\x17bV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x8FW`\0\x80\xFD[a\x18\x9B\x86\x82\x87\x01a\x17\xCDV[\x94\x97\x90\x96P\x93\x94PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x18\xCFW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xE8Wa\x18\xE8a\x18\xA8V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x19\x16Wa\x19\x16a\x18\xA8V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x19.W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x19`W`\0\x80\xFD[a\x19i\x84a\x17bV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\x84W`\0\x80\xFD[a\x19\x90\x86\x82\x87\x01a\x18\xBEV[\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x19\xC8W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x19\xACV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\x01`\x01`@\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x1A\x1B\x90\x83\x01\x84a\x19\xA2V[\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A7W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1A^W`\0\x80\xFD[a\x1Ag\x86a\x17bV[\x94P` \x86\x015a\x1Aw\x81a\x17\x9BV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\x92W`\0\x80\xFD[a\x1A\x9E\x88\x82\x89\x01a\x18\xBEV[\x93PP``\x86\x015\x80\x15\x15\x81\x14a\x1A\xB4W`\0\x80\xFD[\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xCFW`\0\x80\xFD[a\x1A\xDB\x88\x82\x89\x01a\x18\xBEV[\x91PP\x92\x95P\x92\x95\x90\x93PV[` \x81R`\0a\x17\x94` \x83\x01\x84a\x19\xA2V[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\x0EW`\0\x80\xFD[a\x1B\x17\x83a\x17bV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B2W`\0\x80\xFD[a\x1B>\x85\x82\x86\x01a\x18\xBEV[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B[W`\0\x80\xFD[a\x1Bd\x83a\x17bV[\x91P` \x83\x015a\x1Bt\x81a\x17\x9BV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x1B\x97W`\0\x80\xFD[a\x1B\xA0\x86a\x17bV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xBBW`\0\x80\xFD[a\x1B\xC7\x88\x82\x89\x01a\x17\xCDV[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xE6W`\0\x80\xFD[a\x1B\xF2\x88\x82\x89\x01a\x17\xCDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\x16W`\0\x80\xFD[\x825a\x1Bd\x81a\x17\x9BV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15a\x1C=W`\0\x80\xFD[a\x1CF\x89a\x17bV[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1CaW`\0\x80\xFD[a\x1Cm\x8B\x82\x8C\x01a\x17\xCDV[\x90\x98P\x96PP`@\x89\x015a\x1C\x81\x81a\x17\x9BV[\x94P``\x89\x015`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x1C\x9DW`\0\x80\xFD[\x93P`\x80\x89\x015\x92P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xBFW`\0\x80\xFD[a\x1C\xCB\x8B\x82\x8C\x01a\x17\xCDV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15a\x1C\xFBW`\0\x80\xFD[a\x1D\x04\x89a\x17bV[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x1FW`\0\x80\xFD[a\x1D+\x8B\x82\x8C\x01a\x17\xCDV[\x90\x98P\x96PP`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DJW`\0\x80\xFD[a\x1DV\x8B\x82\x8C\x01a\x17\xCDV[\x90\x96P\x94PP``\x89\x015a\x1Dj\x81a\x17\x9BV[\x92P`\x80\x89\x015a\x1Dz\x81a\x17\x9BV[\x91P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x95W`\0\x80\xFD[a\x1D\xA1\x8B\x82\x8C\x01a\x18\xBEV[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1D\xC7W`\0\x80\xFD[a\x1D\xD0\x85a\x17bV[\x93Pa\x1D\xDE` \x86\x01a\x17bV[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\0W`\0\x80\xFD[a\x1E\x0C\x87\x82\x88\x01a\x18\xBEV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1E.W`\0\x80\xFD[a\x1E7\x85a\x17bV[\x93Pa\x1EE` \x86\x01a\x17bV[\x92P`@\x85\x015a\x1EU\x81a\x17\x9BV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1E\x89W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1E\xA9WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x08JWa\x08Ja\x1E\xAFV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[a\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x1A\x1B`@\x83\x01\x84\x86a\x1E\xDCV[a\xFF\xFF\x87\x16\x81R`\x80` \x82\x01R`\0a\x1FA`\x80\x83\x01\x87\x89a\x1E\xDCV[`\x01`\x01`@\x1B\x03\x86\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra\x1Fc\x81\x85\x87a\x1E\xDCV[\x99\x98PPPPPPPPPV[a\xFF\xFF\x86\x16\x81R`\x80` \x82\x01R`\0a\x1F\x8E`\x80\x83\x01\x86\x88a\x1E\xDCV[`\x01`\x01`@\x1B\x03\x94\x90\x94\x16`@\x83\x01RP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16``\x90\x91\x01R\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x82\x16`\x01`\x01`@\x1B\x03\x81\x03a\x1F\xDBWa\x1F\xDBa\x1E\xAFV[`\x01\x01\x92\x91PPV[`\x1F\x82\x11\x15a .W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a \x0BWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a +W`\0\x81U`\x01\x01a \x17V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a LWa La\x18\xA8V[a `\x81a Z\x84Ta\x1EuV[\x84a\x1F\xE4V[` `\x1F\x82\x11`\x01\x81\x14a \x97W`\0\x83\x15a |WP\x84\x82\x01Q[`\x01\x84\x90\x1B`\0\x19`\x03\x86\x90\x1B\x1C\x19\x82\x16\x17[\x85UPa +V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a \xC7W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a \xA7V[P\x84\x82\x10\x15a \xE5W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03\x81\x81\x11\x15a\x08JWa\x08Ja\x1E\xAFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x08JWa\x08Ja\x1E\xAFV[\x81\x81\x03a!;WPPV[a!E\x82Ta\x1EuV[`\x01`\x01`@\x1B\x03\x81\x11\x15a!\\Wa!\\a\x18\xA8V[a!j\x81a Z\x84Ta\x1EuV[`\0`\x1F\x82\x11`\x01\x81\x14a!\x9CW`\0\x83\x15a |WP\x81\x85\x01T`\x01\x84\x90\x1B`\0\x19`\x03\x86\x90\x1B\x1C\x19\x82\x16\x17a \x8FV[`\0\x85\x81R` \x90 `\x1F\x19\x84\x16\x90`\0\x86\x81R` \x90 \x84[\x83\x81\x10\x15a!\xD6W\x82\x86\x01T\x82U`\x01\x95\x86\x01\x95\x90\x91\x01\x90` \x01a!\xB6V[P\x85\x83\x10\x15a!\xF4W\x81\x85\x01T`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\xFF\xFF\x89\x16\x81R`\xC0` \x82\x01R`\0a\"\"`\xC0\x83\x01\x89\x8Ba\x1E\xDCV[`\x01`\x01`\xA0\x1B\x03\x88\x16`@\x84\x01R`\x01`\x01`@\x1B\x03\x87\x16``\x84\x01R\x82\x81\x03`\x80\x84\x01Ra\"S\x81\x86\x88a\x1E\xDCV[\x90P\x82\x81\x03`\xA0\x84\x01Ra\"g\x81\x85a\x19\xA2V[\x9B\x9APPPPPPPPPPPV[a\xFF\xFF\x88\x16\x81R`\xC0` \x82\x01R`\0a\"\x93`\xC0\x83\x01\x89a\x19\xA2V[`\x01`\x01`\xA0\x1B\x03\x88\x16`@\x84\x01R`\x01`\x01`@\x1B\x03\x87\x16``\x84\x01R`\x80\x83\x01\x86\x90R\x82\x81\x03`\xA0\x84\x01Ra\"\xCB\x81\x85\x87a\x1E\xDCV[\x9A\x99PPPPPPPPPPV[a\xFF\xFF\x86\x16\x81R`\x80` \x82\x01R`\0a\"\xF7`\x80\x83\x01\x86\x88a\x1E\xDCV[`\x01`\x01`@\x1B\x03\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra#\x18\x81\x85a\x19\xA2V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 &\x1A\x8C\xE96\x85K\x8C!\xF8\ny\x8BK\x9C\xF7\xC0\"\xB6\xE4\xDAc\xC7Y\xEC\x82\x7F\xAD\xDE\xA9\r\x01dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MOCKLZENDPOINT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x020W`\x005`\xE0\x1C\x80c\x99oy\xC0\x11a\x01.W\x80c\xCA\x06k5\x11a\0\xABW\x80c\xDB\x14\xF3\x05\x11a\0oW\x80c\xDB\x14\xF3\x05\x14a\x07\xAEW\x80c\xE9a\xA9R\x14a\x07\xC9W\x80c\xE9zD\x8A\x14a\x074W\x80c\xF5\xEC\xBD\xBC\x14a\x07\xDFW\x80c\xFD\xC0|p\x14a\x08\x11W`\0\x80\xFD[\x80c\xCA\x06k5\x14a\x074W\x80c\xCB\xED\x8B\x9C\x14a\x07HW\x80c\xD21\x04\xF1\x14a\x07iW\x80c\xDA\x1A|\x9A\x14a\x02\x88W\x80c\xDA\xB3\x12w\x14a\x07\x88W`\0\x80\xFD[\x80c\xB2\x08d\x99\x11a\0\xF2W\x80c\xB2\x08d\x99\x14a\x06AW\x80c\xC0\x8F\x15\xA1\x14a\x06\x82W\x80c\xC2\xFAH\x13\x14a\x06\xCBW\x80c\xC5\x801\0\x14a\x06\xEBW\x80c\xC8\x1B8:\x14a\x06\xFEW`\0\x80\xFD[\x80c\x99oy\xC0\x14a\x05\xDAW\x80c\x9Cr\x9D\xA1\x14a\x04EW\x80c\xAA\xFF_\x16\x14a\x05\xF0W\x80c\xAF@j\xA5\x14a\x06\x10W\x80c\xB1!\x07p\x14a\x06&W`\0\x80\xFD[\x80c@\xA7\xBB\x10\x11a\x01\xBCW\x80cq\xBA/\xD6\x11a\x01\x80W\x80cq\xBA/\xD6\x14a\x04EW\x80cv\xA3\x86\xDC\x14a\x04eW\x80cz\x14WH\x14a\x04\xFEW\x80c\x7Fm\xF8\xE6\x14a\x05hW\x80c\x99$\xD3;\x14a\x05\x88W`\0\x80\xFD[\x80c@\xA7\xBB\x10\x14a\x03\x7FW\x80cB\xD6Z\x8D\x14a\x03\xB4W\x80cO\x05[\x04\x14a\x03\xD4W\x80cYR\xC4\xEC\x14a\x03\xF4W\x80c[0\x115\x14a\x04\x18W`\0\x80\xFD[\x80c\x0E\xAFn\xA6\x11a\x02\x03W\x80c\x0E\xAFn\xA6\x14a\x02\xE1W\x80c\x10\xDD\xB17\x14a\x02hW\x80c\x12\xA9\xEEk\x14a\x03\x11W\x80c&F\xAF\x08\x14a\x03@W\x80c4\x08\xE4p\x14a\x03fW`\0\x80\xFD[\x80c\x05\xBCq\x0F\x14a\x025W\x80c\x07\xE0\xDB\x17\x14a\x02hW\x80c\teh\xF6\x14a\x02\x88W\x80c\t\xED\xDFT\x14a\x02\xA9W[`\0\x80\xFD[4\x80\x15a\x02AW`\0\x80\xFD[P`\x04Ta\x02P\x90a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02tW`\0\x80\xFD[Pa\x02\x86a\x02\x836`\x04a\x17yV[PV[\0[4\x80\x15a\x02\x94W`\0\x80\xFD[Pa\x02Pa\x02\xA36`\x04a\x17\xB0V[P`\x01\x90V[4\x80\x15a\x02\xB5W`\0\x80\xFD[Pa\x02\xC9a\x02\xC46`\x04a\x18\x15V[a\x081V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02_V[4\x80\x15a\x02\xEDW`\0\x80\xFD[Pa\x03\x01a\x02\xFC6`\x04a\x18VV[a\x08PV[`@Q\x90\x15\x15\x81R` \x01a\x02_V[4\x80\x15a\x03\x1DW`\0\x80\xFD[Pa\x031a\x03,6`\x04a\x19KV[a\x08\x95V[`@Qa\x02_\x93\x92\x91\x90a\x19\xE8V[4\x80\x15a\x03LW`\0\x80\xFD[Pa\x02\x86a\x03[6`\x04a\x1A$V[`\x07\x91\x90\x91U`\x08UV[4\x80\x15a\x03rW`\0\x80\xFD[P`\x01Ta\xFF\xFF\x16a\x02PV[4\x80\x15a\x03\x8BW`\0\x80\xFD[Pa\x03\x9Fa\x03\x9A6`\x04a\x1AFV[a\t\x92V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02_V[4\x80\x15a\x03\xC0W`\0\x80\xFD[Pa\x02\x86a\x03\xCF6`\x04a\x18VV[a\t\xB5V[4\x80\x15a\x03\xE0W`\0\x80\xFD[P`\x02Ta\x02\xC9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\0W`\0\x80\xFD[Pa\x04\n`\x03T\x81V[`@Q\x90\x81R` \x01a\x02_V[4\x80\x15a\x04$W`\0\x80\xFD[Pa\x048a\x0436`\x04a\x17\xB0V[a\n\xFCV[`@Qa\x02_\x91\x90a\x1A\xE8V[4\x80\x15a\x04QW`\0\x80\xFD[Pa\x02\xC9a\x04`6`\x04a\x17\xB0V[P0\x90V[4\x80\x15a\x04qW`\0\x80\xFD[Pa\x04\xD1a\x04\x806`\x04a\x1A\xFBV[`\x0C` \x90\x81R`\0\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91R\x80T`\x01\x90\x91\x01T`\x01`\x01`@\x1B\x03\x82\x16\x91`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90\x83V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\x02_V[4\x80\x15a\x05\nW`\0\x80\xFD[Pa\x05Pa\x05\x196`\x04a\x1BHV[a\xFF\xFF\x82\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02_V[4\x80\x15a\x05tW`\0\x80\xFD[Pa\x04\na\x05\x836`\x04a\x18VV[a\x0B.V[4\x80\x15a\x05\x94W`\0\x80\xFD[Pa\x05Pa\x05\xA36`\x04a\x1A\xFBV[`\n` \x90\x81R`\0\x92\x83R`@\x90\x92 \x81Q\x80\x83\x01\x84\x01\x80Q\x92\x81R\x90\x84\x01\x92\x90\x93\x01\x91\x90\x91 \x91RT`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x05\xE6W`\0\x80\xFD[Pa\x04\n`\x07T\x81V[4\x80\x15a\x05\xFCW`\0\x80\xFD[Pa\x02\x86a\x06\x0B6`\x04a\x1B\x7FV[a\x0BjV[4\x80\x15a\x06\x1CW`\0\x80\xFD[Pa\x04\n`\x08T\x81V[4\x80\x15a\x062W`\0\x80\xFD[P`\x06Ta\x02P\x90a\xFF\xFF\x16\x81V[4\x80\x15a\x06MW`\0\x80\xFD[Pa\x05Pa\x06\\6`\x04a\x1BHV[`\x0B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x06\x8EW`\0\x80\xFD[Pa\x02\x86a\x06\x9D6`\x04a\x1C\x03V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UV[4\x80\x15a\x06\xD7W`\0\x80\xFD[Pa\x02\x86a\x06\xE66`\x04a\x1C!V[a\r\x80V[a\x02\x86a\x06\xF96`\x04a\x1C\xDFV[a\x12\xDEV[4\x80\x15a\x07\nW`\0\x80\xFD[Pa\x02\xC9a\x07\x196`\x04a\x17\xB0V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07@W`\0\x80\xFD[P`\0a\x03\x01V[4\x80\x15a\x07TW`\0\x80\xFD[Pa\x02\x86a\x07c6`\x04a\x1D\xB1V[PPPPV[4\x80\x15a\x07uW`\0\x80\xFD[Pa\x02\x86`\t\x80T`\xFF\x19\x16`\x01\x17\x90UV[4\x80\x15a\x07\x94W`\0\x80\xFD[P`\x01Ta\x02\xC9\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\xBAW`\0\x80\xFD[P`\x01Ta\x02P\x90a\xFF\xFF\x16\x81V[4\x80\x15a\x07\xD5W`\0\x80\xFD[Pa\x04\n`\x05T\x81V[4\x80\x15a\x07\xEBW`\0\x80\xFD[Pa\x048a\x07\xFA6`\x04a\x1E\x18V[`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x94\x93PPPPV[4\x80\x15a\x08\x1DW`\0\x80\xFD[Pa\x05Pa\x08,6`\x04a\x18VV[a\x14\xD8V[`\0\x80`@Q`\x02\x84\x01`\x02\x86\x03\x827`\t\x19\x01Q\x91PP[\x92\x91PPV[a\xFF\xFF\x83\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x90Q\x82\x91\x90a\x08v\x90\x86\x90\x86\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 `\x01\x01T\x15\x15\x91PP\x93\x92PPPV[`\r` \x90\x81R`\0\x84\x81R`@\x90 \x83Q\x80\x85\x01\x83\x01\x80Q\x92\x81R\x90\x83\x01\x92\x85\x01\x92\x90\x92 \x91R\x80T\x82\x90\x81\x10a\x08\xCCW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x83\x16\x96P`\x01`\xA0\x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x16\x94P\x91\x92Pa\t\x0F\x90a\x1EuV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t;\x90a\x1EuV[\x80\x15a\t\x88W\x80`\x1F\x10a\t]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x88V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tkW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[`\0\x80\x84Q`\x07Ta\t\xA4\x91\x90a\x1E\xC5V[\x91P`\x08T\x90P\x95P\x95\x93PPPPV[a\xFF\xFF\x83\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x90Qa\t\xD8\x90\x85\x90\x85\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 `\x01\x81\x01T\x90\x91Pa\n@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FLayerZero: no stored payload\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FLayerZero: invalid caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n7V[\x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U`\0`\x01\x82\x01U`@Q\x7F#\xD2hO9n\x92\xA6\xE2\xFF-\x16\xF9\x8Eo\xEA\0\xD5\x0C\xB2zd\xB51\xBC\x07H\xF70!\x1F\x98\x90a\n\xE9\x90\x86\x90\x86\x90\x86\x90a\x1F\x05V[`@Q\x80\x91\x03\x90\xA1a\x07c\x84\x84\x84a\x15\x1DV[`@\x80Q``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x83\x01R\x80Q\x80\x83\x03`\x14\x01\x81R`4\x90\x92\x01\x90R\x90V[a\xFF\xFF\x83\x16`\0\x90\x81R`\r` R`@\x80\x82 \x90Qa\x0BQ\x90\x85\x90\x85\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T\x90P\x93\x92PPPV[a\xFF\xFF\x85\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x90Qa\x0B\x8D\x90\x87\x90\x87\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 `\x01\x81\x01T\x90\x91Pa\x0B\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FLayerZero: no stored payload\0\0\0\0`D\x82\x01R`d\x01a\n7V[\x80T`\x01`\x01`@\x1B\x03\x16\x82\x14\x80\x15a\x0C#WP\x80`\x01\x01T\x83\x83`@Qa\x0C\x19\x92\x91\x90a\x1EeV[`@Q\x80\x91\x03\x90 \x14[a\x0CoW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FLayerZero: invalid payload\0\0\0\0\0\0`D\x82\x01R`d\x01a\n7V[\x80T`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x82U`\0`\x01\x83\x01\x81\x90Ua\xFF\xFF\x88\x16\x81R`\n` R`@\x80\x82 \x90Q`\x01`@\x1B\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x16\x92a\x0C\xB9\x90\x89\x90\x89\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 Tb\x1D5g`\xE0\x1B\x82R`\x01`\x01`@\x1B\x03\x16\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90b\x1D5g\x90a\r\x05\x90\x8B\x90\x8B\x90\x8B\x90\x87\x90\x8C\x90\x8C\x90`\x04\x01a\x1F#V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x1FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r3W=`\0\x80>=`\0\xFD[PPPP\x7Fa$4\xF3\x95\x81\xC8\xE7\xD9\x97F\xC9\xC2\x0Cn\xB0\xCE\x8C\x0E\xB9\x9F\0|W\x19\xD6 \x84\x13p\x95}\x88\x88\x88\x84\x86`@Qa\rn\x95\x94\x93\x92\x91\x90a\x1FpV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[a\xFF\xFF\x88\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x90Qa\r\xA3\x90\x8A\x90\x8A\x90a\x1EeV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x90P`\n`\0\x8Aa\xFF\xFF\x16a\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x88\x88`@Qa\r\xDE\x92\x91\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T`\0\x90a\x0E\x04\x90`\x01`\x01`@\x1B\x03\x16a\x1F\xB9V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90U`\x01`\x01`@\x1B\x03\x16\x85`\x01`\x01`@\x1B\x03\x16\x14a\x0E\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuLayerZero: wrong nonce`P\x1B`D\x82\x01R`d\x01a\n7V[`\x01\x81\x01T\x15a\x11FWa\xFF\xFF\x89\x16`\0\x90\x81R`\r` R`@\x80\x82 \x90Qa\x0E\xAD\x90\x8B\x90\x8B\x90a\x1EeV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x90P`\0`@Q\x80``\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x91RP\x82T\x90\x91P\x15a\x10\xD7W\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x90\x81\x90 \x84Q`\x02\x90\x94\x02\x01\x80T\x91\x85\x01Q`\x01`\x01`@\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x93\x90\x93\x17\x17\x82U`@\x83\x01Q\x83\x92\x91\x82\x01\x90a\x0F\x8E\x90\x82a 3V[PPP`\0[\x82Ta\x0F\xA2\x90`\x01\x90a \xF4V[\x81\x10\x15a\x10\\W\x82\x81\x81T\x81\x10a\x0F\xBBWa\x0F\xBBa!\x07V[\x90`\0R` `\0 \x90`\x02\x02\x01\x83\x82`\x01a\x0F\xD7\x91\x90a!\x1DV[\x81T\x81\x10a\x0F\xE7Wa\x0F\xE7a!\x07V[`\0\x91\x82R` \x90\x91 \x82T`\x02\x90\x92\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x82U\x83T`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x17`\x01`\xA0\x1B\x92\x83\x90\x04`\x01`\x01`@\x1B\x03\x16\x90\x92\x02\x91\x90\x91\x17\x81U`\x01\x80\x82\x01\x90a\x10Q\x90\x84\x01\x82a!0V[PPP`\x01\x01a\x0F\x94V[P\x80\x82`\0\x81T\x81\x10a\x10qWa\x10qa!\x07V[`\0\x91\x82R` \x91\x82\x90 \x83Q`\x02\x90\x92\x02\x01\x80T\x92\x84\x01Q`\x01`\x01`@\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x81U`@\x82\x01Q`\x01\x82\x01\x90a\x10\xCE\x90\x82a 3V[P\x90PPa\x11?V[\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x90\x81\x90 \x84Q`\x02\x90\x94\x02\x01\x80T\x91\x85\x01Q`\x01`\x01`@\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x93\x90\x93\x17\x17\x82U`@\x83\x01Q\x83\x92\x91\x82\x01\x90a\x11;\x90\x82a 3V[PPP[PPa\x12\xD3V[`\tT`\xFF\x16\x15a\x12lW`@Q\x80``\x01`@R\x80\x84\x84\x90P`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x84`@Qa\x11\x8D\x92\x91\x90a\x1EeV[`@\x80Q\x91\x82\x90\x03\x90\x91 \x90\x91Ra\xFF\xFF\x8B\x16`\0\x90\x81R`\x0C` R\x81\x90 \x90Qa\x11\xBC\x90\x8B\x90\x8B\x90a\x1EeV[\x90\x81R`@\x80Q\x91\x82\x90\x03` \x90\x81\x01\x83 \x84Q\x81T\x86\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x17\x81U\x93\x82\x01Q`\x01\x90\x94\x01\x93\x90\x93U\x91\x81\x01\x82R`\0\x81R\x90Q\x7F\x0F\x9EM\x95\xB6/\x08\"-a+Z\xB9 9\xCD\x8F\xBB\xBE\xA5P\xA9^\x8D\xF9\xF9'Ck\xBD\xF5\xDB\x91a\x12U\x91\x8C\x91\x8C\x91\x8C\x91\x8C\x91\x8C\x91\x8B\x91\x8B\x91\x90a\"\x04V[`@Q\x80\x91\x03\x90\xA1`\t\x80T`\xFF\x19\x16\x90Ua\x12\xD3V[`@Qb\x1D5g`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90b\x1D5g\x90a\x12\xA0\x90\x8C\x90\x8C\x90\x8C\x90\x8B\x90\x8A\x90\x8A\x90`\x04\x01a\x1F#V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xBAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xCEW=`\0\x80>=`\0\xFD[PPPP[PPPPPPPPPV[`\0a\x12\xEA\x88\x88a\x081V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x91\x92P\x16\x80a\x13{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FLayerZeroMock: destination Layer`D\x82\x01R\x7FZero Endpoint not found\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n7V[`\x07Ta\x13\x89\x90\x87\x90a\x1E\xC5V[4\x10\x15a\x13\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FLayerZeroMock: not enough native`D\x82\x01Rh for fees`\xB8\x1B`d\x82\x01R`\x84\x01a\n7V[a\xFF\xFF\x8A\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x82\x90a\x14\x1E\x90`\x01`\x01`@\x1B\x03\x16a\x1F\xB9V[\x82T`\x01`\x01`@\x1B\x03\x80\x83\x16a\x01\0\x94\x90\x94\n\x93\x84\x02\x93\x02\x19\x16\x91\x90\x91\x17\x90\x91U\x90P`\0a\x14M3a\n\xFCV[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16c\xC2\xFAH\x13`\x01`\0\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16\x83\x87\x86`\0\x8F\x8F`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x98\x97\x96\x95\x94\x93\x92\x91\x90a\"vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\xB2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xC6W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPPV[a\xFF\xFF\x83\x16`\0\x90\x81R`\n` R`@\x80\x82 \x90Qa\x14\xFB\x90\x85\x90\x85\x90a\x1EeV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`@\x1B\x03\x16\x90P\x93\x92PPPV[a\xFF\xFF\x83\x16`\0\x90\x81R`\r` R`@\x80\x82 \x90Qa\x15@\x90\x85\x90\x85\x90a\x1EeV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x90P[\x80T\x15a\x07cW\x80T`\0\x90\x82\x90a\x15k\x90`\x01\x90a \xF4V[\x81T\x81\x10a\x15{Wa\x15{a!\x07V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R`\x02\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x84R`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x90\x91\x04\x16\x93\x83\x01\x93\x90\x93R`\x01\x83\x01\x80T\x92\x93\x92\x91\x84\x01\x91a\x15\xD4\x90a\x1EuV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\0\x90a\x1EuV[\x80\x15a\x16MW\x80`\x1F\x10a\x16\"Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16MV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x160W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16b\x1D5g\x86\x86\x86\x85` \x01Q\x86`@\x01Q`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x97\x95\x94\x93\x92\x91\x90a\"\xD9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xC5W=`\0\x80>=`\0\xFD[PPPP\x81\x80T\x80a\x16\xD9Wa\x16\xD9a#$V[`\0\x82\x81R` \x81 `\x02`\0\x19\x90\x93\x01\x92\x83\x02\x01\x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\x17\n`\x01\x83\x01\x82a\x17\x14V[PP\x90UPa\x15QV[P\x80Ta\x17 \x90a\x1EuV[`\0\x82U\x80`\x1F\x10a\x170WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x02\x83\x91\x90[\x80\x82\x11\x15a\x17^W`\0\x81U`\x01\x01a\x17JV[P\x90V[\x805a\xFF\xFF\x81\x16\x81\x14a\x17tW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x17\x8BW`\0\x80\xFD[a\x17\x94\x82a\x17bV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x83W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x17\xC2W`\0\x80\xFD[\x815a\x17\x94\x81a\x17\x9BV[`\0\x80\x83`\x1F\x84\x01\x12a\x17\xDFW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xF6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x18\x0EW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x18(W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18>W`\0\x80\xFD[a\x18J\x85\x82\x86\x01a\x17\xCDV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x18kW`\0\x80\xFD[a\x18t\x84a\x17bV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x8FW`\0\x80\xFD[a\x18\x9B\x86\x82\x87\x01a\x17\xCDV[\x94\x97\x90\x96P\x93\x94PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x18\xCFW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xE8Wa\x18\xE8a\x18\xA8V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x19\x16Wa\x19\x16a\x18\xA8V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x19.W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x19`W`\0\x80\xFD[a\x19i\x84a\x17bV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\x84W`\0\x80\xFD[a\x19\x90\x86\x82\x87\x01a\x18\xBEV[\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x19\xC8W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x19\xACV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\x01`\x01`@\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x1A\x1B\x90\x83\x01\x84a\x19\xA2V[\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A7W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1A^W`\0\x80\xFD[a\x1Ag\x86a\x17bV[\x94P` \x86\x015a\x1Aw\x81a\x17\x9BV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\x92W`\0\x80\xFD[a\x1A\x9E\x88\x82\x89\x01a\x18\xBEV[\x93PP``\x86\x015\x80\x15\x15\x81\x14a\x1A\xB4W`\0\x80\xFD[\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xCFW`\0\x80\xFD[a\x1A\xDB\x88\x82\x89\x01a\x18\xBEV[\x91PP\x92\x95P\x92\x95\x90\x93PV[` \x81R`\0a\x17\x94` \x83\x01\x84a\x19\xA2V[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\x0EW`\0\x80\xFD[a\x1B\x17\x83a\x17bV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B2W`\0\x80\xFD[a\x1B>\x85\x82\x86\x01a\x18\xBEV[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B[W`\0\x80\xFD[a\x1Bd\x83a\x17bV[\x91P` \x83\x015a\x1Bt\x81a\x17\x9BV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x1B\x97W`\0\x80\xFD[a\x1B\xA0\x86a\x17bV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xBBW`\0\x80\xFD[a\x1B\xC7\x88\x82\x89\x01a\x17\xCDV[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xE6W`\0\x80\xFD[a\x1B\xF2\x88\x82\x89\x01a\x17\xCDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\x16W`\0\x80\xFD[\x825a\x1Bd\x81a\x17\x9BV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15a\x1C=W`\0\x80\xFD[a\x1CF\x89a\x17bV[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1CaW`\0\x80\xFD[a\x1Cm\x8B\x82\x8C\x01a\x17\xCDV[\x90\x98P\x96PP`@\x89\x015a\x1C\x81\x81a\x17\x9BV[\x94P``\x89\x015`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x1C\x9DW`\0\x80\xFD[\x93P`\x80\x89\x015\x92P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xBFW`\0\x80\xFD[a\x1C\xCB\x8B\x82\x8C\x01a\x17\xCDV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15a\x1C\xFBW`\0\x80\xFD[a\x1D\x04\x89a\x17bV[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x1FW`\0\x80\xFD[a\x1D+\x8B\x82\x8C\x01a\x17\xCDV[\x90\x98P\x96PP`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DJW`\0\x80\xFD[a\x1DV\x8B\x82\x8C\x01a\x17\xCDV[\x90\x96P\x94PP``\x89\x015a\x1Dj\x81a\x17\x9BV[\x92P`\x80\x89\x015a\x1Dz\x81a\x17\x9BV[\x91P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x95W`\0\x80\xFD[a\x1D\xA1\x8B\x82\x8C\x01a\x18\xBEV[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1D\xC7W`\0\x80\xFD[a\x1D\xD0\x85a\x17bV[\x93Pa\x1D\xDE` \x86\x01a\x17bV[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\0W`\0\x80\xFD[a\x1E\x0C\x87\x82\x88\x01a\x18\xBEV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1E.W`\0\x80\xFD[a\x1E7\x85a\x17bV[\x93Pa\x1EE` \x86\x01a\x17bV[\x92P`@\x85\x015a\x1EU\x81a\x17\x9BV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1E\x89W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1E\xA9WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x08JWa\x08Ja\x1E\xAFV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[a\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x1A\x1B`@\x83\x01\x84\x86a\x1E\xDCV[a\xFF\xFF\x87\x16\x81R`\x80` \x82\x01R`\0a\x1FA`\x80\x83\x01\x87\x89a\x1E\xDCV[`\x01`\x01`@\x1B\x03\x86\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra\x1Fc\x81\x85\x87a\x1E\xDCV[\x99\x98PPPPPPPPPV[a\xFF\xFF\x86\x16\x81R`\x80` \x82\x01R`\0a\x1F\x8E`\x80\x83\x01\x86\x88a\x1E\xDCV[`\x01`\x01`@\x1B\x03\x94\x90\x94\x16`@\x83\x01RP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16``\x90\x91\x01R\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x82\x16`\x01`\x01`@\x1B\x03\x81\x03a\x1F\xDBWa\x1F\xDBa\x1E\xAFV[`\x01\x01\x92\x91PPV[`\x1F\x82\x11\x15a .W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a \x0BWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a +W`\0\x81U`\x01\x01a \x17V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a LWa La\x18\xA8V[a `\x81a Z\x84Ta\x1EuV[\x84a\x1F\xE4V[` `\x1F\x82\x11`\x01\x81\x14a \x97W`\0\x83\x15a |WP\x84\x82\x01Q[`\x01\x84\x90\x1B`\0\x19`\x03\x86\x90\x1B\x1C\x19\x82\x16\x17[\x85UPa +V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a \xC7W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a \xA7V[P\x84\x82\x10\x15a \xE5W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03\x81\x81\x11\x15a\x08JWa\x08Ja\x1E\xAFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x08JWa\x08Ja\x1E\xAFV[\x81\x81\x03a!;WPPV[a!E\x82Ta\x1EuV[`\x01`\x01`@\x1B\x03\x81\x11\x15a!\\Wa!\\a\x18\xA8V[a!j\x81a Z\x84Ta\x1EuV[`\0`\x1F\x82\x11`\x01\x81\x14a!\x9CW`\0\x83\x15a |WP\x81\x85\x01T`\x01\x84\x90\x1B`\0\x19`\x03\x86\x90\x1B\x1C\x19\x82\x16\x17a \x8FV[`\0\x85\x81R` \x90 `\x1F\x19\x84\x16\x90`\0\x86\x81R` \x90 \x84[\x83\x81\x10\x15a!\xD6W\x82\x86\x01T\x82U`\x01\x95\x86\x01\x95\x90\x91\x01\x90` \x01a!\xB6V[P\x85\x83\x10\x15a!\xF4W\x81\x85\x01T`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\xFF\xFF\x89\x16\x81R`\xC0` \x82\x01R`\0a\"\"`\xC0\x83\x01\x89\x8Ba\x1E\xDCV[`\x01`\x01`\xA0\x1B\x03\x88\x16`@\x84\x01R`\x01`\x01`@\x1B\x03\x87\x16``\x84\x01R\x82\x81\x03`\x80\x84\x01Ra\"S\x81\x86\x88a\x1E\xDCV[\x90P\x82\x81\x03`\xA0\x84\x01Ra\"g\x81\x85a\x19\xA2V[\x9B\x9APPPPPPPPPPPV[a\xFF\xFF\x88\x16\x81R`\xC0` \x82\x01R`\0a\"\x93`\xC0\x83\x01\x89a\x19\xA2V[`\x01`\x01`\xA0\x1B\x03\x88\x16`@\x84\x01R`\x01`\x01`@\x1B\x03\x87\x16``\x84\x01R`\x80\x83\x01\x86\x90R\x82\x81\x03`\xA0\x84\x01Ra\"\xCB\x81\x85\x87a\x1E\xDCV[\x9A\x99PPPPPPPPPPV[a\xFF\xFF\x86\x16\x81R`\x80` \x82\x01R`\0a\"\xF7`\x80\x83\x01\x86\x88a\x1E\xDCV[`\x01`\x01`@\x1B\x03\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra#\x18\x81\x85a\x19\xA2V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 &\x1A\x8C\xE96\x85K\x8C!\xF8\ny\x8BK\x9C\xF7\xC0\"\xB6\xE4\xDAc\xC7Y\xEC\x82\x7F\xAD\xDE\xA9\r\x01dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKLZENDPOINT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockLZEndpoint<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockLZEndpoint<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockLZEndpoint<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockLZEndpoint<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockLZEndpoint<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockLZEndpoint))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockLZEndpoint<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKLZENDPOINT_ABI.clone(),
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
                MOCKLZENDPOINT_ABI.clone(),
                MOCKLZENDPOINT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addrToPackedBytes` (0x5b301135) function
        pub fn addr_to_packed_bytes(
            &self,
            a: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([91, 48, 17, 53], a)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blockNextMsg` (0xd23104f1) function
        pub fn block_next_msg(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 49, 4, 241], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `estimateFees` (0x40a7bb10) function
        pub fn estimate_fees(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Address,
            payload: ::ethers::core::types::Bytes,
            p3: bool,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, (::ethers::core::types::U256, ::ethers::core::types::U256)>
        {
            self.0
                .method_hash([64, 167, 187, 16], (p0, p1, payload, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forceResumeReceive` (0x42d65a8d) function
        pub fn force_resume_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 214, 90, 141], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(&self) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfig` (0xf5ecbdbc) function
        pub fn get_config(
            &self,
            p0: u16,
            p1: u16,
            p2: ::ethers::core::types::Address,
            p3: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([245, 236, 189, 188], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInboundNonce` (0xfdc07c70) function
        pub fn get_inbound_nonce(
            &self,
            chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([253, 192, 124, 112], (chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLengthOfQueue` (0x7f6df8e6) function
        pub fn get_length_of_queue(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([127, 109, 248, 230], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOutboundNonce` (0x7a145748) function
        pub fn get_outbound_nonce(
            &self,
            chain_id: u16,
            src_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([122, 20, 87, 72], (chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReceiveLibraryAddress` (0x71ba2fd6) function
        pub fn get_receive_library_address(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([113, 186, 47, 214], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReceiveVersion` (0xda1a7c9a) function
        pub fn get_receive_version(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([218, 26, 124, 154], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSendLibraryAddress` (0x9c729da1) function
        pub fn get_send_library_address(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([156, 114, 157, 161], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSendVersion` (0x096568f6) function
        pub fn get_send_version(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([9, 101, 104, 246], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasStoredPayload` (0x0eaf6ea6) function
        pub fn has_stored_payload(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 175, 110, 166], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inboundNonce` (0x9924d33b) function
        pub fn inbound_nonce(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([153, 36, 211, 59], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isReceivingPayload` (0xca066b35) function
        pub fn is_receiving_payload(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([202, 6, 107, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSendingPayload` (0xe97a448a) function
        pub fn is_sending_payload(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 122, 68, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lzEndpointLookup` (0xc81b383a) function
        pub fn lz_endpoint_lookup(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([200, 27, 56, 58], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockBlockConfirmations` (0x5952c4ec) function
        pub fn mock_block_confirmations(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([89, 82, 196, 236], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockChainId` (0xdb14f305) function
        pub fn mock_chain_id(&self) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([219, 20, 243, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockLayerZeroVersion` (0xb1210770) function
        pub fn mock_layer_zero_version(&self) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([177, 33, 7, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockLibraryVersion` (0x05bc710f) function
        pub fn mock_library_version(&self) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([5, 188, 113, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockOracle` (0xdab31277) function
        pub fn mock_oracle(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([218, 179, 18, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockRelayer` (0x4f055b04) function
        pub fn mock_relayer(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([79, 5, 91, 4], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockStaticNativeFee` (0xe961a952) function
        pub fn mock_static_native_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([233, 97, 169, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `msgsToDeliver` (0x12a9ee6b) function
        pub fn msgs_to_deliver(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Bytes,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u64, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([18, 169, 238, 107], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nativeFee` (0x996f79c0) function
        pub fn native_fee(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([153, 111, 121, 192], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `outboundNonce` (0xb2086499) function
        pub fn outbound_nonce(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([178, 8, 100, 153], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `packedBytesToAddr` (0x09eddf54) function
        pub fn packed_bytes_to_addr(
            &self,
            b: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([9, 237, 223, 84], b)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `receivePayload` (0xc2fa4813) function
        pub fn receive_payload(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
            dst_address: ::ethers::core::types::Address,
            nonce: u64,
            p4: ::ethers::core::types::U256,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [194, 250, 72, 19],
                    (src_chain_id, src_address, dst_address, nonce, p4, payload),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `retryPayload` (0xaaff5f16) function
        pub fn retry_payload(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 255, 95, 22], (src_chain_id, src_address, payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `send` (0xc5803100) function
        pub fn send(
            &self,
            chain_id: u16,
            destination: ::ethers::core::types::Bytes,
            payload: ::ethers::core::types::Bytes,
            p3: ::ethers::core::types::Address,
            p4: ::ethers::core::types::Address,
            adapter_params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [197, 128, 49, 0],
                    (chain_id, destination, payload, p3, p4, adapter_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0xcbed8b9c) function
        pub fn set_config(
            &self,
            p0: u16,
            p1: u16,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 237, 139, 156], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDestLzEndpoint` (0xc08f15a1) function
        pub fn set_dest_lz_endpoint(
            &self,
            dest_addr: ::ethers::core::types::Address,
            lz_endpoint_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 143, 21, 161], (dest_addr, lz_endpoint_addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEstimatedFees` (0x2646af08) function
        pub fn set_estimated_fees(
            &self,
            native_fee: ::ethers::core::types::U256,
            zro_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 70, 175, 8], (native_fee, zro_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReceiveVersion` (0x10ddb137) function
        pub fn set_receive_version(&self, p0: u16) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 221, 177, 55], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSendVersion` (0x07e0db17) function
        pub fn set_send_version(&self, p0: u16) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 224, 219, 23], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `storedPayload` (0x76a386dc) function
        pub fn stored_payload(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, ::ethers::core::types::Address, [u8; 32])> {
            self.0
                .method_hash([118, 163, 134, 220], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `zroFee` (0xaf406aa5) function
        pub fn zro_fee(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 64, 106, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PayloadCleared` event
        pub fn payload_cleared_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PayloadClearedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PayloadStored` event
        pub fn payload_stored_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PayloadStoredFilter> {
            self.0.event()
        }
        ///Gets the contract's `UaForceResumeReceive` event
        pub fn ua_force_resume_receive_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UaForceResumeReceiveFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(&self) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MockLZEndpointEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockLZEndpoint<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "PayloadCleared", abi = "PayloadCleared(uint16,bytes,uint64,address)")]
    pub struct PayloadClearedFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub dst_address: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "PayloadStored",
        abi = "PayloadStored(uint16,bytes,address,uint64,bytes,bytes)"
    )]
    pub struct PayloadStoredFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub dst_address: ::ethers::core::types::Address,
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
        pub reason: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "UaForceResumeReceive", abi = "UaForceResumeReceive(uint16,bytes)")]
    pub struct UaForceResumeReceiveFilter {
        pub chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockLZEndpointEvents {
        PayloadClearedFilter(PayloadClearedFilter),
        PayloadStoredFilter(PayloadStoredFilter),
        UaForceResumeReceiveFilter(UaForceResumeReceiveFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockLZEndpointEvents {
        fn decode_log(log: &::ethers::core::abi::RawLog) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PayloadClearedFilter::decode_log(log) {
                return Ok(MockLZEndpointEvents::PayloadClearedFilter(decoded));
            }
            if let Ok(decoded) = PayloadStoredFilter::decode_log(log) {
                return Ok(MockLZEndpointEvents::PayloadStoredFilter(decoded));
            }
            if let Ok(decoded) = UaForceResumeReceiveFilter::decode_log(log) {
                return Ok(MockLZEndpointEvents::UaForceResumeReceiveFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockLZEndpointEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PayloadClearedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayloadStoredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UaForceResumeReceiveFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PayloadClearedFilter> for MockLZEndpointEvents {
        fn from(value: PayloadClearedFilter) -> Self {
            Self::PayloadClearedFilter(value)
        }
    }
    impl ::core::convert::From<PayloadStoredFilter> for MockLZEndpointEvents {
        fn from(value: PayloadStoredFilter) -> Self {
            Self::PayloadStoredFilter(value)
        }
    }
    impl ::core::convert::From<UaForceResumeReceiveFilter> for MockLZEndpointEvents {
        fn from(value: UaForceResumeReceiveFilter) -> Self {
            Self::UaForceResumeReceiveFilter(value)
        }
    }
    ///Container type for all input parameters for the `addrToPackedBytes` function with signature `addrToPackedBytes(address)` and selector `0x5b301135`
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
    #[ethcall(name = "addrToPackedBytes", abi = "addrToPackedBytes(address)")]
    pub struct AddrToPackedBytesCall {
        pub a: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `blockNextMsg` function with signature `blockNextMsg()` and selector `0xd23104f1`
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
    #[ethcall(name = "blockNextMsg", abi = "blockNextMsg()")]
    pub struct BlockNextMsgCall;
    ///Container type for all input parameters for the `estimateFees` function with signature `estimateFees(uint16,address,bytes,bool,bytes)` and selector `0x40a7bb10`
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
    #[ethcall(name = "estimateFees", abi = "estimateFees(uint16,address,bytes,bool,bytes)")]
    pub struct EstimateFeesCall {
        pub p0: u16,
        pub p1: ::ethers::core::types::Address,
        pub payload: ::ethers::core::types::Bytes,
        pub p3: bool,
        pub p4: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `forceResumeReceive` function with signature `forceResumeReceive(uint16,bytes)` and selector `0x42d65a8d`
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
    #[ethcall(name = "forceResumeReceive", abi = "forceResumeReceive(uint16,bytes)")]
    pub struct ForceResumeReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    ///Container type for all input parameters for the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `0xf5ecbdbc`
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
    #[ethcall(name = "getConfig", abi = "getConfig(uint16,uint16,address,uint256)")]
    pub struct GetConfigCall(
        pub u16,
        pub u16,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `getInboundNonce` function with signature `getInboundNonce(uint16,bytes)` and selector `0xfdc07c70`
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
    #[ethcall(name = "getInboundNonce", abi = "getInboundNonce(uint16,bytes)")]
    pub struct GetInboundNonceCall {
        pub chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getLengthOfQueue` function with signature `getLengthOfQueue(uint16,bytes)` and selector `0x7f6df8e6`
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
    #[ethcall(name = "getLengthOfQueue", abi = "getLengthOfQueue(uint16,bytes)")]
    pub struct GetLengthOfQueueCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getOutboundNonce` function with signature `getOutboundNonce(uint16,address)` and selector `0x7a145748`
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
    #[ethcall(name = "getOutboundNonce", abi = "getOutboundNonce(uint16,address)")]
    pub struct GetOutboundNonceCall {
        pub chain_id: u16,
        pub src_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReceiveLibraryAddress` function with signature `getReceiveLibraryAddress(address)` and selector `0x71ba2fd6`
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
    #[ethcall(name = "getReceiveLibraryAddress", abi = "getReceiveLibraryAddress(address)")]
    pub struct GetReceiveLibraryAddressCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getReceiveVersion` function with signature `getReceiveVersion(address)` and selector `0xda1a7c9a`
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
    #[ethcall(name = "getReceiveVersion", abi = "getReceiveVersion(address)")]
    pub struct GetReceiveVersionCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getSendLibraryAddress` function with signature `getSendLibraryAddress(address)` and selector `0x9c729da1`
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
    #[ethcall(name = "getSendLibraryAddress", abi = "getSendLibraryAddress(address)")]
    pub struct GetSendLibraryAddressCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getSendVersion` function with signature `getSendVersion(address)` and selector `0x096568f6`
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
    #[ethcall(name = "getSendVersion", abi = "getSendVersion(address)")]
    pub struct GetSendVersionCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `hasStoredPayload` function with signature `hasStoredPayload(uint16,bytes)` and selector `0x0eaf6ea6`
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
    #[ethcall(name = "hasStoredPayload", abi = "hasStoredPayload(uint16,bytes)")]
    pub struct HasStoredPayloadCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `inboundNonce` function with signature `inboundNonce(uint16,bytes)` and selector `0x9924d33b`
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
    #[ethcall(name = "inboundNonce", abi = "inboundNonce(uint16,bytes)")]
    pub struct InboundNonceCall(pub u16, pub ::ethers::core::types::Bytes);
    ///Container type for all input parameters for the `isReceivingPayload` function with signature `isReceivingPayload()` and selector `0xca066b35`
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
    #[ethcall(name = "isReceivingPayload", abi = "isReceivingPayload()")]
    pub struct IsReceivingPayloadCall;
    ///Container type for all input parameters for the `isSendingPayload` function with signature `isSendingPayload()` and selector `0xe97a448a`
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
    #[ethcall(name = "isSendingPayload", abi = "isSendingPayload()")]
    pub struct IsSendingPayloadCall;
    ///Container type for all input parameters for the `lzEndpointLookup` function with signature `lzEndpointLookup(address)` and selector `0xc81b383a`
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
    #[ethcall(name = "lzEndpointLookup", abi = "lzEndpointLookup(address)")]
    pub struct LzEndpointLookupCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `mockBlockConfirmations` function with signature `mockBlockConfirmations()` and selector `0x5952c4ec`
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
    #[ethcall(name = "mockBlockConfirmations", abi = "mockBlockConfirmations()")]
    pub struct MockBlockConfirmationsCall;
    ///Container type for all input parameters for the `mockChainId` function with signature `mockChainId()` and selector `0xdb14f305`
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
    #[ethcall(name = "mockChainId", abi = "mockChainId()")]
    pub struct MockChainIdCall;
    ///Container type for all input parameters for the `mockLayerZeroVersion` function with signature `mockLayerZeroVersion()` and selector `0xb1210770`
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
    #[ethcall(name = "mockLayerZeroVersion", abi = "mockLayerZeroVersion()")]
    pub struct MockLayerZeroVersionCall;
    ///Container type for all input parameters for the `mockLibraryVersion` function with signature `mockLibraryVersion()` and selector `0x05bc710f`
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
    #[ethcall(name = "mockLibraryVersion", abi = "mockLibraryVersion()")]
    pub struct MockLibraryVersionCall;
    ///Container type for all input parameters for the `mockOracle` function with signature `mockOracle()` and selector `0xdab31277`
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
    #[ethcall(name = "mockOracle", abi = "mockOracle()")]
    pub struct MockOracleCall;
    ///Container type for all input parameters for the `mockRelayer` function with signature `mockRelayer()` and selector `0x4f055b04`
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
    #[ethcall(name = "mockRelayer", abi = "mockRelayer()")]
    pub struct MockRelayerCall;
    ///Container type for all input parameters for the `mockStaticNativeFee` function with signature `mockStaticNativeFee()` and selector `0xe961a952`
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
    #[ethcall(name = "mockStaticNativeFee", abi = "mockStaticNativeFee()")]
    pub struct MockStaticNativeFeeCall;
    ///Container type for all input parameters for the `msgsToDeliver` function with signature `msgsToDeliver(uint16,bytes,uint256)` and selector `0x12a9ee6b`
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
    #[ethcall(name = "msgsToDeliver", abi = "msgsToDeliver(uint16,bytes,uint256)")]
    pub struct MsgsToDeliverCall(
        pub u16,
        pub ::ethers::core::types::Bytes,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `nativeFee` function with signature `nativeFee()` and selector `0x996f79c0`
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
    #[ethcall(name = "nativeFee", abi = "nativeFee()")]
    pub struct NativeFeeCall;
    ///Container type for all input parameters for the `outboundNonce` function with signature `outboundNonce(uint16,address)` and selector `0xb2086499`
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
    #[ethcall(name = "outboundNonce", abi = "outboundNonce(uint16,address)")]
    pub struct OutboundNonceCall(pub u16, pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `packedBytesToAddr` function with signature `packedBytesToAddr(bytes)` and selector `0x09eddf54`
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
    #[ethcall(name = "packedBytesToAddr", abi = "packedBytesToAddr(bytes)")]
    pub struct PackedBytesToAddrCall {
        pub b: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `receivePayload` function with signature `receivePayload(uint16,bytes,address,uint64,uint256,bytes)` and selector `0xc2fa4813`
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
        name = "receivePayload",
        abi = "receivePayload(uint16,bytes,address,uint64,uint256,bytes)"
    )]
    pub struct ReceivePayloadCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub dst_address: ::ethers::core::types::Address,
        pub nonce: u64,
        pub p4: ::ethers::core::types::U256,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `retryPayload` function with signature `retryPayload(uint16,bytes,bytes)` and selector `0xaaff5f16`
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
    #[ethcall(name = "retryPayload", abi = "retryPayload(uint16,bytes,bytes)")]
    pub struct RetryPayloadCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `send` function with signature `send(uint16,bytes,bytes,address,address,bytes)` and selector `0xc5803100`
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
    #[ethcall(name = "send", abi = "send(uint16,bytes,bytes,address,address,bytes)")]
    pub struct SendCall {
        pub chain_id: u16,
        pub destination: ::ethers::core::types::Bytes,
        pub payload: ::ethers::core::types::Bytes,
        pub p3: ::ethers::core::types::Address,
        pub p4: ::ethers::core::types::Address,
        pub adapter_params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig(uint16,uint16,uint256,bytes)` and selector `0xcbed8b9c`
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
    #[ethcall(name = "setConfig", abi = "setConfig(uint16,uint16,uint256,bytes)")]
    pub struct SetConfigCall(
        pub u16,
        pub u16,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `setDestLzEndpoint` function with signature `setDestLzEndpoint(address,address)` and selector `0xc08f15a1`
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
    #[ethcall(name = "setDestLzEndpoint", abi = "setDestLzEndpoint(address,address)")]
    pub struct SetDestLzEndpointCall {
        pub dest_addr: ::ethers::core::types::Address,
        pub lz_endpoint_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setEstimatedFees` function with signature `setEstimatedFees(uint256,uint256)` and selector `0x2646af08`
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
    #[ethcall(name = "setEstimatedFees", abi = "setEstimatedFees(uint256,uint256)")]
    pub struct SetEstimatedFeesCall {
        pub native_fee: ::ethers::core::types::U256,
        pub zro_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setReceiveVersion` function with signature `setReceiveVersion(uint16)` and selector `0x10ddb137`
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
    #[ethcall(name = "setReceiveVersion", abi = "setReceiveVersion(uint16)")]
    pub struct SetReceiveVersionCall(pub u16);
    ///Container type for all input parameters for the `setSendVersion` function with signature `setSendVersion(uint16)` and selector `0x07e0db17`
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
    #[ethcall(name = "setSendVersion", abi = "setSendVersion(uint16)")]
    pub struct SetSendVersionCall(pub u16);
    ///Container type for all input parameters for the `storedPayload` function with signature `storedPayload(uint16,bytes)` and selector `0x76a386dc`
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
    #[ethcall(name = "storedPayload", abi = "storedPayload(uint16,bytes)")]
    pub struct StoredPayloadCall(pub u16, pub ::ethers::core::types::Bytes);
    ///Container type for all input parameters for the `zroFee` function with signature `zroFee()` and selector `0xaf406aa5`
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
    #[ethcall(name = "zroFee", abi = "zroFee()")]
    pub struct ZroFeeCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockLZEndpointCalls {
        AddrToPackedBytes(AddrToPackedBytesCall),
        BlockNextMsg(BlockNextMsgCall),
        EstimateFees(EstimateFeesCall),
        ForceResumeReceive(ForceResumeReceiveCall),
        GetChainId(GetChainIdCall),
        GetConfig(GetConfigCall),
        GetInboundNonce(GetInboundNonceCall),
        GetLengthOfQueue(GetLengthOfQueueCall),
        GetOutboundNonce(GetOutboundNonceCall),
        GetReceiveLibraryAddress(GetReceiveLibraryAddressCall),
        GetReceiveVersion(GetReceiveVersionCall),
        GetSendLibraryAddress(GetSendLibraryAddressCall),
        GetSendVersion(GetSendVersionCall),
        HasStoredPayload(HasStoredPayloadCall),
        InboundNonce(InboundNonceCall),
        IsReceivingPayload(IsReceivingPayloadCall),
        IsSendingPayload(IsSendingPayloadCall),
        LzEndpointLookup(LzEndpointLookupCall),
        MockBlockConfirmations(MockBlockConfirmationsCall),
        MockChainId(MockChainIdCall),
        MockLayerZeroVersion(MockLayerZeroVersionCall),
        MockLibraryVersion(MockLibraryVersionCall),
        MockOracle(MockOracleCall),
        MockRelayer(MockRelayerCall),
        MockStaticNativeFee(MockStaticNativeFeeCall),
        MsgsToDeliver(MsgsToDeliverCall),
        NativeFee(NativeFeeCall),
        OutboundNonce(OutboundNonceCall),
        PackedBytesToAddr(PackedBytesToAddrCall),
        ReceivePayload(ReceivePayloadCall),
        RetryPayload(RetryPayloadCall),
        Send(SendCall),
        SetConfig(SetConfigCall),
        SetDestLzEndpoint(SetDestLzEndpointCall),
        SetEstimatedFees(SetEstimatedFeesCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetSendVersion(SetSendVersionCall),
        StoredPayload(StoredPayloadCall),
        ZroFee(ZroFeeCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockLZEndpointCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddrToPackedBytesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddrToPackedBytes(decoded));
            }
            if let Ok(decoded) = <BlockNextMsgCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BlockNextMsg(decoded));
            }
            if let Ok(decoded) = <EstimateFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EstimateFees(decoded));
            }
            if let Ok(decoded) = <ForceResumeReceiveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ForceResumeReceive(decoded));
            }
            if let Ok(decoded) = <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded) = <GetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetConfig(decoded));
            }
            if let Ok(decoded) = <GetInboundNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetInboundNonce(decoded));
            }
            if let Ok(decoded) = <GetLengthOfQueueCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetLengthOfQueue(decoded));
            }
            if let Ok(decoded) = <GetOutboundNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetOutboundNonce(decoded));
            }
            if let Ok(decoded) = <GetReceiveLibraryAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReceiveLibraryAddress(decoded));
            }
            if let Ok(decoded) = <GetReceiveVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReceiveVersion(decoded));
            }
            if let Ok(decoded) = <GetSendLibraryAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSendLibraryAddress(decoded));
            }
            if let Ok(decoded) = <GetSendVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSendVersion(decoded));
            }
            if let Ok(decoded) = <HasStoredPayloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasStoredPayload(decoded));
            }
            if let Ok(decoded) = <InboundNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InboundNonce(decoded));
            }
            if let Ok(decoded) = <IsReceivingPayloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsReceivingPayload(decoded));
            }
            if let Ok(decoded) = <IsSendingPayloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsSendingPayload(decoded));
            }
            if let Ok(decoded) = <LzEndpointLookupCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LzEndpointLookup(decoded));
            }
            if let Ok(decoded) = <MockBlockConfirmationsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MockBlockConfirmations(decoded));
            }
            if let Ok(decoded) = <MockChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MockChainId(decoded));
            }
            if let Ok(decoded) = <MockLayerZeroVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MockLayerZeroVersion(decoded));
            }
            if let Ok(decoded) = <MockLibraryVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MockLibraryVersion(decoded));
            }
            if let Ok(decoded) = <MockOracleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MockOracle(decoded));
            }
            if let Ok(decoded) = <MockRelayerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MockRelayer(decoded));
            }
            if let Ok(decoded) = <MockStaticNativeFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MockStaticNativeFee(decoded));
            }
            if let Ok(decoded) = <MsgsToDeliverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MsgsToDeliver(decoded));
            }
            if let Ok(decoded) = <NativeFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NativeFee(decoded));
            }
            if let Ok(decoded) = <OutboundNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutboundNonce(decoded));
            }
            if let Ok(decoded) = <PackedBytesToAddrCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PackedBytesToAddr(decoded));
            }
            if let Ok(decoded) = <ReceivePayloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReceivePayload(decoded));
            }
            if let Ok(decoded) = <RetryPayloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RetryPayload(decoded));
            }
            if let Ok(decoded) = <SendCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Send(decoded));
            }
            if let Ok(decoded) = <SetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded) = <SetDestLzEndpointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDestLzEndpoint(decoded));
            }
            if let Ok(decoded) = <SetEstimatedFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEstimatedFees(decoded));
            }
            if let Ok(decoded) = <SetReceiveVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetReceiveVersion(decoded));
            }
            if let Ok(decoded) = <SetSendVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSendVersion(decoded));
            }
            if let Ok(decoded) = <StoredPayloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StoredPayload(decoded));
            }
            if let Ok(decoded) = <ZroFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZroFee(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockLZEndpointCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddrToPackedBytes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BlockNextMsg(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EstimateFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ForceResumeReceive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInboundNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLengthOfQueue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOutboundNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReceiveLibraryAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReceiveVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSendLibraryAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSendVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasStoredPayload(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InboundNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsReceivingPayload(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsSendingPayload(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LzEndpointLookup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MockBlockConfirmations(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MockChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MockLayerZeroVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MockLibraryVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MockOracle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MockRelayer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MockStaticNativeFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MsgsToDeliver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NativeFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutboundNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PackedBytesToAddr(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReceivePayload(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RetryPayload(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Send(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetDestLzEndpoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetEstimatedFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetReceiveVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSendVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StoredPayload(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZroFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockLZEndpointCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddrToPackedBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlockNextMsg(element) => ::core::fmt::Display::fmt(element, f),
                Self::EstimateFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceResumeReceive(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInboundNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLengthOfQueue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOutboundNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReceiveLibraryAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReceiveVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSendLibraryAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSendVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasStoredPayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::InboundNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsReceivingPayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSendingPayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::LzEndpointLookup(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockBlockConfirmations(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockLayerZeroVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockLibraryVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockRelayer(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockStaticNativeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::MsgsToDeliver(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutboundNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::PackedBytesToAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReceivePayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::RetryPayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::Send(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDestLzEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEstimatedFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReceiveVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSendVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::StoredPayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZroFee(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddrToPackedBytesCall> for MockLZEndpointCalls {
        fn from(value: AddrToPackedBytesCall) -> Self {
            Self::AddrToPackedBytes(value)
        }
    }
    impl ::core::convert::From<BlockNextMsgCall> for MockLZEndpointCalls {
        fn from(value: BlockNextMsgCall) -> Self {
            Self::BlockNextMsg(value)
        }
    }
    impl ::core::convert::From<EstimateFeesCall> for MockLZEndpointCalls {
        fn from(value: EstimateFeesCall) -> Self {
            Self::EstimateFees(value)
        }
    }
    impl ::core::convert::From<ForceResumeReceiveCall> for MockLZEndpointCalls {
        fn from(value: ForceResumeReceiveCall) -> Self {
            Self::ForceResumeReceive(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for MockLZEndpointCalls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<GetConfigCall> for MockLZEndpointCalls {
        fn from(value: GetConfigCall) -> Self {
            Self::GetConfig(value)
        }
    }
    impl ::core::convert::From<GetInboundNonceCall> for MockLZEndpointCalls {
        fn from(value: GetInboundNonceCall) -> Self {
            Self::GetInboundNonce(value)
        }
    }
    impl ::core::convert::From<GetLengthOfQueueCall> for MockLZEndpointCalls {
        fn from(value: GetLengthOfQueueCall) -> Self {
            Self::GetLengthOfQueue(value)
        }
    }
    impl ::core::convert::From<GetOutboundNonceCall> for MockLZEndpointCalls {
        fn from(value: GetOutboundNonceCall) -> Self {
            Self::GetOutboundNonce(value)
        }
    }
    impl ::core::convert::From<GetReceiveLibraryAddressCall> for MockLZEndpointCalls {
        fn from(value: GetReceiveLibraryAddressCall) -> Self {
            Self::GetReceiveLibraryAddress(value)
        }
    }
    impl ::core::convert::From<GetReceiveVersionCall> for MockLZEndpointCalls {
        fn from(value: GetReceiveVersionCall) -> Self {
            Self::GetReceiveVersion(value)
        }
    }
    impl ::core::convert::From<GetSendLibraryAddressCall> for MockLZEndpointCalls {
        fn from(value: GetSendLibraryAddressCall) -> Self {
            Self::GetSendLibraryAddress(value)
        }
    }
    impl ::core::convert::From<GetSendVersionCall> for MockLZEndpointCalls {
        fn from(value: GetSendVersionCall) -> Self {
            Self::GetSendVersion(value)
        }
    }
    impl ::core::convert::From<HasStoredPayloadCall> for MockLZEndpointCalls {
        fn from(value: HasStoredPayloadCall) -> Self {
            Self::HasStoredPayload(value)
        }
    }
    impl ::core::convert::From<InboundNonceCall> for MockLZEndpointCalls {
        fn from(value: InboundNonceCall) -> Self {
            Self::InboundNonce(value)
        }
    }
    impl ::core::convert::From<IsReceivingPayloadCall> for MockLZEndpointCalls {
        fn from(value: IsReceivingPayloadCall) -> Self {
            Self::IsReceivingPayload(value)
        }
    }
    impl ::core::convert::From<IsSendingPayloadCall> for MockLZEndpointCalls {
        fn from(value: IsSendingPayloadCall) -> Self {
            Self::IsSendingPayload(value)
        }
    }
    impl ::core::convert::From<LzEndpointLookupCall> for MockLZEndpointCalls {
        fn from(value: LzEndpointLookupCall) -> Self {
            Self::LzEndpointLookup(value)
        }
    }
    impl ::core::convert::From<MockBlockConfirmationsCall> for MockLZEndpointCalls {
        fn from(value: MockBlockConfirmationsCall) -> Self {
            Self::MockBlockConfirmations(value)
        }
    }
    impl ::core::convert::From<MockChainIdCall> for MockLZEndpointCalls {
        fn from(value: MockChainIdCall) -> Self {
            Self::MockChainId(value)
        }
    }
    impl ::core::convert::From<MockLayerZeroVersionCall> for MockLZEndpointCalls {
        fn from(value: MockLayerZeroVersionCall) -> Self {
            Self::MockLayerZeroVersion(value)
        }
    }
    impl ::core::convert::From<MockLibraryVersionCall> for MockLZEndpointCalls {
        fn from(value: MockLibraryVersionCall) -> Self {
            Self::MockLibraryVersion(value)
        }
    }
    impl ::core::convert::From<MockOracleCall> for MockLZEndpointCalls {
        fn from(value: MockOracleCall) -> Self {
            Self::MockOracle(value)
        }
    }
    impl ::core::convert::From<MockRelayerCall> for MockLZEndpointCalls {
        fn from(value: MockRelayerCall) -> Self {
            Self::MockRelayer(value)
        }
    }
    impl ::core::convert::From<MockStaticNativeFeeCall> for MockLZEndpointCalls {
        fn from(value: MockStaticNativeFeeCall) -> Self {
            Self::MockStaticNativeFee(value)
        }
    }
    impl ::core::convert::From<MsgsToDeliverCall> for MockLZEndpointCalls {
        fn from(value: MsgsToDeliverCall) -> Self {
            Self::MsgsToDeliver(value)
        }
    }
    impl ::core::convert::From<NativeFeeCall> for MockLZEndpointCalls {
        fn from(value: NativeFeeCall) -> Self {
            Self::NativeFee(value)
        }
    }
    impl ::core::convert::From<OutboundNonceCall> for MockLZEndpointCalls {
        fn from(value: OutboundNonceCall) -> Self {
            Self::OutboundNonce(value)
        }
    }
    impl ::core::convert::From<PackedBytesToAddrCall> for MockLZEndpointCalls {
        fn from(value: PackedBytesToAddrCall) -> Self {
            Self::PackedBytesToAddr(value)
        }
    }
    impl ::core::convert::From<ReceivePayloadCall> for MockLZEndpointCalls {
        fn from(value: ReceivePayloadCall) -> Self {
            Self::ReceivePayload(value)
        }
    }
    impl ::core::convert::From<RetryPayloadCall> for MockLZEndpointCalls {
        fn from(value: RetryPayloadCall) -> Self {
            Self::RetryPayload(value)
        }
    }
    impl ::core::convert::From<SendCall> for MockLZEndpointCalls {
        fn from(value: SendCall) -> Self {
            Self::Send(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for MockLZEndpointCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetDestLzEndpointCall> for MockLZEndpointCalls {
        fn from(value: SetDestLzEndpointCall) -> Self {
            Self::SetDestLzEndpoint(value)
        }
    }
    impl ::core::convert::From<SetEstimatedFeesCall> for MockLZEndpointCalls {
        fn from(value: SetEstimatedFeesCall) -> Self {
            Self::SetEstimatedFees(value)
        }
    }
    impl ::core::convert::From<SetReceiveVersionCall> for MockLZEndpointCalls {
        fn from(value: SetReceiveVersionCall) -> Self {
            Self::SetReceiveVersion(value)
        }
    }
    impl ::core::convert::From<SetSendVersionCall> for MockLZEndpointCalls {
        fn from(value: SetSendVersionCall) -> Self {
            Self::SetSendVersion(value)
        }
    }
    impl ::core::convert::From<StoredPayloadCall> for MockLZEndpointCalls {
        fn from(value: StoredPayloadCall) -> Self {
            Self::StoredPayload(value)
        }
    }
    impl ::core::convert::From<ZroFeeCall> for MockLZEndpointCalls {
        fn from(value: ZroFeeCall) -> Self {
            Self::ZroFee(value)
        }
    }
    ///Container type for all return fields from the `addrToPackedBytes` function with signature `addrToPackedBytes(address)` and selector `0x5b301135`
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
    pub struct AddrToPackedBytesReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `estimateFees` function with signature `estimateFees(uint16,address,bytes,bool,bytes)` and selector `0x40a7bb10`
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
    pub struct EstimateFeesReturn {
        pub native_fee: ::ethers::core::types::U256,
        pub zro_fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    pub struct GetChainIdReturn(pub u16);
    ///Container type for all return fields from the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `0xf5ecbdbc`
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
    pub struct GetConfigReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getInboundNonce` function with signature `getInboundNonce(uint16,bytes)` and selector `0xfdc07c70`
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
    pub struct GetInboundNonceReturn(pub u64);
    ///Container type for all return fields from the `getLengthOfQueue` function with signature `getLengthOfQueue(uint16,bytes)` and selector `0x7f6df8e6`
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
    pub struct GetLengthOfQueueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getOutboundNonce` function with signature `getOutboundNonce(uint16,address)` and selector `0x7a145748`
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
    pub struct GetOutboundNonceReturn(pub u64);
    ///Container type for all return fields from the `getReceiveLibraryAddress` function with signature `getReceiveLibraryAddress(address)` and selector `0x71ba2fd6`
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
    pub struct GetReceiveLibraryAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getReceiveVersion` function with signature `getReceiveVersion(address)` and selector `0xda1a7c9a`
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
    pub struct GetReceiveVersionReturn(pub u16);
    ///Container type for all return fields from the `getSendLibraryAddress` function with signature `getSendLibraryAddress(address)` and selector `0x9c729da1`
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
    pub struct GetSendLibraryAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSendVersion` function with signature `getSendVersion(address)` and selector `0x096568f6`
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
    pub struct GetSendVersionReturn(pub u16);
    ///Container type for all return fields from the `hasStoredPayload` function with signature `hasStoredPayload(uint16,bytes)` and selector `0x0eaf6ea6`
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
    pub struct HasStoredPayloadReturn(pub bool);
    ///Container type for all return fields from the `inboundNonce` function with signature `inboundNonce(uint16,bytes)` and selector `0x9924d33b`
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
    pub struct InboundNonceReturn(pub u64);
    ///Container type for all return fields from the `isReceivingPayload` function with signature `isReceivingPayload()` and selector `0xca066b35`
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
    pub struct IsReceivingPayloadReturn(pub bool);
    ///Container type for all return fields from the `isSendingPayload` function with signature `isSendingPayload()` and selector `0xe97a448a`
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
    pub struct IsSendingPayloadReturn(pub bool);
    ///Container type for all return fields from the `lzEndpointLookup` function with signature `lzEndpointLookup(address)` and selector `0xc81b383a`
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
    pub struct LzEndpointLookupReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `mockBlockConfirmations` function with signature `mockBlockConfirmations()` and selector `0x5952c4ec`
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
    pub struct MockBlockConfirmationsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mockChainId` function with signature `mockChainId()` and selector `0xdb14f305`
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
    pub struct MockChainIdReturn(pub u16);
    ///Container type for all return fields from the `mockLayerZeroVersion` function with signature `mockLayerZeroVersion()` and selector `0xb1210770`
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
    pub struct MockLayerZeroVersionReturn(pub u16);
    ///Container type for all return fields from the `mockLibraryVersion` function with signature `mockLibraryVersion()` and selector `0x05bc710f`
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
    pub struct MockLibraryVersionReturn(pub u16);
    ///Container type for all return fields from the `mockOracle` function with signature `mockOracle()` and selector `0xdab31277`
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
    pub struct MockOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `mockRelayer` function with signature `mockRelayer()` and selector `0x4f055b04`
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
    pub struct MockRelayerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `mockStaticNativeFee` function with signature `mockStaticNativeFee()` and selector `0xe961a952`
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
    pub struct MockStaticNativeFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `msgsToDeliver` function with signature `msgsToDeliver(uint16,bytes,uint256)` and selector `0x12a9ee6b`
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
    pub struct MsgsToDeliverReturn {
        pub dst_address: ::ethers::core::types::Address,
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `nativeFee` function with signature `nativeFee()` and selector `0x996f79c0`
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
    pub struct NativeFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `outboundNonce` function with signature `outboundNonce(uint16,address)` and selector `0xb2086499`
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
    pub struct OutboundNonceReturn(pub u64);
    ///Container type for all return fields from the `packedBytesToAddr` function with signature `packedBytesToAddr(bytes)` and selector `0x09eddf54`
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
    pub struct PackedBytesToAddrReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `storedPayload` function with signature `storedPayload(uint16,bytes)` and selector `0x76a386dc`
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
    pub struct StoredPayloadReturn {
        pub payload_length: u64,
        pub dst_address: ::ethers::core::types::Address,
        pub payload_hash: [u8; 32],
    }
    ///Container type for all return fields from the `zroFee` function with signature `zroFee()` and selector `0xaf406aa5`
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
    pub struct ZroFeeReturn(pub ::ethers::core::types::U256);
}
