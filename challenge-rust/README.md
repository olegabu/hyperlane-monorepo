# Hyperlane Rust Challenge

This repo hosts Rust and Solidity code for the soluton to [Abacus Works Rust Challenge](https://docs.google.com/document/d/1JGVZPSZ6vFzeFjIhOvxdXiU7-FzTQfFb5mq8UVrL4C4/edit#heading=h.a9izzxpgeg32).

## Directory Structure

The project is structured as a mixed Rust workspace with a Foundry project under
`contracts/` and typesafe auto-generated bindings to the contracts under
`bindings/`.

```
├── Cargo.toml
├── app // <-- Rust application logic
├── contracts // <- The smart contracts + tests using Foundry
├── bindings // <-- Generated bindings to the smart contracts' abis (like Typechain)
```

## Testing

Given the repository contains Solidity and Rust code as well as it uses Foundry, 
there are 3 different workflows.

### Test Solidity

Move to [contracts](./contracts) folder.

Forge is using submodules to manage dependencies. Initialize the dependencies.

```bash
forge install
```

Build contracts in [src](./src) and test with [Messaging.t.sol](test/Messaging.t.sol).

```bash
forge build &&
forge test
```

### Demo with Foundry

Let's use tools from [Foundry](https://getfoundry.sh) suite: 
[forge](https://book.getfoundry.sh/forge/) to deploy, 
[cast](https://book.getfoundry.sh/cast/) to send transactions.
See the [book](https://book.getfoundry.sh/getting-started/installation.html) for instructions on how to install and use Foundry.


#### Initialize

I created *demo* user account on Sepolia and Mumbai test networks and got some tokens from their faucets. 
This user will deploy and call contracts. Here are his account and private key; let's save them into env variables.

```bash
export demo=0xd24fC1ddb91f4C5179b3f2e1a64816eBDEEE4dC0 &&
export demo_key=c1836c120a271f4633073501c04cc93a6ee2ba3b267847cb0fc90e29765d1694
```

Let's also save rpc urls and constants into variables to be used in our commands.

```bash
export sepolia=https://eth-sepolia.g.alchemy.com/v2/demo &&
export mumbai=https://rpc-mumbai.maticvigil.com &&
export mumbai_id=`(cast chain-id --rpc-url $mumbai)` &&
export mailbox=0xCC737a94FecaeC165AbCf12dED095BB13F037685
```

#### Contracts

We'll call a *sender* contract on the origin chain with a message to deliver, it will in turn call 
origin's *mailbox* contract which will deliver the message to the destination's *mailbox* 
and finally to *receiver* contract on the destination chain.

Deploy [HyperlaneMessageSender](./contracts/src/HyperlaneMessageSender.sol) to Sepolia
and [HyperlaneMessageReceiver](./contracts/src/HyperlaneMessageReceiver.sol) to Mumbai.

Their constructors take addresses of Mailbox contracts on their respective chain; 
they happen to be the same, see [Hyperlane's docs](https://docs.hyperlane.xyz/docs/resources/addresses).
Save their addresses into `$sender` and `$receiver`; 
for that we get json output from *forge* and parse it with [jq](https://jqlang.github.io/jq/download/).

```bash
export sender=`(forge create --rpc-url $sepolia --private-key=$demo_key HyperlaneMessageSender --constructor-args $mailbox --json | jq -r .deployedTo)` &&
export receiver=`(forge create --rpc-url $mumbai --private-key=$demo_key HyperlaneMessageReceiver --constructor-args $mailbox --json | jq -r .deployedTo)`
```

Here's the sender in [Sepolia explorer](https://sepolia.etherscan.io/address/0x03C43cDDcfb0DF2a4E670c8a8beeDcE2BAaeC144) deployed at block 4243635
and receiver in [Mumbai](https://mumbai.polygonscan.com/address/0x6482cda5df7605b52592a3d04af1f7e3004262fe) at block 39868015.

#### Message

Note the 20 byte address of the receiver contract needs to be padded to become a byte32 as required by the interface.
Let's save this into `$recipient`.

```bash
export recipient=`(echo $receiver | sed 's/0x/000000000000000000000000/g')`
```

Send a message from Sepolia's sender contract to Mumbai's receiver with *cast*.

```bash
cast send --rpc-url $sepolia --private-key=$demo_key $sender "sendString(uint32,bytes32,string)" $mumbai_id $recipient "hello from foundry"
```

See this message delivered on [Hyperlane Explorer](https://explorer.hyperlane.xyz/message/0xbdd6b25676fdbe6ab20a0895765bd877d246395b8f4ab370f5f7ccefa7a332e7).

#### Logs

Let's query Sepolia and Mumbai for the event logs of our sender and receiver contracts, starting from the blocks they were created at.

```bash
cast logs --rpc-url $sepolia --from-block 4243635 --address $sender &&
cast logs --rpc-url $mumbai --from-block 39868015 --address $receiver
```

## Generate Rust bindings to the contracts

Before we build the Rust CLI app we need to wrap our Solidity contracts into Rust code.
For our one simple method it may look excessive but demonstrates a rich workflow.

Rust bindings to the contracts can be generated via `forge bind`, which requires
first building our contracts. Move back to the project's root.

```bash
forge build --root ./contracts &&
forge bind --bindings-path ./bindings --root ./contracts --crate-name bindings --overwrite
```

## Demo Rust CLI

Build and test.

```bash
cargo test
```

See our CLI's commands and args. 

```bash
cargo run -- -h
```

I use defaults for the RPC, addresses and even the private key, for the ease of this demo only.

```
Demo program to send and query messages via Hyperlane

Usage: app [OPTIONS] <COMMAND>

Commands:
  send   Send message from sender contract on origin chain to reciever contract on destination chain
  query  Query messages from mailbox contract on origin chain
  help   Print this message or the help of the given subcommand(s)

Options:
      --sender-rpc <SENDER_RPC>
          RPC URL of the sender chain [default: https://eth-sepolia.g.alchemy.com/v2/demo]
      --sender-address <SENDER_ADDRESS>
          Contract address on the sender chain [default: 03C43cDDcfb0DF2a4E670c8a8beeDcE2BAaeC144]
      --receiver-id <RECEIVER_ID>
          ID of the receiver chain [default: 80001]
      --receiver-address <RECEIVER_ADDRESS>
          Contract address on the receiver chain [default: 6482CdA5DF7605B52592a3D04af1f7e3004262FE]
```

### Send messages

Use these defaults to send a message from Sepolia to Mumbai between our contracts.

```bash
cargo run -- send
```

The output points to our message on [Hyperlane Explorer](https://explorer.hyperlane.xyz/message/0xe90e9b6237f87405d78c873f94e5a3d34ab18dc3480d8d196816e4b1675aba8a).

```
delivering via https://eth-sepolia.g.alchemy.com/v2/demo from contract 03C43cDDcfb0DF2a4E670c8a8beeDcE2BAaeC144 to contract 6482CdA5DF7605B52592a3D04af1f7e3004262FE on chain 80001 message 'hello from rust'
transaction 0x10ad1bc97a84959fcce25b8d893420e977a5f8f7f869f71912d7ca812ef37e56 mined in block Some(4246113), check https://explorer.hyperlane.xyz/message/0xe90e9b6237f87405d78c873f94e5a3d34ab18dc3480d8d196816e4b1675aba8a
```

Send another message, [check it out](https://explorer.hyperlane.xyz/message/0x2d984635b32fe3400940b9f1fcbf35e81182f501936b571c7ff77b1ceb813780).

```bash
cargo run -- send --message 'another message from rust cli'
```

### Query messages

We can query for *Dispatch* events of the origin chain's Mailbox contract.

Note the arguments' logic may be a bit counterintuitive: if a filter argument is missing it will default to our known addresses. 
To query for all events one needs to explicitely pass wildcards.
This is done to simplify the demo: most likely you'll be querying the events of your own contracts, and appreciate less typing with defaults.

Also, passing multiple values for filter arguments is not yet supported, i.e. we can filter on *one* recipient not on an array of them.

The query starts with the block number we deployed our sender contract at, as a sensible default.

```bash
cargo run -- query
```

Returns events for our messages.

```
16 logs found
sender 0x03c4…c144 destination 80001 recipient 0000000000000000000000006482cda5df7605b52592a3d04af1f7e3004262fe
sender 0x03c4…c144 destination 80001 recipient 0000000000000000000000006482cda5df7605b52592a3d04af1f7e3004262fe
sender 0x03c4…c144 destination 80001 recipient 0000000000000000000000006482cda5df7605b52592a3d04af1f7e3004262fe
...

```

To query for Mailbox's Dispatch events for all senders and receivers not only ours, pass wildcards.
Note the starting block is still our default 4243635 which can be overriden with `---block-number` flag.

```bash
cargo run -- --receiver-address=* --sender-address=* --receiver-id=0 query
```

See here messages to Moonbase Alpha with chain id 1287.

```
27 logs found
sender 0x03c4…c144 destination 80001 recipient 6482cda5df7605b52592a3d04af1f7e3004262fe000000000000000000000000
sender 0x03c4…c144 destination 80001 recipient 6482cda5df7605b52592a3d04af1f7e3004262fe000000000000000000000000
sender 0x5d56…2381 destination 5 recipient 000000000000000000000000405bfdecb33230b4ad93c29ba4499b776cfba189
sender 0x03c4…c144 destination 80001 recipient 6482cda5df7605b52592a3d04af1f7e3004262fe000000000000000000000000
sender 0x5d56…2381 destination 1287 recipient 00000000000000000000000089e02c3c7b97bcba63279e10e2a44e6cef69e6b2
sender 0x03c4…c144 destination 80001 recipient 6482cda5df7605b52592a3d04af1f7e3004262fe000000000000000000000000
```
