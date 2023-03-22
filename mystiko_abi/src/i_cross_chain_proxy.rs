pub use i_cross_chain_proxy::*;
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
pub mod i_cross_chain_proxy {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"toContract\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"toChainId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"fromContract\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TBridgeCrossChainMessage\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_toContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_toChainId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sendMessage\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ICROSSCHAINPROXY_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct ICrossChainProxy<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for ICrossChainProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ICrossChainProxy<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ICrossChainProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ICrossChainProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ICrossChainProxy)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> ICrossChainProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    ICROSSCHAINPROXY_ABI.clone(),
                    client,
                ),
            )
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
        ///Gets the contract's `TBridgeCrossChainMessage` event
        pub fn t_bridge_cross_chain_message_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TbridgeCrossChainMessageFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TbridgeCrossChainMessageFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for ICrossChainProxy<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "sendMessage", abi = "sendMessage(address,uint64,bytes)")]
    pub struct SendMessageCall {
        pub to_contract: ::ethers_core::types::Address,
        pub to_chain_id: u64,
        pub message: ::ethers_core::types::Bytes,
    }
}
