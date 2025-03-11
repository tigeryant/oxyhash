pub mod node_connect;
pub mod get_block_template;
pub mod construct_block;
pub mod calculate_merkle_root;
pub mod build_coinbase;
pub mod mine;
pub mod mining;
pub mod broadcast;
pub mod cli;

use get_block_template::get_block_template;
use node_connect::connect_to_bitcoin_node;
use construct_block::construct_block;
use mining::mine_main::mine_main;
use broadcast::broadcast_block;
use cli::{Commands, Cli};
use clap::Parser;
use blocktalk::BlockTalk;
use tokio::task::LocalSet;

#[tokio::main]
async fn main() {
    // Refactor cli into another file
    // CLI
    let cli = Cli::parse();

    if cli.debug {
        println!("Debug mode enabled");
    }

    if let Some(input) = cli.input {
        println!("Input file: {}", input);
    }

    let miner_address: String = match cli.command {
        Some(Commands::Address { address }) => {
            // define the address
            println!("address defined");
            address
        }
        None => {
            println!("Default - using hardcoded miner address");
            String::from("bc1qq0hyc6ftal99hks3uspapyl8vcscqjf4aad7sp")
        },
    };

    // blocktalk init
    let local = LocalSet::new();
    local.run_until(async {
        let blocktalk = BlockTalk::init("/Users/john/Development/bitcoin/bitcoin-node.sock").await.unwrap();
        let chain = blocktalk.chain();
        let (height, hash) = chain.get_tip().await.unwrap();
        // Get current tip
        println!("Current tip: height={}, hash={}", height, hash);
    }).await;

    let client = connect_to_bitcoin_node();

    let template = get_block_template(&client);

    let candidate_block = construct_block(template, &miner_address);

    let valid_block = mine_main(candidate_block);

    // Update how we connect to the client (should already have an IPC interface initialized)
    // Afterwards, start mining the next block
    let rpc_client = connect_to_bitcoin_node();
    match broadcast_block(valid_block, &rpc_client) {
        Ok(_) => {
            println!("Block broadcast");
            std::process::exit(0);
        },
        Err(e) => {
            println!("Failed to broadcast block: {}", e);
            std::process::exit(1);
        }
    }
}
