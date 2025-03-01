use bitcoincore_rpc::{Client, RpcApi};
use bitcoin::Block;

pub fn broadcast_block(block: Block, rpc_client: &Client) -> Result<(), bitcoincore_rpc::Error> {
    // Submit the block to the network using submitblock RPC call
    rpc_client.submit_block(&block)
}
