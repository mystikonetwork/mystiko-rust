pub use i_message_sender_app::*;
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
pub mod i_message_sender_app {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("sendMessage"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("sendMessage"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_receiver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
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
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IMESSAGESENDERAPP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IMessageSenderApp<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMessageSenderApp<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMessageSenderApp<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IMessageSenderApp<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IMessageSenderApp<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IMessageSenderApp))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IMessageSenderApp<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IMESSAGESENDERAPP_ABI.clone(),
                client,
            ))
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for IMessageSenderApp<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
}
