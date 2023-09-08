use bindings::hyperlane_message_sender::HyperlaneMessageSender;
use ethers::{prelude::*, types::{Address,H256}};
use eyre::Result;
use std::sync::Arc;
use clap::{Parser, Subcommand};
use hex;

#[derive(Subcommand, Debug)]
enum Command {
    /// Send message from sender contract on origin chain to reciever contract on destination chain
    Send {
        /// Private key of demo account
        #[arg(long, default_value = "c1836c120a271f4633073501c04cc93a6ee2ba3b267847cb0fc90e29765d1694")]
        private_key: String,
        /// Message to send
        #[arg(long, default_value = "hello from rust")]
        message: String
    },
    /// Query messages from mailbox contract on origin chain
    Query {
        /// Filter events starting from this block number
        #[arg(long, default_value_t = 4243635)]
        from_block: u32
    }
}

/// Demo program to send and query messages via Hyperlane
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// RPC URL of the sender chain
    #[arg(long, default_value = "https://eth-sepolia.g.alchemy.com/v2/demo")]
    sender_rpc: String,
    /// Contract address on the sender chain
    #[arg(long, default_value = "03C43cDDcfb0DF2a4E670c8a8beeDcE2BAaeC144")]
    sender_address: String,
    /// ID of the receiver chain
    #[arg(long, default_value_t = 80001)]
    receiver_id: u32,
    /// Contract address on the receiver chain
    #[arg(long, default_value = "6482CdA5DF7605B52592a3D04af1f7e3004262FE")]
    receiver_address: String,
    #[clap(subcommand)]
    command: Command
}

async fn send(sender_rpc: String, sender_address: String, private_key: String, receiver_id: u32, receiver_address: String, message: String) -> Result<()> {
    println!("delivering via {} from contract {} to contract {} on chain {} message '{}'", sender_rpc, sender_address, receiver_address, receiver_id, message);

    let sender_address = sender_address.parse::<Address>()?;

    let provider = Provider::try_from(sender_rpc)?;

    let sender_id = provider.get_chainid().await?.as_u32();

    // safer not to hard code nor provide as cli arg
    // let private_key = env::var("demo_key")?;

    let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(sender_id);

    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let sender_contract = HyperlaneMessageSender::new(sender_address, client.clone());

    let recipient = format!("000000000000000000000000{}", receiver_address);
    let recipient = recipient.parse::<H256>()?;
    
    // first `await` returns a PendingTransaction, second one waits for it to be mined
    let res = sender_contract.send_string(receiver_id, recipient.as_fixed_bytes().to_owned(), message.to_owned()).send().await?.await;

    match res {
        Ok(option) => match option {
            Some(receipt) => println!("transaction {:?} mined in block {:?}, check https://explorer.hyperlane.xyz/message/{:?}", receipt.transaction_hash, receipt.block_number, receipt.logs[1].topics[1]),
            None => println!("no transaction was mined")
        },
        Err(error) => panic!("could not mine transaction for {error}")
    }

    Ok(())
}

const MAILBOX_ADDRESS: &str = "0xCC737a94FecaeC165AbCf12dED095BB13F037685";
const EVENT_SIGNATURE: &str = "Dispatch(address,uint32,bytes32,bytes)";

async fn query(sender_rpc: String, from_block: u32, sender_address: String, receiver_id: u32, receiver_address: String) -> Result<()> {
    let provider =  Arc::new(Provider::try_from(sender_rpc)?);
    let client = Arc::new(provider);

    let mut filter = Filter::new()
        .address(MAILBOX_ADDRESS.parse::<Address>()?)
        .event(EVENT_SIGNATURE) // Dispatch (index_topic_1 address sender, index_topic_2 uint32 destination, index_topic_3 bytes32 recipient, bytes message)
        .from_block(from_block);

    if sender_address != "*" {
        filter = filter.topic1(sender_address.parse::<Address>()?); // address sender
    }

    if receiver_id != 0 {
        filter = filter.topic2(U256::from(receiver_id)); // uint32 destination
    }

    if receiver_address != "*" {
        let recipient = format!("000000000000000000000000{}", receiver_address);
        filter = filter.topic3(recipient.parse::<H256>()?); // bytes32 recipient
    }

    let logs = client.get_logs(&filter).await?;
    
    println!("{} logs found", logs.iter().len());
    
    for log in logs.iter() {
        let sender = Address::from(log.topics[1]);
        let destination = U256::from_big_endian(&log.topics[2].as_bytes()[29..32]);
        let recipient = hex::encode(log.topics[3].as_bytes());
        println!("sender {sender} destination {destination} recipient {recipient}");
    }
    
    Ok(())

}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Send {private_key, message} => send(args.sender_rpc, args.sender_address, private_key, args.receiver_id, args.receiver_address, message).await,
        Command::Query {from_block}  => query(args.sender_rpc, from_block, args.sender_address, args.receiver_id, args.receiver_address).await
    }
}
