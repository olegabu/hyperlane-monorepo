pub use mock_mailbox::*;
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
pub mod mock_mailbox {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_domain"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_MESSAGE_BODY_BYTES"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_MESSAGE_BODY_BYTES",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("VERSION"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addInboundMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addInboundMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_origin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_body"),
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
                    ::std::borrow::ToOwned::to_owned("addRemoteMailbox"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addRemoteMailbox"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_domain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_mailbox"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract MockMailbox"),
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
                    ::std::borrow::ToOwned::to_owned("inboundMessages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("inboundMessages"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("origin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("body"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("inboundProcessedNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "inboundProcessedNonce",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("inboundUnprocessedNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "inboundUnprocessedNonce",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("outboundNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("outboundNonce"),
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
                    ::std::borrow::ToOwned::to_owned("processNextInboundMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "processNextInboundMessage",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("remoteMailboxes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("remoteMailboxes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract MockMailbox"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDefaultIsm"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDefaultIsm"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IInterchainSecurityModule",
                                        ),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKMAILBOX_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R`\0\x80T`\x01`\x01``\x1B\x03\x19\x16\x90U4\x80\x15a\0 W`\0\x80\xFD[P`@Qa\x10\x908\x03\x80a\x10\x90\x839\x81\x01`@\x81\x90Ra\0?\x91a\0MV[c\xFF\xFF\xFF\xFF\x16`\x80Ra\0zV[`\0` \x82\x84\x03\x12\x15a\0_W`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\0sW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x0F\xEDa\0\xA3`\09`\0\x81\x81a\x01\xD0\x01R\x81\x81a\x08\x87\x01Ra\t\xE1\x01Ra\x0F\xED`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x92\xD2\x8B=\x11a\0\x8CW\x80c\xF7\x94hz\x11a\0fW\x80c\xF7\x94hz\x14a\x02VW\x80c\xFA1\xDE\x01\x14a\x02\x91W\x80c\xFD\x10\xEB\xE5\x14a\x02\xA4W\x80c\xFF\xA1\xADt\x14a\x02\xB4W`\0\x80\xFD[\x80c\x92\xD2\x8B=\x14a\x01\xF2W\x80c\xA3\xB4\x91\x9F\x14a\x02\x16W\x80c\xD1!d\xE4\x14a\x02?W`\0\x80\xFD[\x80cky\x1C\xA1\x11a\0\xC8W\x80cky\x1C\xA1\x14a\x01YW\x80cn_Qn\x14a\x01lW\x80c\x82\t\xD3\x12\x14a\x01\x9EW\x80c\x8D68\xF4\x14a\x01\xCBW`\0\x80\xFD[\x80c\x16\x05\xC3\x06\x14a\0\xEFW\x80cR*\xE0\x02\x14a\0\xF9W\x80cY\xB3\xF6\xE0\x14a\x01\x15W[`\0\x80\xFD[a\0\xF7a\x02\xCEV[\0[a\x01\x02a\x08\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF7a\x01#6`\x04a\nvV[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\x01` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\0\xF7a\x01g6`\x04a\n\xF6V[a\x05rV[`\0Ta\x01\x86\x90`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x0CV[`\0Ta\x01\xB6\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x0CV[a\x01\xB6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x05a\x02\x006`\x04a\x0B\x80V[a\x06\xDAV[`@Qa\x01\x0C\x95\x94\x93\x92\x91\x90a\x0B\xE9V[a\x01\x86a\x02$6`\x04a\x0C4V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Ta\x01\xB6\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\0\xF7a\x02d6`\x04a\x0CVV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01``\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x01\x02a\x02\x9F6`\x04a\x0CsV[a\x07\xB1V[`\0Ta\x01\xB6\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xBC`\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\x0CV[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x81\x16\x83R`\x02` \x81\x81R`@\x80\x86 \x81Q`\xA0\x81\x01\x83R\x81T\x80\x87\x16\x82Rd\x01\0\0\0\0\x81\x04\x90\x96\x16\x93\x81\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x95\x90\x94\x04\x85\x16\x90\x82\x01R`\x01\x83\x01T\x90\x93\x16``\x84\x01R\x81\x01\x80T`\x80\x84\x01\x91\x90a\x03C\x90a\x0C\xCDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03o\x90a\x0C\xCDV[\x80\x15a\x03\xBCW\x80`\x1F\x10a\x03\x91Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xBCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x9FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP``\x81\x01Q\x90\x91P`\0a\x03\xD9\x82a\t-V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x04\xA6W\x80`\x01`\x01`\xA0\x1B\x03\x16c\xF7\xE8:\xEEa\x04\x02\x85a\t\xBDV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x1E\x91\x90a\r\x07V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04a\x91\x90a\r(V[a\x04\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x12T\xD3H\x1D\x99\\\x9AY\x9EH\x19\x98Z[\x19Y`z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x82``\x01Q`\x01`\x01`\xA0\x1B\x03\x16cV\xD5\xD4u\x84` \x01Qa\x04\xDB\x86`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x86`\x80\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xFE\x93\x92\x91\x90a\rJV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x18W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05,W=`\0\x80>=`\0\xFD[PP`\0\x80T`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x92P\x90P`\x08a\x05N\x83a\rxV[\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[`@Q\x80`\xA0\x01`@R\x80\x87c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP\x80Td\x01\0\0\0\0\x90\x81\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x83R`\x02` \x81\x81R`@\x94\x85\x90 \x87Q\x81T\x92\x89\x01Q\x96\x89\x01Q\x90\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x95\x90\x93\x16\x90\x93\x02\x93\x90\x93\x17h\x01\0\0\0\0\0\0\0\0`\x01`\xE0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x02\x17\x81U``\x85\x01Q`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x93\x16\x17\x90\x91U`\x80\x84\x01Q\x90\x92P\x90\x82\x01\x90a\x06\x92\x90\x82a\x0E\x0EV[PP`\0\x80Td\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x91P`\x04a\x06\xB3\x83a\rxV[\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPPPV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x92\x82\x01\x80Tc\xFF\xFF\xFF\xFF\x80\x84\x16\x95d\x01\0\0\0\0\x85\x04\x90\x91\x16\x94`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x91\x16\x92\x90\x91a\x07.\x90a\x0C\xCDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07Z\x90a\x0C\xCDV[\x80\x15a\x07\xA7W\x80`\x1F\x10a\x07|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xA7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x85V[`\0a\x08\0\x82\x11\x15a\x07\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rkmsg too long`\xA0\x1B`D\x82\x01R`d\x01a\x04\x9DV[c\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x08YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\t\xAD.nm-\xCC\xE4\x0EL\xAD\xAD\xEE\x8C\xA4\r\xAC--\x8CM\xEF`S\x1B`D\x82\x01R`d\x01a\x04\x9DV[`\0T`@Qcky\x1C\xA1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91cky\x1C\xA1\x91a\x08\xB7\x91c\xFF\xFF\xFF\xFF\x16\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x903\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a\x0E\xCEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xE5W=`\0\x80>=`\0\xFD[PP`\0\x80Tc\xFF\xFF\xFF\xFF\x16\x92P\x90P\x80a\x08\xFF\x83a\rxV[\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP`\0\x80\x1B\x91PP\x94\x93PPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xDER<\xF3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\t\x89WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\t\x86\x91\x81\x01\x90a\x0F0V[`\x01[\x15a\t\xA5W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\t\xA3W\x92\x91PPV[P[PP`\0T`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90V[```\0\x82`\0\x01Q\x83` \x01Qa\t\xDF\x85`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x14\x87``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x87`\x80\x01Q`@Q` \x01a\n/\x97\x96\x95\x94\x93\x92\x91\x90a\x0FMV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\nYW`\0\x80\xFD[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nsW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\n\x89W`\0\x80\xFD[a\n\x92\x83a\nEV[\x91P` \x83\x015a\n\xA2\x81a\n^V[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\n\xBFW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xD7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\n\xEFW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x0B\x0FW`\0\x80\xFD[a\x0B\x18\x87a\nEV[\x95Pa\x0B&` \x88\x01a\nEV[\x94P`@\x87\x015a\x0B6\x81a\n^V[\x93P``\x87\x015a\x0BF\x81a\n^V[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BbW`\0\x80\xFD[a\x0Bn\x89\x82\x8A\x01a\n\xADV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0B\x92W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x0B\xB4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\x9CV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0B\xD5\x81` \x86\x01` \x86\x01a\x0B\x99V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[c\xFF\xFF\xFF\xFF\x86\x81\x16\x82R\x85\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`@\x83\x01R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\x0C)\x90\x83\x01\x84a\x0B\xBDV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0CFW`\0\x80\xFD[a\x0CO\x82a\nEV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0ChW`\0\x80\xFD[\x815a\x0CO\x81a\n^V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x0C\x89W`\0\x80\xFD[a\x0C\x92\x85a\nEV[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xB5W`\0\x80\xFD[a\x0C\xC1\x87\x82\x88\x01a\n\xADV[\x95\x98\x94\x97P\x95PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\xE1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\r\x01WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0`@\x82\x01R``` \x82\x01R`\0a\x0CO``\x83\x01\x84a\x0B\xBDV[`\0` \x82\x84\x03\x12\x15a\r:W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0COW`\0\x80\xFD[c\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\ro``\x83\x01\x84a\x0B\xBDV[\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a\r\x9FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x0E\tW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\r\xE6WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0E\x05W\x82\x81U`\x01\x01a\r\xF2V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E(Wa\x0E(a\r\xA9V[a\x0E<\x81a\x0E6\x84Ta\x0C\xCDV[\x84a\r\xBFV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0EqW`\0\x84\x15a\x0EYWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0E\x05V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0E\xA0W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0E\x81V[P\x85\x82\x10\x15a\x0E\xBEW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[c\xFF\xFF\xFF\xFF\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`@\x83\x01R\x84\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R\x81\x01\x82\x90R`\0\x82\x84`\xC0\x84\x017`\0`\xC0\x84\x84\x01\x01R`\xC0`\x1F\x19`\x1F\x85\x01\x16\x83\x01\x01\x90P\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0FBW`\0\x80\xFD[\x81Qa\x0CO\x81a\n^V[`\xFF`\xF8\x1B\x88`\xF8\x1B\x16\x81R`\0c\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x89`\xE0\x1B\x16`\x01\x84\x01R\x80\x88`\xE0\x1B\x16`\x05\x84\x01R\x86`\t\x84\x01R\x80\x86`\xE0\x1B\x16`)\x84\x01RP\x83`-\x83\x01R\x82Qa\x0F\xA4\x81`M\x85\x01` \x87\x01a\x0B\x99V[\x91\x90\x91\x01`M\x01\x98\x97PPPPPPPPV\xFE\xA2dipfsX\"\x12 \xE7\x820\x15\x8E\x89#\xE0q\x83\xBD\xE0\xEE4\x08O\x8F\xE3d?N\x83\x0Ffa#\xC6\x9C\x145\xFD\x84dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOCKMAILBOX_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x92\xD2\x8B=\x11a\0\x8CW\x80c\xF7\x94hz\x11a\0fW\x80c\xF7\x94hz\x14a\x02VW\x80c\xFA1\xDE\x01\x14a\x02\x91W\x80c\xFD\x10\xEB\xE5\x14a\x02\xA4W\x80c\xFF\xA1\xADt\x14a\x02\xB4W`\0\x80\xFD[\x80c\x92\xD2\x8B=\x14a\x01\xF2W\x80c\xA3\xB4\x91\x9F\x14a\x02\x16W\x80c\xD1!d\xE4\x14a\x02?W`\0\x80\xFD[\x80cky\x1C\xA1\x11a\0\xC8W\x80cky\x1C\xA1\x14a\x01YW\x80cn_Qn\x14a\x01lW\x80c\x82\t\xD3\x12\x14a\x01\x9EW\x80c\x8D68\xF4\x14a\x01\xCBW`\0\x80\xFD[\x80c\x16\x05\xC3\x06\x14a\0\xEFW\x80cR*\xE0\x02\x14a\0\xF9W\x80cY\xB3\xF6\xE0\x14a\x01\x15W[`\0\x80\xFD[a\0\xF7a\x02\xCEV[\0[a\x01\x02a\x08\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF7a\x01#6`\x04a\nvV[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\x01` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\0\xF7a\x01g6`\x04a\n\xF6V[a\x05rV[`\0Ta\x01\x86\x90`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x0CV[`\0Ta\x01\xB6\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x0CV[a\x01\xB6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x05a\x02\x006`\x04a\x0B\x80V[a\x06\xDAV[`@Qa\x01\x0C\x95\x94\x93\x92\x91\x90a\x0B\xE9V[a\x01\x86a\x02$6`\x04a\x0C4V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Ta\x01\xB6\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\0\xF7a\x02d6`\x04a\x0CVV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01``\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x01\x02a\x02\x9F6`\x04a\x0CsV[a\x07\xB1V[`\0Ta\x01\xB6\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xBC`\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\x0CV[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x81\x16\x83R`\x02` \x81\x81R`@\x80\x86 \x81Q`\xA0\x81\x01\x83R\x81T\x80\x87\x16\x82Rd\x01\0\0\0\0\x81\x04\x90\x96\x16\x93\x81\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x95\x90\x94\x04\x85\x16\x90\x82\x01R`\x01\x83\x01T\x90\x93\x16``\x84\x01R\x81\x01\x80T`\x80\x84\x01\x91\x90a\x03C\x90a\x0C\xCDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03o\x90a\x0C\xCDV[\x80\x15a\x03\xBCW\x80`\x1F\x10a\x03\x91Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xBCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x9FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP``\x81\x01Q\x90\x91P`\0a\x03\xD9\x82a\t-V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x04\xA6W\x80`\x01`\x01`\xA0\x1B\x03\x16c\xF7\xE8:\xEEa\x04\x02\x85a\t\xBDV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x1E\x91\x90a\r\x07V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04a\x91\x90a\r(V[a\x04\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x12T\xD3H\x1D\x99\\\x9AY\x9EH\x19\x98Z[\x19Y`z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x82``\x01Q`\x01`\x01`\xA0\x1B\x03\x16cV\xD5\xD4u\x84` \x01Qa\x04\xDB\x86`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x86`\x80\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xFE\x93\x92\x91\x90a\rJV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x18W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05,W=`\0\x80>=`\0\xFD[PP`\0\x80T`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x92P\x90P`\x08a\x05N\x83a\rxV[\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[`@Q\x80`\xA0\x01`@R\x80\x87c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP\x80Td\x01\0\0\0\0\x90\x81\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x83R`\x02` \x81\x81R`@\x94\x85\x90 \x87Q\x81T\x92\x89\x01Q\x96\x89\x01Q\x90\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x95\x90\x93\x16\x90\x93\x02\x93\x90\x93\x17h\x01\0\0\0\0\0\0\0\0`\x01`\xE0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x02\x17\x81U``\x85\x01Q`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x93\x16\x17\x90\x91U`\x80\x84\x01Q\x90\x92P\x90\x82\x01\x90a\x06\x92\x90\x82a\x0E\x0EV[PP`\0\x80Td\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x91P`\x04a\x06\xB3\x83a\rxV[\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPPPV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x92\x82\x01\x80Tc\xFF\xFF\xFF\xFF\x80\x84\x16\x95d\x01\0\0\0\0\x85\x04\x90\x91\x16\x94`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x91\x16\x92\x90\x91a\x07.\x90a\x0C\xCDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07Z\x90a\x0C\xCDV[\x80\x15a\x07\xA7W\x80`\x1F\x10a\x07|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xA7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x85V[`\0a\x08\0\x82\x11\x15a\x07\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rkmsg too long`\xA0\x1B`D\x82\x01R`d\x01a\x04\x9DV[c\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x08YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\t\xAD.nm-\xCC\xE4\x0EL\xAD\xAD\xEE\x8C\xA4\r\xAC--\x8CM\xEF`S\x1B`D\x82\x01R`d\x01a\x04\x9DV[`\0T`@Qcky\x1C\xA1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91cky\x1C\xA1\x91a\x08\xB7\x91c\xFF\xFF\xFF\xFF\x16\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x903\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a\x0E\xCEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xE5W=`\0\x80>=`\0\xFD[PP`\0\x80Tc\xFF\xFF\xFF\xFF\x16\x92P\x90P\x80a\x08\xFF\x83a\rxV[\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP`\0\x80\x1B\x91PP\x94\x93PPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xDER<\xF3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\t\x89WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\t\x86\x91\x81\x01\x90a\x0F0V[`\x01[\x15a\t\xA5W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\t\xA3W\x92\x91PPV[P[PP`\0T`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90V[```\0\x82`\0\x01Q\x83` \x01Qa\t\xDF\x85`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x14\x87``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x87`\x80\x01Q`@Q` \x01a\n/\x97\x96\x95\x94\x93\x92\x91\x90a\x0FMV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\nYW`\0\x80\xFD[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nsW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\n\x89W`\0\x80\xFD[a\n\x92\x83a\nEV[\x91P` \x83\x015a\n\xA2\x81a\n^V[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\n\xBFW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xD7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\n\xEFW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x0B\x0FW`\0\x80\xFD[a\x0B\x18\x87a\nEV[\x95Pa\x0B&` \x88\x01a\nEV[\x94P`@\x87\x015a\x0B6\x81a\n^V[\x93P``\x87\x015a\x0BF\x81a\n^V[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BbW`\0\x80\xFD[a\x0Bn\x89\x82\x8A\x01a\n\xADV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0B\x92W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x0B\xB4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\x9CV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0B\xD5\x81` \x86\x01` \x86\x01a\x0B\x99V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[c\xFF\xFF\xFF\xFF\x86\x81\x16\x82R\x85\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`@\x83\x01R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\x0C)\x90\x83\x01\x84a\x0B\xBDV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0CFW`\0\x80\xFD[a\x0CO\x82a\nEV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0ChW`\0\x80\xFD[\x815a\x0CO\x81a\n^V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x0C\x89W`\0\x80\xFD[a\x0C\x92\x85a\nEV[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xB5W`\0\x80\xFD[a\x0C\xC1\x87\x82\x88\x01a\n\xADV[\x95\x98\x94\x97P\x95PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\xE1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\r\x01WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0`@\x82\x01R``` \x82\x01R`\0a\x0CO``\x83\x01\x84a\x0B\xBDV[`\0` \x82\x84\x03\x12\x15a\r:W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0COW`\0\x80\xFD[c\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\ro``\x83\x01\x84a\x0B\xBDV[\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a\r\x9FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x0E\tW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\r\xE6WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0E\x05W\x82\x81U`\x01\x01a\r\xF2V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E(Wa\x0E(a\r\xA9V[a\x0E<\x81a\x0E6\x84Ta\x0C\xCDV[\x84a\r\xBFV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0EqW`\0\x84\x15a\x0EYWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0E\x05V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0E\xA0W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0E\x81V[P\x85\x82\x10\x15a\x0E\xBEW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[c\xFF\xFF\xFF\xFF\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`@\x83\x01R\x84\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R\x81\x01\x82\x90R`\0\x82\x84`\xC0\x84\x017`\0`\xC0\x84\x84\x01\x01R`\xC0`\x1F\x19`\x1F\x85\x01\x16\x83\x01\x01\x90P\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0FBW`\0\x80\xFD[\x81Qa\x0CO\x81a\n^V[`\xFF`\xF8\x1B\x88`\xF8\x1B\x16\x81R`\0c\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x89`\xE0\x1B\x16`\x01\x84\x01R\x80\x88`\xE0\x1B\x16`\x05\x84\x01R\x86`\t\x84\x01R\x80\x86`\xE0\x1B\x16`)\x84\x01RP\x83`-\x83\x01R\x82Qa\x0F\xA4\x81`M\x85\x01` \x87\x01a\x0B\x99V[\x91\x90\x91\x01`M\x01\x98\x97PPPPPPPPV\xFE\xA2dipfsX\"\x12 \xE7\x820\x15\x8E\x89#\xE0q\x83\xBD\xE0\xEE4\x08O\x8F\xE3d?N\x83\x0Ffa#\xC6\x9C\x145\xFD\x84dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKMAILBOX_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockMailbox<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockMailbox<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockMailbox<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockMailbox<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockMailbox<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockMailbox))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockMailbox<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKMAILBOX_ABI.clone(),
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
                MOCKMAILBOX_ABI.clone(),
                MOCKMAILBOX_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_MESSAGE_BODY_BYTES` (0x522ae002) function
        pub fn max_message_body_bytes(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([82, 42, 224, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn version(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addInboundMessage` (0x6b791ca1) function
        pub fn add_inbound_message(
            &self,
            nonce: u32,
            origin: u32,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            body: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [107, 121, 28, 161],
                    (nonce, origin, sender, recipient, body),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addRemoteMailbox` (0x59b3f6e0) function
        pub fn add_remote_mailbox(
            &self,
            domain: u32,
            mailbox: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 179, 246, 224], (domain, mailbox))
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
        ///Calls the contract's `inboundMessages` (0x92d28b3d) function
        pub fn inbound_messages(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u32,
                u32,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([146, 210, 139, 61], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inboundProcessedNonce` (0xd12164e4) function
        pub fn inbound_processed_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([209, 33, 100, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inboundUnprocessedNonce` (0x8209d312) function
        pub fn inbound_unprocessed_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([130, 9, 211, 18], ())
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
        ///Calls the contract's `outboundNonce` (0xfd10ebe5) function
        pub fn outbound_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([253, 16, 235, 229], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processNextInboundMessage` (0x1605c306) function
        pub fn process_next_inbound_message(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 5, 195, 6], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `remoteMailboxes` (0xa3b4919f) function
        pub fn remote_mailboxes(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([163, 180, 145, 159], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDefaultIsm` (0xf794687a) function
        pub fn set_default_ism(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 148, 104, 122], module)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockMailbox<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `MAX_MESSAGE_BODY_BYTES` function with signature `MAX_MESSAGE_BODY_BYTES()` and selector `0x522ae002`
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
    #[ethcall(name = "MAX_MESSAGE_BODY_BYTES", abi = "MAX_MESSAGE_BODY_BYTES()")]
    pub struct MaxMessageBodyBytesCall;
    ///Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `addInboundMessage` function with signature `addInboundMessage(uint32,uint32,address,address,bytes)` and selector `0x6b791ca1`
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
    #[ethcall(
        name = "addInboundMessage",
        abi = "addInboundMessage(uint32,uint32,address,address,bytes)"
    )]
    pub struct AddInboundMessageCall {
        pub nonce: u32,
        pub origin: u32,
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub body: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `addRemoteMailbox` function with signature `addRemoteMailbox(uint32,address)` and selector `0x59b3f6e0`
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
    #[ethcall(name = "addRemoteMailbox", abi = "addRemoteMailbox(uint32,address)")]
    pub struct AddRemoteMailboxCall {
        pub domain: u32,
        pub mailbox: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `inboundMessages` function with signature `inboundMessages(uint256)` and selector `0x92d28b3d`
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
    #[ethcall(name = "inboundMessages", abi = "inboundMessages(uint256)")]
    pub struct InboundMessagesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `inboundProcessedNonce` function with signature `inboundProcessedNonce()` and selector `0xd12164e4`
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
    #[ethcall(name = "inboundProcessedNonce", abi = "inboundProcessedNonce()")]
    pub struct InboundProcessedNonceCall;
    ///Container type for all input parameters for the `inboundUnprocessedNonce` function with signature `inboundUnprocessedNonce()` and selector `0x8209d312`
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
    #[ethcall(name = "inboundUnprocessedNonce", abi = "inboundUnprocessedNonce()")]
    pub struct InboundUnprocessedNonceCall;
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
    ///Container type for all input parameters for the `outboundNonce` function with signature `outboundNonce()` and selector `0xfd10ebe5`
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
    #[ethcall(name = "outboundNonce", abi = "outboundNonce()")]
    pub struct OutboundNonceCall;
    ///Container type for all input parameters for the `processNextInboundMessage` function with signature `processNextInboundMessage()` and selector `0x1605c306`
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
    #[ethcall(name = "processNextInboundMessage", abi = "processNextInboundMessage()")]
    pub struct ProcessNextInboundMessageCall;
    ///Container type for all input parameters for the `remoteMailboxes` function with signature `remoteMailboxes(uint32)` and selector `0xa3b4919f`
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
    #[ethcall(name = "remoteMailboxes", abi = "remoteMailboxes(uint32)")]
    pub struct RemoteMailboxesCall(pub u32);
    ///Container type for all input parameters for the `setDefaultIsm` function with signature `setDefaultIsm(address)` and selector `0xf794687a`
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
    #[ethcall(name = "setDefaultIsm", abi = "setDefaultIsm(address)")]
    pub struct SetDefaultIsmCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockMailboxCalls {
        MaxMessageBodyBytes(MaxMessageBodyBytesCall),
        Version(VersionCall),
        AddInboundMessage(AddInboundMessageCall),
        AddRemoteMailbox(AddRemoteMailboxCall),
        DefaultIsm(DefaultIsmCall),
        Dispatch(DispatchCall),
        InboundMessages(InboundMessagesCall),
        InboundProcessedNonce(InboundProcessedNonceCall),
        InboundUnprocessedNonce(InboundUnprocessedNonceCall),
        LocalDomain(LocalDomainCall),
        OutboundNonce(OutboundNonceCall),
        ProcessNextInboundMessage(ProcessNextInboundMessageCall),
        RemoteMailboxes(RemoteMailboxesCall),
        SetDefaultIsm(SetDefaultIsmCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockMailboxCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <MaxMessageBodyBytesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxMessageBodyBytes(decoded));
            }
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded)
                = <AddInboundMessageCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddInboundMessage(decoded));
            }
            if let Ok(decoded)
                = <AddRemoteMailboxCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddRemoteMailbox(decoded));
            }
            if let Ok(decoded)
                = <DefaultIsmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultIsm(decoded));
            }
            if let Ok(decoded)
                = <DispatchCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Dispatch(decoded));
            }
            if let Ok(decoded)
                = <InboundMessagesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InboundMessages(decoded));
            }
            if let Ok(decoded)
                = <InboundProcessedNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InboundProcessedNonce(decoded));
            }
            if let Ok(decoded)
                = <InboundUnprocessedNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InboundUnprocessedNonce(decoded));
            }
            if let Ok(decoded)
                = <LocalDomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LocalDomain(decoded));
            }
            if let Ok(decoded)
                = <OutboundNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutboundNonce(decoded));
            }
            if let Ok(decoded)
                = <ProcessNextInboundMessageCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProcessNextInboundMessage(decoded));
            }
            if let Ok(decoded)
                = <RemoteMailboxesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoteMailboxes(decoded));
            }
            if let Ok(decoded)
                = <SetDefaultIsmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDefaultIsm(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockMailboxCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxMessageBodyBytes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddInboundMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddRemoteMailbox(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultIsm(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Dispatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InboundMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InboundProcessedNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InboundUnprocessedNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LocalDomain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutboundNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessNextInboundMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoteMailboxes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDefaultIsm(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockMailboxCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxMessageBodyBytes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddInboundMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddRemoteMailbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultIsm(element) => ::core::fmt::Display::fmt(element, f),
                Self::Dispatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::InboundMessages(element) => ::core::fmt::Display::fmt(element, f),
                Self::InboundProcessedNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InboundUnprocessedNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LocalDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutboundNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessNextInboundMessage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoteMailboxes(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDefaultIsm(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxMessageBodyBytesCall> for MockMailboxCalls {
        fn from(value: MaxMessageBodyBytesCall) -> Self {
            Self::MaxMessageBodyBytes(value)
        }
    }
    impl ::core::convert::From<VersionCall> for MockMailboxCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<AddInboundMessageCall> for MockMailboxCalls {
        fn from(value: AddInboundMessageCall) -> Self {
            Self::AddInboundMessage(value)
        }
    }
    impl ::core::convert::From<AddRemoteMailboxCall> for MockMailboxCalls {
        fn from(value: AddRemoteMailboxCall) -> Self {
            Self::AddRemoteMailbox(value)
        }
    }
    impl ::core::convert::From<DefaultIsmCall> for MockMailboxCalls {
        fn from(value: DefaultIsmCall) -> Self {
            Self::DefaultIsm(value)
        }
    }
    impl ::core::convert::From<DispatchCall> for MockMailboxCalls {
        fn from(value: DispatchCall) -> Self {
            Self::Dispatch(value)
        }
    }
    impl ::core::convert::From<InboundMessagesCall> for MockMailboxCalls {
        fn from(value: InboundMessagesCall) -> Self {
            Self::InboundMessages(value)
        }
    }
    impl ::core::convert::From<InboundProcessedNonceCall> for MockMailboxCalls {
        fn from(value: InboundProcessedNonceCall) -> Self {
            Self::InboundProcessedNonce(value)
        }
    }
    impl ::core::convert::From<InboundUnprocessedNonceCall> for MockMailboxCalls {
        fn from(value: InboundUnprocessedNonceCall) -> Self {
            Self::InboundUnprocessedNonce(value)
        }
    }
    impl ::core::convert::From<LocalDomainCall> for MockMailboxCalls {
        fn from(value: LocalDomainCall) -> Self {
            Self::LocalDomain(value)
        }
    }
    impl ::core::convert::From<OutboundNonceCall> for MockMailboxCalls {
        fn from(value: OutboundNonceCall) -> Self {
            Self::OutboundNonce(value)
        }
    }
    impl ::core::convert::From<ProcessNextInboundMessageCall> for MockMailboxCalls {
        fn from(value: ProcessNextInboundMessageCall) -> Self {
            Self::ProcessNextInboundMessage(value)
        }
    }
    impl ::core::convert::From<RemoteMailboxesCall> for MockMailboxCalls {
        fn from(value: RemoteMailboxesCall) -> Self {
            Self::RemoteMailboxes(value)
        }
    }
    impl ::core::convert::From<SetDefaultIsmCall> for MockMailboxCalls {
        fn from(value: SetDefaultIsmCall) -> Self {
            Self::SetDefaultIsm(value)
        }
    }
    ///Container type for all return fields from the `MAX_MESSAGE_BODY_BYTES` function with signature `MAX_MESSAGE_BODY_BYTES()` and selector `0x522ae002`
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
    pub struct MaxMessageBodyBytesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    pub struct VersionReturn(pub u8);
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
    ///Container type for all return fields from the `inboundMessages` function with signature `inboundMessages(uint256)` and selector `0x92d28b3d`
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
    pub struct InboundMessagesReturn {
        pub nonce: u32,
        pub origin: u32,
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub body: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `inboundProcessedNonce` function with signature `inboundProcessedNonce()` and selector `0xd12164e4`
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
    pub struct InboundProcessedNonceReturn(pub u32);
    ///Container type for all return fields from the `inboundUnprocessedNonce` function with signature `inboundUnprocessedNonce()` and selector `0x8209d312`
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
    pub struct InboundUnprocessedNonceReturn(pub u32);
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
    ///Container type for all return fields from the `outboundNonce` function with signature `outboundNonce()` and selector `0xfd10ebe5`
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
    pub struct OutboundNonceReturn(pub u32);
    ///Container type for all return fields from the `remoteMailboxes` function with signature `remoteMailboxes(uint32)` and selector `0xa3b4919f`
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
    pub struct RemoteMailboxesReturn(pub ::ethers::core::types::Address);
}
