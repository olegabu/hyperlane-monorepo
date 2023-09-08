pub use i_mailbox::*;
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
pub mod i_mailbox {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("count"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("count"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultIsm"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultIsm"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IInterchainSecurityModule",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delivered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delivered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dispatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dispatch"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_recipientAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_messageBody"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestCheckpoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestCheckpoint"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("localDomain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("localDomain"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("process"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("process"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_metadata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recipientIsm"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recipientIsm"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IInterchainSecurityModule",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("root"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("root"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Dispatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Dispatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DispatchId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DispatchId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Process"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Process"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("origin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProcessId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProcessId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
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
    pub static IMAILBOX_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IMailbox<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMailbox<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMailbox<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IMailbox<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IMailbox<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IMailbox)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IMailbox<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IMAILBOX_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `count` (0x06661abd) function
        pub fn count(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([6, 102, 26, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultIsm` (0x6e5f516e) function
        pub fn default_ism(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([110, 95, 81, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delivered` (0xe495f1d4) function
        pub fn delivered(
            &self,
            message_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([228, 149, 241, 212], message_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dispatch` (0xfa31de01) function
        pub fn dispatch(
            &self,
            destination_domain: u32,
            recipient_address: [u8; 32],
            message_body: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [250, 49, 222, 1],
                    (destination_domain, recipient_address, message_body),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCheckpoint` (0x907c0f92) function
        pub fn latest_checkpoint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], u32)> {
            self.0
                .method_hash([144, 124, 15, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `localDomain` (0x8d3638f4) function
        pub fn local_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([141, 54, 56, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `process` (0x7c39d130) function
        pub fn process(
            &self,
            metadata: ::ethers::core::types::Bytes,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 57, 209, 48], (metadata, message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recipientIsm` (0xe70f48ac) function
        pub fn recipient_ism(
            &self,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([231, 15, 72, 172], recipient)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `root` (0xebf0c717) function
        pub fn root(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([235, 240, 199, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Dispatch` event
        pub fn dispatch_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DispatchFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DispatchId` event
        pub fn dispatch_id_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DispatchIdFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Process` event
        pub fn process_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProcessFilter> {
            self.0.event()
        }
        ///Gets the contract's `ProcessId` event
        pub fn process_id_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProcessIdFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IMailboxEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IMailbox<M> {
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
    #[ethevent(name = "Dispatch", abi = "Dispatch(address,uint32,bytes32,bytes)")]
    pub struct DispatchFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination: u32,
        #[ethevent(indexed)]
        pub recipient: [u8; 32],
        pub message: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "DispatchId", abi = "DispatchId(bytes32)")]
    pub struct DispatchIdFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
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
    #[ethevent(name = "Process", abi = "Process(uint32,bytes32,address)")]
    pub struct ProcessFilter {
        #[ethevent(indexed)]
        pub origin: u32,
        #[ethevent(indexed)]
        pub sender: [u8; 32],
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
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
    #[ethevent(name = "ProcessId", abi = "ProcessId(bytes32)")]
    pub struct ProcessIdFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IMailboxEvents {
        DispatchFilter(DispatchFilter),
        DispatchIdFilter(DispatchIdFilter),
        ProcessFilter(ProcessFilter),
        ProcessIdFilter(ProcessIdFilter),
    }
    impl ::ethers::contract::EthLogDecode for IMailboxEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DispatchFilter::decode_log(log) {
                return Ok(IMailboxEvents::DispatchFilter(decoded));
            }
            if let Ok(decoded) = DispatchIdFilter::decode_log(log) {
                return Ok(IMailboxEvents::DispatchIdFilter(decoded));
            }
            if let Ok(decoded) = ProcessFilter::decode_log(log) {
                return Ok(IMailboxEvents::ProcessFilter(decoded));
            }
            if let Ok(decoded) = ProcessIdFilter::decode_log(log) {
                return Ok(IMailboxEvents::ProcessIdFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IMailboxEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DispatchFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DispatchIdFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessIdFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DispatchFilter> for IMailboxEvents {
        fn from(value: DispatchFilter) -> Self {
            Self::DispatchFilter(value)
        }
    }
    impl ::core::convert::From<DispatchIdFilter> for IMailboxEvents {
        fn from(value: DispatchIdFilter) -> Self {
            Self::DispatchIdFilter(value)
        }
    }
    impl ::core::convert::From<ProcessFilter> for IMailboxEvents {
        fn from(value: ProcessFilter) -> Self {
            Self::ProcessFilter(value)
        }
    }
    impl ::core::convert::From<ProcessIdFilter> for IMailboxEvents {
        fn from(value: ProcessIdFilter) -> Self {
            Self::ProcessIdFilter(value)
        }
    }
    ///Container type for all input parameters for the `count` function with signature `count()` and selector `0x06661abd`
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
    #[ethcall(name = "count", abi = "count()")]
    pub struct CountCall;
    ///Container type for all input parameters for the `defaultIsm` function with signature `defaultIsm()` and selector `0x6e5f516e`
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
    #[ethcall(name = "defaultIsm", abi = "defaultIsm()")]
    pub struct DefaultIsmCall;
    ///Container type for all input parameters for the `delivered` function with signature `delivered(bytes32)` and selector `0xe495f1d4`
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
    #[ethcall(name = "delivered", abi = "delivered(bytes32)")]
    pub struct DeliveredCall {
        pub message_id: [u8; 32],
    }
    ///Container type for all input parameters for the `dispatch` function with signature `dispatch(uint32,bytes32,bytes)` and selector `0xfa31de01`
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
    #[ethcall(name = "dispatch", abi = "dispatch(uint32,bytes32,bytes)")]
    pub struct DispatchCall {
        pub destination_domain: u32,
        pub recipient_address: [u8; 32],
        pub message_body: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `latestCheckpoint` function with signature `latestCheckpoint()` and selector `0x907c0f92`
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
    #[ethcall(name = "latestCheckpoint", abi = "latestCheckpoint()")]
    pub struct LatestCheckpointCall;
    ///Container type for all input parameters for the `localDomain` function with signature `localDomain()` and selector `0x8d3638f4`
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
    #[ethcall(name = "localDomain", abi = "localDomain()")]
    pub struct LocalDomainCall;
    ///Container type for all input parameters for the `process` function with signature `process(bytes,bytes)` and selector `0x7c39d130`
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
    #[ethcall(name = "process", abi = "process(bytes,bytes)")]
    pub struct ProcessCall {
        pub metadata: ::ethers::core::types::Bytes,
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `recipientIsm` function with signature `recipientIsm(address)` and selector `0xe70f48ac`
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
    #[ethcall(name = "recipientIsm", abi = "recipientIsm(address)")]
    pub struct RecipientIsmCall {
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `root` function with signature `root()` and selector `0xebf0c717`
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
    #[ethcall(name = "root", abi = "root()")]
    pub struct RootCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IMailboxCalls {
        Count(CountCall),
        DefaultIsm(DefaultIsmCall),
        Delivered(DeliveredCall),
        Dispatch(DispatchCall),
        LatestCheckpoint(LatestCheckpointCall),
        LocalDomain(LocalDomainCall),
        Process(ProcessCall),
        RecipientIsm(RecipientIsmCall),
        Root(RootCall),
    }
    impl ::ethers::core::abi::AbiDecode for IMailboxCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Count(decoded));
            }
            if let Ok(decoded)
                = <DefaultIsmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultIsm(decoded));
            }
            if let Ok(decoded)
                = <DeliveredCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delivered(decoded));
            }
            if let Ok(decoded)
                = <DispatchCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Dispatch(decoded));
            }
            if let Ok(decoded)
                = <LatestCheckpointCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LatestCheckpoint(decoded));
            }
            if let Ok(decoded)
                = <LocalDomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LocalDomain(decoded));
            }
            if let Ok(decoded)
                = <ProcessCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Process(decoded));
            }
            if let Ok(decoded)
                = <RecipientIsmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RecipientIsm(decoded));
            }
            if let Ok(decoded)
                = <RootCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Root(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IMailboxCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Count(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultIsm(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delivered(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Dispatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCheckpoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LocalDomain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Process(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecipientIsm(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Root(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IMailboxCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Count(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultIsm(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delivered(element) => ::core::fmt::Display::fmt(element, f),
                Self::Dispatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestCheckpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::LocalDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::Process(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecipientIsm(element) => ::core::fmt::Display::fmt(element, f),
                Self::Root(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CountCall> for IMailboxCalls {
        fn from(value: CountCall) -> Self {
            Self::Count(value)
        }
    }
    impl ::core::convert::From<DefaultIsmCall> for IMailboxCalls {
        fn from(value: DefaultIsmCall) -> Self {
            Self::DefaultIsm(value)
        }
    }
    impl ::core::convert::From<DeliveredCall> for IMailboxCalls {
        fn from(value: DeliveredCall) -> Self {
            Self::Delivered(value)
        }
    }
    impl ::core::convert::From<DispatchCall> for IMailboxCalls {
        fn from(value: DispatchCall) -> Self {
            Self::Dispatch(value)
        }
    }
    impl ::core::convert::From<LatestCheckpointCall> for IMailboxCalls {
        fn from(value: LatestCheckpointCall) -> Self {
            Self::LatestCheckpoint(value)
        }
    }
    impl ::core::convert::From<LocalDomainCall> for IMailboxCalls {
        fn from(value: LocalDomainCall) -> Self {
            Self::LocalDomain(value)
        }
    }
    impl ::core::convert::From<ProcessCall> for IMailboxCalls {
        fn from(value: ProcessCall) -> Self {
            Self::Process(value)
        }
    }
    impl ::core::convert::From<RecipientIsmCall> for IMailboxCalls {
        fn from(value: RecipientIsmCall) -> Self {
            Self::RecipientIsm(value)
        }
    }
    impl ::core::convert::From<RootCall> for IMailboxCalls {
        fn from(value: RootCall) -> Self {
            Self::Root(value)
        }
    }
    ///Container type for all return fields from the `count` function with signature `count()` and selector `0x06661abd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CountReturn(pub u32);
    ///Container type for all return fields from the `defaultIsm` function with signature `defaultIsm()` and selector `0x6e5f516e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultIsmReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `delivered` function with signature `delivered(bytes32)` and selector `0xe495f1d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DeliveredReturn(pub bool);
    ///Container type for all return fields from the `dispatch` function with signature `dispatch(uint32,bytes32,bytes)` and selector `0xfa31de01`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DispatchReturn(pub [u8; 32]);
    ///Container type for all return fields from the `latestCheckpoint` function with signature `latestCheckpoint()` and selector `0x907c0f92`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LatestCheckpointReturn(pub [u8; 32], pub u32);
    ///Container type for all return fields from the `localDomain` function with signature `localDomain()` and selector `0x8d3638f4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LocalDomainReturn(pub u32);
    ///Container type for all return fields from the `recipientIsm` function with signature `recipientIsm(address)` and selector `0xe70f48ac`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RecipientIsmReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `root` function with signature `root()` and selector `0xebf0c717`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RootReturn(pub [u8; 32]);
}
