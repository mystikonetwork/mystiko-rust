pub use message_sender_lib::*;
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
pub mod message_sender_lib {
    const _: () = {
        ::core::include_bytes!("../json/MessageSenderLib.json",);
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MESSAGESENDERLIB_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`V`7`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`*WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xF7\xDF\x04\x8F\xE3\x0B\x06\xF5\xD7\xBA\x87Q\xC4\xB8D\x97~\xCF!\xBE\xED\xBF\xA0,\xE0\xBAi\x83-sL\xDAdsolcC\0\x08\x07\x003";
    /// The bytecode of the contract.
    pub static MESSAGESENDERLIB_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xF7\xDF\x04\x8F\xE3\x0B\x06\xF5\xD7\xBA\x87Q\xC4\xB8D\x97~\xCF!\xBE\xED\xBF\xA0,\xE0\xBAi\x83-sL\xDAdsolcC\0\x08\x07\x003";
    /// The deployed bytecode of the contract.
    pub static MESSAGESENDERLIB_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes =
        ::ethers_core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MessageSenderLib<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MessageSenderLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MessageSenderLib<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MessageSenderLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MessageSenderLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MessageSenderLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> MessageSenderLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                MESSAGESENDERLIB_ABI.clone(),
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
        pub fn deploy<T: ::ethers_core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers_contract::builders::ContractDeployer<M, Self>,
            ::ethers_contract::ContractError<M>,
        > {
            let factory = ::ethers_contract::ContractFactory::new(
                MESSAGESENDERLIB_ABI.clone(),
                MESSAGESENDERLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for MessageSenderLib<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
