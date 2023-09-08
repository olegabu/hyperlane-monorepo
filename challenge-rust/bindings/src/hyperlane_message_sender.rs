pub use hyperlane_message_sender::*;
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
pub mod hyperlane_message_sender {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_outbox"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("sendString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("SentMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SentMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destinationDomain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static HYPERLANEMESSAGESENDER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x02\xCF8\x03\x80a\x02\xCF\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x02<\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xAA\xFA\n\x87\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01\x02V[a\0EV[\0[`\0T`@Qc\xFA1\xDE\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFA1\xDE\x01\x90a\0{\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x01\xBDV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xBE\x91\x90a\x01\xEDV[P\x7F\x10\x87Sr\xA0;~\\}F\xDE\xB5H\xB1\x12\xC0\x9CWn7\x8C\x8F-\xAA3p\xA5\xCB\xE5\x8CA\xB6\x84\x84\x84\x84`@Qa\0\xF4\x94\x93\x92\x91\x90a\x01\xBDV[`@Q\x80\x91\x03\x90\xA1PPPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x01\x18W`\0\x80\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01,W`\0\x80\xFD[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01PW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x01dW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01sW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x01\x85W`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x01\xE3``\x83\x01\x84\x86a\x01\x94V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x01\xFFW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \xE1\x84\x13*e\x84\x19\xBA\x94\xD8\xD6)\x8Ew\xC3\x9B\x93\xA0L$-I\x08(\xE2\xC5\x89\xA2\xBD\x9D@TdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static HYPERLANEMESSAGESENDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xAA\xFA\n\x87\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01\x02V[a\0EV[\0[`\0T`@Qc\xFA1\xDE\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFA1\xDE\x01\x90a\0{\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x01\xBDV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xBE\x91\x90a\x01\xEDV[P\x7F\x10\x87Sr\xA0;~\\}F\xDE\xB5H\xB1\x12\xC0\x9CWn7\x8C\x8F-\xAA3p\xA5\xCB\xE5\x8CA\xB6\x84\x84\x84\x84`@Qa\0\xF4\x94\x93\x92\x91\x90a\x01\xBDV[`@Q\x80\x91\x03\x90\xA1PPPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x01\x18W`\0\x80\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01,W`\0\x80\xFD[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01PW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x01dW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01sW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x01\x85W`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x01\xE3``\x83\x01\x84\x86a\x01\x94V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x01\xFFW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \xE1\x84\x13*e\x84\x19\xBA\x94\xD8\xD6)\x8Ew\xC3\x9B\x93\xA0L$-I\x08(\xE2\xC5\x89\xA2\xBD\x9D@TdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static HYPERLANEMESSAGESENDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct HyperlaneMessageSender<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for HyperlaneMessageSender<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HyperlaneMessageSender<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HyperlaneMessageSender<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HyperlaneMessageSender<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(HyperlaneMessageSender))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HyperlaneMessageSender<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    HYPERLANEMESSAGESENDER_ABI.clone(),
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
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                HYPERLANEMESSAGESENDER_ABI.clone(),
                HYPERLANEMESSAGESENDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `sendString` (0xaafa0a87) function
        pub fn send_string(
            &self,
            destination_domain: u32,
            recipient: [u8; 32],
            message: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [170, 250, 10, 135],
                    (destination_domain, recipient, message),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SentMessage` event
        pub fn sent_message_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SentMessageFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SentMessageFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for HyperlaneMessageSender<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SentMessage", abi = "SentMessage(uint32,bytes32,string)")]
    pub struct SentMessageFilter {
        pub destination_domain: u32,
        pub recipient: [u8; 32],
        pub message: ::std::string::String,
    }
    ///Container type for all input parameters for the `sendString` function with signature `sendString(uint32,bytes32,string)` and selector `0xaafa0a87`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sendString", abi = "sendString(uint32,bytes32,string)")]
    pub struct SendStringCall {
        pub destination_domain: u32,
        pub recipient: [u8; 32],
        pub message: ::std::string::String,
    }
}
