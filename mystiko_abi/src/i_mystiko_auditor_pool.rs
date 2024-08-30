pub use i_mystiko_auditor_pool::*;
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
pub mod i_mystiko_auditor_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IMYSTIKOAUDITORPOOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IMystikoAuditorPool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMystikoAuditorPool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMystikoAuditorPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IMystikoAuditorPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IMystikoAuditorPool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IMystikoAuditorPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IMystikoAuditorPool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IMYSTIKOAUDITORPOOL_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `queryAllAuditorPublicKeys` (0xc112de6c) function
        pub fn query_all_auditor_public_keys(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<::ethers::core::types::U256>> {
            self.0
                .method_hash([193, 18, 222, 108], ())
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for IMystikoAuditorPool<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum IMystikoAuditorPoolCalls {
        QueryAllAuditorPublicKeys(QueryAllAuditorPublicKeysCall),
        QueryAuditorPublicKey(QueryAuditorPublicKeyCall),
    }
    impl ::ethers::core::abi::AbiDecode for IMystikoAuditorPoolCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <QueryAllAuditorPublicKeysCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryAllAuditorPublicKeys(decoded));
            }
            if let Ok(decoded) = <QueryAuditorPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryAuditorPublicKey(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IMystikoAuditorPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::QueryAllAuditorPublicKeys(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryAuditorPublicKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IMystikoAuditorPoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::QueryAllAuditorPublicKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryAuditorPublicKey(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<QueryAllAuditorPublicKeysCall> for IMystikoAuditorPoolCalls {
        fn from(value: QueryAllAuditorPublicKeysCall) -> Self {
            Self::QueryAllAuditorPublicKeys(value)
        }
    }
    impl ::core::convert::From<QueryAuditorPublicKeyCall> for IMystikoAuditorPoolCalls {
        fn from(value: QueryAuditorPublicKeyCall) -> Self {
            Self::QueryAuditorPublicKey(value)
        }
    }
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
}
