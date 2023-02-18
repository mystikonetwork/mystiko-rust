pub use commitment_pool_erc20::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod commitment_pool_erc20 {
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
    #[doc = "CommitmentPoolERC20 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"_treeHeight\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"contract IERC20Metadata\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AuditorIndexError\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AuditorNotesLengthError\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AuditorPublicKeyNotChanged\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CommitmentHasBeenSubmitted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"param\",\"type\":\"string\",\"components\":[]}],\"type\":\"error\",\"name\":\"Duplicated\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IndexOutOfBound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"param\",\"type\":\"string\",\"components\":[]}],\"type\":\"error\",\"name\":\"Invalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NewRootIsDuplicated\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotChanged\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NoteHasBeenSpent\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NumInputsGreaterThanZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOperator\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyWhitelistedRoller\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyWhitelistedSender\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OutputNotesLessThanThree\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RollupFeeToFew\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RollupSizeNotPowerOfTwo\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SanctionedAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TreeHeightLessThanZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TreeHeightOutOfBounds\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TreeIsFull\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"VerifierUpdatesHasBeenDisabled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"publicKey\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AuditorPublicKey\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CommitmentIncluded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"rollupFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"leafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"encryptedNote\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CommitmentQueued\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rootHash\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"serialNumber\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CommitmentSpent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"id\",\"type\":\"uint64\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"auditorPublicKey\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"encryptedAuditorNote\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EncryptedAuditorNote\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct CommitmentPool.AuditorNote[]\",\"name\":\"notes\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"id\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"publicKey\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"note\",\"type\":\"uint256\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"EncryptedAuditorNotes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OperatorChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RollupWhitelistDisabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SanctionsCheck\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"sanctions\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SanctionsList\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"VerifierUpdateDisabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_fullPath\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_rollupSize\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_pathIndices\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_actor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addEnqueueWhitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_roller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addRollupWhitelist\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetDecimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetName\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetSymbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"assetType\",\"outputs\":[{\"internalType\":\"enum AssetPool.AssetType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"auditorCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newOperator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeOperator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"_rollupSize\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disableRollupVerifier\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disableSanctionsCheck\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"_numInputs\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_numOutputs\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disableTransactVerifier\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"_rollupSize\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"contract IVerifier\",\"name\":\"_rollupVerifier\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableRollupVerifier\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableSanctionsCheck\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"_numInputs\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_numOutputs\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"contract IVerifier\",\"name\":\"_transactVerifier\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableTransactVerifier\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ICommitmentPool.CommitmentRequest\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"executorFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rollupFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"encryptedNote\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"_executor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enqueue\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllAuditorPublicKeys\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAuditorPublicKey\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCommitmentIncludedCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinRollupFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTreeCapacity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_commitment\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isHistoricCommitment\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isRollupWhitelistDisabled\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_serialNumber\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpentSerialNumber\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isVerifierUpdateDisabled\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_actor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeEnqueueWhitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_roller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeRollupWhitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ICommitmentPool.RollupRequest\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IVerifier.Proof\",\"name\":\"proof\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IVerifier.G1Point\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct IVerifier.G2Point\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\",\"components\":[]}]},{\"internalType\":\"struct IVerifier.G1Point\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"uint32\",\"name\":\"rollupSize\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newRoot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"leafHash\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rollup\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sanctionsCheck\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sanctionsList\",\"outputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minRollupFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinRollupFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRollupWhitelistDisabled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setVerifierUpdateDisabled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ICommitmentPool.TransactRequest\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IVerifier.Proof\",\"name\":\"proof\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IVerifier.G1Point\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct IVerifier.G2Point\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\",\"components\":[]}]},{\"internalType\":\"struct IVerifier.G1Point\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"rootHash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"serialNumbers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"sigHashes\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"sigPk\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"publicAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"relayerFeeAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"outCommitments\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"outRollupFees\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"publicRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"relayerAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"outEncryptedNotes\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"randomAuditingPublicKey\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"encryptedAuditorNotes\",\"type\":\"uint256[]\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"_signature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transact\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_publicKey\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateAuditorPublicKey\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"_sanction\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateSanctionsListAddress\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static COMMITMENTPOOLERC20_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct CommitmentPoolERC20<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CommitmentPoolERC20<M> {
        fn clone(&self) -> Self {
            CommitmentPoolERC20(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CommitmentPoolERC20<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for CommitmentPoolERC20<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CommitmentPoolERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> CommitmentPoolERC20<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), COMMITMENTPOOLERC20_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `_pathIndices` (0xf2da1d41) function"]
        pub fn path_indices(
            &self,
            full_path: ethers::core::types::U256,
            rollup_size: u32,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([242, 218, 29, 65], (full_path, rollup_size))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addEnqueueWhitelist` (0xa9b1d296) function"]
        pub fn add_enqueue_whitelist(
            &self,
            actor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 177, 210, 150], actor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addRollupWhitelist` (0x02d498f1) function"]
        pub fn add_rollup_whitelist(
            &self,
            roller: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 212, 152, 241], roller)
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `auditorCount` (0x115f574c) function"]
        pub fn auditor_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([17, 95, 87, 76], ())
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
        #[doc = "Calls the contract's `disableRollupVerifier` (0x9b0a6fea) function"]
        pub fn disable_rollup_verifier(
            &self,
            rollup_size: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 10, 111, 234], rollup_size)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disableSanctionsCheck` (0xdd757c34) function"]
        pub fn disable_sanctions_check(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 117, 124, 52], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disableTransactVerifier` (0xc259e2e6) function"]
        pub fn disable_transact_verifier(
            &self,
            num_inputs: u32,
            num_outputs: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 89, 226, 230], (num_inputs, num_outputs))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enableRollupVerifier` (0xdeeff7cd) function"]
        pub fn enable_rollup_verifier(
            &self,
            rollup_size: u32,
            rollup_verifier: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 239, 247, 205], (rollup_size, rollup_verifier))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enableSanctionsCheck` (0x01dbf19f) function"]
        pub fn enable_sanctions_check(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 219, 241, 159], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enableTransactVerifier` (0x7fa4b09c) function"]
        pub fn enable_transact_verifier(
            &self,
            num_inputs: u32,
            num_outputs: u32,
            transact_verifier: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [127, 164, 176, 156],
                    (num_inputs, num_outputs, transact_verifier),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enqueue` (0x78d60cd7) function"]
        pub fn enqueue(
            &self,
            request: CommitmentRequest,
            executor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 214, 12, 215], (request, executor))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAllAuditorPublicKeys` (0x63bc7d32) function"]
        pub fn get_all_auditor_public_keys(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([99, 188, 125, 50], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAuditorPublicKey` (0x87780df9) function"]
        pub fn get_auditor_public_key(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([135, 120, 13, 249], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCommitmentIncludedCount` (0xe500f504) function"]
        pub fn get_commitment_included_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([229, 0, 245, 4], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinRollupFee` (0xb08892d0) function"]
        pub fn get_min_rollup_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([176, 136, 146, 208], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTreeCapacity` (0x484eb652) function"]
        pub fn get_tree_capacity(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([72, 78, 182, 82], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isHistoricCommitment` (0x57060016) function"]
        pub fn is_historic_commitment(
            &self,
            commitment: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([87, 6, 0, 22], commitment)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownRoot` (0xa6232a93) function"]
        pub fn is_known_root(
            &self,
            root: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 35, 42, 147], root)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isRollupWhitelistDisabled` (0xffa89b88) function"]
        pub fn is_rollup_whitelist_disabled(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([255, 168, 155, 136], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpentSerialNumber` (0x3bb8d1b4) function"]
        pub fn is_spent_serial_number(
            &self,
            serial_number: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 184, 209, 180], serial_number)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isVerifierUpdateDisabled` (0x4eb069f7) function"]
        pub fn is_verifier_update_disabled(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([78, 176, 105, 247], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeEnqueueWhitelist` (0x03db9874) function"]
        pub fn remove_enqueue_whitelist(
            &self,
            actor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 219, 152, 116], actor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeRollupWhitelist` (0x9cc6b354) function"]
        pub fn remove_rollup_whitelist(
            &self,
            roller: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 198, 179, 84], roller)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rollup` (0x14a7737d) function"]
        pub fn rollup(
            &self,
            request: RollupRequest,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 167, 115, 125], (request,))
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
        #[doc = "Calls the contract's `setMinRollupFee` (0x7cbf0ff6) function"]
        pub fn set_min_rollup_fee(
            &self,
            min_rollup_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 191, 15, 246], min_rollup_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRollupWhitelistDisabled` (0xf8f05388) function"]
        pub fn set_rollup_whitelist_disabled(
            &self,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 240, 83, 136], state)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setVerifierUpdateDisabled` (0xb3b75631) function"]
        pub fn set_verifier_update_disabled(
            &self,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 183, 86, 49], state)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transact` (0x72082971) function"]
        pub fn transact(
            &self,
            request: TransactRequest,
            signature: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 8, 41, 113], (request, signature))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateAuditorPublicKey` (0x0c8867e6) function"]
        pub fn update_auditor_public_key(
            &self,
            index: ethers::core::types::U256,
            public_key: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 136, 103, 230], (index, public_key))
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
        #[doc = "Gets the contract's `AuditorPublicKey` event"]
        pub fn auditor_public_key_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AuditorPublicKeyFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CommitmentIncluded` event"]
        pub fn commitment_included_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CommitmentIncludedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CommitmentQueued` event"]
        pub fn commitment_queued_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CommitmentQueuedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CommitmentSpent` event"]
        pub fn commitment_spent_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CommitmentSpentFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EncryptedAuditorNote` event"]
        pub fn encrypted_auditor_note_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EncryptedAuditorNoteFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EncryptedAuditorNotes` event"]
        pub fn encrypted_auditor_notes_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EncryptedAuditorNotesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OperatorChanged` event"]
        pub fn operator_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OperatorChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RollupWhitelistDisabled` event"]
        pub fn rollup_whitelist_disabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RollupWhitelistDisabledFilter> {
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
        #[doc = "Gets the contract's `VerifierUpdateDisabled` event"]
        pub fn verifier_update_disabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VerifierUpdateDisabledFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, CommitmentPoolERC20Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for CommitmentPoolERC20<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `AuditorIndexError` with signature `AuditorIndexError()` and selector `[198, 49, 13, 20]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "AuditorIndexError", abi = "AuditorIndexError()")]
    pub struct AuditorIndexError;
    #[doc = "Custom Error type `AuditorNotesLengthError` with signature `AuditorNotesLengthError()` and selector `[235, 61, 34, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "AuditorNotesLengthError", abi = "AuditorNotesLengthError()")]
    pub struct AuditorNotesLengthError;
    #[doc = "Custom Error type `AuditorPublicKeyNotChanged` with signature `AuditorPublicKeyNotChanged()` and selector `[2, 83, 131, 56]`"]
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
        name = "AuditorPublicKeyNotChanged",
        abi = "AuditorPublicKeyNotChanged()"
    )]
    pub struct AuditorPublicKeyNotChanged;
    #[doc = "Custom Error type `CommitmentHasBeenSubmitted` with signature `CommitmentHasBeenSubmitted()` and selector `[227, 140, 209, 77]`"]
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
        name = "CommitmentHasBeenSubmitted",
        abi = "CommitmentHasBeenSubmitted()"
    )]
    pub struct CommitmentHasBeenSubmitted;
    #[doc = "Custom Error type `Duplicated` with signature `Duplicated(string)` and selector `[190, 227, 97, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Duplicated", abi = "Duplicated(string)")]
    pub struct Duplicated {
        pub param: String,
    }
    #[doc = "Custom Error type `IndexOutOfBound` with signature `IndexOutOfBound()` and selector `[211, 72, 47, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "IndexOutOfBound", abi = "IndexOutOfBound()")]
    pub struct IndexOutOfBound;
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
    #[doc = "Custom Error type `NewRootIsDuplicated` with signature `NewRootIsDuplicated()` and selector `[226, 225, 33, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NewRootIsDuplicated", abi = "NewRootIsDuplicated()")]
    pub struct NewRootIsDuplicated;
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
    #[doc = "Custom Error type `NoteHasBeenSpent` with signature `NoteHasBeenSpent()` and selector `[255, 85, 110, 32]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NoteHasBeenSpent", abi = "NoteHasBeenSpent()")]
    pub struct NoteHasBeenSpent;
    #[doc = "Custom Error type `NumInputsGreaterThanZero` with signature `NumInputsGreaterThanZero()` and selector `[159, 123, 217, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NumInputsGreaterThanZero", abi = "NumInputsGreaterThanZero()")]
    pub struct NumInputsGreaterThanZero;
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
    #[doc = "Custom Error type `OnlyWhitelistedRoller` with signature `OnlyWhitelistedRoller()` and selector `[33, 206, 1, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyWhitelistedRoller", abi = "OnlyWhitelistedRoller()")]
    pub struct OnlyWhitelistedRoller;
    #[doc = "Custom Error type `OnlyWhitelistedSender` with signature `OnlyWhitelistedSender()` and selector `[247, 94, 159, 199]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyWhitelistedSender", abi = "OnlyWhitelistedSender()")]
    pub struct OnlyWhitelistedSender;
    #[doc = "Custom Error type `OutputNotesLessThanThree` with signature `OutputNotesLessThanThree()` and selector `[127, 99, 40, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OutputNotesLessThanThree", abi = "OutputNotesLessThanThree()")]
    pub struct OutputNotesLessThanThree;
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
    #[doc = "Custom Error type `RollupSizeNotPowerOfTwo` with signature `RollupSizeNotPowerOfTwo()` and selector `[34, 113, 127, 249]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "RollupSizeNotPowerOfTwo", abi = "RollupSizeNotPowerOfTwo()")]
    pub struct RollupSizeNotPowerOfTwo;
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
    #[doc = "Custom Error type `TreeHeightLessThanZero` with signature `TreeHeightLessThanZero()` and selector `[177, 60, 166, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "TreeHeightLessThanZero", abi = "TreeHeightLessThanZero()")]
    pub struct TreeHeightLessThanZero;
    #[doc = "Custom Error type `TreeHeightOutOfBounds` with signature `TreeHeightOutOfBounds()` and selector `[151, 128, 244, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "TreeHeightOutOfBounds", abi = "TreeHeightOutOfBounds()")]
    pub struct TreeHeightOutOfBounds;
    #[doc = "Custom Error type `TreeIsFull` with signature `TreeIsFull()` and selector `[237, 115, 45, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "TreeIsFull", abi = "TreeIsFull()")]
    pub struct TreeIsFull;
    #[doc = "Custom Error type `VerifierUpdatesHasBeenDisabled` with signature `VerifierUpdatesHasBeenDisabled()` and selector `[54, 227, 224, 149]`"]
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
        name = "VerifierUpdatesHasBeenDisabled",
        abi = "VerifierUpdatesHasBeenDisabled()"
    )]
    pub struct VerifierUpdatesHasBeenDisabled;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CommitmentPoolERC20Errors {
        AuditorIndexError(AuditorIndexError),
        AuditorNotesLengthError(AuditorNotesLengthError),
        AuditorPublicKeyNotChanged(AuditorPublicKeyNotChanged),
        CommitmentHasBeenSubmitted(CommitmentHasBeenSubmitted),
        Duplicated(Duplicated),
        IndexOutOfBound(IndexOutOfBound),
        Invalid(Invalid),
        NewRootIsDuplicated(NewRootIsDuplicated),
        NotChanged(NotChanged),
        NoteHasBeenSpent(NoteHasBeenSpent),
        NumInputsGreaterThanZero(NumInputsGreaterThanZero),
        OnlyOperator(OnlyOperator),
        OnlyWhitelistedRoller(OnlyWhitelistedRoller),
        OnlyWhitelistedSender(OnlyWhitelistedSender),
        OutputNotesLessThanThree(OutputNotesLessThanThree),
        RollupFeeToFew(RollupFeeToFew),
        RollupSizeNotPowerOfTwo(RollupSizeNotPowerOfTwo),
        SanctionedAddress(SanctionedAddress),
        TreeHeightLessThanZero(TreeHeightLessThanZero),
        TreeHeightOutOfBounds(TreeHeightOutOfBounds),
        TreeIsFull(TreeIsFull),
        VerifierUpdatesHasBeenDisabled(VerifierUpdatesHasBeenDisabled),
    }
    impl ethers::core::abi::AbiDecode for CommitmentPoolERC20Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AuditorIndexError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::AuditorIndexError(decoded));
            }
            if let Ok(decoded) =
                <AuditorNotesLengthError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::AuditorNotesLengthError(decoded));
            }
            if let Ok(decoded) =
                <AuditorPublicKeyNotChanged as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::AuditorPublicKeyNotChanged(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CommitmentHasBeenSubmitted as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::CommitmentHasBeenSubmitted(
                    decoded,
                ));
            }
            if let Ok(decoded) = <Duplicated as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::Duplicated(decoded));
            }
            if let Ok(decoded) =
                <IndexOutOfBound as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::IndexOutOfBound(decoded));
            }
            if let Ok(decoded) = <Invalid as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CommitmentPoolERC20Errors::Invalid(decoded));
            }
            if let Ok(decoded) =
                <NewRootIsDuplicated as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::NewRootIsDuplicated(decoded));
            }
            if let Ok(decoded) = <NotChanged as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::NotChanged(decoded));
            }
            if let Ok(decoded) =
                <NoteHasBeenSpent as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::NoteHasBeenSpent(decoded));
            }
            if let Ok(decoded) =
                <NumInputsGreaterThanZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::NumInputsGreaterThanZero(decoded));
            }
            if let Ok(decoded) =
                <OnlyOperator as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::OnlyOperator(decoded));
            }
            if let Ok(decoded) =
                <OnlyWhitelistedRoller as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::OnlyWhitelistedRoller(decoded));
            }
            if let Ok(decoded) =
                <OnlyWhitelistedSender as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::OnlyWhitelistedSender(decoded));
            }
            if let Ok(decoded) =
                <OutputNotesLessThanThree as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::OutputNotesLessThanThree(decoded));
            }
            if let Ok(decoded) =
                <RollupFeeToFew as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::RollupFeeToFew(decoded));
            }
            if let Ok(decoded) =
                <RollupSizeNotPowerOfTwo as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::RollupSizeNotPowerOfTwo(decoded));
            }
            if let Ok(decoded) =
                <SanctionedAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::SanctionedAddress(decoded));
            }
            if let Ok(decoded) =
                <TreeHeightLessThanZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::TreeHeightLessThanZero(decoded));
            }
            if let Ok(decoded) =
                <TreeHeightOutOfBounds as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::TreeHeightOutOfBounds(decoded));
            }
            if let Ok(decoded) = <TreeIsFull as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Errors::TreeIsFull(decoded));
            }
            if let Ok(decoded) =
                <VerifierUpdatesHasBeenDisabled as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CommitmentPoolERC20Errors::VerifierUpdatesHasBeenDisabled(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CommitmentPoolERC20Errors {
        fn encode(self) -> Vec<u8> {
            match self {
                CommitmentPoolERC20Errors::AuditorIndexError(element) => element.encode(),
                CommitmentPoolERC20Errors::AuditorNotesLengthError(element) => element.encode(),
                CommitmentPoolERC20Errors::AuditorPublicKeyNotChanged(element) => element.encode(),
                CommitmentPoolERC20Errors::CommitmentHasBeenSubmitted(element) => element.encode(),
                CommitmentPoolERC20Errors::Duplicated(element) => element.encode(),
                CommitmentPoolERC20Errors::IndexOutOfBound(element) => element.encode(),
                CommitmentPoolERC20Errors::Invalid(element) => element.encode(),
                CommitmentPoolERC20Errors::NewRootIsDuplicated(element) => element.encode(),
                CommitmentPoolERC20Errors::NotChanged(element) => element.encode(),
                CommitmentPoolERC20Errors::NoteHasBeenSpent(element) => element.encode(),
                CommitmentPoolERC20Errors::NumInputsGreaterThanZero(element) => element.encode(),
                CommitmentPoolERC20Errors::OnlyOperator(element) => element.encode(),
                CommitmentPoolERC20Errors::OnlyWhitelistedRoller(element) => element.encode(),
                CommitmentPoolERC20Errors::OnlyWhitelistedSender(element) => element.encode(),
                CommitmentPoolERC20Errors::OutputNotesLessThanThree(element) => element.encode(),
                CommitmentPoolERC20Errors::RollupFeeToFew(element) => element.encode(),
                CommitmentPoolERC20Errors::RollupSizeNotPowerOfTwo(element) => element.encode(),
                CommitmentPoolERC20Errors::SanctionedAddress(element) => element.encode(),
                CommitmentPoolERC20Errors::TreeHeightLessThanZero(element) => element.encode(),
                CommitmentPoolERC20Errors::TreeHeightOutOfBounds(element) => element.encode(),
                CommitmentPoolERC20Errors::TreeIsFull(element) => element.encode(),
                CommitmentPoolERC20Errors::VerifierUpdatesHasBeenDisabled(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for CommitmentPoolERC20Errors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CommitmentPoolERC20Errors::AuditorIndexError(element) => element.fmt(f),
                CommitmentPoolERC20Errors::AuditorNotesLengthError(element) => element.fmt(f),
                CommitmentPoolERC20Errors::AuditorPublicKeyNotChanged(element) => element.fmt(f),
                CommitmentPoolERC20Errors::CommitmentHasBeenSubmitted(element) => element.fmt(f),
                CommitmentPoolERC20Errors::Duplicated(element) => element.fmt(f),
                CommitmentPoolERC20Errors::IndexOutOfBound(element) => element.fmt(f),
                CommitmentPoolERC20Errors::Invalid(element) => element.fmt(f),
                CommitmentPoolERC20Errors::NewRootIsDuplicated(element) => element.fmt(f),
                CommitmentPoolERC20Errors::NotChanged(element) => element.fmt(f),
                CommitmentPoolERC20Errors::NoteHasBeenSpent(element) => element.fmt(f),
                CommitmentPoolERC20Errors::NumInputsGreaterThanZero(element) => element.fmt(f),
                CommitmentPoolERC20Errors::OnlyOperator(element) => element.fmt(f),
                CommitmentPoolERC20Errors::OnlyWhitelistedRoller(element) => element.fmt(f),
                CommitmentPoolERC20Errors::OnlyWhitelistedSender(element) => element.fmt(f),
                CommitmentPoolERC20Errors::OutputNotesLessThanThree(element) => element.fmt(f),
                CommitmentPoolERC20Errors::RollupFeeToFew(element) => element.fmt(f),
                CommitmentPoolERC20Errors::RollupSizeNotPowerOfTwo(element) => element.fmt(f),
                CommitmentPoolERC20Errors::SanctionedAddress(element) => element.fmt(f),
                CommitmentPoolERC20Errors::TreeHeightLessThanZero(element) => element.fmt(f),
                CommitmentPoolERC20Errors::TreeHeightOutOfBounds(element) => element.fmt(f),
                CommitmentPoolERC20Errors::TreeIsFull(element) => element.fmt(f),
                CommitmentPoolERC20Errors::VerifierUpdatesHasBeenDisabled(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AuditorIndexError> for CommitmentPoolERC20Errors {
        fn from(var: AuditorIndexError) -> Self {
            CommitmentPoolERC20Errors::AuditorIndexError(var)
        }
    }
    impl ::std::convert::From<AuditorNotesLengthError> for CommitmentPoolERC20Errors {
        fn from(var: AuditorNotesLengthError) -> Self {
            CommitmentPoolERC20Errors::AuditorNotesLengthError(var)
        }
    }
    impl ::std::convert::From<AuditorPublicKeyNotChanged> for CommitmentPoolERC20Errors {
        fn from(var: AuditorPublicKeyNotChanged) -> Self {
            CommitmentPoolERC20Errors::AuditorPublicKeyNotChanged(var)
        }
    }
    impl ::std::convert::From<CommitmentHasBeenSubmitted> for CommitmentPoolERC20Errors {
        fn from(var: CommitmentHasBeenSubmitted) -> Self {
            CommitmentPoolERC20Errors::CommitmentHasBeenSubmitted(var)
        }
    }
    impl ::std::convert::From<Duplicated> for CommitmentPoolERC20Errors {
        fn from(var: Duplicated) -> Self {
            CommitmentPoolERC20Errors::Duplicated(var)
        }
    }
    impl ::std::convert::From<IndexOutOfBound> for CommitmentPoolERC20Errors {
        fn from(var: IndexOutOfBound) -> Self {
            CommitmentPoolERC20Errors::IndexOutOfBound(var)
        }
    }
    impl ::std::convert::From<Invalid> for CommitmentPoolERC20Errors {
        fn from(var: Invalid) -> Self {
            CommitmentPoolERC20Errors::Invalid(var)
        }
    }
    impl ::std::convert::From<NewRootIsDuplicated> for CommitmentPoolERC20Errors {
        fn from(var: NewRootIsDuplicated) -> Self {
            CommitmentPoolERC20Errors::NewRootIsDuplicated(var)
        }
    }
    impl ::std::convert::From<NotChanged> for CommitmentPoolERC20Errors {
        fn from(var: NotChanged) -> Self {
            CommitmentPoolERC20Errors::NotChanged(var)
        }
    }
    impl ::std::convert::From<NoteHasBeenSpent> for CommitmentPoolERC20Errors {
        fn from(var: NoteHasBeenSpent) -> Self {
            CommitmentPoolERC20Errors::NoteHasBeenSpent(var)
        }
    }
    impl ::std::convert::From<NumInputsGreaterThanZero> for CommitmentPoolERC20Errors {
        fn from(var: NumInputsGreaterThanZero) -> Self {
            CommitmentPoolERC20Errors::NumInputsGreaterThanZero(var)
        }
    }
    impl ::std::convert::From<OnlyOperator> for CommitmentPoolERC20Errors {
        fn from(var: OnlyOperator) -> Self {
            CommitmentPoolERC20Errors::OnlyOperator(var)
        }
    }
    impl ::std::convert::From<OnlyWhitelistedRoller> for CommitmentPoolERC20Errors {
        fn from(var: OnlyWhitelistedRoller) -> Self {
            CommitmentPoolERC20Errors::OnlyWhitelistedRoller(var)
        }
    }
    impl ::std::convert::From<OnlyWhitelistedSender> for CommitmentPoolERC20Errors {
        fn from(var: OnlyWhitelistedSender) -> Self {
            CommitmentPoolERC20Errors::OnlyWhitelistedSender(var)
        }
    }
    impl ::std::convert::From<OutputNotesLessThanThree> for CommitmentPoolERC20Errors {
        fn from(var: OutputNotesLessThanThree) -> Self {
            CommitmentPoolERC20Errors::OutputNotesLessThanThree(var)
        }
    }
    impl ::std::convert::From<RollupFeeToFew> for CommitmentPoolERC20Errors {
        fn from(var: RollupFeeToFew) -> Self {
            CommitmentPoolERC20Errors::RollupFeeToFew(var)
        }
    }
    impl ::std::convert::From<RollupSizeNotPowerOfTwo> for CommitmentPoolERC20Errors {
        fn from(var: RollupSizeNotPowerOfTwo) -> Self {
            CommitmentPoolERC20Errors::RollupSizeNotPowerOfTwo(var)
        }
    }
    impl ::std::convert::From<SanctionedAddress> for CommitmentPoolERC20Errors {
        fn from(var: SanctionedAddress) -> Self {
            CommitmentPoolERC20Errors::SanctionedAddress(var)
        }
    }
    impl ::std::convert::From<TreeHeightLessThanZero> for CommitmentPoolERC20Errors {
        fn from(var: TreeHeightLessThanZero) -> Self {
            CommitmentPoolERC20Errors::TreeHeightLessThanZero(var)
        }
    }
    impl ::std::convert::From<TreeHeightOutOfBounds> for CommitmentPoolERC20Errors {
        fn from(var: TreeHeightOutOfBounds) -> Self {
            CommitmentPoolERC20Errors::TreeHeightOutOfBounds(var)
        }
    }
    impl ::std::convert::From<TreeIsFull> for CommitmentPoolERC20Errors {
        fn from(var: TreeIsFull) -> Self {
            CommitmentPoolERC20Errors::TreeIsFull(var)
        }
    }
    impl ::std::convert::From<VerifierUpdatesHasBeenDisabled> for CommitmentPoolERC20Errors {
        fn from(var: VerifierUpdatesHasBeenDisabled) -> Self {
            CommitmentPoolERC20Errors::VerifierUpdatesHasBeenDisabled(var)
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
    #[ethevent(name = "AuditorPublicKey", abi = "AuditorPublicKey(uint256,uint256)")]
    pub struct AuditorPublicKeyFilter {
        #[ethevent(indexed)]
        pub index: ethers::core::types::U256,
        pub public_key: ethers::core::types::U256,
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
    #[ethevent(name = "CommitmentIncluded", abi = "CommitmentIncluded(uint256)")]
    pub struct CommitmentIncludedFilter {
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
    #[ethevent(
        name = "CommitmentQueued",
        abi = "CommitmentQueued(uint256,uint256,uint256,bytes)"
    )]
    pub struct CommitmentQueuedFilter {
        #[ethevent(indexed)]
        pub commitment: ethers::core::types::U256,
        pub rollup_fee: ethers::core::types::U256,
        pub leaf_index: ethers::core::types::U256,
        pub encrypted_note: ethers::core::types::Bytes,
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
    #[ethevent(name = "CommitmentSpent", abi = "CommitmentSpent(uint256,uint256)")]
    pub struct CommitmentSpentFilter {
        #[ethevent(indexed)]
        pub root_hash: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub serial_number: ethers::core::types::U256,
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
    #[ethevent(
        name = "EncryptedAuditorNote",
        abi = "EncryptedAuditorNote(uint64,uint256,uint256)"
    )]
    pub struct EncryptedAuditorNoteFilter {
        pub id: u64,
        pub auditor_public_key: ethers::core::types::U256,
        pub encrypted_auditor_note: ethers::core::types::U256,
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
    #[ethevent(
        name = "EncryptedAuditorNotes",
        abi = "EncryptedAuditorNotes((uint64,uint256,uint256)[])"
    )]
    pub struct EncryptedAuditorNotesFilter {
        pub notes: ::std::vec::Vec<AuditorNote>,
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
    #[ethevent(
        name = "RollupWhitelistDisabled",
        abi = "RollupWhitelistDisabled(bool)"
    )]
    pub struct RollupWhitelistDisabledFilter {
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
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "VerifierUpdateDisabled", abi = "VerifierUpdateDisabled(bool)")]
    pub struct VerifierUpdateDisabledFilter {
        pub state: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CommitmentPoolERC20Events {
        AuditorPublicKeyFilter(AuditorPublicKeyFilter),
        CommitmentIncludedFilter(CommitmentIncludedFilter),
        CommitmentQueuedFilter(CommitmentQueuedFilter),
        CommitmentSpentFilter(CommitmentSpentFilter),
        EncryptedAuditorNoteFilter(EncryptedAuditorNoteFilter),
        EncryptedAuditorNotesFilter(EncryptedAuditorNotesFilter),
        OperatorChangedFilter(OperatorChangedFilter),
        RollupWhitelistDisabledFilter(RollupWhitelistDisabledFilter),
        SanctionsCheckFilter(SanctionsCheckFilter),
        SanctionsListFilter(SanctionsListFilter),
        VerifierUpdateDisabledFilter(VerifierUpdateDisabledFilter),
    }
    impl ethers::contract::EthLogDecode for CommitmentPoolERC20Events {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AuditorPublicKeyFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::AuditorPublicKeyFilter(decoded));
            }
            if let Ok(decoded) = CommitmentIncludedFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::CommitmentIncludedFilter(decoded));
            }
            if let Ok(decoded) = CommitmentQueuedFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::CommitmentQueuedFilter(decoded));
            }
            if let Ok(decoded) = CommitmentSpentFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::CommitmentSpentFilter(decoded));
            }
            if let Ok(decoded) = EncryptedAuditorNoteFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::EncryptedAuditorNoteFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EncryptedAuditorNotesFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::EncryptedAuditorNotesFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = RollupWhitelistDisabledFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::RollupWhitelistDisabledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::SanctionsListFilter(decoded));
            }
            if let Ok(decoded) = VerifierUpdateDisabledFilter::decode_log(log) {
                return Ok(CommitmentPoolERC20Events::VerifierUpdateDisabledFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CommitmentPoolERC20Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CommitmentPoolERC20Events::AuditorPublicKeyFilter(element) => element.fmt(f),
                CommitmentPoolERC20Events::CommitmentIncludedFilter(element) => element.fmt(f),
                CommitmentPoolERC20Events::CommitmentQueuedFilter(element) => element.fmt(f),
                CommitmentPoolERC20Events::CommitmentSpentFilter(element) => element.fmt(f),
                CommitmentPoolERC20Events::EncryptedAuditorNoteFilter(element) => element.fmt(f),
                CommitmentPoolERC20Events::EncryptedAuditorNotesFilter(element) => element.fmt(f),
                CommitmentPoolERC20Events::OperatorChangedFilter(element) => element.fmt(f),
                CommitmentPoolERC20Events::RollupWhitelistDisabledFilter(element) => element.fmt(f),
                CommitmentPoolERC20Events::SanctionsCheckFilter(element) => element.fmt(f),
                CommitmentPoolERC20Events::SanctionsListFilter(element) => element.fmt(f),
                CommitmentPoolERC20Events::VerifierUpdateDisabledFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `_pathIndices` function with signature `_pathIndices(uint256,uint32)` and selector `[242, 218, 29, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "_pathIndices", abi = "_pathIndices(uint256,uint32)")]
    pub struct PathIndicesCall {
        pub full_path: ethers::core::types::U256,
        pub rollup_size: u32,
    }
    #[doc = "Container type for all input parameters for the `addEnqueueWhitelist` function with signature `addEnqueueWhitelist(address)` and selector `[169, 177, 210, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addEnqueueWhitelist", abi = "addEnqueueWhitelist(address)")]
    pub struct AddEnqueueWhitelistCall {
        pub actor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addRollupWhitelist` function with signature `addRollupWhitelist(address)` and selector `[2, 212, 152, 241]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addRollupWhitelist", abi = "addRollupWhitelist(address)")]
    pub struct AddRollupWhitelistCall {
        pub roller: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `auditorCount` function with signature `auditorCount()` and selector `[17, 95, 87, 76]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "auditorCount", abi = "auditorCount()")]
    pub struct AuditorCountCall;
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
    #[doc = "Container type for all input parameters for the `disableRollupVerifier` function with signature `disableRollupVerifier(uint32)` and selector `[155, 10, 111, 234]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "disableRollupVerifier", abi = "disableRollupVerifier(uint32)")]
    pub struct DisableRollupVerifierCall {
        pub rollup_size: u32,
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
    #[doc = "Container type for all input parameters for the `disableTransactVerifier` function with signature `disableTransactVerifier(uint32,uint32)` and selector `[194, 89, 226, 230]`"]
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
        name = "disableTransactVerifier",
        abi = "disableTransactVerifier(uint32,uint32)"
    )]
    pub struct DisableTransactVerifierCall {
        pub num_inputs: u32,
        pub num_outputs: u32,
    }
    #[doc = "Container type for all input parameters for the `enableRollupVerifier` function with signature `enableRollupVerifier(uint32,address)` and selector `[222, 239, 247, 205]`"]
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
        name = "enableRollupVerifier",
        abi = "enableRollupVerifier(uint32,address)"
    )]
    pub struct EnableRollupVerifierCall {
        pub rollup_size: u32,
        pub rollup_verifier: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `enableTransactVerifier` function with signature `enableTransactVerifier(uint32,uint32,address)` and selector `[127, 164, 176, 156]`"]
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
        name = "enableTransactVerifier",
        abi = "enableTransactVerifier(uint32,uint32,address)"
    )]
    pub struct EnableTransactVerifierCall {
        pub num_inputs: u32,
        pub num_outputs: u32,
        pub transact_verifier: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `enqueue` function with signature `enqueue((uint256,uint256,uint256,uint256,bytes),address)` and selector `[120, 214, 12, 215]`"]
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
        name = "enqueue",
        abi = "enqueue((uint256,uint256,uint256,uint256,bytes),address)"
    )]
    pub struct EnqueueCall {
        pub request: CommitmentRequest,
        pub executor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAllAuditorPublicKeys` function with signature `getAllAuditorPublicKeys()` and selector `[99, 188, 125, 50]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAllAuditorPublicKeys", abi = "getAllAuditorPublicKeys()")]
    pub struct GetAllAuditorPublicKeysCall;
    #[doc = "Container type for all input parameters for the `getAuditorPublicKey` function with signature `getAuditorPublicKey(uint256)` and selector `[135, 120, 13, 249]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAuditorPublicKey", abi = "getAuditorPublicKey(uint256)")]
    pub struct GetAuditorPublicKeyCall {
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getCommitmentIncludedCount` function with signature `getCommitmentIncludedCount()` and selector `[229, 0, 245, 4]`"]
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
        name = "getCommitmentIncludedCount",
        abi = "getCommitmentIncludedCount()"
    )]
    pub struct GetCommitmentIncludedCountCall;
    #[doc = "Container type for all input parameters for the `getMinRollupFee` function with signature `getMinRollupFee()` and selector `[176, 136, 146, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMinRollupFee", abi = "getMinRollupFee()")]
    pub struct GetMinRollupFeeCall;
    #[doc = "Container type for all input parameters for the `getTreeCapacity` function with signature `getTreeCapacity()` and selector `[72, 78, 182, 82]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTreeCapacity", abi = "getTreeCapacity()")]
    pub struct GetTreeCapacityCall;
    #[doc = "Container type for all input parameters for the `isHistoricCommitment` function with signature `isHistoricCommitment(uint256)` and selector `[87, 6, 0, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isHistoricCommitment", abi = "isHistoricCommitment(uint256)")]
    pub struct IsHistoricCommitmentCall {
        pub commitment: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `[166, 35, 42, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isKnownRoot", abi = "isKnownRoot(uint256)")]
    pub struct IsKnownRootCall {
        pub root: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isRollupWhitelistDisabled` function with signature `isRollupWhitelistDisabled()` and selector `[255, 168, 155, 136]`"]
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
        name = "isRollupWhitelistDisabled",
        abi = "isRollupWhitelistDisabled()"
    )]
    pub struct IsRollupWhitelistDisabledCall;
    #[doc = "Container type for all input parameters for the `isSpentSerialNumber` function with signature `isSpentSerialNumber(uint256)` and selector `[59, 184, 209, 180]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isSpentSerialNumber", abi = "isSpentSerialNumber(uint256)")]
    pub struct IsSpentSerialNumberCall {
        pub serial_number: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isVerifierUpdateDisabled` function with signature `isVerifierUpdateDisabled()` and selector `[78, 176, 105, 247]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isVerifierUpdateDisabled", abi = "isVerifierUpdateDisabled()")]
    pub struct IsVerifierUpdateDisabledCall;
    #[doc = "Container type for all input parameters for the `removeEnqueueWhitelist` function with signature `removeEnqueueWhitelist(address)` and selector `[3, 219, 152, 116]`"]
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
        name = "removeEnqueueWhitelist",
        abi = "removeEnqueueWhitelist(address)"
    )]
    pub struct RemoveEnqueueWhitelistCall {
        pub actor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `removeRollupWhitelist` function with signature `removeRollupWhitelist(address)` and selector `[156, 198, 179, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "removeRollupWhitelist", abi = "removeRollupWhitelist(address)")]
    pub struct RemoveRollupWhitelistCall {
        pub roller: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `rollup` function with signature `rollup((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256))` and selector `[20, 167, 115, 125]`"]
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
        name = "rollup",
        abi = "rollup((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256))"
    )]
    pub struct RollupCall {
        pub request: RollupRequest,
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
    #[doc = "Container type for all input parameters for the `setMinRollupFee` function with signature `setMinRollupFee(uint256)` and selector `[124, 191, 15, 246]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMinRollupFee", abi = "setMinRollupFee(uint256)")]
    pub struct SetMinRollupFeeCall {
        pub min_rollup_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setRollupWhitelistDisabled` function with signature `setRollupWhitelistDisabled(bool)` and selector `[248, 240, 83, 136]`"]
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
        name = "setRollupWhitelistDisabled",
        abi = "setRollupWhitelistDisabled(bool)"
    )]
    pub struct SetRollupWhitelistDisabledCall {
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `setVerifierUpdateDisabled` function with signature `setVerifierUpdateDisabled(bool)` and selector `[179, 183, 86, 49]`"]
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
        name = "setVerifierUpdateDisabled",
        abi = "setVerifierUpdateDisabled(bool)"
    )]
    pub struct SetVerifierUpdateDisabledCall {
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `transact` function with signature `transact((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[]),bytes)` and selector `[114, 8, 41, 113]`"]
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
        name = "transact",
        abi = "transact((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[]),bytes)"
    )]
    pub struct TransactCall {
        pub request: TransactRequest,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `updateAuditorPublicKey` function with signature `updateAuditorPublicKey(uint256,uint256)` and selector `[12, 136, 103, 230]`"]
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
        name = "updateAuditorPublicKey",
        abi = "updateAuditorPublicKey(uint256,uint256)"
    )]
    pub struct UpdateAuditorPublicKeyCall {
        pub index: ethers::core::types::U256,
        pub public_key: ethers::core::types::U256,
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
    pub enum CommitmentPoolERC20Calls {
        PathIndices(PathIndicesCall),
        AddEnqueueWhitelist(AddEnqueueWhitelistCall),
        AddRollupWhitelist(AddRollupWhitelistCall),
        AssetDecimals(AssetDecimalsCall),
        AssetName(AssetNameCall),
        AssetSymbol(AssetSymbolCall),
        AssetType(AssetTypeCall),
        AuditorCount(AuditorCountCall),
        ChangeOperator(ChangeOperatorCall),
        DisableRollupVerifier(DisableRollupVerifierCall),
        DisableSanctionsCheck(DisableSanctionsCheckCall),
        DisableTransactVerifier(DisableTransactVerifierCall),
        EnableRollupVerifier(EnableRollupVerifierCall),
        EnableSanctionsCheck(EnableSanctionsCheckCall),
        EnableTransactVerifier(EnableTransactVerifierCall),
        Enqueue(EnqueueCall),
        GetAllAuditorPublicKeys(GetAllAuditorPublicKeysCall),
        GetAuditorPublicKey(GetAuditorPublicKeyCall),
        GetCommitmentIncludedCount(GetCommitmentIncludedCountCall),
        GetMinRollupFee(GetMinRollupFeeCall),
        GetTreeCapacity(GetTreeCapacityCall),
        IsHistoricCommitment(IsHistoricCommitmentCall),
        IsKnownRoot(IsKnownRootCall),
        IsRollupWhitelistDisabled(IsRollupWhitelistDisabledCall),
        IsSpentSerialNumber(IsSpentSerialNumberCall),
        IsVerifierUpdateDisabled(IsVerifierUpdateDisabledCall),
        RemoveEnqueueWhitelist(RemoveEnqueueWhitelistCall),
        RemoveRollupWhitelist(RemoveRollupWhitelistCall),
        Rollup(RollupCall),
        SanctionsCheck(SanctionsCheckCall),
        SanctionsList(SanctionsListCall),
        SetMinRollupFee(SetMinRollupFeeCall),
        SetRollupWhitelistDisabled(SetRollupWhitelistDisabledCall),
        SetVerifierUpdateDisabled(SetVerifierUpdateDisabledCall),
        Transact(TransactCall),
        UpdateAuditorPublicKey(UpdateAuditorPublicKeyCall),
        UpdateSanctionsListAddress(UpdateSanctionsListAddressCall),
    }
    impl ethers::core::abi::AbiDecode for CommitmentPoolERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <PathIndicesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::PathIndices(decoded));
            }
            if let Ok(decoded) =
                <AddEnqueueWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::AddEnqueueWhitelist(decoded));
            }
            if let Ok(decoded) =
                <AddRollupWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::AddRollupWhitelist(decoded));
            }
            if let Ok(decoded) =
                <AssetDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::AssetDecimals(decoded));
            }
            if let Ok(decoded) =
                <AssetNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::AssetName(decoded));
            }
            if let Ok(decoded) =
                <AssetSymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::AssetSymbol(decoded));
            }
            if let Ok(decoded) =
                <AssetTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::AssetType(decoded));
            }
            if let Ok(decoded) =
                <AuditorCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::AuditorCount(decoded));
            }
            if let Ok(decoded) =
                <ChangeOperatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::ChangeOperator(decoded));
            }
            if let Ok(decoded) =
                <DisableRollupVerifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::DisableRollupVerifier(decoded));
            }
            if let Ok(decoded) =
                <DisableSanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::DisableSanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <DisableTransactVerifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::DisableTransactVerifier(decoded));
            }
            if let Ok(decoded) =
                <EnableRollupVerifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::EnableRollupVerifier(decoded));
            }
            if let Ok(decoded) =
                <EnableSanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::EnableSanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <EnableTransactVerifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::EnableTransactVerifier(decoded));
            }
            if let Ok(decoded) =
                <EnqueueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::Enqueue(decoded));
            }
            if let Ok(decoded) =
                <GetAllAuditorPublicKeysCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::GetAllAuditorPublicKeys(decoded));
            }
            if let Ok(decoded) =
                <GetAuditorPublicKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::GetAuditorPublicKey(decoded));
            }
            if let Ok(decoded) =
                <GetCommitmentIncludedCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CommitmentPoolERC20Calls::GetCommitmentIncludedCount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetMinRollupFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::GetMinRollupFee(decoded));
            }
            if let Ok(decoded) =
                <GetTreeCapacityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::GetTreeCapacity(decoded));
            }
            if let Ok(decoded) =
                <IsHistoricCommitmentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::IsHistoricCommitment(decoded));
            }
            if let Ok(decoded) =
                <IsKnownRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::IsKnownRoot(decoded));
            }
            if let Ok(decoded) =
                <IsRollupWhitelistDisabledCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CommitmentPoolERC20Calls::IsRollupWhitelistDisabled(decoded));
            }
            if let Ok(decoded) =
                <IsSpentSerialNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::IsSpentSerialNumber(decoded));
            }
            if let Ok(decoded) =
                <IsVerifierUpdateDisabledCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CommitmentPoolERC20Calls::IsVerifierUpdateDisabled(decoded));
            }
            if let Ok(decoded) =
                <RemoveEnqueueWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::RemoveEnqueueWhitelist(decoded));
            }
            if let Ok(decoded) =
                <RemoveRollupWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::RemoveRollupWhitelist(decoded));
            }
            if let Ok(decoded) = <RollupCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::Rollup(decoded));
            }
            if let Ok(decoded) =
                <SanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::SanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <SanctionsListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::SanctionsList(decoded));
            }
            if let Ok(decoded) =
                <SetMinRollupFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::SetMinRollupFee(decoded));
            }
            if let Ok(decoded) =
                <SetRollupWhitelistDisabledCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CommitmentPoolERC20Calls::SetRollupWhitelistDisabled(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetVerifierUpdateDisabledCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CommitmentPoolERC20Calls::SetVerifierUpdateDisabled(decoded));
            }
            if let Ok(decoded) =
                <TransactCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::Transact(decoded));
            }
            if let Ok(decoded) =
                <UpdateAuditorPublicKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CommitmentPoolERC20Calls::UpdateAuditorPublicKey(decoded));
            }
            if let Ok(decoded) =
                <UpdateSanctionsListAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CommitmentPoolERC20Calls::UpdateSanctionsListAddress(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CommitmentPoolERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                CommitmentPoolERC20Calls::PathIndices(element) => element.encode(),
                CommitmentPoolERC20Calls::AddEnqueueWhitelist(element) => element.encode(),
                CommitmentPoolERC20Calls::AddRollupWhitelist(element) => element.encode(),
                CommitmentPoolERC20Calls::AssetDecimals(element) => element.encode(),
                CommitmentPoolERC20Calls::AssetName(element) => element.encode(),
                CommitmentPoolERC20Calls::AssetSymbol(element) => element.encode(),
                CommitmentPoolERC20Calls::AssetType(element) => element.encode(),
                CommitmentPoolERC20Calls::AuditorCount(element) => element.encode(),
                CommitmentPoolERC20Calls::ChangeOperator(element) => element.encode(),
                CommitmentPoolERC20Calls::DisableRollupVerifier(element) => element.encode(),
                CommitmentPoolERC20Calls::DisableSanctionsCheck(element) => element.encode(),
                CommitmentPoolERC20Calls::DisableTransactVerifier(element) => element.encode(),
                CommitmentPoolERC20Calls::EnableRollupVerifier(element) => element.encode(),
                CommitmentPoolERC20Calls::EnableSanctionsCheck(element) => element.encode(),
                CommitmentPoolERC20Calls::EnableTransactVerifier(element) => element.encode(),
                CommitmentPoolERC20Calls::Enqueue(element) => element.encode(),
                CommitmentPoolERC20Calls::GetAllAuditorPublicKeys(element) => element.encode(),
                CommitmentPoolERC20Calls::GetAuditorPublicKey(element) => element.encode(),
                CommitmentPoolERC20Calls::GetCommitmentIncludedCount(element) => element.encode(),
                CommitmentPoolERC20Calls::GetMinRollupFee(element) => element.encode(),
                CommitmentPoolERC20Calls::GetTreeCapacity(element) => element.encode(),
                CommitmentPoolERC20Calls::IsHistoricCommitment(element) => element.encode(),
                CommitmentPoolERC20Calls::IsKnownRoot(element) => element.encode(),
                CommitmentPoolERC20Calls::IsRollupWhitelistDisabled(element) => element.encode(),
                CommitmentPoolERC20Calls::IsSpentSerialNumber(element) => element.encode(),
                CommitmentPoolERC20Calls::IsVerifierUpdateDisabled(element) => element.encode(),
                CommitmentPoolERC20Calls::RemoveEnqueueWhitelist(element) => element.encode(),
                CommitmentPoolERC20Calls::RemoveRollupWhitelist(element) => element.encode(),
                CommitmentPoolERC20Calls::Rollup(element) => element.encode(),
                CommitmentPoolERC20Calls::SanctionsCheck(element) => element.encode(),
                CommitmentPoolERC20Calls::SanctionsList(element) => element.encode(),
                CommitmentPoolERC20Calls::SetMinRollupFee(element) => element.encode(),
                CommitmentPoolERC20Calls::SetRollupWhitelistDisabled(element) => element.encode(),
                CommitmentPoolERC20Calls::SetVerifierUpdateDisabled(element) => element.encode(),
                CommitmentPoolERC20Calls::Transact(element) => element.encode(),
                CommitmentPoolERC20Calls::UpdateAuditorPublicKey(element) => element.encode(),
                CommitmentPoolERC20Calls::UpdateSanctionsListAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CommitmentPoolERC20Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CommitmentPoolERC20Calls::PathIndices(element) => element.fmt(f),
                CommitmentPoolERC20Calls::AddEnqueueWhitelist(element) => element.fmt(f),
                CommitmentPoolERC20Calls::AddRollupWhitelist(element) => element.fmt(f),
                CommitmentPoolERC20Calls::AssetDecimals(element) => element.fmt(f),
                CommitmentPoolERC20Calls::AssetName(element) => element.fmt(f),
                CommitmentPoolERC20Calls::AssetSymbol(element) => element.fmt(f),
                CommitmentPoolERC20Calls::AssetType(element) => element.fmt(f),
                CommitmentPoolERC20Calls::AuditorCount(element) => element.fmt(f),
                CommitmentPoolERC20Calls::ChangeOperator(element) => element.fmt(f),
                CommitmentPoolERC20Calls::DisableRollupVerifier(element) => element.fmt(f),
                CommitmentPoolERC20Calls::DisableSanctionsCheck(element) => element.fmt(f),
                CommitmentPoolERC20Calls::DisableTransactVerifier(element) => element.fmt(f),
                CommitmentPoolERC20Calls::EnableRollupVerifier(element) => element.fmt(f),
                CommitmentPoolERC20Calls::EnableSanctionsCheck(element) => element.fmt(f),
                CommitmentPoolERC20Calls::EnableTransactVerifier(element) => element.fmt(f),
                CommitmentPoolERC20Calls::Enqueue(element) => element.fmt(f),
                CommitmentPoolERC20Calls::GetAllAuditorPublicKeys(element) => element.fmt(f),
                CommitmentPoolERC20Calls::GetAuditorPublicKey(element) => element.fmt(f),
                CommitmentPoolERC20Calls::GetCommitmentIncludedCount(element) => element.fmt(f),
                CommitmentPoolERC20Calls::GetMinRollupFee(element) => element.fmt(f),
                CommitmentPoolERC20Calls::GetTreeCapacity(element) => element.fmt(f),
                CommitmentPoolERC20Calls::IsHistoricCommitment(element) => element.fmt(f),
                CommitmentPoolERC20Calls::IsKnownRoot(element) => element.fmt(f),
                CommitmentPoolERC20Calls::IsRollupWhitelistDisabled(element) => element.fmt(f),
                CommitmentPoolERC20Calls::IsSpentSerialNumber(element) => element.fmt(f),
                CommitmentPoolERC20Calls::IsVerifierUpdateDisabled(element) => element.fmt(f),
                CommitmentPoolERC20Calls::RemoveEnqueueWhitelist(element) => element.fmt(f),
                CommitmentPoolERC20Calls::RemoveRollupWhitelist(element) => element.fmt(f),
                CommitmentPoolERC20Calls::Rollup(element) => element.fmt(f),
                CommitmentPoolERC20Calls::SanctionsCheck(element) => element.fmt(f),
                CommitmentPoolERC20Calls::SanctionsList(element) => element.fmt(f),
                CommitmentPoolERC20Calls::SetMinRollupFee(element) => element.fmt(f),
                CommitmentPoolERC20Calls::SetRollupWhitelistDisabled(element) => element.fmt(f),
                CommitmentPoolERC20Calls::SetVerifierUpdateDisabled(element) => element.fmt(f),
                CommitmentPoolERC20Calls::Transact(element) => element.fmt(f),
                CommitmentPoolERC20Calls::UpdateAuditorPublicKey(element) => element.fmt(f),
                CommitmentPoolERC20Calls::UpdateSanctionsListAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PathIndicesCall> for CommitmentPoolERC20Calls {
        fn from(var: PathIndicesCall) -> Self {
            CommitmentPoolERC20Calls::PathIndices(var)
        }
    }
    impl ::std::convert::From<AddEnqueueWhitelistCall> for CommitmentPoolERC20Calls {
        fn from(var: AddEnqueueWhitelistCall) -> Self {
            CommitmentPoolERC20Calls::AddEnqueueWhitelist(var)
        }
    }
    impl ::std::convert::From<AddRollupWhitelistCall> for CommitmentPoolERC20Calls {
        fn from(var: AddRollupWhitelistCall) -> Self {
            CommitmentPoolERC20Calls::AddRollupWhitelist(var)
        }
    }
    impl ::std::convert::From<AssetDecimalsCall> for CommitmentPoolERC20Calls {
        fn from(var: AssetDecimalsCall) -> Self {
            CommitmentPoolERC20Calls::AssetDecimals(var)
        }
    }
    impl ::std::convert::From<AssetNameCall> for CommitmentPoolERC20Calls {
        fn from(var: AssetNameCall) -> Self {
            CommitmentPoolERC20Calls::AssetName(var)
        }
    }
    impl ::std::convert::From<AssetSymbolCall> for CommitmentPoolERC20Calls {
        fn from(var: AssetSymbolCall) -> Self {
            CommitmentPoolERC20Calls::AssetSymbol(var)
        }
    }
    impl ::std::convert::From<AssetTypeCall> for CommitmentPoolERC20Calls {
        fn from(var: AssetTypeCall) -> Self {
            CommitmentPoolERC20Calls::AssetType(var)
        }
    }
    impl ::std::convert::From<AuditorCountCall> for CommitmentPoolERC20Calls {
        fn from(var: AuditorCountCall) -> Self {
            CommitmentPoolERC20Calls::AuditorCount(var)
        }
    }
    impl ::std::convert::From<ChangeOperatorCall> for CommitmentPoolERC20Calls {
        fn from(var: ChangeOperatorCall) -> Self {
            CommitmentPoolERC20Calls::ChangeOperator(var)
        }
    }
    impl ::std::convert::From<DisableRollupVerifierCall> for CommitmentPoolERC20Calls {
        fn from(var: DisableRollupVerifierCall) -> Self {
            CommitmentPoolERC20Calls::DisableRollupVerifier(var)
        }
    }
    impl ::std::convert::From<DisableSanctionsCheckCall> for CommitmentPoolERC20Calls {
        fn from(var: DisableSanctionsCheckCall) -> Self {
            CommitmentPoolERC20Calls::DisableSanctionsCheck(var)
        }
    }
    impl ::std::convert::From<DisableTransactVerifierCall> for CommitmentPoolERC20Calls {
        fn from(var: DisableTransactVerifierCall) -> Self {
            CommitmentPoolERC20Calls::DisableTransactVerifier(var)
        }
    }
    impl ::std::convert::From<EnableRollupVerifierCall> for CommitmentPoolERC20Calls {
        fn from(var: EnableRollupVerifierCall) -> Self {
            CommitmentPoolERC20Calls::EnableRollupVerifier(var)
        }
    }
    impl ::std::convert::From<EnableSanctionsCheckCall> for CommitmentPoolERC20Calls {
        fn from(var: EnableSanctionsCheckCall) -> Self {
            CommitmentPoolERC20Calls::EnableSanctionsCheck(var)
        }
    }
    impl ::std::convert::From<EnableTransactVerifierCall> for CommitmentPoolERC20Calls {
        fn from(var: EnableTransactVerifierCall) -> Self {
            CommitmentPoolERC20Calls::EnableTransactVerifier(var)
        }
    }
    impl ::std::convert::From<EnqueueCall> for CommitmentPoolERC20Calls {
        fn from(var: EnqueueCall) -> Self {
            CommitmentPoolERC20Calls::Enqueue(var)
        }
    }
    impl ::std::convert::From<GetAllAuditorPublicKeysCall> for CommitmentPoolERC20Calls {
        fn from(var: GetAllAuditorPublicKeysCall) -> Self {
            CommitmentPoolERC20Calls::GetAllAuditorPublicKeys(var)
        }
    }
    impl ::std::convert::From<GetAuditorPublicKeyCall> for CommitmentPoolERC20Calls {
        fn from(var: GetAuditorPublicKeyCall) -> Self {
            CommitmentPoolERC20Calls::GetAuditorPublicKey(var)
        }
    }
    impl ::std::convert::From<GetCommitmentIncludedCountCall> for CommitmentPoolERC20Calls {
        fn from(var: GetCommitmentIncludedCountCall) -> Self {
            CommitmentPoolERC20Calls::GetCommitmentIncludedCount(var)
        }
    }
    impl ::std::convert::From<GetMinRollupFeeCall> for CommitmentPoolERC20Calls {
        fn from(var: GetMinRollupFeeCall) -> Self {
            CommitmentPoolERC20Calls::GetMinRollupFee(var)
        }
    }
    impl ::std::convert::From<GetTreeCapacityCall> for CommitmentPoolERC20Calls {
        fn from(var: GetTreeCapacityCall) -> Self {
            CommitmentPoolERC20Calls::GetTreeCapacity(var)
        }
    }
    impl ::std::convert::From<IsHistoricCommitmentCall> for CommitmentPoolERC20Calls {
        fn from(var: IsHistoricCommitmentCall) -> Self {
            CommitmentPoolERC20Calls::IsHistoricCommitment(var)
        }
    }
    impl ::std::convert::From<IsKnownRootCall> for CommitmentPoolERC20Calls {
        fn from(var: IsKnownRootCall) -> Self {
            CommitmentPoolERC20Calls::IsKnownRoot(var)
        }
    }
    impl ::std::convert::From<IsRollupWhitelistDisabledCall> for CommitmentPoolERC20Calls {
        fn from(var: IsRollupWhitelistDisabledCall) -> Self {
            CommitmentPoolERC20Calls::IsRollupWhitelistDisabled(var)
        }
    }
    impl ::std::convert::From<IsSpentSerialNumberCall> for CommitmentPoolERC20Calls {
        fn from(var: IsSpentSerialNumberCall) -> Self {
            CommitmentPoolERC20Calls::IsSpentSerialNumber(var)
        }
    }
    impl ::std::convert::From<IsVerifierUpdateDisabledCall> for CommitmentPoolERC20Calls {
        fn from(var: IsVerifierUpdateDisabledCall) -> Self {
            CommitmentPoolERC20Calls::IsVerifierUpdateDisabled(var)
        }
    }
    impl ::std::convert::From<RemoveEnqueueWhitelistCall> for CommitmentPoolERC20Calls {
        fn from(var: RemoveEnqueueWhitelistCall) -> Self {
            CommitmentPoolERC20Calls::RemoveEnqueueWhitelist(var)
        }
    }
    impl ::std::convert::From<RemoveRollupWhitelistCall> for CommitmentPoolERC20Calls {
        fn from(var: RemoveRollupWhitelistCall) -> Self {
            CommitmentPoolERC20Calls::RemoveRollupWhitelist(var)
        }
    }
    impl ::std::convert::From<RollupCall> for CommitmentPoolERC20Calls {
        fn from(var: RollupCall) -> Self {
            CommitmentPoolERC20Calls::Rollup(var)
        }
    }
    impl ::std::convert::From<SanctionsCheckCall> for CommitmentPoolERC20Calls {
        fn from(var: SanctionsCheckCall) -> Self {
            CommitmentPoolERC20Calls::SanctionsCheck(var)
        }
    }
    impl ::std::convert::From<SanctionsListCall> for CommitmentPoolERC20Calls {
        fn from(var: SanctionsListCall) -> Self {
            CommitmentPoolERC20Calls::SanctionsList(var)
        }
    }
    impl ::std::convert::From<SetMinRollupFeeCall> for CommitmentPoolERC20Calls {
        fn from(var: SetMinRollupFeeCall) -> Self {
            CommitmentPoolERC20Calls::SetMinRollupFee(var)
        }
    }
    impl ::std::convert::From<SetRollupWhitelistDisabledCall> for CommitmentPoolERC20Calls {
        fn from(var: SetRollupWhitelistDisabledCall) -> Self {
            CommitmentPoolERC20Calls::SetRollupWhitelistDisabled(var)
        }
    }
    impl ::std::convert::From<SetVerifierUpdateDisabledCall> for CommitmentPoolERC20Calls {
        fn from(var: SetVerifierUpdateDisabledCall) -> Self {
            CommitmentPoolERC20Calls::SetVerifierUpdateDisabled(var)
        }
    }
    impl ::std::convert::From<TransactCall> for CommitmentPoolERC20Calls {
        fn from(var: TransactCall) -> Self {
            CommitmentPoolERC20Calls::Transact(var)
        }
    }
    impl ::std::convert::From<UpdateAuditorPublicKeyCall> for CommitmentPoolERC20Calls {
        fn from(var: UpdateAuditorPublicKeyCall) -> Self {
            CommitmentPoolERC20Calls::UpdateAuditorPublicKey(var)
        }
    }
    impl ::std::convert::From<UpdateSanctionsListAddressCall> for CommitmentPoolERC20Calls {
        fn from(var: UpdateSanctionsListAddressCall) -> Self {
            CommitmentPoolERC20Calls::UpdateSanctionsListAddress(var)
        }
    }
    #[doc = "Container type for all return fields from the `_pathIndices` function with signature `_pathIndices(uint256,uint32)` and selector `[242, 218, 29, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PathIndicesReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `auditorCount` function with signature `auditorCount()` and selector `[17, 95, 87, 76]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AuditorCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getAllAuditorPublicKeys` function with signature `getAllAuditorPublicKeys()` and selector `[99, 188, 125, 50]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAllAuditorPublicKeysReturn(pub ::std::vec::Vec<ethers::core::types::U256>);
    #[doc = "Container type for all return fields from the `getAuditorPublicKey` function with signature `getAuditorPublicKey(uint256)` and selector `[135, 120, 13, 249]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAuditorPublicKeyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getCommitmentIncludedCount` function with signature `getCommitmentIncludedCount()` and selector `[229, 0, 245, 4]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCommitmentIncludedCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getMinRollupFee` function with signature `getMinRollupFee()` and selector `[176, 136, 146, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMinRollupFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getTreeCapacity` function with signature `getTreeCapacity()` and selector `[72, 78, 182, 82]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTreeCapacityReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isHistoricCommitment` function with signature `isHistoricCommitment(uint256)` and selector `[87, 6, 0, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsHistoricCommitmentReturn(pub bool);
    #[doc = "Container type for all return fields from the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `[166, 35, 42, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsKnownRootReturn(pub bool);
    #[doc = "Container type for all return fields from the `isRollupWhitelistDisabled` function with signature `isRollupWhitelistDisabled()` and selector `[255, 168, 155, 136]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsRollupWhitelistDisabledReturn(pub bool);
    #[doc = "Container type for all return fields from the `isSpentSerialNumber` function with signature `isSpentSerialNumber(uint256)` and selector `[59, 184, 209, 180]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsSpentSerialNumberReturn(pub bool);
    #[doc = "Container type for all return fields from the `isVerifierUpdateDisabled` function with signature `isVerifierUpdateDisabled()` and selector `[78, 176, 105, 247]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsVerifierUpdateDisabledReturn(pub bool);
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
    #[doc = "`AuditorNote(uint64,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AuditorNote {
        pub id: u64,
        pub public_key: ethers::core::types::U256,
        pub note: ethers::core::types::U256,
    }
    #[doc = "`CommitmentRequest(uint256,uint256,uint256,uint256,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CommitmentRequest {
        pub amount: ethers::core::types::U256,
        pub commitment: ethers::core::types::U256,
        pub executor_fee: ethers::core::types::U256,
        pub rollup_fee: ethers::core::types::U256,
        pub encrypted_note: ethers::core::types::Bytes,
    }
    #[doc = "`RollupRequest(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RollupRequest {
        pub proof: Proof,
        pub rollup_size: u32,
        pub new_root: ethers::core::types::U256,
        pub leaf_hash: ethers::core::types::U256,
    }
    #[doc = "`TransactRequest(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransactRequest {
        pub proof: Proof,
        pub root_hash: ethers::core::types::U256,
        pub serial_numbers: Vec<ethers::core::types::U256>,
        pub sig_hashes: Vec<ethers::core::types::U256>,
        pub sig_pk: [u8; 32],
        pub public_amount: ethers::core::types::U256,
        pub relayer_fee_amount: ethers::core::types::U256,
        pub out_commitments: Vec<ethers::core::types::U256>,
        pub out_rollup_fees: Vec<ethers::core::types::U256>,
        pub public_recipient: ethers::core::types::Address,
        pub relayer_address: ethers::core::types::Address,
        pub out_encrypted_notes: Vec<ethers::core::types::Bytes>,
        pub random_auditing_public_key: ethers::core::types::U256,
        pub encrypted_auditor_notes: Vec<ethers::core::types::U256>,
    }
    #[doc = "`G1Point(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct G1Point {
        pub x: ethers::core::types::U256,
        pub y: ethers::core::types::U256,
    }
    #[doc = "`G2Point(uint256[2],uint256[2])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct G2Point {
        pub x: [ethers::core::types::U256; 2],
        pub y: [ethers::core::types::U256; 2],
    }
    #[doc = "`Proof((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Proof {
        pub a: G1Point,
        pub b: G2Point,
        pub c: G1Point,
    }
}
