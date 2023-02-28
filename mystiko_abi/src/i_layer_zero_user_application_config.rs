pub use i_layer_zero_user_application_config::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_layer_zero_user_application_config {
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
    #[doc = "ILayerZeroUserApplicationConfig was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forceResumeReceive\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_chainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_configType\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_config\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setConfig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReceiveVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSendVersion\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ILAYERZEROUSERAPPLICATIONCONFIG_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ILayerZeroUserApplicationConfig<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ILayerZeroUserApplicationConfig<M> {
        fn clone(&self) -> Self {
            ILayerZeroUserApplicationConfig(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ILayerZeroUserApplicationConfig<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ILayerZeroUserApplicationConfig<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ILayerZeroUserApplicationConfig))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ILayerZeroUserApplicationConfig<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                ILAYERZEROUSERAPPLICATIONCONFIG_ABI.clone(),
                client,
            )
            .into()
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ILayerZeroUserApplicationConfig<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum ILayerZeroUserApplicationConfigCalls {
        ForceResumeReceive(ForceResumeReceiveCall),
        SetConfig(SetConfigCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetSendVersion(SetSendVersionCall),
    }
    impl ethers::core::abi::AbiDecode for ILayerZeroUserApplicationConfigCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ForceResumeReceiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroUserApplicationConfigCalls::ForceResumeReceive(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroUserApplicationConfigCalls::SetConfig(decoded));
            }
            if let Ok(decoded) =
                <SetReceiveVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroUserApplicationConfigCalls::SetReceiveVersion(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetSendVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILayerZeroUserApplicationConfigCalls::SetSendVersion(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ILayerZeroUserApplicationConfigCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ILayerZeroUserApplicationConfigCalls::ForceResumeReceive(element) => {
                    element.encode()
                }
                ILayerZeroUserApplicationConfigCalls::SetConfig(element) => element.encode(),
                ILayerZeroUserApplicationConfigCalls::SetReceiveVersion(element) => {
                    element.encode()
                }
                ILayerZeroUserApplicationConfigCalls::SetSendVersion(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ILayerZeroUserApplicationConfigCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ILayerZeroUserApplicationConfigCalls::ForceResumeReceive(element) => element.fmt(f),
                ILayerZeroUserApplicationConfigCalls::SetConfig(element) => element.fmt(f),
                ILayerZeroUserApplicationConfigCalls::SetReceiveVersion(element) => element.fmt(f),
                ILayerZeroUserApplicationConfigCalls::SetSendVersion(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ForceResumeReceiveCall> for ILayerZeroUserApplicationConfigCalls {
        fn from(var: ForceResumeReceiveCall) -> Self {
            ILayerZeroUserApplicationConfigCalls::ForceResumeReceive(var)
        }
    }
    impl ::std::convert::From<SetConfigCall> for ILayerZeroUserApplicationConfigCalls {
        fn from(var: SetConfigCall) -> Self {
            ILayerZeroUserApplicationConfigCalls::SetConfig(var)
        }
    }
    impl ::std::convert::From<SetReceiveVersionCall> for ILayerZeroUserApplicationConfigCalls {
        fn from(var: SetReceiveVersionCall) -> Self {
            ILayerZeroUserApplicationConfigCalls::SetReceiveVersion(var)
        }
    }
    impl ::std::convert::From<SetSendVersionCall> for ILayerZeroUserApplicationConfigCalls {
        fn from(var: SetSendVersionCall) -> Self {
            ILayerZeroUserApplicationConfigCalls::SetSendVersion(var)
        }
    }
}
