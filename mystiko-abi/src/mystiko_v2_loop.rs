pub use mystiko_v2_loop::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mystiko_v2_loop {
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
    #[doc = "MystikoV2Loop was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"AmountTooLarge\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AmountTooSmall\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CommitmentHashIncorrect\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"DepositsDisabled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"HashKGreaterThanFieldSize\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MaxAmountLessThanMinAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MinAmountGreaterThanMaxAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotChanged\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyOperator\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RandomSGreaterThanFieldSize\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SanctionedAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DepositsDisabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"maxAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MaxAmount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"minAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinAmount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OperatorChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SanctionsCheck\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"sanctions\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SanctionsList\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetType\",\"outputs\":[{\"internalType\":\"enum AssetPool.AssetType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"bridgeType\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newOperator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeOperator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IMystikoLoop.DepositRequest\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"hashK\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"randomS\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"encryptedNote\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rollupFee\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disableSanctionsCheck\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableSanctionsCheck\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssociatedCommitmentPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMaxAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isDepositsDisabled\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sanctionsCheck\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sanctionsList\",\"outputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_commitmentPoolAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAssociatedCommitmentPool\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDepositsDisabled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_maxAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMaxAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ISanctionsList\",\"name\":\"_sanction\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateSanctionsListAddress\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MYSTIKOV2LOOP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct MystikoV2Loop<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MystikoV2Loop<M> {
        fn clone(&self) -> Self {
            MystikoV2Loop(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MystikoV2Loop<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MystikoV2Loop<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MystikoV2Loop))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MystikoV2Loop<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MYSTIKOV2LOOP_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `assetType` (0x3fe3347a) function"]
        pub fn asset_type(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([63, 227, 52, 122], ())
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
        #[doc = "Calls the contract's `deposit` (0xf6afe88f) function"]
        pub fn deposit(
            &self,
            request: DepositRequest,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 175, 232, 143], (request,))
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
        #[doc = "Calls the contract's `isDepositsDisabled` (0xed6ea33a) function"]
        pub fn is_deposits_disabled(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([237, 110, 163, 58], ())
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
        #[doc = "Calls the contract's `updateSanctionsListAddress` (0x30f49cac) function"]
        pub fn update_sanctions_list_address(
            &self,
            sanction: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 244, 156, 172], sanction)
                .expect("method not found (this should never happen)")
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
        #[doc = "Gets the contract's `OperatorChanged` event"]
        pub fn operator_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OperatorChangedFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, MystikoV2LoopEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MystikoV2Loop<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MystikoV2LoopErrors {
        AmountTooLarge(AmountTooLarge),
        AmountTooSmall(AmountTooSmall),
        CommitmentHashIncorrect(CommitmentHashIncorrect),
        DepositsDisabled(DepositsDisabled),
        HashKGreaterThanFieldSize(HashKGreaterThanFieldSize),
        MaxAmountLessThanMinAmount(MaxAmountLessThanMinAmount),
        MinAmountGreaterThanMaxAmount(MinAmountGreaterThanMaxAmount),
        NotChanged(NotChanged),
        OnlyOperator(OnlyOperator),
        RandomSGreaterThanFieldSize(RandomSGreaterThanFieldSize),
        SanctionedAddress(SanctionedAddress),
    }
    impl ethers::core::abi::AbiDecode for MystikoV2LoopErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AmountTooLarge as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopErrors::AmountTooLarge(decoded));
            }
            if let Ok(decoded) =
                <AmountTooSmall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopErrors::AmountTooSmall(decoded));
            }
            if let Ok(decoded) =
                <CommitmentHashIncorrect as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopErrors::CommitmentHashIncorrect(decoded));
            }
            if let Ok(decoded) =
                <DepositsDisabled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopErrors::DepositsDisabled(decoded));
            }
            if let Ok(decoded) =
                <HashKGreaterThanFieldSize as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopErrors::HashKGreaterThanFieldSize(decoded));
            }
            if let Ok(decoded) =
                <MaxAmountLessThanMinAmount as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopErrors::MaxAmountLessThanMinAmount(decoded));
            }
            if let Ok(decoded) =
                <MinAmountGreaterThanMaxAmount as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2LoopErrors::MinAmountGreaterThanMaxAmount(decoded));
            }
            if let Ok(decoded) = <NotChanged as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopErrors::NotChanged(decoded));
            }
            if let Ok(decoded) =
                <OnlyOperator as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopErrors::OnlyOperator(decoded));
            }
            if let Ok(decoded) =
                <RandomSGreaterThanFieldSize as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopErrors::RandomSGreaterThanFieldSize(decoded));
            }
            if let Ok(decoded) =
                <SanctionedAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopErrors::SanctionedAddress(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MystikoV2LoopErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                MystikoV2LoopErrors::AmountTooLarge(element) => element.encode(),
                MystikoV2LoopErrors::AmountTooSmall(element) => element.encode(),
                MystikoV2LoopErrors::CommitmentHashIncorrect(element) => element.encode(),
                MystikoV2LoopErrors::DepositsDisabled(element) => element.encode(),
                MystikoV2LoopErrors::HashKGreaterThanFieldSize(element) => element.encode(),
                MystikoV2LoopErrors::MaxAmountLessThanMinAmount(element) => element.encode(),
                MystikoV2LoopErrors::MinAmountGreaterThanMaxAmount(element) => element.encode(),
                MystikoV2LoopErrors::NotChanged(element) => element.encode(),
                MystikoV2LoopErrors::OnlyOperator(element) => element.encode(),
                MystikoV2LoopErrors::RandomSGreaterThanFieldSize(element) => element.encode(),
                MystikoV2LoopErrors::SanctionedAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MystikoV2LoopErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoV2LoopErrors::AmountTooLarge(element) => element.fmt(f),
                MystikoV2LoopErrors::AmountTooSmall(element) => element.fmt(f),
                MystikoV2LoopErrors::CommitmentHashIncorrect(element) => element.fmt(f),
                MystikoV2LoopErrors::DepositsDisabled(element) => element.fmt(f),
                MystikoV2LoopErrors::HashKGreaterThanFieldSize(element) => element.fmt(f),
                MystikoV2LoopErrors::MaxAmountLessThanMinAmount(element) => element.fmt(f),
                MystikoV2LoopErrors::MinAmountGreaterThanMaxAmount(element) => element.fmt(f),
                MystikoV2LoopErrors::NotChanged(element) => element.fmt(f),
                MystikoV2LoopErrors::OnlyOperator(element) => element.fmt(f),
                MystikoV2LoopErrors::RandomSGreaterThanFieldSize(element) => element.fmt(f),
                MystikoV2LoopErrors::SanctionedAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AmountTooLarge> for MystikoV2LoopErrors {
        fn from(var: AmountTooLarge) -> Self {
            MystikoV2LoopErrors::AmountTooLarge(var)
        }
    }
    impl ::std::convert::From<AmountTooSmall> for MystikoV2LoopErrors {
        fn from(var: AmountTooSmall) -> Self {
            MystikoV2LoopErrors::AmountTooSmall(var)
        }
    }
    impl ::std::convert::From<CommitmentHashIncorrect> for MystikoV2LoopErrors {
        fn from(var: CommitmentHashIncorrect) -> Self {
            MystikoV2LoopErrors::CommitmentHashIncorrect(var)
        }
    }
    impl ::std::convert::From<DepositsDisabled> for MystikoV2LoopErrors {
        fn from(var: DepositsDisabled) -> Self {
            MystikoV2LoopErrors::DepositsDisabled(var)
        }
    }
    impl ::std::convert::From<HashKGreaterThanFieldSize> for MystikoV2LoopErrors {
        fn from(var: HashKGreaterThanFieldSize) -> Self {
            MystikoV2LoopErrors::HashKGreaterThanFieldSize(var)
        }
    }
    impl ::std::convert::From<MaxAmountLessThanMinAmount> for MystikoV2LoopErrors {
        fn from(var: MaxAmountLessThanMinAmount) -> Self {
            MystikoV2LoopErrors::MaxAmountLessThanMinAmount(var)
        }
    }
    impl ::std::convert::From<MinAmountGreaterThanMaxAmount> for MystikoV2LoopErrors {
        fn from(var: MinAmountGreaterThanMaxAmount) -> Self {
            MystikoV2LoopErrors::MinAmountGreaterThanMaxAmount(var)
        }
    }
    impl ::std::convert::From<NotChanged> for MystikoV2LoopErrors {
        fn from(var: NotChanged) -> Self {
            MystikoV2LoopErrors::NotChanged(var)
        }
    }
    impl ::std::convert::From<OnlyOperator> for MystikoV2LoopErrors {
        fn from(var: OnlyOperator) -> Self {
            MystikoV2LoopErrors::OnlyOperator(var)
        }
    }
    impl ::std::convert::From<RandomSGreaterThanFieldSize> for MystikoV2LoopErrors {
        fn from(var: RandomSGreaterThanFieldSize) -> Self {
            MystikoV2LoopErrors::RandomSGreaterThanFieldSize(var)
        }
    }
    impl ::std::convert::From<SanctionedAddress> for MystikoV2LoopErrors {
        fn from(var: SanctionedAddress) -> Self {
            MystikoV2LoopErrors::SanctionedAddress(var)
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
    pub enum MystikoV2LoopEvents {
        DepositsDisabledFilter(DepositsDisabledFilter),
        MaxAmountFilter(MaxAmountFilter),
        MinAmountFilter(MinAmountFilter),
        OperatorChangedFilter(OperatorChangedFilter),
        SanctionsCheckFilter(SanctionsCheckFilter),
        SanctionsListFilter(SanctionsListFilter),
    }
    impl ethers::contract::EthLogDecode for MystikoV2LoopEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DepositsDisabledFilter::decode_log(log) {
                return Ok(MystikoV2LoopEvents::DepositsDisabledFilter(decoded));
            }
            if let Ok(decoded) = MaxAmountFilter::decode_log(log) {
                return Ok(MystikoV2LoopEvents::MaxAmountFilter(decoded));
            }
            if let Ok(decoded) = MinAmountFilter::decode_log(log) {
                return Ok(MystikoV2LoopEvents::MinAmountFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(MystikoV2LoopEvents::OperatorChangedFilter(decoded));
            }
            if let Ok(decoded) = SanctionsCheckFilter::decode_log(log) {
                return Ok(MystikoV2LoopEvents::SanctionsCheckFilter(decoded));
            }
            if let Ok(decoded) = SanctionsListFilter::decode_log(log) {
                return Ok(MystikoV2LoopEvents::SanctionsListFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MystikoV2LoopEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoV2LoopEvents::DepositsDisabledFilter(element) => element.fmt(f),
                MystikoV2LoopEvents::MaxAmountFilter(element) => element.fmt(f),
                MystikoV2LoopEvents::MinAmountFilter(element) => element.fmt(f),
                MystikoV2LoopEvents::OperatorChangedFilter(element) => element.fmt(f),
                MystikoV2LoopEvents::SanctionsCheckFilter(element) => element.fmt(f),
                MystikoV2LoopEvents::SanctionsListFilter(element) => element.fmt(f),
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
        Default,
    )]
    #[ethcall(name = "assetType", abi = "assetType()")]
    pub struct AssetTypeCall;
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
    #[doc = "Container type for all input parameters for the `deposit` function with signature `deposit((uint256,uint256,uint256,uint128,bytes,uint256))` and selector `[246, 175, 232, 143]`"]
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
        abi = "deposit((uint256,uint256,uint256,uint128,bytes,uint256))"
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
    pub enum MystikoV2LoopCalls {
        AssetType(AssetTypeCall),
        BridgeType(BridgeTypeCall),
        ChangeOperator(ChangeOperatorCall),
        Deposit(DepositCall),
        DisableSanctionsCheck(DisableSanctionsCheckCall),
        EnableSanctionsCheck(EnableSanctionsCheckCall),
        GetAssociatedCommitmentPool(GetAssociatedCommitmentPoolCall),
        GetMaxAmount(GetMaxAmountCall),
        GetMinAmount(GetMinAmountCall),
        IsDepositsDisabled(IsDepositsDisabledCall),
        SanctionsCheck(SanctionsCheckCall),
        SanctionsList(SanctionsListCall),
        SetAssociatedCommitmentPool(SetAssociatedCommitmentPoolCall),
        SetDepositsDisabled(SetDepositsDisabledCall),
        SetMaxAmount(SetMaxAmountCall),
        SetMinAmount(SetMinAmountCall),
        UpdateSanctionsListAddress(UpdateSanctionsListAddressCall),
    }
    impl ethers::core::abi::AbiDecode for MystikoV2LoopCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AssetTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::AssetType(decoded));
            }
            if let Ok(decoded) =
                <BridgeTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::BridgeType(decoded));
            }
            if let Ok(decoded) =
                <ChangeOperatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::ChangeOperator(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DisableSanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::DisableSanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <EnableSanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::EnableSanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <GetAssociatedCommitmentPoolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2LoopCalls::GetAssociatedCommitmentPool(decoded));
            }
            if let Ok(decoded) =
                <GetMaxAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::GetMaxAmount(decoded));
            }
            if let Ok(decoded) =
                <GetMinAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::GetMinAmount(decoded));
            }
            if let Ok(decoded) =
                <IsDepositsDisabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::IsDepositsDisabled(decoded));
            }
            if let Ok(decoded) =
                <SanctionsCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::SanctionsCheck(decoded));
            }
            if let Ok(decoded) =
                <SanctionsListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::SanctionsList(decoded));
            }
            if let Ok(decoded) =
                <SetAssociatedCommitmentPoolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2LoopCalls::SetAssociatedCommitmentPool(decoded));
            }
            if let Ok(decoded) =
                <SetDepositsDisabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::SetDepositsDisabled(decoded));
            }
            if let Ok(decoded) =
                <SetMaxAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::SetMaxAmount(decoded));
            }
            if let Ok(decoded) =
                <SetMinAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MystikoV2LoopCalls::SetMinAmount(decoded));
            }
            if let Ok(decoded) =
                <UpdateSanctionsListAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MystikoV2LoopCalls::UpdateSanctionsListAddress(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MystikoV2LoopCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MystikoV2LoopCalls::AssetType(element) => element.encode(),
                MystikoV2LoopCalls::BridgeType(element) => element.encode(),
                MystikoV2LoopCalls::ChangeOperator(element) => element.encode(),
                MystikoV2LoopCalls::Deposit(element) => element.encode(),
                MystikoV2LoopCalls::DisableSanctionsCheck(element) => element.encode(),
                MystikoV2LoopCalls::EnableSanctionsCheck(element) => element.encode(),
                MystikoV2LoopCalls::GetAssociatedCommitmentPool(element) => element.encode(),
                MystikoV2LoopCalls::GetMaxAmount(element) => element.encode(),
                MystikoV2LoopCalls::GetMinAmount(element) => element.encode(),
                MystikoV2LoopCalls::IsDepositsDisabled(element) => element.encode(),
                MystikoV2LoopCalls::SanctionsCheck(element) => element.encode(),
                MystikoV2LoopCalls::SanctionsList(element) => element.encode(),
                MystikoV2LoopCalls::SetAssociatedCommitmentPool(element) => element.encode(),
                MystikoV2LoopCalls::SetDepositsDisabled(element) => element.encode(),
                MystikoV2LoopCalls::SetMaxAmount(element) => element.encode(),
                MystikoV2LoopCalls::SetMinAmount(element) => element.encode(),
                MystikoV2LoopCalls::UpdateSanctionsListAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MystikoV2LoopCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MystikoV2LoopCalls::AssetType(element) => element.fmt(f),
                MystikoV2LoopCalls::BridgeType(element) => element.fmt(f),
                MystikoV2LoopCalls::ChangeOperator(element) => element.fmt(f),
                MystikoV2LoopCalls::Deposit(element) => element.fmt(f),
                MystikoV2LoopCalls::DisableSanctionsCheck(element) => element.fmt(f),
                MystikoV2LoopCalls::EnableSanctionsCheck(element) => element.fmt(f),
                MystikoV2LoopCalls::GetAssociatedCommitmentPool(element) => element.fmt(f),
                MystikoV2LoopCalls::GetMaxAmount(element) => element.fmt(f),
                MystikoV2LoopCalls::GetMinAmount(element) => element.fmt(f),
                MystikoV2LoopCalls::IsDepositsDisabled(element) => element.fmt(f),
                MystikoV2LoopCalls::SanctionsCheck(element) => element.fmt(f),
                MystikoV2LoopCalls::SanctionsList(element) => element.fmt(f),
                MystikoV2LoopCalls::SetAssociatedCommitmentPool(element) => element.fmt(f),
                MystikoV2LoopCalls::SetDepositsDisabled(element) => element.fmt(f),
                MystikoV2LoopCalls::SetMaxAmount(element) => element.fmt(f),
                MystikoV2LoopCalls::SetMinAmount(element) => element.fmt(f),
                MystikoV2LoopCalls::UpdateSanctionsListAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AssetTypeCall> for MystikoV2LoopCalls {
        fn from(var: AssetTypeCall) -> Self {
            MystikoV2LoopCalls::AssetType(var)
        }
    }
    impl ::std::convert::From<BridgeTypeCall> for MystikoV2LoopCalls {
        fn from(var: BridgeTypeCall) -> Self {
            MystikoV2LoopCalls::BridgeType(var)
        }
    }
    impl ::std::convert::From<ChangeOperatorCall> for MystikoV2LoopCalls {
        fn from(var: ChangeOperatorCall) -> Self {
            MystikoV2LoopCalls::ChangeOperator(var)
        }
    }
    impl ::std::convert::From<DepositCall> for MystikoV2LoopCalls {
        fn from(var: DepositCall) -> Self {
            MystikoV2LoopCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<DisableSanctionsCheckCall> for MystikoV2LoopCalls {
        fn from(var: DisableSanctionsCheckCall) -> Self {
            MystikoV2LoopCalls::DisableSanctionsCheck(var)
        }
    }
    impl ::std::convert::From<EnableSanctionsCheckCall> for MystikoV2LoopCalls {
        fn from(var: EnableSanctionsCheckCall) -> Self {
            MystikoV2LoopCalls::EnableSanctionsCheck(var)
        }
    }
    impl ::std::convert::From<GetAssociatedCommitmentPoolCall> for MystikoV2LoopCalls {
        fn from(var: GetAssociatedCommitmentPoolCall) -> Self {
            MystikoV2LoopCalls::GetAssociatedCommitmentPool(var)
        }
    }
    impl ::std::convert::From<GetMaxAmountCall> for MystikoV2LoopCalls {
        fn from(var: GetMaxAmountCall) -> Self {
            MystikoV2LoopCalls::GetMaxAmount(var)
        }
    }
    impl ::std::convert::From<GetMinAmountCall> for MystikoV2LoopCalls {
        fn from(var: GetMinAmountCall) -> Self {
            MystikoV2LoopCalls::GetMinAmount(var)
        }
    }
    impl ::std::convert::From<IsDepositsDisabledCall> for MystikoV2LoopCalls {
        fn from(var: IsDepositsDisabledCall) -> Self {
            MystikoV2LoopCalls::IsDepositsDisabled(var)
        }
    }
    impl ::std::convert::From<SanctionsCheckCall> for MystikoV2LoopCalls {
        fn from(var: SanctionsCheckCall) -> Self {
            MystikoV2LoopCalls::SanctionsCheck(var)
        }
    }
    impl ::std::convert::From<SanctionsListCall> for MystikoV2LoopCalls {
        fn from(var: SanctionsListCall) -> Self {
            MystikoV2LoopCalls::SanctionsList(var)
        }
    }
    impl ::std::convert::From<SetAssociatedCommitmentPoolCall> for MystikoV2LoopCalls {
        fn from(var: SetAssociatedCommitmentPoolCall) -> Self {
            MystikoV2LoopCalls::SetAssociatedCommitmentPool(var)
        }
    }
    impl ::std::convert::From<SetDepositsDisabledCall> for MystikoV2LoopCalls {
        fn from(var: SetDepositsDisabledCall) -> Self {
            MystikoV2LoopCalls::SetDepositsDisabled(var)
        }
    }
    impl ::std::convert::From<SetMaxAmountCall> for MystikoV2LoopCalls {
        fn from(var: SetMaxAmountCall) -> Self {
            MystikoV2LoopCalls::SetMaxAmount(var)
        }
    }
    impl ::std::convert::From<SetMinAmountCall> for MystikoV2LoopCalls {
        fn from(var: SetMinAmountCall) -> Self {
            MystikoV2LoopCalls::SetMinAmount(var)
        }
    }
    impl ::std::convert::From<UpdateSanctionsListAddressCall> for MystikoV2LoopCalls {
        fn from(var: UpdateSanctionsListAddressCall) -> Self {
            MystikoV2LoopCalls::UpdateSanctionsListAddress(var)
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
        Default,
    )]
    pub struct AssetTypeReturn(pub u8);
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
    #[doc = "`DepositRequest(uint256,uint256,uint256,uint128,bytes,uint256)`"]
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
        pub rollup_fee: ethers::core::types::U256,
    }
}
