pub mod node_connect;
pub mod get_transactions;
pub mod construct_block;
pub mod calculate_merkle_root;
pub mod build_coinbase;

use get_transactions::get_block_template;

use crate::node_connect::connect_to_bitcoin_node;
use crate::get_transactions::get_mempool_transactions;
use crate::construct_block::construct_block;

fn main() {
    let client = connect_to_bitcoin_node();

    // Can remove this
    let txs = get_mempool_transactions(&client);
    dbg!(&txs);

    let template = get_block_template(&client);
    // dbg!(&template);

    // Add the option for the user to define this later
    let miner_address = "bc1qq0hyc6ftal99hks3uspapyl8vcscqjf4aad7sp";

    let block = construct_block(template, miner_address);
    dbg!(block);
}
