pub use i_message_receiver_app::*;
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
pub mod i_message_receiver_app {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_srcChainId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_message\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_executor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"executeMessage\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IMESSAGERECEIVERAPP_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IMessageReceiverApp<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMessageReceiverApp<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMessageReceiverApp<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IMessageReceiverApp<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IMessageReceiverApp<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IMessageReceiverApp))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IMessageReceiverApp<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IMESSAGERECEIVERAPP_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `executeMessage` (0x9c649fdf) function
        pub fn execute_message(
            &self,
            sender: ::ethers_core::types::Address,
            src_chain_id: u64,
            message: ::ethers_core::types::Bytes,
            executor: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [156, 100, 159, 223],
                    (sender, src_chain_id, message, executor),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IMessageReceiverApp<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `executeMessage` function with signature `executeMessage(address,uint64,bytes,address)` and selector `0x9c649fdf`
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
    #[ethcall(
        name = "executeMessage",
        abi = "executeMessage(address,uint64,bytes,address)"
    )]
    pub struct ExecuteMessageCall {
        pub sender: ::ethers_core::types::Address,
        pub src_chain_id: u64,
        pub message: ::ethers_core::types::Bytes,
        pub executor: ::ethers_core::types::Address,
    }
    ///Container type for all return fields from the `executeMessage` function with signature `executeMessage(address,uint64,bytes,address)` and selector `0x9c649fdf`
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
        Hash
    )]
    pub struct ExecuteMessageReturn(pub bool);
}
