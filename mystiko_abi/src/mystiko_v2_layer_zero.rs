pub use mystiko_v2_layer_zero::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mystiko_v2_layer_zero {
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
    #[doc = "MystikoV2LayerZero was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"AmountLessThanZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AmountTooLarge\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AmountTooSmall\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BridgeFeeTooFew\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CallIsNotLzApp\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CommitmentHashIncorrect\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"DepositsDisabled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"DestinationChainIsNotTrusted\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ExecutorFeeTooFew\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FromChainIdNotMatched\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FromProxyAddressNotMatched\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"HashKGreaterThanFieldSize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"param\",\"type\":\"string\",\"components\":[]}],\"type\":\"error\",\"name\":\"Invalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MinAmountGreaterThanMaxAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NoStoredMessage\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotChanged\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOperator\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RandomSGreaterThanFieldSize\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RollupFeeToFew\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SanctionedAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CommitmentCrossChain\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"maxAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"minAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DepositAmountLimits\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DepositsDisabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MessageFailed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"minBridgeFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinBridgeFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"minExecutorFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinExecutorFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OperatorChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"peerMinExecutorFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerMinExecutorFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"peerMinRollupFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerMinRollupFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SanctionsCheck\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"sanctions\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SanctionsList\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedRemote\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetType\",\"outputs\":[{\"internalType\":\"enum AssetPool.AssetType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bridgeProxyAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"bridgeType\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newOperator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeOperator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IMystikoBridge.DepositRequest\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"hashK\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"randomS\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"encryptedNote\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bridgeFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"executorFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rollupFee\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disableSanctionsCheck\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableSanctionsCheck\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"failedMessages\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forceResumeReceive\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssociatedCommitmentPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_chainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_configType\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getConfig\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMaxAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinBridgeFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinExecutorFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPeerMinExecutorFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPeerMinRollupFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isDepositsDisabled\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isTrustedRemote\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"localLayerZeroChainId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lzEndpoint\",\"outputs\":[{\"internalType\":\"contract ILayerZeroEndpoint\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"lzReceive\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"nonblockingLzReceive\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerChainId\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerChainName\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerLayerZeroChainId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"retryMessage\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sanctionsCheck\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sanctionsList\",\"outputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_commitmentPoolAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAssociatedCommitmentPool\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_bridgeProxyAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBridgeProxyAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_chainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_configType\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_config\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setConfig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDepositsDisabled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_lzChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_lzEndpoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minBridgeFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinBridgeFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minExecutorFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinExecutorFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"_peerChainId\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_peerChainName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_peerContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPeerContract\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_peerMinExecutorFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPeerMinExecutorFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_peerMinRollupFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPeerMinRollupFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReceiveVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSendVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_peerLayerZeroChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_peerAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedRemote\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedRemoteLookup\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_maxAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_minAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateDepositAmountLimits\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"_sanction\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateSanctionsListAddress\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MYSTIKOV2LAYERZERO_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct MystikoV2LayerZero<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MystikoV2LayerZero<M> {
        fn clone(&self) -> Self {
            MystikoV2LayerZero(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MystikoV2LayerZero<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MystikoV2LayerZero<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MystikoV2LayerZero))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MystikoV2LayerZero<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MYSTIKOV2LAYERZERO_ABI.clone(), client)
                .into()
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
        #[doc = "Calls the contract's `failedMessages` (0x5b8c41e6) function"]
        pub fn failed_messages(
            &self,
            p0: u16,
            p1: ethers::core::types::Bytes,
            p2: u64,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([91, 140, 65, 230], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forceResumeReceive` (0x42d65a8d) function"]
        pub fn force_resume_receive(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 214, 90, 141], (src_chain_id, src_address))
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
        #[doc = "Calls the contract's `getConfig` (0xf5ecbdbc) function"]
        pub fn get_config(
            &self,
            version: u16,
            chain_id: u16,
            p2: ethers::core::types::Address,
            config_type: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([245, 236, 189, 188], (version, chain_id, p2, config_type))
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
        #[doc = "Calls the contract's `isTrustedRemote` (0x3d8b38f6) function"]
        pub fn is_trusted_remote(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 139, 56, 246], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `localLayerZeroChainId` (0x302d5f4b) function"]
        pub fn local_layer_zero_chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([48, 45, 95, 75], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lzEndpoint` (0xb353aaa7) function"]
        pub fn lz_endpoint(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([179, 83, 170, 167], ())
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `nonblockingLzReceive` (0x66ad5c8a) function"]
        pub fn nonblocking_lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
            nonce: u64,
            payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [102, 173, 92, 138],
                    (src_chain_id, src_address, nonce, payload),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
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
        #[doc = "Calls the contract's `peerLayerZeroChainId` (0x0097a063) function"]
        pub fn peer_layer_zero_chain_id(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([0, 151, 160, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `retryMessage` (0xd1deba1f) function"]
        pub fn retry_message(
            &self,
            src_chain_id: u16,
            src_address: ethers::core::types::Bytes,
            nonce: u64,
            payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [209, 222, 186, 31],
                    (src_chain_id, src_address, nonce, payload),
                )
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
        #[doc = "Calls the contract's `setConfig` (0xcbed8b9c) function"]
        pub fn set_config(
            &self,
            version: u16,
            chain_id: u16,
            config_type: ethers::core::types::U256,
            config: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [203, 237, 139, 156],
                    (version, chain_id, config_type, config),
                )
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
        #[doc = "Calls the contract's `setEndpoint` (0x4ee7ded6) function"]
        pub fn set_endpoint(
            &self,
            lz_chain_id: u16,
            lz_endpoint: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 231, 222, 214], (lz_chain_id, lz_endpoint))
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
        #[doc = "Calls the contract's `setReceiveVersion` (0x10ddb137) function"]
        pub fn set_receive_version(
            &self,
            version: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 221, 177, 55], version)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSendVersion` (0x07e0db17) function"]
        pub fn set_send_version(
            &self,
            version: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 224, 219, 23], version)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTrustedRemote` (0xeb8d72b7) function"]
        pub fn set_trusted_remote(
            &self,
            peer_layer_zero_chain_id: u16,
            peer_address: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [235, 141, 114, 183],
                    (peer_layer_zero_chain_id, peer_address),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedRemoteLookup` (0x7533d788) function"]
        pub fn trusted_remote_lookup(
            &self,
            p0: u16,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([117, 51, 215, 136], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateDepositAmountLimits` (0xe8183c44) function"]
        pub fn update_deposit_amount_limits(
            &self,
            max_amount: ethers::core::types::U256,
            min_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 24, 60, 68], (max_amount, min_amount))
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
        #[doc = "Gets the contract's `DepositAmountLimits` event"]
        pub fn deposit_amount_limits_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DepositAmountLimitsFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DepositsDisabled` event"]
        pub fn deposits_disabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DepositsDisabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MessageFailed` event"]
        pub fn message_failed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MessageFailedFilter> {
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
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
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
        #[doc = "Gets the contract's `SetTrustedRemote` event"]
        pub fn set_trusted_remote_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetTrustedRemoteFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MystikoV2LayerZeroEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MystikoV2LayerZero<M>
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(name = "BridgeFeeTooFew", abi = "BridgeFeeTooFew()")]
    pub struct BridgeFeeTooFew;
    #[doc = "Custom Error type `CallIsNotLzApp` with signature `CallIsNotLzApp()` and selector `[227, 234, 29, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(name = "CallIsNotLzApp", abi = "CallIsNotLzApp()")]
    pub struct CallIsNotLzApp;
    #[doc = "Custom Error type `CommitmentHashIncorrect` with signature `CommitmentHashIncorrect()` and selector `[55, 245, 68, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(name = "DepositsDisabled", abi = "DepositsDisabled()")]
    pub struct DepositsDisabled;
    #[doc = "Custom Error type `DestinationChainIsNotTrusted` with signature `DestinationChainIsNotTrusted()` and selector `[2, 11, 53, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(
        name = "DestinationChainIsNotTrusted",
        abi = "DestinationChainIsNotTrusted()"
    )]
    pub struct DestinationChainIsNotTrusted;
    #[doc = "Custom Error type `ExecutorFeeTooFew` with signature `ExecutorFeeTooFew()` and selector `[171, 77, 173, 66]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(name = "Invalid", abi = "Invalid(string)")]
    pub struct Invalid {
        pub param: String,
    }
    #[doc = "Custom Error type `MinAmountGreaterThanMaxAmount` with signature `MinAmountGreaterThanMaxAmount()` and selector `[192, 7, 208, 66]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(
        name = "MinAmountGreaterThanMaxAmount",
        abi = "MinAmountGreaterThanMaxAmount()"
    )]
    pub struct MinAmountGreaterThanMaxAmount;
    #[doc = "Custom Error type `NoStoredMessage` with signature `NoStoredMessage()` and selector `[174, 91, 38, 20]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(name = "NoStoredMessage", abi = "NoStoredMessage()")]
    pub struct NoStoredMessage;
    #[doc = "Custom Error type `NotChanged` with signature `NotChanged()` and selector `[54, 161, 195, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[etherror(name = "SanctionedAddress", abi = "SanctionedAddress()")]
    pub struct SanctionedAddress;
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum MystikoV2LayerZeroErrors {
        AmountLessThanZero(AmountLessThanZero),
        AmountTooLarge(AmountTooLarge),
        AmountTooSmall(AmountTooSmall),
        BridgeFeeTooFew(BridgeFeeTooFew),
        CallIsNotLzApp(CallIsNotLzApp),
        CommitmentHashIncorrect(CommitmentHashIncorrect),
        DepositsDisabled(DepositsDisabled),
        DestinationChainIsNotTrusted(DestinationChainIsNotTrusted),
        ExecutorFeeTooFew(ExecutorFeeTooFew),
        FromChainIdNotMatched(FromChainIdNotMatched),
        FromProxyAddressNotMatched(FromProxyAddressNotMatched),
        HashKGreaterThanFieldSize(HashKGreaterThanFieldSize),
        Invalid(Invalid),
        MinAmountGreaterThanMaxAmount(MinAmountGreaterThanMaxAmount),
        NoStoredMessage(NoStoredMessage),
        NotChanged(NotChanged),
        OnlyOperator(OnlyOperator),
        RandomSGreaterThanFieldSize(RandomSGreaterThanFieldSize),
        RollupFeeToFew(RollupFeeToFew),
        SanctionedAddress(SanctionedAddress),
    }
    impl ethers::core::abi::AbiDecode for MystikoV2LayerZeroErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AmountLessThanZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::AmountLessThanZero(decoded));
            }
            if let Ok(decoded) =
                <AmountTooLarge as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::AmountTooLarge(decoded));
            }
            if let Ok(decoded) =
                <AmountTooSmall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::AmountTooSmall(decoded));
            }
            if let Ok(decoded) =
                <BridgeFeeTooFew as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::BridgeFeeTooFew(decoded));
            }
            if let Ok(decoded) =
                <CallIsNotLzApp as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::CallIsNotLzApp(decoded));
            }
            if let Ok(decoded) =
                <CommitmentHashIncorrect as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::CommitmentHashIncorrect(decoded));
            }
            if let Ok(decoded) =
                <DepositsDisabled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::DepositsDisabled(decoded));
            }
            if let Ok(decoded) =
                <DestinationChainIsNotTrusted as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2LayerZeroErrors::DestinationChainIsNotTrusted(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ExecutorFeeTooFew as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::ExecutorFeeTooFew(decoded));
            }
            if let Ok(decoded) =
                <FromChainIdNotMatched as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::FromChainIdNotMatched(decoded));
            }
            if let Ok(decoded) =
                <FromProxyAddressNotMatched as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::FromProxyAddressNotMatched(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <HashKGreaterThanFieldSize as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::HashKGreaterThanFieldSize(decoded));
            }
            if let Ok(decoded) = <Invalid as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MystikoV2LayerZeroErrors::Invalid(decoded));
            }
            if let Ok(decoded) =
                <MinAmountGreaterThanMaxAmount as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2LayerZeroErrors::MinAmountGreaterThanMaxAmount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <NoStoredMessage as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::NoStoredMessage(decoded));
            }
            if let Ok(decoded) = <NotChanged as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::NotChanged(decoded));
            }
            if let Ok(decoded) =
                <OnlyOperator as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::OnlyOperator(decoded));
            }
            if let Ok(decoded) =
                <RandomSGreaterThanFieldSize as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::RandomSGreaterThanFieldSize(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RollupFeeToFew as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::RollupFeeToFew(decoded));
            }
            if let Ok(decoded) =
                <SanctionedAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroErrors::SanctionedAddress(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MystikoV2LayerZeroErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                MystikoV2LayerZeroErrors::AmountLessThanZero(element) => element.encode(),
                MystikoV2LayerZeroErrors::AmountTooLarge(element) => element.encode(),
                MystikoV2LayerZeroErrors::AmountTooSmall(element) => element.encode(),
                MystikoV2LayerZeroErrors::BridgeFeeTooFew(element) => element.encode(),
                MystikoV2LayerZeroErrors::CallIsNotLzApp(element) => element.encode(),
                MystikoV2LayerZeroErrors::CommitmentHashIncorrect(element) => element.encode(),
                MystikoV2LayerZeroErrors::DepositsDisabled(element) => element.encode(),
                MystikoV2LayerZeroErrors::DestinationChainIsNotTrusted(element) => element.encode(),
                MystikoV2LayerZeroErrors::ExecutorFeeTooFew(element) => element.encode(),
                MystikoV2LayerZeroErrors::FromChainIdNotMatched(element) => element.encode(),
                MystikoV2LayerZeroErrors::FromProxyAddressNotMatched(element) => element.encode(),
                MystikoV2LayerZeroErrors::HashKGreaterThanFieldSize(element) => element.encode(),
                MystikoV2LayerZeroErrors::Invalid(element) => element.encode(),
                MystikoV2LayerZeroErrors::MinAmountGreaterThanMaxAmount(element) => {
                    element.encode()
                }
                MystikoV2LayerZeroErrors::NoStoredMessage(element) => element.encode(),
                MystikoV2LayerZeroErrors::NotChanged(element) => element.encode(),
                MystikoV2LayerZeroErrors::OnlyOperator(element) => element.encode(),
                MystikoV2LayerZeroErrors::RandomSGreaterThanFieldSize(element) => element.encode(),
                MystikoV2LayerZeroErrors::RollupFeeToFew(element) => element.encode(),
                MystikoV2LayerZeroErrors::SanctionedAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MystikoV2LayerZeroErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoV2LayerZeroErrors::AmountLessThanZero(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::AmountTooLarge(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::AmountTooSmall(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::BridgeFeeTooFew(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::CallIsNotLzApp(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::CommitmentHashIncorrect(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::DepositsDisabled(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::DestinationChainIsNotTrusted(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::ExecutorFeeTooFew(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::FromChainIdNotMatched(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::FromProxyAddressNotMatched(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::HashKGreaterThanFieldSize(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::Invalid(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::MinAmountGreaterThanMaxAmount(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::NoStoredMessage(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::NotChanged(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::OnlyOperator(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::RandomSGreaterThanFieldSize(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::RollupFeeToFew(element) => element.fmt(f),
                MystikoV2LayerZeroErrors::SanctionedAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AmountLessThanZero> for MystikoV2LayerZeroErrors {
        fn from(var: AmountLessThanZero) -> Self {
            MystikoV2LayerZeroErrors::AmountLessThanZero(var)
        }
    }
    impl ::std::convert::From<AmountTooLarge> for MystikoV2LayerZeroErrors {
        fn from(var: AmountTooLarge) -> Self {
            MystikoV2LayerZeroErrors::AmountTooLarge(var)
        }
    }
    impl ::std::convert::From<AmountTooSmall> for MystikoV2LayerZeroErrors {
        fn from(var: AmountTooSmall) -> Self {
            MystikoV2LayerZeroErrors::AmountTooSmall(var)
        }
    }
    impl ::std::convert::From<BridgeFeeTooFew> for MystikoV2LayerZeroErrors {
        fn from(var: BridgeFeeTooFew) -> Self {
            MystikoV2LayerZeroErrors::BridgeFeeTooFew(var)
        }
    }
    impl ::std::convert::From<CallIsNotLzApp> for MystikoV2LayerZeroErrors {
        fn from(var: CallIsNotLzApp) -> Self {
            MystikoV2LayerZeroErrors::CallIsNotLzApp(var)
        }
    }
    impl ::std::convert::From<CommitmentHashIncorrect> for MystikoV2LayerZeroErrors {
        fn from(var: CommitmentHashIncorrect) -> Self {
            MystikoV2LayerZeroErrors::CommitmentHashIncorrect(var)
        }
    }
    impl ::std::convert::From<DepositsDisabled> for MystikoV2LayerZeroErrors {
        fn from(var: DepositsDisabled) -> Self {
            MystikoV2LayerZeroErrors::DepositsDisabled(var)
        }
    }
    impl ::std::convert::From<DestinationChainIsNotTrusted> for MystikoV2LayerZeroErrors {
        fn from(var: DestinationChainIsNotTrusted) -> Self {
            MystikoV2LayerZeroErrors::DestinationChainIsNotTrusted(var)
        }
    }
    impl ::std::convert::From<ExecutorFeeTooFew> for MystikoV2LayerZeroErrors {
        fn from(var: ExecutorFeeTooFew) -> Self {
            MystikoV2LayerZeroErrors::ExecutorFeeTooFew(var)
        }
    }
    impl ::std::convert::From<FromChainIdNotMatched> for MystikoV2LayerZeroErrors {
        fn from(var: FromChainIdNotMatched) -> Self {
            MystikoV2LayerZeroErrors::FromChainIdNotMatched(var)
        }
    }
    impl ::std::convert::From<FromProxyAddressNotMatched> for MystikoV2LayerZeroErrors {
        fn from(var: FromProxyAddressNotMatched) -> Self {
            MystikoV2LayerZeroErrors::FromProxyAddressNotMatched(var)
        }
    }
    impl ::std::convert::From<HashKGreaterThanFieldSize> for MystikoV2LayerZeroErrors {
        fn from(var: HashKGreaterThanFieldSize) -> Self {
            MystikoV2LayerZeroErrors::HashKGreaterThanFieldSize(var)
        }
    }
    impl ::std::convert::From<Invalid> for MystikoV2LayerZeroErrors {
        fn from(var: Invalid) -> Self {
            MystikoV2LayerZeroErrors::Invalid(var)
        }
    }
    impl ::std::convert::From<MinAmountGreaterThanMaxAmount> for MystikoV2LayerZeroErrors {
        fn from(var: MinAmountGreaterThanMaxAmount) -> Self {
            MystikoV2LayerZeroErrors::MinAmountGreaterThanMaxAmount(var)
        }
    }
    impl ::std::convert::From<NoStoredMessage> for MystikoV2LayerZeroErrors {
        fn from(var: NoStoredMessage) -> Self {
            MystikoV2LayerZeroErrors::NoStoredMessage(var)
        }
    }
    impl ::std::convert::From<NotChanged> for MystikoV2LayerZeroErrors {
        fn from(var: NotChanged) -> Self {
            MystikoV2LayerZeroErrors::NotChanged(var)
        }
    }
    impl ::std::convert::From<OnlyOperator> for MystikoV2LayerZeroErrors {
        fn from(var: OnlyOperator) -> Self {
            MystikoV2LayerZeroErrors::OnlyOperator(var)
        }
    }
    impl ::std::convert::From<RandomSGreaterThanFieldSize> for MystikoV2LayerZeroErrors {
        fn from(var: RandomSGreaterThanFieldSize) -> Self {
            MystikoV2LayerZeroErrors::RandomSGreaterThanFieldSize(var)
        }
    }
    impl ::std::convert::From<RollupFeeToFew> for MystikoV2LayerZeroErrors {
        fn from(var: RollupFeeToFew) -> Self {
            MystikoV2LayerZeroErrors::RollupFeeToFew(var)
        }
    }
    impl ::std::convert::From<SanctionedAddress> for MystikoV2LayerZeroErrors {
        fn from(var: SanctionedAddress) -> Self {
            MystikoV2LayerZeroErrors::SanctionedAddress(var)
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
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethevent(
        name = "DepositAmountLimits",
        abi = "DepositAmountLimits(uint256,uint256)"
    )]
    pub struct DepositAmountLimitsFilter {
        pub max_amount: ethers::core::types::U256,
        pub min_amount: ethers::core::types::U256,
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
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethevent(
        name = "MessageFailed",
        abi = "MessageFailed(uint16,bytes,uint64,bytes)"
    )]
    pub struct MessageFailedFilter {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub nonce: u64,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethevent(name = "SanctionsList", abi = "SanctionsList(address)")]
    pub struct SanctionsListFilter {
        pub sanctions: ethers::core::types::Address,
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
    #[ethevent(name = "SetTrustedRemote", abi = "SetTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteFilter {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
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
    pub enum MystikoV2LayerZeroEvents {
        CommitmentCrossChainFilter(CommitmentCrossChainFilter),
        DepositAmountLimitsFilter(DepositAmountLimitsFilter),
        DepositsDisabledFilter(DepositsDisabledFilter),
        MessageFailedFilter(MessageFailedFilter),
        MinBridgeFeeFilter(MinBridgeFeeFilter),
        MinExecutorFeeFilter(MinExecutorFeeFilter),
        OperatorChangedFilter(OperatorChangedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PeerMinExecutorFeeFilter(PeerMinExecutorFeeFilter),
        PeerMinRollupFeeFilter(PeerMinRollupFeeFilter),
        SanctionsCheckFilter(SanctionsCheckFilter),
        SanctionsListFilter(SanctionsListFilter),
        SetTrustedRemoteFilter(SetTrustedRemoteFilter),
    }
    impl ethers::contract::EthLogDecode for MystikoV2LayerZeroEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CommitmentCrossChainFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::CommitmentCrossChainFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = DepositAmountLimitsFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::DepositAmountLimitsFilter(decoded));
            }
            if let Ok(decoded) = DepositsDisabledFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::DepositsDisabledFilter(decoded));
            }
            if let Ok(decoded) = MessageFailedFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::MessageFailedFilter(decoded));
            }
            if let Ok(decoded) = MinBridgeFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::MinBridgeFeeFilter(decoded));
            }
            if let Ok(decoded) = MinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::MinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PeerMinExecutorFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::PeerMinExecutorFeeFilter(decoded));
            }
            if let Ok(decoded) = PeerMinRollupFeeFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::PeerMinRollupFeeFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::SanctionsListFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedRemoteFilter::decode_log(log) {
                return Ok(MystikoV2LayerZeroEvents::SetTrustedRemoteFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MystikoV2LayerZeroEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoV2LayerZeroEvents::CommitmentCrossChainFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::DepositAmountLimitsFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::DepositsDisabledFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::MessageFailedFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::MinBridgeFeeFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::MinExecutorFeeFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::OperatorChangedFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::PeerMinExecutorFeeFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::PeerMinRollupFeeFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::SanctionsCheckFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::SanctionsListFilter(element) => element.fmt(f),
                MystikoV2LayerZeroEvents::SetTrustedRemoteFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `assetType` function with signature `assetType()` and selector `[63, 227, 52, 122]`"]
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(name = "enableSanctionsCheck", abi = "enableSanctionsCheck()")]
    pub struct EnableSanctionsCheckCall;
    #[doc = "Container type for all input parameters for the `failedMessages` function with signature `failedMessages(uint16,bytes,uint64)` and selector `[91, 140, 65, 230]`"]
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
    #[ethcall(name = "failedMessages", abi = "failedMessages(uint16,bytes,uint64)")]
    pub struct FailedMessagesCall(pub u16, pub ethers::core::types::Bytes, pub u64);
    #[doc = "Container type for all input parameters for the `forceResumeReceive` function with signature `forceResumeReceive(uint16,bytes)` and selector `[66, 214, 90, 141]`"]
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
    #[ethcall(name = "forceResumeReceive", abi = "forceResumeReceive(uint16,bytes)")]
    pub struct ForceResumeReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getAssociatedCommitmentPool` function with signature `getAssociatedCommitmentPool()` and selector `[221, 172, 93, 193]`"]
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
        name = "getAssociatedCommitmentPool",
        abi = "getAssociatedCommitmentPool()"
    )]
    pub struct GetAssociatedCommitmentPoolCall;
    #[doc = "Container type for all input parameters for the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `[245, 236, 189, 188]`"]
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
    #[ethcall(name = "getConfig", abi = "getConfig(uint16,uint16,address,uint256)")]
    pub struct GetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub p2: ethers::core::types::Address,
        pub config_type: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getMaxAmount` function with signature `getMaxAmount()` and selector `[11, 169, 89, 9]`"]
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(name = "isDepositsDisabled", abi = "isDepositsDisabled()")]
    pub struct IsDepositsDisabledCall;
    #[doc = "Container type for all input parameters for the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `[61, 139, 56, 246]`"]
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
    #[ethcall(name = "isTrustedRemote", abi = "isTrustedRemote(uint16,bytes)")]
    pub struct IsTrustedRemoteCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `localLayerZeroChainId` function with signature `localLayerZeroChainId()` and selector `[48, 45, 95, 75]`"]
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
    #[ethcall(name = "localLayerZeroChainId", abi = "localLayerZeroChainId()")]
    pub struct LocalLayerZeroChainIdCall;
    #[doc = "Container type for all input parameters for the `lzEndpoint` function with signature `lzEndpoint()` and selector `[179, 83, 170, 167]`"]
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
    #[ethcall(name = "lzEndpoint", abi = "lzEndpoint()")]
    pub struct LzEndpointCall;
    #[doc = "Container type for all input parameters for the `lzReceive` function with signature `lzReceive(uint16,bytes,uint64,bytes)` and selector `[0, 29, 53, 103]`"]
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
    #[ethcall(name = "lzReceive", abi = "lzReceive(uint16,bytes,uint64,bytes)")]
    pub struct LzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `nonblockingLzReceive` function with signature `nonblockingLzReceive(uint16,bytes,uint64,bytes)` and selector `[102, 173, 92, 138]`"]
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
        name = "nonblockingLzReceive",
        abi = "nonblockingLzReceive(uint16,bytes,uint64,bytes)"
    )]
    pub struct NonblockingLzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `peerChainId` function with signature `peerChainId()` and selector `[205, 252, 238, 186]`"]
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(name = "peerContract", abi = "peerContract()")]
    pub struct PeerContractCall;
    #[doc = "Container type for all input parameters for the `peerLayerZeroChainId` function with signature `peerLayerZeroChainId()` and selector `[0, 151, 160, 99]`"]
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
    #[ethcall(name = "peerLayerZeroChainId", abi = "peerLayerZeroChainId()")]
    pub struct PeerLayerZeroChainIdCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `retryMessage` function with signature `retryMessage(uint16,bytes,uint64,bytes)` and selector `[209, 222, 186, 31]`"]
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
    #[ethcall(name = "retryMessage", abi = "retryMessage(uint16,bytes,uint64,bytes)")]
    pub struct RetryMessageCall {
        pub src_chain_id: u16,
        pub src_address: ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `sanctionsCheck` function with signature `sanctionsCheck()` and selector `[177, 195, 148, 34]`"]
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(name = "setBridgeProxyAddress", abi = "setBridgeProxyAddress(address)")]
    pub struct SetBridgeProxyAddressCall {
        pub bridge_proxy_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setConfig` function with signature `setConfig(uint16,uint16,uint256,bytes)` and selector `[203, 237, 139, 156]`"]
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
    #[ethcall(name = "setConfig", abi = "setConfig(uint16,uint16,uint256,bytes)")]
    pub struct SetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub config_type: ethers::core::types::U256,
        pub config: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setDepositsDisabled` function with signature `setDepositsDisabled(bool)` and selector `[234, 12, 222, 133]`"]
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
    #[ethcall(name = "setDepositsDisabled", abi = "setDepositsDisabled(bool)")]
    pub struct SetDepositsDisabledCall {
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `setEndpoint` function with signature `setEndpoint(uint16,address)` and selector `[78, 231, 222, 214]`"]
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
    #[ethcall(name = "setEndpoint", abi = "setEndpoint(uint16,address)")]
    pub struct SetEndpointCall {
        pub lz_chain_id: u16,
        pub lz_endpoint: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setMinBridgeFee` function with signature `setMinBridgeFee(uint256)` and selector `[25, 231, 93, 110]`"]
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
        Default,
    )]
    #[ethcall(name = "setPeerMinRollupFee", abi = "setPeerMinRollupFee(uint256)")]
    pub struct SetPeerMinRollupFeeCall {
        pub peer_min_rollup_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setReceiveVersion` function with signature `setReceiveVersion(uint16)` and selector `[16, 221, 177, 55]`"]
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
    #[ethcall(name = "setReceiveVersion", abi = "setReceiveVersion(uint16)")]
    pub struct SetReceiveVersionCall {
        pub version: u16,
    }
    #[doc = "Container type for all input parameters for the `setSendVersion` function with signature `setSendVersion(uint16)` and selector `[7, 224, 219, 23]`"]
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
    #[ethcall(name = "setSendVersion", abi = "setSendVersion(uint16)")]
    pub struct SetSendVersionCall {
        pub version: u16,
    }
    #[doc = "Container type for all input parameters for the `setTrustedRemote` function with signature `setTrustedRemote(uint16,bytes)` and selector `[235, 141, 114, 183]`"]
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
    #[ethcall(name = "setTrustedRemote", abi = "setTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteCall {
        pub peer_layer_zero_chain_id: u16,
        pub peer_address: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `[117, 51, 215, 136]`"]
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
    #[ethcall(name = "trustedRemoteLookup", abi = "trustedRemoteLookup(uint16)")]
    pub struct TrustedRemoteLookupCall(pub u16);
    #[doc = "Container type for all input parameters for the `updateDepositAmountLimits` function with signature `updateDepositAmountLimits(uint256,uint256)` and selector `[232, 24, 60, 68]`"]
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
        name = "updateDepositAmountLimits",
        abi = "updateDepositAmountLimits(uint256,uint256)"
    )]
    pub struct UpdateDepositAmountLimitsCall {
        pub max_amount: ethers::core::types::U256,
        pub min_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateSanctionsListAddress` function with signature `updateSanctionsListAddress(address)` and selector `[48, 244, 156, 172]`"]
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
        name = "updateSanctionsListAddress",
        abi = "updateSanctionsListAddress(address)"
    )]
    pub struct UpdateSanctionsListAddressCall {
        pub sanction: ethers::core::types::Address,
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
    pub enum MystikoV2LayerZeroCalls {
        AssetType(AssetTypeCall),
        BridgeProxyAddress(BridgeProxyAddressCall),
        BridgeType(BridgeTypeCall),
        ChangeOperator(ChangeOperatorCall),
        Deposit(DepositCall),
        DisableSanctionsCheck(DisableSanctionsCheckCall),
        EnableSanctionsCheck(EnableSanctionsCheckCall),
        FailedMessages(FailedMessagesCall),
        ForceResumeReceive(ForceResumeReceiveCall),
        GetAssociatedCommitmentPool(GetAssociatedCommitmentPoolCall),
        GetConfig(GetConfigCall),
        GetMaxAmount(GetMaxAmountCall),
        GetMinAmount(GetMinAmountCall),
        GetMinBridgeFee(GetMinBridgeFeeCall),
        GetMinExecutorFee(GetMinExecutorFeeCall),
        GetPeerMinExecutorFee(GetPeerMinExecutorFeeCall),
        GetPeerMinRollupFee(GetPeerMinRollupFeeCall),
        IsDepositsDisabled(IsDepositsDisabledCall),
        IsTrustedRemote(IsTrustedRemoteCall),
        LocalLayerZeroChainId(LocalLayerZeroChainIdCall),
        LzEndpoint(LzEndpointCall),
        LzReceive(LzReceiveCall),
        NonblockingLzReceive(NonblockingLzReceiveCall),
        Owner(OwnerCall),
        PeerChainId(PeerChainIdCall),
        PeerChainName(PeerChainNameCall),
        PeerContract(PeerContractCall),
        PeerLayerZeroChainId(PeerLayerZeroChainIdCall),
        RenounceOwnership(RenounceOwnershipCall),
        RetryMessage(RetryMessageCall),
        SanctionsCheck(SanctionsCheckCall),
        SanctionsList(SanctionsListCall),
        SetAssociatedCommitmentPool(SetAssociatedCommitmentPoolCall),
        SetBridgeProxyAddress(SetBridgeProxyAddressCall),
        SetConfig(SetConfigCall),
        SetDepositsDisabled(SetDepositsDisabledCall),
        SetEndpoint(SetEndpointCall),
        SetMinBridgeFee(SetMinBridgeFeeCall),
        SetMinExecutorFee(SetMinExecutorFeeCall),
        SetPeerContract(SetPeerContractCall),
        SetPeerMinExecutorFee(SetPeerMinExecutorFeeCall),
        SetPeerMinRollupFee(SetPeerMinRollupFeeCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetSendVersion(SetSendVersionCall),
        SetTrustedRemote(SetTrustedRemoteCall),
        TransferOwnership(TransferOwnershipCall),
        TrustedRemoteLookup(TrustedRemoteLookupCall),
        UpdateDepositAmountLimits(UpdateDepositAmountLimitsCall),
        UpdateSanctionsListAddress(UpdateSanctionsListAddressCall),
    }
    impl ethers::core::abi::AbiDecode for MystikoV2LayerZeroCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AssetTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::AssetType(decoded));
            }
            if let Ok(decoded) =
                <BridgeProxyAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::BridgeProxyAddress(decoded));
            }
            if let Ok(decoded) =
                <BridgeTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::BridgeType(decoded));
            }
            if let Ok(decoded) =
                <ChangeOperatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::ChangeOperator(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DisableSanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::DisableSanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <EnableSanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::EnableSanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <FailedMessagesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::FailedMessages(decoded));
            }
            if let Ok(decoded) =
                <ForceResumeReceiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::ForceResumeReceive(decoded));
            }
            if let Ok(decoded) =
                <GetAssociatedCommitmentPoolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2LayerZeroCalls::GetAssociatedCommitmentPool(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::GetConfig(decoded));
            }
            if let Ok(decoded) =
                <GetMaxAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::GetMaxAmount(decoded));
            }
            if let Ok(decoded) =
                <GetMinAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::GetMinAmount(decoded));
            }
            if let Ok(decoded) =
                <GetMinBridgeFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::GetMinBridgeFee(decoded));
            }
            if let Ok(decoded) =
                <GetMinExecutorFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::GetMinExecutorFee(decoded));
            }
            if let Ok(decoded) =
                <GetPeerMinExecutorFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::GetPeerMinExecutorFee(decoded));
            }
            if let Ok(decoded) =
                <GetPeerMinRollupFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::GetPeerMinRollupFee(decoded));
            }
            if let Ok(decoded) =
                <IsDepositsDisabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::IsDepositsDisabled(decoded));
            }
            if let Ok(decoded) =
                <IsTrustedRemoteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::IsTrustedRemote(decoded));
            }
            if let Ok(decoded) =
                <LocalLayerZeroChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::LocalLayerZeroChainId(decoded));
            }
            if let Ok(decoded) =
                <LzEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::LzEndpoint(decoded));
            }
            if let Ok(decoded) =
                <LzReceiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::LzReceive(decoded));
            }
            if let Ok(decoded) =
                <NonblockingLzReceiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::NonblockingLzReceive(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PeerChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::PeerChainId(decoded));
            }
            if let Ok(decoded) =
                <PeerChainNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::PeerChainName(decoded));
            }
            if let Ok(decoded) =
                <PeerContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::PeerContract(decoded));
            }
            if let Ok(decoded) =
                <PeerLayerZeroChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::PeerLayerZeroChainId(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RetryMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::RetryMessage(decoded));
            }
            if let Ok(decoded) =
                <SanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <SanctionsListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SanctionsList(decoded));
            }
            if let Ok(decoded) =
                <SetAssociatedCommitmentPoolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2LayerZeroCalls::SetAssociatedCommitmentPool(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetBridgeProxyAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetBridgeProxyAddress(decoded));
            }
            if let Ok(decoded) =
                <SetConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetConfig(decoded));
            }
            if let Ok(decoded) =
                <SetDepositsDisabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetDepositsDisabled(decoded));
            }
            if let Ok(decoded) =
                <SetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <SetMinBridgeFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetMinBridgeFee(decoded));
            }
            if let Ok(decoded) =
                <SetMinExecutorFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetMinExecutorFee(decoded));
            }
            if let Ok(decoded) =
                <SetPeerContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetPeerContract(decoded));
            }
            if let Ok(decoded) =
                <SetPeerMinExecutorFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetPeerMinExecutorFee(decoded));
            }
            if let Ok(decoded) =
                <SetPeerMinRollupFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetPeerMinRollupFee(decoded));
            }
            if let Ok(decoded) =
                <SetReceiveVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetReceiveVersion(decoded));
            }
            if let Ok(decoded) =
                <SetSendVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetSendVersion(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedRemoteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::SetTrustedRemote(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TrustedRemoteLookupCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LayerZeroCalls::TrustedRemoteLookup(decoded));
            }
            if let Ok(decoded) =
                <UpdateDepositAmountLimitsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2LayerZeroCalls::UpdateDepositAmountLimits(decoded));
            }
            if let Ok(decoded) =
                <UpdateSanctionsListAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2LayerZeroCalls::UpdateSanctionsListAddress(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MystikoV2LayerZeroCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MystikoV2LayerZeroCalls::AssetType(element) => element.encode(),
                MystikoV2LayerZeroCalls::BridgeProxyAddress(element) => element.encode(),
                MystikoV2LayerZeroCalls::BridgeType(element) => element.encode(),
                MystikoV2LayerZeroCalls::ChangeOperator(element) => element.encode(),
                MystikoV2LayerZeroCalls::Deposit(element) => element.encode(),
                MystikoV2LayerZeroCalls::DisableSanctionsCheck(element) => element.encode(),
                MystikoV2LayerZeroCalls::EnableSanctionsCheck(element) => element.encode(),
                MystikoV2LayerZeroCalls::FailedMessages(element) => element.encode(),
                MystikoV2LayerZeroCalls::ForceResumeReceive(element) => element.encode(),
                MystikoV2LayerZeroCalls::GetAssociatedCommitmentPool(element) => element.encode(),
                MystikoV2LayerZeroCalls::GetConfig(element) => element.encode(),
                MystikoV2LayerZeroCalls::GetMaxAmount(element) => element.encode(),
                MystikoV2LayerZeroCalls::GetMinAmount(element) => element.encode(),
                MystikoV2LayerZeroCalls::GetMinBridgeFee(element) => element.encode(),
                MystikoV2LayerZeroCalls::GetMinExecutorFee(element) => element.encode(),
                MystikoV2LayerZeroCalls::GetPeerMinExecutorFee(element) => element.encode(),
                MystikoV2LayerZeroCalls::GetPeerMinRollupFee(element) => element.encode(),
                MystikoV2LayerZeroCalls::IsDepositsDisabled(element) => element.encode(),
                MystikoV2LayerZeroCalls::IsTrustedRemote(element) => element.encode(),
                MystikoV2LayerZeroCalls::LocalLayerZeroChainId(element) => element.encode(),
                MystikoV2LayerZeroCalls::LzEndpoint(element) => element.encode(),
                MystikoV2LayerZeroCalls::LzReceive(element) => element.encode(),
                MystikoV2LayerZeroCalls::NonblockingLzReceive(element) => element.encode(),
                MystikoV2LayerZeroCalls::Owner(element) => element.encode(),
                MystikoV2LayerZeroCalls::PeerChainId(element) => element.encode(),
                MystikoV2LayerZeroCalls::PeerChainName(element) => element.encode(),
                MystikoV2LayerZeroCalls::PeerContract(element) => element.encode(),
                MystikoV2LayerZeroCalls::PeerLayerZeroChainId(element) => element.encode(),
                MystikoV2LayerZeroCalls::RenounceOwnership(element) => element.encode(),
                MystikoV2LayerZeroCalls::RetryMessage(element) => element.encode(),
                MystikoV2LayerZeroCalls::SanctionsCheck(element) => element.encode(),
                MystikoV2LayerZeroCalls::SanctionsList(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetAssociatedCommitmentPool(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetBridgeProxyAddress(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetConfig(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetDepositsDisabled(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetEndpoint(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetMinBridgeFee(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetMinExecutorFee(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetPeerContract(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetPeerMinExecutorFee(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetPeerMinRollupFee(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetReceiveVersion(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetSendVersion(element) => element.encode(),
                MystikoV2LayerZeroCalls::SetTrustedRemote(element) => element.encode(),
                MystikoV2LayerZeroCalls::TransferOwnership(element) => element.encode(),
                MystikoV2LayerZeroCalls::TrustedRemoteLookup(element) => element.encode(),
                MystikoV2LayerZeroCalls::UpdateDepositAmountLimits(element) => element.encode(),
                MystikoV2LayerZeroCalls::UpdateSanctionsListAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MystikoV2LayerZeroCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoV2LayerZeroCalls::AssetType(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::BridgeProxyAddress(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::BridgeType(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::ChangeOperator(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::Deposit(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::DisableSanctionsCheck(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::EnableSanctionsCheck(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::FailedMessages(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::ForceResumeReceive(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::GetAssociatedCommitmentPool(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::GetConfig(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::GetMaxAmount(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::GetMinAmount(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::GetMinBridgeFee(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::GetMinExecutorFee(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::GetPeerMinExecutorFee(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::GetPeerMinRollupFee(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::IsDepositsDisabled(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::IsTrustedRemote(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::LocalLayerZeroChainId(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::LzEndpoint(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::LzReceive(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::NonblockingLzReceive(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::Owner(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::PeerChainId(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::PeerChainName(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::PeerContract(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::PeerLayerZeroChainId(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::RenounceOwnership(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::RetryMessage(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SanctionsCheck(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SanctionsList(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetAssociatedCommitmentPool(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetBridgeProxyAddress(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetConfig(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetDepositsDisabled(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetEndpoint(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetMinBridgeFee(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetMinExecutorFee(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetPeerContract(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetPeerMinExecutorFee(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetPeerMinRollupFee(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetReceiveVersion(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetSendVersion(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::SetTrustedRemote(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::TransferOwnership(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::TrustedRemoteLookup(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::UpdateDepositAmountLimits(element) => element.fmt(f),
                MystikoV2LayerZeroCalls::UpdateSanctionsListAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AssetTypeCall> for MystikoV2LayerZeroCalls {
        fn from(var: AssetTypeCall) -> Self {
            MystikoV2LayerZeroCalls::AssetType(var)
        }
    }
    impl ::std::convert::From<BridgeProxyAddressCall> for MystikoV2LayerZeroCalls {
        fn from(var: BridgeProxyAddressCall) -> Self {
            MystikoV2LayerZeroCalls::BridgeProxyAddress(var)
        }
    }
    impl ::std::convert::From<BridgeTypeCall> for MystikoV2LayerZeroCalls {
        fn from(var: BridgeTypeCall) -> Self {
            MystikoV2LayerZeroCalls::BridgeType(var)
        }
    }
    impl ::std::convert::From<ChangeOperatorCall> for MystikoV2LayerZeroCalls {
        fn from(var: ChangeOperatorCall) -> Self {
            MystikoV2LayerZeroCalls::ChangeOperator(var)
        }
    }
    impl ::std::convert::From<DepositCall> for MystikoV2LayerZeroCalls {
        fn from(var: DepositCall) -> Self {
            MystikoV2LayerZeroCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<DisableSanctionsCheckCall> for MystikoV2LayerZeroCalls {
        fn from(var: DisableSanctionsCheckCall) -> Self {
            MystikoV2LayerZeroCalls::DisableSanctionsCheck(var)
        }
    }
    impl ::std::convert::From<EnableSanctionsCheckCall> for MystikoV2LayerZeroCalls {
        fn from(var: EnableSanctionsCheckCall) -> Self {
            MystikoV2LayerZeroCalls::EnableSanctionsCheck(var)
        }
    }
    impl ::std::convert::From<FailedMessagesCall> for MystikoV2LayerZeroCalls {
        fn from(var: FailedMessagesCall) -> Self {
            MystikoV2LayerZeroCalls::FailedMessages(var)
        }
    }
    impl ::std::convert::From<ForceResumeReceiveCall> for MystikoV2LayerZeroCalls {
        fn from(var: ForceResumeReceiveCall) -> Self {
            MystikoV2LayerZeroCalls::ForceResumeReceive(var)
        }
    }
    impl ::std::convert::From<GetAssociatedCommitmentPoolCall> for MystikoV2LayerZeroCalls {
        fn from(var: GetAssociatedCommitmentPoolCall) -> Self {
            MystikoV2LayerZeroCalls::GetAssociatedCommitmentPool(var)
        }
    }
    impl ::std::convert::From<GetConfigCall> for MystikoV2LayerZeroCalls {
        fn from(var: GetConfigCall) -> Self {
            MystikoV2LayerZeroCalls::GetConfig(var)
        }
    }
    impl ::std::convert::From<GetMaxAmountCall> for MystikoV2LayerZeroCalls {
        fn from(var: GetMaxAmountCall) -> Self {
            MystikoV2LayerZeroCalls::GetMaxAmount(var)
        }
    }
    impl ::std::convert::From<GetMinAmountCall> for MystikoV2LayerZeroCalls {
        fn from(var: GetMinAmountCall) -> Self {
            MystikoV2LayerZeroCalls::GetMinAmount(var)
        }
    }
    impl ::std::convert::From<GetMinBridgeFeeCall> for MystikoV2LayerZeroCalls {
        fn from(var: GetMinBridgeFeeCall) -> Self {
            MystikoV2LayerZeroCalls::GetMinBridgeFee(var)
        }
    }
    impl ::std::convert::From<GetMinExecutorFeeCall> for MystikoV2LayerZeroCalls {
        fn from(var: GetMinExecutorFeeCall) -> Self {
            MystikoV2LayerZeroCalls::GetMinExecutorFee(var)
        }
    }
    impl ::std::convert::From<GetPeerMinExecutorFeeCall> for MystikoV2LayerZeroCalls {
        fn from(var: GetPeerMinExecutorFeeCall) -> Self {
            MystikoV2LayerZeroCalls::GetPeerMinExecutorFee(var)
        }
    }
    impl ::std::convert::From<GetPeerMinRollupFeeCall> for MystikoV2LayerZeroCalls {
        fn from(var: GetPeerMinRollupFeeCall) -> Self {
            MystikoV2LayerZeroCalls::GetPeerMinRollupFee(var)
        }
    }
    impl ::std::convert::From<IsDepositsDisabledCall> for MystikoV2LayerZeroCalls {
        fn from(var: IsDepositsDisabledCall) -> Self {
            MystikoV2LayerZeroCalls::IsDepositsDisabled(var)
        }
    }
    impl ::std::convert::From<IsTrustedRemoteCall> for MystikoV2LayerZeroCalls {
        fn from(var: IsTrustedRemoteCall) -> Self {
            MystikoV2LayerZeroCalls::IsTrustedRemote(var)
        }
    }
    impl ::std::convert::From<LocalLayerZeroChainIdCall> for MystikoV2LayerZeroCalls {
        fn from(var: LocalLayerZeroChainIdCall) -> Self {
            MystikoV2LayerZeroCalls::LocalLayerZeroChainId(var)
        }
    }
    impl ::std::convert::From<LzEndpointCall> for MystikoV2LayerZeroCalls {
        fn from(var: LzEndpointCall) -> Self {
            MystikoV2LayerZeroCalls::LzEndpoint(var)
        }
    }
    impl ::std::convert::From<LzReceiveCall> for MystikoV2LayerZeroCalls {
        fn from(var: LzReceiveCall) -> Self {
            MystikoV2LayerZeroCalls::LzReceive(var)
        }
    }
    impl ::std::convert::From<NonblockingLzReceiveCall> for MystikoV2LayerZeroCalls {
        fn from(var: NonblockingLzReceiveCall) -> Self {
            MystikoV2LayerZeroCalls::NonblockingLzReceive(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for MystikoV2LayerZeroCalls {
        fn from(var: OwnerCall) -> Self {
            MystikoV2LayerZeroCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PeerChainIdCall> for MystikoV2LayerZeroCalls {
        fn from(var: PeerChainIdCall) -> Self {
            MystikoV2LayerZeroCalls::PeerChainId(var)
        }
    }
    impl ::std::convert::From<PeerChainNameCall> for MystikoV2LayerZeroCalls {
        fn from(var: PeerChainNameCall) -> Self {
            MystikoV2LayerZeroCalls::PeerChainName(var)
        }
    }
    impl ::std::convert::From<PeerContractCall> for MystikoV2LayerZeroCalls {
        fn from(var: PeerContractCall) -> Self {
            MystikoV2LayerZeroCalls::PeerContract(var)
        }
    }
    impl ::std::convert::From<PeerLayerZeroChainIdCall> for MystikoV2LayerZeroCalls {
        fn from(var: PeerLayerZeroChainIdCall) -> Self {
            MystikoV2LayerZeroCalls::PeerLayerZeroChainId(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for MystikoV2LayerZeroCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            MystikoV2LayerZeroCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RetryMessageCall> for MystikoV2LayerZeroCalls {
        fn from(var: RetryMessageCall) -> Self {
            MystikoV2LayerZeroCalls::RetryMessage(var)
        }
    }
    impl ::std::convert::From<SanctionsCheckCall> for MystikoV2LayerZeroCalls {
        fn from(var: SanctionsCheckCall) -> Self {
            MystikoV2LayerZeroCalls::SanctionsCheck(var)
        }
    }
    impl ::std::convert::From<SanctionsListCall> for MystikoV2LayerZeroCalls {
        fn from(var: SanctionsListCall) -> Self {
            MystikoV2LayerZeroCalls::SanctionsList(var)
        }
    }
    impl ::std::convert::From<SetAssociatedCommitmentPoolCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetAssociatedCommitmentPoolCall) -> Self {
            MystikoV2LayerZeroCalls::SetAssociatedCommitmentPool(var)
        }
    }
    impl ::std::convert::From<SetBridgeProxyAddressCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetBridgeProxyAddressCall) -> Self {
            MystikoV2LayerZeroCalls::SetBridgeProxyAddress(var)
        }
    }
    impl ::std::convert::From<SetConfigCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetConfigCall) -> Self {
            MystikoV2LayerZeroCalls::SetConfig(var)
        }
    }
    impl ::std::convert::From<SetDepositsDisabledCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetDepositsDisabledCall) -> Self {
            MystikoV2LayerZeroCalls::SetDepositsDisabled(var)
        }
    }
    impl ::std::convert::From<SetEndpointCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetEndpointCall) -> Self {
            MystikoV2LayerZeroCalls::SetEndpoint(var)
        }
    }
    impl ::std::convert::From<SetMinBridgeFeeCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetMinBridgeFeeCall) -> Self {
            MystikoV2LayerZeroCalls::SetMinBridgeFee(var)
        }
    }
    impl ::std::convert::From<SetMinExecutorFeeCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetMinExecutorFeeCall) -> Self {
            MystikoV2LayerZeroCalls::SetMinExecutorFee(var)
        }
    }
    impl ::std::convert::From<SetPeerContractCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetPeerContractCall) -> Self {
            MystikoV2LayerZeroCalls::SetPeerContract(var)
        }
    }
    impl ::std::convert::From<SetPeerMinExecutorFeeCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetPeerMinExecutorFeeCall) -> Self {
            MystikoV2LayerZeroCalls::SetPeerMinExecutorFee(var)
        }
    }
    impl ::std::convert::From<SetPeerMinRollupFeeCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetPeerMinRollupFeeCall) -> Self {
            MystikoV2LayerZeroCalls::SetPeerMinRollupFee(var)
        }
    }
    impl ::std::convert::From<SetReceiveVersionCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetReceiveVersionCall) -> Self {
            MystikoV2LayerZeroCalls::SetReceiveVersion(var)
        }
    }
    impl ::std::convert::From<SetSendVersionCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetSendVersionCall) -> Self {
            MystikoV2LayerZeroCalls::SetSendVersion(var)
        }
    }
    impl ::std::convert::From<SetTrustedRemoteCall> for MystikoV2LayerZeroCalls {
        fn from(var: SetTrustedRemoteCall) -> Self {
            MystikoV2LayerZeroCalls::SetTrustedRemote(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for MystikoV2LayerZeroCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            MystikoV2LayerZeroCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<TrustedRemoteLookupCall> for MystikoV2LayerZeroCalls {
        fn from(var: TrustedRemoteLookupCall) -> Self {
            MystikoV2LayerZeroCalls::TrustedRemoteLookup(var)
        }
    }
    impl ::std::convert::From<UpdateDepositAmountLimitsCall> for MystikoV2LayerZeroCalls {
        fn from(var: UpdateDepositAmountLimitsCall) -> Self {
            MystikoV2LayerZeroCalls::UpdateDepositAmountLimits(var)
        }
    }
    impl ::std::convert::From<UpdateSanctionsListAddressCall> for MystikoV2LayerZeroCalls {
        fn from(var: UpdateSanctionsListAddressCall) -> Self {
            MystikoV2LayerZeroCalls::UpdateSanctionsListAddress(var)
        }
    }
    #[doc = "Container type for all return fields from the `assetType` function with signature `assetType()` and selector `[63, 227, 52, 122]`"]
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
    pub struct AssetTypeReturn(pub u8);
    #[doc = "Container type for all return fields from the `bridgeProxyAddress` function with signature `bridgeProxyAddress()` and selector `[44, 210, 109, 69]`"]
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
    pub struct BridgeProxyAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `bridgeType` function with signature `bridgeType()` and selector `[36, 33, 225, 85]`"]
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
    pub struct BridgeTypeReturn(pub String);
    #[doc = "Container type for all return fields from the `failedMessages` function with signature `failedMessages(uint16,bytes,uint64)` and selector `[91, 140, 65, 230]`"]
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
    pub struct FailedMessagesReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getAssociatedCommitmentPool` function with signature `getAssociatedCommitmentPool()` and selector `[221, 172, 93, 193]`"]
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
    pub struct GetAssociatedCommitmentPoolReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `[245, 236, 189, 188]`"]
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
    pub struct GetConfigReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `getMaxAmount` function with signature `getMaxAmount()` and selector `[11, 169, 89, 9]`"]
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
    pub struct GetMaxAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getMinAmount` function with signature `getMinAmount()` and selector `[207, 199, 226, 218]`"]
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
    pub struct GetMinAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getMinBridgeFee` function with signature `getMinBridgeFee()` and selector `[239, 191, 178, 174]`"]
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
    pub struct GetMinBridgeFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getMinExecutorFee` function with signature `getMinExecutorFee()` and selector `[244, 173, 23, 198]`"]
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
    pub struct GetMinExecutorFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPeerMinExecutorFee` function with signature `getPeerMinExecutorFee()` and selector `[88, 152, 160, 168]`"]
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
    pub struct GetPeerMinExecutorFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPeerMinRollupFee` function with signature `getPeerMinRollupFee()` and selector `[130, 91, 95, 141]`"]
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
    pub struct GetPeerMinRollupFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isDepositsDisabled` function with signature `isDepositsDisabled()` and selector `[237, 110, 163, 58]`"]
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
    pub struct IsDepositsDisabledReturn(pub bool);
    #[doc = "Container type for all return fields from the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `[61, 139, 56, 246]`"]
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
    pub struct IsTrustedRemoteReturn(pub bool);
    #[doc = "Container type for all return fields from the `localLayerZeroChainId` function with signature `localLayerZeroChainId()` and selector `[48, 45, 95, 75]`"]
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
    pub struct LocalLayerZeroChainIdReturn(pub u16);
    #[doc = "Container type for all return fields from the `lzEndpoint` function with signature `lzEndpoint()` and selector `[179, 83, 170, 167]`"]
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
    pub struct LzEndpointReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
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
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `peerChainId` function with signature `peerChainId()` and selector `[205, 252, 238, 186]`"]
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
    pub struct PeerChainIdReturn(pub u64);
    #[doc = "Container type for all return fields from the `peerChainName` function with signature `peerChainName()` and selector `[78, 60, 16, 183]`"]
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
    pub struct PeerChainNameReturn(pub String);
    #[doc = "Container type for all return fields from the `peerContract` function with signature `peerContract()` and selector `[33, 227, 45, 85]`"]
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
    pub struct PeerContractReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `peerLayerZeroChainId` function with signature `peerLayerZeroChainId()` and selector `[0, 151, 160, 99]`"]
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
    pub struct PeerLayerZeroChainIdReturn(pub u16);
    #[doc = "Container type for all return fields from the `sanctionsCheck` function with signature `sanctionsCheck()` and selector `[177, 195, 148, 34]`"]
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
    pub struct SanctionsCheckReturn(pub bool);
    #[doc = "Container type for all return fields from the `sanctionsList` function with signature `sanctionsList()` and selector `[236, 87, 28, 106]`"]
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
    pub struct SanctionsListReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `[117, 51, 215, 136]`"]
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
    pub struct TrustedRemoteLookupReturn(pub ethers::core::types::Bytes);
    #[doc = "`DepositRequest(uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
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
