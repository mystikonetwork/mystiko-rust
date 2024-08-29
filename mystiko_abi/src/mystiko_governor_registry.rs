pub use mystiko_governor_registry::*;
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
pub mod mystiko_governor_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("dao"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dao"),
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
                    ::std::borrow::ToOwned::to_owned("daoMap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("daoMap"),
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
                    ::std::borrow::ToOwned::to_owned("deployer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deployer"),
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
                    ::std::borrow::ToOwned::to_owned("renounceDeployer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceDeployer"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rollBackMystikoDAO"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rollBackMystikoDAO"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_previousDao"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMystikoDAO"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMystikoDAO"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_newDao"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnerToDAO"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnerToDAO"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_newDao"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DeployerRenounced"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DeployerRenounced"),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MystikoDAOChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MystikoDAOChanged"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("newDao"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMystikoDAOAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidMystikoDAOAddress",),
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
                    ::std::borrow::ToOwned::to_owned("OnlyBeforeDaoInitialized"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyBeforeDaoInitialized",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyDeployer"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyDeployer"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MYSTIKOGOVERNORREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\0\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x81\x17\x83U`\x01\x80T\x90\x92\x16\x17\x90Ua\x04D\x90\x81\x90a\0?\x909`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x95E\xC1q\x11a\0[W\x80c\x95E\xC1q\x14a\0\xDAW\x80c\xBFo\0\xD8\x14a\x01\rW\x80c\xD5\xF3\x94\x88\x14a\x01\x15W\x80c\xDD\xEE\x8F\x9B\x14a\x01(W`\0\x80\xFD[\x80c&\xAB\xB3\xFD\x14a\0\x82W\x80c'x\x1F\xF0\x14a\0\x97W\x80cAb\x16\x9F\x14a\0\xAAW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x03\xDEV[a\x01;V[\0[a\0\x95a\0\xA56`\x04a\x03\xDEV[a\x02\x15V[`\0Ta\0\xBD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xFDa\0\xE86`\x04a\x03\xDEV[`\x02` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xD1V[a\0\x95a\x03\tV[`\x01Ta\0\xBD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\x95a\x0166`\x04a\x03\xDEV[a\x03oV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01fW`@Qca\x8B\xBD\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x01\x95W`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x01\xCDW`@Qb>\x14\x87`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x91\x7F\xDE.\x13$\xF3\xDF<\x01\xCA\x8Dm\xF2\xFF\xE5TC\x0C\x8A\xD8LH\xBE\xF8_\x85\x08u\xE1_\x81\xEB\x86\x91\xA2PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15\x80a\x029WP`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14[\x15a\x02WW`@Qc\x17{\xC9Q`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02}W`@Qb>\x14\x87`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x03a\x02\xABW`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U\x80\x82R`\x02` R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91\x7F\xDE.\x13$\xF3\xDF<\x01\xCA\x8Dm\xF2\xFF\xE5TC\x0C\x8A\xD8LH\xBE\xF8_\x85\x08u\xE1_\x81\xEB\x86\x91\xA2PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x034W`@Qca\x8B\xBD\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`@Q\x7F\x10\xDC\x946,\xCE\xAF\xB0\x17A\xF9\xB4\xA7s\xA3(\xC5I\xFB\x0F\xF6\xF7-\xDA\xDD\xED\xC4\xB7\xB9oo\x19\x90`\0\x90\xA1V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15\x80a\x03\x9AWP`\x01T`\0T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x15[\x15a\x03\xB8W`@Qc6=\xA3I`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02\xABW`@Qb>\x14\x87`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xF0W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x07W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 8\x90\xB1:\xD8\xFE|\xAB7\0p\xBF\xE1\x0E\x11\x89\xB0\xF0\xFE\xA2\x85,\xEB>\x84\x87cmy\x80\x15\xA4dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MYSTIKOGOVERNORREGISTRY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x95E\xC1q\x11a\0[W\x80c\x95E\xC1q\x14a\0\xDAW\x80c\xBFo\0\xD8\x14a\x01\rW\x80c\xD5\xF3\x94\x88\x14a\x01\x15W\x80c\xDD\xEE\x8F\x9B\x14a\x01(W`\0\x80\xFD[\x80c&\xAB\xB3\xFD\x14a\0\x82W\x80c'x\x1F\xF0\x14a\0\x97W\x80cAb\x16\x9F\x14a\0\xAAW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x03\xDEV[a\x01;V[\0[a\0\x95a\0\xA56`\x04a\x03\xDEV[a\x02\x15V[`\0Ta\0\xBD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xFDa\0\xE86`\x04a\x03\xDEV[`\x02` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xD1V[a\0\x95a\x03\tV[`\x01Ta\0\xBD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\x95a\x0166`\x04a\x03\xDEV[a\x03oV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01fW`@Qca\x8B\xBD\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x01\x95W`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x01\xCDW`@Qb>\x14\x87`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x91\x7F\xDE.\x13$\xF3\xDF<\x01\xCA\x8Dm\xF2\xFF\xE5TC\x0C\x8A\xD8LH\xBE\xF8_\x85\x08u\xE1_\x81\xEB\x86\x91\xA2PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15\x80a\x029WP`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14[\x15a\x02WW`@Qc\x17{\xC9Q`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02}W`@Qb>\x14\x87`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x03a\x02\xABW`@Qc6\xA1\xC3?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U\x80\x82R`\x02` R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91\x7F\xDE.\x13$\xF3\xDF<\x01\xCA\x8Dm\xF2\xFF\xE5TC\x0C\x8A\xD8LH\xBE\xF8_\x85\x08u\xE1_\x81\xEB\x86\x91\xA2PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x034W`@Qca\x8B\xBD\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`@Q\x7F\x10\xDC\x946,\xCE\xAF\xB0\x17A\xF9\xB4\xA7s\xA3(\xC5I\xFB\x0F\xF6\xF7-\xDA\xDD\xED\xC4\xB7\xB9oo\x19\x90`\0\x90\xA1V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15\x80a\x03\x9AWP`\x01T`\0T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x15[\x15a\x03\xB8W`@Qc6=\xA3I`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02\xABW`@Qb>\x14\x87`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xF0W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x07W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 8\x90\xB1:\xD8\xFE|\xAB7\0p\xBF\xE1\x0E\x11\x89\xB0\xF0\xFE\xA2\x85,\xEB>\x84\x87cmy\x80\x15\xA4dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MYSTIKOGOVERNORREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MystikoGovernorRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoGovernorRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoGovernorRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoGovernorRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoGovernorRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MystikoGovernorRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MystikoGovernorRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MYSTIKOGOVERNORREGISTRY_ABI.clone(),
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
                MYSTIKOGOVERNORREGISTRY_ABI.clone(),
                MYSTIKOGOVERNORREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `dao` (0x4162169f) function
        pub fn dao(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([65, 98, 22, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `daoMap` (0x9545c171) function
        pub fn dao_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([149, 69, 193, 113], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployer` (0xd5f39488) function
        pub fn deployer(&self) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([213, 243, 148, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceDeployer` (0xbf6f00d8) function
        pub fn renounce_deployer(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 111, 0, 216], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollBackMystikoDAO` (0x26abb3fd) function
        pub fn roll_back_mystiko_dao(
            &self,
            previous_dao: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 171, 179, 253], previous_dao)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMystikoDAO` (0x27781ff0) function
        pub fn set_mystiko_dao(
            &self,
            new_dao: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 120, 31, 240], new_dao)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnerToDAO` (0xddee8f9b) function
        pub fn transfer_owner_to_dao(
            &self,
            new_dao: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 238, 143, 155], new_dao)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DeployerRenounced` event
        pub fn deployer_renounced_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DeployerRenouncedFilter> {
            self.0.event()
        }
        ///Gets the contract's `MystikoDAOChanged` event
        pub fn mystiko_dao_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MystikoDAOChangedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MystikoGovernorRegistryEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MystikoGovernorRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidMystikoDAOAddress` with signature `InvalidMystikoDAOAddress()` and selector `0x03e14870`
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
    #[etherror(name = "InvalidMystikoDAOAddress", abi = "InvalidMystikoDAOAddress()")]
    pub struct InvalidMystikoDAOAddress;
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
    ///Custom Error type `OnlyBeforeDaoInitialized` with signature `OnlyBeforeDaoInitialized()` and selector `0x6c7b4692`
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
    #[etherror(name = "OnlyBeforeDaoInitialized", abi = "OnlyBeforeDaoInitialized()")]
    pub struct OnlyBeforeDaoInitialized;
    ///Custom Error type `OnlyDeployer` with signature `OnlyDeployer()` and selector `0x618bbdd5`
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
    #[etherror(name = "OnlyDeployer", abi = "OnlyDeployer()")]
    pub struct OnlyDeployer;
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
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoGovernorRegistryErrors {
        InvalidMystikoDAOAddress(InvalidMystikoDAOAddress),
        NotChanged(NotChanged),
        OnlyBeforeDaoInitialized(OnlyBeforeDaoInitialized),
        OnlyDeployer(OnlyDeployer),
        OnlyMystikoDAO(OnlyMystikoDAO),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MystikoGovernorRegistryErrors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidMystikoDAOAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidMystikoDAOAddress(decoded));
            }
            if let Ok(decoded) = <NotChanged as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotChanged(decoded));
            }
            if let Ok(decoded) = <OnlyBeforeDaoInitialized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyBeforeDaoInitialized(decoded));
            }
            if let Ok(decoded) = <OnlyDeployer as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyDeployer(decoded));
            }
            if let Ok(decoded) = <OnlyMystikoDAO as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyMystikoDAO(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MystikoGovernorRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidMystikoDAOAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotChanged(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyBeforeDaoInitialized(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyDeployer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyMystikoDAO(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MystikoGovernorRegistryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <InvalidMystikoDAOAddress as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NotChanged as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <OnlyBeforeDaoInitialized as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <OnlyDeployer as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <OnlyMystikoDAO as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MystikoGovernorRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidMystikoDAOAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotChanged(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyBeforeDaoInitialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyDeployer(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyMystikoDAO(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MystikoGovernorRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidMystikoDAOAddress> for MystikoGovernorRegistryErrors {
        fn from(value: InvalidMystikoDAOAddress) -> Self {
            Self::InvalidMystikoDAOAddress(value)
        }
    }
    impl ::core::convert::From<NotChanged> for MystikoGovernorRegistryErrors {
        fn from(value: NotChanged) -> Self {
            Self::NotChanged(value)
        }
    }
    impl ::core::convert::From<OnlyBeforeDaoInitialized> for MystikoGovernorRegistryErrors {
        fn from(value: OnlyBeforeDaoInitialized) -> Self {
            Self::OnlyBeforeDaoInitialized(value)
        }
    }
    impl ::core::convert::From<OnlyDeployer> for MystikoGovernorRegistryErrors {
        fn from(value: OnlyDeployer) -> Self {
            Self::OnlyDeployer(value)
        }
    }
    impl ::core::convert::From<OnlyMystikoDAO> for MystikoGovernorRegistryErrors {
        fn from(value: OnlyMystikoDAO) -> Self {
            Self::OnlyMystikoDAO(value)
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
    #[ethevent(name = "DeployerRenounced", abi = "DeployerRenounced()")]
    pub struct DeployerRenouncedFilter;
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
    #[ethevent(name = "MystikoDAOChanged", abi = "MystikoDAOChanged(address)")]
    pub struct MystikoDAOChangedFilter {
        #[ethevent(indexed)]
        pub new_dao: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoGovernorRegistryEvents {
        DeployerRenouncedFilter(DeployerRenouncedFilter),
        MystikoDAOChangedFilter(MystikoDAOChangedFilter),
    }
    impl ::ethers::contract::EthLogDecode for MystikoGovernorRegistryEvents {
        fn decode_log(log: &::ethers::core::abi::RawLog) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DeployerRenouncedFilter::decode_log(log) {
                return Ok(MystikoGovernorRegistryEvents::DeployerRenouncedFilter(decoded));
            }
            if let Ok(decoded) = MystikoDAOChangedFilter::decode_log(log) {
                return Ok(MystikoGovernorRegistryEvents::MystikoDAOChangedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MystikoGovernorRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeployerRenouncedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MystikoDAOChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DeployerRenouncedFilter> for MystikoGovernorRegistryEvents {
        fn from(value: DeployerRenouncedFilter) -> Self {
            Self::DeployerRenouncedFilter(value)
        }
    }
    impl ::core::convert::From<MystikoDAOChangedFilter> for MystikoGovernorRegistryEvents {
        fn from(value: MystikoDAOChangedFilter) -> Self {
            Self::MystikoDAOChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `dao` function with signature `dao()` and selector `0x4162169f`
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
    #[ethcall(name = "dao", abi = "dao()")]
    pub struct DaoCall;
    ///Container type for all input parameters for the `daoMap` function with signature `daoMap(address)` and selector `0x9545c171`
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
    #[ethcall(name = "daoMap", abi = "daoMap(address)")]
    pub struct DaoMapCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `deployer` function with signature `deployer()` and selector `0xd5f39488`
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
    #[ethcall(name = "deployer", abi = "deployer()")]
    pub struct DeployerCall;
    ///Container type for all input parameters for the `renounceDeployer` function with signature `renounceDeployer()` and selector `0xbf6f00d8`
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
    #[ethcall(name = "renounceDeployer", abi = "renounceDeployer()")]
    pub struct RenounceDeployerCall;
    ///Container type for all input parameters for the `rollBackMystikoDAO` function with signature `rollBackMystikoDAO(address)` and selector `0x26abb3fd`
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
    #[ethcall(name = "rollBackMystikoDAO", abi = "rollBackMystikoDAO(address)")]
    pub struct RollBackMystikoDAOCall {
        pub previous_dao: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setMystikoDAO` function with signature `setMystikoDAO(address)` and selector `0x27781ff0`
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
    #[ethcall(name = "setMystikoDAO", abi = "setMystikoDAO(address)")]
    pub struct SetMystikoDAOCall {
        pub new_dao: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnerToDAO` function with signature `transferOwnerToDAO(address)` and selector `0xddee8f9b`
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
    #[ethcall(name = "transferOwnerToDAO", abi = "transferOwnerToDAO(address)")]
    pub struct TransferOwnerToDAOCall {
        pub new_dao: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MystikoGovernorRegistryCalls {
        Dao(DaoCall),
        DaoMap(DaoMapCall),
        Deployer(DeployerCall),
        RenounceDeployer(RenounceDeployerCall),
        RollBackMystikoDAO(RollBackMystikoDAOCall),
        SetMystikoDAO(SetMystikoDAOCall),
        TransferOwnerToDAO(TransferOwnerToDAOCall),
    }
    impl ::ethers::core::abi::AbiDecode for MystikoGovernorRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DaoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Dao(decoded));
            }
            if let Ok(decoded) = <DaoMapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DaoMap(decoded));
            }
            if let Ok(decoded) = <DeployerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deployer(decoded));
            }
            if let Ok(decoded) = <RenounceDeployerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceDeployer(decoded));
            }
            if let Ok(decoded) = <RollBackMystikoDAOCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RollBackMystikoDAO(decoded));
            }
            if let Ok(decoded) = <SetMystikoDAOCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMystikoDAO(decoded));
            }
            if let Ok(decoded) = <TransferOwnerToDAOCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferOwnerToDAO(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MystikoGovernorRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Dao(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DaoMap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deployer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceDeployer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RollBackMystikoDAO(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMystikoDAO(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnerToDAO(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MystikoGovernorRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Dao(element) => ::core::fmt::Display::fmt(element, f),
                Self::DaoMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deployer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceDeployer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollBackMystikoDAO(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMystikoDAO(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnerToDAO(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DaoCall> for MystikoGovernorRegistryCalls {
        fn from(value: DaoCall) -> Self {
            Self::Dao(value)
        }
    }
    impl ::core::convert::From<DaoMapCall> for MystikoGovernorRegistryCalls {
        fn from(value: DaoMapCall) -> Self {
            Self::DaoMap(value)
        }
    }
    impl ::core::convert::From<DeployerCall> for MystikoGovernorRegistryCalls {
        fn from(value: DeployerCall) -> Self {
            Self::Deployer(value)
        }
    }
    impl ::core::convert::From<RenounceDeployerCall> for MystikoGovernorRegistryCalls {
        fn from(value: RenounceDeployerCall) -> Self {
            Self::RenounceDeployer(value)
        }
    }
    impl ::core::convert::From<RollBackMystikoDAOCall> for MystikoGovernorRegistryCalls {
        fn from(value: RollBackMystikoDAOCall) -> Self {
            Self::RollBackMystikoDAO(value)
        }
    }
    impl ::core::convert::From<SetMystikoDAOCall> for MystikoGovernorRegistryCalls {
        fn from(value: SetMystikoDAOCall) -> Self {
            Self::SetMystikoDAO(value)
        }
    }
    impl ::core::convert::From<TransferOwnerToDAOCall> for MystikoGovernorRegistryCalls {
        fn from(value: TransferOwnerToDAOCall) -> Self {
            Self::TransferOwnerToDAO(value)
        }
    }
    ///Container type for all return fields from the `dao` function with signature `dao()` and selector `0x4162169f`
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
    pub struct DaoReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `daoMap` function with signature `daoMap(address)` and selector `0x9545c171`
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
    pub struct DaoMapReturn(pub bool);
    ///Container type for all return fields from the `deployer` function with signature `deployer()` and selector `0xd5f39488`
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
    pub struct DeployerReturn(pub ::ethers::core::types::Address);
}
