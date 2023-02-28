pub use rollup::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod rollup {
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
    #[doc = "Rollup was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"name\":\"oldRoot\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"newRoot\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"leavesHash\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"pathIndices\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"pathElements\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"leaves\",\"type\":\"uint256[]\",\"components\":[]}],\"type\":\"event\",\"name\":\"rollup\",\"outputs\":[],\"anonymous\":false}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ROLLUP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Rollup<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Rollup<M> {
        fn clone(&self) -> Self {
            Rollup(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Rollup<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Rollup<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Rollup))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Rollup<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ROLLUP_ABI.clone(), client).into()
        }
        #[doc = "Gets the contract's `rollup` event"]
        pub fn rollup_filter(&self) -> ethers::contract::builders::Event<M, RollupFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, RollupFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Rollup<M> {
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
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethevent(
        name = "rollup",
        abi = "rollup(uint256,uint256,uint256,uint256,uint256[],uint256[])"
    )]
    pub struct RollupFilter {
        pub old_root: ethers::core::types::U256,
        pub new_root: ethers::core::types::U256,
        pub leaves_hash: ethers::core::types::U256,
        pub path_indices: ethers::core::types::U256,
        pub path_elements: Vec<ethers::core::types::U256>,
        pub leaves: Vec<ethers::core::types::U256>,
    }
}
