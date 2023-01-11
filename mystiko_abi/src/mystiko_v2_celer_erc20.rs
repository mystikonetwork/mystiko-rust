pub use mystiko_v2_celer_erc20::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mystiko_v2_celer_erc20 {
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
    #[doc = "MystikoV2CelerERC20 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"contract IHasher3\",\"name\":\"_hasher3\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20Metadata\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AmountLessThanZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AmountTooLarge\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AmountTooSmall\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BridgeFeeTooFew\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CommitmentHashIncorrect\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"DepositsDisabled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ExecutorFeeTooFew\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FromChainIdNotMatched\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FromProxyAddressNotMatched\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"HashKGreaterThanFieldSize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"param\",\"type\":\"string\",\"components\":[]}],\"type\":\"error\",\"name\":\"Invalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MaxAmountLessThanMinAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MinAmountGreaterThanMaxAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotChanged\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOperator\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RandomSGreaterThanFieldSize\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RollupFeeToFew\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SanctionedAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SenderIsNotBridgeProxy\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CommitmentCrossChain\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DepositsDisabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"maxAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MaxAmount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"minAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinAmount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"minBridgeFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinBridgeFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"minExecutorFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinExecutorFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OperatorChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"peerMinExecutorFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerMinExecutorFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"peerMinRollupFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerMinRollupFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SanctionsCheck\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"sanctions\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SanctionsList\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetDecimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetName\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetSymbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"assetType\",\"outputs\":[{\"internalType\":\"enum AssetPool.AssetType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bridgeProxyAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"bridgeType\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newOperator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeOperator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IMystikoBridge.DepositRequest\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"hashK\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"randomS\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"encryptedNote\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bridgeFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"executorFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rollupFee\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disableSanctionsCheck\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableSanctionsCheck\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_srcChainId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_message\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_executor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"executeMessage\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssociatedCommitmentPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMaxAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinBridgeFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinExecutorFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPeerMinExecutorFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPeerMinRollupFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isDepositsDisabled\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerChainId\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerChainName\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sanctionsCheck\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sanctionsList\",\"outputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_commitmentPoolAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAssociatedCommitmentPool\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_bridgeProxyAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBridgeProxyAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDepositsDisabled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_maxAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMaxAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minBridgeFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinBridgeFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minExecutorFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinExecutorFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"_peerChainId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_peerChainName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_peerContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPeerContract\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_peerMinExecutorFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPeerMinExecutorFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_peerMinRollupFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPeerMinRollupFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"_sanction\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateSanctionsListAddress\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MYSTIKOV2CELERERC20_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct MystikoV2CelerERC20<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MystikoV2CelerERC20<M> {
        fn clone(&self) -> Self {
            MystikoV2CelerERC20(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MystikoV2CelerERC20<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MystikoV2CelerERC20<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MystikoV2CelerERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MystikoV2CelerERC20<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MYSTIKOV2CELERERC20_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `assetDecimals` (0xc2d41601) function"]
        pub fn asset_decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([194, 212, 22, 1], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetName` (0xc9230c5d) function"]
        pub fn asset_name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([201, 35, 12, 93], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetSymbol` (0x176de7a8) function"]
        pub fn asset_symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([23, 109, 231, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetType` (0x3fe3347a) function"]
        pub fn asset_type(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([63, 227, 52, 122], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bridgeProxyAddress` (0x2cd26d45) function"]
        pub fn bridge_proxy_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([44, 210, 109, 69], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bridgeType` (0x2421e155) function"]
        pub fn bridge_type(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([36, 33, 225, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeOperator` (0x06394c9b) function"]
        pub fn change_operator(
            &self,
            new_operator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 57, 76, 155], new_operator)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x9a03636c) function"]
        pub fn deposit(
            &self,
            request: DepositRequest,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 3, 99, 108], (request,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disableSanctionsCheck` (0xdd757c34) function"]
        pub fn disable_sanctions_check(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 117, 124, 52], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enableSanctionsCheck` (0x01dbf19f) function"]
        pub fn enable_sanctions_check(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 219, 241, 159], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeMessage` (0x9c649fdf) function"]
        pub fn execute_message(
            &self,
            sender: ethers::core::types::Address,
            src_chain_id: u64,
            message: ethers::core::types::Bytes,
            executor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [156, 100, 159, 223],
                    (sender, src_chain_id, message, executor),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssociatedCommitmentPool` (0xddac5dc1) function"]
        pub fn get_associated_commitment_pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([221, 172, 93, 193], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMaxAmount` (0x0ba95909) function"]
        pub fn get_max_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([11, 169, 89, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinAmount` (0xcfc7e2da) function"]
        pub fn get_min_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([207, 199, 226, 218], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinBridgeFee` (0xefbfb2ae) function"]
        pub fn get_min_bridge_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([239, 191, 178, 174], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinExecutorFee` (0xf4ad17c6) function"]
        pub fn get_min_executor_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([244, 173, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPeerMinExecutorFee` (0x5898a0a8) function"]
        pub fn get_peer_min_executor_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([88, 152, 160, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPeerMinRollupFee` (0x825b5f8d) function"]
        pub fn get_peer_min_rollup_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([130, 91, 95, 141], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isDepositsDisabled` (0xed6ea33a) function"]
        pub fn is_deposits_disabled(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([237, 110, 163, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `peerChainId` (0xcdfceeba) function"]
        pub fn peer_chain_id(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([205, 252, 238, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `peerChainName` (0x4e3c10b7) function"]
        pub fn peer_chain_name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([78, 60, 16, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `peerContract` (0x21e32d55) function"]
        pub fn peer_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([33, 227, 45, 85], ())
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `setAssociatedCommitmentPool` (0xe19abef8) function"]
        pub fn set_associated_commitment_pool(
            &self,
            commitment_pool_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 154, 190, 248], commitment_pool_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBridgeProxyAddress` (0xa3bc64f2) function"]
        pub fn set_bridge_proxy_address(
            &self,
            bridge_proxy_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 188, 100, 242], bridge_proxy_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDepositsDisabled` (0xea0cde85) function"]
        pub fn set_deposits_disabled(
            &self,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 12, 222, 133], state)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMaxAmount` (0x4fe47f70) function"]
        pub fn set_max_amount(
            &self,
            max_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 228, 127, 112], max_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMinAmount` (0x897b0637) function"]
        pub fn set_min_amount(
            &self,
            min_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 123, 6, 55], min_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMinBridgeFee` (0x19e75d6e) function"]
        pub fn set_min_bridge_fee(
            &self,
            min_bridge_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 231, 93, 110], min_bridge_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMinExecutorFee` (0x5e10b2b7) function"]
        pub fn set_min_executor_fee(
            &self,
            min_executor_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 16, 178, 183], min_executor_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPeerContract` (0x7d2c8520) function"]
        pub fn set_peer_contract(
            &self,
            peer_chain_id: u64,
            peer_chain_name: String,
            peer_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [125, 44, 133, 32],
                    (peer_chain_id, peer_chain_name, peer_contract),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPeerMinExecutorFee` (0x153dc450) function"]
        pub fn set_peer_min_executor_fee(
            &self,
            peer_min_executor_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 61, 196, 80], peer_min_executor_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPeerMinRollupFee` (0x521ff057) function"]
        pub fn set_peer_min_rollup_fee(
            &self,
            peer_min_rollup_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 31, 240, 87], peer_min_rollup_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateSanctionsListAddress` (0x30f49cac) function"]
        pub fn update_sanctions_list_address(
            &self,
            sanction: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 244, 156, 172], sanction)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CommitmentCrossChain` event"]
        pub fn commitment_cross_chain_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CommitmentCrossChainFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DepositsDisabled` event"]
        pub fn deposits_disabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DepositsDisabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MaxAmount` event"]
        pub fn max_amount_filter(&self) -> ethers::contract::builders::Event<M, MaxAmountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MinAmount` event"]
        pub fn min_amount_filter(&self) -> ethers::contract::builders::Event<M, MinAmountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MinBridgeFee` event"]
        pub fn min_bridge_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MinBridgeFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MinExecutorFee` event"]
        pub fn min_executor_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MinExecutorFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OperatorChanged` event"]
        pub fn operator_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OperatorChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeerMinExecutorFee` event"]
        pub fn peer_min_executor_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeerMinExecutorFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeerMinRollupFee` event"]
        pub fn peer_min_rollup_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeerMinRollupFeeFilter> {
            self.0.event()
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, MystikoV2CelerERC20Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MystikoV2CelerERC20<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `AmountLessThanZero` with signature `AmountLessThanZero()` and selector `[130, 11, 241, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "AmountLessThanZero", abi = "AmountLessThanZero()")]
    pub struct AmountLessThanZero;
    #[doc = "Custom Error type `AmountTooLarge` with signature `AmountTooLarge()` and selector `[6, 37, 4, 1]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "AmountTooLarge", abi = "AmountTooLarge()")]
    pub struct AmountTooLarge;
    #[doc = "Custom Error type `AmountTooSmall` with signature `AmountTooSmall()` and selector `[194, 245, 98, 90]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "AmountTooSmall", abi = "AmountTooSmall()")]
    pub struct AmountTooSmall;
    #[doc = "Custom Error type `BridgeFeeTooFew` with signature `BridgeFeeTooFew()` and selector `[196, 216, 208, 13]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "BridgeFeeTooFew", abi = "BridgeFeeTooFew()")]
    pub struct BridgeFeeTooFew;
    #[doc = "Custom Error type `CommitmentHashIncorrect` with signature `CommitmentHashIncorrect()` and selector `[55, 245, 68, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "CommitmentHashIncorrect", abi = "CommitmentHashIncorrect()")]
    pub struct CommitmentHashIncorrect;
    #[doc = "Custom Error type `DepositsDisabled` with signature `DepositsDisabled()` and selector `[113, 122, 22, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "DepositsDisabled", abi = "DepositsDisabled()")]
    pub struct DepositsDisabled;
    #[doc = "Custom Error type `ExecutorFeeTooFew` with signature `ExecutorFeeTooFew()` and selector `[171, 77, 173, 66]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "ExecutorFeeTooFew", abi = "ExecutorFeeTooFew()")]
    pub struct ExecutorFeeTooFew;
    #[doc = "Custom Error type `FromChainIdNotMatched` with signature `FromChainIdNotMatched()` and selector `[225, 216, 193, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "FromChainIdNotMatched", abi = "FromChainIdNotMatched()")]
    pub struct FromChainIdNotMatched;
    #[doc = "Custom Error type `FromProxyAddressNotMatched` with signature `FromProxyAddressNotMatched()` and selector `[40, 129, 192, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "FromProxyAddressNotMatched",
        abi = "FromProxyAddressNotMatched()"
    )]
    pub struct FromProxyAddressNotMatched;
    #[doc = "Custom Error type `HashKGreaterThanFieldSize` with signature `HashKGreaterThanFieldSize()` and selector `[128, 95, 42, 73]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "HashKGreaterThanFieldSize",
        abi = "HashKGreaterThanFieldSize()"
    )]
    pub struct HashKGreaterThanFieldSize;
    #[doc = "Custom Error type `Invalid` with signature `Invalid(string)` and selector `[83, 162, 85, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Invalid", abi = "Invalid(string)")]
    pub struct Invalid {
        pub param: String,
    }
    #[doc = "Custom Error type `MaxAmountLessThanMinAmount` with signature `MaxAmountLessThanMinAmount()` and selector `[201, 28, 83, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "MaxAmountLessThanMinAmount",
        abi = "MaxAmountLessThanMinAmount()"
    )]
    pub struct MaxAmountLessThanMinAmount;
    #[doc = "Custom Error type `MinAmountGreaterThanMaxAmount` with signature `MinAmountGreaterThanMaxAmount()` and selector `[192, 7, 208, 66]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "MinAmountGreaterThanMaxAmount",
        abi = "MinAmountGreaterThanMaxAmount()"
    )]
    pub struct MinAmountGreaterThanMaxAmount;
    #[doc = "Custom Error type `NotChanged` with signature `NotChanged()` and selector `[54, 161, 195, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NotChanged", abi = "NotChanged()")]
    pub struct NotChanged;
    #[doc = "Custom Error type `OnlyOperator` with signature `OnlyOperator()` and selector `[39, 225, 241, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyOperator", abi = "OnlyOperator()")]
    pub struct OnlyOperator;
    #[doc = "Custom Error type `RandomSGreaterThanFieldSize` with signature `RandomSGreaterThanFieldSize()` and selector `[238, 247, 130, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "RandomSGreaterThanFieldSize",
        abi = "RandomSGreaterThanFieldSize()"
    )]
    pub struct RandomSGreaterThanFieldSize;
    #[doc = "Custom Error type `RollupFeeToFew` with signature `RollupFeeToFew()` and selector `[240, 158, 5, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "RollupFeeToFew", abi = "RollupFeeToFew()")]
    pub struct RollupFeeToFew;
    #[doc = "Custom Error type `SanctionedAddress` with signature `SanctionedAddress()` and selector `[46, 112, 192, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "SanctionedAddress", abi = "SanctionedAddress()")]
    pub struct SanctionedAddress;
    #[doc = "Custom Error type `SenderIsNotBridgeProxy` with signature `SenderIsNotBridgeProxy()` and selector `[123, 148, 3, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "SenderIsNotBridgeProxy", abi = "SenderIsNotBridgeProxy()")]
    pub struct SenderIsNotBridgeProxy;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MystikoV2CelerERC20Errors {
        AmountLessThanZero(AmountLessThanZero),
        AmountTooLarge(AmountTooLarge),
        AmountTooSmall(AmountTooSmall),
        BridgeFeeTooFew(BridgeFeeTooFew),
        CommitmentHashIncorrect(CommitmentHashIncorrect),
        DepositsDisabled(DepositsDisabled),
        ExecutorFeeTooFew(ExecutorFeeTooFew),
        FromChainIdNotMatched(FromChainIdNotMatched),
        FromProxyAddressNotMatched(FromProxyAddressNotMatched),
        HashKGreaterThanFieldSize(HashKGreaterThanFieldSize),
        Invalid(Invalid),
        MaxAmountLessThanMinAmount(MaxAmountLessThanMinAmount),
        MinAmountGreaterThanMaxAmount(MinAmountGreaterThanMaxAmount),
        NotChanged(NotChanged),
        OnlyOperator(OnlyOperator),
        RandomSGreaterThanFieldSize(RandomSGreaterThanFieldSize),
        RollupFeeToFew(RollupFeeToFew),
        SanctionedAddress(SanctionedAddress),
        SenderIsNotBridgeProxy(SenderIsNotBridgeProxy),
    }
    impl ethers::core::abi::AbiDecode for MystikoV2CelerERC20Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AmountLessThanZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::AmountLessThanZero(decoded));
            }
            if let Ok(decoded) =
                <AmountTooLarge as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::AmountTooLarge(decoded));
            }
            if let Ok(decoded) =
                <AmountTooSmall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::AmountTooSmall(decoded));
            }
            if let Ok(decoded) =
                <BridgeFeeTooFew as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::BridgeFeeTooFew(decoded));
            }
            if let Ok(decoded) =
                <CommitmentHashIncorrect as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::CommitmentHashIncorrect(decoded));
            }
            if let Ok(decoded) =
                <DepositsDisabled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::DepositsDisabled(decoded));
            }
            if let Ok(decoded) =
                <ExecutorFeeTooFew as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::ExecutorFeeTooFew(decoded));
            }
            if let Ok(decoded) =
                <FromChainIdNotMatched as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::FromChainIdNotMatched(decoded));
            }
            if let Ok(decoded) =
                <FromProxyAddressNotMatched as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::FromProxyAddressNotMatched(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <HashKGreaterThanFieldSize as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::HashKGreaterThanFieldSize(
                    decoded,
                ));
            }
            if let Ok(decoded) = <Invalid as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MystikoV2CelerERC20Errors::Invalid(decoded));
            }
            if let Ok(decoded) =
                <MaxAmountLessThanMinAmount as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::MaxAmountLessThanMinAmount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MinAmountGreaterThanMaxAmount as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2CelerERC20Errors::MinAmountGreaterThanMaxAmount(
                    decoded,
                ));
            }
            if let Ok(decoded) = <NotChanged as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::NotChanged(decoded));
            }
            if let Ok(decoded) =
                <OnlyOperator as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::OnlyOperator(decoded));
            }
            if let Ok(decoded) =
                <RandomSGreaterThanFieldSize as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::RandomSGreaterThanFieldSize(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RollupFeeToFew as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::RollupFeeToFew(decoded));
            }
            if let Ok(decoded) =
                <SanctionedAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::SanctionedAddress(decoded));
            }
            if let Ok(decoded) =
                <SenderIsNotBridgeProxy as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Errors::SenderIsNotBridgeProxy(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MystikoV2CelerERC20Errors {
        fn encode(self) -> Vec<u8> {
            match self {
                MystikoV2CelerERC20Errors::AmountLessThanZero(element) => element.encode(),
                MystikoV2CelerERC20Errors::AmountTooLarge(element) => element.encode(),
                MystikoV2CelerERC20Errors::AmountTooSmall(element) => element.encode(),
                MystikoV2CelerERC20Errors::BridgeFeeTooFew(element) => element.encode(),
                MystikoV2CelerERC20Errors::CommitmentHashIncorrect(element) => element.encode(),
                MystikoV2CelerERC20Errors::DepositsDisabled(element) => element.encode(),
                MystikoV2CelerERC20Errors::ExecutorFeeTooFew(element) => element.encode(),
                MystikoV2CelerERC20Errors::FromChainIdNotMatched(element) => element.encode(),
                MystikoV2CelerERC20Errors::FromProxyAddressNotMatched(element) => element.encode(),
                MystikoV2CelerERC20Errors::HashKGreaterThanFieldSize(element) => element.encode(),
                MystikoV2CelerERC20Errors::Invalid(element) => element.encode(),
                MystikoV2CelerERC20Errors::MaxAmountLessThanMinAmount(element) => element.encode(),
                MystikoV2CelerERC20Errors::MinAmountGreaterThanMaxAmount(element) => {
                    element.encode()
                }
                MystikoV2CelerERC20Errors::NotChanged(element) => element.encode(),
                MystikoV2CelerERC20Errors::OnlyOperator(element) => element.encode(),
                MystikoV2CelerERC20Errors::RandomSGreaterThanFieldSize(element) => element.encode(),
                MystikoV2CelerERC20Errors::RollupFeeToFew(element) => element.encode(),
                MystikoV2CelerERC20Errors::SanctionedAddress(element) => element.encode(),
                MystikoV2CelerERC20Errors::SenderIsNotBridgeProxy(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MystikoV2CelerERC20Errors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoV2CelerERC20Errors::AmountLessThanZero(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::AmountTooLarge(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::AmountTooSmall(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::BridgeFeeTooFew(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::CommitmentHashIncorrect(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::DepositsDisabled(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::ExecutorFeeTooFew(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::FromChainIdNotMatched(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::FromProxyAddressNotMatched(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::HashKGreaterThanFieldSize(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::Invalid(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::MaxAmountLessThanMinAmount(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::MinAmountGreaterThanMaxAmount(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::NotChanged(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::OnlyOperator(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::RandomSGreaterThanFieldSize(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::RollupFeeToFew(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::SanctionedAddress(element) => element.fmt(f),
                MystikoV2CelerERC20Errors::SenderIsNotBridgeProxy(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AmountLessThanZero> for MystikoV2CelerERC20Errors {
        fn from(var: AmountLessThanZero) -> Self {
            MystikoV2CelerERC20Errors::AmountLessThanZero(var)
        }
    }
    impl ::std::convert::From<AmountTooLarge> for MystikoV2CelerERC20Errors {
        fn from(var: AmountTooLarge) -> Self {
            MystikoV2CelerERC20Errors::AmountTooLarge(var)
        }
    }
    impl ::std::convert::From<AmountTooSmall> for MystikoV2CelerERC20Errors {
        fn from(var: AmountTooSmall) -> Self {
            MystikoV2CelerERC20Errors::AmountTooSmall(var)
        }
    }
    impl ::std::convert::From<BridgeFeeTooFew> for MystikoV2CelerERC20Errors {
        fn from(var: BridgeFeeTooFew) -> Self {
            MystikoV2CelerERC20Errors::BridgeFeeTooFew(var)
        }
    }
    impl ::std::convert::From<CommitmentHashIncorrect> for MystikoV2CelerERC20Errors {
        fn from(var: CommitmentHashIncorrect) -> Self {
            MystikoV2CelerERC20Errors::CommitmentHashIncorrect(var)
        }
    }
    impl ::std::convert::From<DepositsDisabled> for MystikoV2CelerERC20Errors {
        fn from(var: DepositsDisabled) -> Self {
            MystikoV2CelerERC20Errors::DepositsDisabled(var)
        }
    }
    impl ::std::convert::From<ExecutorFeeTooFew> for MystikoV2CelerERC20Errors {
        fn from(var: ExecutorFeeTooFew) -> Self {
            MystikoV2CelerERC20Errors::ExecutorFeeTooFew(var)
        }
    }
    impl ::std::convert::From<FromChainIdNotMatched> for MystikoV2CelerERC20Errors {
        fn from(var: FromChainIdNotMatched) -> Self {
            MystikoV2CelerERC20Errors::FromChainIdNotMatched(var)
        }
    }
    impl ::std::convert::From<FromProxyAddressNotMatched> for MystikoV2CelerERC20Errors {
        fn from(var: FromProxyAddressNotMatched) -> Self {
            MystikoV2CelerERC20Errors::FromProxyAddressNotMatched(var)
        }
    }
    impl ::std::convert::From<HashKGreaterThanFieldSize> for MystikoV2CelerERC20Errors {
        fn from(var: HashKGreaterThanFieldSize) -> Self {
            MystikoV2CelerERC20Errors::HashKGreaterThanFieldSize(var)
        }
    }
    impl ::std::convert::From<Invalid> for MystikoV2CelerERC20Errors {
        fn from(var: Invalid) -> Self {
            MystikoV2CelerERC20Errors::Invalid(var)
        }
    }
    impl ::std::convert::From<MaxAmountLessThanMinAmount> for MystikoV2CelerERC20Errors {
        fn from(var: MaxAmountLessThanMinAmount) -> Self {
            MystikoV2CelerERC20Errors::MaxAmountLessThanMinAmount(var)
        }
    }
    impl ::std::convert::From<MinAmountGreaterThanMaxAmount> for MystikoV2CelerERC20Errors {
        fn from(var: MinAmountGreaterThanMaxAmount) -> Self {
            MystikoV2CelerERC20Errors::MinAmountGreaterThanMaxAmount(var)
        }
    }
    impl ::std::convert::From<NotChanged> for MystikoV2CelerERC20Errors {
        fn from(var: NotChanged) -> Self {
            MystikoV2CelerERC20Errors::NotChanged(var)
        }
    }
    impl ::std::convert::From<OnlyOperator> for MystikoV2CelerERC20Errors {
        fn from(var: OnlyOperator) -> Self {
            MystikoV2CelerERC20Errors::OnlyOperator(var)
        }
    }
    impl ::std::convert::From<RandomSGreaterThanFieldSize> for MystikoV2CelerERC20Errors {
        fn from(var: RandomSGreaterThanFieldSize) -> Self {
            MystikoV2CelerERC20Errors::RandomSGreaterThanFieldSize(var)
        }
    }
    impl ::std::convert::From<RollupFeeToFew> for MystikoV2CelerERC20Errors {
        fn from(var: RollupFeeToFew) -> Self {
            MystikoV2CelerERC20Errors::RollupFeeToFew(var)
        }
    }
    impl ::std::convert::From<SanctionedAddress> for MystikoV2CelerERC20Errors {
        fn from(var: SanctionedAddress) -> Self {
            MystikoV2CelerERC20Errors::SanctionedAddress(var)
        }
    }
    impl ::std::convert::From<SenderIsNotBridgeProxy> for MystikoV2CelerERC20Errors {
        fn from(var: SenderIsNotBridgeProxy) -> Self {
            MystikoV2CelerERC20Errors::SenderIsNotBridgeProxy(var)
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
    #[ethevent(name = "CommitmentCrossChain", abi = "CommitmentCrossChain(uint256)")]
    pub struct CommitmentCrossChainFilter {
        #[ethevent(indexed)]
        pub commitment: ethers::core::types::U256,
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
    #[ethevent(name = "DepositsDisabled", abi = "DepositsDisabled(bool)")]
    pub struct DepositsDisabledFilter {
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
    #[ethevent(name = "MaxAmount", abi = "MaxAmount(uint256)")]
    pub struct MaxAmountFilter {
        pub max_amount: ethers::core::types::U256,
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
    #[ethevent(name = "MinAmount", abi = "MinAmount(uint256)")]
    pub struct MinAmountFilter {
        pub min_amount: ethers::core::types::U256,
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
    #[ethevent(name = "MinBridgeFee", abi = "MinBridgeFee(uint256)")]
    pub struct MinBridgeFeeFilter {
        pub min_bridge_fee: ethers::core::types::U256,
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
    #[ethevent(name = "MinExecutorFee", abi = "MinExecutorFee(uint256)")]
    pub struct MinExecutorFeeFilter {
        pub min_executor_fee: ethers::core::types::U256,
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
    #[ethevent(name = "OperatorChanged", abi = "OperatorChanged(address)")]
    pub struct OperatorChangedFilter {
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
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
    #[ethevent(name = "PeerMinExecutorFee", abi = "PeerMinExecutorFee(uint256)")]
    pub struct PeerMinExecutorFeeFilter {
        pub peer_min_executor_fee: ethers::core::types::U256,
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
    #[ethevent(name = "PeerMinRollupFee", abi = "PeerMinRollupFee(uint256)")]
    pub struct PeerMinRollupFeeFilter {
        pub peer_min_rollup_fee: ethers::core::types::U256,
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
    pub enum MystikoV2CelerERC20Events {
        CommitmentCrossChainFilter(CommitmentCrossChainFilter),
        DepositsDisabledFilter(DepositsDisabledFilter),
        MaxAmountFilter(MaxAmountFilter),
        MinAmountFilter(MinAmountFilter),
        MinBridgeFeeFilter(MinBridgeFeeFilter),
        MinExecutorFeeFilter(MinExecutorFeeFilter),
        OperatorChangedFilter(OperatorChangedFilter),
        PeerMinExecutorFeeFilter(PeerMinExecutorFeeFilter),
        PeerMinRollupFeeFilter(PeerMinRollupFeeFilter),
        SanctionsCheckFilter(SanctionsCheckFilter),
        SanctionsListFilter(SanctionsListFilter),
    }
    impl ethers::contract::EthLogDecode for MystikoV2CelerERC20Events {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CommitmentCrossChainFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::CommitmentCrossChainFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = DepositsDisabledFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::DepositsDisabledFilter(decoded));
            }
            if let Ok(decoded) = MaxAmountFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::MaxAmountFilter(decoded));
            }
            if let Ok(decoded) = MinAmountFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::MinAmountFilter(decoded));
            }
            if let Ok(decoded) = MinBridgeFeeFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::MinBridgeFeeFilter(decoded));
            }
            if let Ok(decoded) = MinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::MinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = PeerMinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::PeerMinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = PeerMinRollupFeeFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::PeerMinRollupFeeFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(MystikoV2CelerERC20Events::SanctionsListFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MystikoV2CelerERC20Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoV2CelerERC20Events::CommitmentCrossChainFilter(element) => element.fmt(f),
                MystikoV2CelerERC20Events::DepositsDisabledFilter(element) => element.fmt(f),
                MystikoV2CelerERC20Events::MaxAmountFilter(element) => element.fmt(f),
                MystikoV2CelerERC20Events::MinAmountFilter(element) => element.fmt(f),
                MystikoV2CelerERC20Events::MinBridgeFeeFilter(element) => element.fmt(f),
                MystikoV2CelerERC20Events::MinExecutorFeeFilter(element) => element.fmt(f),
                MystikoV2CelerERC20Events::OperatorChangedFilter(element) => element.fmt(f),
                MystikoV2CelerERC20Events::PeerMinExecutorFeeFilter(element) => element.fmt(f),
                MystikoV2CelerERC20Events::PeerMinRollupFeeFilter(element) => element.fmt(f),
                MystikoV2CelerERC20Events::SanctionsCheckFilter(element) => element.fmt(f),
                MystikoV2CelerERC20Events::SanctionsListFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `assetDecimals` function with signature `assetDecimals()` and selector `[194, 212, 22, 1]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "assetDecimals", abi = "assetDecimals()")]
    pub struct AssetDecimalsCall;
    #[doc = "Container type for all input parameters for the `assetName` function with signature `assetName()` and selector `[201, 35, 12, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "assetName", abi = "assetName()")]
    pub struct AssetNameCall;
    #[doc = "Container type for all input parameters for the `assetSymbol` function with signature `assetSymbol()` and selector `[23, 109, 231, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "assetSymbol", abi = "assetSymbol()")]
    pub struct AssetSymbolCall;
    #[doc = "Container type for all input parameters for the `assetType` function with signature `assetType()` and selector `[63, 227, 52, 122]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "assetType", abi = "assetType()")]
    pub struct AssetTypeCall;
    #[doc = "Container type for all input parameters for the `bridgeProxyAddress` function with signature `bridgeProxyAddress()` and selector `[44, 210, 109, 69]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "bridgeProxyAddress", abi = "bridgeProxyAddress()")]
    pub struct BridgeProxyAddressCall;
    #[doc = "Container type for all input parameters for the `bridgeType` function with signature `bridgeType()` and selector `[36, 33, 225, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "bridgeType", abi = "bridgeType()")]
    pub struct BridgeTypeCall;
    #[doc = "Container type for all input parameters for the `changeOperator` function with signature `changeOperator(address)` and selector `[6, 57, 76, 155]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "changeOperator", abi = "changeOperator(address)")]
    pub struct ChangeOperatorCall {
        pub new_operator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `deposit` function with signature `deposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256))` and selector `[154, 3, 99, 108]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "deposit",
        abi = "deposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256))"
    )]
    pub struct DepositCall {
        pub request: DepositRequest,
    }
    #[doc = "Container type for all input parameters for the `disableSanctionsCheck` function with signature `disableSanctionsCheck()` and selector `[221, 117, 124, 52]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "disableSanctionsCheck", abi = "disableSanctionsCheck()")]
    pub struct DisableSanctionsCheckCall;
    #[doc = "Container type for all input parameters for the `enableSanctionsCheck` function with signature `enableSanctionsCheck()` and selector `[1, 219, 241, 159]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "enableSanctionsCheck", abi = "enableSanctionsCheck()")]
    pub struct EnableSanctionsCheckCall;
    #[doc = "Container type for all input parameters for the `executeMessage` function with signature `executeMessage(address,uint64,bytes,address)` and selector `[156, 100, 159, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "executeMessage",
        abi = "executeMessage(address,uint64,bytes,address)"
    )]
    pub struct ExecuteMessageCall {
        pub sender: ethers::core::types::Address,
        pub src_chain_id: u64,
        pub message: ethers::core::types::Bytes,
        pub executor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAssociatedCommitmentPool` function with signature `getAssociatedCommitmentPool()` and selector `[221, 172, 93, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getAssociatedCommitmentPool",
        abi = "getAssociatedCommitmentPool()"
    )]
    pub struct GetAssociatedCommitmentPoolCall;
    #[doc = "Container type for all input parameters for the `getMaxAmount` function with signature `getMaxAmount()` and selector `[11, 169, 89, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMaxAmount", abi = "getMaxAmount()")]
    pub struct GetMaxAmountCall;
    #[doc = "Container type for all input parameters for the `getMinAmount` function with signature `getMinAmount()` and selector `[207, 199, 226, 218]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMinAmount", abi = "getMinAmount()")]
    pub struct GetMinAmountCall;
    #[doc = "Container type for all input parameters for the `getMinBridgeFee` function with signature `getMinBridgeFee()` and selector `[239, 191, 178, 174]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMinBridgeFee", abi = "getMinBridgeFee()")]
    pub struct GetMinBridgeFeeCall;
    #[doc = "Container type for all input parameters for the `getMinExecutorFee` function with signature `getMinExecutorFee()` and selector `[244, 173, 23, 198]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMinExecutorFee", abi = "getMinExecutorFee()")]
    pub struct GetMinExecutorFeeCall;
    #[doc = "Container type for all input parameters for the `getPeerMinExecutorFee` function with signature `getPeerMinExecutorFee()` and selector `[88, 152, 160, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPeerMinExecutorFee", abi = "getPeerMinExecutorFee()")]
    pub struct GetPeerMinExecutorFeeCall;
    #[doc = "Container type for all input parameters for the `getPeerMinRollupFee` function with signature `getPeerMinRollupFee()` and selector `[130, 91, 95, 141]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPeerMinRollupFee", abi = "getPeerMinRollupFee()")]
    pub struct GetPeerMinRollupFeeCall;
    #[doc = "Container type for all input parameters for the `isDepositsDisabled` function with signature `isDepositsDisabled()` and selector `[237, 110, 163, 58]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isDepositsDisabled", abi = "isDepositsDisabled()")]
    pub struct IsDepositsDisabledCall;
    #[doc = "Container type for all input parameters for the `peerChainId` function with signature `peerChainId()` and selector `[205, 252, 238, 186]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "peerChainId", abi = "peerChainId()")]
    pub struct PeerChainIdCall;
    #[doc = "Container type for all input parameters for the `peerChainName` function with signature `peerChainName()` and selector `[78, 60, 16, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "peerChainName", abi = "peerChainName()")]
    pub struct PeerChainNameCall;
    #[doc = "Container type for all input parameters for the `peerContract` function with signature `peerContract()` and selector `[33, 227, 45, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "peerContract", abi = "peerContract()")]
    pub struct PeerContractCall;
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
    #[doc = "Container type for all input parameters for the `setAssociatedCommitmentPool` function with signature `setAssociatedCommitmentPool(address)` and selector `[225, 154, 190, 248]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setAssociatedCommitmentPool",
        abi = "setAssociatedCommitmentPool(address)"
    )]
    pub struct SetAssociatedCommitmentPoolCall {
        pub commitment_pool_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setBridgeProxyAddress` function with signature `setBridgeProxyAddress(address)` and selector `[163, 188, 100, 242]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setBridgeProxyAddress", abi = "setBridgeProxyAddress(address)")]
    pub struct SetBridgeProxyAddressCall {
        pub bridge_proxy_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setDepositsDisabled` function with signature `setDepositsDisabled(bool)` and selector `[234, 12, 222, 133]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setDepositsDisabled", abi = "setDepositsDisabled(bool)")]
    pub struct SetDepositsDisabledCall {
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `setMaxAmount` function with signature `setMaxAmount(uint256)` and selector `[79, 228, 127, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMaxAmount", abi = "setMaxAmount(uint256)")]
    pub struct SetMaxAmountCall {
        pub max_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setMinAmount` function with signature `setMinAmount(uint256)` and selector `[137, 123, 6, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMinAmount", abi = "setMinAmount(uint256)")]
    pub struct SetMinAmountCall {
        pub min_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setMinBridgeFee` function with signature `setMinBridgeFee(uint256)` and selector `[25, 231, 93, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMinBridgeFee", abi = "setMinBridgeFee(uint256)")]
    pub struct SetMinBridgeFeeCall {
        pub min_bridge_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setMinExecutorFee` function with signature `setMinExecutorFee(uint256)` and selector `[94, 16, 178, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMinExecutorFee", abi = "setMinExecutorFee(uint256)")]
    pub struct SetMinExecutorFeeCall {
        pub min_executor_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPeerContract` function with signature `setPeerContract(uint64,string,address)` and selector `[125, 44, 133, 32]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setPeerContract",
        abi = "setPeerContract(uint64,string,address)"
    )]
    pub struct SetPeerContractCall {
        pub peer_chain_id: u64,
        pub peer_chain_name: String,
        pub peer_contract: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPeerMinExecutorFee` function with signature `setPeerMinExecutorFee(uint256)` and selector `[21, 61, 196, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setPeerMinExecutorFee", abi = "setPeerMinExecutorFee(uint256)")]
    pub struct SetPeerMinExecutorFeeCall {
        pub peer_min_executor_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPeerMinRollupFee` function with signature `setPeerMinRollupFee(uint256)` and selector `[82, 31, 240, 87]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setPeerMinRollupFee", abi = "setPeerMinRollupFee(uint256)")]
    pub struct SetPeerMinRollupFeeCall {
        pub peer_min_rollup_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateSanctionsListAddress` function with signature `updateSanctionsListAddress(address)` and selector `[48, 244, 156, 172]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "updateSanctionsListAddress",
        abi = "updateSanctionsListAddress(address)"
    )]
    pub struct UpdateSanctionsListAddressCall {
        pub sanction: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MystikoV2CelerERC20Calls {
        AssetDecimals(AssetDecimalsCall),
        AssetName(AssetNameCall),
        AssetSymbol(AssetSymbolCall),
        AssetType(AssetTypeCall),
        BridgeProxyAddress(BridgeProxyAddressCall),
        BridgeType(BridgeTypeCall),
        ChangeOperator(ChangeOperatorCall),
        Deposit(DepositCall),
        DisableSanctionsCheck(DisableSanctionsCheckCall),
        EnableSanctionsCheck(EnableSanctionsCheckCall),
        ExecuteMessage(ExecuteMessageCall),
        GetAssociatedCommitmentPool(GetAssociatedCommitmentPoolCall),
        GetMaxAmount(GetMaxAmountCall),
        GetMinAmount(GetMinAmountCall),
        GetMinBridgeFee(GetMinBridgeFeeCall),
        GetMinExecutorFee(GetMinExecutorFeeCall),
        GetPeerMinExecutorFee(GetPeerMinExecutorFeeCall),
        GetPeerMinRollupFee(GetPeerMinRollupFeeCall),
        IsDepositsDisabled(IsDepositsDisabledCall),
        PeerChainId(PeerChainIdCall),
        PeerChainName(PeerChainNameCall),
        PeerContract(PeerContractCall),
        SanctionsCheck(SanctionsCheckCall),
        SanctionsList(SanctionsListCall),
        SetAssociatedCommitmentPool(SetAssociatedCommitmentPoolCall),
        SetBridgeProxyAddress(SetBridgeProxyAddressCall),
        SetDepositsDisabled(SetDepositsDisabledCall),
        SetMaxAmount(SetMaxAmountCall),
        SetMinAmount(SetMinAmountCall),
        SetMinBridgeFee(SetMinBridgeFeeCall),
        SetMinExecutorFee(SetMinExecutorFeeCall),
        SetPeerContract(SetPeerContractCall),
        SetPeerMinExecutorFee(SetPeerMinExecutorFeeCall),
        SetPeerMinRollupFee(SetPeerMinRollupFeeCall),
        UpdateSanctionsListAddress(UpdateSanctionsListAddressCall),
    }
    impl ethers::core::abi::AbiDecode for MystikoV2CelerERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AssetDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::AssetDecimals(decoded));
            }
            if let Ok(decoded) =
                <AssetNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::AssetName(decoded));
            }
            if let Ok(decoded) =
                <AssetSymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::AssetSymbol(decoded));
            }
            if let Ok(decoded) =
                <AssetTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::AssetType(decoded));
            }
            if let Ok(decoded) =
                <BridgeProxyAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::BridgeProxyAddress(decoded));
            }
            if let Ok(decoded) =
                <BridgeTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::BridgeType(decoded));
            }
            if let Ok(decoded) =
                <ChangeOperatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::ChangeOperator(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DisableSanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::DisableSanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <EnableSanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::EnableSanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <ExecuteMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::ExecuteMessage(decoded));
            }
            if let Ok(decoded) =
                <GetAssociatedCommitmentPoolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2CelerERC20Calls::GetAssociatedCommitmentPool(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetMaxAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::GetMaxAmount(decoded));
            }
            if let Ok(decoded) =
                <GetMinAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::GetMinAmount(decoded));
            }
            if let Ok(decoded) =
                <GetMinBridgeFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::GetMinBridgeFee(decoded));
            }
            if let Ok(decoded) =
                <GetMinExecutorFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::GetMinExecutorFee(decoded));
            }
            if let Ok(decoded) =
                <GetPeerMinExecutorFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::GetPeerMinExecutorFee(decoded));
            }
            if let Ok(decoded) =
                <GetPeerMinRollupFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::GetPeerMinRollupFee(decoded));
            }
            if let Ok(decoded) =
                <IsDepositsDisabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::IsDepositsDisabled(decoded));
            }
            if let Ok(decoded) =
                <PeerChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::PeerChainId(decoded));
            }
            if let Ok(decoded) =
                <PeerChainNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::PeerChainName(decoded));
            }
            if let Ok(decoded) =
                <PeerContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::PeerContract(decoded));
            }
            if let Ok(decoded) =
                <SanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <SanctionsListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SanctionsList(decoded));
            }
            if let Ok(decoded) =
                <SetAssociatedCommitmentPoolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2CelerERC20Calls::SetAssociatedCommitmentPool(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetBridgeProxyAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SetBridgeProxyAddress(decoded));
            }
            if let Ok(decoded) =
                <SetDepositsDisabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SetDepositsDisabled(decoded));
            }
            if let Ok(decoded) =
                <SetMaxAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SetMaxAmount(decoded));
            }
            if let Ok(decoded) =
                <SetMinAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SetMinAmount(decoded));
            }
            if let Ok(decoded) =
                <SetMinBridgeFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SetMinBridgeFee(decoded));
            }
            if let Ok(decoded) =
                <SetMinExecutorFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SetMinExecutorFee(decoded));
            }
            if let Ok(decoded) =
                <SetPeerContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SetPeerContract(decoded));
            }
            if let Ok(decoded) =
                <SetPeerMinExecutorFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SetPeerMinExecutorFee(decoded));
            }
            if let Ok(decoded) =
                <SetPeerMinRollupFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2CelerERC20Calls::SetPeerMinRollupFee(decoded));
            }
            if let Ok(decoded) =
                <UpdateSanctionsListAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2CelerERC20Calls::UpdateSanctionsListAddress(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MystikoV2CelerERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                MystikoV2CelerERC20Calls::AssetDecimals(element) => element.encode(),
                MystikoV2CelerERC20Calls::AssetName(element) => element.encode(),
                MystikoV2CelerERC20Calls::AssetSymbol(element) => element.encode(),
                MystikoV2CelerERC20Calls::AssetType(element) => element.encode(),
                MystikoV2CelerERC20Calls::BridgeProxyAddress(element) => element.encode(),
                MystikoV2CelerERC20Calls::BridgeType(element) => element.encode(),
                MystikoV2CelerERC20Calls::ChangeOperator(element) => element.encode(),
                MystikoV2CelerERC20Calls::Deposit(element) => element.encode(),
                MystikoV2CelerERC20Calls::DisableSanctionsCheck(element) => element.encode(),
                MystikoV2CelerERC20Calls::EnableSanctionsCheck(element) => element.encode(),
                MystikoV2CelerERC20Calls::ExecuteMessage(element) => element.encode(),
                MystikoV2CelerERC20Calls::GetAssociatedCommitmentPool(element) => element.encode(),
                MystikoV2CelerERC20Calls::GetMaxAmount(element) => element.encode(),
                MystikoV2CelerERC20Calls::GetMinAmount(element) => element.encode(),
                MystikoV2CelerERC20Calls::GetMinBridgeFee(element) => element.encode(),
                MystikoV2CelerERC20Calls::GetMinExecutorFee(element) => element.encode(),
                MystikoV2CelerERC20Calls::GetPeerMinExecutorFee(element) => element.encode(),
                MystikoV2CelerERC20Calls::GetPeerMinRollupFee(element) => element.encode(),
                MystikoV2CelerERC20Calls::IsDepositsDisabled(element) => element.encode(),
                MystikoV2CelerERC20Calls::PeerChainId(element) => element.encode(),
                MystikoV2CelerERC20Calls::PeerChainName(element) => element.encode(),
                MystikoV2CelerERC20Calls::PeerContract(element) => element.encode(),
                MystikoV2CelerERC20Calls::SanctionsCheck(element) => element.encode(),
                MystikoV2CelerERC20Calls::SanctionsList(element) => element.encode(),
                MystikoV2CelerERC20Calls::SetAssociatedCommitmentPool(element) => element.encode(),
                MystikoV2CelerERC20Calls::SetBridgeProxyAddress(element) => element.encode(),
                MystikoV2CelerERC20Calls::SetDepositsDisabled(element) => element.encode(),
                MystikoV2CelerERC20Calls::SetMaxAmount(element) => element.encode(),
                MystikoV2CelerERC20Calls::SetMinAmount(element) => element.encode(),
                MystikoV2CelerERC20Calls::SetMinBridgeFee(element) => element.encode(),
                MystikoV2CelerERC20Calls::SetMinExecutorFee(element) => element.encode(),
                MystikoV2CelerERC20Calls::SetPeerContract(element) => element.encode(),
                MystikoV2CelerERC20Calls::SetPeerMinExecutorFee(element) => element.encode(),
                MystikoV2CelerERC20Calls::SetPeerMinRollupFee(element) => element.encode(),
                MystikoV2CelerERC20Calls::UpdateSanctionsListAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MystikoV2CelerERC20Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoV2CelerERC20Calls::AssetDecimals(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::AssetName(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::AssetSymbol(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::AssetType(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::BridgeProxyAddress(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::BridgeType(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::ChangeOperator(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::Deposit(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::DisableSanctionsCheck(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::EnableSanctionsCheck(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::ExecuteMessage(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::GetAssociatedCommitmentPool(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::GetMaxAmount(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::GetMinAmount(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::GetMinBridgeFee(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::GetMinExecutorFee(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::GetPeerMinExecutorFee(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::GetPeerMinRollupFee(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::IsDepositsDisabled(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::PeerChainId(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::PeerChainName(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::PeerContract(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SanctionsCheck(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SanctionsList(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SetAssociatedCommitmentPool(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SetBridgeProxyAddress(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SetDepositsDisabled(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SetMaxAmount(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SetMinAmount(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SetMinBridgeFee(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SetMinExecutorFee(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SetPeerContract(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SetPeerMinExecutorFee(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::SetPeerMinRollupFee(element) => element.fmt(f),
                MystikoV2CelerERC20Calls::UpdateSanctionsListAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AssetDecimalsCall> for MystikoV2CelerERC20Calls {
        fn from(var: AssetDecimalsCall) -> Self {
            MystikoV2CelerERC20Calls::AssetDecimals(var)
        }
    }
    impl ::std::convert::From<AssetNameCall> for MystikoV2CelerERC20Calls {
        fn from(var: AssetNameCall) -> Self {
            MystikoV2CelerERC20Calls::AssetName(var)
        }
    }
    impl ::std::convert::From<AssetSymbolCall> for MystikoV2CelerERC20Calls {
        fn from(var: AssetSymbolCall) -> Self {
            MystikoV2CelerERC20Calls::AssetSymbol(var)
        }
    }
    impl ::std::convert::From<AssetTypeCall> for MystikoV2CelerERC20Calls {
        fn from(var: AssetTypeCall) -> Self {
            MystikoV2CelerERC20Calls::AssetType(var)
        }
    }
    impl ::std::convert::From<BridgeProxyAddressCall> for MystikoV2CelerERC20Calls {
        fn from(var: BridgeProxyAddressCall) -> Self {
            MystikoV2CelerERC20Calls::BridgeProxyAddress(var)
        }
    }
    impl ::std::convert::From<BridgeTypeCall> for MystikoV2CelerERC20Calls {
        fn from(var: BridgeTypeCall) -> Self {
            MystikoV2CelerERC20Calls::BridgeType(var)
        }
    }
    impl ::std::convert::From<ChangeOperatorCall> for MystikoV2CelerERC20Calls {
        fn from(var: ChangeOperatorCall) -> Self {
            MystikoV2CelerERC20Calls::ChangeOperator(var)
        }
    }
    impl ::std::convert::From<DepositCall> for MystikoV2CelerERC20Calls {
        fn from(var: DepositCall) -> Self {
            MystikoV2CelerERC20Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<DisableSanctionsCheckCall> for MystikoV2CelerERC20Calls {
        fn from(var: DisableSanctionsCheckCall) -> Self {
            MystikoV2CelerERC20Calls::DisableSanctionsCheck(var)
        }
    }
    impl ::std::convert::From<EnableSanctionsCheckCall> for MystikoV2CelerERC20Calls {
        fn from(var: EnableSanctionsCheckCall) -> Self {
            MystikoV2CelerERC20Calls::EnableSanctionsCheck(var)
        }
    }
    impl ::std::convert::From<ExecuteMessageCall> for MystikoV2CelerERC20Calls {
        fn from(var: ExecuteMessageCall) -> Self {
            MystikoV2CelerERC20Calls::ExecuteMessage(var)
        }
    }
    impl ::std::convert::From<GetAssociatedCommitmentPoolCall> for MystikoV2CelerERC20Calls {
        fn from(var: GetAssociatedCommitmentPoolCall) -> Self {
            MystikoV2CelerERC20Calls::GetAssociatedCommitmentPool(var)
        }
    }
    impl ::std::convert::From<GetMaxAmountCall> for MystikoV2CelerERC20Calls {
        fn from(var: GetMaxAmountCall) -> Self {
            MystikoV2CelerERC20Calls::GetMaxAmount(var)
        }
    }
    impl ::std::convert::From<GetMinAmountCall> for MystikoV2CelerERC20Calls {
        fn from(var: GetMinAmountCall) -> Self {
            MystikoV2CelerERC20Calls::GetMinAmount(var)
        }
    }
    impl ::std::convert::From<GetMinBridgeFeeCall> for MystikoV2CelerERC20Calls {
        fn from(var: GetMinBridgeFeeCall) -> Self {
            MystikoV2CelerERC20Calls::GetMinBridgeFee(var)
        }
    }
    impl ::std::convert::From<GetMinExecutorFeeCall> for MystikoV2CelerERC20Calls {
        fn from(var: GetMinExecutorFeeCall) -> Self {
            MystikoV2CelerERC20Calls::GetMinExecutorFee(var)
        }
    }
    impl ::std::convert::From<GetPeerMinExecutorFeeCall> for MystikoV2CelerERC20Calls {
        fn from(var: GetPeerMinExecutorFeeCall) -> Self {
            MystikoV2CelerERC20Calls::GetPeerMinExecutorFee(var)
        }
    }
    impl ::std::convert::From<GetPeerMinRollupFeeCall> for MystikoV2CelerERC20Calls {
        fn from(var: GetPeerMinRollupFeeCall) -> Self {
            MystikoV2CelerERC20Calls::GetPeerMinRollupFee(var)
        }
    }
    impl ::std::convert::From<IsDepositsDisabledCall> for MystikoV2CelerERC20Calls {
        fn from(var: IsDepositsDisabledCall) -> Self {
            MystikoV2CelerERC20Calls::IsDepositsDisabled(var)
        }
    }
    impl ::std::convert::From<PeerChainIdCall> for MystikoV2CelerERC20Calls {
        fn from(var: PeerChainIdCall) -> Self {
            MystikoV2CelerERC20Calls::PeerChainId(var)
        }
    }
    impl ::std::convert::From<PeerChainNameCall> for MystikoV2CelerERC20Calls {
        fn from(var: PeerChainNameCall) -> Self {
            MystikoV2CelerERC20Calls::PeerChainName(var)
        }
    }
    impl ::std::convert::From<PeerContractCall> for MystikoV2CelerERC20Calls {
        fn from(var: PeerContractCall) -> Self {
            MystikoV2CelerERC20Calls::PeerContract(var)
        }
    }
    impl ::std::convert::From<SanctionsCheckCall> for MystikoV2CelerERC20Calls {
        fn from(var: SanctionsCheckCall) -> Self {
            MystikoV2CelerERC20Calls::SanctionsCheck(var)
        }
    }
    impl ::std::convert::From<SanctionsListCall> for MystikoV2CelerERC20Calls {
        fn from(var: SanctionsListCall) -> Self {
            MystikoV2CelerERC20Calls::SanctionsList(var)
        }
    }
    impl ::std::convert::From<SetAssociatedCommitmentPoolCall> for MystikoV2CelerERC20Calls {
        fn from(var: SetAssociatedCommitmentPoolCall) -> Self {
            MystikoV2CelerERC20Calls::SetAssociatedCommitmentPool(var)
        }
    }
    impl ::std::convert::From<SetBridgeProxyAddressCall> for MystikoV2CelerERC20Calls {
        fn from(var: SetBridgeProxyAddressCall) -> Self {
            MystikoV2CelerERC20Calls::SetBridgeProxyAddress(var)
        }
    }
    impl ::std::convert::From<SetDepositsDisabledCall> for MystikoV2CelerERC20Calls {
        fn from(var: SetDepositsDisabledCall) -> Self {
            MystikoV2CelerERC20Calls::SetDepositsDisabled(var)
        }
    }
    impl ::std::convert::From<SetMaxAmountCall> for MystikoV2CelerERC20Calls {
        fn from(var: SetMaxAmountCall) -> Self {
            MystikoV2CelerERC20Calls::SetMaxAmount(var)
        }
    }
    impl ::std::convert::From<SetMinAmountCall> for MystikoV2CelerERC20Calls {
        fn from(var: SetMinAmountCall) -> Self {
            MystikoV2CelerERC20Calls::SetMinAmount(var)
        }
    }
    impl ::std::convert::From<SetMinBridgeFeeCall> for MystikoV2CelerERC20Calls {
        fn from(var: SetMinBridgeFeeCall) -> Self {
            MystikoV2CelerERC20Calls::SetMinBridgeFee(var)
        }
    }
    impl ::std::convert::From<SetMinExecutorFeeCall> for MystikoV2CelerERC20Calls {
        fn from(var: SetMinExecutorFeeCall) -> Self {
            MystikoV2CelerERC20Calls::SetMinExecutorFee(var)
        }
    }
    impl ::std::convert::From<SetPeerContractCall> for MystikoV2CelerERC20Calls {
        fn from(var: SetPeerContractCall) -> Self {
            MystikoV2CelerERC20Calls::SetPeerContract(var)
        }
    }
    impl ::std::convert::From<SetPeerMinExecutorFeeCall> for MystikoV2CelerERC20Calls {
        fn from(var: SetPeerMinExecutorFeeCall) -> Self {
            MystikoV2CelerERC20Calls::SetPeerMinExecutorFee(var)
        }
    }
    impl ::std::convert::From<SetPeerMinRollupFeeCall> for MystikoV2CelerERC20Calls {
        fn from(var: SetPeerMinRollupFeeCall) -> Self {
            MystikoV2CelerERC20Calls::SetPeerMinRollupFee(var)
        }
    }
    impl ::std::convert::From<UpdateSanctionsListAddressCall> for MystikoV2CelerERC20Calls {
        fn from(var: UpdateSanctionsListAddressCall) -> Self {
            MystikoV2CelerERC20Calls::UpdateSanctionsListAddress(var)
        }
    }
    #[doc = "Container type for all return fields from the `assetDecimals` function with signature `assetDecimals()` and selector `[194, 212, 22, 1]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AssetDecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `assetName` function with signature `assetName()` and selector `[201, 35, 12, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AssetNameReturn(pub String);
    #[doc = "Container type for all return fields from the `assetSymbol` function with signature `assetSymbol()` and selector `[23, 109, 231, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AssetSymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `assetType` function with signature `assetType()` and selector `[63, 227, 52, 122]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AssetTypeReturn(pub u8);
    #[doc = "Container type for all return fields from the `bridgeProxyAddress` function with signature `bridgeProxyAddress()` and selector `[44, 210, 109, 69]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BridgeProxyAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `bridgeType` function with signature `bridgeType()` and selector `[36, 33, 225, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BridgeTypeReturn(pub String);
    #[doc = "Container type for all return fields from the `executeMessage` function with signature `executeMessage(address,uint64,bytes,address)` and selector `[156, 100, 159, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExecuteMessageReturn(pub bool);
    #[doc = "Container type for all return fields from the `getAssociatedCommitmentPool` function with signature `getAssociatedCommitmentPool()` and selector `[221, 172, 93, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAssociatedCommitmentPoolReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getMaxAmount` function with signature `getMaxAmount()` and selector `[11, 169, 89, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMaxAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getMinAmount` function with signature `getMinAmount()` and selector `[207, 199, 226, 218]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMinAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getMinBridgeFee` function with signature `getMinBridgeFee()` and selector `[239, 191, 178, 174]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMinBridgeFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getMinExecutorFee` function with signature `getMinExecutorFee()` and selector `[244, 173, 23, 198]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMinExecutorFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPeerMinExecutorFee` function with signature `getPeerMinExecutorFee()` and selector `[88, 152, 160, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPeerMinExecutorFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPeerMinRollupFee` function with signature `getPeerMinRollupFee()` and selector `[130, 91, 95, 141]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPeerMinRollupFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isDepositsDisabled` function with signature `isDepositsDisabled()` and selector `[237, 110, 163, 58]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsDepositsDisabledReturn(pub bool);
    #[doc = "Container type for all return fields from the `peerChainId` function with signature `peerChainId()` and selector `[205, 252, 238, 186]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PeerChainIdReturn(pub u64);
    #[doc = "Container type for all return fields from the `peerChainName` function with signature `peerChainName()` and selector `[78, 60, 16, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PeerChainNameReturn(pub String);
    #[doc = "Container type for all return fields from the `peerContract` function with signature `peerContract()` and selector `[33, 227, 45, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PeerContractReturn(pub ethers::core::types::Address);
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
    #[doc = "`DepositRequest(uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DepositRequest {
        pub amount: ethers::core::types::U256,
        pub commitment: ethers::core::types::U256,
        pub hash_k: ethers::core::types::U256,
        pub random_s: u128,
        pub encrypted_note: ethers::core::types::Bytes,
        pub bridge_fee: ethers::core::types::U256,
        pub executor_fee: ethers::core::types::U256,
        pub rollup_fee: ethers::core::types::U256,
    }
}
