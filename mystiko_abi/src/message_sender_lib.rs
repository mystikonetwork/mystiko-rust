pub use message_sender_lib::*;
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
pub mod message_sender_lib {
    #[rustfmt::skip]
    const __ABI: &str = "[]";
    ///The parsed JSON ABI of the contract.
    pub static MESSAGESENDERLIB_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        86,
        96,
        55,
        96,
        11,
        130,
        130,
        130,
        57,
        128,
        81,
        96,
        0,
        26,
        96,
        115,
        20,
        96,
        42,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        0,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        48,
        96,
        0,
        82,
        96,
        115,
        129,
        83,
        130,
        129,
        243,
        254,
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        247,
        223,
        4,
        143,
        227,
        11,
        6,
        245,
        215,
        186,
        135,
        81,
        196,
        184,
        68,
        151,
        126,
        207,
        33,
        190,
        237,
        191,
        160,
        44,
        224,
        186,
        105,
        131,
        45,
        115,
        76,
        218,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        7,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static MESSAGESENDERLIB_BYTECODE: ::ethers_core::types::Bytes = ::ethers_core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        247,
        223,
        4,
        143,
        227,
        11,
        6,
        245,
        215,
        186,
        135,
        81,
        196,
        184,
        68,
        151,
        126,
        207,
        33,
        190,
        237,
        191,
        160,
        44,
        224,
        186,
        105,
        131,
        45,
        115,
        76,
        218,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        7,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static MESSAGESENDERLIB_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes = ::ethers_core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
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
            f.debug_tuple(stringify!(MessageSenderLib)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> MessageSenderLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    MESSAGESENDERLIB_ABI.clone(),
                    client,
                ),
            )
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
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for MessageSenderLib<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
