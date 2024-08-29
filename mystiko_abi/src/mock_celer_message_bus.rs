pub use mock_celer_message_bus::*;
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
pub mod mock_celer_message_bus {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("chainIdA"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("chainIdA"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("chainIdB"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("chainIdB"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("contractAddressA"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("contractAddressA"),
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
                    ::std::borrow::ToOwned::to_owned("contractAddressB"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("contractAddressB"),
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
                    ::std::borrow::ToOwned::to_owned("sendMessage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sendMessage"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_receiver"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_message"),
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
                    ::std::borrow::ToOwned::to_owned("setChainPair"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setChainPair"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainIdA"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_contractAddressA"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_chainIdB"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint64"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_contractAddressB"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKCELERMESSAGEBUS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x05x\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0UW`\x005`\xE0\x1C\x80c\x06>q(\x14a\0ZW\x80c\x10\xEFw\x95\x14a\0\x97W\x80c.`\x12\x1F\x14a\0\xD6W\x80cr\xB5=\xFA\x14a\0\xF6W\x80c\x9F<\xE5Z\x14a\x01^W\x80c\xBB\xFBR\xFB\x14a\x01qW[`\0\x80\xFD[4\x80\x15a\0fW`\0\x80\xFD[P`\0Ta\0z\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xA3W`\0\x80\xFD[P`\0Ta\0\xBE\x90`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x8EV[4\x80\x15a\0\xE2W`\0\x80\xFD[P`\x01Ta\0z\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x01\x02W`\0\x80\xFD[Pa\x01\\a\x01\x116`\x04a\x03\\V[`\0\x80T`\x01`\x01`@\x1B\x03\x95\x86\x16`\x01`\x01`\xE0\x1B\x03\x19\x91\x82\x16\x17`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x81\x02\x91\x90\x91\x17\x90\x92U`\x01\x80T\x94\x90\x96\x16\x93\x16\x92\x90\x92\x17\x92\x16\x02\x17\x90UV[\0[a\x01\\a\x01l6`\x04a\x03\xC6V[a\x01\x98V[4\x80\x15a\x01}W`\0\x80\xFD[P`\x01Ta\0\xBE\x90`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80a\x01\xA9V[`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`@\x1B\x03\x16\x84\x03a\x01\xE0WPP`\x01T`\x01`\x01`@\x1B\x03\x81\x16\x90`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x02WV[`\x01T`\x01`\x01`@\x1B\x03\x16\x84\x03a\x02\x17WPP`\0T`\x01`\x01`@\x1B\x03\x81\x16\x90`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x02WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt77\xBA\x109\xBA\xB887\xB9:\x10:44\xB9\x9082\xB2\xB9`Y\x1B`D\x82\x01R`d\x01a\x01\xA0V[`@Qc\x9Cd\x9F\xDF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x9Cd\x9F\xDF\x90a\x02\x89\x90\x84\x90\x86\x90\x88\x902\x90`\x04\x01a\x04\x96V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCC\x91\x90a\x05\x19V[a\x03\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fcall executeMessage returns erro`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01a\x01\xA0V[PPPPPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x03@W`\0\x80\xFD[\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03@W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x03rW`\0\x80\xFD[a\x03{\x85a\x03)V[\x93Pa\x03\x89` \x86\x01a\x03EV[\x92Pa\x03\x97`@\x86\x01a\x03)V[\x91Pa\x03\xA5``\x86\x01a\x03EV[\x90P\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x03\xDBW`\0\x80\xFD[a\x03\xE4\x84a\x03EV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x04\x06W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x04\x17W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x040Wa\x040a\x03\xB0V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04^Wa\x04^a\x03\xB0V[`@R\x81\x81R\x82\x82\x01` \x01\x88\x10\x15a\x04vW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R`\x01`\x01`@\x1B\x03\x84\x16` \x82\x01R`\x80`@\x82\x01R`\0\x83Q\x80`\x80\x84\x01R`\0[\x81\x81\x10\x15a\x04\xE1W` \x81\x87\x01\x81\x01Q`\xA0\x86\x84\x01\x01R\x01a\x04\xC4V[P`\0`\xA0\x82\x85\x01\x01R`\xA0`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PPa\x05\x10``\x83\x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x90RV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x05+W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05;W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xDE\xA3\x1B\xEF\xB8w\xF1o\x7F%a\x16\x86\xC3<\xC3~\x1C\xA2I\x8Ehu\x1A\xAB\xCA\x8EG|!\xD6\xF8dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MOCKCELERMESSAGEBUS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0UW`\x005`\xE0\x1C\x80c\x06>q(\x14a\0ZW\x80c\x10\xEFw\x95\x14a\0\x97W\x80c.`\x12\x1F\x14a\0\xD6W\x80cr\xB5=\xFA\x14a\0\xF6W\x80c\x9F<\xE5Z\x14a\x01^W\x80c\xBB\xFBR\xFB\x14a\x01qW[`\0\x80\xFD[4\x80\x15a\0fW`\0\x80\xFD[P`\0Ta\0z\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xA3W`\0\x80\xFD[P`\0Ta\0\xBE\x90`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x8EV[4\x80\x15a\0\xE2W`\0\x80\xFD[P`\x01Ta\0z\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x01\x02W`\0\x80\xFD[Pa\x01\\a\x01\x116`\x04a\x03\\V[`\0\x80T`\x01`\x01`@\x1B\x03\x95\x86\x16`\x01`\x01`\xE0\x1B\x03\x19\x91\x82\x16\x17`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x81\x02\x91\x90\x91\x17\x90\x92U`\x01\x80T\x94\x90\x96\x16\x93\x16\x92\x90\x92\x17\x92\x16\x02\x17\x90UV[\0[a\x01\\a\x01l6`\x04a\x03\xC6V[a\x01\x98V[4\x80\x15a\x01}W`\0\x80\xFD[P`\x01Ta\0\xBE\x90`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80a\x01\xA9V[`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`@\x1B\x03\x16\x84\x03a\x01\xE0WPP`\x01T`\x01`\x01`@\x1B\x03\x81\x16\x90`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x02WV[`\x01T`\x01`\x01`@\x1B\x03\x16\x84\x03a\x02\x17WPP`\0T`\x01`\x01`@\x1B\x03\x81\x16\x90`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x02WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt77\xBA\x109\xBA\xB887\xB9:\x10:44\xB9\x9082\xB2\xB9`Y\x1B`D\x82\x01R`d\x01a\x01\xA0V[`@Qc\x9Cd\x9F\xDF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x9Cd\x9F\xDF\x90a\x02\x89\x90\x84\x90\x86\x90\x88\x902\x90`\x04\x01a\x04\x96V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCC\x91\x90a\x05\x19V[a\x03\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fcall executeMessage returns erro`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01a\x01\xA0V[PPPPPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x03@W`\0\x80\xFD[\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03@W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x03rW`\0\x80\xFD[a\x03{\x85a\x03)V[\x93Pa\x03\x89` \x86\x01a\x03EV[\x92Pa\x03\x97`@\x86\x01a\x03)V[\x91Pa\x03\xA5``\x86\x01a\x03EV[\x90P\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x03\xDBW`\0\x80\xFD[a\x03\xE4\x84a\x03EV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x04\x06W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x04\x17W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x040Wa\x040a\x03\xB0V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04^Wa\x04^a\x03\xB0V[`@R\x81\x81R\x82\x82\x01` \x01\x88\x10\x15a\x04vW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R`\x01`\x01`@\x1B\x03\x84\x16` \x82\x01R`\x80`@\x82\x01R`\0\x83Q\x80`\x80\x84\x01R`\0[\x81\x81\x10\x15a\x04\xE1W` \x81\x87\x01\x81\x01Q`\xA0\x86\x84\x01\x01R\x01a\x04\xC4V[P`\0`\xA0\x82\x85\x01\x01R`\xA0`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PPa\x05\x10``\x83\x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x90RV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x05+W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05;W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xDE\xA3\x1B\xEF\xB8w\xF1o\x7F%a\x16\x86\xC3<\xC3~\x1C\xA2I\x8Ehu\x1A\xAB\xCA\x8EG|!\xD6\xF8dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKCELERMESSAGEBUS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockCelerMessageBus<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockCelerMessageBus<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockCelerMessageBus<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockCelerMessageBus<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockCelerMessageBus<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockCelerMessageBus))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockCelerMessageBus<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKCELERMESSAGEBUS_ABI.clone(),
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
                MOCKCELERMESSAGEBUS_ABI.clone(),
                MOCKCELERMESSAGEBUS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `chainIdA` (0x063e7128) function
        pub fn chain_id_a(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([6, 62, 113, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainIdB` (0x2e60121f) function
        pub fn chain_id_b(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([46, 96, 18, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contractAddressA` (0x10ef7795) function
        pub fn contract_address_a(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([16, 239, 119, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contractAddressB` (0xbbfb52fb) function
        pub fn contract_address_b(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([187, 251, 82, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendMessage` (0x9f3ce55a) function
        pub fn send_message(
            &self,
            receiver: ::ethers::core::types::Address,
            dst_chain_id: ::ethers::core::types::U256,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 60, 229, 90], (receiver, dst_chain_id, message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setChainPair` (0x72b53dfa) function
        pub fn set_chain_pair(
            &self,
            chain_id_a: u64,
            contract_address_a: ::ethers::core::types::Address,
            chain_id_b: u64,
            contract_address_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [114, 181, 61, 250],
                    (chain_id_a, contract_address_a, chain_id_b, contract_address_b),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockCelerMessageBus<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `chainIdA` function with signature `chainIdA()` and selector `0x063e7128`
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
    #[ethcall(name = "chainIdA", abi = "chainIdA()")]
    pub struct ChainIdACall;
    ///Container type for all input parameters for the `chainIdB` function with signature `chainIdB()` and selector `0x2e60121f`
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
    #[ethcall(name = "chainIdB", abi = "chainIdB()")]
    pub struct ChainIdBCall;
    ///Container type for all input parameters for the `contractAddressA` function with signature `contractAddressA()` and selector `0x10ef7795`
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
    #[ethcall(name = "contractAddressA", abi = "contractAddressA()")]
    pub struct ContractAddressACall;
    ///Container type for all input parameters for the `contractAddressB` function with signature `contractAddressB()` and selector `0xbbfb52fb`
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
    #[ethcall(name = "contractAddressB", abi = "contractAddressB()")]
    pub struct ContractAddressBCall;
    ///Container type for all input parameters for the `sendMessage` function with signature `sendMessage(address,uint256,bytes)` and selector `0x9f3ce55a`
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
    #[ethcall(name = "sendMessage", abi = "sendMessage(address,uint256,bytes)")]
    pub struct SendMessageCall {
        pub receiver: ::ethers::core::types::Address,
        pub dst_chain_id: ::ethers::core::types::U256,
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setChainPair` function with signature `setChainPair(uint64,address,uint64,address)` and selector `0x72b53dfa`
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
    #[ethcall(name = "setChainPair", abi = "setChainPair(uint64,address,uint64,address)")]
    pub struct SetChainPairCall {
        pub chain_id_a: u64,
        pub contract_address_a: ::ethers::core::types::Address,
        pub chain_id_b: u64,
        pub contract_address_b: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockCelerMessageBusCalls {
        ChainIdA(ChainIdACall),
        ChainIdB(ChainIdBCall),
        ContractAddressA(ContractAddressACall),
        ContractAddressB(ContractAddressBCall),
        SendMessage(SendMessageCall),
        SetChainPair(SetChainPairCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockCelerMessageBusCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ChainIdACall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChainIdA(decoded));
            }
            if let Ok(decoded) = <ChainIdBCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChainIdB(decoded));
            }
            if let Ok(decoded) = <ContractAddressACall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ContractAddressA(decoded));
            }
            if let Ok(decoded) = <ContractAddressBCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ContractAddressB(decoded));
            }
            if let Ok(decoded) = <SendMessageCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendMessage(decoded));
            }
            if let Ok(decoded) = <SetChainPairCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetChainPair(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockCelerMessageBusCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ChainIdA(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChainIdB(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ContractAddressA(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ContractAddressB(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SendMessage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetChainPair(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockCelerMessageBusCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChainIdA(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainIdB(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractAddressA(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractAddressB(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetChainPair(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChainIdACall> for MockCelerMessageBusCalls {
        fn from(value: ChainIdACall) -> Self {
            Self::ChainIdA(value)
        }
    }
    impl ::core::convert::From<ChainIdBCall> for MockCelerMessageBusCalls {
        fn from(value: ChainIdBCall) -> Self {
            Self::ChainIdB(value)
        }
    }
    impl ::core::convert::From<ContractAddressACall> for MockCelerMessageBusCalls {
        fn from(value: ContractAddressACall) -> Self {
            Self::ContractAddressA(value)
        }
    }
    impl ::core::convert::From<ContractAddressBCall> for MockCelerMessageBusCalls {
        fn from(value: ContractAddressBCall) -> Self {
            Self::ContractAddressB(value)
        }
    }
    impl ::core::convert::From<SendMessageCall> for MockCelerMessageBusCalls {
        fn from(value: SendMessageCall) -> Self {
            Self::SendMessage(value)
        }
    }
    impl ::core::convert::From<SetChainPairCall> for MockCelerMessageBusCalls {
        fn from(value: SetChainPairCall) -> Self {
            Self::SetChainPair(value)
        }
    }
    ///Container type for all return fields from the `chainIdA` function with signature `chainIdA()` and selector `0x063e7128`
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
    pub struct ChainIdAReturn(pub u64);
    ///Container type for all return fields from the `chainIdB` function with signature `chainIdB()` and selector `0x2e60121f`
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
    pub struct ChainIdBReturn(pub u64);
    ///Container type for all return fields from the `contractAddressA` function with signature `contractAddressA()` and selector `0x10ef7795`
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
    pub struct ContractAddressAReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `contractAddressB` function with signature `contractAddressB()` and selector `0xbbfb52fb`
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
    pub struct ContractAddressBReturn(pub ::ethers::core::types::Address);
}
