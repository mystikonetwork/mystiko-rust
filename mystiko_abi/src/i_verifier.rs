pub use i_verifier::*;
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
pub mod i_verifier {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("verifyTx"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("verifyTx"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("proof"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Uint(256usize),),
                                        2usize,
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Uint(256usize),),
                                        2usize,
                                    ),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct IVerifier.Proof"
                            ),),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ),),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256[]"),),
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
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IVERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(__abi);
    pub struct IVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IVerifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IVERIFIER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `verifyTx` (0xc9417647) function
        pub fn verify_tx(
            &self,
            proof: Proof,
            input: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([201, 65, 118, 71], (proof, input))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for IVerifier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `verifyTx` function with signature `verifyTx(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256[])` and selector `0xc9417647`
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
        name = "verifyTx",
        abi = "verifyTx(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256[])"
    )]
    pub struct VerifyTxCall {
        pub proof: Proof,
        pub input: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `verifyTx` function with signature `verifyTx(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256[])` and selector `0xc9417647`
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
    pub struct VerifyTxReturn(pub bool);
}
