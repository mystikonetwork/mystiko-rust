pub use i_layer_zero_user_application_config::*;
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
pub mod i_layer_zero_user_application_config {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_srcChainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_srcAddress\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forceResumeReceive\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_chainId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_configType\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_config\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setConfig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReceiveVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_version\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSendVersion\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ILAYERZEROUSERAPPLICATIONCONFIG_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct ILayerZeroUserApplicationConfig<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for ILayerZeroUserApplicationConfig<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ILayerZeroUserApplicationConfig<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ILayerZeroUserApplicationConfig<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ILayerZeroUserApplicationConfig<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ILayerZeroUserApplicationConfig))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> ILayerZeroUserApplicationConfig<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    ILAYERZEROUSERAPPLICATIONCONFIG_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `forceResumeReceive` (0x42d65a8d) function
        pub fn force_resume_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 214, 90, 141], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0xcbed8b9c) function
        pub fn set_config(
            &self,
            version: u16,
            chain_id: u16,
            config_type: ::ethers_core::types::U256,
            config: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [203, 237, 139, 156],
                    (version, chain_id, config_type, config),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReceiveVersion` (0x10ddb137) function
        pub fn set_receive_version(
            &self,
            version: u16,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 221, 177, 55], version)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSendVersion` (0x07e0db17) function
        pub fn set_send_version(
            &self,
            version: u16,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 224, 219, 23], version)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for ILayerZeroUserApplicationConfig<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `forceResumeReceive` function with signature `forceResumeReceive(uint16,bytes)` and selector `0x42d65a8d`
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
    #[ethcall(name = "forceResumeReceive", abi = "forceResumeReceive(uint16,bytes)")]
    pub struct ForceResumeReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig(uint16,uint16,uint256,bytes)` and selector `0xcbed8b9c`
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
    #[ethcall(name = "setConfig", abi = "setConfig(uint16,uint16,uint256,bytes)")]
    pub struct SetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub config_type: ::ethers_core::types::U256,
        pub config: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `setReceiveVersion` function with signature `setReceiveVersion(uint16)` and selector `0x10ddb137`
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
    #[ethcall(name = "setReceiveVersion", abi = "setReceiveVersion(uint16)")]
    pub struct SetReceiveVersionCall {
        pub version: u16,
    }
    ///Container type for all input parameters for the `setSendVersion` function with signature `setSendVersion(uint16)` and selector `0x07e0db17`
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
    #[ethcall(name = "setSendVersion", abi = "setSendVersion(uint16)")]
    pub struct SetSendVersionCall {
        pub version: u16,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum ILayerZeroUserApplicationConfigCalls {
        ForceResumeReceive(ForceResumeReceiveCall),
        SetConfig(SetConfigCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetSendVersion(SetSendVersionCall),
    }
    impl ::ethers_core::abi::AbiDecode for ILayerZeroUserApplicationConfigCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ForceResumeReceiveCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ForceResumeReceive(decoded));
            }
            if let Ok(decoded)
                = <SetConfigCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded)
                = <SetReceiveVersionCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReceiveVersion(decoded));
            }
            if let Ok(decoded)
                = <SetSendVersionCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSendVersion(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for ILayerZeroUserApplicationConfigCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ForceResumeReceive(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetConfig(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetReceiveVersion(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetSendVersion(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ILayerZeroUserApplicationConfigCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ForceResumeReceive(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReceiveVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSendVersion(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ForceResumeReceiveCall>
    for ILayerZeroUserApplicationConfigCalls {
        fn from(value: ForceResumeReceiveCall) -> Self {
            Self::ForceResumeReceive(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for ILayerZeroUserApplicationConfigCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetReceiveVersionCall>
    for ILayerZeroUserApplicationConfigCalls {
        fn from(value: SetReceiveVersionCall) -> Self {
            Self::SetReceiveVersion(value)
        }
    }
    impl ::core::convert::From<SetSendVersionCall>
    for ILayerZeroUserApplicationConfigCalls {
        fn from(value: SetSendVersionCall) -> Self {
            Self::SetSendVersion(value)
        }
    }
}
