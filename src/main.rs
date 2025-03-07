pub mod node_connect;
pub mod get_transactions;
pub mod construct_block;
pub mod calculate_merkle_root;
pub mod build_coinbase;
pub mod mine;
pub mod mining;
pub mod broadcast;
pub mod cli;

use std::env;

use get_transactions::get_block_template;
use node_connect::connect_to_bitcoin_node;
use get_transactions::get_mempool_transactions;
use construct_block::construct_block;
use mining::{main_proc::mine_main, worker_proc::mine_worker};
use broadcast::broadcast_block;
use cli::{Commands, Cli};
use clap::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).map(|s| s.as_str()) == Some("worker") {
        let block = mine_worker().unwrap();

        // broadcast the block
        let rpc_client = connect_to_bitcoin_node();
        match broadcast_block(block, &rpc_client) {
            Ok(_) => {
                eprintln!("Block broadcast");
                std::process::exit(0);
            },
            Err(e) => {
                eprintln!("Failed to broadcast block: {}", e);
                std::process::exit(1);
            }
        }
    }

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

    let client = connect_to_bitcoin_node();

    // Can remove this
    let _txs = get_mempool_transactions(&client);
    // dbg!(&txs);

    let template = get_block_template(&client);
    // dbg!(&template);

    // Add the option for the user to define this later
    // let miner_address = "bc1qq0hyc6ftal99hks3uspapyl8vcscqjf4aad7sp";

    let candidate_block = construct_block(template, &miner_address);
    // dbg!(&candidate_block);

    // This function returns the unit type
    mine_main(candidate_block);
    // dbg!(valid_block);
}
