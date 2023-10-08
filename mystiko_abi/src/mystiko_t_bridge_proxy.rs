pub use mystiko_t_bridge_proxy::*;
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
pub mod mystiko_t_bridge_proxy {
    const _: () = {
        ::core::include_bytes!("../json/MystikoTBridgeProxy.json",);
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers_core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addExecutorWhitelist"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addExecutorWhitelist",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_executor"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addRegisterWhitelist"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addRegisterWhitelist",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_register"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("crossChainSyncTx"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("crossChainSyncTx"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_fromChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_fromContract"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_toContract"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_executor"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_message"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeExecutorWhitelist"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeExecutorWhitelist",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_executor"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeRegisterWhitelist"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeRegisterWhitelist",),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_register"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendMessage"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sendMessage"),
                        inputs: ::std::vec![
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_toContract"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_toChainId"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers_core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_message"),
                                kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![::ethers_core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdraw"),
                        inputs: ::std::vec![::ethers_core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_recipient"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers_core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("TBridgeCrossChainMessage"),
                ::std::vec![::ethers_core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("TBridgeCrossChainMessage",),
                    inputs: ::std::vec![
                        ::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("toContract"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },
                        ::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("toChainId"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("fromContract"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },
                        ::ethers_core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("message"),
                            kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CallCrossChainSyncTxError"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CallCrossChainSyncTxError",),
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
                    ::std::borrow::ToOwned::to_owned("OnlyRegister"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyRegister"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyWhitelistedExecutor"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyWhitelistedExecutor",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawFailed"),
                    ::std::vec![::ethers_core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawFailed"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MYSTIKOTBRIDGEPROXY_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x088\x80a\x002`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0{W`\x005`\xE0\x1C\x80c\xBF\xDF\xD5c\x11a\0NW\x80c\xBF\xDF\xD5c\x14a\x01\tW\x80c\xC8\x179\xCD\x14a\x01)W\x80c\xD1R\tH\x14a\x01<W\x80c\xF9\x19\xD4i\x14a\x01\\W`\0\x80\xFD[\x80c\x069L\x9B\x14a\0\x80W\x80cQ\xCF\xF8\xD9\x14a\0\xA2W\x80c\x91\x9C\x1D\xF5\x14a\0\xB5W\x80c\xA0q\xE9\xB1\x14a\0\xE9W[`\0\x80\xFD[4\x80\x15a\0\x8CW`\0\x80\xFD[Pa\0\xA0a\0\x9B6`\x04a\x05BV[a\x01|V[\0[a\0\xA0a\0\xB06`\x04a\x05BV[a\x01\xE1V[4\x80\x15a\0\xC1W`\0\x80\xFD[Pa\0\xD5a\0\xD06`\x04a\x06XV[a\x02\x84V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xF5W`\0\x80\xFD[Pa\0\xA0a\x01\x046`\x04a\x05BV[a\x03cV[4\x80\x15a\x01\x15W`\0\x80\xFD[Pa\0\xA0a\x01$6`\x04a\x05BV[a\x03\xB5V[a\0\xA0a\x0176`\x04a\x05dV[a\x04\x04V[4\x80\x15a\x01HW`\0\x80\xFD[Pa\0\xA0a\x01W6`\x04a\x05BV[a\x04vV[4\x80\x15a\x01hW`\0\x80\xFD[Pa\0\xA0a\x01w6`\x04a\x05BV[a\x04\xC2V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xA7W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\x0CW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16G`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x02YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02^V[``\x91P[PP\x90P\x80a\x02\x80W`@Qc\x1DB\xC8g`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPV[3`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16a\x02\xB3W`@Qb\x11\x1E\x99`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x10ZC\x9B`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x82\xD2\x1C\xD8\x90a\x02\xE7\x90\x8A\x90\x8A\x90\x88\x90\x88\x90\x8B\x90`\x04\x01a\x07\x8FV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x01W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x039\x91\x90a\x066V[a\x03VW`@Qc^\xB9\x87\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x96\x95PPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x8EW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xE0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[3`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x044W`@Qc-xX\xE1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xD8\x08W\xA1\x83\xB2\t/\x9E\x8A\xC41\xB7g}\xA3\x83\xDA\xB7\0,\x16\x7F\xD8.k1r\xAB\x86\xE8\xD8\x83\x833\x84`@Qa\x04i\x94\x93\x92\x91\x90a\x07\x0CV[`@Q\x80\x91\x03\x90\xA1PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xA1W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 \x80T`\xFF\x19\x16\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xEDW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16\x90UV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05%W`\0\x80\xFD[\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05%W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x05TW`\0\x80\xFD[a\x05]\x82a\x05\x0EV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05yW`\0\x80\xFD[a\x05\x82\x84a\x05\x0EV[\x92Pa\x05\x90` \x85\x01a\x05*V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xADW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x05\xC1W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\xD3Wa\x05\xD3a\x07\xECV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x05\xFBWa\x05\xFBa\x07\xECV[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x06\x14W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x06HW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05]W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x06qW`\0\x80\xFD[a\x06z\x87a\x05*V[\x95Pa\x06\x88` \x88\x01a\x05\x0EV[\x94Pa\x06\x96`@\x88\x01a\x05\x0EV[\x93Pa\x06\xA4``\x88\x01a\x05\x0EV[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xC1W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x06\xD5W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06\xE4W`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x06\xF6W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x83R` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81\x85\x01R\x81\x86\x16`@\x85\x01R`\x80``\x85\x01R\x84Q\x91P\x81`\x80\x85\x01R`\0[\x82\x81\x10\x15a\x07dW\x85\x81\x01\x82\x01Q\x85\x82\x01`\xA0\x01R\x81\x01a\x07HV[\x82\x81\x11\x15a\x07vW`\0`\xA0\x84\x87\x01\x01R[PP`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`\xA0\x01\x95\x94PPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R`\0`\x01`\x01`\xA0\x1B\x03\x80\x87\x16` \x84\x01R`\x80`@\x84\x01R\x84`\x80\x84\x01R\x84\x86`\xA0\x85\x017`\0\x83\x86\x01`\xA0\x90\x81\x01\x91\x90\x91R\x93\x16``\x83\x01RP`\x1F\x90\x92\x01`\x1F\x19\x16\x90\x91\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 o\x11\x9F?Z \xD3\x99x4\xEC\xD0\x9F:\xA1\xB3\xD0\xF7\")\x846\xE6$\xAB&\xC4>7nA2dsolcC\0\x08\x07\x003";
    /// The bytecode of the contract.
    pub static MYSTIKOTBRIDGEPROXY_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0{W`\x005`\xE0\x1C\x80c\xBF\xDF\xD5c\x11a\0NW\x80c\xBF\xDF\xD5c\x14a\x01\tW\x80c\xC8\x179\xCD\x14a\x01)W\x80c\xD1R\tH\x14a\x01<W\x80c\xF9\x19\xD4i\x14a\x01\\W`\0\x80\xFD[\x80c\x069L\x9B\x14a\0\x80W\x80cQ\xCF\xF8\xD9\x14a\0\xA2W\x80c\x91\x9C\x1D\xF5\x14a\0\xB5W\x80c\xA0q\xE9\xB1\x14a\0\xE9W[`\0\x80\xFD[4\x80\x15a\0\x8CW`\0\x80\xFD[Pa\0\xA0a\0\x9B6`\x04a\x05BV[a\x01|V[\0[a\0\xA0a\0\xB06`\x04a\x05BV[a\x01\xE1V[4\x80\x15a\0\xC1W`\0\x80\xFD[Pa\0\xD5a\0\xD06`\x04a\x06XV[a\x02\x84V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xF5W`\0\x80\xFD[Pa\0\xA0a\x01\x046`\x04a\x05BV[a\x03cV[4\x80\x15a\x01\x15W`\0\x80\xFD[Pa\0\xA0a\x01$6`\x04a\x05BV[a\x03\xB5V[a\0\xA0a\x0176`\x04a\x05dV[a\x04\x04V[4\x80\x15a\x01HW`\0\x80\xFD[Pa\0\xA0a\x01W6`\x04a\x05BV[a\x04vV[4\x80\x15a\x01hW`\0\x80\xFD[Pa\0\xA0a\x01w6`\x04a\x05BV[a\x04\xC2V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xA7W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\x0CW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16G`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x02YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02^V[``\x91P[PP\x90P\x80a\x02\x80W`@Qc\x1DB\xC8g`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPV[3`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16a\x02\xB3W`@Qb\x11\x1E\x99`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x10ZC\x9B`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x82\xD2\x1C\xD8\x90a\x02\xE7\x90\x8A\x90\x8A\x90\x88\x90\x88\x90\x8B\x90`\x04\x01a\x07\x8FV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x01W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x039\x91\x90a\x066V[a\x03VW`@Qc^\xB9\x87\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x96\x95PPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x8EW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xE0W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[3`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x044W`@Qc-xX\xE1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xD8\x08W\xA1\x83\xB2\t/\x9E\x8A\xC41\xB7g}\xA3\x83\xDA\xB7\0,\x16\x7F\xD8.k1r\xAB\x86\xE8\xD8\x83\x833\x84`@Qa\x04i\x94\x93\x92\x91\x90a\x07\x0CV[`@Q\x80\x91\x03\x90\xA1PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xA1W`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 \x80T`\xFF\x19\x16\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xEDW`@Qc'\xE1\xF1\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16\x90UV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05%W`\0\x80\xFD[\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05%W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x05TW`\0\x80\xFD[a\x05]\x82a\x05\x0EV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05yW`\0\x80\xFD[a\x05\x82\x84a\x05\x0EV[\x92Pa\x05\x90` \x85\x01a\x05*V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xADW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x05\xC1W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\xD3Wa\x05\xD3a\x07\xECV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x05\xFBWa\x05\xFBa\x07\xECV[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x06\x14W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x06HW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05]W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x06qW`\0\x80\xFD[a\x06z\x87a\x05*V[\x95Pa\x06\x88` \x88\x01a\x05\x0EV[\x94Pa\x06\x96`@\x88\x01a\x05\x0EV[\x93Pa\x06\xA4``\x88\x01a\x05\x0EV[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xC1W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x06\xD5W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06\xE4W`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x06\xF6W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x83R` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81\x85\x01R\x81\x86\x16`@\x85\x01R`\x80``\x85\x01R\x84Q\x91P\x81`\x80\x85\x01R`\0[\x82\x81\x10\x15a\x07dW\x85\x81\x01\x82\x01Q\x85\x82\x01`\xA0\x01R\x81\x01a\x07HV[\x82\x81\x11\x15a\x07vW`\0`\xA0\x84\x87\x01\x01R[PP`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`\xA0\x01\x95\x94PPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R`\0`\x01`\x01`\xA0\x1B\x03\x80\x87\x16` \x84\x01R`\x80`@\x84\x01R\x84`\x80\x84\x01R\x84\x86`\xA0\x85\x017`\0\x83\x86\x01`\xA0\x90\x81\x01\x91\x90\x91R\x93\x16``\x83\x01RP`\x1F\x90\x92\x01`\x1F\x19\x16\x90\x91\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 o\x11\x9F?Z \xD3\x99x4\xEC\xD0\x9F:\xA1\xB3\xD0\xF7\")\x846\xE6$\xAB&\xC4>7nA2dsolcC\0\x08\x07\x003";
    /// The deployed bytecode of the contract.
    pub static MYSTIKOTBRIDGEPROXY_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MystikoTBridgeProxy<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoTBridgeProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoTBridgeProxy<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoTBridgeProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoTBridgeProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MystikoTBridgeProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> MystikoTBridgeProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                MYSTIKOTBRIDGEPROXY_ABI.clone(),
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
                MYSTIKOTBRIDGEPROXY_ABI.clone(),
                MYSTIKOTBRIDGEPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addExecutorWhitelist` (0xa071e9b1) function
        pub fn add_executor_whitelist(
            &self,
            executor: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 113, 233, 177], executor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addRegisterWhitelist` (0xbfdfd563) function
        pub fn add_register_whitelist(
            &self,
            register: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 223, 213, 99], register)
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
        ///Calls the contract's `crossChainSyncTx` (0x919c1df5) function
        pub fn cross_chain_sync_tx(
            &self,
            from_chain_id: u64,
            from_contract: ::ethers_core::types::Address,
            to_contract: ::ethers_core::types::Address,
            executor: ::ethers_core::types::Address,
            message: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [145, 156, 29, 245],
                    (from_chain_id, from_contract, to_contract, executor, message),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeExecutorWhitelist` (0xd1520948) function
        pub fn remove_executor_whitelist(
            &self,
            executor: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 82, 9, 72], executor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeRegisterWhitelist` (0xf919d469) function
        pub fn remove_register_whitelist(
            &self,
            register: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 25, 212, 105], register)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendMessage` (0xc81739cd) function
        pub fn send_message(
            &self,
            to_contract: ::ethers_core::types::Address,
            to_chain_id: u64,
            message: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 23, 57, 205], (to_contract, to_chain_id, message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x51cff8d9) function
        pub fn withdraw(
            &self,
            recipient: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 207, 248, 217], recipient)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `TBridgeCrossChainMessage` event
        pub fn t_bridge_cross_chain_message_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, TbridgeCrossChainMessageFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, TbridgeCrossChainMessageFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for MystikoTBridgeProxy<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallCrossChainSyncTxError` with signature `CallCrossChainSyncTxError()` and selector `0xbd730e06`
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
    #[etherror(name = "CallCrossChainSyncTxError", abi = "CallCrossChainSyncTxError()")]
    pub struct CallCrossChainSyncTxError;
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
    ///Custom Error type `OnlyRegister` with signature `OnlyRegister()` and selector `0x5af0b1c2`
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
    #[etherror(name = "OnlyRegister", abi = "OnlyRegister()")]
    pub struct OnlyRegister;
    ///Custom Error type `OnlyWhitelistedExecutor` with signature `OnlyWhitelistedExecutor()` and selector `0x00447a64`
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
    #[etherror(name = "OnlyWhitelistedExecutor", abi = "OnlyWhitelistedExecutor()")]
    pub struct OnlyWhitelistedExecutor;
    ///Custom Error type `WithdrawFailed` with signature `WithdrawFailed()` and selector `0x750b219c`
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
    #[etherror(name = "WithdrawFailed", abi = "WithdrawFailed()")]
    pub struct WithdrawFailed;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers_contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoTBridgeProxyErrors {
        CallCrossChainSyncTxError(CallCrossChainSyncTxError),
        OnlyOperator(OnlyOperator),
        OnlyRegister(OnlyRegister),
        OnlyWhitelistedExecutor(OnlyWhitelistedExecutor),
        WithdrawFailed(WithdrawFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoTBridgeProxyErrors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <CallCrossChainSyncTxError as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallCrossChainSyncTxError(decoded));
            }
            if let Ok(decoded) = <OnlyOperator as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyOperator(decoded));
            }
            if let Ok(decoded) = <OnlyRegister as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyRegister(decoded));
            }
            if let Ok(decoded) = <OnlyWhitelistedExecutor as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyWhitelistedExecutor(decoded));
            }
            if let Ok(decoded) = <WithdrawFailed as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawFailed(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for MystikoTBridgeProxyErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallCrossChainSyncTxError(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnlyOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnlyRegister(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnlyWhitelistedExecutor(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::WithdrawFailed(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for MystikoTBridgeProxyErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <CallCrossChainSyncTxError as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OnlyOperator as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OnlyRegister as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <OnlyWhitelistedExecutor as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <WithdrawFailed as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MystikoTBridgeProxyErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallCrossChainSyncTxError(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyRegister(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyWhitelistedExecutor(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MystikoTBridgeProxyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CallCrossChainSyncTxError> for MystikoTBridgeProxyErrors {
        fn from(value: CallCrossChainSyncTxError) -> Self {
            Self::CallCrossChainSyncTxError(value)
        }
    }
    impl ::core::convert::From<OnlyOperator> for MystikoTBridgeProxyErrors {
        fn from(value: OnlyOperator) -> Self {
            Self::OnlyOperator(value)
        }
    }
    impl ::core::convert::From<OnlyRegister> for MystikoTBridgeProxyErrors {
        fn from(value: OnlyRegister) -> Self {
            Self::OnlyRegister(value)
        }
    }
    impl ::core::convert::From<OnlyWhitelistedExecutor> for MystikoTBridgeProxyErrors {
        fn from(value: OnlyWhitelistedExecutor) -> Self {
            Self::OnlyWhitelistedExecutor(value)
        }
    }
    impl ::core::convert::From<WithdrawFailed> for MystikoTBridgeProxyErrors {
        fn from(value: WithdrawFailed) -> Self {
            Self::WithdrawFailed(value)
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
    #[ethevent(
        name = "TBridgeCrossChainMessage",
        abi = "TBridgeCrossChainMessage(address,uint256,address,bytes)"
    )]
    pub struct TbridgeCrossChainMessageFilter {
        pub to_contract: ::ethers_core::types::Address,
        pub to_chain_id: ::ethers_core::types::U256,
        pub from_contract: ::ethers_core::types::Address,
        pub message: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `addExecutorWhitelist` function with signature `addExecutorWhitelist(address)` and selector `0xa071e9b1`
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
    #[ethcall(name = "addExecutorWhitelist", abi = "addExecutorWhitelist(address)")]
    pub struct AddExecutorWhitelistCall {
        pub executor: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `addRegisterWhitelist` function with signature `addRegisterWhitelist(address)` and selector `0xbfdfd563`
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
    #[ethcall(name = "addRegisterWhitelist", abi = "addRegisterWhitelist(address)")]
    pub struct AddRegisterWhitelistCall {
        pub register: ::ethers_core::types::Address,
    }
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
    ///Container type for all input parameters for the `crossChainSyncTx` function with signature `crossChainSyncTx(uint64,address,address,address,bytes)` and selector `0x919c1df5`
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
        name = "crossChainSyncTx",
        abi = "crossChainSyncTx(uint64,address,address,address,bytes)"
    )]
    pub struct CrossChainSyncTxCall {
        pub from_chain_id: u64,
        pub from_contract: ::ethers_core::types::Address,
        pub to_contract: ::ethers_core::types::Address,
        pub executor: ::ethers_core::types::Address,
        pub message: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `removeExecutorWhitelist` function with signature `removeExecutorWhitelist(address)` and selector `0xd1520948`
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
    #[ethcall(name = "removeExecutorWhitelist", abi = "removeExecutorWhitelist(address)")]
    pub struct RemoveExecutorWhitelistCall {
        pub executor: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `removeRegisterWhitelist` function with signature `removeRegisterWhitelist(address)` and selector `0xf919d469`
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
    #[ethcall(name = "removeRegisterWhitelist", abi = "removeRegisterWhitelist(address)")]
    pub struct RemoveRegisterWhitelistCall {
        pub register: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `sendMessage` function with signature `sendMessage(address,uint64,bytes)` and selector `0xc81739cd`
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
    #[ethcall(name = "sendMessage", abi = "sendMessage(address,uint64,bytes)")]
    pub struct SendMessageCall {
        pub to_contract: ::ethers_core::types::Address,
        pub to_chain_id: u64,
        pub message: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(address)` and selector `0x51cff8d9`
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
    #[ethcall(name = "withdraw", abi = "withdraw(address)")]
    pub struct WithdrawCall {
        pub recipient: ::ethers_core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers_contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoTBridgeProxyCalls {
        AddExecutorWhitelist(AddExecutorWhitelistCall),
        AddRegisterWhitelist(AddRegisterWhitelistCall),
        ChangeOperator(ChangeOperatorCall),
        CrossChainSyncTx(CrossChainSyncTxCall),
        RemoveExecutorWhitelist(RemoveExecutorWhitelistCall),
        RemoveRegisterWhitelist(RemoveRegisterWhitelistCall),
        SendMessage(SendMessageCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoTBridgeProxyCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddExecutorWhitelistCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddExecutorWhitelist(decoded));
            }
            if let Ok(decoded) = <AddRegisterWhitelistCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddRegisterWhitelist(decoded));
            }
            if let Ok(decoded) = <ChangeOperatorCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChangeOperator(decoded));
            }
            if let Ok(decoded) = <CrossChainSyncTxCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CrossChainSyncTx(decoded));
            }
            if let Ok(decoded) = <RemoveExecutorWhitelistCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveExecutorWhitelist(decoded));
            }
            if let Ok(decoded) = <RemoveRegisterWhitelistCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveRegisterWhitelist(decoded));
            }
            if let Ok(decoded) = <SendMessageCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendMessage(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for MystikoTBridgeProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddExecutorWhitelist(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::AddRegisterWhitelist(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ChangeOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CrossChainSyncTx(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RemoveExecutorWhitelist(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RemoveRegisterWhitelist(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SendMessage(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MystikoTBridgeProxyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddExecutorWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddRegisterWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::CrossChainSyncTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveExecutorWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveRegisterWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddExecutorWhitelistCall> for MystikoTBridgeProxyCalls {
        fn from(value: AddExecutorWhitelistCall) -> Self {
            Self::AddExecutorWhitelist(value)
        }
    }
    impl ::core::convert::From<AddRegisterWhitelistCall> for MystikoTBridgeProxyCalls {
        fn from(value: AddRegisterWhitelistCall) -> Self {
            Self::AddRegisterWhitelist(value)
        }
    }
    impl ::core::convert::From<ChangeOperatorCall> for MystikoTBridgeProxyCalls {
        fn from(value: ChangeOperatorCall) -> Self {
            Self::ChangeOperator(value)
        }
    }
    impl ::core::convert::From<CrossChainSyncTxCall> for MystikoTBridgeProxyCalls {
        fn from(value: CrossChainSyncTxCall) -> Self {
            Self::CrossChainSyncTx(value)
        }
    }
    impl ::core::convert::From<RemoveExecutorWhitelistCall> for MystikoTBridgeProxyCalls {
        fn from(value: RemoveExecutorWhitelistCall) -> Self {
            Self::RemoveExecutorWhitelist(value)
        }
    }
    impl ::core::convert::From<RemoveRegisterWhitelistCall> for MystikoTBridgeProxyCalls {
        fn from(value: RemoveRegisterWhitelistCall) -> Self {
            Self::RemoveRegisterWhitelist(value)
        }
    }
    impl ::core::convert::From<SendMessageCall> for MystikoTBridgeProxyCalls {
        fn from(value: SendMessageCall) -> Self {
            Self::SendMessage(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for MystikoTBridgeProxyCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `crossChainSyncTx` function with signature `crossChainSyncTx(uint64,address,address,address,bytes)` and selector `0x919c1df5`
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
    pub struct CrossChainSyncTxReturn(pub bool);
}
