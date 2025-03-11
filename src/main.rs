pub mod node_connect;
pub mod get_block_template;
pub mod construct_block;
pub mod calculate_merkle_root;
pub mod build_coinbase;
pub mod mining;
pub mod broadcast;
pub mod cli;

use get_block_template::get_block_template;
use node_connect::node_connect;
use construct_block::construct_block;
use mining::mine_main::mine_main;
use broadcast::broadcast_block;
use blocktalk::BlockTalk;
use tokio::task::LocalSet;
use cli::cli_args;

#[tokio::main]
async fn main() {
    let (miner_address, socket_path) = cli_args();

    // blocktalk init
    let local = LocalSet::new();
    local.run_until(async {
        let blocktalk = BlockTalk::init(&socket_path).await.unwrap();
        let chain = blocktalk.chain();
        let (height, hash) = chain.get_tip().await.unwrap();
        // Get current tip
        println!("Current tip: height={}, hash={}", height, hash);
    }).await;

    let client = node_connect();

    let template = get_block_template(&client);

    let candidate_block = construct_block(template, &miner_address);

    let valid_block = mine_main(candidate_block);

    // Update how we connect to the client (should already have an IPC interface initialized)
    // Afterwards, start mining the next block
    let rpc_client = node_connect();
    match broadcast_block(valid_block, &rpc_client) {
        Ok(_) => {
            // Start mining the new block after the block is broadcast
            println!("Block broadcast");
            std::process::exit(0);
        },
        Err(e) => {
            println!("Failed to broadcast block: {}", e);
            std::process::exit(1);
        }
    }
}
