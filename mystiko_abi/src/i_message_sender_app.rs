pub use i_message_sender_app::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_message_sender_app {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_dstChainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sendMessage\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IMESSAGESENDERAPP_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IMessageSenderApp<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMessageSenderApp<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMessageSenderApp<M> {
        type Target = ::ethers_contract::Contract<M>;
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
            f.debug_tuple(stringify!(IMessageSenderApp)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IMessageSenderApp<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IMESSAGESENDERAPP_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `sendMessage` (0x9f3ce55a) function
        pub fn send_message(
            &self,
            receiver: ::ethers_core::types::Address,
            dst_chain_id: ::ethers_core::types::U256,
            message: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 60, 229, 90], (receiver, dst_chain_id, message))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IMessageSenderApp<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `sendMessage` function with signature `sendMessage(address,uint256,bytes)` and selector `0x9f3ce55a`
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
        Hash
    )]
    #[ethcall(name = "sendMessage", abi = "sendMessage(address,uint256,bytes)")]
    pub struct SendMessageCall {
        pub receiver: ::ethers_core::types::Address,
        pub dst_chain_id: ::ethers_core::types::U256,
        pub message: ::ethers_core::types::Bytes,
    }
}
