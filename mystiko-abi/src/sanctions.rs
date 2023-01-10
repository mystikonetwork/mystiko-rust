pub use sanctions::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod sanctions {
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
    #[doc = "Sanctions was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SanctionsCheck\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"sanctions\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SanctionsList\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sanctionsCheck\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sanctionsList\",\"outputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static SANCTIONS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Sanctions<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Sanctions<M> {
        fn clone(&self) -> Self {
            Sanctions(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Sanctions<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Sanctions<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Sanctions))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Sanctions<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), SANCTIONS_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `sanctionsCheck` (0xb1c39422) function"]
        pub fn sanctions_check(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 195, 148, 34], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sanctionsList` (0xec571c6a) function"]
        pub fn sanctions_list(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([236, 87, 28, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SanctionsCheck` event"]
        pub fn sanctions_check_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SanctionsCheckFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SanctionsList` event"]
        pub fn sanctions_list_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SanctionsListFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, SanctionsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Sanctions<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SanctionsCheck", abi = "SanctionsCheck(bool)")]
    pub struct SanctionsCheckFilter {
        pub state: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SanctionsList", abi = "SanctionsList(address)")]
    pub struct SanctionsListFilter {
        pub sanctions: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum SanctionsEvents {
        SanctionsCheckFilter(SanctionsCheckFilter),
        SanctionsListFilter(SanctionsListFilter),
    }
    impl ethers::contract::EthLogDecode for SanctionsEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(SanctionsEvents::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(SanctionsEvents::SanctionsListFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for SanctionsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                SanctionsEvents::SanctionsCheckFilter(element) => element.fmt(f),
                SanctionsEvents::SanctionsListFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `sanctionsCheck` function with signature `sanctionsCheck()` and selector `[177, 195, 148, 34]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sanctionsCheck", abi = "sanctionsCheck()")]
    pub struct SanctionsCheckCall;
    #[doc = "Container type for all input parameters for the `sanctionsList` function with signature `sanctionsList()` and selector `[236, 87, 28, 106]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sanctionsList", abi = "sanctionsList()")]
    pub struct SanctionsListCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum SanctionsCalls {
        SanctionsCheck(SanctionsCheckCall),
        SanctionsList(SanctionsListCall),
    }
    impl ethers::core::abi::AbiDecode for SanctionsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <SanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SanctionsCalls::SanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <SanctionsListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SanctionsCalls::SanctionsList(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for SanctionsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                SanctionsCalls::SanctionsCheck(element) => element.encode(),
                SanctionsCalls::SanctionsList(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for SanctionsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                SanctionsCalls::SanctionsCheck(element) => element.fmt(f),
                SanctionsCalls::SanctionsList(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<SanctionsCheckCall> for SanctionsCalls {
        fn from(var: SanctionsCheckCall) -> Self {
            SanctionsCalls::SanctionsCheck(var)
        }
    }
    impl ::std::convert::From<SanctionsListCall> for SanctionsCalls {
        fn from(var: SanctionsListCall) -> Self {
            SanctionsCalls::SanctionsList(var)
        }
    }
    #[doc = "Container type for all return fields from the `sanctionsCheck` function with signature `sanctionsCheck()` and selector `[177, 195, 148, 34]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SanctionsCheckReturn(pub bool);
    #[doc = "Container type for all return fields from the `sanctionsList` function with signature `sanctionsList()` and selector `[236, 87, 28, 106]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SanctionsListReturn(pub ethers::core::types::Address);
}
