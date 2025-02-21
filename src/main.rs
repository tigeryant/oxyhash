pub mod node_connect;
pub mod get_transactions;

use get_transactions::get_block_template;

use crate::node_connect::connect_to_bitcoin_node;
use crate::get_transactions::get_mempool_transactions;

fn main() {
    let client = connect_to_bitcoin_node();

    let txs = get_mempool_transactions(&client);

    let template = get_block_template(&client);

    dbg!(&txs);
    dbg!(&template);
}
