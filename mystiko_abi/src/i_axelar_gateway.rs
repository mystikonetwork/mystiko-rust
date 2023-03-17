pub use i_axelar_gateway::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_axelar_gateway {
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
    #[doc = "IAxelarGateway was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AccountBlacklisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AccountWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"AllTokensFrozen\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"AllTokensUnfrozen\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationContractAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ContractCall\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commandId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"sourceChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"sourceAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"contractAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"sourceTxHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"sourceEventIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ContractCallApproved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commandId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"sourceChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"sourceAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"contractAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"sourceTxHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"sourceEventIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ContractCallApprovedWithMint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationContractAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ContractCallWithToken\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commandId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Executed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"tokenAddresses\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenFrozen\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenSent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenUnfrozen\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"adminEpoch\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"epoch\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"adminThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"epoch\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admins\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allTokensFrozen\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"contractAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"callContract\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"contractAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"callContractWithToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"input\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"freezeAllTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"freezeToken\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commandId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCommandExecuted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commandId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"contractAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isContractCallAndMintApproved\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commandId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"contractAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isContractCallApproved\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"destinationChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"destinationAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sendToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setup\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenAddresses\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenFrozen\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unfreezeAllTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unfreezeToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newImplementationCodeHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"setupParams\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commandId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"validateContractCall\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commandId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceChain\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"sourceAddress\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"payloadHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"validateContractCallAndMint\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IAXELARGATEWAY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IAxelarGateway<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IAxelarGateway<M> {
        fn clone(&self) -> Self {
            IAxelarGateway(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IAxelarGateway<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IAxelarGateway<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAxelarGateway))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IAxelarGateway<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IAXELARGATEWAY_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `adminEpoch` (0x364940d8) function"]
        pub fn admin_epoch(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([54, 73, 64, 216], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adminThreshold` (0x88b30587) function"]
        pub fn admin_threshold(
            &self,
            epoch: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([136, 179, 5, 135], epoch)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admins` (0x14bfd6d0) function"]
        pub fn admins(
            &self,
            epoch: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([20, 191, 214, 208], epoch)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allTokensFrozen` (0xaa1e1f0a) function"]
        pub fn all_tokens_frozen(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([170, 30, 31, 10], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `callContract` (0x1c92115f) function"]
        pub fn call_contract(
            &self,
            destination_chain: String,
            contract_address: String,
            payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [28, 146, 17, 95],
                    (destination_chain, contract_address, payload),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `callContractWithToken` (0xb5417084) function"]
        pub fn call_contract_with_token(
            &self,
            destination_chain: String,
            contract_address: String,
            payload: ethers::core::types::Bytes,
            symbol: String,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [181, 65, 112, 132],
                    (destination_chain, contract_address, payload, symbol, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execute` (0x09c5eabe) function"]
        pub fn execute(
            &self,
            input: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 197, 234, 190], input)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `freezeAllTokens` (0xd2bc37f8) function"]
        pub fn freeze_all_tokens(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 188, 55, 248], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `freezeToken` (0x646c5d34) function"]
        pub fn freeze_token(
            &self,
            symbol: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 108, 93, 52], symbol)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `implementation` (0x5c60da1b) function"]
        pub fn implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isCommandExecuted` (0xd26ff210) function"]
        pub fn is_command_executed(
            &self,
            command_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([210, 111, 242, 16], command_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isContractCallAndMintApproved` (0xbc00c216) function"]
        pub fn is_contract_call_and_mint_approved(
            &self,
            command_id: [u8; 32],
            source_chain: String,
            source_address: String,
            contract_address: ethers::core::types::Address,
            payload_hash: [u8; 32],
            symbol: String,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [188, 0, 194, 22],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        contract_address,
                        payload_hash,
                        symbol,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isContractCallApproved` (0xf6a5f9f5) function"]
        pub fn is_contract_call_approved(
            &self,
            command_id: [u8; 32],
            source_chain: String,
            source_address: String,
            contract_address: ethers::core::types::Address,
            payload_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [246, 165, 249, 245],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        contract_address,
                        payload_hash,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sendToken` (0x26ef699d) function"]
        pub fn send_token(
            &self,
            destination_chain: String,
            destination_address: String,
            symbol: String,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 239, 105, 157],
                    (destination_chain, destination_address, symbol, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setup` (0x9ded06df) function"]
        pub fn setup(
            &self,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 237, 6, 223], params)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenAddresses` (0x935b13f6) function"]
        pub fn token_addresses(
            &self,
            symbol: String,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([147, 91, 19, 246], symbol)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenFrozen` (0x7b1b769e) function"]
        pub fn token_frozen(
            &self,
            symbol: String,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([123, 27, 118, 158], symbol)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unfreezeAllTokens` (0xe3dfa299) function"]
        pub fn unfreeze_all_tokens(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 223, 162, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unfreezeToken` (0x34ff6983) function"]
        pub fn unfreeze_token(
            &self,
            symbol: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 255, 105, 131], symbol)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgrade` (0xa3499c73) function"]
        pub fn upgrade(
            &self,
            new_implementation: ethers::core::types::Address,
            new_implementation_code_hash: [u8; 32],
            setup_params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [163, 73, 156, 115],
                    (
                        new_implementation,
                        new_implementation_code_hash,
                        setup_params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `validateContractCall` (0x5f6970c3) function"]
        pub fn validate_contract_call(
            &self,
            command_id: [u8; 32],
            source_chain: String,
            source_address: String,
            payload_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [95, 105, 112, 195],
                    (command_id, source_chain, source_address, payload_hash),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `validateContractCallAndMint` (0x1876eed9) function"]
        pub fn validate_contract_call_and_mint(
            &self,
            command_id: [u8; 32],
            source_chain: String,
            source_address: String,
            payload_hash: [u8; 32],
            symbol: String,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [24, 118, 238, 217],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        payload_hash,
                        symbol,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AccountBlacklisted` event"]
        pub fn account_blacklisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AccountBlacklistedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AccountWhitelisted` event"]
        pub fn account_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AccountWhitelistedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AllTokensFrozen` event"]
        pub fn all_tokens_frozen_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AllTokensFrozenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AllTokensUnfrozen` event"]
        pub fn all_tokens_unfrozen_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AllTokensUnfrozenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ContractCall` event"]
        pub fn contract_call_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ContractCallFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ContractCallApproved` event"]
        pub fn contract_call_approved_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ContractCallApprovedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ContractCallApprovedWithMint` event"]
        pub fn contract_call_approved_with_mint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ContractCallApprovedWithMintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ContractCallWithToken` event"]
        pub fn contract_call_with_token_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ContractCallWithTokenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Executed` event"]
        pub fn executed_filter(&self) -> ethers::contract::builders::Event<M, ExecutedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenDeployed` event"]
        pub fn token_deployed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenDeployedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenFrozen` event"]
        pub fn token_frozen_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenFrozenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenSent` event"]
        pub fn token_sent_filter(&self) -> ethers::contract::builders::Event<M, TokenSentFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenUnfrozen` event"]
        pub fn token_unfrozen_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenUnfrozenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IAxelarGatewayEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IAxelarGateway<M> {
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
    #[ethevent(name = "AccountBlacklisted", abi = "AccountBlacklisted(address)")]
    pub struct AccountBlacklistedFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "AccountWhitelisted", abi = "AccountWhitelisted(address)")]
    pub struct AccountWhitelistedFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "AllTokensFrozen", abi = "AllTokensFrozen()")]
    pub struct AllTokensFrozenFilter();
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
    #[ethevent(name = "AllTokensUnfrozen", abi = "AllTokensUnfrozen()")]
    pub struct AllTokensUnfrozenFilter();
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
        name = "ContractCall",
        abi = "ContractCall(address,string,string,bytes32,bytes)"
    )]
    pub struct ContractCallFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_contract_address: String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub payload: ethers::core::types::Bytes,
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
        name = "ContractCallApproved",
        abi = "ContractCallApproved(bytes32,string,string,address,bytes32,bytes32,uint256)"
    )]
    pub struct ContractCallApprovedFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
        pub source_chain: String,
        pub source_address: String,
        #[ethevent(indexed)]
        pub contract_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub source_tx_hash: [u8; 32],
        pub source_event_index: ethers::core::types::U256,
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
        name = "ContractCallApprovedWithMint",
        abi = "ContractCallApprovedWithMint(bytes32,string,string,address,bytes32,string,uint256,bytes32,uint256)"
    )]
    pub struct ContractCallApprovedWithMintFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
        pub source_chain: String,
        pub source_address: String,
        #[ethevent(indexed)]
        pub contract_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub symbol: String,
        pub amount: ethers::core::types::U256,
        pub source_tx_hash: [u8; 32],
        pub source_event_index: ethers::core::types::U256,
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
        name = "ContractCallWithToken",
        abi = "ContractCallWithToken(address,string,string,bytes32,bytes,string,uint256)"
    )]
    pub struct ContractCallWithTokenFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_contract_address: String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub payload: ethers::core::types::Bytes,
        pub symbol: String,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "Executed", abi = "Executed(bytes32)")]
    pub struct ExecutedFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
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
    #[ethevent(name = "TokenDeployed", abi = "TokenDeployed(string,address)")]
    pub struct TokenDeployedFilter {
        pub symbol: String,
        pub token_addresses: ethers::core::types::Address,
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
    #[ethevent(name = "TokenFrozen", abi = "TokenFrozen(string)")]
    pub struct TokenFrozenFilter {
        pub symbol: String,
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
        name = "TokenSent",
        abi = "TokenSent(address,string,string,string,uint256)"
    )]
    pub struct TokenSentFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub destination_chain: String,
        pub destination_address: String,
        pub symbol: String,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "TokenUnfrozen", abi = "TokenUnfrozen(string)")]
    pub struct TokenUnfrozenFilter {
        pub symbol: String,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum IAxelarGatewayEvents {
        AccountBlacklistedFilter(AccountBlacklistedFilter),
        AccountWhitelistedFilter(AccountWhitelistedFilter),
        AllTokensFrozenFilter(AllTokensFrozenFilter),
        AllTokensUnfrozenFilter(AllTokensUnfrozenFilter),
        ContractCallFilter(ContractCallFilter),
        ContractCallApprovedFilter(ContractCallApprovedFilter),
        ContractCallApprovedWithMintFilter(ContractCallApprovedWithMintFilter),
        ContractCallWithTokenFilter(ContractCallWithTokenFilter),
        ExecutedFilter(ExecutedFilter),
        TokenDeployedFilter(TokenDeployedFilter),
        TokenFrozenFilter(TokenFrozenFilter),
        TokenSentFilter(TokenSentFilter),
        TokenUnfrozenFilter(TokenUnfrozenFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ethers::contract::EthLogDecode for IAxelarGatewayEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AccountBlacklistedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::AccountBlacklistedFilter(decoded));
            }
            if let Ok(decoded) = AccountWhitelistedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::AccountWhitelistedFilter(decoded));
            }
            if let Ok(decoded) = AllTokensFrozenFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::AllTokensFrozenFilter(decoded));
            }
            if let Ok(decoded) = AllTokensUnfrozenFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::AllTokensUnfrozenFilter(decoded));
            }
            if let Ok(decoded) = ContractCallFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::ContractCallFilter(decoded));
            }
            if let Ok(decoded) = ContractCallApprovedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::ContractCallApprovedFilter(decoded));
            }
            if let Ok(decoded) = ContractCallApprovedWithMintFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::ContractCallApprovedWithMintFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ContractCallWithTokenFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::ContractCallWithTokenFilter(decoded));
            }
            if let Ok(decoded) = ExecutedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::ExecutedFilter(decoded));
            }
            if let Ok(decoded) = TokenDeployedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::TokenDeployedFilter(decoded));
            }
            if let Ok(decoded) = TokenFrozenFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::TokenFrozenFilter(decoded));
            }
            if let Ok(decoded) = TokenSentFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::TokenSentFilter(decoded));
            }
            if let Ok(decoded) = TokenUnfrozenFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::TokenUnfrozenFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(IAxelarGatewayEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IAxelarGatewayEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAxelarGatewayEvents::AccountBlacklistedFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::AccountWhitelistedFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::AllTokensFrozenFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::AllTokensUnfrozenFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::ContractCallFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::ContractCallApprovedFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::ContractCallApprovedWithMintFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::ContractCallWithTokenFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::ExecutedFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::TokenDeployedFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::TokenFrozenFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::TokenSentFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::TokenUnfrozenFilter(element) => element.fmt(f),
                IAxelarGatewayEvents::UpgradedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `adminEpoch` function with signature `adminEpoch()` and selector `[54, 73, 64, 216]`"]
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
    #[ethcall(name = "adminEpoch", abi = "adminEpoch()")]
    pub struct AdminEpochCall;
    #[doc = "Container type for all input parameters for the `adminThreshold` function with signature `adminThreshold(uint256)` and selector `[136, 179, 5, 135]`"]
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
    #[ethcall(name = "adminThreshold", abi = "adminThreshold(uint256)")]
    pub struct AdminThresholdCall {
        pub epoch: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `admins` function with signature `admins(uint256)` and selector `[20, 191, 214, 208]`"]
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
    #[ethcall(name = "admins", abi = "admins(uint256)")]
    pub struct AdminsCall {
        pub epoch: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `allTokensFrozen` function with signature `allTokensFrozen()` and selector `[170, 30, 31, 10]`"]
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
    #[ethcall(name = "allTokensFrozen", abi = "allTokensFrozen()")]
    pub struct AllTokensFrozenCall;
    #[doc = "Container type for all input parameters for the `callContract` function with signature `callContract(string,string,bytes)` and selector `[28, 146, 17, 95]`"]
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
    #[ethcall(name = "callContract", abi = "callContract(string,string,bytes)")]
    pub struct CallContractCall {
        pub destination_chain: String,
        pub contract_address: String,
        pub payload: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `callContractWithToken` function with signature `callContractWithToken(string,string,bytes,string,uint256)` and selector `[181, 65, 112, 132]`"]
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
        name = "callContractWithToken",
        abi = "callContractWithToken(string,string,bytes,string,uint256)"
    )]
    pub struct CallContractWithTokenCall {
        pub destination_chain: String,
        pub contract_address: String,
        pub payload: ethers::core::types::Bytes,
        pub symbol: String,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `execute` function with signature `execute(bytes)` and selector `[9, 197, 234, 190]`"]
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
    #[ethcall(name = "execute", abi = "execute(bytes)")]
    pub struct ExecuteCall {
        pub input: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `freezeAllTokens` function with signature `freezeAllTokens()` and selector `[210, 188, 55, 248]`"]
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
    #[ethcall(name = "freezeAllTokens", abi = "freezeAllTokens()")]
    pub struct FreezeAllTokensCall;
    #[doc = "Container type for all input parameters for the `freezeToken` function with signature `freezeToken(string)` and selector `[100, 108, 93, 52]`"]
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
    #[ethcall(name = "freezeToken", abi = "freezeToken(string)")]
    pub struct FreezeTokenCall {
        pub symbol: String,
    }
    #[doc = "Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
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
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    #[doc = "Container type for all input parameters for the `isCommandExecuted` function with signature `isCommandExecuted(bytes32)` and selector `[210, 111, 242, 16]`"]
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
    #[ethcall(name = "isCommandExecuted", abi = "isCommandExecuted(bytes32)")]
    pub struct IsCommandExecutedCall {
        pub command_id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isContractCallAndMintApproved` function with signature `isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)` and selector `[188, 0, 194, 22]`"]
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
        name = "isContractCallAndMintApproved",
        abi = "isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)"
    )]
    pub struct IsContractCallAndMintApprovedCall {
        pub command_id: [u8; 32],
        pub source_chain: String,
        pub source_address: String,
        pub contract_address: ethers::core::types::Address,
        pub payload_hash: [u8; 32],
        pub symbol: String,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isContractCallApproved` function with signature `isContractCallApproved(bytes32,string,string,address,bytes32)` and selector `[246, 165, 249, 245]`"]
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
        name = "isContractCallApproved",
        abi = "isContractCallApproved(bytes32,string,string,address,bytes32)"
    )]
    pub struct IsContractCallApprovedCall {
        pub command_id: [u8; 32],
        pub source_chain: String,
        pub source_address: String,
        pub contract_address: ethers::core::types::Address,
        pub payload_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `sendToken` function with signature `sendToken(string,string,string,uint256)` and selector `[38, 239, 105, 157]`"]
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
    #[ethcall(name = "sendToken", abi = "sendToken(string,string,string,uint256)")]
    pub struct SendTokenCall {
        pub destination_chain: String,
        pub destination_address: String,
        pub symbol: String,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setup` function with signature `setup(bytes)` and selector `[157, 237, 6, 223]`"]
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
    #[ethcall(name = "setup", abi = "setup(bytes)")]
    pub struct SetupCall {
        pub params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `tokenAddresses` function with signature `tokenAddresses(string)` and selector `[147, 91, 19, 246]`"]
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
    #[ethcall(name = "tokenAddresses", abi = "tokenAddresses(string)")]
    pub struct TokenAddressesCall {
        pub symbol: String,
    }
    #[doc = "Container type for all input parameters for the `tokenFrozen` function with signature `tokenFrozen(string)` and selector `[123, 27, 118, 158]`"]
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
    #[ethcall(name = "tokenFrozen", abi = "tokenFrozen(string)")]
    pub struct TokenFrozenCall {
        pub symbol: String,
    }
    #[doc = "Container type for all input parameters for the `unfreezeAllTokens` function with signature `unfreezeAllTokens()` and selector `[227, 223, 162, 153]`"]
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
    #[ethcall(name = "unfreezeAllTokens", abi = "unfreezeAllTokens()")]
    pub struct UnfreezeAllTokensCall;
    #[doc = "Container type for all input parameters for the `unfreezeToken` function with signature `unfreezeToken(string)` and selector `[52, 255, 105, 131]`"]
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
    #[ethcall(name = "unfreezeToken", abi = "unfreezeToken(string)")]
    pub struct UnfreezeTokenCall {
        pub symbol: String,
    }
    #[doc = "Container type for all input parameters for the `upgrade` function with signature `upgrade(address,bytes32,bytes)` and selector `[163, 73, 156, 115]`"]
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
    #[ethcall(name = "upgrade", abi = "upgrade(address,bytes32,bytes)")]
    pub struct UpgradeCall {
        pub new_implementation: ethers::core::types::Address,
        pub new_implementation_code_hash: [u8; 32],
        pub setup_params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `validateContractCall` function with signature `validateContractCall(bytes32,string,string,bytes32)` and selector `[95, 105, 112, 195]`"]
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
        name = "validateContractCall",
        abi = "validateContractCall(bytes32,string,string,bytes32)"
    )]
    pub struct ValidateContractCallCall {
        pub command_id: [u8; 32],
        pub source_chain: String,
        pub source_address: String,
        pub payload_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `validateContractCallAndMint` function with signature `validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)` and selector `[24, 118, 238, 217]`"]
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
        name = "validateContractCallAndMint",
        abi = "validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)"
    )]
    pub struct ValidateContractCallAndMintCall {
        pub command_id: [u8; 32],
        pub source_chain: String,
        pub source_address: String,
        pub payload_hash: [u8; 32],
        pub symbol: String,
        pub amount: ethers::core::types::U256,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum IAxelarGatewayCalls {
        AdminEpoch(AdminEpochCall),
        AdminThreshold(AdminThresholdCall),
        Admins(AdminsCall),
        AllTokensFrozen(AllTokensFrozenCall),
        CallContract(CallContractCall),
        CallContractWithToken(CallContractWithTokenCall),
        Execute(ExecuteCall),
        FreezeAllTokens(FreezeAllTokensCall),
        FreezeToken(FreezeTokenCall),
        Implementation(ImplementationCall),
        IsCommandExecuted(IsCommandExecutedCall),
        IsContractCallAndMintApproved(IsContractCallAndMintApprovedCall),
        IsContractCallApproved(IsContractCallApprovedCall),
        SendToken(SendTokenCall),
        Setup(SetupCall),
        TokenAddresses(TokenAddressesCall),
        TokenFrozen(TokenFrozenCall),
        UnfreezeAllTokens(UnfreezeAllTokensCall),
        UnfreezeToken(UnfreezeTokenCall),
        Upgrade(UpgradeCall),
        ValidateContractCall(ValidateContractCallCall),
        ValidateContractCallAndMint(ValidateContractCallAndMintCall),
    }
    impl ethers::core::abi::AbiDecode for IAxelarGatewayCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AdminEpochCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::AdminEpoch(decoded));
            }
            if let Ok(decoded) =
                <AdminThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::AdminThreshold(decoded));
            }
            if let Ok(decoded) = <AdminsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::Admins(decoded));
            }
            if let Ok(decoded) =
                <AllTokensFrozenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::AllTokensFrozen(decoded));
            }
            if let Ok(decoded) =
                <CallContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::CallContract(decoded));
            }
            if let Ok(decoded) =
                <CallContractWithTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::CallContractWithToken(decoded));
            }
            if let Ok(decoded) =
                <ExecuteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::Execute(decoded));
            }
            if let Ok(decoded) =
                <FreezeAllTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::FreezeAllTokens(decoded));
            }
            if let Ok(decoded) =
                <FreezeTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::FreezeToken(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <IsCommandExecutedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::IsCommandExecuted(decoded));
            }
            if let Ok(decoded) =
                <IsContractCallAndMintApprovedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAxelarGatewayCalls::IsContractCallAndMintApproved(decoded));
            }
            if let Ok(decoded) =
                <IsContractCallApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::IsContractCallApproved(decoded));
            }
            if let Ok(decoded) =
                <SendTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::SendToken(decoded));
            }
            if let Ok(decoded) = <SetupCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::Setup(decoded));
            }
            if let Ok(decoded) =
                <TokenAddressesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::TokenAddresses(decoded));
            }
            if let Ok(decoded) =
                <TokenFrozenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::TokenFrozen(decoded));
            }
            if let Ok(decoded) =
                <UnfreezeAllTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::UnfreezeAllTokens(decoded));
            }
            if let Ok(decoded) =
                <UnfreezeTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::UnfreezeToken(decoded));
            }
            if let Ok(decoded) =
                <UpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::Upgrade(decoded));
            }
            if let Ok(decoded) =
                <ValidateContractCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAxelarGatewayCalls::ValidateContractCall(decoded));
            }
            if let Ok(decoded) =
                <ValidateContractCallAndMintCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAxelarGatewayCalls::ValidateContractCallAndMint(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAxelarGatewayCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAxelarGatewayCalls::AdminEpoch(element) => element.encode(),
                IAxelarGatewayCalls::AdminThreshold(element) => element.encode(),
                IAxelarGatewayCalls::Admins(element) => element.encode(),
                IAxelarGatewayCalls::AllTokensFrozen(element) => element.encode(),
                IAxelarGatewayCalls::CallContract(element) => element.encode(),
                IAxelarGatewayCalls::CallContractWithToken(element) => element.encode(),
                IAxelarGatewayCalls::Execute(element) => element.encode(),
                IAxelarGatewayCalls::FreezeAllTokens(element) => element.encode(),
                IAxelarGatewayCalls::FreezeToken(element) => element.encode(),
                IAxelarGatewayCalls::Implementation(element) => element.encode(),
                IAxelarGatewayCalls::IsCommandExecuted(element) => element.encode(),
                IAxelarGatewayCalls::IsContractCallAndMintApproved(element) => element.encode(),
                IAxelarGatewayCalls::IsContractCallApproved(element) => element.encode(),
                IAxelarGatewayCalls::SendToken(element) => element.encode(),
                IAxelarGatewayCalls::Setup(element) => element.encode(),
                IAxelarGatewayCalls::TokenAddresses(element) => element.encode(),
                IAxelarGatewayCalls::TokenFrozen(element) => element.encode(),
                IAxelarGatewayCalls::UnfreezeAllTokens(element) => element.encode(),
                IAxelarGatewayCalls::UnfreezeToken(element) => element.encode(),
                IAxelarGatewayCalls::Upgrade(element) => element.encode(),
                IAxelarGatewayCalls::ValidateContractCall(element) => element.encode(),
                IAxelarGatewayCalls::ValidateContractCallAndMint(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAxelarGatewayCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAxelarGatewayCalls::AdminEpoch(element) => element.fmt(f),
                IAxelarGatewayCalls::AdminThreshold(element) => element.fmt(f),
                IAxelarGatewayCalls::Admins(element) => element.fmt(f),
                IAxelarGatewayCalls::AllTokensFrozen(element) => element.fmt(f),
                IAxelarGatewayCalls::CallContract(element) => element.fmt(f),
                IAxelarGatewayCalls::CallContractWithToken(element) => element.fmt(f),
                IAxelarGatewayCalls::Execute(element) => element.fmt(f),
                IAxelarGatewayCalls::FreezeAllTokens(element) => element.fmt(f),
                IAxelarGatewayCalls::FreezeToken(element) => element.fmt(f),
                IAxelarGatewayCalls::Implementation(element) => element.fmt(f),
                IAxelarGatewayCalls::IsCommandExecuted(element) => element.fmt(f),
                IAxelarGatewayCalls::IsContractCallAndMintApproved(element) => element.fmt(f),
                IAxelarGatewayCalls::IsContractCallApproved(element) => element.fmt(f),
                IAxelarGatewayCalls::SendToken(element) => element.fmt(f),
                IAxelarGatewayCalls::Setup(element) => element.fmt(f),
                IAxelarGatewayCalls::TokenAddresses(element) => element.fmt(f),
                IAxelarGatewayCalls::TokenFrozen(element) => element.fmt(f),
                IAxelarGatewayCalls::UnfreezeAllTokens(element) => element.fmt(f),
                IAxelarGatewayCalls::UnfreezeToken(element) => element.fmt(f),
                IAxelarGatewayCalls::Upgrade(element) => element.fmt(f),
                IAxelarGatewayCalls::ValidateContractCall(element) => element.fmt(f),
                IAxelarGatewayCalls::ValidateContractCallAndMint(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AdminEpochCall> for IAxelarGatewayCalls {
        fn from(var: AdminEpochCall) -> Self {
            IAxelarGatewayCalls::AdminEpoch(var)
        }
    }
    impl ::std::convert::From<AdminThresholdCall> for IAxelarGatewayCalls {
        fn from(var: AdminThresholdCall) -> Self {
            IAxelarGatewayCalls::AdminThreshold(var)
        }
    }
    impl ::std::convert::From<AdminsCall> for IAxelarGatewayCalls {
        fn from(var: AdminsCall) -> Self {
            IAxelarGatewayCalls::Admins(var)
        }
    }
    impl ::std::convert::From<AllTokensFrozenCall> for IAxelarGatewayCalls {
        fn from(var: AllTokensFrozenCall) -> Self {
            IAxelarGatewayCalls::AllTokensFrozen(var)
        }
    }
    impl ::std::convert::From<CallContractCall> for IAxelarGatewayCalls {
        fn from(var: CallContractCall) -> Self {
            IAxelarGatewayCalls::CallContract(var)
        }
    }
    impl ::std::convert::From<CallContractWithTokenCall> for IAxelarGatewayCalls {
        fn from(var: CallContractWithTokenCall) -> Self {
            IAxelarGatewayCalls::CallContractWithToken(var)
        }
    }
    impl ::std::convert::From<ExecuteCall> for IAxelarGatewayCalls {
        fn from(var: ExecuteCall) -> Self {
            IAxelarGatewayCalls::Execute(var)
        }
    }
    impl ::std::convert::From<FreezeAllTokensCall> for IAxelarGatewayCalls {
        fn from(var: FreezeAllTokensCall) -> Self {
            IAxelarGatewayCalls::FreezeAllTokens(var)
        }
    }
    impl ::std::convert::From<FreezeTokenCall> for IAxelarGatewayCalls {
        fn from(var: FreezeTokenCall) -> Self {
            IAxelarGatewayCalls::FreezeToken(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for IAxelarGatewayCalls {
        fn from(var: ImplementationCall) -> Self {
            IAxelarGatewayCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<IsCommandExecutedCall> for IAxelarGatewayCalls {
        fn from(var: IsCommandExecutedCall) -> Self {
            IAxelarGatewayCalls::IsCommandExecuted(var)
        }
    }
    impl ::std::convert::From<IsContractCallAndMintApprovedCall> for IAxelarGatewayCalls {
        fn from(var: IsContractCallAndMintApprovedCall) -> Self {
            IAxelarGatewayCalls::IsContractCallAndMintApproved(var)
        }
    }
    impl ::std::convert::From<IsContractCallApprovedCall> for IAxelarGatewayCalls {
        fn from(var: IsContractCallApprovedCall) -> Self {
            IAxelarGatewayCalls::IsContractCallApproved(var)
        }
    }
    impl ::std::convert::From<SendTokenCall> for IAxelarGatewayCalls {
        fn from(var: SendTokenCall) -> Self {
            IAxelarGatewayCalls::SendToken(var)
        }
    }
    impl ::std::convert::From<SetupCall> for IAxelarGatewayCalls {
        fn from(var: SetupCall) -> Self {
            IAxelarGatewayCalls::Setup(var)
        }
    }
    impl ::std::convert::From<TokenAddressesCall> for IAxelarGatewayCalls {
        fn from(var: TokenAddressesCall) -> Self {
            IAxelarGatewayCalls::TokenAddresses(var)
        }
    }
    impl ::std::convert::From<TokenFrozenCall> for IAxelarGatewayCalls {
        fn from(var: TokenFrozenCall) -> Self {
            IAxelarGatewayCalls::TokenFrozen(var)
        }
    }
    impl ::std::convert::From<UnfreezeAllTokensCall> for IAxelarGatewayCalls {
        fn from(var: UnfreezeAllTokensCall) -> Self {
            IAxelarGatewayCalls::UnfreezeAllTokens(var)
        }
    }
    impl ::std::convert::From<UnfreezeTokenCall> for IAxelarGatewayCalls {
        fn from(var: UnfreezeTokenCall) -> Self {
            IAxelarGatewayCalls::UnfreezeToken(var)
        }
    }
    impl ::std::convert::From<UpgradeCall> for IAxelarGatewayCalls {
        fn from(var: UpgradeCall) -> Self {
            IAxelarGatewayCalls::Upgrade(var)
        }
    }
    impl ::std::convert::From<ValidateContractCallCall> for IAxelarGatewayCalls {
        fn from(var: ValidateContractCallCall) -> Self {
            IAxelarGatewayCalls::ValidateContractCall(var)
        }
    }
    impl ::std::convert::From<ValidateContractCallAndMintCall> for IAxelarGatewayCalls {
        fn from(var: ValidateContractCallAndMintCall) -> Self {
            IAxelarGatewayCalls::ValidateContractCallAndMint(var)
        }
    }
    #[doc = "Container type for all return fields from the `adminEpoch` function with signature `adminEpoch()` and selector `[54, 73, 64, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct AdminEpochReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `adminThreshold` function with signature `adminThreshold(uint256)` and selector `[136, 179, 5, 135]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct AdminThresholdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `admins` function with signature `admins(uint256)` and selector `[20, 191, 214, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct AdminsReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `allTokensFrozen` function with signature `allTokensFrozen()` and selector `[170, 30, 31, 10]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct AllTokensFrozenReturn(pub bool);
    #[doc = "Container type for all return fields from the `implementation` function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct ImplementationReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isCommandExecuted` function with signature `isCommandExecuted(bytes32)` and selector `[210, 111, 242, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct IsCommandExecutedReturn(pub bool);
    #[doc = "Container type for all return fields from the `isContractCallAndMintApproved` function with signature `isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)` and selector `[188, 0, 194, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct IsContractCallAndMintApprovedReturn(pub bool);
    #[doc = "Container type for all return fields from the `isContractCallApproved` function with signature `isContractCallApproved(bytes32,string,string,address,bytes32)` and selector `[246, 165, 249, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct IsContractCallApprovedReturn(pub bool);
    #[doc = "Container type for all return fields from the `tokenAddresses` function with signature `tokenAddresses(string)` and selector `[147, 91, 19, 246]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct TokenAddressesReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `tokenFrozen` function with signature `tokenFrozen(string)` and selector `[123, 27, 118, 158]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct TokenFrozenReturn(pub bool);
    #[doc = "Container type for all return fields from the `validateContractCall` function with signature `validateContractCall(bytes32,string,string,bytes32)` and selector `[95, 105, 112, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct ValidateContractCallReturn(pub bool);
    #[doc = "Container type for all return fields from the `validateContractCallAndMint` function with signature `validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)` and selector `[24, 118, 238, 217]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    pub struct ValidateContractCallAndMintReturn(pub bool);
}
