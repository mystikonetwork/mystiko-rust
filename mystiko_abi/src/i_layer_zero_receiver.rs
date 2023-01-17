pub use i_layer_zero_receiver::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_layer_zero_receiver {
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
    #[doc = "ILayerZeroReceiver was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"lzReceive\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ILAYERZERORECEIVER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ILayerZeroReceiver<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ILayerZeroReceiver<M> {
        fn clone(&self) -> Self {
            ILayerZeroReceiver(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ILayerZeroReceiver<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ILayerZeroReceiver<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ILayerZeroReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ILayerZeroReceiver<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ILAYERZERORECEIVER_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `lzReceive` (0x001d3567) function"]
        pub fn lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
            nonce: u64,
            payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 29, 53, 103],
                    (src_chain_id, src_address, nonce, payload),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ILayerZeroReceiver<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `lzReceive` function with signature `lzReceive(uint16,bytes,uint64,bytes)` and selector `[0, 29, 53, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lzReceive", abi = "lzReceive(uint16,bytes,uint64,bytes)")]
    pub struct LzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ethers::core::types::Bytes,
    }
}
