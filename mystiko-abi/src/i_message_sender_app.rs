pub use i_message_sender_app::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_message_sender_app {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IMessageSenderApp was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_dstChainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sendMessage\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IMESSAGESENDERAPP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IMessageSenderApp<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IMessageSenderApp<M> {
        fn clone(&self) -> Self {
            IMessageSenderApp(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IMessageSenderApp<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IMessageSenderApp<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IMessageSenderApp))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IMessageSenderApp<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IMESSAGESENDERAPP_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `sendMessage` (0x9f3ce55a) function"]
        pub fn send_message(
            &self,
            receiver: ethers::core::types::Address,
            dst_chain_id: ethers::core::types::U256,
            message: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 60, 229, 90], (receiver, dst_chain_id, message))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IMessageSenderApp<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `sendMessage` function with signature `sendMessage(address,uint256,bytes)` and selector `[159, 60, 229, 90]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sendMessage", abi = "sendMessage(address,uint256,bytes)")]
    pub struct SendMessageCall {
        pub receiver: ethers::core::types::Address,
        pub dst_chain_id: ethers::core::types::U256,
        pub message: ethers::core::types::Bytes,
    }
}
