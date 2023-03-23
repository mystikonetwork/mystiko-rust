pub use i_layer_zero_receiver::*;
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
pub mod i_layer_zero_receiver {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"lzReceive\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ILAYERZERORECEIVER_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct ILayerZeroReceiver<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for ILayerZeroReceiver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ILayerZeroReceiver<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ILayerZeroReceiver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ILayerZeroReceiver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ILayerZeroReceiver)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> ILayerZeroReceiver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    ILAYERZERORECEIVER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `lzReceive` (0x001d3567) function
        pub fn lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers_core::types::Bytes,
            nonce: u64,
            payload: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 29, 53, 103],
                    (src_chain_id, src_address, nonce, payload),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for ILayerZeroReceiver<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `lzReceive` function with signature `lzReceive(uint16,bytes,uint64,bytes)` and selector `0x001d3567`
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
    #[ethcall(name = "lzReceive", abi = "lzReceive(uint16,bytes,uint64,bytes)")]
    pub struct LzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers_core::types::Bytes,
    }
}
