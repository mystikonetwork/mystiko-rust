pub use mystiko_v2_loop_main::*;
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
pub mod mystiko_v2_loop_main {
    const _: () = {
        ::core::include_bytes!("../json/MystikoV2LoopMain.json",);
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
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct IMystikoLoop.DepositRequest",
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
                    ::std::borrow::ToOwned::to_owned("HashKGreaterThanFieldSize"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("HashKGreaterThanFieldSize",),
                        inputs: ::std::vec![],
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
    pub static MYSTIKOV2LOOPMAIN_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16t\x01@\xC5y#\x92K\\\\TU\xC4\x8D\x931q9\xAD\xDA\xC8\xFB\x17\x90U4\x80\x15a\x007W`\0\x80\xFD[P`@Qa\x0F\x9C8\x03\x80a\x0F\x9C\x839\x81\x01`@\x81\x90Ra\0V\x91a\0\x89V[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x163\x17\x90\x91U`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90Ua\0\xB9V[`\0` \x82\x84\x03\x12\x15a\0\x9BW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xB2W`\0\x80\xFD[\x93\x92PPPV[a\x0E\xD4\x80a\0\xC8`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xF3W`\x005`\xE0\x1C\x80c\xDDu|4\x11a\0\x8AW\x80c\xEA\x0C\xDE\x85\x11a\0YW\x80c\xEA\x0C\xDE\x85\x14a\x02\x8FW\x80c\xECW\x1Cj\x14a\x02\xAFW\x80c\xEDn\xA3:\x14a\x02\xCFW\x80c\xF6\xAF\xE8\x8F\x14a\x02\xEEW`\0\x80\xFD[\x80c\xDDu|4\x14a\x02\x08W\x80c\xDD\xAC]\xC1\x14a\x02\x1DW\x80c\xE1\x9A\xBE\xF8\x14a\x02OW\x80c\xE8\x18<D\x14a\x02oW`\0\x80\xFD[\x80c0\xF4\x9C\xAC\x11a\0\xC6W\x80c0\xF4\x9C\xAC\x14a\x01\x86W\x80c?\xE34z\x14a\x01\xA6W\x80c\xB1\xC3\x94\"\x14a\x01\xC2W\x80c\xCF\xC7\xE2\xDA\x14a\x01\xF3W`\0\x80\xFD[\x80c\x01\xDB\xF1\x9F\x14a\0\xF8W\x80c\x069L\x9B\x14a\x01\x0FW\x80c\x0B\xA9Y\t\x14a\x01/W\x80c$!\xE1U\x14a\x01SW[`\0\x80\xFD[4\x80\x15a\x01\x04W`\0\x80\xFD[Pa\x01\ra\x03\x01V[\0[4\x80\x15a\x01\x1BW`\0\x80\xFD[Pa\x01\ra\x01*6`\x04a\x0B\xA4V[a\x03\x85V[4\x80\x15a\x01;W`\0\x80\xFD[P`\x04T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01_W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x82R`\x04\x81Rc\x06\xC6\xF6\xF7`\xE4\x1B` \x82\x01R\x90Qa\x01J\x91\x90a\r\xA3V[4\x80\x15a\x01\x92W`\0\x80\xFD[Pa\x01\ra\x01\xA16`\x04a\x0B\xA4V[a\x046V[4\x80\x15a\x01\xB2W`\0\x80\xFD[P`\x01`@Qa\x01J\x91\x90a\r{V[4\x80\x15a\x01\xCEW`\0\x80\xFD[P`\0Ta\x01\xE3\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01JV[4\x80\x15a\x01\xFFW`\0\x80\xFD[P`\x03Ta\x01@V[4\x80\x15a\x02\x14W`\0\x80\xFD[Pa\x01\ra\x04\xC3V[4\x80\x15a\x02)W`\0\x80\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01JV[4\x80\x15a\x02[W`\0\x80\xFD[Pa\x01\ra\x02j6`\x04a\x0B\xA4V[a\x05;V[4\x80\x15a\x02{W`\0\x80\xFD[Pa\x01\ra\x02\x8A6`\x04a\x0C\xDBV[a\x05\x95V[4\x80\x15a\x02\x9BW`\0\x80\xFD[Pa\x01\ra\x02\xAA6`\x04a\x0B\xC8V[a\x06(V[4\x80\x15a\x02\xBBW`\0\x80\xFD[P`\0Ta\x027\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\xDBW`\0\x80\xFD[P`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x01\xE3V[a\x01\ra\x02\xFC6`\x04a\x0C\x02V[a\x06\xA0V[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03,W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x90\x81\x17\x91\x82\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x92a\x03{\x92\x90\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1V[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xB0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x15a\x03\xDFW`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FG!\x12\x9E\x0Egn\xD6\xA9)\t\xBB$\xE8S\xCC\xDDc\xADr(\x0C\xC2\xE9t\xE3\x8EH\x0E\x0EnT\x90`\0\x90\xA2PV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04aW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xBE\x18\x08\x91~\t\x81O\xCA}\x80(\x9A\x97`\xA4\x06_\x10\xD6$\xAC\x1A\xEE\x0B\x9F\x90<O]\x92\xB1\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xEEW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90\x81\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x91a\x03{\x91`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05fW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\xC0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x05\xE1W`@Qc`\x03\xE8!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x82\x90U`\x03\x81\x90U`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x7Fv3\0Lz\"\x98i\xAE\xEA\x10\xDBO\xF3\xE5~;\x154\xAE\xB2\xC9\xE7,]\xB2_\x96X\x95\xC30\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06SW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T\x82\x15\x15`\x01`\xA0\x1B\x02`\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x90U`@Q\x7F\xCD\x16,o\xC2B\x85\xBF\xBE9\x9E\xC0\xCC,\xE2\xC3\x80\xAD'\xD2\xEB\x1F\xCAA\x8Ce-\xF1%~~\t\x90a\x04\xB8\x90\x83\x15\x15\x81R` \x01\x90V[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x06\xCBW`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T\x81Q\x10\x15a\x06\xEFW`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04T\x81Q\x11\x15a\x07\x13W`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07,\x82`@\x01Q\x83`\0\x01Q\x84``\x01Qa\x07\x99V[\x90P\x80\x82` \x01Q\x14a\x07RW`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07[3a\x08\xBFV[\x15a\x07yW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x95\x82`\0\x01Q\x83` \x01Q\x84`\xA0\x01Q\x85`\x80\x01Qa\t\\V[PPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x07\xDCW`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x08\x0EW`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x08f\x91`\x04\x01a\rJV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08~W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xB6\x91\x90a\x0C\xC2V[\x95\x94PPPPPV[`\0\x80T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x08\xD9WP`\0\x91\x90PV[`\0T`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x1EW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\t2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tV\x91\x90a\x0B\xE5V[\x92\x91PPV[`@\x80Q`\xA0\x81\x01\x82R\x85\x81R` \x81\x01\x85\x90R`\0\x81\x83\x01\x81\x90R``\x82\x01\x85\x90R`\x80\x82\x01\x84\x90R`\x02T\x92Qcx\xD6\x0C\xD7`\xE0\x1B\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cx\xD6\x0C\xD7\x91a\t\xB6\x91\x85\x91\x90`\x04\x01a\r\xB6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xD0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xE4W=`\0\x80>=`\0\xFD[PP`\x02Ta\n\n\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90Pa\n\x03\x85\x88a\x0E<V[`\0a\n\x11V[PPPPPV[a\n\x1B\x81\x83a\x0E<V[4\x14a\nnW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Finsufficient token\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\xBBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\xC0V[``\x91P[PP\x90P\x80a\x0B\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Famount transfer failed\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\neV[PPPPV[`\0\x82`\x1F\x83\x01\x12a\x0B(W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BCWa\x0BCa\x0EbV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0BkWa\x0Bka\x0EbV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0B\x84W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0B\xB6W`\0\x80\xFD[\x815a\x0B\xC1\x81a\x0ExV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0B\xDAW`\0\x80\xFD[\x815a\x0B\xC1\x81a\x0E\x90V[`\0` \x82\x84\x03\x12\x15a\x0B\xF7W`\0\x80\xFD[\x81Qa\x0B\xC1\x81a\x0E\x90V[`\0` \x82\x84\x03\x12\x15a\x0C\x14W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C,W`\0\x80\xFD[\x90\x83\x01\x90`\xC0\x82\x86\x03\x12\x15a\x0C@W`\0\x80\xFD[a\x0CHa\x0E\x13V[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\x83W`\0\x80\xFD[``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x0C\x9AW`\0\x80\xFD[a\x0C\xA6\x87\x82\x86\x01a\x0B\x17V[`\x80\x83\x01RP`\xA0\x83\x015`\xA0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0C\xD4W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xEEW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\r#W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\r\x07V[\x81\x81\x11\x15a\r5W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a\rrW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\rSV[PPP\x92\x91PPV[` \x81\x01`\x02\x83\x10a\r\x9DWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x81R`\0a\x0B\xC1` \x83\x01\x84a\x0C\xFDV[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra\r\xFA`\xE0\x84\x01\x82a\x0C\xFDV[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E6Wa\x0E6a\x0EbV[`@R\x90V[`\0\x82\x19\x82\x11\x15a\x0E]WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\x8DW`\0\x80\xFD[PV[\x80\x15\x15\x81\x14a\x0E\x8DW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xA4a\xF7C!\xA4\x8F\xFC\xA2W\xA9\x1DSk>\xE1\x9C\x92\xE7^$\xF2\xD4<V.\x90K\xB5\xF9\xC4QdsolcC\0\x08\x07\x003";
    /// The bytecode of the contract.
    pub static MYSTIKOV2LOOPMAIN_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xF3W`\x005`\xE0\x1C\x80c\xDDu|4\x11a\0\x8AW\x80c\xEA\x0C\xDE\x85\x11a\0YW\x80c\xEA\x0C\xDE\x85\x14a\x02\x8FW\x80c\xECW\x1Cj\x14a\x02\xAFW\x80c\xEDn\xA3:\x14a\x02\xCFW\x80c\xF6\xAF\xE8\x8F\x14a\x02\xEEW`\0\x80\xFD[\x80c\xDDu|4\x14a\x02\x08W\x80c\xDD\xAC]\xC1\x14a\x02\x1DW\x80c\xE1\x9A\xBE\xF8\x14a\x02OW\x80c\xE8\x18<D\x14a\x02oW`\0\x80\xFD[\x80c0\xF4\x9C\xAC\x11a\0\xC6W\x80c0\xF4\x9C\xAC\x14a\x01\x86W\x80c?\xE34z\x14a\x01\xA6W\x80c\xB1\xC3\x94\"\x14a\x01\xC2W\x80c\xCF\xC7\xE2\xDA\x14a\x01\xF3W`\0\x80\xFD[\x80c\x01\xDB\xF1\x9F\x14a\0\xF8W\x80c\x069L\x9B\x14a\x01\x0FW\x80c\x0B\xA9Y\t\x14a\x01/W\x80c$!\xE1U\x14a\x01SW[`\0\x80\xFD[4\x80\x15a\x01\x04W`\0\x80\xFD[Pa\x01\ra\x03\x01V[\0[4\x80\x15a\x01\x1BW`\0\x80\xFD[Pa\x01\ra\x01*6`\x04a\x0B\xA4V[a\x03\x85V[4\x80\x15a\x01;W`\0\x80\xFD[P`\x04T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01_W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x82R`\x04\x81Rc\x06\xC6\xF6\xF7`\xE4\x1B` \x82\x01R\x90Qa\x01J\x91\x90a\r\xA3V[4\x80\x15a\x01\x92W`\0\x80\xFD[Pa\x01\ra\x01\xA16`\x04a\x0B\xA4V[a\x046V[4\x80\x15a\x01\xB2W`\0\x80\xFD[P`\x01`@Qa\x01J\x91\x90a\r{V[4\x80\x15a\x01\xCEW`\0\x80\xFD[P`\0Ta\x01\xE3\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01JV[4\x80\x15a\x01\xFFW`\0\x80\xFD[P`\x03Ta\x01@V[4\x80\x15a\x02\x14W`\0\x80\xFD[Pa\x01\ra\x04\xC3V[4\x80\x15a\x02)W`\0\x80\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01JV[4\x80\x15a\x02[W`\0\x80\xFD[Pa\x01\ra\x02j6`\x04a\x0B\xA4V[a\x05;V[4\x80\x15a\x02{W`\0\x80\xFD[Pa\x01\ra\x02\x8A6`\x04a\x0C\xDBV[a\x05\x95V[4\x80\x15a\x02\x9BW`\0\x80\xFD[Pa\x01\ra\x02\xAA6`\x04a\x0B\xC8V[a\x06(V[4\x80\x15a\x02\xBBW`\0\x80\xFD[P`\0Ta\x027\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\xDBW`\0\x80\xFD[P`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x01\xE3V[a\x01\ra\x02\xFC6`\x04a\x0C\x02V[a\x06\xA0V[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03,W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x90\x81\x17\x91\x82\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x92a\x03{\x92\x90\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1V[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xB0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x15a\x03\xDFW`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7FG!\x12\x9E\x0Egn\xD6\xA9)\t\xBB$\xE8S\xCC\xDDc\xADr(\x0C\xC2\xE9t\xE3\x8EH\x0E\x0EnT\x90`\0\x90\xA2PV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04aW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xBE\x18\x08\x91~\t\x81O\xCA}\x80(\x9A\x97`\xA4\x06_\x10\xD6$\xAC\x1A\xEE\x0B\x9F\x90<O]\x92\xB1\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xEEW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA0\x1B\x19\x16\x90\x81\x90U`@Q\x7F\xA5\xFF\xE1`\x1E\xB9:\x7F\xEF\xD0\xD0\xEE\xB5\xFC\x94\xA2\xF6Tu\xF33\x83o\xE2\xB8\x7F\xD7\xBB\xF5\x95 i\x91a\x03{\x91`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15\x81R` \x01\x90V[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05fW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\xC0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x05\xE1W`@Qc`\x03\xE8!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x82\x90U`\x03\x81\x90U`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x7Fv3\0Lz\"\x98i\xAE\xEA\x10\xDBO\xF3\xE5~;\x154\xAE\xB2\xC9\xE7,]\xB2_\x96X\x95\xC30\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06SW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T\x82\x15\x15`\x01`\xA0\x1B\x02`\xFF`\xA0\x1B\x19\x90\x91\x16\x17\x90U`@Q\x7F\xCD\x16,o\xC2B\x85\xBF\xBE9\x9E\xC0\xCC,\xE2\xC3\x80\xAD'\xD2\xEB\x1F\xCAA\x8Ce-\xF1%~~\t\x90a\x04\xB8\x90\x83\x15\x15\x81R` \x01\x90V[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x06\xCBW`@Qc\x0E/B\xC9`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T\x81Q\x10\x15a\x06\xEFW`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04T\x81Q\x11\x15a\x07\x13W`@Qc\x06%\x04\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07,\x82`@\x01Q\x83`\0\x01Q\x84``\x01Qa\x07\x99V[\x90P\x80\x82` \x01Q\x14a\x07RW`@Qc\x01\xBF\xAA%`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07[3a\x08\xBFV[\x15a\x07yW`@Qc.p\xC0\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x95\x82`\0\x01Q\x83` \x01Q\x84`\xA0\x01Q\x85`\x80\x01Qa\t\\V[PPV[`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x85\x10a\x07\xDCW`@Qc\x80_*I`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x08\x0EW`@Qc;\xBD\xE0\xBF`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@\x80Q``\x81\x01\x82R\x87\x81R` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81\x83\x01R\x90Qc\x04\xB9\x8E\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c%\xCCp\xE8\x91a\x08f\x91`\x04\x01a\rJV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08~W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xB6\x91\x90a\x0C\xC2V[\x95\x94PPPPPV[`\0\x80T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x08\xD9WP`\0\x91\x90PV[`\0T`@Qc\xDFY/}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x1EW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\t2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tV\x91\x90a\x0B\xE5V[\x92\x91PPV[`@\x80Q`\xA0\x81\x01\x82R\x85\x81R` \x81\x01\x85\x90R`\0\x81\x83\x01\x81\x90R``\x82\x01\x85\x90R`\x80\x82\x01\x84\x90R`\x02T\x92Qcx\xD6\x0C\xD7`\xE0\x1B\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cx\xD6\x0C\xD7\x91a\t\xB6\x91\x85\x91\x90`\x04\x01a\r\xB6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xD0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xE4W=`\0\x80>=`\0\xFD[PP`\x02Ta\n\n\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90Pa\n\x03\x85\x88a\x0E<V[`\0a\n\x11V[PPPPPV[a\n\x1B\x81\x83a\x0E<V[4\x14a\nnW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Finsufficient token\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\xBBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\xC0V[``\x91P[PP\x90P\x80a\x0B\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Famount transfer failed\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\neV[PPPPV[`\0\x82`\x1F\x83\x01\x12a\x0B(W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BCWa\x0BCa\x0EbV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0BkWa\x0Bka\x0EbV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0B\x84W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0B\xB6W`\0\x80\xFD[\x815a\x0B\xC1\x81a\x0ExV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0B\xDAW`\0\x80\xFD[\x815a\x0B\xC1\x81a\x0E\x90V[`\0` \x82\x84\x03\x12\x15a\x0B\xF7W`\0\x80\xFD[\x81Qa\x0B\xC1\x81a\x0E\x90V[`\0` \x82\x84\x03\x12\x15a\x0C\x14W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C,W`\0\x80\xFD[\x90\x83\x01\x90`\xC0\x82\x86\x03\x12\x15a\x0C@W`\0\x80\xFD[a\x0CHa\x0E\x13V[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\x83W`\0\x80\xFD[``\x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x0C\x9AW`\0\x80\xFD[a\x0C\xA6\x87\x82\x86\x01a\x0B\x17V[`\x80\x83\x01RP`\xA0\x83\x015`\xA0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0C\xD4W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xEEW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\r#W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\r\x07V[\x81\x81\x11\x15a\r5W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a\rrW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\rSV[PPP\x92\x91PPV[` \x81\x01`\x02\x83\x10a\r\x9DWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x81R`\0a\x0B\xC1` \x83\x01\x84a\x0C\xFDV[`@\x81R\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83\x01Q`\x80\x82\x01R``\x83\x01Q`\xA0\x82\x01R`\0`\x80\x84\x01Q`\xA0`\xC0\x84\x01Ra\r\xFA`\xE0\x84\x01\x82a\x0C\xFDV[\x91PP`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E6Wa\x0E6a\x0EbV[`@R\x90V[`\0\x82\x19\x82\x11\x15a\x0E]WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\x8DW`\0\x80\xFD[PV[\x80\x15\x15\x81\x14a\x0E\x8DW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xA4a\xF7C!\xA4\x8F\xFC\xA2W\xA9\x1DSk>\xE1\x9C\x92\xE7^$\xF2\xD4<V.\x90K\xB5\xF9\xC4QdsolcC\0\x08\x07\x003";
    /// The deployed bytecode of the contract.
    pub static MYSTIKOV2LOOPMAIN_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MystikoV2LoopMain<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoV2LoopMain<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoV2LoopMain<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoV2LoopMain<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoV2LoopMain<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MystikoV2LoopMain))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> MystikoV2LoopMain<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                MYSTIKOV2LOOPMAIN_ABI.clone(),
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
                MYSTIKOV2LOOPMAIN_ABI.clone(),
                MYSTIKOV2LOOPMAIN_BYTECODE.clone().into(),
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
        ///Calls the contract's `deposit` (0xf6afe88f) function
        pub fn deposit(&self, request: DepositRequest) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 175, 232, 143], (request,))
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
        ///Calls the contract's `isDepositsDisabled` (0xed6ea33a) function
        pub fn is_deposits_disabled(&self) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([237, 110, 163, 58], ())
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
        ///Calls the contract's `setDepositsDisabled` (0xea0cde85) function
        pub fn set_deposits_disabled(&self, state: bool) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 12, 222, 133], state)
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
        ///Gets the contract's `OperatorChanged` event
        pub fn operator_changed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, OperatorChangedFilter> {
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
        pub fn events(&self) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, MystikoV2LoopMainEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for MystikoV2LoopMain<M> {
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
    pub enum MystikoV2LoopMainErrors {
        AmountTooLarge(AmountTooLarge),
        AmountTooSmall(AmountTooSmall),
        CommitmentHashIncorrect(CommitmentHashIncorrect),
        DepositsDisabled(DepositsDisabled),
        HashKGreaterThanFieldSize(HashKGreaterThanFieldSize),
        MinAmountGreaterThanMaxAmount(MinAmountGreaterThanMaxAmount),
        NotChanged(NotChanged),
        OnlyOperator(OnlyOperator),
        RandomSGreaterThanFieldSize(RandomSGreaterThanFieldSize),
        SanctionedAddress(SanctionedAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoV2LoopMainErrors {
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
            if let Ok(decoded) = <CommitmentHashIncorrect as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CommitmentHashIncorrect(decoded));
            }
            if let Ok(decoded) = <DepositsDisabled as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositsDisabled(decoded));
            }
            if let Ok(decoded) = <HashKGreaterThanFieldSize as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashKGreaterThanFieldSize(decoded));
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
            if let Ok(decoded) = <SanctionedAddress as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SanctionedAddress(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for MystikoV2LoopMainErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AmountTooLarge(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AmountTooSmall(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CommitmentHashIncorrect(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DepositsDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::HashKGreaterThanFieldSize(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::MinAmountGreaterThanMaxAmount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::NotChanged(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnlyOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RandomSGreaterThanFieldSize(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionedAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for MystikoV2LoopMainErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AmountTooLarge as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <AmountTooSmall as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <CommitmentHashIncorrect as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <DepositsDisabled as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <HashKGreaterThanFieldSize as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <MinAmountGreaterThanMaxAmount as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <NotChanged as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OnlyOperator as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <RandomSGreaterThanFieldSize as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <SanctionedAddress as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2LoopMainErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AmountTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentHashIncorrect(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashKGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinAmountGreaterThanMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotChanged(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RandomSGreaterThanFieldSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionedAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MystikoV2LoopMainErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AmountTooLarge> for MystikoV2LoopMainErrors {
        fn from(value: AmountTooLarge) -> Self {
            Self::AmountTooLarge(value)
        }
    }
    impl ::core::convert::From<AmountTooSmall> for MystikoV2LoopMainErrors {
        fn from(value: AmountTooSmall) -> Self {
            Self::AmountTooSmall(value)
        }
    }
    impl ::core::convert::From<CommitmentHashIncorrect> for MystikoV2LoopMainErrors {
        fn from(value: CommitmentHashIncorrect) -> Self {
            Self::CommitmentHashIncorrect(value)
        }
    }
    impl ::core::convert::From<DepositsDisabled> for MystikoV2LoopMainErrors {
        fn from(value: DepositsDisabled) -> Self {
            Self::DepositsDisabled(value)
        }
    }
    impl ::core::convert::From<HashKGreaterThanFieldSize> for MystikoV2LoopMainErrors {
        fn from(value: HashKGreaterThanFieldSize) -> Self {
            Self::HashKGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<MinAmountGreaterThanMaxAmount> for MystikoV2LoopMainErrors {
        fn from(value: MinAmountGreaterThanMaxAmount) -> Self {
            Self::MinAmountGreaterThanMaxAmount(value)
        }
    }
    impl ::core::convert::From<NotChanged> for MystikoV2LoopMainErrors {
        fn from(value: NotChanged) -> Self {
            Self::NotChanged(value)
        }
    }
    impl ::core::convert::From<OnlyOperator> for MystikoV2LoopMainErrors {
        fn from(value: OnlyOperator) -> Self {
            Self::OnlyOperator(value)
        }
    }
    impl ::core::convert::From<RandomSGreaterThanFieldSize> for MystikoV2LoopMainErrors {
        fn from(value: RandomSGreaterThanFieldSize) -> Self {
            Self::RandomSGreaterThanFieldSize(value)
        }
    }
    impl ::core::convert::From<SanctionedAddress> for MystikoV2LoopMainErrors {
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
    pub enum MystikoV2LoopMainEvents {
        DepositAmountLimitsFilter(DepositAmountLimitsFilter),
        DepositsDisabledFilter(DepositsDisabledFilter),
        OperatorChangedFilter(OperatorChangedFilter),
        SanctionsCheckFilter(SanctionsCheckFilter),
        SanctionsListFilter(SanctionsListFilter),
    }
    impl ::ethers_contract::EthLogDecode for MystikoV2LoopMainEvents {
        fn decode_log(log: &::ethers_core::abi::RawLog) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = DepositAmountLimitsFilter::decode_log(log) {
                return Ok(MystikoV2LoopMainEvents::DepositAmountLimitsFilter(decoded));
            }
            if let Ok(decoded) = DepositsDisabledFilter::decode_log(log) {
                return Ok(MystikoV2LoopMainEvents::DepositsDisabledFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(MystikoV2LoopMainEvents::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(MystikoV2LoopMainEvents::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(MystikoV2LoopMainEvents::SanctionsListFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MystikoV2LoopMainEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositAmountLimitsFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsDisabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsCheckFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsListFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositAmountLimitsFilter> for MystikoV2LoopMainEvents {
        fn from(value: DepositAmountLimitsFilter) -> Self {
            Self::DepositAmountLimitsFilter(value)
        }
    }
    impl ::core::convert::From<DepositsDisabledFilter> for MystikoV2LoopMainEvents {
        fn from(value: DepositsDisabledFilter) -> Self {
            Self::DepositsDisabledFilter(value)
        }
    }
    impl ::core::convert::From<OperatorChangedFilter> for MystikoV2LoopMainEvents {
        fn from(value: OperatorChangedFilter) -> Self {
            Self::OperatorChangedFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckFilter> for MystikoV2LoopMainEvents {
        fn from(value: SanctionsCheckFilter) -> Self {
            Self::SanctionsCheckFilter(value)
        }
    }
    impl ::core::convert::From<SanctionsListFilter> for MystikoV2LoopMainEvents {
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
    ///Container type for all input parameters for the `deposit` function with signature `deposit((uint256,uint256,uint256,uint128,bytes,uint256))` and selector `0xf6afe88f`
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
    #[ethcall(name = "deposit", abi = "deposit((uint256,uint256,uint256,uint128,bytes,uint256))")]
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
    pub enum MystikoV2LoopMainCalls {
        AssetType(AssetTypeCall),
        BridgeType(BridgeTypeCall),
        ChangeOperator(ChangeOperatorCall),
        Deposit(DepositCall),
        DisableSanctionsCheck(DisableSanctionsCheckCall),
        EnableSanctionsCheck(EnableSanctionsCheckCall),
        GetAssociatedCommitmentPool(GetAssociatedCommitmentPoolCall),
        GetMaxAmount(GetMaxAmountCall),
        GetMinAmount(GetMinAmountCall),
        IsDepositsDisabled(IsDepositsDisabledCall),
        SanctionsCheck(SanctionsCheckCall),
        SanctionsList(SanctionsListCall),
        SetAssociatedCommitmentPool(SetAssociatedCommitmentPoolCall),
        SetDepositsDisabled(SetDepositsDisabledCall),
        UpdateDepositAmountLimits(UpdateDepositAmountLimitsCall),
        UpdateSanctionsListAddress(UpdateSanctionsListAddressCall),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoV2LoopMainCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AssetTypeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetType(decoded));
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
            if let Ok(decoded) = <IsDepositsDisabledCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsDepositsDisabled(decoded));
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
            if let Ok(decoded) = <SetDepositsDisabledCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDepositsDisabled(decoded));
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
    impl ::ethers_core::abi::AbiEncode for MystikoV2LoopMainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetType(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::BridgeType(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ChangeOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DisableSanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::EnableSanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetAssociatedCommitmentPool(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMaxAmount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetMinAmount(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::IsDepositsDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionsCheck(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SanctionsList(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetAssociatedCommitmentPool(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetDepositsDisabled(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UpdateDepositAmountLimits(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UpdateSanctionsListAddress(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MystikoV2LoopMainCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetType(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeType(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableSanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableSanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssociatedCommitmentPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::SanctionsList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAssociatedCommitmentPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositsDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDepositAmountLimits(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateSanctionsListAddress(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssetTypeCall> for MystikoV2LoopMainCalls {
        fn from(value: AssetTypeCall) -> Self {
            Self::AssetType(value)
        }
    }
    impl ::core::convert::From<BridgeTypeCall> for MystikoV2LoopMainCalls {
        fn from(value: BridgeTypeCall) -> Self {
            Self::BridgeType(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorCall> for MystikoV2LoopMainCalls {
        fn from(value: ChangeOperatorCall) -> Self {
            Self::ChangeOperator(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MystikoV2LoopMainCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DisableSanctionsCheckCall> for MystikoV2LoopMainCalls {
        fn from(value: DisableSanctionsCheckCall) -> Self {
            Self::DisableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<EnableSanctionsCheckCall> for MystikoV2LoopMainCalls {
        fn from(value: EnableSanctionsCheckCall) -> Self {
            Self::EnableSanctionsCheck(value)
        }
    }
    impl ::core::convert::From<GetAssociatedCommitmentPoolCall> for MystikoV2LoopMainCalls {
        fn from(value: GetAssociatedCommitmentPoolCall) -> Self {
            Self::GetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<GetMaxAmountCall> for MystikoV2LoopMainCalls {
        fn from(value: GetMaxAmountCall) -> Self {
            Self::GetMaxAmount(value)
        }
    }
    impl ::core::convert::From<GetMinAmountCall> for MystikoV2LoopMainCalls {
        fn from(value: GetMinAmountCall) -> Self {
            Self::GetMinAmount(value)
        }
    }
    impl ::core::convert::From<IsDepositsDisabledCall> for MystikoV2LoopMainCalls {
        fn from(value: IsDepositsDisabledCall) -> Self {
            Self::IsDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<SanctionsCheckCall> for MystikoV2LoopMainCalls {
        fn from(value: SanctionsCheckCall) -> Self {
            Self::SanctionsCheck(value)
        }
    }
    impl ::core::convert::From<SanctionsListCall> for MystikoV2LoopMainCalls {
        fn from(value: SanctionsListCall) -> Self {
            Self::SanctionsList(value)
        }
    }
    impl ::core::convert::From<SetAssociatedCommitmentPoolCall> for MystikoV2LoopMainCalls {
        fn from(value: SetAssociatedCommitmentPoolCall) -> Self {
            Self::SetAssociatedCommitmentPool(value)
        }
    }
    impl ::core::convert::From<SetDepositsDisabledCall> for MystikoV2LoopMainCalls {
        fn from(value: SetDepositsDisabledCall) -> Self {
            Self::SetDepositsDisabled(value)
        }
    }
    impl ::core::convert::From<UpdateDepositAmountLimitsCall> for MystikoV2LoopMainCalls {
        fn from(value: UpdateDepositAmountLimitsCall) -> Self {
            Self::UpdateDepositAmountLimits(value)
        }
    }
    impl ::core::convert::From<UpdateSanctionsListAddressCall> for MystikoV2LoopMainCalls {
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
    ///`DepositRequest(uint256,uint256,uint256,uint128,bytes,uint256)`
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
        pub rollup_fee: ::ethers_core::types::U256,
    }
}
