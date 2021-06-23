/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import {
  Signer,
  utils,
  BigNumberish,
  Contract,
  ContractFactory,
  Overrides,
} from "ethers";
import { Provider, TransactionRequest } from "@ethersproject/providers";
import type { TestReplica, TestReplicaInterface } from "../TestReplica";

const _abi = [
  {
    inputs: [
      {
        internalType: "uint32",
        name: "_localDomain",
        type: "uint32",
      },
    ],
    stateMutability: "nonpayable",
    type: "constructor",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: "bytes32",
        name: "oldRoot",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes32[2]",
        name: "newRoot",
        type: "bytes32[2]",
      },
      {
        indexed: false,
        internalType: "bytes",
        name: "signature",
        type: "bytes",
      },
      {
        indexed: false,
        internalType: "bytes",
        name: "signature2",
        type: "bytes",
      },
    ],
    name: "DoubleUpdate",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: "bytes",
        name: "error",
        type: "bytes",
      },
    ],
    name: "ProcessError",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "uint32",
        name: "homeDomain",
        type: "uint32",
      },
      {
        indexed: true,
        internalType: "bytes32",
        name: "oldRoot",
        type: "bytes32",
      },
      {
        indexed: true,
        internalType: "bytes32",
        name: "newRoot",
        type: "bytes32",
      },
      {
        indexed: false,
        internalType: "bytes",
        name: "signature",
        type: "bytes",
      },
    ],
    name: "Update",
    type: "event",
  },
  {
    inputs: [],
    name: "PROCESS_GAS",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "RESERVE_GAS",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "canConfirm",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "confirm",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "",
        type: "bytes32",
      },
    ],
    name: "confirmAt",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "current",
    outputs: [
      {
        internalType: "bytes32",
        name: "",
        type: "bytes32",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "_oldRoot",
        type: "bytes32",
      },
      {
        internalType: "bytes32[2]",
        name: "_newRoot",
        type: "bytes32[2]",
      },
      {
        internalType: "bytes",
        name: "_signature",
        type: "bytes",
      },
      {
        internalType: "bytes",
        name: "_signature2",
        type: "bytes",
      },
    ],
    name: "doubleUpdate",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "homeDomainHash",
    outputs: [
      {
        internalType: "bytes32",
        name: "",
        type: "bytes32",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "uint32",
        name: "_remoteDomain",
        type: "uint32",
      },
      {
        internalType: "address",
        name: "_updater",
        type: "address",
      },
      {
        internalType: "bytes32",
        name: "_current",
        type: "bytes32",
      },
      {
        internalType: "uint256",
        name: "_optimisticSeconds",
        type: "uint256",
      },
      {
        internalType: "uint32",
        name: "_nextToProcess",
        type: "uint32",
      },
    ],
    name: "initialize",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "localDomain",
    outputs: [
      {
        internalType: "uint32",
        name: "",
        type: "uint32",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "",
        type: "bytes32",
      },
    ],
    name: "messages",
    outputs: [
      {
        internalType: "enum Replica.MessageStatus",
        name: "",
        type: "uint8",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "nextPending",
    outputs: [
      {
        internalType: "bytes32",
        name: "_pending",
        type: "bytes32",
      },
      {
        internalType: "uint256",
        name: "_confirmAt",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "nextToProcess",
    outputs: [
      {
        internalType: "uint32",
        name: "",
        type: "uint32",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "optimisticSeconds",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes",
        name: "_message",
        type: "bytes",
      },
    ],
    name: "process",
    outputs: [
      {
        internalType: "bool",
        name: "_success",
        type: "bool",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "_leaf",
        type: "bytes32",
      },
      {
        internalType: "bytes32[32]",
        name: "_proof",
        type: "bytes32[32]",
      },
      {
        internalType: "uint256",
        name: "_index",
        type: "uint256",
      },
    ],
    name: "prove",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes",
        name: "_message",
        type: "bytes",
      },
      {
        internalType: "bytes32[32]",
        name: "_proof",
        type: "bytes32[32]",
      },
      {
        internalType: "uint256",
        name: "_index",
        type: "uint256",
      },
    ],
    name: "proveAndProcess",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "_item",
        type: "bytes32",
      },
    ],
    name: "queueContains",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "queueEnd",
    outputs: [
      {
        internalType: "bytes32",
        name: "",
        type: "bytes32",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "queueLength",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "remoteDomain",
    outputs: [
      {
        internalType: "uint32",
        name: "",
        type: "uint32",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "_newRoot",
        type: "bytes32",
      },
    ],
    name: "setCurrentRoot",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "setFailed",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes",
        name: "_message",
        type: "bytes",
      },
    ],
    name: "setMessagePending",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "uint32",
        name: "_remoteDomain",
        type: "uint32",
      },
    ],
    name: "setRemoteDomain",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "_updater",
        type: "address",
      },
    ],
    name: "setUpdater",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "state",
    outputs: [
      {
        internalType: "enum Common.States",
        name: "",
        type: "uint8",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "leaf",
        type: "bytes32",
      },
      {
        internalType: "bytes32[32]",
        name: "proof",
        type: "bytes32[32]",
      },
      {
        internalType: "uint256",
        name: "index",
        type: "uint256",
      },
    ],
    name: "testBranchRoot",
    outputs: [
      {
        internalType: "bytes32",
        name: "",
        type: "bytes32",
      },
    ],
    stateMutability: "pure",
    type: "function",
  },
  {
    inputs: [],
    name: "testHomeDomainHash",
    outputs: [
      {
        internalType: "bytes32",
        name: "",
        type: "bytes32",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes",
        name: "_message",
        type: "bytes",
      },
    ],
    name: "testProcess",
    outputs: [
      {
        internalType: "bool",
        name: "_success",
        type: "bool",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "timestamp",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "_oldRoot",
        type: "bytes32",
      },
      {
        internalType: "bytes32",
        name: "_newRoot",
        type: "bytes32",
      },
      {
        internalType: "bytes",
        name: "_signature",
        type: "bytes",
      },
    ],
    name: "update",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "updater",
    outputs: [
      {
        internalType: "address",
        name: "",
        type: "address",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
];

const _bytecode =
  "0x60a060405234801561001057600080fd5b506040516133e63803806133e68339818101604052602081101561003357600080fd5b505160e081901b6001600160e01b03191660805263ffffffff1661337d61006960003980610fc3528061102a525061337d6000f3fe608060405234801561001057600080fd5b506004361061020b5760003560e01c80638d3638f41161012a578063b31c01fb116100bd578063bf30a55d1161008c578063d88beda211610071578063d88beda214610895578063df034cd01461089d578063f6d16102146108ce5761020b565b8063bf30a55d14610838578063c19d93fb1461088d5761020b565b8063b31c01fb146106b7578063b61c19e814610769578063b80777ea1461080f578063ba739a62146108175761020b565b80639868a273116100f95780639868a2731461064a5780639d54f419146106745780639fa6a6e3146106a7578063ab91c7b0146106af5761020b565b80638d3638f4146104ee578063928bc4b2146104f6578063961681dc1461059c57806396ae1a89146105a45761020b565b806339992668116101a257806351d7bcd71161017157806351d7bcd7146103f85780636188af0e1461041b5780637022b58e146104c957806371bfb7b8146104d15761020b565b806339992668146103bf57806345630b1a146103c7578063456d0672146103cf5780635146366e146103f05761020b565b806325e3beda116101de57806325e3beda146103205780632bbd59ca1461033a5780632bef289214610378578063371d3071146103955761020b565b8063016bcc3514610210578063146901db1461022f57806314cfabb31461023757806319d9d21a14610253575b600080fd5b61022d6004803603602081101561022657600080fd5b50356108d6565b005b61022d6108ed565b61023f6108f7565b604080519115158252519081900360200190f35b61022d600480360360a081101561026957600080fd5b813591602081019181019060808101606082013564010000000081111561028f57600080fd5b8201836020820111156102a157600080fd5b803590602001918460018302840111640100000000831117156102c357600080fd5b9193909290916020810190356401000000008111156102e157600080fd5b8201836020820111156102f357600080fd5b8035906020019184600183028401116401000000008311171561031557600080fd5b509092509050610923565b610328610b82565b60408051918252519081900360200190f35b6103576004803603602081101561035057600080fd5b5035610b88565b6040518082600281111561036757fe5b815260200191505060405180910390f35b61023f6004803603602081101561038e57600080fd5b5035610b9d565b61023f60048036036104408110156103ac57600080fd5b5080359060208101906104200135610bb2565b610328610cd2565b610328610cd8565b6103d7610ced565b6040805163ffffffff9092168252519081900360200190f35b610328610cf9565b61022d6004803603602081101561040e57600080fd5b503563ffffffff16610d03565b61022d600480360361044081101561043257600080fd5b81019060208101813564010000000081111561044d57600080fd5b82018360208201111561045f57600080fd5b8035906020019184600183028401116401000000008311171561048157600080fd5b91908080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525092955092935050506104008201359050610d3a565b61022d610dc6565b610328600480360360208110156104e757600080fd5b5035610faf565b6103d7610fc1565b61023f6004803603602081101561050c57600080fd5b81019060208101813564010000000081111561052757600080fd5b82018360208201111561053957600080fd5b8035906020019184600183028401116401000000008311171561055b57600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250929550610fe5945050505050565b6103d76115b5565b61023f600480360360208110156105ba57600080fd5b8101906020810181356401000000008111156105d557600080fd5b8201836020820111156105e757600080fd5b8035906020019184600183028401116401000000008311171561060957600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295506115c1945050505050565b610328600480360361044081101561066157600080fd5b50803590602081019061042001356115cc565b61022d6004803603602081101561068a57600080fd5b503573ffffffffffffffffffffffffffffffffffffffff1661160a565b610328611657565b61032861165d565b61022d600480360360608110156106cd57600080fd5b8135916020810135918101906060810160408201356401000000008111156106f457600080fd5b82018360208201111561070657600080fd5b8035906020019184600183028401116401000000008311171561072857600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250929550611669945050505050565b61022d6004803603602081101561077f57600080fd5b81019060208101813564010000000081111561079a57600080fd5b8201836020820111156107ac57600080fd5b803590602001918460018302840111640100000000831117156107ce57600080fd5b91908080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525092955061194f945050505050565b6103286119d9565b61081f6119dd565b6040805192835260208301919091528051918290030190f35b61022d600480360360a081101561084e57600080fd5b5063ffffffff813581169173ffffffffffffffffffffffffffffffffffffffff6020820135169160408201359160608101359160809091013516611a2b565b610357611bcf565b610328611bf2565b6108a5611bf9565b6040805173ffffffffffffffffffffffffffffffffffffffff9092168252519081900360200190f35b610328611c1b565b600181815560009182526007602052604090912055565b6108f5611c27565b565b60006109036002611c6a565b1580159061091e575061091e6109196002611ca2565b611d43565b905090565b6002600054760100000000000000000000000000000000000000000000900460ff16600281111561095057fe5b14156109bd57604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f6661696c65642073746174650000000000000000000000000000000000000000604482015290519081900360640190fd5b604080516020601f86018190048102820181019092528481526109ff9188918835918890889081908401838280828437600092019190915250611d6a92505050565b8015610a4e5750610a4e86866001602002013584848080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250611d6a92505050565b8015610a5f57508435602086013514155b15610b7a57610a6c6108ed565b7f2c3f60bab4170347826231b75a920b5053941ddebc6eed6fd2c25721648b186f8686868686866040518087815260200186600260200280828437600083820152601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01690910182810360409081018252810186905290506020810160608201878780828437600083820152601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01690910184810383528581526020019050858580828437600083820152604051601f9091017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169092018290039a509098505050505050505050a15b505050505050565b61271081565b60086020526000908152604090205460ff1681565b6000610baa600283611e02565b90505b919050565b60008060008581526008602052604090205460ff166002811115610bd257fe5b14610c3e57604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f214d6573736167655374617475732e4e6f6e6500000000000000000000000000604482015290519081900360640190fd5b6000610c74858560208060200260405190810160405280929190826020800280828437600092019190915250879150611e7e9050565b9050610c7f81611d43565b15610cc5575050600083815260086020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001908117909155610ccb565b60009150505b9392505050565b60055481565b60045460009061091e9063ffffffff16611f29565b60065463ffffffff1681565b600061091e610cd8565b600480547fffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000001663ffffffff92909216919091179055565b610d4c83805190602001208383610bb2565b610db757604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600660248201527f2170726f76650000000000000000000000000000000000000000000000000000604482015290519081900360640190fd5b610dc083610fe5565b50505050565b6002600054760100000000000000000000000000000000000000000000900460ff166002811115610df357fe5b1415610e6057604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f6661696c65642073746174650000000000000000000000000000000000000000604482015290519081900360640190fd5b610e6a6002611c6a565b610ed557604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600860248201527f2170656e64696e67000000000000000000000000000000000000000000000000604482015290519081900360640190fd5b600080610ee26002611c6a565b90505b600081118015610efd5750610efd6109196002611ca2565b15610f3557610f0c6002611f9e565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01610ee5565b81610fa157604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600560248201527f2174696d65000000000000000000000000000000000000000000000000000000604482015290519081900360640190fd5b610fa96108f5565b50600155565b60076020526000908152604090205481565b7f000000000000000000000000000000000000000000000000000000000000000081565b600080610ff283826120d7565b905060006110217fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000083166120fb565b905063ffffffff7f0000000000000000000000000000000000000000000000000000000000000000166110757fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000841661212c565b63ffffffff16146110e757604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f2164657374696e6174696f6e0000000000000000000000000000000000000000604482015290519081900360640190fd5b60065463ffffffff82811691161461116057604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600960248201527f2173657175656e63650000000000000000000000000000000000000000000000604482015290519081900360640190fd5b6001845160208087019190912060009081526008909152604090205460ff16600281111561118a57fe5b146111f657604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600860248201527f2170656e64696e67000000000000000000000000000000000000000000000000604482015290519081900360640190fd5b6002600860006112277fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000861661215d565b8152602081019190915260400160002080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600183600281111561126957fe5b02179055506207c8305a10156112e257604080517f08c379a0000000000000000000000000000000000000000000000000000000008152602060048083019190915260248201527f2167617300000000000000000000000000000000000000000000000000000000604482015290519081900360640190fd5b600061130f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000841661219c565b905073ffffffffffffffffffffffffffffffffffffffff81166356d5d4756207a12061135c7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000087166121af565b6113877fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000088166121df565b6113dc6113b57fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000008a16612210565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000016612281565b6040518563ffffffff1660e01b8152600401808463ffffffff16815260200183815260200180602001828103825283818151815260200191508051906020019080838360005b8381101561143a578181015183820152602001611422565b50505050905090810190601f1680156114675780820380516001836020036101000a031916815260200191505b50945050505050600060405180830381600088803b15801561148857600080fd5b5087f19350505050801561149a575060015b611571573d8080156114c8576040519150601f19603f3d011682016040523d82523d6000602084013e6114cd565b606091505b50600094507f3c688a5f4cd6e38b537641d2b38bdf1f52e7da4d083c5c3b16a0847c1c7c642d816040518080602001828103825283818151815260200191508051906020019080838360005b83811015611531578181015183820152602001611519565b50505050905090810190601f16801561155e5780820380516001836020036101000a031916815260200191505b509250505060405180910390a150611576565b600193505b50600680547fffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000001660019290920163ffffffff1691909117905550919050565b60045463ffffffff1681565b6000610baa82610fe5565b6000611602848460208060200260405190810160405280929190826020800280828437600092019190915250869150611e7e9050565b949350505050565b6000805473ffffffffffffffffffffffffffffffffffffffff90921662010000027fffffffffffffffffffff0000000000000000000000000000000000000000ffff909216919091179055565b60015481565b600061091e6002611c6a565b6002600054760100000000000000000000000000000000000000000000900460ff16600281111561169657fe5b141561170357604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f6661696c65642073746174650000000000000000000000000000000000000000604482015290519081900360640190fd5b600061170f6002611c6a565b11156117915761171f60026122c5565b831461178c57604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601060248201527f6e6f7420656e64206f6620717565756500000000000000000000000000000000604482015290519081900360640190fd5b611801565b826001541461180157604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601260248201527f6e6f742063757272656e74207570646174650000000000000000000000000000604482015290519081900360640190fd5b61180c838383611d6a565b61187757604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600760248201527f6261642073696700000000000000000000000000000000000000000000000000604482015290519081900360640190fd5b61187f6108f5565b60055460008381526007602052604090204290910190556118a1600283612302565b5060045460408051602080825284518183015284518694889463ffffffff909116937f608828ad904a0c9250c09004ba7226efb08f35a5c815bb3f76b5a8a271cd08b2938893919283929083019185019080838360005b838110156119105781810151838201526020016118f8565b50505050905090810190601f16801561193d5780820380516001836020036101000a031916815260200191505b509250505060405180910390a4505050565b600061195b82826120d7565b905060016008600061198e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000851661215d565b8152602081019190915260400160002080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660018360028111156119d057fe5b02179055505050565b4290565b6000806119ea6002611c6a565b15611a12576119f96002611ca2565b6000818152600760205260409020549092509050611a27565b50506001546000818152600760205260409020545b9091565b600054610100900460ff1680611a445750611a4461236f565b80611a52575060005460ff16155b611aa7576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602e815260200180613236602e913960400191505060405180910390fd5b600054610100900460ff16158015611b0d57600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff909116610100171660011790555b600480547fffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000001663ffffffff8816179055611b476002612380565b60018481556000858152600760205260409020556005839055600680547fffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000001663ffffffff8416179055611b99856123c5565b8015610b7a57600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff169055505050505050565b600054760100000000000000000000000000000000000000000000900460ff1681565b6207a12081565b60005462010000900473ffffffffffffffffffffffffffffffffffffffff1681565b600061091e60026122c5565b600080547fffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffffffff16760200000000000000000000000000000000000000000000179055565b80546000906fffffffffffffffffffffffffffffffff700100000000000000000000000000000000820481169116611602828261255a565b6000611cad82612574565b15611d1957604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600560248201527f456d707479000000000000000000000000000000000000000000000000000000604482015290519081900360640190fd5b5080546fffffffffffffffffffffffffffffffff1660009081526001909101602052604090205490565b60008181526007602052604081205480611d61576000915050610bad565b42101592915050565b600080611d75610cd8565b8585604051602001808481526020018381526020018281526020019350505050604051602081830303815290604052805190602001209050611db6816125a3565b60005490915062010000900473ffffffffffffffffffffffffffffffffffffffff16611de282856125f4565b73ffffffffffffffffffffffffffffffffffffffff161495945050505050565b81546000906fffffffffffffffffffffffffffffffff165b835470010000000000000000000000000000000090046fffffffffffffffffffffffffffffffff168111611e72576000818152600185016020526040902054831415611e6a576001915050611e78565b600101611e1a565b50600090505b92915050565b8260005b6020811015611f2157600183821c166000858360208110611e9f57fe5b602002015190508160011415611ee55780846040516020018083815260200182815260200192505050604051602081830303815290604052805190602001209350611f17565b838160405160200180838152602001828152602001925050506040516020818303038152906040528051906020012093505b5050600101611e82565b509392505050565b6040805160e09290921b7fffffffff00000000000000000000000000000000000000000000000000000000166020808401919091527f4f5054494353000000000000000000000000000000000000000000000000000060248401528151808403600a018152602a909301909152815191012090565b80546000906fffffffffffffffffffffffffffffffff700100000000000000000000000000000000820481169116611fd6828261255a565b61204157604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600560248201527f456d707479000000000000000000000000000000000000000000000000000000604482015290519081900360640190fd5b6fffffffffffffffffffffffffffffffff8116600090815260018501602052604090205492508215612092576fffffffffffffffffffffffffffffffff811660009081526001850160205260408120555b83547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166001919091016fffffffffffffffffffffffffffffffff1617909255919050565b8151600090602084016120f264ffffffffff8516828461268e565b95945050505050565b6000610baa7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000008316602460046126e4565b6000610baa7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000008316602860046126e4565b60008061216983612705565b6bffffffffffffffffffffffff169050600061218484612719565b6bffffffffffffffffffffffff169091209392505050565b6000610baa6121aa8361272d565b61275e565b6000610baa7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000083168260046126e4565b6000610baa7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000831660046020612761565b6000610baa604c806122437fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000008616612719565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000861692916bffffffffffffffffffffffff9190911603600061290c565b606060008061228f84612719565b6bffffffffffffffffffffffff16905060405191508192506122b48483602001612992565b508181016020016040529052919050565b805470010000000000000000000000000000000090046fffffffffffffffffffffffffffffffff1660009081526001909101602052604090205490565b81546fffffffffffffffffffffffffffffffff8082167001000000000000000000000000000000009283900482166001019182169092029190911783558115611e78576fffffffffffffffffffffffffffffffff8116600090815260019390930160205260409092205590565b600061237a30612abe565b15905090565b80546fffffffffffffffffffffffffffffffff166123c25780547fffffffffffffffffffffffffffffffff000000000000000000000000000000001660011781555b50565b600054610100900460ff16806123de57506123de61236f565b806123ec575060005460ff16155b612441576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602e815260200180613236602e913960400191505060405180910390fd5b600054610100900460ff161580156124a757600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff909116610100171660011790555b600080547fffffffffffffffffffff0000000000000000000000000000000000000000ffff166201000073ffffffffffffffffffffffffffffffffffffffff851602177fffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffffffff16760100000000000000000000000000000000000000000000179055801561255657600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff1690555b5050565b60019103016fffffffffffffffffffffffffffffffff1690565b546fffffffffffffffffffffffffffffffff808216700100000000000000000000000000000000909204161090565b604080517f19457468657265756d205369676e6564204d6573736167653a0a333200000000602080830191909152603c8083019490945282518083039094018452605c909101909152815191012090565b6000815160411461266657604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e67746800604482015290519081900360640190fd5b60208201516040830151606084015160001a61268486828585612ac4565b9695505050505050565b60008061269b8484612cb2565b90506040518111156126ab575060005b806126d9577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000915050610ccb565b6120f2858585612d24565b60008160200360080260ff166126fb858585612761565b901c949350505050565b60781c6bffffffffffffffffffffffff1690565b60181c6bffffffffffffffffffffffff1690565b6000610baa7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000008316602c6020612761565b90565b600060ff821661277357506000610ccb565b61277c84612719565b6bffffffffffffffffffffffff166127978460ff8516612cb2565b1115612876576127d86127a985612705565b6bffffffffffffffffffffffff166127c086612719565b6bffffffffffffffffffffffff16858560ff16612d37565b6040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825283818151815260200191508051906020019080838360005b8381101561283b578181015183820152602001612823565b50505050905090810190601f1680156128685780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b60208260ff1611156128d3576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252603a815260200180613286603a913960400191505060405180910390fd5b6008820260006128e286612705565b6bffffffffffffffffffffffff16905060006128fd83612e92565b91909501511695945050505050565b60008061291886612705565b6bffffffffffffffffffffffff16905061293186612edb565b6129458561293f8489612cb2565b90612cb2565b1115612974577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000915050611602565b61297e8186612cb2565b90506126848364ffffffffff16828661268e565b600061299d83612f05565b6129f2576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260288152602001806132c06028913960400191505060405180910390fd5b6129fb83612f17565b612a50576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602b8152602001806132e8602b913960400191505060405180910390fd5b6000612a5b84612719565b6bffffffffffffffffffffffff1690506000612a7685612705565b6bffffffffffffffffffffffff1690506000604051905084811115612a9b5760206060fd5b8285848460045afa50612684612ab087612f54565b64ffffffffff168685612d24565b3b151590565b60007f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0821115612b3f576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260228152602001806131f36022913960400191505060405180910390fd5b8360ff16601b1480612b5457508360ff16601c145b612ba9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260228152602001806132646022913960400191505060405180910390fd5b600060018686868660405160008152602001604052604051808581526020018460ff1681526020018381526020018281526020019450505050506020604051602081039080840390855afa158015612c05573d6000803e3d6000fd5b50506040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0015191505073ffffffffffffffffffffffffffffffffffffffff81166120f257604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f45434453413a20696e76616c6964207369676e61747572650000000000000000604482015290519081900360640190fd5b81810182811015611e7857604080517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601960248201527f4f766572666c6f7720647572696e67206164646974696f6e2e00000000000000604482015290519081900360640190fd5b606092831b9190911790911b1760181b90565b60606000612d4486612f5a565b9150506000612d5286612f5a565b9150506000612d6086612f5a565b9150506000612d6e86612f5a565b915050838383836040516020018080613313603591397fffffffffffff000000000000000000000000000000000000000000000000000060d087811b821660358401527f2077697468206c656e6774682030780000000000000000000000000000000000603b84015286901b16604a820152605001602161321582397fffffffffffff000000000000000000000000000000000000000000000000000060d094851b811660218301527f2077697468206c656e677468203078000000000000000000000000000000000060278301529290931b9091166036830152507f2e00000000000000000000000000000000000000000000000000000000000000603c82015260408051601d818403018152603d90920190529b9a5050505050505050505050565b7f80000000000000000000000000000000000000000000000000000000000000007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9091011d90565b6000612ee682612719565b612eef83612705565b016bffffffffffffffffffffffff169050919050565b6000612f108261302e565b1592915050565b6000612f2282612f54565b64ffffffffff1664ffffffffff1415612f3d57506000610bad565b6000612f4883612edb565b60405110199392505050565b60d81c90565b600080601f5b600f8160ff161115612fc25760ff600882021684901c612f7f81613056565b61ffff16841793508160ff16601014612f9a57601084901b93505b507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01612f60565b50600f5b60ff8160ff1610156130285760ff600882021684901c612fe581613056565b61ffff16831792508160ff1660001461300057601083901b92505b507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01612fc6565b50915091565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000009081161490565b600061306860048360ff16901c613086565b60ff161760081b62ffff001661307d82613086565b60ff1617919050565b600060f08083179060ff821614156130a2576030915050610bad565b8060ff1660f114156130b8576031915050610bad565b8060ff1660f214156130ce576032915050610bad565b8060ff1660f314156130e4576033915050610bad565b8060ff1660f414156130fa576034915050610bad565b8060ff1660f51415613110576035915050610bad565b8060ff1660f61415613126576036915050610bad565b8060ff1660f7141561313c576037915050610bad565b8060ff1660f81415613152576038915050610bad565b8060ff1660f91415613168576039915050610bad565b8060ff1660fa141561317e576061915050610bad565b8060ff1660fb1415613194576062915050610bad565b8060ff1660fc14156131aa576063915050610bad565b8060ff1660fd14156131c0576064915050610bad565b8060ff1660fe14156131d6576065915050610bad565b8060ff1660ff14156131ec576066915050610bad565b5091905056fe45434453413a20696e76616c6964207369676e6174757265202773272076616c75652e20417474656d7074656420746f20696e646578206174206f6666736574203078496e697469616c697a61626c653a20636f6e747261637420697320616c726561647920696e697469616c697a656445434453413a20696e76616c6964207369676e6174757265202776272076616c756554797065644d656d566965772f696e646578202d20417474656d7074656420746f20696e646578206d6f7265207468616e20333220627974657354797065644d656d566965772f636f7079546f202d204e756c6c20706f696e74657220646572656654797065644d656d566965772f636f7079546f202d20496e76616c696420706f696e74657220646572656654797065644d656d566965772f696e646578202d204f76657272616e2074686520766965772e20536c696365206973206174203078a26469706673582212205eab682400e5f5d5f10b086ac94d96e262cea67909719354ce1a005bf341568e64736f6c63430007060033";

export class TestReplica__factory extends ContractFactory {
  constructor(signer?: Signer) {
    super(_abi, _bytecode, signer);
  }

  deploy(
    _localDomain: BigNumberish,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<TestReplica> {
    return super.deploy(_localDomain, overrides || {}) as Promise<TestReplica>;
  }
  getDeployTransaction(
    _localDomain: BigNumberish,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): TransactionRequest {
    return super.getDeployTransaction(_localDomain, overrides || {});
  }
  attach(address: string): TestReplica {
    return super.attach(address) as TestReplica;
  }
  connect(signer: Signer): TestReplica__factory {
    return super.connect(signer) as TestReplica__factory;
  }
  static readonly bytecode = _bytecode;
  static readonly abi = _abi;
  static createInterface(): TestReplicaInterface {
    return new utils.Interface(_abi) as TestReplicaInterface;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): TestReplica {
    return new Contract(address, _abi, signerOrProvider) as TestReplica;
  }
}