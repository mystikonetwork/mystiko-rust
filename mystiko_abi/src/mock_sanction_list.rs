pub use mock_sanction_list::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod mock_sanction_list {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addToSanctionsList"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addToSanctionsList"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("addr"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isSanctioned"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isSanctioned"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("addr"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeFromSanctionsList"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeFromSanctionsList",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("addr"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKSANCTIONLIST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x01\xBB\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x004W`\x005`\xE0\x1C\x80c%\x18k\xD0\x14a\0\x8BW\x80c\x93>,\x93\x14a\0\xCCW\x80c\xDFY/}\x14a\x01\x08W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FFallback function MockSanctionLi`D\x82\x01Ra\x1C\xDD`\xF2\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[4\x80\x15a\0\x97W`\0\x80\xFD[Pa\0\xCAa\0\xA66`\x04a\x01UV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[\0[4\x80\x15a\0\xD8W`\0\x80\xFD[Pa\0\xCAa\0\xE76`\x04a\x01UV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16\x90UV[4\x80\x15a\x01\x14W`\0\x80\xFD[Pa\x01Aa\x01#6`\x04a\x01UV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0` \x82\x84\x03\x12\x15a\x01gW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01~W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xE7\xE3\xB3\xD3L#1[h\x81\x9B\xC3\x11\xFC\x8D\xEDR\xE7\xB2\xAAH'\xEA\xF5\xD0\xCAe@q\xEA\x8F\xFCdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MOCKSANCTIONLIST_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x004W`\x005`\xE0\x1C\x80c%\x18k\xD0\x14a\0\x8BW\x80c\x93>,\x93\x14a\0\xCCW\x80c\xDFY/}\x14a\x01\x08W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FFallback function MockSanctionLi`D\x82\x01Ra\x1C\xDD`\xF2\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[4\x80\x15a\0\x97W`\0\x80\xFD[Pa\0\xCAa\0\xA66`\x04a\x01UV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[\0[4\x80\x15a\0\xD8W`\0\x80\xFD[Pa\0\xCAa\0\xE76`\x04a\x01UV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16\x90UV[4\x80\x15a\x01\x14W`\0\x80\xFD[Pa\x01Aa\x01#6`\x04a\x01UV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0` \x82\x84\x03\x12\x15a\x01gW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01~W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xE7\xE3\xB3\xD3L#1[h\x81\x9B\xC3\x11\xFC\x8D\xEDR\xE7\xB2\xAAH'\xEA\xF5\xD0\xCAe@q\xEA\x8F\xFCdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKSANCTIONLIST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockSanctionList<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockSanctionList<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockSanctionList<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockSanctionList<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockSanctionList<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockSanctionList))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockSanctionList<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKSANCTIONLIST_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MOCKSANCTIONLIST_ABI.clone(),
                MOCKSANCTIONLIST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addToSanctionsList` (0x25186bd0) function
        pub fn add_to_sanctions_list(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 24, 107, 208], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSanctioned` (0xdf592f7d) function
        pub fn is_sanctioned(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([223, 89, 47, 125], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeFromSanctionsList` (0x933e2c93) function
        pub fn remove_from_sanctions_list(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 62, 44, 147], addr)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockSanctionList<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `addToSanctionsList` function with signature `addToSanctionsList(address)` and selector `0x25186bd0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "addToSanctionsList", abi = "addToSanctionsList(address)")]
    pub struct AddToSanctionsListCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isSanctioned` function with signature `isSanctioned(address)` and selector `0xdf592f7d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isSanctioned", abi = "isSanctioned(address)")]
    pub struct IsSanctionedCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeFromSanctionsList` function with signature `removeFromSanctionsList(address)` and selector `0x933e2c93`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "removeFromSanctionsList", abi = "removeFromSanctionsList(address)")]
    pub struct RemoveFromSanctionsListCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum MockSanctionListCalls {
        AddToSanctionsList(AddToSanctionsListCall),
        IsSanctioned(IsSanctionedCall),
        RemoveFromSanctionsList(RemoveFromSanctionsListCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockSanctionListCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddToSanctionsListCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddToSanctionsList(decoded));
            }
            if let Ok(decoded) = <IsSanctionedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsSanctioned(decoded));
            }
            if let Ok(decoded) = <RemoveFromSanctionsListCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveFromSanctionsList(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockSanctionListCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddToSanctionsList(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsSanctioned(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveFromSanctionsList(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockSanctionListCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddToSanctionsList(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSanctioned(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveFromSanctionsList(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddToSanctionsListCall> for MockSanctionListCalls {
        fn from(value: AddToSanctionsListCall) -> Self {
            Self::AddToSanctionsList(value)
        }
    }
    impl ::core::convert::From<IsSanctionedCall> for MockSanctionListCalls {
        fn from(value: IsSanctionedCall) -> Self {
            Self::IsSanctioned(value)
        }
    }
    impl ::core::convert::From<RemoveFromSanctionsListCall> for MockSanctionListCalls {
        fn from(value: RemoveFromSanctionsListCall) -> Self {
            Self::RemoveFromSanctionsList(value)
        }
    }
    ///Container type for all return fields from the `isSanctioned` function with signature `isSanctioned(address)` and selector `0xdf592f7d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsSanctionedReturn(pub bool);
}
