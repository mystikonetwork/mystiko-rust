pub use transact::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod transact {
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
    #[doc = "Transact was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"serialNumbers\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"sigHashes\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"sigPublicKey\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"publicAmount\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"relayerFeeAmount\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"outCommitments\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"rollupFeeAmounts\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"randomPublicKeyXSign\",\"type\":\"bool\",\"components\":[]},{\"name\":\"randomPublicKeyY\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"auditorPublicKeyXSigns\",\"type\":\"bool[5]\",\"components\":[]},{\"name\":\"auditorPublicKeyYs\",\"type\":\"uint256[5]\",\"components\":[]},{\"name\":\"encryptedCommitmentShares\",\"type\":\"uint256[][]\",\"components\":[]},{\"name\":\"inCommitments\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"inAmount\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"inRandomP\",\"type\":\"uint128[]\",\"components\":[]},{\"name\":\"inRandomR\",\"type\":\"uint128[]\",\"components\":[]},{\"name\":\"inRandomS\",\"type\":\"uint128[]\",\"components\":[]},{\"name\":\"inSecretKey\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"inPublicKey\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"inPathElements\",\"type\":\"uint256[][]\",\"components\":[]},{\"name\":\"inPathIndices\",\"type\":\"bool[][]\",\"components\":[]},{\"name\":\"outAmount\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"outRandomP\",\"type\":\"uint128[]\",\"components\":[]},{\"name\":\"outRandomR\",\"type\":\"uint128[]\",\"components\":[]},{\"name\":\"outRandomS\",\"type\":\"uint128[]\",\"components\":[]},{\"name\":\"outPublicKey\",\"type\":\"uint256[]\",\"components\":[]},{\"name\":\"randomPublicKeyX\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"auditorPublicKeyXs\",\"type\":\"uint256[5]\",\"components\":[]},{\"name\":\"randomSecretKey\",\"type\":\"uint256\",\"components\":[]},{\"name\":\"coefficients\",\"type\":\"uint256[][3]\",\"components\":[]},{\"name\":\"commitmentShares\",\"type\":\"uint256[][5]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transact\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static TRANSACT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Transact<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Transact<M> {
        fn clone(&self) -> Self {
            Transact(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Transact<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Transact<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Transact))
                .field(&self.address())
                .finish()
        }
    }
    // impl<M: ethers::providers::Middleware> Transact<M> {
    //     #[doc = r" Creates a new contract instance with the specified `ethers`"]
    //     #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
    //     #[doc = r" object"]
    //     pub fn new<T: Into<ethers::core::types::Address>>(
    //         address: T,
    //         client: ::std::sync::Arc<M>,
    //     ) -> Self {
    //         ethers::contract::Contract::new(address.into(), TRANSACT_ABI.clone(), client).into()
    //     }
    //     #[doc = "Calls the contract's `transact` (0xaf5f62da) function"]
    //     pub fn transact(
    //         &self,
    //         root: ethers::core::types::U256,
    //         serial_numbers: ::std::vec::Vec<ethers::core::types::U256>,
    //         sig_hashes: ::std::vec::Vec<ethers::core::types::U256>,
    //         sig_public_key: ethers::core::types::U256,
    //         public_amount: ethers::core::types::U256,
    //         relayer_fee_amount: ethers::core::types::U256,
    //         out_commitments: ::std::vec::Vec<ethers::core::types::U256>,
    //         rollup_fee_amounts: ::std::vec::Vec<ethers::core::types::U256>,
    //         random_public_key_x_sign: bool,
    //         random_public_key_y: ethers::core::types::U256,
    //         auditor_public_key_x_signs: [bool; 5usize],
    //         auditor_public_key_ys: [ethers::core::types::U256; 5usize],
    //         encrypted_commitment_shares: ::std::vec::Vec<
    //             ::std::vec::Vec<ethers::core::types::U256>,
    //         >,
    //         in_commitments: ::std::vec::Vec<ethers::core::types::U256>,
    //         in_amount: ::std::vec::Vec<ethers::core::types::U256>,
    //         in_random_p: ::std::vec::Vec<u128>,
    //         in_random_r: ::std::vec::Vec<u128>,
    //         in_random_s: ::std::vec::Vec<u128>,
    //         in_secret_key: ::std::vec::Vec<ethers::core::types::U256>,
    //         in_public_key: ::std::vec::Vec<ethers::core::types::U256>,
    //         in_path_elements: ::std::vec::Vec<::std::vec::Vec<ethers::core::types::U256>>,
    //         in_path_indices: ::std::vec::Vec<::std::vec::Vec<bool>>,
    //         out_amount: ::std::vec::Vec<ethers::core::types::U256>,
    //         out_random_p: ::std::vec::Vec<u128>,
    //         out_random_r: ::std::vec::Vec<u128>,
    //         out_random_s: ::std::vec::Vec<u128>,
    //         out_public_key: ::std::vec::Vec<ethers::core::types::U256>,
    //         random_public_key_x: ethers::core::types::U256,
    //         auditor_public_key_xs: [ethers::core::types::U256; 5usize],
    //         random_secret_key: ethers::core::types::U256,
    //         coefficients: [::std::vec::Vec<ethers::core::types::U256>; 3usize],
    //         commitment_shares: [::std::vec::Vec<ethers::core::types::U256>; 5usize],
    //     ) -> ethers::contract::builders::ContractCall<M, ()> {
    //         self.0
    //             .method_hash(
    //                 [175, 95, 98, 218],
    //                 (
    //                     root,
    //                     serial_numbers,
    //                     sig_hashes,
    //                     sig_public_key,
    //                     public_amount,
    //                     relayer_fee_amount,
    //                     out_commitments,
    //                     rollup_fee_amounts,
    //                     random_public_key_x_sign,
    //                     random_public_key_y,
    //                     auditor_public_key_x_signs,
    //                     auditor_public_key_ys,
    //                     encrypted_commitment_shares,
    //                     in_commitments,
    //                     in_amount,
    //                     in_random_p,
    //                     in_random_r,
    //                     in_random_s,
    //                     in_secret_key,
    //                     in_public_key,
    //                     in_path_elements,
    //                     in_path_indices,
    //                     out_amount,
    //                     out_random_p,
    //                     out_random_r,
    //                     out_random_s,
    //                     out_public_key,
    //                     random_public_key_x,
    //                     auditor_public_key_xs,
    //                     random_secret_key,
    //                     coefficients,
    //                     commitment_shares,
    //                 ),
    //             )
    //             .expect("method not found (this should never happen)")
    //     }
    // }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Transact<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `transact` function with signature `transact(uint256,uint256[],uint256[],uint256,uint256,uint256,uint256[],uint256[],bool,uint256,bool[5],uint256[5],uint256[][],uint256[],uint256[],uint128[],uint128[],uint128[],uint256[],uint256[],uint256[][],bool[][],uint256[],uint128[],uint128[],uint128[],uint256[],uint256,uint256[5],uint256,uint256[][3],uint256[][5])` and selector `[175, 95, 98, 218]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(
        name = "transact",
        abi = "transact(uint256,uint256[],uint256[],uint256,uint256,uint256,uint256[],uint256[],bool,uint256,bool[5],uint256[5],uint256[][],uint256[],uint256[],uint128[],uint128[],uint128[],uint256[],uint256[],uint256[][],bool[][],uint256[],uint128[],uint128[],uint128[],uint256[],uint256,uint256[5],uint256,uint256[][3],uint256[][5])"
    )]
    pub struct TransactCall {
        pub root: ethers::core::types::U256,
        pub serial_numbers: ::std::vec::Vec<ethers::core::types::U256>,
        pub sig_hashes: ::std::vec::Vec<ethers::core::types::U256>,
        pub sig_public_key: ethers::core::types::U256,
        pub public_amount: ethers::core::types::U256,
        pub relayer_fee_amount: ethers::core::types::U256,
        pub out_commitments: ::std::vec::Vec<ethers::core::types::U256>,
        pub rollup_fee_amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub random_public_key_x_sign: bool,
        pub random_public_key_y: ethers::core::types::U256,
        pub auditor_public_key_x_signs: [bool; 5usize],
        pub auditor_public_key_ys: [ethers::core::types::U256; 5usize],
        pub encrypted_commitment_shares: ::std::vec::Vec<[ethers::core::types::U256; 5usize]>,
        pub in_commitments: ::std::vec::Vec<ethers::core::types::U256>,
        pub in_amount: ::std::vec::Vec<ethers::core::types::U256>,
        pub in_random_p: ::std::vec::Vec<ethers::core::types::U128>,
        pub in_random_r: ::std::vec::Vec<ethers::core::types::U128>,
        pub in_random_s: ::std::vec::Vec<ethers::core::types::U128>,
        pub in_secret_key: ::std::vec::Vec<ethers::core::types::U256>,
        pub in_public_key: ::std::vec::Vec<ethers::core::types::U256>,
        pub in_path_elements: ::std::vec::Vec<::std::vec::Vec<ethers::core::types::U256>>,
        pub in_path_indices: ::std::vec::Vec<::std::vec::Vec<bool>>,
        pub out_amount: ::std::vec::Vec<ethers::core::types::U256>,
        pub out_random_p: ::std::vec::Vec<ethers::core::types::U128>,
        pub out_random_r: ::std::vec::Vec<ethers::core::types::U128>,
        pub out_random_s: ::std::vec::Vec<ethers::core::types::U128>,
        pub out_public_key: ::std::vec::Vec<ethers::core::types::U256>,
        pub random_public_key_x: ethers::core::types::U256,
        pub auditor_public_key_xs: [ethers::core::types::U256; 5usize],
        pub random_secret_key: ethers::core::types::U256,
        pub coefficients: ::std::vec::Vec<[ethers::core::types::U256; 3usize]>,
        pub commitment_shares: ::std::vec::Vec<[ethers::core::types::U256; 5usize]>,
    }
}
